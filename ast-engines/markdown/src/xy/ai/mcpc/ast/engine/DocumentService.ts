import { readFile, stat, writeFile } from 'node:fs/promises';
import type { Root } from 'mdast';
import type { AddressableNode } from './AddressableNode.ts';
import { AstEngineException } from './AstEngineException.ts';
import type { NodeRecord, Results, Task } from './AstTask.ts';
import { DocumentCache } from './DocumentCache.ts';
import { MarkdownAstEngine } from './MarkdownAstEngine.ts';
import { locateAll } from './NodeLocator.ts';

interface Resolved {
  readonly root: Root;
  readonly path: string | null;
}

/**
 * The transport-free document operations: engine + cache + `path`/`source` resolution. A request either
 * names a `path` (cached, persisted to disk on every mutation) or carries a stateless `source` (parsed
 * in-memory only, never written anywhere); when both are given for an existing file, a mismatch is
 * reported as a conflict (the client's view of the file is stale). One instance lives per worker thread;
 * callers must feed it one task at a time.
 */
export class DocumentService {
  private readonly engine = new MarkdownAstEngine();
  private readonly cache = new DocumentCache(this.engine);

  async execute(task: Task): Promise<Results[Task['op']]> {
    switch (task.op) {
      case 'append':
        return this.append(task);
      case 'list':
        return this.list(task);
      case 'get':
        return this.get(task);
      case 'delete':
        return this.delete(task);
      case 'insert':
        return this.insert(task);
      case 'replace':
        return this.replace(task);
      case 'validate':
        return this.validate(task);
      case 'normalize':
        return this.normalize(task);
    }
  }

  private async append(task: Extract<Task, { op: 'append' }>): Promise<Results['append']> {
    const resolved = await this.resolve(task.path, task.source, true);
    const units = this.engine.append(resolved.root, task.code);
    return { source: await this.persist(resolved), units };
  }

  private async list(task: Extract<Task, { op: 'list' }>): Promise<Results['list']> {
    const { root } = await this.resolve(task.path, task.source, false);
    return { nodes: locateAll(root, this.engine).map((a) => this.toRecord(root, a, task.includeCode)) };
  }

  private async get(task: Extract<Task, { op: 'get' }>): Promise<Results['get']> {
    const { root } = await this.resolve(task.path, task.source, false);
    return this.toRecord(root, this.find(root, task.nodeId), true);
  }

  private async delete(task: Extract<Task, { op: 'delete' }>): Promise<Results['delete']> {
    const resolved = await this.resolve(task.path, task.source, true);
    this.engine.delete(resolved.root, this.find(resolved.root, task.nodeId));
    return { source: await this.persist(resolved) };
  }

  private async insert(task: Extract<Task, { op: 'insert' }>): Promise<Results['insert']> {
    const position = task.position ?? 'after';
    if (position !== 'before' && position !== 'after')
      throw new AstEngineException('SYNTAX', "position must be 'before' or 'after'");
    const resolved = await this.resolve(task.path, task.source, true);
    const units = this.engine.insert(resolved.root, this.find(resolved.root, task.nodeId), task.code, position);
    return { source: await this.persist(resolved), units };
  }

  private async replace(task: Extract<Task, { op: 'replace' }>): Promise<Results['replace']> {
    const resolved = await this.resolve(task.path, task.source, true);
    this.engine.replace(resolved.root, this.find(resolved.root, task.nodeId), task.code);
    return { source: await this.persist(resolved) };
  }

  private async validate(task: Extract<Task, { op: 'validate' }>): Promise<Results['validate']> {
    try {
      let source = task.source;
      if (source === null) {
        if (task.path === null || task.path.trim() === '')
          throw new AstEngineException('SYNTAX', "either 'path' or 'source' is required");
        source = await readFile(task.path, 'utf8');
      }
      return { error: this.engine.validate(source) };
    } catch (e) {
      return { error: e instanceof AstEngineException ? e.message : `cannot read ${task.path}: ${messageOf(e)}` };
    }
  }

  /** Re-prints a file in the engine's normalised style; writes only if the text changes. */
  private async normalize(task: Extract<Task, { op: 'normalize' }>): Promise<Results['normalize']> {
    const source = await readFile(task.path, 'utf8');
    const normalized = this.engine.print(this.engine.parseDocument(source));
    if (normalized === source)
      return { changed: false };
    await writeFile(task.path, normalized, 'utf8');
    this.cache.invalidate(task.path);
    return { changed: true };
  }

  private async resolve(path: string | null, sourceOverride: string | null, mutable: boolean): Promise<Resolved> {
    if (path !== null && path.trim() !== '') {
      if (await exists(path)) {
        let entry;
        try {
          entry = await this.cache.get(path);
        } catch (e) {
          if (e instanceof AstEngineException)
            throw e;
          throw new AstEngineException('SYNTAX', `cannot read ${path}: ${messageOf(e)}`);
        }
        if (sourceOverride !== null && sourceOverride !== entry.source)
          throw new AstEngineException('CONFLICT', `file changed on disk since the given source was captured: ${path}`);
        return { root: mutable ? structuredClone(entry.root) : entry.root, path };
      }
      return { root: sourceOverride !== null ? this.engine.parseDocument(sourceOverride) : this.engine.emptyDocument(), path };
    }
    if (sourceOverride === null)
      throw new AstEngineException('SYNTAX', "either 'path' or 'source' is required");
    return { root: this.engine.parseDocument(sourceOverride), path: null };
  }

  private async persist(resolved: Resolved): Promise<string> {
    if (resolved.path === null)
      return this.engine.print(resolved.root);
    try {
      return await this.cache.save(resolved.path, resolved.root);
    } catch (e) {
      if (e instanceof AstEngineException)
        throw e;
      throw new AstEngineException('SYNTAX', `cannot write ${resolved.path}: ${messageOf(e)}`);
    }
  }

  private find(root: Root, nodeId: string): AddressableNode {
    const located = locateAll(root, this.engine);
    const hit = located.find((a) => a.id === nodeId);
    if (hit)
      return hit;
    const candidates = located.map((a) => a.id).filter((id) => id.includes(nodeId) || nodeId.includes(id)).slice(0, 5);
    throw new AstEngineException('NOT_FOUND', `node not found: ${nodeId}`, candidates);
  }

  private toRecord(root: Root, a: AddressableNode, includeCode: boolean): NodeRecord {
    const record: NodeRecord = { id: a.id, type: a.type };
    if (a.name !== null)
      record['name'] = a.name;
    record['lineno'] = a.lineno;
    record['endLineno'] = a.endLineno;
    record['parentType'] = a.parentType;
    record['expandable'] = a.expandable;
    record['isDefinition'] = a.isDefinition;
    if (a.isDefinition) {
      record['signature'] = this.engine.signature(root, a.range, 80);
      const doc = this.engine.docstring(root, a.range, 80);
      if (doc !== null)
        record['docstring'] = doc;
    }
    if (includeCode)
      record['code'] = this.engine.printRange(root, a.range);
    return record;
  }
}

async function exists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}

function messageOf(e: unknown): string {
  return e instanceof Error ? e.message : String(e);
}
