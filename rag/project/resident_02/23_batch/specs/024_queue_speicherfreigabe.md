# Spec: Queue-Speicherfreigabe (module `queue_reclaim`, build order 24)

## 1. Task, restrictions and boundaries

Implement, in Rust, the node type of the queue chain and its reference-counted release.

- The module frees nodes through `Arc` reference counting.
- It provides an optional best-effort unlink by compare-and-swap.
- It tears down long chains iteratively.

Restrictions:
- **No `unsafe` in this module's code.** The one allowed component for the atomic pointer swap is the `Link` type. `Link` is built on `arc_swap::ArcSwapOption`, so it also contains no `unsafe` of its own. Everything else is safe Rust.
- Allowed external crates: `arc_swap`, `log`, and `queue_state` (the dependency).
- Not allowed: epoch-based reclamation, slabs, freelists, generation counters, and hazard pointers.
- Not part of this module: queue enqueue/dequeue logic, worker pools, scheduling, and entry state transitions.
- Nothing blocks. Do not use locks or spin loops.

## 2. Context

- Ownership chain: queue head (`Link`) → chain of `Node`s via `Node::next` (`Link`) → iterators.
- Each iterator holds an `Arc<Node>` for the node it is currently at, taking one reference per step.
- Each `Node` holds the `Arc<EntryState<P,T,E>>` of its entry. `EntryState` comes from `queue_state`.
- After a claim, the emptied node stays in the chain.
- The node is freed automatically when it is unlinked (or the queue is dropped) and the last iterator has left it.
- Unlinked nodes keep their `next`, so an iterator standing on one continues without a restart.
- The queue module (a neighbor) creates nodes, appends them with `Link::compare_and_swap`, owns the head `Link`, and calls `unlink` as optional cleanup.

## 3. Interface

```rust
use std::sync::Arc;
use queue_state::{EntryState, QueueState};

/// Atomic, shareable pointer slot to an optional node (queue head or Node::next).
/// Wraps arc_swap::ArcSwapOption<Node<P,T,E>>.
pub struct Link<P, T, E> { /* private */ }

impl<P, T, E> Link<P, T, E> {
    pub fn empty() -> Self;
    pub fn new(node: Arc<Node<P, T, E>>) -> Self;
    /// Clone of the current target (one extra strong reference), None if empty.
    pub fn load(&self) -> Option<Arc<Node<P, T, E>>>;
    /// Unconditional atomic swap; returns the previous target.
    pub fn swap(&self, new: Option<Arc<Node<P, T, E>>>) -> Option<Arc<Node<P, T, E>>>;
    /// CAS by pointer identity (Arc::ptr_eq; None matches None).
    /// Ok(()) = swapped. Err(actual) = current target, nothing changed.
    pub fn compare_and_swap(
        &self,
        expected: Option<&Arc<Node<P, T, E>>>,
        new: Option<Arc<Node<P, T, E>>>,
    ) -> Result<(), Option<Arc<Node<P, T, E>>>>;
}

impl<P, T, E> Default for Link<P, T, E> { /* = empty() */ }

pub struct Node<P, T, E> { /* private: entry: Arc<EntryState<P,T,E>>, next: Link<P,T,E> */ }

impl<P, T, E> Node<P, T, E> {
    pub fn new(entry: Arc<EntryState<P, T, E>>) -> Arc<Self>;
    pub fn entry(&self) -> &Arc<EntryState<P, T, E>>;
    /// The successor slot (queue appends via next().compare_and_swap(None, ..)).
    pub fn next(&self) -> &Link<P, T, E>;
}

impl<P, T, E> Drop for Node<P, T, E> { /* iterative chain teardown, see §4 */ }

/// Optional cleanup: remove `node` from the chain behind `pred`.
/// Returns true only if this call's CAS removed it.
pub fn unlink<P, T, E>(
    pred: &Link<P, T, E>,
    node: &Arc<Node<P, T, E>>,
) -> bool;
```

Bounds:
- `Link` and `Node` are `Send + Sync` when `P: Send, T: Send + Sync, E: Send + Sync`. `P` need not be `Sync`.
- Do not add stronger bounds than this on the types or on the methods.

## 4. Behavior and invariants

**Reference counting**
- Memory is owned only through `Arc<Node>` strong counts. There are no raw pointers and no manual frees.
- A node is freed exactly when its last strong reference is dropped. Those references are the predecessor's `Link`, the head `Link`, and any iterator or caller clone.
- `Link::load` yields an owned `Arc`, so a node that an iterator holds stays valid even if it is unlinked or the queue is dropped.
- A node's `entry` is dropped together with the node, which drops its payload if it was never claimed. A node not unlinked is freed at the latest when the queue (head `Link`) is dropped, provided no iterator still holds it.

**Unlink**
- `unlink` performs a single `compare_and_swap(Some(node), node.next().load())` on `pred`.
- Preconditions that make it return `false` without change:
  - `node.entry().state()` is `Enqueued`. Only claimed, completed or cancelled nodes are unlinked.
  - `node.next()` is empty, meaning the node is the tail. Unlinking it could lose a concurrent append.
  - `pred` no longer points to `node`. This includes the case where it was already unlinked, so a repeated `unlink` returns `false`.
- The unlinked node's own `next` is never modified. An iterator on it continues through `next()` to the old successor without restarting.
- `unlink` is best-effort and never retries or blocks. When two adjacent nodes are unlinked concurrently, a node may stay linked. That node is an empty husk and is freed later like any other node. No node is lost or double-freed.
- A failed `unlink` is not an error. Do not panic and do not log above `debug!`.

**Iterative teardown (`Drop for Node`)**
- Dropping a chain of any length (≥ 1,000,000 nodes) must not use recursion proportional to its length and must not overflow the stack. This must hold in a debug build on a default 2 MiB test-thread stack.
- Algorithm:
  1. In `drop`, take the successor with `next.swap(None)`.
  2. Loop: for the current `Arc<Node>`, call `Arc::into_inner`.
  3. If it returns `Some(node)`, this call held the last reference. Take that node's `next` with `swap(None)`, drop the node (its own `Drop` then sees an empty `next` and finishes at once), and continue with the taken successor.
  4. If it returns `None`, another owner holds the node. Stop. The remaining part of the chain stays intact and valid until that owner releases it, and its release is again iterative.
- Parts of the chain still held by other owners (iterators, other `Arc` clones) remain fully valid and traversable after a prefix is dropped. A node that is shared is never cleared or detached by the teardown.
- Teardown also applies when a `Link` is dropped (head or `next`), through the `Drop` of the node it holds.
- With concurrent drops, each node is freed exactly once.

**Concurrency**
- `load`, `swap` and `compare_and_swap` are lock-free atomic operations. They are safe under arbitrary concurrent use from many threads.
- Any number of iterators may walk the chain while appends, unlinks and drops happen.
- An iterator that holds a node always sees a valid `entry()` and a valid `next()` chain.

**Errors**
- There are no error types. Failure of `compare_and_swap` is reported as `Err(actual)`. `unlink` reports failure as `false`.
- The module never panics on valid use.

## 5. Relied-on elements

From `queue_state`:
- `EntryState<P,T,E>`, created as `Arc<EntryState<P,T,E>>` via `EntryState::new(payload)` or `EntryState::sentinel()`.
- `EntryState::state(&self) -> QueueState`.
- `QueueState { Enqueued, Claimed, Completed, Cancelled }`.
- Claim, complete and cancel are atomic. This module only reads the state, using `state()`, and never changes it.
- `EntryState` is `Send + Sync` when `P: Send, T: Send + Sync, E: Send + Sync`.
- The payload is dropped with the entry if it is still inside. A claimed entry's payload was moved out.