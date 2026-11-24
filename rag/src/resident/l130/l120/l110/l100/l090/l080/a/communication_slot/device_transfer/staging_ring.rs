use std::fmt;
use std::sync::{Condvar, Mutex, MutexGuard};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::gpu_backend::{
    GpuBackend, PinnedBuffer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::memory_backend::BackendError;
pub use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::memory_backend::TransferDirection;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::{
    Arena, ArenaKind,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, EventError,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotState {
    Free,
    Occupied,
    InFlight,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferError {
    pub direction: TransferDirection,
    pub cause: EventError,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StagingError {
    Exhausted,
    InvalidSlot,
    Transfer(TransferError),
    InvalidConfig,
    Backend(BackendError),
}
impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} transfer failed: {}", self.direction, self.cause)
    }
}
impl std::error::Error for TransferError {}
impl fmt::Display for StagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StagingError::Exhausted => write!(f, "staging ring exhausted"),
            StagingError::InvalidSlot => write!(f, "invalid staging slot"),
            StagingError::Transfer(e) => write!(f, "{}", e),
            StagingError::InvalidConfig => write!(f, "invalid staging configuration"),
            StagingError::Backend(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for StagingError {}
enum Payload {
    Heap(Box<[u8]>),
    Pinned(PinnedBuffer),
}
impl Payload {
    fn bytes_mut(&mut self) -> &mut [u8] {
        match self {
            Payload::Heap(bytes) => bytes,
            Payload::Pinned(buffer) => buffer.as_mut_slice(),
        }
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.bytes_mut().as_mut_ptr()
    }
}
struct Slot {
    state: SlotState,
    event: Option<Event>,
    seq: u64,
    payload: Payload,
}
struct Inner {
    slots: Vec<Slot>,
    last: usize,
    next_seq: u64,
}
pub struct StagingRing {
    slot_size: usize,
    inner: Mutex<Inner>,
    /// Signalled when a slot is released or starts a transfer.
    changed: Condvar,
}
static_assertions::assert_impl_all!(StagingRing : Send, Sync);
impl Inner {
    fn reclaim(&mut self) {
        for slot in self.slots.iter_mut() {
            if slot.state != SlotState::InFlight {
                continue;
            }
            let done = matches!(
                slot.event.as_ref().map(| e | e.is_complete()), Some(Ok(true))
            );
            if done {
                slot.state = SlotState::Free;
                slot.event = None;
            }
        }
    }
    fn oldest_in_flight(&self) -> Option<(usize, u64, Event)> {
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.state == SlotState::InFlight)
            .min_by_key(|(_, s)| s.seq)
            .and_then(|(i, s)| s.event.clone().map(|e| (i, s.seq, e)))
    }
    fn occupied_mut(&mut self, slot: SlotId) -> Result<&mut Slot, StagingError> {
        match self.slots.get_mut(slot.0) {
            Some(s) if s.state == SlotState::Occupied => Ok(s),
            _ => Err(StagingError::InvalidSlot),
        }
    }
}
impl StagingRing {
    pub fn new(
        slot_count: usize,
        slot_size: usize,
    ) -> Result<StagingRing, StagingError> {
        if slot_count == 0 {
            return Err(StagingError::InvalidConfig);
        }
        let slots = (0..slot_count)
            .map(|_| Slot {
                state: SlotState::Free,
                event: None,
                seq: 0,
                payload: Payload::Heap(vec![0u8; slot_size].into_boxed_slice()),
            })
            .collect();
        Ok(StagingRing {
            slot_size,
            inner: Mutex::new(Inner {
                slots,
                last: slot_count - 1,
                next_seq: 0,
            }),
            changed: Condvar::new(),
        })
    }
    pub fn from_arena(
        arena: &Arena,
        slot_count: usize,
    ) -> Result<StagingRing, StagingError> {
        if arena.kind != ArenaKind::Staging || slot_count == 0 {
            return Err(StagingError::InvalidConfig);
        }
        let size = arena.capacity / slot_count as u64;
        let size = usize::try_from(size).map_err(|_| StagingError::InvalidConfig)?;
        StagingRing::new(slot_count, size)
    }
    /// Slots backed by pinned host memory, so uploads from a slot stay asynchronous.
    pub fn pinned(
        backend: &GpuBackend,
        slot_count: usize,
        slot_size: usize,
    ) -> Result<StagingRing, StagingError> {
        if slot_count == 0 {
            return Err(StagingError::InvalidConfig);
        }
        let mut slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            let buffer = backend
                .allocate_pinned(slot_size)
                .map_err(StagingError::Backend)?;
            slots
                .push(Slot {
                    state: SlotState::Free,
                    event: None,
                    seq: 0,
                    payload: Payload::Pinned(buffer),
                });
        }
        Ok(StagingRing {
            slot_size,
            inner: Mutex::new(Inner {
                slots,
                last: slot_count - 1,
                next_seq: 0,
            }),
            changed: Condvar::new(),
        })
    }
    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|p| p.into_inner())
    }
    pub fn slot_count(&self) -> usize {
        self.lock().slots.len()
    }
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    /// Free -> Occupied. Waits for the oldest transfer when the candidate slot is in flight;
    /// returns `Exhausted` when every slot is occupied and none is in flight.
    pub fn acquire(&self) -> Result<SlotId, StagingError> {
        self.acquire_with(false)
    }
    /// Like `acquire`, but when every slot is occupied and none is in flight it waits until a
    /// slot is released or starts a transfer instead of returning `Exhausted`. A caller that
    /// holds slots while waiting here can deadlock itself; it is meant for callers that hold
    /// at most one slot at a time and never wait on the ring while holding it.
    pub fn acquire_wait(&self) -> Result<SlotId, StagingError> {
        self.acquire_with(true)
    }
    fn acquire_with(&self, wait_for_release: bool) -> Result<SlotId, StagingError> {
        let mut guard = self.lock();
        loop {
            guard.reclaim();
            let n = guard.slots.len();
            let candidate = (guard.last + 1) % n;
            if guard.slots[candidate].state == SlotState::Free {
                guard.slots[candidate].state = SlotState::Occupied;
                guard.last = candidate;
                return Ok(SlotId(candidate));
            }
            let (idx, seq, event) = match guard.oldest_in_flight() {
                Some(t) => t,
                None if wait_for_release => {
                    guard = self.changed.wait(guard).unwrap_or_else(|p| p.into_inner());
                    continue;
                }
                None => return Err(StagingError::Exhausted),
            };
            drop(guard);
            let result = event.wait();
            guard = self.lock();
            match result {
                Ok(()) => {
                    let s = &mut guard.slots[idx];
                    if s.state == SlotState::InFlight && s.seq == seq {
                        s.state = SlotState::Free;
                        s.event = None;
                    }
                }
                Err(cause) => {
                    return Err(
                        StagingError::Transfer(TransferError {
                            direction: TransferDirection::H2D,
                            cause,
                        }),
                    );
                }
            }
        }
    }
    pub fn with_payload<R>(
        &self,
        slot: SlotId,
        f: impl FnOnce(&mut [u8]) -> R,
    ) -> Result<R, StagingError> {
        let mut guard = self.lock();
        let s = guard.occupied_mut(slot)?;
        Ok(f(s.payload.bytes_mut()))
    }
    pub fn payload_ptr(&self, slot: SlotId) -> Result<*mut u8, StagingError> {
        let mut guard = self.lock();
        let s = guard.occupied_mut(slot)?;
        Ok(s.payload.as_mut_ptr())
    }
    pub fn begin_transfer(
        &self,
        slot: SlotId,
        event: Event,
    ) -> Result<(), StagingError> {
        let mut guard = self.lock();
        let seq = guard.next_seq;
        let s = guard.occupied_mut(slot)?;
        s.state = SlotState::InFlight;
        s.event = Some(event);
        s.seq = seq;
        guard.next_seq += 1;
        self.changed.notify_all();
        Ok(())
    }
    pub fn release(&self, slot: SlotId) -> Result<(), StagingError> {
        let mut guard = self.lock();
        let s = guard.occupied_mut(slot)?;
        s.state = SlotState::Free;
        self.changed.notify_all();
        Ok(())
    }
    pub fn in_flight_count(&self) -> usize {
        let mut guard = self.lock();
        guard.reclaim();
        guard.slots.iter().filter(|s| s.state == SlotState::InFlight).count()
    }
    pub fn in_flight_slots(&self) -> Vec<SlotId> {
        let mut guard = self.lock();
        guard.reclaim();
        let mut v: Vec<(u64, usize)> = guard
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.state == SlotState::InFlight)
            .map(|(i, s)| (s.seq, i))
            .collect();
        v.sort();
        v.into_iter().map(|(_, i)| SlotId(i)).collect()
    }
    pub fn slot_state(&self, slot: SlotId) -> Option<SlotState> {
        let mut guard = self.lock();
        guard.reclaim();
        guard.slots.get(slot.0).map(|s| s.state)
    }
}
#[cfg(test)]
#[path = "test/staging_ring.rs"]
mod tests;
