//! Index reads that may page-fault or hit disk run on the blocking pool, never on an async
//! worker, after the page cache has been hinted.
use std::ops::Range;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::task::JoinError;
use crate::resident::manager::advice::{AdviceSink, Hint};
pub struct BlockingReads {
    handle: Handle,
    advice: Option<Arc<dyn AdviceSink>>,
}
impl BlockingReads {
    pub fn new(handle: Handle) -> Self {
        Self { handle, advice: None }
    }
    pub fn with_advice(mut self, sink: Arc<dyn AdviceSink>) -> Self {
        self.advice = Some(sink);
        self
    }
    pub fn prefetch(&self, range: Range<usize>) {
        self.advise(range, Hint::WillNeed);
    }
    /// Sets `hint` on `range`. A rejected hint is logged and otherwise ignored.
    pub fn advise(&self, range: Range<usize>, hint: Hint) {
        let Some(sink) = &self.advice else { return };
        match sink.advise(range.clone(), hint) {
            Ok(()) => tracing::debug!(? range, ? hint, "residency hint set"),
            Err(e) => tracing::debug!(error = % e, ? hint, "residency hint rejected"),
        }
    }
    pub async fn read<T, F>(&self, range: Range<usize>, read: F) -> Result<T, JoinError>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.read_hinted(range, Hint::WillNeed, read).await
    }
    /// Like `read` with an explicit hint, e.g. `Sequential` for dense scans.
    pub async fn read_hinted<T, F>(
        &self,
        range: Range<usize>,
        hint: Hint,
        read: F,
    ) -> Result<T, JoinError>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.advise(range, hint);
        self.run(read).await
    }
    /// Runs `read` on the blocking pool inside the caller's span without hinting.
    pub async fn run<T, F>(&self, read: F) -> Result<T, JoinError>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        let span = tracing::Span::current();
        self.handle.spawn_blocking(move || span.in_scope(read)).await
    }
}
#[cfg(test)]
#[path = "test/blocking.rs"]
mod tests;
