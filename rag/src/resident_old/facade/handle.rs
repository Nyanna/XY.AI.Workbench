use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use super::error::TaskError;
use super::params::Output;
use crate::resident::queue::{ClaimState, TaskState};

pub(crate) type State = TaskState<Output, TaskError>;

/// Dropping an unfinished handle cancels the task if no worker has claimed it yet.
pub struct TaskHandle {
    state: Arc<State>,
    id: u64,
}

impl TaskHandle {
    pub(crate) fn new(state: Arc<State>, id: u64) -> Self {
        Self { state, id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn try_get(&self) -> Option<Result<Output, TaskError>> {
        match self.state.claim_state() {
            ClaimState::Done => Some(
                self.state
                    .result()
                    .cloned()
                    .unwrap_or(Err(TaskError::Cancelled)),
            ),
            ClaimState::Cancelled => Some(Err(TaskError::Cancelled)),
            ClaimState::Queued | ClaimState::Taken => None,
        }
    }

    /// On timeout the handle is dropped, which cancels the task if still queued.
    pub async fn wait_timeout(self, limit: Duration) -> Result<Output, TaskError> {
        tokio::time::timeout(limit, self.resolve())
            .await
            .unwrap_or(Err(TaskError::Timeout))
    }

    async fn resolve(self) -> Result<Output, TaskError> {
        self.state.wait().await;
        self.try_get().unwrap_or(Err(TaskError::Cancelled))
    }
}

impl IntoFuture for TaskHandle {
    type Output = Result<Output, TaskError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.resolve())
    }
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.state.cancel();
    }
}

impl std::fmt::Debug for TaskHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskHandle")
            .field("id", &self.id)
            .field("state", &self.state.claim_state())
            .finish()
    }
}
