use std::fmt;
use std::future::{poll_fn, Future};
use std::pin::pin;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;
use tokio::time::{timeout, Instant};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::{
    EntryState, QueueState,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleError {
    Cancelled,
    NotReady,
    Timeout,
}
impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                HandleError::Cancelled => "cancelled",
                HandleError::NotReady => "noch kein Ergebnis",
                HandleError::Timeout => "timeout",
            },
        )
    }
}
impl std::error::Error for HandleError {}
/// Interval of the periodic state re-check while waiting.
pub const RECHECK_INTERVAL: Duration = Duration::from_millis(100);
/// Owned access to a finished task's result.
pub struct TaskOutcome<P, T, E> {
    entry: Arc<EntryState<P, T, E>>,
}
impl<P, T, E> TaskOutcome<P, T, E> {
    /// The task's output (Ok) or its error (Err).
    ///
    /// A `TaskOutcome` is built only after the entry's result was observed as set,
    /// and the entry never clears it, so the internal `expect` cannot fail.
    pub fn result(&self) -> &Result<T, E> {
        self.entry.result().expect("outcome is created only once the result is set")
    }
}
pub struct TaskHandle<P, T, E> {
    entry: Arc<EntryState<P, T, E>>,
}
impl<P, T, E> TaskHandle<P, T, E> {
    pub fn new(entry: Arc<EntryState<P, T, E>>) -> Self {
        Self { entry }
    }
    pub fn state(&self) -> QueueState {
        self.entry.state()
    }
    pub fn try_result(&self) -> Result<&Result<T, E>, HandleError> {
        Self::evaluate(&self.entry, self.entry.state())
    }
    pub async fn wait(self) -> Result<TaskOutcome<P, T, E>, HandleError> {
        TaskHandle::<P, T, E>::wait_until(self, None).await
    }
    pub async fn wait_timeout(
        self,
        timeout: Duration,
    ) -> Result<TaskOutcome<P, T, E>, HandleError> {
        TaskHandle::<P, T, E>::wait_until(self, Some(Instant::now() + timeout)).await
    }
    fn evaluate(
        entry: &EntryState<P, T, E>,
        state: QueueState,
    ) -> Result<&Result<T, E>, HandleError> {
        match state {
            QueueState::Enqueued | QueueState::Claimed => Err(HandleError::NotReady),
            QueueState::Cancelled => Err(HandleError::Cancelled),
            QueueState::Completed => entry.result().ok_or(HandleError::Cancelled),
        }
    }
    async fn wait_until(
        self,
        deadline: Option<Instant>,
    ) -> Result<TaskOutcome<P, T, E>, HandleError> {
        loop {
            let mut finished = pin!(self.entry.wait_finished());
            let ready = poll_fn(|cx| Poll::Ready(finished.as_mut().poll(cx).is_ready()))
                .await;
            let state = self.entry.state();
            match Self::evaluate(&self.entry, state) {
                Ok(_) => {
                    return Ok(TaskOutcome {
                        entry: Arc::clone(&self.entry),
                    });
                }
                Err(HandleError::NotReady) => {}
                Err(e) => return Err(e),
            }
            let slice = match deadline {
                None => RECHECK_INTERVAL,
                Some(d) => {
                    let remaining = d.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        return Err(HandleError::Timeout);
                    }
                    remaining.min(RECHECK_INTERVAL)
                }
            };
            if !ready {
                let _ = timeout(slice, finished.as_mut()).await;
            }
        }
    }
}
impl<P, T, E> Drop for TaskHandle<P, T, E> {
    fn drop(&mut self) {
        let _ = self.entry.cancel();
    }
}
#[cfg(test)]
#[path = "test/task_handle.rs"]
mod tests;
