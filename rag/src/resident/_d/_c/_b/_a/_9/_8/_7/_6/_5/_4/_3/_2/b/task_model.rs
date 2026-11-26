use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock, TryLockError};
use log::{debug, warn};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::completion_wait::CompletionNotifier;
const ENQUEUED: u8 = 0;
const CLAIMED: u8 = 1;
const COMPLETED: u8 = 2;
const CANCELLED: u8 = 3;
/// Lifecycle of a task. Transitions only forward; Completed and Cancelled are final.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Enqueued,
    Claimed,
    Completed,
    Cancelled,
}
impl TaskState {
    fn from_raw(raw: u8) -> TaskState {
        match raw {
            ENQUEUED => TaskState::Enqueued,
            CLAIMED => TaskState::Claimed,
            COMPLETED => TaskState::Completed,
            CANCELLED => TaskState::Cancelled,
            other => panic!("task state holds invalid value {other}"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError2 {
    /// Task was not Enqueued; carries the observed state.
    NotEnqueued(TaskState),
    /// Task was cancelled.
    Cancelled,
    /// Payload compartment was locked at the moment of the attempt; state unchanged.
    Busy,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteError {
    /// State was not Claimed (carries the observed state); result unchanged.
    NotClaimed(TaskState),
}
/// Ordinary metadata. Immutable after creation. Not interpreted by this module.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskParams {
    pub priority: i32,
}
/// Error side of a task result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError<E> {
    Cancelled,
    Failed(E),
}
struct TaskInner<D, T, E> {
    params: TaskParams,
    state: AtomicU8,
    payload: Mutex<Option<D>>,
    result: OnceLock<Result<T, E>>,
    notifier: CompletionNotifier,
}
/// Cheaply clonable handle to one task; clones refer to the same task.
pub struct Task<D, T, E> {
    inner: Arc<TaskInner<D, T, E>>,
}
impl<D, T, E> Clone for Task<D, T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
impl<D, T, E> Task<D, T, E> {
    pub fn new(data: D, params: TaskParams) -> Self {
        Self {
            inner: Arc::new(TaskInner {
                params,
                state: AtomicU8::new(ENQUEUED),
                payload: Mutex::new(Some(data)),
                result: OnceLock::new(),
                notifier: CompletionNotifier::new(),
            }),
        }
    }
    pub fn params(&self) -> &TaskParams {
        &self.inner.params
    }
    pub fn state(&self) -> TaskState {
        TaskState::from_raw(self.inner.state.load(Ordering::Acquire))
    }
    /// `None` until the task is Completed or Cancelled.
    pub fn result(&self) -> Option<Result<&T, TaskError<&E>>> {
        match self.state() {
            TaskState::Completed => {
                self.inner
                    .result
                    .get()
                    .map(|r| match r {
                        Ok(value) => Ok(value),
                        Err(error) => Err(TaskError::Failed(error)),
                    })
            }
            TaskState::Cancelled => Some(Err(TaskError::Cancelled)),
            TaskState::Enqueued | TaskState::Claimed => None,
        }
    }
    /// Moves the task data out and marks the task Claimed. Exactly one caller can win.
    pub fn claim(&self) -> Result<D, ClaimError2> {
        let mut guard = match self.inner.payload.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => {
                debug!("claim failed: payload compartment busy");
                return Err(ClaimError2::Busy);
            }
        };
        if let Err(raw) = self
            .inner
            .state
            .compare_exchange(ENQUEUED, CLAIMED, Ordering::AcqRel, Ordering::Acquire)
        {
            let error = match TaskState::from_raw(raw) {
                TaskState::Cancelled => ClaimError2::Cancelled,
                other => ClaimError2::NotEnqueued(other),
            };
            debug!("claim failed: {error:?}");
            return Err(error);
        }
        match guard.take() {
            Some(data) => {
                debug!("claim succeeded");
                Ok(data)
            }
            None => {
                unreachable!("task data vanished while the compartment lock was held")
            }
        }
    }
    pub fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError> {
        let observed = self.state();
        if observed != TaskState::Claimed {
            warn!("complete rejected: task is {observed:?}, expected Claimed");
            if observed == TaskState::Completed {
                debug_assert!(false, "task completed more than once");
            }
            return Err(CompleteError::NotClaimed(observed));
        }
        if self.inner.result.set(result).is_err() {
            warn!("complete rejected: result already set by a concurrent completion");
            debug_assert!(false, "task completed more than once");
            return Err(CompleteError::NotClaimed(TaskState::Completed));
        }
        self.inner.state.store(COMPLETED, Ordering::Release);
        self.inner.notifier.notify_all();
        debug!("task completed");
        Ok(())
    }
    /// Cancels a task that is still Enqueued. Returns whether the cancellation took effect.
    pub fn cancel(&self) -> bool {
        match self
            .inner
            .state
            .compare_exchange(ENQUEUED, CANCELLED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => {
                self.inner.notifier.notify_all();
                debug!("cancel succeeded");
                true
            }
            Err(raw) => {
                debug!("cancel failed: task is {:?}", TaskState::from_raw(raw));
                false
            }
        }
    }
    /// Resolves once the task is Completed or Cancelled; returns the value of `result()`.
    pub async fn wait(&self) -> Result<&T, TaskError<&E>> {
        self.inner.notifier.wait_for_terminal(|| self.result()).await
    }
    pub fn notifier(&self) -> CompletionNotifier {
        self.inner.notifier.clone()
    }
}
#[cfg(test)]
#[path = "test/task_model.rs"]
mod tests;
