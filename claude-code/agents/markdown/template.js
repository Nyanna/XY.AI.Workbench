import { read, write } from 'to-vfile';
import { createRemark } from './remark.js';

const processor = createRemark({
  // frontmatter: true,
  // behead: { depth: 1 },
});

processor.use(() => (tree, file) => {
  // Verarbeitungscode hier einfügen
});

const file = await read('test.md');
await processor.process(file);
file.path = 'test.md';
await write(file);
