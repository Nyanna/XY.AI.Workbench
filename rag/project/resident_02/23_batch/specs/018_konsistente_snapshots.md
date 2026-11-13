# Konsistente Snapshots: Spezifikation (Modul 18)

## 1. Task, restrictions and boundaries
Implement in Rust a seqlock over a region of machine words (`usize`).
- Readers get a complete, untorn copy of the region together with the number of retries they needed.
- Writers exclude each other, always write the whole region, and raise the sequence counter by exactly 2 per write.
- A region can be built from raw addresses if the caller guarantees alignment, validity and a uniform protocol.

Restrictions:
- Dependencies: none. Use only `core` and `std` (`std::sync::atomic`).
- No other responsibilities: no persistence, no naming, no typed payload layer, no blocking primitives such as `Mutex`, and no timeouts.
- Waiting is by spinning only (`std::hint::spin_loop`).

## 2. Context
Other modules share data that changes concurrently. Examples are shared-memory or mmap'd regions and statistics blocks that several threads or processes write and read. They use this module to read such data consistently without locking readers. Readers never write to the region or the counter, so read-only access is enough for them. All parties on one region must use this protocol: the counter is the first element and the data words follow.

## 3. Interface
```rust
use std::sync::atomic::{AtomicU64, AtomicUsize};

/// Copyable, non-owning view of one seqlock-protected region.
#[derive(Clone, Copy)]
pub struct SeqRegion { /* seq: *const AtomicU64, data: *const AtomicUsize, len: usize */ }

unsafe impl Send for SeqRegion {}
unsafe impl Sync for SeqRegion {}

impl SeqRegion {
    /// # Safety
    /// - `seq` is aligned for `AtomicU64`; `data` is aligned for `AtomicUsize`.
    /// - `seq` and `data[0..len]` are valid for reads and writes for the whole lifetime of every
    ///   copy of this view, and do not overlap.
    /// - Every access to this memory, in any thread or process, goes only through this protocol
    ///   (`SeqRegion` or `SeqBuf`). No plain non-atomic access.
    /// - `*seq` is even when first used (normally 0).
    pub unsafe fn from_raw_parts(seq: *const AtomicU64, data: *const AtomicUsize, len: usize) -> SeqRegion;

    /// Number of words in the region.
    pub fn len(&self) -> usize;

    /// Copies the entire region into `out` and returns the retry count.
    /// Panics if `out.len() != self.len()`.
    pub fn read_into(&self, out: &mut [usize]) -> u64;

    /// Convenience: allocates a Vec of `len()` words, reads it, and returns `(copy, retries)`.
    pub fn read(&self) -> (Vec<usize>, u64);

    /// Writes the entire region from `src`. The counter increases by 2.
    /// Panics (before any write or lock) if `src.len() != self.len()`.
    pub fn write(&self, src: &[usize]);

    /// Current counter value (Acquire load). Even means no writer is active.
    pub fn sequence(&self) -> u64;
}

/// Owning, heap-backed region (counter plus words, initially 0).
pub struct SeqBuf { /* Box<AtomicU64>, Box<[AtomicUsize]> */ }
unsafe impl Send for SeqBuf {}
unsafe impl Sync for SeqBuf {}

impl SeqBuf {
    pub fn new(len: usize) -> SeqBuf;                  // all words 0, counter 0
    pub fn from_words(init: &[usize]) -> SeqBuf;       // words = init, counter 0
    pub fn region(&self) -> SeqRegion;                 // view; valid as long as the SeqBuf lives
    pub fn len(&self) -> usize;
    pub fn read_into(&self, out: &mut [usize]) -> u64; // same as SeqRegion
    pub fn read(&self) -> (Vec<usize>, u64);
    pub fn write(&self, src: &[usize]);
    pub fn sequence(&self) -> u64;
}
```

## 4. Behavior and invariants
**Counter**
- The counter is a `u64`. Even means stable. Odd means a writer is active.
- It increases with wrapping arithmetic.
- One `write` raises it by exactly 2: even→odd at the start, odd→even (+1) at the end.

**Writer**
- It acquires by looping on `compare_exchange(s, s+1, Acquire, Relaxed)`, starting from an even `s`.
- If the counter is odd, it spins and reloads.
- This makes writers mutually exclusive. No second lock exists.
- It writes all `len` words with `Relaxed` atomic stores. Before these stores comes `fence(Release)`.
- It finishes with `store(s+2, Release)`.
- A writer never releases the lock early and never writes only part of the region.
- A zero-length region is valid. `write` still raises the counter by 2, and reads return an empty copy.

**Reader**
- It loops as follows:
  1. Load `s1` (Acquire).
  2. If `s1` is odd, count one retry and spin.
  3. Copy the words with `Relaxed` atomic loads, one word at a time.
  4. Run `fence(Acquire)`.
  5. Load `s2` (Relaxed). If `s2 != s1`, count one retry and loop. Otherwise return.
- The result is always a copy that equals the state after exactly one complete write (or the initial state). It is never a mix of two writes.
- The retry count is the number of discarded attempts, as a `u64`. It is 0 if the first attempt succeeded.
- A reader never writes to the data or the counter.
- Readers are lock-free with respect to each other. They may spin for as long as writers keep the counter odd or keep changing it.

**Errors and panics**
- Length mismatch in `read_into` or `write` panics with a clear message.
- There is no `Result` type, because there are no recoverable errors.
- If a writer panics between acquire and release, the region stays locked. This cannot occur in the safe code path, because only infallible stores run between acquire and release, so the length check happens first.

**Edge cases**
- Concurrent `write` calls from many threads each increase the counter by 2. After N completed writes the counter is `initial + 2N` (mod 2^64).
- `from_raw_parts` performs no validation. Violating its safety contract is undefined behavior and is the caller's responsibility.
- Copies of a `SeqRegion` view all refer to the same region and the same counter.

## 5. Relied-on elements
None (no dependencies).