use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::completion_wait::CompletionNotifier;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::{
    ClaimError, CompleteError, EntryState, QueueState,
};

/// A task has exactly the queue states: Enqueued, Claimed, Completed, Cancelled.
pub type TaskState = QueueState;

/// Ordinary metadata. Immutable after creation. Not interpreted by this module.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskParams {
    pub priority: i32,
    pub resource: Option<String>,
    pub model: Option<String>,
    pub mode: Option<String>,
}

/// Error side of a task result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError<E> {
    Cancelled,
    Failed(E),
}

struct TaskInner<D, T, E> {
    params: TaskParams,
    entry: Arc<EntryState<D, T, E>>,
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
                entry: EntryState::new(data),
                notifier: CompletionNotifier::new(),
            }),
        }
    }

    pub fn params(&self) -> &TaskParams {
        &self.inner.params
    }

    pub fn state(&self) -> TaskState {
        self.inner.entry.state()
    }

    /// `None` until the task is Completed or Cancelled.
    pub fn result(&self) -> Option<Result<&T, TaskError<&E>>> {
        match self.inner.entry.state() {
            QueueState::Completed => self.inner.entry.result().map(|r| match r {
                Ok(value) => Ok(value),
                Err(error) => Err(TaskError::Failed(error)),
            }),
            QueueState::Cancelled => Some(Err(TaskError::Cancelled)),
            QueueState::Enqueued | QueueState::Claimed => None,
        }
    }

    pub fn claim(&self) -> Result<D, ClaimError> {
        self.inner.entry.claim()
    }

    pub fn complete(&self, result: Result<T, E>) -> Result<(), CompleteError> {
        self.inner.entry.complete(result)?;
        self.inner.notifier.notify_all();
        Ok(())
    }

    pub fn cancel(&self) -> bool {
        if self.inner.entry.cancel() {
            self.inner.notifier.notify_all();
            true
        } else {
            false
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
