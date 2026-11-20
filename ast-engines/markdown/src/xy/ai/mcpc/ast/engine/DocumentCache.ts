import { createHash } from 'node:crypto';
import { readFile, stat, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import type { Root } from 'mdast';
import type { MarkdownAstEngine } from './MarkdownAstEngine.ts';

/** The parsed tree plus the exact text it was parsed from. Callers must not mutate `root`; clone it first. */
export interface DocumentEntry {
  readonly root: Root;
  readonly source: string;
}

interface CacheEntry extends DocumentEntry {
  readonly mtimeMs: number;
  readonly size: number;
  readonly contentHash: string;
}

/**
 * Timestamp/content-hash validated cache of parsed documents, keyed by absolute path: a cheap
 * mtime+size check first, a SHA-256 content check only on mismatch, and a real re-parse only if the
 * content actually changed outside this process. Every save writes the printed source back to disk
 * and refreshes the cache from that exact text, so reported line numbers always match what's on disk.
 */
export class DocumentCache {
  private readonly entries = new Map<string, CacheEntry>();
  private readonly engine: MarkdownAstEngine;

  constructor(engine: MarkdownAstEngine) {
    this.engine = engine;
  }

  async get(path: string): Promise<DocumentEntry> {
    const key = resolve(path);
    const entry = this.entries.get(key);
    const attrs = await stat(key);
    if (entry && entry.mtimeMs === attrs.mtimeMs && entry.size === attrs.size)
      return entry;
    const source = await readFile(key, 'utf8');
    const contentHash = sha256(source);
    if (entry && entry.contentHash === contentHash) {
      const refreshed = { ...entry, mtimeMs: attrs.mtimeMs, size: attrs.size };
      this.entries.set(key, refreshed);
      return refreshed;
    }
    const loaded = { root: this.engine.parseDocument(source), source, mtimeMs: attrs.mtimeMs, size: attrs.size, contentHash };
    this.entries.set(key, loaded);
    return loaded;
  }

  /** Serialises `root`, writes it to `path` and refreshes the cache entry from that text. */
  async save(path: string, root: Root): Promise<string> {
    const key = resolve(path);
    const source = this.engine.print(root);
    await writeFile(key, source, 'utf8');
    const attrs = await stat(key);
    this.entries.set(key, { root: this.engine.parseDocument(source), source, mtimeMs: attrs.mtimeMs, size: attrs.size, contentHash: sha256(source) });
    return source;
  }

  invalidate(path: string): void {
    this.entries.delete(resolve(path));
  }
}

function sha256(text: string): string {
  return createHash('sha256').update(text, 'utf8').digest('hex');
}
