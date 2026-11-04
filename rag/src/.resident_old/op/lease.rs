//! RAII lease on a device region; pinned regions are not evicted.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::resident::memory::Region;

#[derive(Debug, Default)]
pub struct PinCount(AtomicUsize);

impl PinCount {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn get(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }

    pub fn is_unpinned(&self) -> bool {
        self.get() == 0
    }
}

#[derive(Debug)]
pub struct Lease {
    region: Region,
    pin: Arc<PinCount>,
    release: Option<UnboundedSender<Region>>,
}

pub type DeviceHandle = Lease;

impl Lease {
    /// Increments the pin count; `release` receives the region on drop.
    pub fn new(region: Region, pin: Arc<PinCount>, release: Option<UnboundedSender<Region>>) -> Self {
        pin.0.fetch_add(1, Ordering::AcqRel);
        Self { region, pin, release }
    }

    pub fn region(&self) -> Region {
        self.region
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        self.pin.0.fetch_sub(1, Ordering::AcqRel);
        if let Some(tx) = &self.release {
            let _ = tx.send(self.region);
        }
    }
}

#[cfg(test)]
#[path = "test/lease.rs"]
mod tests;
