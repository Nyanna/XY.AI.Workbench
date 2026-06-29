#!/bin/bash

node --input-type=module << 'EOF'
	import { read, write } from 'to-vfile';
	import { createRemark } from './remark.js';

	const processor = createRemark({
	  // frontmatter: true, // if required
	  // behead: { depth: 1 }, // if required
	});

	processor.use(() => (tree, file) => {
	  // insert code here
	});

	const file = await read('test.md');
	await processor.process(file);
	file.path = 'test.md';
	await write(file);
	
	console.log(String("Done"));
EOF
  