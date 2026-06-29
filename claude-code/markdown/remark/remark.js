import { remark } from 'remark';
import remarkFrontmatter from 'remark-frontmatter';
import remarkBehead from 'remark-behead';

export function createRemark(options = {}) {
  const {
    frontmatter = true,
    behead = null,  // z.B. { depth: 1 }
  } = options;

  const processor = remark();

  if (frontmatter) {
    processor.use(remarkFrontmatter, ['yaml']);
  }

  if (behead !== null) {
    processor.use(remarkBehead, behead);
  }

  return processor;
}

export default createRemark();
