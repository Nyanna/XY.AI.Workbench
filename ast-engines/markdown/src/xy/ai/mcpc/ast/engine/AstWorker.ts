import { parentPort } from 'node:worker_threads';
import { AstEngineException } from './AstEngineException.ts';
import type { Outcome, OutcomeMessage, TaskMessage } from './AstTask.ts';
import { DocumentService } from './DocumentService.ts';

/** Worker-thread entry point: owns one {@link DocumentService} and processes its tasks strictly in arrival order. */
const port = parentPort;
if (!port)
  throw new Error('AstWorker must run as a worker thread');

const service = new DocumentService();
let chain: Promise<void> = Promise.resolve();

port.on('message', (message: TaskMessage) => {
  chain = chain.then(async () => {
    port.postMessage({ id: message.id, outcome: await run(message) } satisfies OutcomeMessage);
  });
});

async function run({ task }: TaskMessage): Promise<Outcome> {
  try {
    return { ok: true, value: await service.execute(task) };
  } catch (e) {
    if (e instanceof AstEngineException)
      return { ok: false, error: { kind: e.kind, message: e.message, candidates: [...e.candidates] } };
    return { ok: false, fatal: e instanceof Error ? (e.stack ?? e.message) : String(e) };
  }
}
