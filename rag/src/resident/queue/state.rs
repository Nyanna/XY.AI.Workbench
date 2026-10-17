//! Claim state machine and result cell shared by every queue node.
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use tokio::sync::Notify;
#[cfg(loom)]
use loom::sync::atomic::AtomicU8;
#[cfg(not(loom))]
use std::sync::atomic::AtomicU8;
const QUEUED: u8 = 0;
const TAKEN: u8 = 1;
const DONE: u8 = 2;
const CANCELLED: u8 = 3;
/// Snapshot of a [`TaskState`]'s atomic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimState {
    Queued,
    Taken,
    Done,
    Cancelled,
}
impl From<u8> for ClaimState {
    fn from(value: u8) -> Self {
        match value {
            QUEUED => ClaimState::Queued,
            TAKEN => ClaimState::Taken,
            DONE => ClaimState::Done,
            CANCELLED => ClaimState::Cancelled,
            other => unreachable!("invalid claim state byte: {other}"),
        }
    }
}
/// Per-task claim/result/wakeup cell. Every transition is a single
/// `compare_exchange`; completion and cancellation notify all waiters.
pub struct TaskState<O, E> {
    state: AtomicU8,
    result: OnceLock<Result<O, E>>,
    ready: Notify,
}
impl<O, E> Default for TaskState<O, E> {
    fn default() -> Self {
        Self::new()
    }
}
impl<O, E> TaskState<O, E> {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(QUEUED),
            result: OnceLock::new(),
            ready: Notify::new(),
        }
    }
    pub fn claim_state(&self) -> ClaimState {
        ClaimState::from(self.state.load(Ordering::Acquire))
    }
    /// `QUEUED -> TAKEN`. The only transition where cancellation matters.
    pub(crate) fn claim(&self) -> bool {
        let ok = self
            .state
            .compare_exchange(QUEUED, TAKEN, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        tracing::debug!(ok, "claim");
        ok
    }
    /// Sets the result and transitions `TAKEN -> DONE`. Must be called at
    /// most once, by the claim winner only.
    pub fn complete(&self, out: Result<O, E>) {
        if self.result.set(out).is_err() {
            debug_assert!(false, "TaskState::result set more than once");
            tracing::warn!("task result already set; ignoring duplicate completion");
            return;
        }
        self.state.store(DONE, Ordering::Release);
        self.ready.notify_waiters();
        tracing::debug!("complete");
    }
    /// `QUEUED -> CANCELLED`. No-op if already `TAKEN`/`DONE`/`CANCELLED`.
    pub fn cancel(&self) -> bool {
        let ok = self
            .state
            .compare_exchange(QUEUED, CANCELLED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        if ok {
            self.ready.notify_waiters();
        }
        tracing::debug!(ok, "cancel");
        ok
    }
    pub fn result(&self) -> Option<&Result<O, E>> {
        self.result.get()
    }
    /// Waits for `Done`/`Cancelled`. Registers interest before checking state
    /// so a completion landing between check and await is never missed.
    pub async fn wait(&self) {
        loop {
            let notified = self.ready.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            match self.claim_state() {
                ClaimState::Done | ClaimState::Cancelled => return,
                _ => notified.await,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn claim_once_wins() {
        let s: TaskState<u32, ()> = TaskState::new();
        assert!(s.claim());
        assert!(! s.claim());
        assert_eq!(s.claim_state(), ClaimState::Taken);
    }
    #[test]
    fn cancel_before_claim_blocks_claim() {
        let s: TaskState<u32, ()> = TaskState::new();
        assert!(s.cancel());
        assert!(! s.claim());
        assert_eq!(s.claim_state(), ClaimState::Cancelled);
    }
    #[test]
    fn cancel_after_claim_is_noop() {
        let s: TaskState<u32, ()> = TaskState::new();
        assert!(s.claim());
        assert!(! s.cancel());
        assert_eq!(s.claim_state(), ClaimState::Taken);
    }
    #[test]
    fn complete_sets_result_before_done_is_visible() {
        let s: TaskState<u32, ()> = TaskState::new();
        assert!(s.claim());
        s.complete(Ok(42));
        assert_eq!(s.claim_state(), ClaimState::Done);
        assert_eq!(s.result(), Some(& Ok(42)));
    }
    #[tokio::test]
    async fn wait_sees_completion_set_before_await() {
        let s: std::sync::Arc<TaskState<u32, ()>> = std::sync::Arc::new(
            TaskState::new(),
        );
        s.claim();
        s.complete(Ok(7));
        s.wait().await;
        assert_eq!(s.result(), Some(& Ok(7)));
    }
    #[tokio::test]
    async fn wait_wakes_on_later_completion() {
        let s: std::sync::Arc<TaskState<u32, ()>> = std::sync::Arc::new(
            TaskState::new(),
        );
        s.claim();
        let s2 = s.clone();
        let waiter = tokio::spawn(async move { s2.wait().await });
        tokio::task::yield_now().await;
        s.complete(Ok(1));
        waiter.await.unwrap();
    }
}
