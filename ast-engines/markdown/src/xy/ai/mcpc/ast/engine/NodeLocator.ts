import { createHash } from 'node:crypto';
import type { Root, RootContent } from 'mdast';
import { KIND_BLOCKS, KIND_FRONTMATTER, KIND_SECTION, type AddressableNode, type NodeKind } from './AddressableNode.ts';
import { headingText, type MarkdownAstEngine } from './MarkdownAstEngine.ts';

/** A group keeps accumulating sibling blocks until adding the next one would push its source past this many characters. */
const SEGMENT_MAX_CHARS = 1000;
const UNSAFE = /[^\p{L}\p{N}_]+/gu;
const HASH_ALPHABET = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

/** Shortest public id length; grows per node only while a shorter prefix would collide within the document. */
const MIN_ID_LENGTH = 5;

const FULL_HASH_LENGTH = 12;

/**
 * Flattens a markdown tree into every addressable node, in document order. Headings open nested
 * "Section" definitions (heading up to the next heading of the same or a higher level); the YAML
 * frontmatter and consecutive runs of other blocks (paragraphs, lists, code, ...) are collapsed into
 * anonymous groups capped at {@link SEGMENT_MAX_CHARS}.
 *
 * Each node has an internal structural key (heading-name path for sections, content-hash path for
 * anonymous groups). The public id is the shortest prefix (at least {@link MIN_ID_LENGTH} chars) of
 * the base62 hash of that key which is unique within the document, e.g. `a1B2c`. Ids are rebuilt on
 * every call from the live tree.
 */
export function locateAll(root: Root, engine: MarkdownAstEngine): AddressableNode[] {
  const out: AddressableNode[] = [];
  walk(root, 0, root.children.length, 'Root', '', engine, out);
  return shortenIds(out);
}

function shortenIds(nodes: AddressableNode[]): AddressableNode[] {
  const hashes = nodes.map((n) => base62Hash(n.id, FULL_HASH_LENGTH));
  const order = hashes.map((_, i) => i).sort((a, b) => (hashes[a]! < hashes[b]! ? -1 : hashes[a]! > hashes[b]! ? 1 : 0));
  const lengths = new Array<number>(nodes.length).fill(MIN_ID_LENGTH);
  for (let k = 1; k < order.length; k++) {
    const a = order[k - 1]!;
    const b = order[k]!;
    const needed = commonPrefix(hashes[a]!, hashes[b]!) + 1;
    lengths[a] = Math.max(lengths[a]!, needed);
    lengths[b] = Math.max(lengths[b]!, needed);
  }
  return nodes.map((n, i) => ({ ...n, id: hashes[i]!.slice(0, lengths[i]) }));
}

function commonPrefix(a: string, b: string): number {
  let n = 0;
  while (n < a.length && n < b.length && a.charAt(n) === b.charAt(n))
    n++;
  return n;
}

function walk(root: Root, from: number, to: number, parentType: string, parentId: string, engine: MarkdownAstEngine, out: AddressableNode[]): void {
  const children = root.children;
  const used = new Map<string, number>();
  let i = from;
  while (i < to) {
    const node = children[i]!;
    if (node.type === 'heading') {
      const end = sectionEnd(children, i, to);
      const name = headingText(node);
      const id = join(parentId, segment(name, KIND_SECTION, used));
      out.push({
        id,
        range: { kind: KIND_SECTION, start: i, end },
        type: KIND_SECTION,
        name,
        lineno: line(node),
        endLineno: endLine(children[end - 1]!),
        parentType,
        expandable: children.slice(i + 1, end).some((c) => c.type === 'heading'),
        isDefinition: true,
      });
      walk(root, i + 1, end, KIND_SECTION, id, engine, out);
      i = end;
      continue;
    }
    if (node.type === 'yaml') {
      addGroup(root, KIND_FRONTMATTER, i, i + 1, parentType, parentId, used, engine, out);
      i++;
      continue;
    }
    const start = i;
    let length = 0;
    while (i < to) {
      const current = children[i]!;
      if (current.type === 'heading' || current.type === 'yaml')
        break;
      const piece = engine.printNodes([current]).length;
      if (i > start && length + piece > SEGMENT_MAX_CHARS)
        break;
      length += piece;
      i++;
    }
    addGroup(root, KIND_BLOCKS, start, i, parentType, parentId, used, engine, out);
  }
}

function sectionEnd(children: readonly RootContent[], at: number, to: number): number {
  const depth = (children[at] as { depth: number }).depth;
  let end = at + 1;
  while (end < to) {
    const next = children[end]!;
    if (next.type === 'heading' && next.depth <= depth)
      break;
    end++;
  }
  return end;
}

function addGroup(root: Root, kind: NodeKind, start: number, end: number, parentType: string, parentId: string, used: Map<string, number>, engine: MarkdownAstEngine, out: AddressableNode[]): void {
  const content = engine.printRange(root, { kind, start, end });
  out.push({
    id: join(parentId, anonymousSegment(content, used)),
    range: { kind, start, end },
    type: kind,
    name: null,
    lineno: line(root.children[start]!),
    endLineno: endLine(root.children[end - 1]!),
    parentType,
    expandable: false,
    isDefinition: false,
  });
}

function join(parentId: string, seg: string): string {
  return parentId === '' ? seg : `${parentId}.${seg}`;
}

function segment(name: string, type: string, used: Map<string, number>): string {
  const base = name.replace(UNSAFE, '_').replace(/^_+|_+$/g, '') || type;
  return unique(base, used);
}

/** Unique-within-siblings id segment for an anonymous group: `<shapeHash>|<contentHash>`, both 6-char base62. */
function anonymousSegment(content: string, used: Map<string, number>): string {
  return unique(`${contentPrefixHash(content)}|${base62Hash(content, 6)}`, used);
}

function unique(base: string, used: Map<string, number>): string {
  const count = (used.get(base) ?? 0) + 1;
  used.set(base, count);
  return count === 1 ? base : `${base}_${count}`;
}

/** Hash of the whitespace-stripped first/last 20 chars: stays put even when unrelated edits shift the group's interior. */
function contentPrefixHash(content: string): string {
  const stripped = content.replace(/\s+/g, '');
  return base62Hash(stripped.slice(0, 20) + stripped.slice(Math.max(0, stripped.length - 20)), 6);
}

function base62Hash(text: string, length: number): string {
  let digest = BigInt('0x' + createHash('sha1').update(text, 'utf8').digest('hex'));
  const base = BigInt(HASH_ALPHABET.length);
  let out = '';
  for (let i = 0; i < length; i++) {
    out += HASH_ALPHABET.charAt(Number(digest % base));
    digest /= base;
  }
  return out;
}

function line(node: RootContent): number {
  return node.position?.start.line ?? 1;
}

function endLine(node: RootContent): number {
  return node.position?.end.line ?? 1;
}
