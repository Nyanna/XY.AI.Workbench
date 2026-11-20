import type { Nodes, Root, RootContent } from 'mdast';
import { KIND_FRONTMATTER, KIND_SECTION, type AddressableNode, type NodeRange } from './AddressableNode.ts';
import type { PositionEnum } from '../openapi/request/nodes/nodeid/insert/post/json/PositionEnum.ts';
import { AstEngineException } from './AstEngineException.ts';
import { createProcessor } from './MarkdownProcessor.ts';

/**
 * remark mechanics: parse/print, fragment parsing keyed by the target node's kind, and the four
 * mutation primitives. Every print goes through remark-stringify, so every write normalises formatting.
 * Mutations operate on the (flat) `root.children` list in place; callers pass a tree they own.
 */
export class MarkdownAstEngine {
  private readonly processor = createProcessor();

  parseDocument(source: string): Root {
    try {
      return this.processor.parse(source);
    } catch (e) {
      throw new AstEngineException('SYNTAX', `invalid markdown document: ${messageOf(e)}`);
    }
  }

  emptyDocument(): Root {
    return { type: 'root', children: [] };
  }

  print(root: Root): string {
    return this.printNodes(root.children);
  }

  printNodes(nodes: readonly RootContent[]): string {
    return this.processor.stringify({ type: 'root', children: [...nodes] });
  }

  printRange(root: Root, range: NodeRange): string {
    return this.printNodes(root.children.slice(range.start, range.end)).replace(/\n+$/, '');
  }

  /** Returns an error message if `source` is malformed, else `null`. Markdown has no syntax errors beyond parser failures. */
  validate(source: string): string | null {
    try {
      this.parseDocument(source);
      return null;
    } catch (e) {
      return messageOf(e);
    }
  }

  append(root: Root, code: string | null): number {
    const fragments = this.parseFragment(code);
    this.splice(root, root.children.length, 0, fragments);
    return fragments.length;
  }

  replace(root: Root, target: AddressableNode, code: string | null): void {
    const fragments = this.parseFragmentFor(target.range, code);
    this.splice(root, target.range.start, target.range.end - target.range.start, fragments);
  }

  insert(root: Root, target: AddressableNode, code: string | null, position: PositionEnum): number {
    const fragments = this.parseFragmentFor(target.range, code);
    this.splice(root, position === 'after' ? target.range.end : target.range.start, 0, fragments);
    return fragments.length;
  }

  delete(root: Root, target: AddressableNode): void {
    this.splice(root, target.range.start, target.range.end - target.range.start, []);
  }

  /** One-line header rendering: the heading line of a section, else the first non-blank printed line. */
  signature(root: Root, range: NodeRange, limit: number): string {
    const first = root.children[range.start];
    const printed = first ? this.printNodes([first]) : '';
    return abbreviate(collapse(firstLine(printed)), limit);
  }

  /** First paragraph directly under a section's heading, as plain text. */
  docstring(root: Root, range: NodeRange, limit: number): string | null {
    if (range.kind !== KIND_SECTION)
      return null;
    for (let i = range.start + 1; i < range.end; i++) {
      const node = root.children[i];
      if (!node || node.type === 'heading')
        break;
      if (node.type === 'paragraph') {
        const text = collapse(textOf(node));
        return text ? abbreviate(text, limit) : null;
      }
    }
    return null;
  }

  private parseFragmentFor(range: NodeRange, code: string | null): RootContent[] {
    const fragments = this.parseFragment(code);
    if (range.kind === KIND_FRONTMATTER && !fragments.every((f) => f.type === 'yaml'))
      throw new AstEngineException('SYNTAX', "expected frontmatter: YAML delimited by '---' lines");
    return fragments;
  }

  private parseFragment(code: string | null): RootContent[] {
    if (code == null || code.trim() === '')
      throw new AstEngineException('SYNTAX', 'no markdown content found: code is empty');
    return this.parseDocument(code).children;
  }

  private splice(root: Root, start: number, deleteCount: number, fragments: RootContent[]): void {
    const next = [...root.children.slice(0, start), ...fragments, ...root.children.slice(start + deleteCount)];
    const yaml = next.flatMap((node, i) => (node.type === 'yaml' ? [i] : []));
    if (yaml.length > 1 || (yaml.length === 1 && yaml[0] !== 0))
      throw new AstEngineException('SYNTAX', 'frontmatter must be the single first block of the document');
    root.children = next;
  }
}

export function headingText(node: Nodes): string {
  return collapse(textOf(node));
}

function textOf(node: Nodes): string {
  if ('value' in node)
    return node.value;
  if ('children' in node)
    return node.children.map((child) => textOf(child)).join('');
  return node.type === 'image' ? node.alt ?? '' : '';
}

function collapse(text: string): string {
  return text.replace(/\s+/g, ' ').trim();
}

function abbreviate(text: string, limit: number): string {
  return text.length <= limit ? text : text.slice(0, limit - 1) + '…';
}

function firstLine(printed: string): string {
  return printed.split('\n').find((line) => line.trim() !== '') ?? '';
}

function messageOf(e: unknown): string {
  return e instanceof Error ? e.message : String(e);
}
