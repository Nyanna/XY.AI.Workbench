use std::any::Any;
use std::fmt;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    Event, EventError,
};

/// One 8-byte, 8-aligned u64 cell shared with the device.
pub struct DoorbellCell {
    ptr: NonNull<AtomicU64>,
    _keepalive: Arc<dyn Any + Send + Sync>,
}

unsafe impl Send for DoorbellCell {}
unsafe impl Sync for DoorbellCell {}

impl DoorbellCell {
    /// Host-only heap cell with initial value 0.
    pub fn new_local() -> DoorbellCell {
        let cell = Arc::new(AtomicU64::new(0));
        let ptr = NonNull::from(&*cell);
        DoorbellCell {
            ptr,
            _keepalive: cell,
        }
    }

    /// # Safety
    /// `ptr` must be 8-aligned, valid for reads and writes of 8 bytes and accessed
    /// only atomically, and must stay valid as long as `keepalive` lives.
    /// The cell content is the caller's responsibility (initially 0).
    pub unsafe fn from_raw(
        ptr: NonNull<AtomicU64>,
        keepalive: Arc<dyn Any + Send + Sync>,
    ) -> DoorbellCell {
        DoorbellCell {
            ptr,
            _keepalive: keepalive,
        }
    }

    fn atomic(&self) -> &AtomicU64 {
        // SAFETY: validity is guaranteed by the `from_raw` contract and the keepalive.
        unsafe { self.ptr.as_ref() }
    }

    fn load(&self, order: Ordering) -> u64 {
        self.atomic().load(order)
    }

    fn store(&self, value: u64, order: Ordering) {
        self.atomic().store(value, order)
    }

    #[cfg(test)]
    pub(crate) fn share(&self) -> DoorbellCell {
        DoorbellCell {
            ptr: self.ptr,
            _keepalive: Arc::clone(&self._keepalive),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_load(&self, order: Ordering) -> u64 {
        self.load(order)
    }

    #[cfg(test)]
    pub(crate) fn test_store(&self, value: u64, order: Ordering) {
        self.store(value, order)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorbellError {
    /// Device error from the batch event, passed on unchanged.
    Event(EventError),
    /// `completed < acked` (protocol violation).
    Inconsistent { completed: u64, acked: u64 },
}

impl fmt::Display for DoorbellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoorbellError::Event(e) => write!(f, "doorbell batch event failed: {e}"),
            DoorbellError::Inconsistent { completed, acked } => write!(
                f,
                "doorbell inconsistent: completed {completed} < acked {acked}"
            ),
        }
    }
}

impl std::error::Error for DoorbellError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DoorbellError::Event(e) => Some(e),
            DoorbellError::Inconsistent { .. } => None,
        }
    }
}

/// Host side of the doorbell.
pub struct Doorbell {
    completed: DoorbellCell,
    acked: DoorbellCell,
}

impl Doorbell {
    pub fn new(completed: DoorbellCell, acked: DoorbellCell) -> Doorbell {
        Doorbell { completed, acked }
    }

    /// Non-blocking. `Ok(None)` if `completed == acked`.
    pub fn poll(&mut self) -> Result<Option<Gap<'_>>, DoorbellError> {
        let completed = self.completed.load(Ordering::Acquire);
        let acked = self.acked.load(Ordering::Acquire);
        if completed == acked {
            return Ok(None);
        }
        if completed < acked {
            return Err(DoorbellError::Inconsistent { completed, acked });
        }
        Ok(Some(Gap {
            doorbell: self,
            first: acked + 1,
            last: completed,
            closed: false,
        }))
    }

    /// Waits for `event`, then behaves like `poll`.
    pub fn wait(&mut self, event: &Event) -> Result<Option<Gap<'_>>, DoorbellError> {
        event.wait().map_err(DoorbellError::Event)?;
        self.poll()
    }

    /// Current acked value, for diagnostics.
    pub fn acked(&self) -> u64 {
        self.acked.load(Ordering::Acquire)
    }
}

/// Open gap `(acked, completed]`. Exclusively borrows the `Doorbell`.
#[must_use = "an open gap must be closed"]
pub struct Gap<'a> {
    doorbell: &'a mut Doorbell,
    first: u64,
    last: u64,
    closed: bool,
}

impl Gap<'_> {
    /// Lowest unacked batch number.
    pub fn first(&self) -> u64 {
        self.first
    }

    /// Highest completed batch number as read.
    pub fn last(&self) -> u64 {
        self.last
    }

    /// Number of batches in the gap, always at least 1.
    pub fn len(&self) -> u64 {
        self.last - self.first + 1
    }

    /// Publishes `acked = last` with Release; the gap is closed.
    pub fn close(mut self) {
        self.doorbell.acked.store(self.last, Ordering::Release);
        self.closed = true;
    }
}

impl Drop for Gap<'_> {
    fn drop(&mut self) {
        if !self.closed && !std::thread::panicking() {
            panic!("doorbell gap dropped without close");
        }
    }
}

const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = {
    assert_send_sync_static::<DoorbellCell>();
    assert_send_sync_static::<Doorbell>();
    assert_send_sync_static::<DoorbellError>();
};

#[cfg(test)]
#[path = "test/doorbell.rs"]
mod tests;
