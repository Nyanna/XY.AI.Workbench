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
mod tests {
    use super::*;
    use crate::resident::memory::region::ArenaId;

    fn region() -> Region {
        Region { arena: ArenaId(0), offset: 0, len: 16, gen: 1 }
    }

    #[test]
    fn drop_decrements_pin() {
        let pin = PinCount::new();
        let a = Lease::new(region(), pin.clone(), None);
        let b = Lease::new(region(), pin.clone(), None);
        assert_eq!(pin.get(), 2);
        drop(a);
        assert!(!pin.is_unpinned());
        drop(b);
        assert!(pin.is_unpinned());
    }

    #[test]
    fn panic_does_not_leak_pin() {
        let pin = PinCount::new();
        let p = pin.clone();
        let r = std::panic::catch_unwind(move || {
            let _l = Lease::new(region(), p, None);
            panic!("boom");
        });
        assert!(r.is_err());
        assert!(pin.is_unpinned());
    }

    #[test]
    fn drop_sends_region_without_blocking() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let lease = Lease::new(region(), PinCount::new(), Some(tx));
        drop(lease);
        assert_eq!(rx.try_recv().unwrap(), region());
    }
}
