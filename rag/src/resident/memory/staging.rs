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
mod tests {
    use super::*;
    use crate::resident::memory::arena::testing::Gate;

    #[test]
    fn slots_cycle_round_robin() {
        let ring = StagingRing::new([0u8, 1, 2]);
        let a = ring.acquire().unwrap();
        let b = ring.acquire().unwrap();
        assert_ne!(a, b);
        ring.release(a).unwrap();
        ring.release(b).unwrap();
        assert_eq!(ring.slots(), 3);
    }

    #[test]
    fn slot_is_not_reused_before_its_event_completes() {
        let ring = StagingRing::new([()]);
        let gate = Gate::closed();
        let i = ring.acquire().unwrap();
        ring.publish(i, gate.event.clone()).unwrap();
        assert_eq!(ring.in_flight(), 1);
        assert!(ring.acquire().is_none());
        gate.open();
        assert_eq!(ring.acquire(), Some(i));
    }

    #[test]
    fn acquire_wait_blocks_on_oldest_transfer() {
        let ring = StagingRing::new([()]);
        let gate = Gate::closed();
        let i = ring.acquire().unwrap();
        ring.publish(i, gate.event.clone()).unwrap();
        let waiter = std::thread::scope(|s| {
            let h = s.spawn(|| ring.acquire_wait());
            std::thread::sleep(std::time::Duration::from_millis(30));
            assert!(!h.is_finished());
            gate.open();
            h.join().unwrap()
        });
        assert_eq!(waiter.unwrap(), i);
    }

    #[test]
    fn acquire_wait_fails_when_all_slots_are_held() {
        let ring = StagingRing::new([()]);
        ring.acquire().unwrap();
        assert!(matches!(ring.acquire_wait(), Err(MemoryError::ArenaExhausted { .. })));
    }

    #[test]
    fn payload_access_requires_a_held_slot() {
        let ring = StagingRing::new([5u32]);
        assert!(matches!(ring.with_slot(0, |_| ()), Err(MemoryError::InvalidSlot { .. })));
        let i = ring.acquire().unwrap();
        ring.with_slot(i, |p| *p += 1).unwrap();
        assert_eq!(ring.with_slot(i, |p| *p).unwrap(), 6);
        assert!(ring.publish(9, Gate::closed().event).is_err());
    }
}
