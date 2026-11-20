import { availableParallelism } from 'node:os';
import { readdir } from 'node:fs/promises';
import { join } from 'node:path';
import { MarkdownAstServer } from './MarkdownAstServer.ts';
import { WorkerPool } from './TaskExecutors.ts';

/** Starts the remark AST engine as a standalone JSON/HTTP backed engine. */
async function main(args: string[]): Promise<void> {
  const workers = workerCount();
  if (args[0] === 'convert') {
    if (args[1] === undefined) {
      console.error('usage: convert <directory>');
      process.exit(1);
    }
    await convert(args[1], workers);
    return;
  }
  const port = args[0] !== undefined ? Number(args[0]) : Number(process.env['PORT'] ?? '8788');
  if (!Number.isInteger(port) || port < 0 || port > 65535) {
    console.error(`invalid port: ${args[0] ?? process.env['PORT']}`);
    process.exit(1);
  }
  const host = process.env['HOST'] ?? '127.0.0.1';
  const server = await new MarkdownAstServer({ workers }).start(port, host);
  const address = server.address();
  console.log(`markdown-remark ast engine listening on port ${typeof address === 'object' && address ? address.port : port} (${workers} worker threads)`);
}

function workerCount(): number {
  const raw = process.env['WORKERS'];
  if (raw === undefined)
    return availableParallelism();
  const n = Number(raw);
  if (!Number.isInteger(n) || n < 0) {
    console.error(`invalid WORKERS: ${raw}`);
    process.exit(1);
  }
  return n;
}

/** Recursively re-prints every `.md` file under `root` in the engine's normalised style, one thread per file. */
async function convert(root: string, workers: number): Promise<void> {
  const entries = await readdir(root, { recursive: true, withFileTypes: true });
  const files = entries.filter((e) => e.isFile() && e.name.endsWith('.md')).map((e) => join(e.parentPath, e.name));
  const pool = new WorkerPool(Math.max(1, Math.min(workers, files.length)));
  let converted = 0;
  let failed = 0;
  await Promise.all(files.map(async (file) => {
    try {
      if ((await pool.submit({ op: 'normalize', path: file })).changed) {
        converted++;
        console.log(`converted ${file}`);
      }
    } catch (e) {
      failed++;
      console.error(`failed to convert ${file}: ${e instanceof Error ? e.message : String(e)}`);
    }
  }));
  await pool.close();
  console.log(`${files.length} files, ${converted} converted, ${failed} failed (${pool.size} worker threads)`);
}

await main(process.argv.slice(2));
