# Queue: Technical Specification (build order 4)

## 1. Task, restrictions and boundaries

**Task:** Provide a generic, unbounded, lock-free MPMC queue of work items (Rust). Each item has its own state, result and completion notification. Each queue instance also has its own set of notification channels.

**Uses:** The same type serves as task queue, CPU queue, GPU queue and shared queue. For operations queues, the caller uses its own metadata type and uses values as the payload type.

**Restrictions:**
- The queue knows no capabilities, priorities, compatible resources, models or modes. It only sees the caller's opaque `M`, `P`, `R` and `E`.
- There is no head-of-line blocking, no reservation and no pre-taking. An item is only claimed at the moment a consumer takes it.
- Instances share no state. Each instance has its own channels. There are no globals.
- Out of scope: scheduling, routing, cancellation, retry, persistence and bounded capacity.
- The module has no dependencies on other modules.
- Implementation: `std` only, plus optionally `crossbeam-epoch` or `crossbeam-utils`. No `unsafe` outside the lock-free core. Enqueue, claim and complete must not take a mutex or block. Blocking waits (`wait_*`) may park threads.

## 2. Context

Other modules create one `Queue` per role (task queue, CPU operations queue, GPU operations queue, shared operations queue). Producers `push` items and keep the returned `Ticket`. Consumers (workers) choose which item they can run by passing a predicate over the metadata to `take_where`. The consumer then executes the item and calls `Claim::complete`. The queue never interprets `M`. Compatibility decisions belong entirely to the caller's predicate.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Duration;

pub struct Queue<M, P, R, E> { /* private */ }
// Send + Sync + 'static-agnostic when M: Send + Sync, P: Send, R: Send, E: Send.
// Wrap in Arc for sharing; Queue is not Clone.

/// Item lifecycle. Transitions only forward: Pending -> Running -> {Succeeded|Failed|Abandoned}.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State { Pending, Running, Succeeded, Failed, Abandoned }

/// Per-instance notification channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Available, // an item was pushed
    Completed, // any item reached a final state (Succeeded/Failed/Abandoned)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultError {
    NotReady,   // item not in a final state yet
    Taken,      // result was already taken
    Abandoned,  // Claim dropped without complete()
}

impl<M, P, R, E> Queue<M, P, R, E>
where M: Send + Sync, P: Send, R: Send, E: Send
{
    pub fn new() -> Self;

    /// Enqueue at the tail. Never fails, never blocks. State = Pending. Bumps Channel::Available.
    pub fn push(&self, meta: M, payload: P) -> Ticket<M, P, R, E>;

    /// Claim the oldest Pending item (FIFO). None if no Pending item exists.
    pub fn take(&self) -> Option<Claim<M, P, R, E>>;

    /// Claim the oldest Pending item for which pred(&meta) is true.
    /// Items for which pred is false are skipped and left untouched (no reservation, no state change).
    pub fn take_where<F: FnMut(&M) -> bool>(&self, pred: F) -> Option<Claim<M, P, R, E>>;

    /// Number of Pending items (approximate under concurrency, exact when quiescent).
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    /// Current event counter of a channel (monotonic, starts at 0).
    pub fn epoch(&self, ch: Channel) -> u64;

    /// Block until epoch(ch) != seen or timeout expires (None = no timeout).
    /// Returns the current epoch. Returns immediately if it already differs from `seen`.
    /// Typical use: let e = epoch(ch); try take; if none -> wait_change(ch, e, t).
    pub fn wait_change(&self, ch: Channel, seen: u64, timeout: Option<Duration>) -> u64;
}

impl<M, P, R, E> Default for Queue<M, P, R, E> { /* = new() */ }

/// Producer-side handle to one item. Cheap clone (Arc). Send + Sync under the same bounds.
pub struct Ticket<M, P, R, E> { /* Arc to item */ }
impl<M, P, R, E> Clone for Ticket<M, P, R, E> { /* ... */ }

impl<M, P, R, E> Ticket<M, P, R, E> {
    pub fn state(&self) -> State;
    pub fn meta(&self) -> &M;
    /// Move the result out (once). Ok(Ok(r)) = success, Ok(Err(e)) = task error.
    pub fn take_result(&self) -> Result<Result<R, E>, ResultError>;
    /// Block until the item is in a final state or the timeout expires.
    /// Returns the state at return (final state, or non-final on timeout).
    pub fn wait(&self, timeout: Option<Duration>) -> State;
}

/// Exclusive consumer-side ownership of one Running item.
pub struct Claim<M, P, R, E> { /* private */ }
impl<M, P, R, E> Claim<M, P, R, E> {
    pub fn meta(&self) -> &M;
    /// Move the payload out (once). None on repeated call.
    pub fn take_payload(&mut self) -> Option<P>;
    /// Ticket for this item.
    pub fn ticket(&self) -> Ticket<M, P, R, E>;
    /// Store the result, set Succeeded (Ok) or Failed (Err), wake waiters, bump Channel::Completed.
    pub fn complete(self, result: Result<R, E>);
}
impl<M, P, R, E> Drop for Claim<M, P, R, E> { /* if not completed: state = Abandoned, notify like complete */ }
```

## 4. Behavior and invariants

**Ordering and claiming**
- Items are scanned in push order (FIFO per queue). With a single producer and a single consumer, `take` returns items exactly in push order.
- Each item is claimed by at most one consumer. The Pending→Running transition is atomic (CAS). Under concurrent `take`/`take_where`, no item is returned twice and none is lost.
- `take_where` never blocks on an item the predicate rejects. A later matching item is returned even if earlier ones do not match. Rejected items stay Pending and keep their position.
- The predicate may be called on items that another consumer claims concurrently. In that case the item is skipped. The predicate may be called more than once for the same item across calls. It must not have side effects the queue relies on.
- A claimed item is removed from the queue and is no longer counted in `len`. A Ticket keeps the item alive.
- There is no pre-taking. No API hands out an item before the consumer is ready to run it.

**State and result**
- State transitions are monotonic and forward-only (see `State`). A final state is never changed.
- `complete(Ok(r))` → `Succeeded`. `complete(Err(e))` → `Failed`. Dropping a `Claim` without `complete` → `Abandoned`, and `take_result` returns `Err(ResultError::Abandoned)`. The payload, if not taken, is dropped.
- `take_result` returns:
  - `Err(NotReady)` while Pending or Running.
  - The result exactly once after completion.
  - `Err(Taken)` on later calls.
- The result is stored before the final state is published (release/acquire). A thread that sees a final state also sees the result.
- Dropping a `Ticket` does not affect the item. Unconsumed results are dropped together with the last reference.
- Dropping the `Queue` drops Pending items (payloads and metadata). Outstanding Tickets and Claims remain valid.

**Notification**
- Each `Queue` has its own epoch counters for `Available` and `Completed`. These are independent of other queues.
- `push` increments `Available` after the item is visible. A consumer that read `epoch` before `take` and then calls `wait_change` with that value cannot miss a push (no lost wakeup).
- Completion or abandonment increments `Completed` and wakes `Ticket::wait` waiters for that item.
- `wait_change` and `Ticket::wait` may return spuriously only by timeout. They return early only when the condition holds.
- Epoch counters wrap on overflow (`u64`, wrapping add). Compare with `!=`.

**Concurrency**
- `push`, `take`, `take_where`, `len`, `epoch`, `Ticket::*` and `Claim::*` are safe to call from any thread concurrently. `push`, `take`, `take_where` and `complete` are lock-free.
- No panics on valid use. A panic in a caller's predicate leaves the queue consistent: the item stays Pending.
- There is no error type for `push` (unbounded). Allocation failure is the standard Rust abort.

**Edge cases**
- An empty queue makes `take` and `take_where` return `None`, and `len` return 0.
- If `pred` rejects all items, `take_where` returns `None`.
- `wait_change` with `Some(Duration::ZERO)` does not block.

## 5. Relied-on elements

None. This module has no dependencies.