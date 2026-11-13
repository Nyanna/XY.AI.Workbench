# Spec: Queue-Struktur und Enqueue (module `queue`, build order 25)

## 1. Task, restrictions and boundaries

Implement, in Rust, the queue data structure and its append operation.

- **Structure:** exactly one singly linked list with an internal sentinel node. There are no separate lists per priority or resource.
- **Enqueue:** append a node at the end with an atomic compare-and-swap (CAS) on the `next` pointer of the last node.
- **Signaling:** every enqueue signals all notification channels without blocking.

Restrictions:
- No `unsafe`. Allowed external crates: `arc-swap`, `log`, `tokio` (feature `sync`).
- Allowed project dependency: `queue_state` (`EntryState`).
- No version, snapshot or lock on the chain. Only the payload compartment inside `EntryState` is mutex-protected, and that mutex belongs to `queue_state`.
- Metadata is opaque. The queue never interprets, compares or clones it.
- Not in this module: claiming, filtering, selecting, removing entries, workers, scheduling. Other modules do those, using the read accessors below.
- Enqueue never blocks, waits or spins on a lock. It is lock-free in the CAS-retry sense.

## 2. Context

Producers call `Queue::enqueue(meta, state)` and keep the returned node as their handle to the entry. The `Arc<EntryState>` they pass in is the shared state object, and the caller keeps a clone as its own handle. The payload is already inside `EntryState` (`EntryState::new(payload)`). The node has no payload compartment of its own. Consumers (sub-engines, built later) traverse from the head with `first()` and `next()`, filter on `meta()`, and then call `claim` on `state()`. Each sub-engine owns one notification channel and is woken on every enqueue. A later Remove operation will unlink nodes by atomically swapping `next` pointers. The `next` pointer must therefore be an atomically swappable pointer, and the node fields must be reachable without exclusive access.

## 3. Interface

```rust
use std::sync::Arc;
use tokio::sync::Notify;
use queue_state::EntryState;

/// Notification channel: one per sub-engine. Signaled with `notify_one()` on every enqueue
/// (stores a permit, so a signal is never lost, and never blocks).
pub type NotifyChannel = Arc<Notify>;

pub struct QueueNode<M, P, T, E> { /* private: meta: M, state: Arc<EntryState<P,T,E>>,
                                     next: arc_swap::ArcSwapOption<QueueNode<..>> */ }

impl<M, P, T, E> QueueNode<M, P, T, E> {
    /// Immutable parameters for filtering before the claim. None for the internal sentinel node.
    pub fn meta(&self) -> Option<&M>;
    /// Shared state object. None for the internal sentinel node.
    pub fn state(&self) -> Option<&Arc<EntryState<P, T, E>>>;
    /// Current successor (atomic load); None at the end of the list.
    pub fn next(&self) -> Option<Arc<QueueNode<M, P, T, E>>>;
    pub fn is_sentinel(&self) -> bool;
}

pub struct Queue<M, P, T, E> { /* private: sentinel/head, tail hint, channels, test flag */ }

impl<M, P, T, E> Queue<M, P, T, E> {
    /// Any number of channels, including none (empty Vec). Creates the sentinel node;
    /// head and tail hint start at the sentinel.
    pub fn new(channels: Vec<NotifyChannel>) -> Self;

    /// Appends a new node at the end and returns it. Infallible, never blocks.
    /// Afterwards signals every channel (`notify_one`) exactly once.
    pub fn enqueue(&self, meta: M, state: Arc<EntryState<P, T, E>>) -> Arc<QueueNode<M, P, T, E>>;

    /// The internal sentinel node (start of traversal; it is not an entry). Its `next()` is the oldest entry.
    pub fn head(&self) -> Arc<QueueNode<M, P, T, E>>;
    /// First (oldest) entry node, equivalent to `head().next()`.
    pub fn first(&self) -> Option<Arc<QueueNode<M, P, T, E>>>;

    /// Test hook: false suppresses updates of the tail hint after enqueue (default true).
    pub fn set_tail_hint_update(&self, enabled: bool);
}
```

`Queue<M,P,T,E>` and `QueueNode<M,P,T,E>` are `Send + Sync` when `M: Send + Sync, P: Send, T: Send + Sync, E: Send + Sync`. `M` has no `Clone` or `Debug` bound.

## 4. Behavior and invariants

**Structure**
- The list is `sentinel → n1 → n2 → …`. The sentinel carries no meta and no state. It is never returned by `enqueue` and is never an entry.
- Order equals age, meaning the order of successful CASes. A new node is always appended at the end.
- Enqueues from one thread (one producer) appear in the list in their call order.
- `meta` and `state` of a node are fixed at creation and immutable. The new node's `next` is `None`.

**Enqueue algorithm**
1. Start at the tail hint. Walk `next` pointers to the real last node (`next == None`).
2. Do `compare_and_swap(None → new)` on that node's `next`.
3. If the CAS fails, continue the walk from the node observed in the failed CAS and retry. No entry is lost or overwritten, and no lock is taken.
4. After success, set the tail hint to the new node, unless updates are suppressed. A hint update is only an optimization and does not affect correctness.
5. Signal all channels. The signal happens after the append, so a woken consumer finds the entry.

- A stale tail hint (it points to a node that is not the last) never causes a lost entry or a wrong order. The real end is found by walking.
- With `set_tail_hint_update(false)` the hint stays at the sentinel (or its last value), and every enqueue must still append correctly at the real end. The hint may lag behind arbitrarily.
- With N concurrent producers and K enqueues each, the list afterwards holds exactly N·K entries, each exactly once. Each producer's entries keep their relative order.
- The returned node is the same node (`Arc::ptr_eq`) that is reachable in the list.

**Signaling**
- Every `enqueue` calls `notify_one()` on each channel, once per channel. The call is non-blocking and infallible.
- With an empty channel set, `enqueue` works and signals nothing.
- A consumer that is not yet waiting does not miss the signal, because `Notify` stores one permit. Several signals may coalesce into one permit.

**Memory and drop**
- Nodes are shared via `Arc`. A node stays alive while the list or any handle references it.
- The `Drop` of `Queue`, and of a node chain, must unlink iteratively. A long list must not overflow the stack through recursive drop.
- Entry states are not touched when the queue is dropped.

**Errors**
- There are no error types. `enqueue` cannot fail. Panics occur only on internal bugs.

## 5. Relied-on elements

From `queue_state` (used as specified, not redefined):
- `EntryState<P,T,E>`: `new(payload) -> Arc<Self>`, `sentinel() -> Arc<Self>`, `state()`, `result()`, `claim()`, `complete()`, `cancel()`, `wait_finished()`.
- `QueueState { Enqueued, Claimed, Completed, Cancelled }`.
- `claim_entry(&EntryState) -> Result<P, ClaimError>`.
- `EntryState<P,T,E>` is `Send + Sync` if `P: Send, T: Send + Sync, E: Send + Sync`. The payload compartment (`Mutex<Option<P>>`) lives inside `EntryState`. This module does not read or lock it.
- The queue never changes the state of an entry. It only stores the `Arc`.