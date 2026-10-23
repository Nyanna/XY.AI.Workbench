//! Gather path of the load strategy: scattered entries are packed on the host into one pinned
//! staging slot and leave for the device as a single copy.

use std::ops::Range;

use crate::resident::error::ManagerError;
use crate::resident::index::{EntryId, Index};
use crate::resident::memory::error::{MemoryError, TransferDir};
use crate::resident::memory::region::Region;
use crate::resident::memory::StagingRing;
use crate::resident::subengine::EventHandle;

/// Host memory of one staging slot.
pub trait StagingBuffer: Send {
    fn bytes_mut(&mut self) -> Result<&mut [u8], MemoryError>;
}

impl StagingBuffer for Vec<u8> {
    fn bytes_mut(&mut self) -> Result<&mut [u8], MemoryError> {
        Ok(self.as_mut_slice())
    }
}

#[cfg(feature = "cuda")]
impl StagingBuffer for cudarc::driver::PinnedHostSlice<u8> {
    fn bytes_mut(&mut self) -> Result<&mut [u8], MemoryError> {
        self.as_mut_slice().map_err(|e| MemoryError::TransferFailed {
            dir: TransferDir::HostToDevice,
            reason: e.to_string(),
        })
    }
}

/// Asynchronous host-to-device copy.
pub trait UploadPort: Send + Sync {
    /// `src` must stay untouched until the returned event has completed.
    fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placement {
    pub entry: EntryId,
    /// Byte range relative to the start of the destination region.
    pub range: Range<u64>,
}

pub struct StagedGather<S> {
    ring: StagingRing<S>,
}

impl<S: StagingBuffer> StagedGather<S> {
    pub fn new(ring: StagingRing<S>) -> Self {
        Self { ring }
    }

    /// Packs entries from the front of `entries` into one free slot, each at `align`, until the
    /// next one no longer fits. Removed entries (empty data) are consumed without a placement.
    /// Blocks while every slot is still in flight, and may block on index I/O.
    pub fn gather<'a>(
        &'a self,
        index: &dyn Index,
        entries: &[EntryId],
        align: u64,
    ) -> Result<Prepared<'a, S>, ManagerError> {
        let slot = self.ring.acquire_wait()?;
        let packed = self.ring.with_slot(slot, |buffer| pack(buffer, index, entries, align));
        match packed.map_err(ManagerError::from).and_then(|r| r) {
            Ok(packed) => Ok(Prepared {
                owner: self,
                slot: Some(slot),
                placements: packed.placements,
                bytes: packed.bytes,
                taken: packed.taken,
            }),
            Err(e) => {
                let _ = self.ring.release(slot);
                Err(e)
            }
        }
    }
}

struct Packed {
    placements: Vec<Placement>,
    bytes: u64,
    taken: usize,
}

fn pack<S: StagingBuffer>(
    buffer: &mut S,
    index: &dyn Index,
    entries: &[EntryId],
    align: u64,
) -> Result<Packed, ManagerError> {
    let buf = buffer.bytes_mut()?;
    let capacity = buf.len() as u64;
    let align = align.max(1);
    let mut packed = Packed { placements: Vec::new(), bytes: 0, taken: 0 };
    for &entry in entries {
        let data = index.read(entry);
        let src = data.as_bytes();
        if src.is_empty() {
            packed.taken += 1;
            continue;
        }
        let len = src.len() as u64;
        let start = packed.bytes.div_ceil(align) * align;
        let end = start.saturating_add(len);
        if end > capacity {
            if packed.placements.is_empty() {
                return Err(ManagerError::ArenaExhausted {
                    requested: len,
                    available: capacity,
                });
            }
            break;
        }
        buf[start as usize..end as usize].copy_from_slice(src);
        packed.placements.push(Placement { entry, range: start..end });
        packed.bytes = end;
        packed.taken += 1;
    }
    Ok(packed)
}

/// Packed slot awaiting its destination. Dropping it releases the slot unused.
pub struct Prepared<'a, S: StagingBuffer> {
    owner: &'a StagedGather<S>,
    slot: Option<usize>,
    placements: Vec<Placement>,
    bytes: u64,
    taken: usize,
}

impl<S: StagingBuffer> Prepared<'_, S> {
    pub fn placements(&self) -> &[Placement] {
        &self.placements
    }

    /// Size of the destination region the copy needs.
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Number of leading input entries handled, placed or skipped.
    pub fn taken(&self) -> usize {
        self.taken
    }

    /// One copy of the packed bytes into `dst`; the slot is reused only after its event.
    pub fn submit(mut self, port: &dyn UploadPort, dst: Region) -> Result<EventHandle, ManagerError> {
        let slot = self.slot.take().expect("slot is held until submit or drop");
        let ring = &self.owner.ring;
        let bytes = self.bytes as usize;
        let sent = ring
            .with_slot(slot, |buffer| {
                let buf = buffer.bytes_mut()?;
                port.upload(&buf[..bytes], dst)
            })
            .and_then(|r| r);
        match sent {
            Ok(event) => {
                ring.publish(slot, event.clone())?;
                tracing::debug!(bytes, entries = self.placements.len(), "gather submitted");
                Ok(event)
            }
            Err(e) => {
                let _ = ring.release(slot);
                Err(e.into())
            }
        }
    }
}

impl<S: StagingBuffer> Drop for Prepared<'_, S> {
    fn drop(&mut self) {
        if let Some(slot) = self.slot.take() {
            let _ = self.owner.ring.release(slot);
        }
    }
}

#[cfg(feature = "cuda")]
pub use cuda_port::CudaUploadPort;

#[cfg(feature = "cuda")]
mod cuda_port {
    use std::sync::Arc;

    use parking_lot::Mutex;

    use super::*;
    use crate::resident::memory::cuda::CudaMemoryManager;
    use crate::resident::subengine::{CudaLane, ExecContext};

    /// Copies on the stream of a dedicated copy lane.
    pub struct CudaUploadPort {
        memory: Arc<CudaMemoryManager>,
        lane: Mutex<CudaLane>,
    }

    impl CudaUploadPort {
        pub fn new(memory: Arc<CudaMemoryManager>, lane: CudaLane) -> Self {
            Self { memory, lane: Mutex::new(lane) }
        }
    }

    impl UploadPort for CudaUploadPort {
        fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError> {
            let lane = self.lane.lock();
            // The staging ring keeps the slot out of reuse until the event recorded below
            // completes, which is the pinned-source contract of the copy.
            unsafe { self.memory.upload(lane.stream(), src, dst)? };
            lane.record_event().map_err(|e| MemoryError::TransferFailed {
                dir: TransferDir::HostToDevice,
                reason: e.to_string(),
            })
        }
    }
}

#[cfg(test)]
#[path = "test/upload.rs"]
mod tests;
