pub mod staging_ring;
use std::fmt;
use std::sync::Arc;
use log::{debug, warn};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::gpu_backend::{
    GpuBackend, HostSource, PinnedBuffer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::memory_backend::{
    BackendError, MemoryBackend,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::RegionHandle;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    AccessMethod, ByteRange as IndexByteRange, EntryId, Index, IndexError, SharedBytes,
    Stamp,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, EventError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::page_advice::{
    self, read_blocking, Advice, AdviceTarget, BlockingError, PageRange,
};
use self::staging_ring::{
    SlotId, StagingError, StagingRing, TransferDirection, TransferError,
};
const MAX_ALIGN: u64 = 256;
const DEFAULT_ALIGN: u64 = 16;
/// Bytes of an index range in host memory.
#[derive(Clone, Copy, Debug)]
pub enum ArenaBytes<'a> {
    Ordinary(&'a [u8]),
    Pinned(&'a PinnedBuffer),
}
impl ArenaBytes<'_> {
    pub fn len(&self) -> usize {
        match self {
            ArenaBytes::Ordinary(bytes) => bytes.len(),
            ArenaBytes::Pinned(buffer) => buffer.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn host_source(&self) -> HostSource<'_> {
        match self {
            ArenaBytes::Ordinary(bytes) => HostSource::Pageable(bytes),
            ArenaBytes::Pinned(buffer) => HostSource::Pinned(buffer),
        }
    }
}
/// Source of segment ranges (the index arena).
pub trait IndexArenaSource: Send + Sync {
    fn index(&self) -> &dyn Index;
    /// Exactly the bytes of this range. May touch pages, but does not block for I/O before the
    /// bytes are read.
    fn range_bytes(&self, range: &IndexByteRange) -> Result<ArenaBytes<'_>, IndexError>;
    /// Target for page-cache hints; `None` means no hints.
    fn advice_target(&self) -> Option<Arc<dyn AdviceTarget>>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding {
    pub entry: EntryId,
    pub slot: usize,
    pub stamp: Stamp,
}
/// A loaded segment: region, slot assignment and bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedSegment {
    /// Arena `IndexSegments` (direct copy, gather) or `MappedHost` (mapping).
    pub region: RegionHandle,
    /// Ascending; slot `i` holds `entries[i]`.
    pub entries: Vec<EntryId>,
    /// `entries.len() + 1` absolute arena offsets; slot `i` is `[bounds[i], bounds[i + 1])`, the
    /// last value is the end marker and `bounds[0] == region.offset()`.
    pub bounds: Vec<u64>,
    /// Only entries with a known stamp, ascending by slot.
    pub bindings: Vec<Binding>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SinkError(pub String);
impl fmt::Display for SinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sink rejected segment: {}", self.0)
    }
}
impl std::error::Error for SinkError {}
/// Replaceable receiver of loaded segments. Called from the blocking pool.
pub trait SegmentSink: Send + Sync {
    /// Takes ownership of segment and region. On `Err` the region stays with the module, which
    /// frees it.
    fn accept(&self, segment: LoadedSegment) -> Result<(), SinkError>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransferConfig {
    /// Alignment of entry starts; power of two in `1..=256`.
    pub align: u64,
}
impl Default for TransferConfig {
    fn default() -> Self {
        TransferConfig {
            align: DEFAULT_ALIGN,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferReport {
    pub bytes: u64,
    pub segments: usize,
    /// Finished entries, removed ones included.
    pub entries_done: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceTransferError {
    /// Cancelled or panicked blocking task, or a failed event wait. The direction is always H2D.
    /// After a failed event wait the slot and the region stay quarantined (in flight and pending)
    /// until the event completes; later calls may fail at slot acquisition with the same cause.
    Transfer(TransferError),
    GatherNoProgress,
    ArenaExhausted,
    InvalidConfig,
    Backend(BackendError),
    Staging(StagingError),
    Sink(SinkError),
}
impl fmt::Display for DeviceTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceTransferError::Transfer(e) => write!(f, "{e}"),
            DeviceTransferError::GatherNoProgress => {
                f.write_str("Gather ohne Fortschritt")
            }
            DeviceTransferError::ArenaExhausted => f.write_str("Arena erschöpft"),
            DeviceTransferError::InvalidConfig => f.write_str("invalid transfer config"),
            DeviceTransferError::Backend(e) => write!(f, "{e}"),
            DeviceTransferError::Staging(e) => write!(f, "{e}"),
            DeviceTransferError::Sink(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for DeviceTransferError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DeviceTransferError::Transfer(e) => Some(e),
            DeviceTransferError::Backend(e) => Some(e),
            DeviceTransferError::Staging(e) => Some(e),
            DeviceTransferError::Sink(e) => Some(e),
            _ => None,
        }
    }
}
fn check_config(config: &TransferConfig) -> Result<(), DeviceTransferError> {
    if (1..=MAX_ALIGN).contains(&config.align) && config.align.is_power_of_two() {
        Ok(())
    } else {
        Err(DeviceTransferError::InvalidConfig)
    }
}
fn align_up(value: u64, align: u64) -> Option<u64> {
    let mask = align - 1;
    value.checked_add(mask).map(|v| v & !mask)
}
/// Start offset for an entry of `len` bytes behind `cur`, if it fits into `limit` bytes.
fn place(cur: u64, len: u64, align: u64, limit: u64) -> Option<u64> {
    let off = align_up(cur, align)?;
    (off.checked_add(len)? <= limit).then_some(off)
}
fn normalize(entries: &[EntryId]) -> Vec<EntryId> {
    let mut sorted = entries.to_vec();
    sorted.sort();
    sorted.dedup();
    sorted
}
fn capture_stamps(index: &dyn Index, entries: &[EntryId]) -> Vec<Option<Stamp>> {
    entries.iter().map(|&id| index.stamp(id)).collect()
}
fn fetch_ranges(
    index: &dyn Index,
    first: EntryId,
    last: EntryId,
) -> Result<Vec<IndexByteRange>, IndexError> {
    if !index.access_methods().contains(&AccessMethod::Range) {
        return Err(IndexError::Unsupported(AccessMethod::Range));
    }
    index.byte_ranges(first, last)
}
fn h2d(cause: EventError) -> DeviceTransferError {
    DeviceTransferError::Transfer(TransferError {
        direction: TransferDirection::H2D,
        cause,
    })
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DirectPlan {
    range: IndexByteRange,
    stride: u64,
}
/// Direct-copy eligibility of the ranges returned for the sorted, deduplicated `entries`.
fn plan_direct(
    ranges: &[IndexByteRange],
    entries: &[EntryId],
    align: u64,
) -> Option<DirectPlan> {
    let [range] = ranges else {
        return None;
    };
    let count = entries.len() as u64;
    if range.first != *entries.first()? || range.count != count {
        return None;
    }
    if range.len == 0 || range.count == 0 || range.len % range.count != 0 {
        return None;
    }
    let stride = range.len / range.count;
    if range.offset % align != 0 || stride % align != 0 {
        return None;
    }
    Some(DirectPlan {
        range: *range,
        stride,
    })
}
/// `offsets` are relative to the region start; `stamps` run parallel to `entries`.
fn build_segment(
    region: RegionHandle,
    entries: Vec<EntryId>,
    stamps: &[Option<Stamp>],
    offsets: &[u64],
    end: u64,
) -> LoadedSegment {
    let base = region.offset();
    let mut bounds: Vec<u64> = offsets.iter().map(|off| base + off).collect();
    bounds.push(base + end);
    let bindings = stamps
        .iter()
        .enumerate()
        .filter_map(|(slot, stamp)| {
            stamp
                .map(|stamp| Binding {
                    entry: entries[slot],
                    slot,
                    stamp,
                })
        })
        .collect();
    LoadedSegment {
        region,
        entries,
        bounds,
        bindings,
    }
}
#[derive(Debug, Default, PartialEq, Eq)]
struct PackedRun {
    entries: Vec<EntryId>,
    stamps: Vec<Option<Stamp>>,
    offsets: Vec<u64>,
    end: u64,
    removed: usize,
}
impl PackedRun {
    fn done(&self) -> usize {
        self.entries.len() + self.removed
    }
}
#[derive(Debug)]
enum PackError {
    Exhausted,
    Staging(StagingError),
}
/// Packs the leading entries of `items` into one slot of `slot_size` bytes. Entries without a
/// stamp are removed and take no space. An entry that does not fit ends the run and stays in
/// `pending` with its payload; if it is the first present entry, the run fails as exhausted.
/// `write(from, to, bytes)` zero-fills `[from, to)` and then stores `bytes` at `to`.
fn pack_run(
    items: &[(EntryId, Option<Stamp>)],
    pending: &mut Option<SharedBytes>,
    slot_size: u64,
    align: u64,
    mut read: impl FnMut(EntryId) -> SharedBytes,
    mut write: impl FnMut(u64, u64, &[u8]) -> Result<(), StagingError>,
) -> Result<PackedRun, PackError> {
    let mut run = PackedRun::default();
    let mut cur = 0u64;
    for &(entry, stamp) in items {
        if stamp.is_none() {
            run.removed += 1;
            continue;
        }
        let data = pending.take().unwrap_or_else(|| read(entry));
        let len = data.len() as u64;
        let Some(off) = place(cur, len, align, slot_size) else {
            if run.entries.is_empty() {
                return Err(PackError::Exhausted);
            }
            *pending = Some(data);
            break;
        };
        write(cur, off, &*data).map_err(PackError::Staging)?;
        run.entries.push(entry);
        run.stamps.push(stamp);
        run.offsets.push(off);
        cur = off + len;
    }
    run.end = cur;
    Ok(run)
}
/// Zero-fills `[from, to)` of the slot payload and stores `bytes` at `to`.
fn write_slot(
    ring: &StagingRing,
    slot: SlotId,
    from: u64,
    to: u64,
    bytes: &[u8],
) -> Result<(), StagingError> {
    ring.with_payload(
        slot,
        |payload| {
            let (from, to) = (from as usize, to as usize);
            payload[from..to].fill(0);
            payload[to..to + bytes.len()].copy_from_slice(bytes);
        },
    )
}
/// Releases an occupied slot on drop unless it was handed over to a transfer.
struct SlotLease<'a> {
    ring: &'a StagingRing,
    slot: SlotId,
    armed: bool,
}
impl SlotLease<'_> {
    fn hand_over(&mut self) {
        self.armed = false;
    }
}
impl Drop for SlotLease<'_> {
    fn drop(&mut self) {
        if self.armed {
            if let Err(e) = self.ring.release(self.slot) {
                warn!("staging slot {:?} could not be released: {e}", self.slot);
            }
        }
    }
}
/// Moves index entries from the host index arena into VRAM segments and reports them to a sink.
pub struct DeviceTransfer {
    source: Arc<dyn IndexArenaSource>,
    backend: Arc<GpuBackend>,
    ring: Arc<StagingRing>,
    sink: Arc<dyn SegmentSink>,
    config: TransferConfig,
}
static_assertions::assert_impl_all!(DeviceTransfer : Send, Sync);
impl DeviceTransfer {
    pub fn new(
        source: Arc<dyn IndexArenaSource>,
        backend: Arc<GpuBackend>,
        ring: Arc<StagingRing>,
        sink: Arc<dyn SegmentSink>,
        config: TransferConfig,
    ) -> Result<DeviceTransfer, DeviceTransferError> {
        check_config(&config)?;
        Ok(DeviceTransfer {
            source,
            backend,
            ring,
            sink,
            config,
        })
    }
    /// Whole segment: direct copy if possible, otherwise the gather path. Runs in the blocking pool.
    pub async fn load_segment(
        self: &Arc<Self>,
        entries: Vec<EntryId>,
    ) -> Result<TransferReport, DeviceTransferError> {
        let this = Arc::clone(self);
        match read_blocking(move || this.load_segment_blocking(&entries)).await {
            Ok(result) => result,
            Err(e) => Err(self.blocking_failure(e)),
        }
    }
    /// Very small set via direct mapping, falling back to the gather path. Runs in the blocking pool.
    pub async fn load_mapped(
        self: &Arc<Self>,
        entries: Vec<EntryId>,
    ) -> Result<TransferReport, DeviceTransferError> {
        let this = Arc::clone(self);
        match read_blocking(move || this.load_mapped_blocking(&entries)).await {
            Ok(result) => result,
            Err(e) => Err(self.blocking_failure(e)),
        }
    }
    /// Runs on the current thread; must not be called from async threads.
    pub fn load_segment_blocking(
        &self,
        entries: &[EntryId],
    ) -> Result<TransferReport, DeviceTransferError> {
        let entries = normalize(entries);
        if entries.is_empty() {
            return Ok(TransferReport::default());
        }
        match self.try_direct(&entries)? {
            Some(report) => Ok(report),
            None => self.gather(&entries),
        }
    }
    /// Runs on the current thread; must not be called from async threads.
    pub fn load_mapped_blocking(
        &self,
        entries: &[EntryId],
    ) -> Result<TransferReport, DeviceTransferError> {
        let entries = normalize(entries);
        if entries.is_empty() {
            return Ok(TransferReport::default());
        }
        match self.try_mapped(&entries)? {
            Some(report) => Ok(report),
            None => self.gather(&entries),
        }
    }
    fn blocking_failure(&self, error: BlockingError) -> DeviceTransferError {
        let cause = match error {
            BlockingError::Cancelled => "cancelled",
            BlockingError::Panicked => "panicked",
        };
        h2d(EventError {
            device: self.backend.device(),
            cause: cause.to_string(),
        })
    }
    fn advise(&self, advice: Advice, offset: u64, len: u64) {
        let Some(target) = self.source.advice_target() else {
            return;
        };
        let range = PageRange {
            start: offset,
            end: offset.saturating_add(len),
        };
        if let Err(e) = page_advice::advise(target.as_ref(), advice, range) {
            warn!("page advice {advice:?} failed: {e}");
        }
    }
    fn free_region(&self, region: &RegionHandle) {
        if let Err(e) = self.backend.free(region) {
            warn!("region could not be freed: {e}");
        }
    }
    fn free_region_deferred(&self, region: &RegionHandle, event: Event) {
        if let Err(e) = self.backend.free_deferred(region, event) {
            warn!("region could not be freed deferred: {e}");
        }
    }
    fn deliver(&self, segment: LoadedSegment) -> Result<(), DeviceTransferError> {
        let region = segment.region;
        self.sink
            .accept(segment)
            .map_err(|e| {
                self.free_region(&region);
                DeviceTransferError::Sink(e)
            })
    }
    fn lease_slot(&self) -> Result<SlotLease<'_>, DeviceTransferError> {
        match self.ring.acquire_wait() {
            Ok(slot) => {
                Ok(SlotLease {
                    ring: &self.ring,
                    slot,
                    armed: true,
                })
            }
            Err(StagingError::Transfer(t)) => Err(DeviceTransferError::Transfer(t)),
            Err(e) => Err(DeviceTransferError::Staging(e)),
        }
    }
    /// `Ok(None)` when the entries are not directly copyable.
    fn try_direct(
        &self,
        entries: &[EntryId],
    ) -> Result<Option<TransferReport>, DeviceTransferError> {
        let index = self.source.index();
        let n = entries.len();
        let ranges = match fetch_ranges(index, entries[0], entries[n - 1]) {
            Ok(ranges) => ranges,
            Err(e) => {
                debug!("direct copy not possible, using gather path: {e}");
                return Ok(None);
            }
        };
        let Some(plan) = plan_direct(&ranges, entries, self.config.align) else {
            debug!(
                "direct copy not possible, using gather path: ranges not directly copyable"
            );
            return Ok(None);
        };
        let range = plan.range;
        let bytes = match self.source.range_bytes(&range) {
            Ok(bytes) if bytes.len() as u64 == range.len => bytes,
            Ok(bytes) => {
                debug!(
                    "direct copy not possible, using gather path: range yields {} of {} bytes",
                    bytes.len(), range.len
                );
                return Ok(None);
            }
            Err(e) => {
                debug!("direct copy not possible, using gather path: {e}");
                return Ok(None);
            }
        };
        self.advise(Advice::Sequential, range.offset, range.len);
        let stamps = capture_stamps(index, entries);
        let region = self
            .backend
            .allocate(ArenaKind::IndexSegments, range.len, self.config.align)
            .map_err(DeviceTransferError::Backend)?;
        let event = match self.backend.upload(&region, bytes.host_source()) {
            Ok(event) => event,
            Err(e) => {
                self.free_region(&region);
                return Err(DeviceTransferError::Backend(e));
            }
        };
        if let Err(cause) = event.wait() {
            self.free_region_deferred(&region, event);
            return Err(h2d(cause));
        }
        let offsets: Vec<u64> = (0..n as u64).map(|i| i * plan.stride).collect();
        let segment = build_segment(
            region,
            entries.to_vec(),
            &stamps,
            &offsets,
            range.len,
        );
        self.deliver(segment)?;
        Ok(
            Some(TransferReport {
                bytes: range.len,
                segments: 1,
                entries_done: n,
            }),
        )
    }
    fn gather(
        &self,
        entries: &[EntryId],
    ) -> Result<TransferReport, DeviceTransferError> {
        let index = self.source.index();
        let stamps = capture_stamps(index, entries);
        if let Ok(ranges) = fetch_ranges(index, entries[0], entries[entries.len() - 1]) {
            for range in &ranges {
                self.advise(Advice::Random, range.offset, range.len);
            }
        }
        let items: Vec<(EntryId, Option<Stamp>)> = entries
            .iter()
            .copied()
            .zip(stamps.iter().copied())
            .collect();
        let align = self.config.align;
        let slot_size = self.ring.slot_size() as u64;
        let mut report = TransferReport::default();
        let mut pending: Option<SharedBytes> = None;
        let mut rest = &items[..];
        while !rest.is_empty() {
            let mut lease = self.lease_slot()?;
            let slot = lease.slot;
            let run = pack_run(
                    rest,
                    &mut pending,
                    slot_size,
                    align,
                    |id| index.read(id).vector,
                    |from, to, bytes| write_slot(&self.ring, slot, from, to, bytes),
                )
                .map_err(|e| match e {
                    PackError::Exhausted => DeviceTransferError::ArenaExhausted,
                    PackError::Staging(e) => DeviceTransferError::Staging(e),
                })?;
            let done = run.done();
            if done == 0 {
                return Err(DeviceTransferError::GatherNoProgress);
            }
            rest = &rest[done..];
            report.entries_done += done;
            if run.end == 0 {
                drop(lease);
                continue;
            }
            let ptr = self.ring.payload_ptr(slot).map_err(DeviceTransferError::Staging)?;
            let region = self
                .backend
                .allocate(ArenaKind::IndexSegments, run.end, align)
                .map_err(DeviceTransferError::Backend)?;
            let data = unsafe {
                std::slice::from_raw_parts(ptr as *const u8, run.end as usize)
            };
            let event = match self.backend.upload(&region, HostSource::Pageable(data)) {
                Ok(event) => event,
                Err(e) => {
                    self.free_region(&region);
                    return Err(DeviceTransferError::Backend(e));
                }
            };
            lease.hand_over();
            if let Err(e) = self.ring.begin_transfer(slot, event.clone()) {
                self.free_region_deferred(&region, event);
                return Err(DeviceTransferError::Staging(e));
            }
            if let Err(cause) = event.wait() {
                self.free_region_deferred(&region, event);
                return Err(h2d(cause));
            }
            let segment = build_segment(
                region,
                run.entries,
                &run.stamps,
                &run.offsets,
                run.end,
            );
            self.deliver(segment)?;
            report.bytes += run.end;
            report.segments += 1;
        }
        Ok(report)
    }
    /// `Ok(None)` when the gather path has to take over.
    fn try_mapped(
        &self,
        entries: &[EntryId],
    ) -> Result<Option<TransferReport>, DeviceTransferError> {
        if self.backend.mapped_device_addr().is_none() {
            warn!("mapped host memory unavailable, using gather path");
            return Ok(None);
        }
        let index = self.source.index();
        let align = self.config.align;
        let stamps = capture_stamps(index, entries);
        let mut present = Vec::new();
        let mut present_stamps = Vec::new();
        let mut offsets = Vec::new();
        let mut vectors = Vec::new();
        let mut cur = 0u64;
        for (&entry, &stamp) in entries.iter().zip(&stamps) {
            if stamp.is_none() {
                continue;
            }
            let vector = index.read(entry).vector;
            let Some(off) = align_up(cur, align) else {
                warn!("mapped layout overflows, using gather path");
                return Ok(None);
            };
            cur = off + vector.len() as u64;
            present.push(entry);
            present_stamps.push(stamp);
            offsets.push(off);
            vectors.push(vector);
        }
        let total = cur;
        if total == 0 {
            warn!("nothing to map, using gather path");
            return Ok(None);
        }
        let region = match self.backend.allocate(ArenaKind::MappedHost, total, align) {
            Ok(region) => region,
            Err(e) => {
                warn!("mapped allocation failed, using gather path: {e}");
                return Ok(None);
            }
        };
        for (&off, vector) in offsets.iter().zip(&vectors) {
            let written = usize::try_from(off)
                .map_err(|_| DeviceTransferError::ArenaExhausted)
                .and_then(|off| {
                    self.backend
                        .mapped_write(&region, off, vector)
                        .map_err(DeviceTransferError::Backend)
                });
            if let Err(e) = written {
                self.free_region(&region);
                return Err(e);
            }
        }
        let segment = build_segment(region, present, &present_stamps, &offsets, total);
        self.deliver(segment)?;
        Ok(
            Some(TransferReport {
                bytes: total,
                segments: 1,
                entries_done: entries.len(),
            }),
        )
    }
}
#[cfg(test)]
#[path = "test/device_transfer.rs"]
mod tests;
