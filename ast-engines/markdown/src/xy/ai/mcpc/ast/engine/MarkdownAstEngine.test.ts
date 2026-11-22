import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import type { AddressInfo } from 'node:net';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { after, before, describe, it } from 'node:test';
import { AstEngineException } from './AstEngineException.ts';
import { MarkdownAstEngine } from './MarkdownAstEngine.ts';
import { MarkdownAstServer } from './MarkdownAstServer.ts';
import { locateAll } from './NodeLocator.ts';
import { InlineExecutor, WorkerPool } from './TaskExecutors.ts';

interface NodeDto {
  id: string;
  code?: string;
}

interface LocateBody {
  nodes: NodeDto[];
}

const DOC = `---
title: Demo
---

Intro text.

# Guide

Guide intro.

## Install

- one
- two

## Über uns

Text.

## Install

Again.

# Reference

\`\`\`js
let x = 1;
\`\`\`
`;

describe('NodeLocator', () => {
  const engine = new MarkdownAstEngine();
  const nodes = locateAll(engine.parseDocument(DOC), engine);
  const section = (name: string) => nodes.find((n) => n.isDefinition && n.name === name)!;
  const SEG = '[0-9a-zA-Z]{4}';

  it('addresses sections by dot-joined hash segments, unique per document', () => {
    const ids = nodes.map((n) => n.id);
    assert.equal(new Set(ids).size, ids.length);
    const guide = section('Guide');
    assert.match(guide.id, new RegExp(`^${SEG}$`));
    assert.match(section('Über uns').id, new RegExp(`^${guide.id}\\.${SEG}$`));
  });

  it('keeps sibling sections with equal names apart', () => {
    const installs = nodes.filter((n) => n.name === 'Install');
    assert.equal(installs.length, 2);
    assert.notEqual(installs[0]!.id, installs[1]!.id);
    assert.match(installs[1]!.id, /_2$/);
  });

  it('keeps section ids stable across edits elsewhere and across repeated runs', () => {
    const changed = locateAll(engine.parseDocument(DOC.replace('let x = 1;', 'let y = 2;')), engine);
    assert.deepEqual(changed.filter((n) => n.isDefinition).map((n) => n.id), nodes.filter((n) => n.isDefinition).map((n) => n.id));
    assert.deepEqual(locateAll(engine.parseDocument(DOC), engine).map((n) => n.id), nodes.map((n) => n.id));
  });

  it('reports section metadata', () => {
    const guide = section('Guide');
    assert.equal(guide.type, 'Section');
    assert.equal(guide.parentType, 'Root');
    assert.equal(guide.expandable, true);
    assert.equal(section('Install').parentType, 'Section');
    assert.equal(section('Install').expandable, false);
    assert.equal(guide.lineno, 7);
  });

  it('collapses frontmatter and loose blocks into anonymous groups', () => {
    const groups = nodes.filter((n) => !n.isDefinition);
    assert.equal(groups[0]!.type, 'Frontmatter');
    assert.equal(groups[1]!.type, 'BlockGroup');
    assert.match(groups[1]!.id, new RegExp(`^${SEG}\\|${SEG}$`));
    assert.match(groups[2]!.id, new RegExp(`^${section('Guide').id}\\.${SEG}\\|${SEG}$`));
  });

  it('keeps the group prefix when only the middle changes', () => {
    const text = 'Alpha beta gamma delta epsilon zeta eta theta MIDDLE iota kappa lambda mu nu xi omicron pi rho sigma tau end.';
    const before = locateAll(engine.parseDocument(`# A\n\n${text}\n`), engine).find((n) => !n.isDefinition)!.id;
    const after = locateAll(engine.parseDocument(`# A\n\n${text.replace('MIDDLE', 'CHANGED')}\n`), engine).find((n) => !n.isDefinition)!.id;
    assert.equal(before.split('|')[0], after.split('|')[0]);
    assert.notEqual(before, after);
  });

  it('splits groups at the character cap', () => {
    const doc = Array.from({ length: 30 }, (_, i) => `Paragraph ${i} ${'x'.repeat(100)}`).join('\n\n');
    const groups = locateAll(engine.parseDocument(doc), engine);
    assert.ok(groups.length > 1);
    assert.ok(groups.every((g) => g.type === 'BlockGroup'));
  });
})

describe('MarkdownAstEngine mutations', () => {
  const engine = new MarkdownAstEngine();
  const target = (root: ReturnType<typeof engine.parseDocument>, name: string) => locateAll(root, engine).find((n) => n.isDefinition && n.name === name)!;
  const sectionNames = (root: ReturnType<typeof engine.parseDocument>) => locateAll(root, engine).filter((n) => n.isDefinition).map((n) => n.name);

  it('replaces a section including nested sections', () => {
    const root = engine.parseDocument(DOC);
    engine.replace(root, target(root, 'Guide'), '# Guide\n\nNew.');
    assert.deepEqual(sectionNames(root), ['Guide', 'Reference']);
  });

  it('inserts before and after a section', () => {
    const root = engine.parseDocument(DOC);
    assert.equal(engine.insert(root, target(root, 'Reference'), '# Appendix', 'before'), 1);
    assert.equal(engine.insert(root, target(root, 'Reference'), '# Last', 'after'), 1);
    assert.deepEqual(sectionNames(root).slice(-3), ['Appendix', 'Reference', 'Last']);
  });

  it('deletes a section', () => {
    const root = engine.parseDocument(DOC);
    engine.delete(root, target(root, 'Install'));
    assert.ok(!engine.print(root).includes('- one'));
  });

  it('rejects frontmatter outside the document start and leaves the tree untouched', () => {
    const root = engine.parseDocument(DOC);
    const before = engine.print(root);
    assert.throws(() => engine.insert(root, target(root, 'Reference'), '---\na: 1\n---', 'after'), { kind: 'SYNTAX' });
    assert.throws(() => engine.replace(root, target(root, 'Reference'), ''), { kind: 'SYNTAX' });
    assert.equal(engine.print(root), before);
  });

  it('only accepts yaml for the frontmatter node', () => {
    const root = engine.parseDocument(DOC);
    const fm = locateAll(root, engine).find((n) => n.type === 'Frontmatter')!;
    assert.throws(() => engine.replace(root, fm, 'plain'), { kind: 'SYNTAX' });
    engine.replace(root, fm, '---\ntitle: Changed\n---');
    assert.ok(engine.print(root).startsWith('---\ntitle: Changed\n---'));
  });

  it('appends to an empty and to an existing document', () => {
    const empty = engine.emptyDocument();
    assert.equal(engine.append(empty, '# A\n\ntext'), 2);
    engine.append(empty, '# B');
    assert.equal(engine.print(empty), '# A\n\ntext\n\n# B\n');
  });

  it('renders signature and docstring', () => {
    const root = engine.parseDocument(DOC);
    const guide = target(root, 'Guide');
    assert.equal(engine.signature(root, guide.range, 80), '# Guide');
    assert.equal(engine.docstring(root, guide.range, 80), 'Guide intro.');
    assert.equal(engine.docstring(root, target(root, 'Install').range, 80), null);
  });
});

describe('MarkdownAstServer', () => {
  let base = '';
  let dir = '';
  let close: () => Promise<void> = async () => {};

  before(async () => {
    dir = await mkdtemp(join(tmpdir(), 'md-ast-'));
    const app = new MarkdownAstServer({ workers: 3 });
    const server = await app.start(0, '127.0.0.1');
    base = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
    close = async () => {
      server.closeAllConnections();
      server.close();
      await app.close();
    };
  });

  after(async () => {
    await close();
    await rm(dir, { recursive: true, force: true });
  });

  async function idOf(codePrefix: string, body: unknown = { source: DOC }): Promise<string> {
    const listed = await call('POST', '/nodes', { ...(body as object), includeCode: true });
    return (listed.json as LocateBody).nodes.find((n) => n.code?.startsWith(codePrefix))!.id;
  }

  async function call(method: string, path: string, body?: unknown): Promise<{ status: number; json: any }> {
    const res = await fetch(base + path, { method, body: body === undefined ? undefined : JSON.stringify(body) });
    const text = await res.text();
    return { status: res.status, json: res.headers.get('content-type')?.includes('json') ? JSON.parse(text) : text };
  }

  it('reports engine info', async () => {
    const res = await call('GET', '/info');
    assert.deepEqual(res.json, { name: 'markdown-remark', validatesSyntax: false });
  });

  it('lists nodes statelessly, with and without code', async () => {
    const plain = await call('POST', '/nodes', { source: DOC });
    assert.equal(plain.status, 200);
    assert.equal((plain.json as LocateBody).nodes.length, 12);
    assert.equal((plain.json as LocateBody).nodes[0]!.code, undefined);
    const withCode = await call('POST', '/nodes', { source: DOC, includeCode: true });
    assert.ok((withCode.json as LocateBody).nodes.every((n: NodeDto) => typeof n.code === 'string'));
  });

  it('gets a node by url-encoded id', async () => {
    const id = await idOf('## Über uns');
    const res = await call('POST', `/nodes/${encodeURIComponent(id)}`, { source: DOC });
    assert.equal(res.status, 200);
    assert.equal(res.json.code, '## Über uns\n\nText.');
  });

  it('answers 404 with candidates and 422 without input', async () => {
    const guideId = await idOf('# Guide');
    const missing = await call('POST', `/nodes/${guideId}x`, { source: DOC });
    assert.equal(missing.status, 404);
    assert.ok(missing.json.candidates.includes(guideId));
    assert.equal((await call('POST', '/nodes', {})).status, 422);
    assert.equal((await call('POST', '/nodes', 'x')).status, 422);
    assert.equal((await call('POST', `/nodes/${guideId}/insert`, { source: DOC, code: 'x', position: 'middle' })).status, 422);
    assert.equal((await call('GET', '/nope')).status, 404);
  });

  it('mutates statelessly without touching disk', async () => {
    const res = await call('POST', `/nodes/${await idOf('# Reference')}/insert`, { source: DOC, code: '# Appendix', position: 'before' });
    assert.equal(res.status, 200);
    assert.equal(res.json.unitsInserted, 1);
    assert.ok(res.json.source.includes('# Appendix\n\n# Reference'));
  });

  it('persists path-based mutations, detects stale sources and creates new files', async () => {
    const file = join(dir, 'doc.md');
    await writeFile(file, DOC);
    const installId = await idOf('## Install\n\n- one', { path: file });
    const guideId = await idOf('# Guide', { path: file });
    const del = await call('POST', `/nodes/${installId}/delete`, { path: file });
    assert.equal(del.status, 200);
    assert.equal(await readFile(file, 'utf8'), del.json.source);
    assert.ok(!del.json.source.includes('- one'));

    const stale = await call('POST', `/nodes/${guideId}/replace`, { path: file, source: DOC, code: '# Guide' });
    assert.equal(stale.status, 409);

    await writeFile(file, '# External\n');
    const listed = await call('POST', '/nodes', { path: file });
    assert.equal(listed.json.nodes.length, 1);

    const created = join(dir, 'new.md');
    const appended = await call('POST', '/append', { path: created, code: '# Fresh\n\ntext' });
    assert.equal(appended.status, 200);
    assert.equal(appended.json.unitsAppended, 2);
    assert.equal(await readFile(created, 'utf8'), '# Fresh\n\ntext\n');
  });

  it('validates sources', async () => {
    assert.deepEqual((await call('POST', '/validate', { source: DOC })).json, {});
    assert.equal((await call('POST', '/validate', {})).json.error, "either 'path' or 'source' is required");
  });
});

describe('task executors', () => {
  let dir = '';

  before(async () => {
    dir = await mkdtemp(join(tmpdir(), 'md-pool-'));
  });

  after(async () => {
    await rm(dir, { recursive: true, force: true });
  });

  it('never loses updates when many tasks hit the same file at once', async () => {
    const pool = new WorkerPool(4);
    try {
      const file = join(dir, 'same.md');
      await Promise.all(Array.from({ length: 25 }, (_, i) => pool.submit({ op: 'append', path: file, source: null, code: `para ${i}` })));
      const text = await readFile(file, 'utf8');
      for (let i = 0; i < 25; i++)
        assert.ok(text.includes(`para ${i}\n`), `missing para ${i}`);
    } finally {
      await pool.close();
    }
  });

  it('processes many files at once and keeps their results apart', async () => {
    const pool = new WorkerPool(4);
    try {
      const files = await Promise.all(Array.from({ length: 24 }, async (_, i) => {
        const file = join(dir, `many-${i}.md`);
        await writeFile(file, `# Doc ${i}\n\ntext ${i}\n`);
        return file;
      }));
      const results = await Promise.all(files.map((path) => pool.submit({ op: 'list', path, source: null, includeCode: true })));
      results.forEach((r, i) => assert.ok(String(r.nodes[0]!['code']).startsWith(`# Doc ${i}\n`)));
      assert.equal(pool.size, 4);
    } finally {
      await pool.close();
    }
  });

  it('gives pooled and inline execution identical results', async () => {
    const pool = new WorkerPool(2);
    const inline = new InlineExecutor();
    try {
      const task = { op: 'list', path: null, source: DOC, includeCode: true } as const;
      assert.deepEqual(await pool.submit(task), await inline.submit(task));
    } finally {
      await pool.close();
    }
  });

  it('rebuilds engine errors, including candidates, on the calling thread', async () => {
    const pool = new WorkerPool(2);
    try {
      const engine = new MarkdownAstEngine();
      const guideId = locateAll(engine.parseDocument(DOC), engine).find((n) => n.name === 'Guide')!.id;
      await assert.rejects(pool.submit({ op: 'get', path: null, source: DOC, nodeId: `${guideId}x` }), (e: unknown) => {
        assert.ok(e instanceof AstEngineException);
        assert.equal(e.kind, 'NOT_FOUND');
        assert.ok(e.candidates.includes(guideId));
        return true;
      });
    } finally {
      await pool.close();
    }
  });

  it('normalizes files and leaves already normal ones untouched', async () => {
    const pool = new WorkerPool(2);
    try {
      const file = join(dir, 'norm.md');
      await writeFile(file, '# T\n- a\n- b');
      assert.deepEqual(await pool.submit({ op: 'normalize', path: file }), { changed: true });
      assert.equal(await readFile(file, 'utf8'), '# T\n\n- a\n- b\n');
      assert.deepEqual(await pool.submit({ op: 'normalize', path: file }), { changed: false });
    } finally {
      await pool.close();
    }
  });
});
