use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::Index;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::IndexId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagerError {
    IndexUnavailable { index_id: IndexId },
    InitFailed { index_id: IndexId, reason: String },
}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManagerError::IndexUnavailable { index_id } => {
                write!(f, "no index attached for {index_id}")
            }
            ManagerError::InitFailed { index_id, reason } => {
                write!(f, "initialization failed for {index_id}: {reason}")
            }
        }
    }
}

impl std::error::Error for ManagerError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagerStatus {
    pub index_id: IndexId,
    pub attached: bool,
    pub active: bool,
    pub resident_entries: usize,
}

/// Outcome slot shared by the initializer and the callers waiting on its attempt.
struct Attempt {
    result: Mutex<Option<Result<(), String>>>,
    done: Condvar,
}

impl Attempt {
    fn new() -> Arc<Self> {
        Arc::new(Attempt {
            result: Mutex::new(None),
            done: Condvar::new(),
        })
    }

    fn finish(&self, outcome: Result<(), String>) {
        let mut slot = self.result.lock().unwrap_or_else(|e| e.into_inner());
        *slot = Some(outcome);
        drop(slot);
        self.done.notify_all();
    }

    fn wait(&self) -> Result<(), String> {
        let mut slot = self.result.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(outcome) = slot.as_ref() {
                return outcome.clone();
            }
            slot = self.done.wait(slot).unwrap_or_else(|e| e.into_inner());
        }
    }
}

enum Phase2 {
    Inactive,
    Initializing(Arc<Attempt>),
    Active,
}

struct State6 {
    index: Option<Arc<dyn Index>>,
    phase: Phase2,
}

/// Marks the lifetime of one cycle. Dropping it ends the cycle scope, also while unwinding.
pub trait CycleGuard {}

impl<T> CycleGuard for T {}

/// Receives the begin of each cycle run by the manager.
pub trait CycleObserver: Send + Sync {
    fn enter(&self, index_id: &IndexId, cycle: &str) -> Box<dyn CycleGuard>;
}

/// Observer that does nothing.
pub struct NoCycleObserver;

impl CycleObserver for NoCycleObserver {
    fn enter(&self, _index_id: &IndexId, _cycle: &str) -> Box<dyn CycleGuard> {
        Box::new(())
    }
}

/// Observer that maps a cycle onto a tracing span.
pub struct TracingCycleObserver;

impl CycleObserver for TracingCycleObserver {
    fn enter(&self, index_id: &IndexId, cycle: &str) -> Box<dyn CycleGuard> {
        Box::new(
            tracing::info_span!("index_cycle", index_id = % index_id, cycle = % cycle)
                .entered(),
        )
    }
}

pub struct IndexManager {
    index_id: IndexId,
    state: Mutex<State6>,
    observer: Arc<dyn CycleObserver>,
}

enum Role3 {
    Done,
    Initializer(Arc<Attempt>),
    Waiter(Arc<Attempt>),
}

impl IndexManager {
    pub fn new(index_id: IndexId) -> Self {
        Self::with_observer(index_id, Arc::new(TracingCycleObserver))
    }

    pub fn with_observer(index_id: IndexId, observer: Arc<dyn CycleObserver>) -> Self {
        IndexManager {
            index_id,
            state: Mutex::new(State6 {
                index: None,
                phase: Phase2::Inactive,
            }),
            observer,
        }
    }

    pub fn index_id(&self) -> &IndexId {
        &self.index_id
    }

    fn lock(&self) -> MutexGuard<'_, State6> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn unavailable(&self) -> ManagerError {
        ManagerError::IndexUnavailable {
            index_id: self.index_id.clone(),
        }
    }

    pub fn attach(&self, index: Arc<dyn Index>) -> bool {
        let mut state = self.lock();
        if state.index.is_some() {
            drop(state);
            tracing::warn!(
                index_id = % self.index_id, "index already attached; attach ignored"
            );
            return false;
        }
        state.index = Some(index);
        true
    }

    pub fn index(&self) -> Result<Arc<dyn Index>, ManagerError> {
        self.lock().index.clone().ok_or_else(|| self.unavailable())
    }

    pub fn activate<F>(&self, init: F) -> Result<(), ManagerError>
    where
        F: FnOnce(&Arc<dyn Index>) -> Result<(), String>,
    {
        let (index, role) = {
            let mut state = self.lock();
            let index = state.index.clone().ok_or_else(|| self.unavailable())?;
            let role = match &state.phase {
                Phase2::Active => Role3::Done,
                Phase2::Initializing(attempt) => Role3::Waiter(Arc::clone(attempt)),
                Phase2::Inactive => {
                    let attempt = Attempt::new();
                    state.phase = Phase2::Initializing(Arc::clone(&attempt));
                    Role3::Initializer(attempt)
                }
            };
            (index, role)
        };
        let failed = |reason: String| ManagerError::InitFailed {
            index_id: self.index_id.clone(),
            reason,
        };
        match role {
            Role3::Done => Ok(()),
            Role3::Waiter(attempt) => attempt.wait().map_err(failed),
            Role3::Initializer(attempt) => {
                let outcome = match catch_unwind(AssertUnwindSafe(|| init(&index))) {
                    Ok(result) => result,
                    Err(_) => Err("init panicked".to_string()),
                };
                {
                    let mut state = self.lock();
                    state.phase = if outcome.is_ok() {
                        Phase2::Active
                    } else {
                        Phase2::Inactive
                    };
                }
                attempt.finish(outcome.clone());
                outcome.map_err(failed)
            }
        }
    }

    pub fn status(&self) -> ManagerStatus {
        let (index, active) = {
            let state = self.lock();
            (state.index.clone(), matches!(state.phase, Phase2::Active))
        };
        ManagerStatus {
            index_id: self.index_id.clone(),
            attached: index.is_some(),
            active,
            resident_entries: index.map_or(0, |i| i.stamps().len()),
        }
    }

    pub fn run_cycle<R, F>(&self, name: &str, f: F) -> Result<R, ManagerError>
    where
        F: FnOnce(&Arc<dyn Index>) -> R,
    {
        let index = self.index()?;
        let _scope = self.observer.enter(&self.index_id, name);
        Ok(f(&index))
    }
}

#[cfg(test)]
#[path = "test/manager_lifecycle.rs"]
mod tests;
