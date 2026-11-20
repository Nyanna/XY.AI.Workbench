import { createHash } from 'node:crypto';
import type { Root, RootContent } from 'mdast';
import { KIND_BLOCKS, KIND_FRONTMATTER, KIND_SECTION, type AddressableNode, type NodeKind } from './AddressableNode.ts';
import { headingText, type MarkdownAstEngine } from './MarkdownAstEngine.ts';

/** A group keeps accumulating sibling blocks until adding the next one would push its source past this many characters. */
const SEGMENT_MAX_CHARS = 1000;
const UNSAFE = /[^\p{L}\p{N}_]+/gu;
const HASH_ALPHABET = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

/**
 * Flattens a markdown tree into every addressable node, in document order. Headings open nested
 * "Section" definitions (heading up to the next heading of the same or a higher level); the YAML
 * frontmatter and consecutive runs of other blocks (paragraphs, lists, code, ...) are collapsed into
 * anonymous groups capped at {@link SEGMENT_MAX_CHARS}.
 *
 * Ids are name-based structural paths (e.g. `Guide.Install_2`) for sections and content-hash
 * paths (e.g. `a1B2c3|d4E5f6`) for anonymous groups, rebuilt on every call from the live tree.
 */
export function locateAll(root: Root, engine: MarkdownAstEngine): AddressableNode[] {
  const out: AddressableNode[] = [];
  walk(root, 0, root.children.length, 'Root', '', engine, out);
  return out;
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
