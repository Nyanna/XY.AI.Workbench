//! Ring of staging slots; a slot is reused only after the event of its last transfer completed.

use parking_lot::Mutex;

use super::budget::ArenaKind;
use super::error::{MemoryError, TransferDir};
use crate::resident::subengine::EventHandle;

enum State {
    Free,
    Held,
    InFlight(EventHandle),
}

struct Entry<S> {
    payload: S,
    state: State,
}

struct Ring<S> {
    entries: Vec<Entry<S>>,
    cursor: usize,
}

pub struct StagingRing<S> {
    ring: Mutex<Ring<S>>,
}

impl<S> StagingRing<S> {
    pub fn new(payloads: impl IntoIterator<Item = S>) -> Self {
        let entries = payloads.into_iter().map(|payload| Entry { payload, state: State::Free }).collect();
        Self { ring: Mutex::new(Ring { entries, cursor: 0 }) }
    }

    pub fn slots(&self) -> usize {
        self.ring.lock().entries.len()
    }

    pub fn in_flight(&self) -> usize {
        self.ring.lock().entries.iter().filter(|e| matches!(e.state, State::InFlight(_))).count()
    }

    /// Takes the next slot that is free or whose transfer has completed.
    pub fn acquire(&self) -> Option<usize> {
        let mut ring = self.ring.lock();
        let n = ring.entries.len();
        for step in 0..n {
            let i = (ring.cursor + step) % n;
            let usable = match &ring.entries[i].state {
                State::Free => true,
                State::InFlight(e) => e.is_complete(),
                State::Held => false,
            };
            if usable {
                ring.entries[i].state = State::Held;
                ring.cursor = (i + 1) % n;
                return Some(i);
            }
        }
        None
    }

    /// Like `acquire`, but blocks on the oldest in-flight transfer when every slot is busy.
    pub fn acquire_wait(&self) -> Result<usize, MemoryError> {
        loop {
            if let Some(i) = self.acquire() {
                return Ok(i);
            }
            let oldest = {
                let ring = self.ring.lock();
                let n = ring.entries.len();
                (0..n).map(|s| (ring.cursor + s) % n).find_map(|i| match &ring.entries[i].state {
                    State::InFlight(e) => Some(e.clone()),
                    _ => None,
                })
            };
            let Some(event) = oldest else {
                return Err(MemoryError::ArenaExhausted {
                    arena: ArenaKind::Staging,
                    requested: 1,
                    free: 0,
                });
            };
            event.synchronize().map_err(|e| MemoryError::TransferFailed {
                dir: TransferDir::HostToDevice,
                reason: e.to_string(),
            })?;
        }
    }

    pub fn with_slot<R>(&self, index: usize, f: impl FnOnce(&mut S) -> R) -> Result<R, MemoryError> {
        let mut ring = self.ring.lock();
        match ring.entries.get_mut(index) {
            Some(e) if matches!(e.state, State::Held) => Ok(f(&mut e.payload)),
            _ => Err(MemoryError::InvalidSlot { index }),
        }
    }

    /// Marks a held slot as in use by a transfer that completes with `event`.
    pub fn publish(&self, index: usize, event: EventHandle) -> Result<(), MemoryError> {
        self.transition(index, State::InFlight(event))
    }

    /// Returns a held slot that was never used for a transfer.
    pub fn release(&self, index: usize) -> Result<(), MemoryError> {
        self.transition(index, State::Free)
    }

    fn transition(&self, index: usize, next: State) -> Result<(), MemoryError> {
        let mut ring = self.ring.lock();
        match ring.entries.get_mut(index) {
            Some(e) if matches!(e.state, State::Held) => {
                e.state = next;
                Ok(())
            }
            _ => Err(MemoryError::InvalidSlot { index }),
        }
    }
}

#[cfg(test)]
#[path = "test/staging.rs"]
mod tests;
