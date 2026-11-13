# Region-Pinning (build order 12)

## 1. Task, restrictions and boundaries
- Implement in Rust a **pin** on a device region. While at least one pin exists, the region must not be evicted.
- Each region has a shared, concurrency-safe pin count. It starts at 0.
- Dropping a pin decrements the count automatically (RAII). The pin can optionally report its released region over a feedback channel.
- The pin is also available under the name `DeviceHandle`.
- Out of scope:
  - The eviction logic itself. The eviction component only queries the count and treats a region as non-evictable while the count is above 0.
  - Creating or owning regions.
  - Any other behavior.
- Dependencies: none. The region type is not defined here, so the module is generic over `R`.

## 2. Context
- Device regions are managed elsewhere. Each region owner holds one `PinCount` for its region and hands out pins through it.
- Users of a region hold a `Pin<R>` for as long as they access it.
- The eviction logic reads `PinCount::is_unpinned()` and must not evict a region when it returns false.
- The optional channel lets the owner learn which region was released, for example to retry eviction.

## 3. Interface
```rust
use std::sync::{Arc, atomic::AtomicUsize, mpsc::Sender};

/// Shared pin counter of one region. Clones share the same counter.
#[derive(Clone, Debug, Default)]
pub struct PinCount(/* Arc<AtomicUsize> */);

impl PinCount {
    pub fn new() -> Self;                 // count == 0
    pub fn count(&self) -> usize;         // current number of live pins
    pub fn is_unpinned(&self) -> bool;    // count() == 0
}

/// RAII pin on a region. Not Clone. Send + Sync iff R: Send + Sync.
pub struct Pin<R> { /* region: Option<R>, count: PinCount, release_tx: Option<Sender<R>> */ }

impl<R> Pin<R> {
    /// Increments `count` and returns a pin without a feedback channel.
    pub fn new(region: R, count: &PinCount) -> Pin<R>;
    /// Same as `new`, but on release the region is sent to `release_tx`.
    pub fn with_release_channel(region: R, count: &PinCount, release_tx: Sender<R>) -> Pin<R>;
    /// The pinned region.
    pub fn region(&self) -> &R;
}

impl<R> Drop for Pin<R> { fn drop(&mut self); }

/// Alternative name for a pin.
pub type DeviceHandle<R> = Pin<R>;
```
- `Pin<R>` is `Send` when `R: Send`, because `Sender<R>` is `Send` in that case.
- No public API returns an error.

## 4. Behavior and invariants
- A new `PinCount` has `count() == 0` and `is_unpinned() == true`.
- Creating a pin increments the count by exactly 1 before the constructor returns. Dropping a pin decrements it by exactly 1.
- Invariant: `count()` equals the number of live pins created from that counter (or its clones).
- `is_unpinned()` is true exactly when `count() == 0`.
- `region()` returns the region passed at construction for the pin's whole lifetime.
- On drop, the count is decremented first. Then, if a channel is set, the region is sent over it exactly once. A receiver therefore sees `count()` already reduced when it gets the message.
- If the receiver has been dropped, the send error is ignored. Drop does not panic, and the count is still decremented.
- Without a channel, drop only decrements.
- Concurrency:
  - Increment and decrement are atomic (`fetch_add` and `fetch_sub`). Concurrent creation and drop from many threads never lose updates.
  - Ordering: `AcqRel` or stronger on the modifying operations, `Acquire` on reads.
  - The count can never underflow or wrap, because each decrement matches one increment.
- Pins on different counters are independent.
- `std::mem::forget` on a pin leaves the count raised. This is acceptable and is not guarded against.

## 5. Relied-on elements
None.