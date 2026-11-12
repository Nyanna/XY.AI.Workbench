use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellId(pub u64);

/// Totally ordered change stamp. `Stamp(0)` is never assigned to a write or delete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Stamp(pub u64);

/// Shared immutable bytes; cloning is cheap.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SharedBytes(pub Arc<[u8]>);

impl Deref for SharedBytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for SharedBytes {
    fn from(bytes: &[u8]) -> Self {
        SharedBytes(Arc::from(bytes))
    }
}

impl From<Vec<u8>> for SharedBytes {
    fn from(bytes: Vec<u8>) -> Self {
        SharedBytes(Arc::from(bytes))
    }
}

/// Borrowed payload for writing.
#[derive(Debug, Clone, Copy)]
pub struct PayloadRef<'a> {
    pub vector: &'a [u8],
    pub meta: &'a [u8],
}

/// Shared payload as read. Both parts are empty for a missing entry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Payload {
    pub vector: SharedBytes,
    pub meta: SharedBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessMethod {
    Centroid,
    Prefix,
    Subtree,
    Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexError {
    InvalidVectorLength { len: usize, elem_size: usize },
    Unsupported(AccessMethod),
    NotFound,
    Backend(String),
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexError::InvalidVectorLength { len, elem_size } => write!(
                f,
                "vector length {len} is not a multiple of element size {elem_size}"
            ),
            IndexError::Unsupported(method) => {
                write!(f, "access method {method:?} is not supported by this index")
            }
            IndexError::NotFound => write!(f, "unknown node or cell"),
            IndexError::Backend(msg) => write!(f, "index backend error: {msg}"),
        }
    }
}

impl std::error::Error for IndexError {}

/// Summary of a node (centroid access).
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub node: NodeId,
    pub centroid: SharedBytes,
    pub count: u64,
    pub stamp: Stamp,
}

/// Contiguously stored entries (range access).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub first: EntryId,
    pub count: u64,
    pub offset: u64,
    pub len: u64,
}

pub trait Index: Send + Sync {
    fn dtype(&self) -> DType;
    fn access_methods(&self) -> &[AccessMethod];

    fn stamp(&self, id: EntryId) -> Option<Stamp>;
    /// Snapshot of all (id, stamp), ascending by id.
    fn stamps(&self) -> Vec<(EntryId, Stamp)>;
    /// Reads the entry; may block. Missing entry yields `Payload::default()`.
    fn read(&self, id: EntryId) -> Payload;
    /// Replaces the entry entirely. Returns the stamp of exactly this write.
    fn write(&self, id: EntryId, payload: PayloadRef<'_>) -> Result<Stamp, IndexError>;

    /// Writes in order, stopping at the first error. Not atomic.
    fn write_many(
        &self,
        items: &[(EntryId, PayloadRef<'_>)],
    ) -> Result<Vec<(EntryId, Stamp)>, IndexError> {
        let mut out = Vec::with_capacity(items.len());
        for (id, payload) in items {
            let stamp = self.write(*id, *payload)?;
            out.push((*id, stamp));
        }
        Ok(out)
    }

    /// Deletes the entry. `Some(stamp)` of the delete, `None` if absent.
    fn delete(&self, id: EntryId) -> Result<Option<Stamp>, IndexError>;

    /// Level signal; a new observer starts at the current max stamp.
    fn subscribe(&self) -> StampObserver;

    fn summary(&self, node: NodeId) -> Result<Summary, IndexError> {
        let _ = node;
        Err(IndexError::Unsupported(AccessMethod::Centroid))
    }

    fn children(&self, node: NodeId) -> Result<Vec<NodeId>, IndexError> {
        let _ = node;
        Err(IndexError::Unsupported(AccessMethod::Subtree))
    }

    fn root(&self) -> Result<NodeId, IndexError> {
        Err(IndexError::Unsupported(AccessMethod::Subtree))
    }

    fn cell_entries(&self, cell: CellId) -> Result<Vec<EntryId>, IndexError> {
        let _ = cell;
        Err(IndexError::Unsupported(AccessMethod::Prefix))
    }

    fn byte_ranges(&self, first: EntryId, last: EntryId) -> Result<Vec<ByteRange>, IndexError> {
        let _ = (first, last);
        Err(IndexError::Unsupported(AccessMethod::Range))
    }
}

/// Monotonic level signal carrying the current max stamp.
pub struct StampSignal {
    value: Mutex<Stamp>,
    changed: Condvar,
}

impl StampSignal {
    pub fn new(initial: Stamp) -> Arc<StampSignal> {
        Arc::new(StampSignal {
            value: Mutex::new(initial),
            changed: Condvar::new(),
        })
    }

    /// Sets the value to `max(old, new)` and wakes all observers.
    pub fn publish(&self, stamp: Stamp) {
        let mut guard = self.value.lock().unwrap_or_else(|e| e.into_inner());
        if stamp > *guard {
            *guard = stamp;
        }
        drop(guard);
        self.changed.notify_all();
    }

    pub fn current(&self) -> Stamp {
        *self.value.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn observer(self: &Arc<Self>) -> StampObserver {
        StampObserver {
            signal: Arc::clone(self),
            seen: self.current(),
        }
    }
}

#[derive(Clone)]
pub struct StampObserver {
    signal: Arc<StampSignal>,
    seen: Stamp,
}

impl StampObserver {
    pub fn current(&self) -> Stamp {
        self.signal.current()
    }

    /// Blocks until the signal exceeds the last seen stamp, or the timeout elapses.
    pub fn wait_newer(&mut self, timeout: Option<Duration>) -> Option<Stamp> {
        let deadline = timeout.map(|t| Instant::now() + t);
        let mut guard = self.signal.value.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if *guard > self.seen {
                self.seen = *guard;
                return Some(self.seen);
            }
            match deadline {
                None => {
                    guard = self
                        .signal
                        .changed
                        .wait(guard)
                        .unwrap_or_else(|e| e.into_inner());
                }
                Some(deadline) => {
                    let now = Instant::now();
                    if now >= deadline {
                        return None;
                    }
                    guard = self
                        .signal
                        .changed
                        .wait_timeout(guard, deadline - now)
                        .unwrap_or_else(|e| e.into_inner())
                        .0;
                }
            }
        }
    }
}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<StampSignal>();
    assert_send_sync::<StampObserver>();
    assert_send_sync::<Payload>();
    assert_send_sync::<IndexError>();
};

#[allow(dead_code)]
fn _object_safe(_: &dyn Index) {}

#[cfg(test)]
#[path = "test/index_contract.rs"]
mod tests;
