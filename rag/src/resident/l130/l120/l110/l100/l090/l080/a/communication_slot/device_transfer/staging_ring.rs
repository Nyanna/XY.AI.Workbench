use std::fmt;
use std::sync::{Mutex, MutexGuard};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransferDirection {
    H2D,
    D2H,
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
            StagingError::Exhausted => write!(f, "Staging erschöpft"),
            StagingError::InvalidSlot => write!(f, "ungültiger Slot"),
            StagingError::Transfer(e) => write!(f, "{}", e),
            StagingError::InvalidConfig => write!(f, "ungültige Staging-Konfiguration"),
        }
    }
}

impl std::error::Error for StagingError {}

struct Slot {
    state: SlotState,
    event: Option<Event>,
    seq: u64,
    payload: Box<[u8]>,
}

struct Inner {
    slots: Vec<Slot>,
    last: usize,
    next_seq: u64,
}

pub struct StagingRing {
    slot_size: usize,
    inner: Mutex<Inner>,
}

static_assertions::assert_impl_all!(StagingRing: Send, Sync);

impl Inner {
    fn reclaim(&mut self) {
        for slot in self.slots.iter_mut() {
            if slot.state != SlotState::InFlight {
                continue;
            }
            let done = matches!(slot.event.as_ref().map(|e| e.is_complete()), Some(Ok(true)));
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
    pub fn new(slot_count: usize, slot_size: usize) -> Result<StagingRing, StagingError> {
        if slot_count == 0 {
            return Err(StagingError::InvalidConfig);
        }
        let slots = (0..slot_count)
            .map(|_| Slot {
                state: SlotState::Free,
                event: None,
                seq: 0,
                payload: vec![0u8; slot_size].into_boxed_slice(),
            })
            .collect();
        Ok(StagingRing {
            slot_size,
            inner: Mutex::new(Inner {
                slots,
                last: slot_count - 1,
                next_seq: 0,
            }),
        })
    }

    pub fn from_arena(arena: &Arena, slot_count: usize) -> Result<StagingRing, StagingError> {
        if arena.kind != ArenaKind::Staging || slot_count == 0 {
            return Err(StagingError::InvalidConfig);
        }
        let size = arena.capacity / slot_count as u64;
        let size = usize::try_from(size).map_err(|_| StagingError::InvalidConfig)?;
        StagingRing::new(slot_count, size)
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

    pub fn acquire(&self) -> Result<SlotId, StagingError> {
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
                    return Err(StagingError::Transfer(TransferError {
                        direction: TransferDirection::H2D,
                        cause,
                    }));
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
        Ok(f(&mut s.payload))
    }

    pub fn payload_ptr(&self, slot: SlotId) -> Result<*mut u8, StagingError> {
        let mut guard = self.lock();
        let s = guard.occupied_mut(slot)?;
        Ok(s.payload.as_mut_ptr())
    }

    pub fn begin_transfer(&self, slot: SlotId, event: Event) -> Result<(), StagingError> {
        let mut guard = self.lock();
        let seq = guard.next_seq;
        let s = guard.occupied_mut(slot)?;
        s.state = SlotState::InFlight;
        s.event = Some(event);
        s.seq = seq;
        guard.next_seq += 1;
        Ok(())
    }

    pub fn release(&self, slot: SlotId) -> Result<(), StagingError> {
        let mut guard = self.lock();
        let s = guard.occupied_mut(slot)?;
        s.state = SlotState::Free;
        Ok(())
    }

    pub fn in_flight_count(&self) -> usize {
        let mut guard = self.lock();
        guard.reclaim();
        guard
            .slots
            .iter()
            .filter(|s| s.state == SlotState::InFlight)
            .count()
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
