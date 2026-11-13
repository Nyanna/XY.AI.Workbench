# Processor Parking-Lot Removal: Technical Specification (build order 56a)

## 1. Task, restrictions and boundaries

**Task:** Remove the unused parking lot from the `Processor` and align the affected specifications. Parking and waking of worker threads belongs only to `worker_pool` (build order 56).

Removed from the `processor` module:
- `WorkerPool`, `WorkerId`
- `Notification`
- `IdleNotifications`
- the accessors `Processor::pool()`, `Processor::notification()`, `Processor::idle()`
- the internal state behind them (pool state, notify core, idle core) and the helpers only they use

**Restrictions:**
- `std` only. No `unsafe`.
- No behavior of `submit`, `expand`, `task_queue` or `ops_queue` other than the notification bump changes.
- `worker_pool` is not modified.
- Out of scope: wiring of `Planner::set_idle`, the Facade, any new notification mechanism.

## 2. Context

- Processor (build order 37) specified a parking lot, a `Notification` and `IdleNotifications`. No module, spec or code outside the Processor and its tests uses them.
- Worker-Pool (build order 56) owns the only worker wake-up channel (`Notifier`). The Facade (build order 135) supplies the `WorkSource` and calls `Notifier::notify()` after each `submit`.
- Sub-engines (build order 134) wait on `Queue::wait_change`. Operations pushed by `expand` reach them through the queue channels, not through a `Notification`.
- Idle signals for expansion control come from `report_iteration` of the backpressure module (build order 130), not from the Processor. The condition "operations queue empty" is checked by the consumer through `Queue::len()`.

## 3. Interface

Public surface of `processor` after the change:

```rust
pub enum QueueKind { Cpu, Gpu, Shared }
pub struct PlannedOp<O, V> { pub kind: QueueKind, pub meta: O, pub values: V }
pub trait Planner<D, T, E, O, V>: Send + Sync { /* unchanged */ }
pub enum ExpandError<PE> { Planner(PE) }
pub type TaskQueue<D, T, E> = Queue<TaskParams, Task<D, T, E>, (), ()>;
pub type OpsQueue<O, V, R, OE> = Queue<O, V, R, OE>;

pub struct Processor<D, T, E, O, V, R, OE, PL> { /* private */ }

impl<D, T, E, O, V, R, OE, PL> Processor<D, T, E, O, V, R, OE, PL>
where PL: Planner<D, T, E, O, V> {
    pub fn new(planner: PL) -> Arc<Self>;
    pub fn submit(&self, data: D, params: TaskParams) -> Task<D, T, E>;
    pub fn expand(&self, task: &Task<D, T, E>)
        -> Result<Vec<(QueueKind, Ticket<O, V, R, OE>)>, ExpandError<PL::Error>>;
    pub fn task_queue(&self) -> &TaskQueue<D, T, E>;
    pub fn ops_queue(&self, kind: QueueKind) -> &OpsQueue<O, V, R, OE>;
}
```

No other public item of the module changes.

## 4. Behavior and invariants

**Code**
- `Processor::new` creates the four queues and holds the planner. It creates no pool, notification or idle state.
- `submit` pushes one item into the task queue. The queue bumps its own `Channel::Available`. Nothing else is notified.
- `expand` calls `planner.expand(task)` exactly once and pushes every `PlannedOp` to the queue of its `kind`, in Planner order. Nothing else is notified. On `Err(e)` nothing is pushed and the result is `Err(ExpandError::Planner(e))`. An empty result is `Ok(vec![])`.
- All other invariants of build order 37 stay unchanged: four independent queues, stable accessors, the task is never claimed, completed or cancelled by the Processor, `submit` and `expand` never block, and concurrent calls are safe.
- Imports used only by the removed items (`Condvar`, `HashSet`, `AtomicU64` and similar) are removed. No unused-import or dead-code warning remains.

**Tests**
- Remove every test of `pool()`, `notification()` and `idle()`: registration, parking, epoch and timeout rules, wake-all, idle epoch rules, and tests that read the notification epoch after `submit` or `expand`.
- Keep and, where needed, reduce to queue assertions: submit state and queue content, expand routing and order, expand on empty and error, queue independence.

**Specifications to update**
- Build order 37 (Processor):
  - Remove the WorkerPool, Notification and Idle-notifications items from §1, §3 and §4.
  - In §2, replace the sentence on parking with: workers take from the queues, and waking is done by `worker_pool::Notifier`, which the caller of `submit` triggers.
  - Remove the notification bump from the `submit` and `expand` rules. Keep the queue channel rules.
- Build order 56 (Worker-Pool), §2: the Facade implements `WorkSource`, not the Processor. Remove "The Processor owns the queues ... It implements `WorkSource`" and state that the Processor only owns the queues.
- Build order 56 (Worker-Pool), §5: replace "The Processor must call `Notifier::notify()`" with "The owner of the `WorkSource` (the Facade) must call it".
- Build order 135 (Engine-Fassade), §5: remove the note that `processor::WorkerPool` is a different type.
- Module descriptions (same edit in each file; the Processor entry is module 36 in `23_batch/modules.yaml`, and the same entry in `15_corpus2modules/01.yml` and `16_modules_optimized/01.yml`):
  - Replace "Besitzt die Task-Queue und drei getrennte Operations-Queues (CPU, GPU, gemeinsam), den Parkplatz der Worker (Worker-Pool), die Notification und die Idle-Notifications." with a sentence that names only the task queue and the three operations queues.
  - In `modules.yaml` module 36, replace "Task-Queue, Operations-Queues, Worker-Pool, Notification und Idle-Notifications besitzen" with "Task-Queue und Operations-Queues besitzen".
  - In `modules.yaml` module 36, remove or rephrase "Orchestriert mit Workern, Queues, Notification und Backpressure." so that it no longer names workers or notification.
  - In the two `.yml` files, rephrase "Worker parken im Processor und wachen bei jeder Zustandsänderung per Notification auf." to state that workers park in the Worker-Pool and are woken by its `Notifier`.
  - The module list entry `Worker-Pool` stays as a child of Engine-Fassade. No other module description changes.
- `25_implement/index.md` is the generic implementation prompt and needs no entry for this build order.

**Errors**
- No new error type or function. Public API is only reduced.

**Concurrency**
- Unchanged. `submit`, `expand`, `task_queue` and `ops_queue` stay callable from any thread concurrently.

## 5. Relied-on elements

From Queue:
- `push`, `take`, `take_where`, `len`, `wait_change`. `push` bumps `Channel::Available` of that queue.

From Task-Modell:
- `Task` handle and `TaskParams`, unchanged.

From Worker-Pool (informational):
- `Notifier::notify()` is the only wake-up for dispatcher workers. The Facade calls it after `submit`.

From Backpressure (informational):
- Idle signals come from `report_iteration`. The Processor provides none.

Verification:
- The crate builds without warnings and the whole test run passes.
- A search for `WorkerId`, `IdleNotifications`, `.pool()`, `.notification()` and `.idle()` finds no remaining use.
