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
mod tests {
    use super::*;
    use parking_lot::Mutex;
    use std::io;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::{self, ThreadId};

    #[derive(Default)]
    struct Recorder {
        calls: Mutex<Vec<(Range<usize>, Hint)>>,
        advised: AtomicBool,
    }

    impl AdviceSink for Recorder {
        fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()> {
            self.calls.lock().push((range, hint));
            self.advised.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    struct Failing;

    impl AdviceSink for Failing {
        fn advise(&self, _: Range<usize>, _: Hint) -> io::Result<()> {
            Err(io::Error::other("unsupported"))
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn hint_precedes_the_read_which_runs_off_the_caller_thread() {
        let rec = Arc::new(Recorder::default());
        let reads = BlockingReads::new(Handle::current()).with_advice(rec.clone());
        let caller: ThreadId = thread::current().id();

        let probe = rec.clone();
        let (ran_on, hinted_first) = reads
            .read(16..64, move || (thread::current().id(), probe.advised.load(Ordering::SeqCst)))
            .await
            .unwrap();

        assert_ne!(ran_on, caller);
        assert!(hinted_first);
        assert_eq!(*rec.calls.lock(), vec![(16..64, Hint::WillNeed)]);
    }

    #[tokio::test]
    async fn rejected_hint_does_not_fail_the_read() {
        let reads = BlockingReads::new(Handle::current()).with_advice(Arc::new(Failing));
        assert_eq!(reads.read(0..8, || 5u32).await.unwrap(), 5);
    }

    #[tokio::test]
    async fn works_without_an_advice_sink() {
        let reads = BlockingReads::new(Handle::current());
        assert_eq!(reads.read(0..8, || "ok").await.unwrap(), "ok");
    }
}
