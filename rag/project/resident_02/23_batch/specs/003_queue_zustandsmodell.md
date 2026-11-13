# Spec: Queue-Zustandsmodell (module `queue_state`, build order 3)

## 1. Task, restrictions and boundaries

Implement, in Rust, the shared state object of a queue entry. It is also the caller's handle. The module covers:
- the four states `Enqueued`, `Claimed`, `Completed` and `Cancelled`
- the atomic transitions Claim, Completion and Cancel
- one-time ownership transfer of the payload
- a write-once result that is visible before the `Completed` state
- notification of waiters

Restrictions:
- No queue data structure, worker pool, scheduling or timeouts. Those belong to other modules.
- No dependencies on other project modules.
- External crates: `log` (logging) and `tokio` (feature `sync`, for `tokio::sync::Notify`).
- No `unsafe`.
- Nothing may block. There is no blocking wait anywhere in this module.

## 2. Context

The queue module creates one `Arc<EntryState<P,T,E>>` per entry. The caller keeps one clone as the handle and the queue gives the other to workers. A worker does the following:
1. It claims the entry and receives the payload by move.
2. It runs the work.
3. It completes the entry with a `Result`.

The caller can cancel the entry as long as it is `Enqueued`. It can poll the state and result, or wait for completion or cancellation. The queue module may also call the claim free function (`claim_entry`). A sentinel entry is an entry created without a payload, for example as a stop marker. Claiming it must always fail.

## 3. Interface

```rust
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueState { Enqueued, Claimed, Completed, Cancelled }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError {
    /// Entry was not Enqueued (already Claimed/Completed) – carries the observed state.
    NotEnqueued(QueueState),
    /// Entry was cancelled.
    Cancelled,
    /// Entry has no payload (sentinel); state is left unchanged.
    NoPayload,
    /// Payload compartment was locked at the moment of the attempt; state unchanged.
    Busy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteError {
    /// State was not Claimed (carries the observed state); result unchanged.
    NotClaimed(QueueState),
}

pub struct EntryState<P, T, E> { /* private: AtomicU8 state, Mutex<Option<P>> payload,
                                    OnceLock<Result<T,E>> result, tokio Notify */ }

impl<P, T, E> EntryState<P, T, E> {
    /// New entry in state Enqueued holding Some(payload).
    pub fn new(payload: P) -> Arc<Self>;
    /// Sentinel: state Enqueued, payload compartment None.
    pub fn sentinel() -> Arc<Self>;

    /// Consistent snapshot. Panics (unrecoverable internal error) if the internal value is invalid.
    pub fn state(&self) -> QueueState;
    /// Some only once the result has been set; never for Cancelled entries.
    pub fn result(&self) -> Option<&Result<T, E>>;

    /// Enqueued -> Claimed. Moves the payload out. Never blocks.
    pub fn claim(&self) -> Result<P, ClaimError>;
    /// Claimed -> Completed. Stores result first, then publishes the state.
    pub fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError>;
    /// Enqueued -> Cancelled. true = success, false = no change.
    pub fn cancel(&self) -> bool;

    /// Resolves with the final state (Completed or Cancelled) once reached;
    /// returns immediately if already final. Must not miss a wake-up
    /// (register the Notified future before checking the state).
    pub async fn wait_finished(&self) -> QueueState;
}

/// Queue-level claim function; identical behavior to `EntryState::claim` (method delegates to it or vice versa).
pub fn claim_entry<P, T, E>(entry: &EntryState<P, T, E>) -> Result<P, ClaimError>;
```

`EntryState<P,T,E>` is `Send + Sync` whenever `P: Send, T: Send + Sync, E: Send + Sync` (this follows from the chosen fields). `P` need not be `Sync` or `Clone`.

## 4. Behavior and invariants

**States and transitions**
- A new entry is `Enqueued`.
- The only transitions are Enqueued→Claimed, Claimed→Completed and Enqueued→Cancelled. `Completed` and `Cancelled` are final.
- Every transition is a single atomic compare-exchange on the state. With concurrent attempts at most one succeeds.
- Claim and Cancel on the same entry are mutually exclusive. The final state always matches the winner.
- Once a worker has claimed an entry, `cancel()` returns false.
- Once an entry is cancelled, `claim()` fails.
- The state is stored as a `u8`. An invalid value makes `state()` and all internal reads panic with a clear message. Do not map it to an error or a default.

**Claim**
- The order is: `try_lock` the payload compartment, then compare-exchange the state Enqueued→Claimed, then `take()` the payload.
- A successful claim returns the payload by move (never a copy), exactly once. The compartment is `None` afterwards.
- If `try_lock` fails (the compartment is locked), return `Err(Busy)` at once. Never wait or spin. The state stays unchanged.
- A mutex poisoned by a panic is not a failure of its own. Recover the inner value.
- Sentinel (payload `None`) returns `Err(NoPayload)` and the state stays `Enqueued`.
- Claiming an entry that is Claimed or Completed returns `Err(NotEnqueued(observed))`. A Cancelled entry returns `Err(Cancelled)`. No payload is returned in any failure case.
- In every failure case the state is unchanged.
- Under concurrent claims exactly one wins, and the others get a failure.
- Every claim attempt is logged with its outcome through the `log` crate: `debug!` or `info!` for success, `debug!` for failure.
- The free function and the method behave identically.

**Completion**
- Only valid from `Claimed`. The claimer is the only party that is supposed to call it. It is not enforced beyond the state check.
- The sequence is:
  1. Check that the state is `Claimed`.
  2. Set the `OnceLock` result.
  3. Store the state `Completed` (Release ordering).
  4. Call `notify_waiters()`.
- A reader that observes `Completed` (Acquire ordering) always sees the result. `result()` may already return `Some` while the state is still `Claimed`.
- A second completion attempt, whether concurrent or later, leaves the first result unchanged and returns `Err(NotClaimed(Completed))`.
  - It logs `warn!`.
  - It runs `debug_assert!(false, ..)`, so it panics in debug builds (a programming error) and does not panic in release builds.
- Completion on an `Enqueued` or `Cancelled` entry returns `Err(NotClaimed(state))`, logs `warn!`, does not panic, and sets no result.
- The result can be set exactly once. It is never set on a cancelled entry, so `result()` stays `None` there.

**Cancel**
- Returns `true` only for a successful Enqueued→Cancelled. It then calls `notify_waiters()`.
- In every other state it returns `false`, changes nothing and sends no notification.
- Every attempt is logged with its outcome.
- The payload is not touched by cancel. It stays in the compartment and is dropped with the entry.

**Waiting**
- `wait_finished` returns `Completed` or `Cancelled`.
- All waiters present at the time of a successful Completion or Cancel are woken.
- Waiters that arrive later return immediately because they check the state.

## 5. Relied-on elements

None (no dependencies).