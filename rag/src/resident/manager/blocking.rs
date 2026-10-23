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

    /// Asks the OS to read `range` ahead without waiting for it. Hints may be ignored.
    pub fn prefetch(&self, range: Range<usize>) {
        if let Some(sink) = &self.advice {
            if let Err(e) = sink.advise(range, Hint::WillNeed) {
                tracing::debug!(error = % e, "prefetch hint rejected");
            }
        }
    }

    /// Hints `range`, then runs `read` on the blocking pool inside the caller's span.
    pub async fn read<T, F>(&self, range: Range<usize>, read: F) -> Result<T, JoinError>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.prefetch(range);
        let span = tracing::Span::current();
        self.handle.spawn_blocking(move || span.in_scope(read)).await
    }
}

#[cfg(test)]
#[path = "test/blocking.rs"]
mod tests;
