# Spec: Queue-Iteration (module `queue_iter`, build order 23)

## 1. Task, restrictions and boundaries

Implement non-blocking, concurrency-safe iteration over the queue entries in enqueue order (oldest first). There are two variants: all entries, and active entries only. Each delivered item also gives access to the entry's opaque metadata.

Restrictions:
- This module does not enqueue, claim, complete or cancel. It does not unlink nodes, and it does not interpret the metadata.
- No locks, blocking, waiting or spinning. No `unsafe`. No dependencies other than `Queue-Zustandsmodell` (module `queue_state`) and `std`.
- It does not own or define the queue's list structure. It walks the list only through the `QueueNode` trait below, which the queue module implements for its node type.

## 2. Context

- The queue is a singly linked list of reference-counted nodes, ordered oldest to newest. A sentinel node marks the head, and new nodes are appended at the tail.
- Each node holds an `Arc<EntryState<P,T,E>>` and opaque metadata `M`. Callers use `M` for filtering, and this module never inspects it.
- Completed and cancelled entries are logically removed. The queue physically unlinks them later.
- Workers claim entries and callers enqueue and cancel entries at the same time as iteration runs.
- Neighbours:
  - `queue_state` provides the entry state.
  - The queue module implements `QueueNode` and calls the two constructor functions below.
  - Dispatch and inspection code consumes the iterators.

## 3. Interface

```rust
use std::sync::Arc;
use queue_state::{EntryState, QueueState};

/// Implemented by the queue's node type. All methods must be non-blocking and callable concurrently.
pub trait QueueNode: Send + Sync + 'static {
    type Payload; type Output; type Error; type Meta;
    /// True for the sentinel node (it has no deliverable entry).
    fn is_sentinel(&self) -> bool;
    /// Successor in enqueue order (next younger node), or None at the tail.
    /// A node that was unlinked must still return a valid successor chain or None; never block.
    fn next(&self) -> Option<Arc<Self>>;
    fn entry(&self) -> &Arc<EntryState<Self::Payload, Self::Output, Self::Error>>;
    fn meta(&self) -> &Self::Meta;
}

/// Delivered item: keeps the node alive, so it stays valid after logical removal or unlinking.
pub struct QueueItem<N: QueueNode>(Arc<N>);
impl<N: QueueNode> QueueItem<N> {
    pub fn entry(&self) -> &Arc<EntryState<N::Payload, N::Output, N::Error>>;
    pub fn meta(&self) -> &N::Meta;          // opaque metadata, unchanged
    pub fn state(&self) -> QueueState;       // = entry().state()
    pub fn node(&self) -> &Arc<N>;
}
impl<N: QueueNode> Clone for QueueItem<N> {}

/// Iterator over all linked entries.
pub struct AllIter<N: QueueNode> { /* private */ }
/// Iterator over active entries (state Enqueued or Claimed).
pub struct ActiveIter<N: QueueNode> { /* private */ }

impl<N: QueueNode> Iterator for AllIter<N>    { type Item = QueueItem<N>; fn next(&mut self) -> Option<QueueItem<N>>; }
impl<N: QueueNode> Iterator for ActiveIter<N> { type Item = QueueItem<N>; fn next(&mut self) -> Option<QueueItem<N>>; }
// Both are Send. Both are FusedIterator.

/// `head` is the sentinel node (or any node; iteration starts at head.next()).
pub fn iter_all<N: QueueNode>(head: &Arc<N>) -> AllIter<N>;
pub fn iter_active<N: QueueNode>(head: &Arc<N>) -> ActiveIter<N>;
```

## 4. Behavior and invariants

**Order and completeness**
- Both iterators start at `head.next()` and follow `next()` links. They yield nodes oldest first, each node at most once.
- An empty queue (`head.next()` is `None`) yields an empty iteration.
- Any node with `is_sentinel() == true` is skipped and never yielded.
- Without concurrent changes, `iter_all` yields exactly every linked entry. It yields none twice and omits none.
- With concurrent changes, `iter_all` yields every entry that is linked and reachable for the whole duration of the iteration. This holds for entries enqueued before iteration started and for entries enqueued during iteration that are appended behind the current position. Entries appended after the iterator has reached the tail (`next()` returned `None`) need not appear. After the end, `next()` keeps returning `None`.

**Filtering**
- `iter_all` yields completed and cancelled entries as long as they are still linked.
- `ActiveIter` yields a node only if `entry().state()` is neither `Completed` nor `Cancelled` when it is evaluated. `Enqueued` and `Claimed` are active.
- The state is read once per node, at the moment of advancing to it. An item can become inactive after it was yielded, and the item stays valid.
- `ActiveIter` yields exactly the nodes that `AllIter` yields, minus those inactive when read. The relative order is the same.

**Metadata**
- `QueueItem::meta()` returns a reference to the node's metadata, unmodified and without copying.

**Concurrency and validity**
- No iterator operation blocks, waits or spins. The work per `next()` is bounded by the number of skipped nodes, and each skipped node costs one `next()` link read and one state read.
- A yielded `QueueItem` holds an `Arc` to its node. The entry, its state and the metadata stay valid after logical removal, after unlinking and after the iterator is dropped.
- Iteration never alters entry states or list links.
- Iterators are not shared between threads. Many iterators may run at the same time on the same queue.

**Errors**
- There are no error values.
- The module does not catch panics from `state()`, which panics on an invalid internal value. Such a panic propagates.

## 5. Relied-on elements

From `queue_state`:
- `EntryState<P,T,E>`, created as `Arc<EntryState>`. It is `Send + Sync` when `P: Send, T: Send + Sync, E: Send + Sync`.
- `pub enum QueueState { Enqueued, Claimed, Completed, Cancelled }`. It derives `Debug, Clone, Copy, PartialEq, Eq`.
- `pub fn state(&self) -> QueueState`. It is a consistent atomic snapshot, never blocks, and panics on an invalid internal value.
- `Completed` and `Cancelled` are final states.
- A sentinel entry (`EntryState::sentinel()`) is `Enqueued` with no payload. Its claim always fails, so the queue's own `QueueNode::is_sentinel` identifies it, not the state.