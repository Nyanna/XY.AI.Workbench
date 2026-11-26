use std::cell::UnsafeCell;
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use log::{debug, warn};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::SegmentId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::{
    KernelHit, Slot5,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
const LOG_TARGET2: &str = "hit_queues";
/// Address assignment of an embedding: the entry lives in (segment, slot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmbeddingWrite {
    pub entry: EntryId,
    pub segment: SegmentId,
    pub slot: Slot5,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    /// The queue is full; the element was not accepted.
    Full,
}
/// Error of `push`; hands the rejected element back.
#[derive(Debug, PartialEq)]
pub struct PushError<T> {
    pub item: T,
    pub error: QueueError,
}
impl<T> fmt::Display for PushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            QueueError::Full => f.write_str("queue full"),
        }
    }
}
impl<T: fmt::Debug> std::error::Error for PushError<T> {}
/// Result of one consumer pass.
#[derive(Debug, Default, PartialEq)]
pub struct Drained {
    pub writes: Vec<EmbeddingWrite>,
    pub hits: Vec<KernelHit>,
}
struct Ring<T> {
    buf: Box<[UnsafeCell<MaybeUninit<T>>]>,
    /// Monotonic count of popped elements; written by the consumer only.
    head: AtomicUsize,
    /// Monotonic count of pushed elements; written by the producer only.
    tail: AtomicUsize,
}
unsafe impl<T: Send> Send for Ring<T> {}
unsafe impl<T: Send> Sync for Ring<T> {}
impl<T> Drop for Ring<T> {
    fn drop(&mut self) {
        let head = *self.head.get_mut();
        let tail = *self.tail.get_mut();
        let cap = self.buf.len();
        for i in head..tail {
            unsafe { self.buf[i % cap].get_mut().assume_init_drop() };
        }
    }
}
struct RingTx<T> {
    ring: Arc<Ring<T>>,
}
struct RingRx<T> {
    ring: Arc<Ring<T>>,
}
fn ring_pair<T>(capacity: usize) -> (RingTx<T>, RingRx<T>) {
    let capacity = capacity.max(1);
    let buf = (0..capacity)
        .map(|_| UnsafeCell::new(MaybeUninit::uninit()))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let ring = Arc::new(Ring {
        buf,
        head: AtomicUsize::new(0),
        tail: AtomicUsize::new(0),
    });
    (RingTx { ring: ring.clone() }, RingRx { ring })
}
impl<T> RingTx<T> {
    fn capacity(&self) -> usize {
        self.ring.buf.len()
    }
    fn depth(&self) -> usize {
        let head = self.ring.head.load(Ordering::Acquire);
        let tail = self.ring.tail.load(Ordering::Relaxed);
        tail - head
    }
    fn push(&mut self, item: T) -> Result<(), T> {
        let ring = &*self.ring;
        let tail = ring.tail.load(Ordering::Relaxed);
        let head = ring.head.load(Ordering::Acquire);
        if tail - head >= ring.buf.len() {
            return Err(item);
        }
        unsafe { (*ring.buf[tail % ring.buf.len()].get()).write(item) };
        ring.tail.store(tail + 1, Ordering::Release);
        Ok(())
    }
}
impl<T> RingRx<T> {
    fn depth(&self) -> usize {
        let head = self.ring.head.load(Ordering::Relaxed);
        let tail = self.ring.tail.load(Ordering::Acquire);
        tail - head
    }
    fn pop(&mut self) -> Option<T> {
        let ring = &*self.ring;
        let head = ring.head.load(Ordering::Relaxed);
        let tail = ring.tail.load(Ordering::Acquire);
        if head == tail {
            return None;
        }
        let item = unsafe {
            (*ring.buf[head % ring.buf.len()].get()).assume_init_read()
        };
        ring.head.store(head + 1, Ordering::Release);
        Some(item)
    }
}
/// Only producer of the write queue.
pub struct WriteProducer {
    tx: RingTx<EmbeddingWrite>,
}
impl WriteProducer {
    pub fn push(&mut self, w: EmbeddingWrite) -> Result<(), PushError<EmbeddingWrite>> {
        match self.tx.push(w) {
            Ok(()) => {
                debug!(
                    target : LOG_TARGET2, "push queue=write depth={}", self.tx.depth()
                );
                Ok(())
            }
            Err(item) => {
                warn!(
                    target : LOG_TARGET2, "queue full queue=write capacity={}", self.tx
                    .capacity()
                );
                Err(PushError {
                    item,
                    error: QueueError::Full,
                })
            }
        }
    }
    pub fn depth(&self) -> usize {
        self.tx.depth()
    }
    pub fn capacity(&self) -> usize {
        self.tx.capacity()
    }
}
/// Only producer of the hit queue.
pub struct HitProducer {
    tx: RingTx<KernelHit>,
}
impl HitProducer {
    pub fn push(&mut self, h: KernelHit) -> Result<(), PushError<KernelHit>> {
        match self.tx.push(h) {
            Ok(()) => {
                debug!(target : LOG_TARGET2, "push queue=hit depth={}", self.tx.depth());
                Ok(())
            }
            Err(item) => {
                warn!(
                    target : LOG_TARGET2, "queue full queue=hit capacity={}", self.tx
                    .capacity()
                );
                Err(PushError {
                    item,
                    error: QueueError::Full,
                })
            }
        }
    }
    pub fn depth(&self) -> usize {
        self.tx.depth()
    }
    pub fn capacity(&self) -> usize {
        self.tx.capacity()
    }
}
/// Only consumer of both queues.
pub struct QueueConsumer {
    writes: RingRx<EmbeddingWrite>,
    hits: RingRx<KernelHit>,
}
impl QueueConsumer {
    /// Takes all writes first, then the hits that were visible before the writes were taken.
    pub fn drain(&mut self) -> Drained {
        let hit_count = self.hits.depth();
        let mut writes = Vec::new();
        while let Some(w) = self.writes.pop() {
            writes.push(w);
        }
        let mut hits = Vec::with_capacity(hit_count);
        for _ in 0..hit_count {
            match self.hits.pop() {
                Some(h) => hits.push(h),
                None => break,
            }
        }
        debug!(
            target : LOG_TARGET2, "drain writes={} hits={} write_depth={} hit_depth={}",
            writes.len(), hits.len(), self.writes.depth(), self.hits.depth()
        );
        Drained { writes, hits }
    }
    pub fn write_depth(&self) -> usize {
        self.writes.depth()
    }
    pub fn hit_depth(&self) -> usize {
        self.hits.depth()
    }
}
/// Creates the queue pair. A capacity of 0 becomes 1.
pub fn channel(
    write_capacity: usize,
    hit_capacity: usize,
) -> (WriteProducer, HitProducer, QueueConsumer) {
    let (write_tx, write_rx) = ring_pair(write_capacity);
    let (hit_tx, hit_rx) = ring_pair(hit_capacity);
    (
        WriteProducer { tx: write_tx },
        HitProducer { tx: hit_tx },
        QueueConsumer {
            writes: write_rx,
            hits: hit_rx,
        },
    )
}
#[cfg(test)]
#[path = "test/hit_queues.rs"]
mod tests;
