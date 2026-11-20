export const KIND_SECTION = 'Section';
export const KIND_FRONTMATTER = 'Frontmatter';
export const KIND_BLOCKS = 'BlockGroup';

export type NodeKind = typeof KIND_SECTION | typeof KIND_FRONTMATTER | typeof KIND_BLOCKS;

/**
 * A contiguous run `[start, end)` of the root's flat child list. A section spans its heading up to the
 * next heading of the same or a higher level (including nested sections); a group spans sibling blocks
 * that are not individually addressable.
 */
export interface NodeRange {
  readonly kind: NodeKind;
  readonly start: number;
  readonly end: number;
}

/** One node the HTTP API can list/get/replace/insert/delete. */
export interface AddressableNode {
  readonly id: string;
  readonly range: NodeRange;
  readonly type: NodeKind;
  readonly name: string | null;
  readonly lineno: number;
  readonly endLineno: number;
  readonly parentType: string;
  readonly expandable: boolean;
  readonly isDefinition: boolean;
}
