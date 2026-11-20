import { resolve } from 'node:path';
import { Worker } from 'node:worker_threads';
import { AstEngineException } from './AstEngineException.ts';
import type { Outcome, OutcomeMessage, ResultOf, Task, TaskExecutor, TaskMessage } from './AstTask.ts';
import { DocumentService } from './DocumentService.ts';

function unwrap<T extends Task>(outcome: Outcome): ResultOf<T> {
  if (outcome.ok)
    return outcome.value as ResultOf<T>;
  if ('fatal' in outcome) {
    console.error(outcome.fatal);
    throw new Error(outcome.fatal);
  }
  throw new AstEngineException(outcome.error.kind, outcome.error.message, outcome.error.candidates);
}

/** Runs every task on the calling thread, one at a time. For `WORKERS=0`, debugging and tests. */
export class InlineExecutor implements TaskExecutor {
  private readonly service = new DocumentService();
  private chain: Promise<unknown> = Promise.resolve();

  submit<T extends Task>(task: T): Promise<ResultOf<T>> {
    const result = this.chain.then(() => this.service.execute(task)) as Promise<ResultOf<T>>;
    this.chain = result.catch(() => undefined);
    return result;
  }

  async close(): Promise<void> {}
}

interface Slot {
  index: number;
  worker: Worker;
  readonly pending: Map<number, { resolve: (o: Outcome) => void; reject: (e: Error) => void }>;
  files: number;
  dead: boolean;
}

/**
 * A fixed pool of worker threads, each with its own parser and document cache. A file path is pinned to
 * one worker (the least loaded at first sight), so at most one thread ever works on a file, tasks for it
 * run in submission order and its cached tree stays warm. Tasks without a path go to the least loaded worker.
 */
export class WorkerPool implements TaskExecutor {
  private readonly slots: Slot[] = [];
  private readonly affinity = new Map<string, Slot>();
  private nextId = 0;
  private closed = false;

  constructor(size: number) {
    for (let i = 0; i < size; i++)
      this.slots.push(this.spawn(i));
  }

  get size(): number {
    return this.slots.length;
  }

  submit<T extends Task>(task: T): Promise<ResultOf<T>> {
    if (this.closed)
      return Promise.reject(new Error('worker pool is closed'));
    const key = task.path === null || task.path.trim() === '' ? null : resolve(task.path);
    const slot = key === null ? this.leastLoaded() : this.pin(key);
    const id = this.nextId++;
    return new Promise<Outcome>((resolveOutcome, reject) => {
      slot.pending.set(id, { resolve: resolveOutcome, reject });
      slot.worker.postMessage({ id, task } satisfies TaskMessage);
    }).then((outcome) => unwrap<T>(outcome));
  }

  async close(): Promise<void> {
    this.closed = true;
    await Promise.all(this.slots.map((slot) => slot.worker.terminate()));
  }

  private pin(key: string): Slot {
    let slot = this.affinity.get(key);
    if (!slot) {
      slot = this.leastLoaded();
      slot.files++;
      this.affinity.set(key, slot);
    }
    return slot;
  }

  private leastLoaded(): Slot {
    return this.slots.reduce((best, s) => (s.pending.size < best.pending.size || (s.pending.size === best.pending.size && s.files < best.files) ? s : best));
  }

  private spawn(index: number): Slot {
    const slot: Slot = { index, worker: new Worker(new URL('./AstWorker.ts', import.meta.url)), pending: new Map(), files: 0, dead: false };
    slot.worker.on('message', ({ id, outcome }: OutcomeMessage) => {
      const entry = slot.pending.get(id);
      slot.pending.delete(id);
      entry?.resolve(outcome);
    });
    slot.worker.on('error', (e: unknown) => this.retire(slot, e instanceof Error ? e : new Error(String(e))));
    slot.worker.on('exit', (code) => this.retire(slot, new Error(`worker exited with code ${code}`)));
    return slot;
  }

  /** A crashed worker fails its in-flight tasks and is replaced; its pinned files are re-pinned on next use. */
  private retire(slot: Slot, cause: Error): void {
    if (slot.dead)
      return;
    slot.dead = true;
    for (const entry of slot.pending.values())
      entry.reject(cause);
    slot.pending.clear();
    for (const [key, pinned] of this.affinity)
      if (pinned === slot)
        this.affinity.delete(key);
    if (!this.closed)
      this.slots[slot.index] = this.spawn(slot.index);
  }
}
