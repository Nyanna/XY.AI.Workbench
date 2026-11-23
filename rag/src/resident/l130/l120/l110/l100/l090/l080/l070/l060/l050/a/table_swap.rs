//! Segment table of the index in VRAM: immutable versions, root pointer,
//! epoch tracking with readers, serialized publication, retire and reclaim.
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use log::{info, warn};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::allocator::{
    Allocator, AllocatorError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::{
    Generation, RegionHandle,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::AccessRegion;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::Event;
pub type Epoch = u64;
/// Root value before the first table has been published.
pub const ROOT_SENTINEL: u64 = u64::MAX;
pub const TABLE_HEADER_BYTES: usize = 16;
pub const SEGMENT_BYTES: usize = 24;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableSwapError {
    StaleMapping { expected: Epoch, got: Epoch },
    Allocation(AllocatorError),
    /// Error reported by the `TableSink`.
    Write(String),
    /// Nothing has been published yet.
    NoTable,
    UnknownReader(ReaderId),
    EpochAhead { reported: Epoch, published: Epoch },
    Decode(String),
}
impl fmt::Display for TableSwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableSwapError::StaleMapping { .. } => write!(f, "stale Mapping"),
            TableSwapError::Allocation(e) => write!(f, "table allocation failed: {e}"),
            TableSwapError::Write(text) => write!(f, "table write failed: {text}"),
            TableSwapError::NoTable => write!(f, "no table published"),
            TableSwapError::UnknownReader(id) => write!(f, "unknown reader {}", id.0),
            TableSwapError::EpochAhead { reported, published } => {
                write!(
                    f,
                    "reported epoch {reported} is ahead of published epoch {published}"
                )
            }
            TableSwapError::Decode(text) => write!(f, "table decode failed: {text}"),
        }
    }
}
impl std::error::Error for TableSwapError {}
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}
/// Ordered list of segment regions of one epoch. Immutable once published.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SegmentTable {
    epoch: Epoch,
    segments: Vec<RegionHandle>,
}
impl SegmentTable {
    pub fn new(epoch: Epoch) -> SegmentTable {
        SegmentTable {
            epoch,
            segments: Vec::new(),
        }
    }
    pub fn epoch(&self) -> Epoch {
        self.epoch
    }
    pub fn segments(&self) -> &[RegionHandle] {
        &self.segments
    }
    pub fn len(&self) -> usize {
        self.segments.len()
    }
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
    pub fn add(&mut self, region: RegionHandle) {
        self.segments.push(region);
    }
    /// Replaces the first occurrence of `old` in place; `false` if `old` is missing.
    pub fn replace(&mut self, old: &RegionHandle, new: RegionHandle) -> bool {
        match self.segments.iter().position(|r| r == old) {
            Some(i) => {
                self.segments[i] = new;
                true
            }
            None => false,
        }
    }
    /// Removes the first occurrence of `region`, keeping the order; `false` if missing.
    pub fn remove(&mut self, region: &RegionHandle) -> bool {
        match self.segments.iter().position(|r| r == region) {
            Some(i) => {
                self.segments.remove(i);
                true
            }
            None => false,
        }
    }
    pub fn encoded_len(&self) -> usize {
        TABLE_HEADER_BYTES + SEGMENT_BYTES * self.segments.len()
    }
    /// Little endian: header `epoch u64, count u32, reserved u32`, then per segment
    /// `arena_id u32, generation u32, offset u64, len u64`. The generation is stored modulo 2^32.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.encoded_len());
        out.extend_from_slice(&self.epoch.to_le_bytes());
        let count = u32::try_from(self.segments.len()).unwrap_or(u32::MAX);
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        for r in &self.segments {
            out.extend_from_slice(&r.arena().id().to_le_bytes());
            out.extend_from_slice(&(r.generation().0 as u32).to_le_bytes());
            out.extend_from_slice(&r.offset().to_le_bytes());
            out.extend_from_slice(&r.len().to_le_bytes());
        }
        out
    }
    pub fn decode(bytes: &[u8]) -> Result<SegmentTable, TableSwapError> {
        let decode_err = |text: String| TableSwapError::Decode(text);
        if bytes.len() < TABLE_HEADER_BYTES {
            return Err(
                decode_err(
                    format!(
                        "{} bytes is shorter than the {TABLE_HEADER_BYTES}-byte header",
                        bytes.len()
                    ),
                ),
            );
        }
        let epoch = read_u64(bytes, 0);
        let count = read_u32(bytes, 8);
        let reserved = read_u32(bytes, 12);
        if reserved != 0 {
            return Err(decode_err(format!("reserved header field is {reserved}")));
        }
        let expected = usize::try_from(count)
            .ok()
            .and_then(|c| c.checked_mul(SEGMENT_BYTES))
            .and_then(|b| b.checked_add(TABLE_HEADER_BYTES))
            .ok_or_else(|| decode_err(format!("segment count {count} overflows")))?;
        if bytes.len() != expected {
            return Err(
                decode_err(
                    format!(
                        "length {} does not match {expected} bytes for {count} segments",
                        bytes.len()
                    ),
                ),
            );
        }
        let mut segments = Vec::with_capacity(bytes.len() / SEGMENT_BYTES);
        for chunk in bytes[TABLE_HEADER_BYTES..].chunks_exact(SEGMENT_BYTES) {
            let arena_id = read_u32(chunk, 0);
            let arena = ArenaKind::from_id(arena_id)
                .ok_or_else(|| decode_err(format!("unknown arena id {arena_id}")))?;
            let generation = Generation(u64::from(read_u32(chunk, 4)));
            let region = RegionHandle::new(
                    arena,
                    read_u64(chunk, 8),
                    read_u64(chunk, 16),
                    generation,
                )
                .map_err(|e| decode_err(e.to_string()))?;
            segments.push(region);
        }
        Ok(SegmentTable { epoch, segments })
    }
}
fn read_u32(bytes: &[u8], at: usize) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&bytes[at..at + 4]);
    u32::from_le_bytes(buf)
}
fn read_u64(bytes: &[u8], at: usize) -> u64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[at..at + 8]);
    u64::from_le_bytes(buf)
}
/// Storage of the root value. Only the host writes; the device reads with acquire.
pub trait RootSlot: Send + Sync {
    fn store_release(&self, value: u64);
    fn load_acquire(&self) -> u64;
}
pub struct HostRootSlot {
    value: AtomicU64,
}
impl HostRootSlot {
    pub fn new() -> HostRootSlot {
        HostRootSlot {
            value: AtomicU64::new(ROOT_SENTINEL),
        }
    }
}
impl Default for HostRootSlot {
    fn default() -> Self {
        HostRootSlot::new()
    }
}
impl RootSlot for HostRootSlot {
    fn store_release(&self, value: u64) {
        self.value.store(value, Ordering::Release);
    }
    fn load_acquire(&self) -> u64 {
        self.value.load(Ordering::Acquire)
    }
}
pub trait TableSink: Send + Sync {
    /// Writes `bytes` from the start of `region`. Returns only once the content is
    /// completely visible to the device (copy event completed or flush done).
    fn write_visible(&self, region: &RegionHandle, bytes: &[u8]) -> Result<(), String>;
}
type ResolveFn = dyn Fn(&RegionHandle) -> Option<AccessRegion> + Send + Sync;
type AfterWriteFn = dyn Fn() -> Result<Option<Event>, String> + Send + Sync;
/// Sink on top of `memory_access`: resolve, write, `after_write`, `Event::wait`.
pub struct AccessSink {
    resolve: Box<ResolveFn>,
    after_write: Box<AfterWriteFn>,
}
impl AccessSink {
    /// `after_write` runs after the write. Mapped memory: perform the flush and
    /// return `Ok(None)`. Copy stream: return `Ok(Some(event))` of the copy.
    pub fn new(resolve: Box<ResolveFn>, after_write: Box<AfterWriteFn>) -> AccessSink {
        AccessSink { resolve, after_write }
    }
}
impl TableSink for AccessSink {
    fn write_visible(&self, region: &RegionHandle, bytes: &[u8]) -> Result<(), String> {
        let target = (self.resolve)(region)
            .ok_or_else(|| {
                format!("region at offset {} cannot be resolved", region.offset())
            })?;
        target.write(0, bytes).map_err(|e| e.to_string())?;
        if let Some(event) = (self.after_write)()? {
            event.wait().map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReaderId(pub u32);
struct ReaderState {
    reported: Epoch,
    quiescent: bool,
}
struct Tracker {
    readers: HashMap<ReaderId, ReaderState>,
    next_id: u32,
}
struct Published {
    epoch: Epoch,
    table: Option<Arc<SegmentTable>>,
    region: Option<RegionHandle>,
}
struct Retired {
    region: RegionHandle,
    retire_epoch: Epoch,
}
pub struct TableSwap {
    allocator: Arc<Allocator>,
    sink: Arc<dyn TableSink>,
    root: Arc<dyn RootSlot>,
    publish_lock: Mutex<()>,
    state: Mutex<Published>,
    tracker: Mutex<Tracker>,
    retired: Mutex<Vec<Retired>>,
}
static_assertions::assert_impl_all!(TableSwap : Send, Sync);
impl TableSwap {
    /// Writes `ROOT_SENTINEL` into `root`; the published epoch starts at 0.
    pub fn new(
        allocator: Arc<Allocator>,
        sink: Arc<dyn TableSink>,
        root: Arc<dyn RootSlot>,
    ) -> TableSwap {
        root.store_release(ROOT_SENTINEL);
        TableSwap {
            allocator,
            sink,
            root,
            publish_lock: Mutex::new(()),
            state: Mutex::new(Published {
                epoch: 0,
                table: None,
                region: None,
            }),
            tracker: Mutex::new(Tracker {
                readers: HashMap::new(),
                next_id: 0,
            }),
            retired: Mutex::new(Vec::new()),
        }
    }
    /// Independent copy of the current table with epoch = published + 1.
    pub fn next_table(&self) -> SegmentTable {
        let state = lock(&self.state);
        let epoch = state.epoch.saturating_add(1);
        match &state.table {
            Some(table) => {
                SegmentTable {
                    epoch,
                    segments: table.segments.clone(),
                }
            }
            None => SegmentTable::new(epoch),
        }
    }
    /// Publishes `table`; returns the new published epoch. The table content is
    /// visible to the device before the root is stored.
    pub fn publish(&self, table: SegmentTable) -> Result<Epoch, TableSwapError> {
        let _serial = lock(&self.publish_lock);
        let published = lock(&self.state).epoch;
        let expected = published.saturating_add(1);
        if table.epoch() != expected {
            return Err(TableSwapError::StaleMapping {
                expected,
                got: table.epoch(),
            });
        }
        let bytes = table.encode();
        let region = self
            .allocator
            .allocate(bytes.len() as u64)
            .map_err(TableSwapError::Allocation)?;
        if let Err(text) = self.sink.write_visible(&region, &bytes) {
            if let Err(e) = self.allocator.free(&region) {
                warn!(
                    "table region at offset {} not freed after failed write: {e}", region
                    .offset()
                );
            }
            return Err(TableSwapError::Write(text));
        }
        let epoch = table.epoch();
        let segment_count = table.len();
        let table = Arc::new(table);
        {
            let mut state = lock(&self.state);
            self.root.store_release(region.offset());
            let previous = state.region.replace(region);
            state.table = Some(table);
            state.epoch = epoch;
            if let Some(old) = previous {
                lock(&self.retired)
                    .push(Retired {
                        region: old,
                        retire_epoch: epoch,
                    });
            }
        }
        info!(
            "table published: epoch={epoch} segments={segment_count} offset={}", region
            .offset()
        );
        Ok(epoch)
    }
    pub fn published_epoch(&self) -> Epoch {
        lock(&self.state).epoch
    }
    pub fn root_value(&self) -> u64 {
        self.root.load_acquire()
    }
    pub fn current_table(&self) -> Option<Arc<SegmentTable>> {
        lock(&self.state).table.clone()
    }
    /// New readers start quiescent with the published epoch as reported epoch.
    pub fn register_reader(&self) -> ReaderId {
        let state = lock(&self.state);
        let mut tracker = lock(&self.tracker);
        let mut id = ReaderId(tracker.next_id);
        while tracker.readers.contains_key(&id) {
            id = ReaderId(id.0.wrapping_add(1));
        }
        tracker.next_id = id.0.wrapping_add(1);
        tracker
            .readers
            .insert(
                id,
                ReaderState {
                    reported: state.epoch,
                    quiescent: true,
                },
            );
        id
    }
    pub fn unregister_reader(&self, id: ReaderId) -> Result<(), TableSwapError> {
        lock(&self.tracker)
            .readers
            .remove(&id)
            .map(|_| ())
            .ok_or(TableSwapError::UnknownReader(id))
    }
    /// Raises the reported epoch to `max(old, epoch)`; smaller values are ignored.
    pub fn report_epoch(
        &self,
        id: ReaderId,
        epoch: Epoch,
    ) -> Result<(), TableSwapError> {
        let state = lock(&self.state);
        let mut tracker = lock(&self.tracker);
        let reader = tracker
            .readers
            .get_mut(&id)
            .ok_or(TableSwapError::UnknownReader(id))?;
        if epoch > state.epoch {
            return Err(TableSwapError::EpochAhead {
                reported: epoch,
                published: state.epoch,
            });
        }
        reader.reported = reader.reported.max(epoch);
        Ok(())
    }
    pub fn set_quiescent(
        &self,
        id: ReaderId,
        quiescent: bool,
    ) -> Result<(), TableSwapError> {
        lock(&self.tracker)
            .readers
            .get_mut(&id)
            .map(|reader| reader.quiescent = quiescent)
            .ok_or(TableSwapError::UnknownReader(id))
    }
    /// Reported epoch of the reader, independent of its quiescent flag.
    pub fn reader_epoch(&self, id: ReaderId) -> Result<Epoch, TableSwapError> {
        lock(&self.tracker)
            .readers
            .get(&id)
            .map(|reader| reader.reported)
            .ok_or(TableSwapError::UnknownReader(id))
    }
    /// Minimum over active readers' reported epochs and the published epoch
    /// (which stands in for quiescent readers). Never above the published epoch.
    pub fn min_reader_epoch(&self) -> Epoch {
        let state = lock(&self.state);
        let tracker = lock(&self.tracker);
        tracker
            .readers
            .values()
            .filter(|reader| !reader.quiescent)
            .map(|reader| reader.reported)
            .fold(state.epoch, Epoch::min)
    }
    /// Pins the current version for `id`: the reader becomes active at that epoch.
    pub fn acquire(&self, id: ReaderId) -> Result<TableGuard<'_>, TableSwapError> {
        let state = lock(&self.state);
        let table = state.table.clone().ok_or(TableSwapError::NoTable)?;
        let root = self.root.load_acquire();
        let epoch = table.epoch();
        let mut tracker = lock(&self.tracker);
        let reader = tracker
            .readers
            .get_mut(&id)
            .ok_or(TableSwapError::UnknownReader(id))?;
        reader.quiescent = false;
        reader.reported = reader.reported.max(epoch);
        Ok(TableGuard {
            swap: self,
            reader: id,
            table,
            root,
        })
    }
    pub fn retired_count(&self) -> usize {
        lock(&self.retired).len()
    }
    /// Frees retired regions with `retire_epoch <= min(min_reader_epoch(), up_to)`.
    /// Returns how many regions were actually freed.
    pub fn reclaim(&self, up_to: Epoch) -> usize {
        let bound = self.min_reader_epoch().min(up_to);
        let mut retired = lock(&self.retired);
        let mut freed = 0;
        let mut kept = Vec::with_capacity(retired.len());
        for entry in retired.drain(..) {
            if entry.retire_epoch > bound {
                kept.push(entry);
                continue;
            }
            match self.allocator.free(&entry.region) {
                Ok(()) => freed += 1,
                Err(e) => {
                    warn!(
                        "retired table region at offset {} could not be freed: {e}",
                        entry.region.offset()
                    )
                }
            }
        }
        *retired = kept;
        freed
    }
}
/// Holds one table version for a reader; reports its epoch on drop.
pub struct TableGuard<'a> {
    swap: &'a TableSwap,
    reader: ReaderId,
    table: Arc<SegmentTable>,
    root: u64,
}
impl TableGuard<'_> {
    pub fn epoch(&self) -> Epoch {
        self.table.epoch()
    }
    /// Root value (arena offset) read at acquire time.
    pub fn root(&self) -> u64 {
        self.root
    }
    pub fn table(&self) -> &SegmentTable {
        &self.table
    }
}
impl Drop for TableGuard<'_> {
    fn drop(&mut self) {
        let _ = self.swap.report_epoch(self.reader, self.table.epoch());
    }
}
#[cfg(test)]
#[path = "test/table_swap.rs"]
mod tests;
