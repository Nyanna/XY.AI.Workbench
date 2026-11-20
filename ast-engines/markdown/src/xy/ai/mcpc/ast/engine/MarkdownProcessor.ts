import { remark } from 'remark';
import remarkFrontmatter from 'remark-frontmatter';

/** Counterpart of `createRemark` in remark.js, with the stringify style pinned so every write normalises the same way. */
export function createProcessor() {
  return remark()
    .use(remarkFrontmatter, ['yaml'])
    .data('settings', {
      bullet: '-',
      emphasis: '_',
      strong: '*',
      fence: '`',
      fences: true,
      rule: '-',
      ruleRepetition: 3,
      setext: false,
      listItemIndent: 'one',
      incrementListMarker: true,
    });
}

export type MarkdownProcessor = ReturnType<typeof createProcessor>;
