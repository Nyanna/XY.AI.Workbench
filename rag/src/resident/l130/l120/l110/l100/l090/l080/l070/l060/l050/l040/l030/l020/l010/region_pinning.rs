use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;

/// Shared pin counter of one region. Clones share the same counter.
#[derive(Clone, Debug, Default)]
pub struct PinCount(Arc<AtomicUsize>);

impl PinCount {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub fn count(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }

    pub fn is_unpinned(&self) -> bool {
        self.count() == 0
    }
}

/// RAII pin on a region.
pub struct Pin<R> {
    region: Option<R>,
    count: PinCount,
    release_tx: Option<Sender<R>>,
}

impl<R> Pin<R> {
    pub fn new(region: R, count: &PinCount) -> Pin<R> {
        count.0.fetch_add(1, Ordering::AcqRel);
        Pin {
            region: Some(region),
            count: count.clone(),
            release_tx: None,
        }
    }

    pub fn with_release_channel(region: R, count: &PinCount, release_tx: Sender<R>) -> Pin<R> {
        let mut pin = Pin::new(region, count);
        pin.release_tx = Some(release_tx);
        pin
    }

    pub fn region(&self) -> &R {
        self.region.as_ref().expect("region present until drop")
    }
}

impl<R> Drop for Pin<R> {
    fn drop(&mut self) {
        self.count.0.fetch_sub(1, Ordering::AcqRel);
        if let (Some(tx), Some(region)) = (self.release_tx.take(), self.region.take()) {
            let _ = tx.send(region);
        }
    }
}

/// Alternative name for a pin.
pub type DeviceHandle<R> = Pin<R>;

#[cfg(test)]
#[path = "test/region_pinning.rs"]
mod tests;
