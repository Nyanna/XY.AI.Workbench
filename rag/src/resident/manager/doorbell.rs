//! Doorbell between persistent-kernel batches and the host mutation gap.
//!
//! The device publishes the number of finished batches (release); the host polls (acquire) and
//! mutates only between that and its acknowledgement, which the device checks before it starts
//! the next batch. The host never waits for the kernel itself, only for a batch event.
//!
//! Open: visibility of the mapped cells across PCIe (posted writes, `__threadfence_system`) is
//! not yet verified on the target hardware.
use crate::resident::manager::swap::RootPointer;
use crate::resident::subengine::{DeviceError, EventHandle};
/// Right to mutate between two batches; handed back through [`Doorbell::close`].
#[derive(Debug)]
#[must_use = "the device does not start the next batch before the gap is closed"]
pub struct Gap {
    batch: u64,
}
impl Gap {
    pub fn batch(&self) -> u64 {
        self.batch
    }
}
pub struct Doorbell {
    done: RootPointer,
    ack: RootPointer,
}
impl Default for Doorbell {
    fn default() -> Self {
        Self::new()
    }
}
impl Doorbell {
    pub fn new() -> Self {
        Self {
            done: RootPointer::new(0),
            ack: RootPointer::new(0),
        }
    }
    /// Wraps two cells in pinned, mapped host memory.
    ///
    /// # Safety
    /// Both pointers must satisfy the contract of [`RootPointer::from_mapped`] and be distinct.
    #[cfg(all(feature = "cuda", not(loom)))]
    pub unsafe fn from_mapped(done: *mut u64, ack: *mut u64) -> Self {
        Self {
            done: unsafe { RootPointer::from_mapped(done) },
            ack: unsafe { RootPointer::from_mapped(ack) },
        }
    }
    /// Device side: batch `seq` (1-based) finished; its writes and hits are visible.
    pub fn complete(&self, seq: u64) {
        self.done.publish(seq);
    }
    /// Device side: batch `seq` may start once the gap after its predecessor is closed.
    pub fn may_start(&self, seq: u64) -> bool {
        seq <= 1 || self.ack.load() >= seq - 1
    }
    /// Host side, non-blocking.
    pub fn poll(&self) -> Option<Gap> {
        let done = self.done.load();
        (done != self.ack.load()).then_some(Gap { batch: done })
    }
    /// Host side: waits for the batch event, then looks at the doorbell.
    pub fn wait_gap(
        &self,
        batch_event: &EventHandle,
    ) -> Result<Option<Gap>, DeviceError> {
        batch_event.synchronize()?;
        Ok(self.poll())
    }
    /// Host side: all mutations of the gap are done; releases the next batch.
    pub fn close(&self, gap: Gap) {
        self.ack.publish(gap.batch);
    }
}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    #[test]
    fn gap_opens_after_completion_and_blocks_the_next_start_until_closed() {
        let bell = Doorbell::new();
        assert!(bell.may_start(1));
        assert!(bell.poll().is_none());
        bell.complete(1);
        assert!(! bell.may_start(2));
        let gap = bell.poll().expect("gap after batch end");
        assert_eq!(gap.batch(), 1);
        assert!(! bell.may_start(2));
        bell.close(gap);
        assert!(bell.may_start(2));
        assert!(bell.poll().is_none());
    }
    #[test]
    fn coalesced_completions_open_one_gap() {
        let bell = Doorbell::new();
        bell.complete(3);
        let gap = bell.poll().unwrap();
        assert_eq!(gap.batch(), 3);
        bell.close(gap);
        assert!(bell.poll().is_none());
        assert!(bell.may_start(4));
    }
    #[test]
    fn wait_gap_returns_after_the_event() {
        use crate::resident::segment::testutil::region;
        use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};
        let lane = CpuLane::new(LaneClass::Copy, LaneConfig::new(region(0, 4096)))
            .unwrap();
        let event = lane.record_event().unwrap();
        let bell = Doorbell::new();
        bell.complete(1);
        assert_eq!(bell.wait_gap(& event).unwrap().map(| g | g.batch()), Some(1));
    }
}
#[cfg(loom)]
mod loom_tests {
    use super::*;
    use loom::sync::atomic::{AtomicU64, Ordering};
    use loom::sync::Arc;
    use loom::thread;
    #[test]
    fn results_and_mapping_are_ordered_by_the_doorbell() {
        loom::model(|| {
            let bell = Arc::new(Doorbell::new());
            let results = Arc::new(AtomicU64::new(0));
            let mapping = Arc::new(AtomicU64::new(0));
            let device = {
                let (bell, results, mapping) = (
                    bell.clone(),
                    results.clone(),
                    mapping.clone(),
                );
                thread::spawn(move || {
                    results.store(5, Ordering::Relaxed);
                    bell.complete(1);
                    if bell.may_start(2) {
                        assert_eq!(mapping.load(Ordering::Relaxed), 9);
                    }
                })
            };
            if let Some(gap) = bell.poll() {
                assert_eq!(results.load(Ordering::Relaxed), 5);
                mapping.store(9, Ordering::Relaxed);
                bell.close(gap);
            }
            device.join().unwrap();
        });
    }
}
