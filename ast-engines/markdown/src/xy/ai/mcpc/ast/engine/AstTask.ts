import type { ErrorKind } from './AstEngineException.ts';

/** One unit of document work. Plain data only: it crosses the worker-thread boundary by structured clone. */
export type Task =
  | { op: 'append'; path: string | null; source: string | null; code: string | null }
  | { op: 'list'; path: string | null; source: string | null; includeCode: boolean }
  | { op: 'get'; path: string | null; source: string | null; nodeId: string }
  | { op: 'delete'; path: string | null; source: string | null; nodeId: string }
  | { op: 'insert'; path: string | null; source: string | null; nodeId: string; code: string | null; position: string | null }
  | { op: 'replace'; path: string | null; source: string | null; nodeId: string; code: string | null }
  | { op: 'validate'; path: string | null; source: string | null }
  | { op: 'normalize'; path: string };

/** JSON shape of the contract's `Node` object. */
export type NodeRecord = Record<string, string | number | boolean>;

export interface Results {
  append: { source: string; units: number };
  list: { nodes: NodeRecord[] };
  get: NodeRecord;
  delete: { source: string };
  insert: { source: string; units: number };
  replace: { source: string };
  validate: { error: string | null };
  normalize: { changed: boolean };
}

export type ResultOf<T extends Task> = Results[T['op']];

export type Outcome =
  | { ok: true; value: unknown }
  | { ok: false; error: { kind: ErrorKind; message: string; candidates: string[] } }
  | { ok: false; fatal: string };

export interface TaskMessage {
  id: number;
  task: Task;
}

export interface OutcomeMessage {
  id: number;
  outcome: Outcome;
}

/** Runs tasks; a task for a given file is never processed by two threads at once. */
export interface TaskExecutor {
  /** Resolves with the result; rejects with an `AstEngineException` for user-facing errors, any other error otherwise. */
  submit<T extends Task>(task: T): Promise<ResultOf<T>>;
  close(): Promise<void>;
}
