use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock, TryLockError};

use log::{debug, warn};
use tokio::sync::Notify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueState {
    Enqueued,
    Claimed,
    Completed,
    Cancelled,
}

const ENQUEUED: u8 = 0;
const CLAIMED: u8 = 1;
const COMPLETED: u8 = 2;
const CANCELLED: u8 = 3;

impl QueueState {
    fn to_raw(self) -> u8 {
        match self {
            QueueState::Enqueued => ENQUEUED,
            QueueState::Claimed => CLAIMED,
            QueueState::Completed => COMPLETED,
            QueueState::Cancelled => CANCELLED,
        }
    }

    fn from_raw(raw: u8) -> QueueState {
        match raw {
            ENQUEUED => QueueState::Enqueued,
            CLAIMED => QueueState::Claimed,
            COMPLETED => QueueState::Completed,
            CANCELLED => QueueState::Cancelled,
            other => panic!("queue entry state holds invalid value {other}"),
        }
    }

    fn is_final(self) -> bool {
        matches!(self, QueueState::Completed | QueueState::Cancelled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError {
    /// Entry was not Enqueued; carries the observed state.
    NotEnqueued(QueueState),
    /// Entry was cancelled.
    Cancelled,
    /// Entry has no payload (sentinel); state is left unchanged.
    NoPayload,
    /// Payload compartment was locked at the moment of the attempt; state unchanged.
    Busy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteError {
    /// State was not Claimed (carries the observed state); result unchanged.
    NotClaimed(QueueState),
}

pub struct EntryState<P, T, E> {
    state: AtomicU8,
    payload: Mutex<Option<P>>,
    result: OnceLock<Result<T, E>>,
    notify: Notify,
}

impl<P, T, E> EntryState<P, T, E> {
    fn build(payload: Option<P>) -> Arc<Self> {
        Arc::new(Self {
            state: AtomicU8::new(ENQUEUED),
            payload: Mutex::new(payload),
            result: OnceLock::new(),
            notify: Notify::new(),
        })
    }

    pub fn new(payload: P) -> Arc<Self> {
        Self::build(Some(payload))
    }

    pub fn sentinel() -> Arc<Self> {
        Self::build(None)
    }

    pub fn state(&self) -> QueueState {
        QueueState::from_raw(self.state.load(Ordering::Acquire))
    }

    pub fn result(&self) -> Option<&Result<T, E>> {
        self.result.get()
    }

    pub fn claim(&self) -> Result<P, ClaimError> {
        claim_entry(self)
    }

    pub fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError> {
        let observed = self.state();
        if observed != QueueState::Claimed {
            warn!("complete rejected: entry is {observed:?}, expected Claimed");
            if observed == QueueState::Completed {
                debug_assert!(false, "entry completed more than once");
            }
            return Err(CompleteError::NotClaimed(observed));
        }
        if self.result.set(result).is_err() {
            warn!("complete rejected: result already set by a concurrent completion");
            debug_assert!(false, "entry completed more than once");
            return Err(CompleteError::NotClaimed(QueueState::Completed));
        }
        self.state.store(COMPLETED, Ordering::Release);
        self.notify.notify_waiters();
        debug!("entry completed");
        Ok(())
    }

    pub fn cancel(&self) -> bool {
        match self.state.compare_exchange(
            ENQUEUED,
            CANCELLED,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                self.notify.notify_waiters();
                debug!("cancel succeeded");
                true
            }
            Err(raw) => {
                debug!("cancel failed: entry is {:?}", QueueState::from_raw(raw));
                false
            }
        }
    }

    pub async fn wait_finished(&self) -> QueueState {
        loop {
            let notified = self.notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            let state = self.state();
            if state.is_final() {
                return state;
            }
            notified.await;
        }
    }
}

pub fn claim_entry<P, T, E>(entry: &EntryState<P, T, E>) -> Result<P, ClaimError> {
    let mut guard = match entry.payload.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
        Err(TryLockError::WouldBlock) => {
            debug!("claim failed: payload compartment busy");
            return Err(ClaimError::Busy);
        }
    };

    let observed = entry.state();
    if observed != QueueState::Enqueued {
        return Err(log_claim_failure(not_enqueued_error(observed)));
    }
    if guard.is_none() {
        return Err(log_claim_failure(ClaimError::NoPayload));
    }

    if let Err(raw) = entry.state.compare_exchange(
        QueueState::Enqueued.to_raw(),
        QueueState::Claimed.to_raw(),
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        return Err(log_claim_failure(not_enqueued_error(QueueState::from_raw(raw))));
    }

    match guard.take() {
        Some(payload) => {
            debug!("claim succeeded");
            Ok(payload)
        }
        None => unreachable!("payload vanished while the compartment lock was held"),
    }
}

fn not_enqueued_error(observed: QueueState) -> ClaimError {
    match observed {
        QueueState::Cancelled => ClaimError::Cancelled,
        other => ClaimError::NotEnqueued(other),
    }
}

fn log_claim_failure(error: ClaimError) -> ClaimError {
    debug!("claim failed: {error:?}");
    error
}

#[cfg(test)]
#[path = "test/queue_state.rs"]
mod tests;
