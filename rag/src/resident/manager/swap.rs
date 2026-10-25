//! Mutation discipline of the index manager: copy-on-write segment tables, a single-writer
//! 8-byte root pointer and epoch-gated reclamation. The kernel is never blocked or locked out.
use std::sync::Arc;
#[cfg(loom)]
use loom::sync::atomic::{AtomicU64, Ordering};
#[cfg(not(loom))]
use std::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;
use crate::resident::error::ManagerError;
use crate::resident::manager::arena::IndexArena;
use crate::resident::memory::Region;
/// Root value before the first table is published.
pub const EMPTY_ROOT: u64 = u64::MAX;
enum Cell {
    Owned(AtomicU64),
    #[cfg(all(feature = "cuda", not(loom)))]
    Mapped(*const AtomicU64),
}
/// Naturally aligned 8-byte cell. Only the host stores; the device reads with `ld.acquire.sys`.
pub struct RootPointer {
    cell: Cell,
}
#[cfg(all(feature = "cuda", not(loom)))]
unsafe impl Send for RootPointer {}
#[cfg(all(feature = "cuda", not(loom)))]
unsafe impl Sync for RootPointer {}
impl RootPointer {
    pub fn new(initial: u64) -> Self {
        Self {
            cell: Cell::Owned(AtomicU64::new(initial)),
        }
    }
    /// Wraps a cell in pinned, mapped host memory.
    ///
    /// # Safety
    /// `ptr` must be 8-byte aligned, valid and unmoved for the lifetime of the pointer, and
    /// written only through this wrapper.
    #[cfg(all(feature = "cuda", not(loom)))]
    pub(crate) unsafe fn from_mapped(ptr: *mut u64) -> Self {
        debug_assert_eq!(ptr as usize % 8, 0);
        Self {
            cell: Cell::Mapped(ptr as *const AtomicU64),
        }
    }
    fn atomic(&self) -> &AtomicU64 {
        match &self.cell {
            Cell::Owned(a) => a,
            #[cfg(all(feature = "cuda", not(loom)))]
            Cell::Mapped(p) => unsafe { &**p }
        }
    }
    /// Everything written before this call is visible to a reader that observes the value.
    pub fn publish(&self, value: u64) {
        self.atomic().store(value, Ordering::Release);
    }
    pub fn load(&self) -> u64 {
        self.atomic().load(Ordering::Acquire)
    }
}
/// Immutable once published; a new version is always a copy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SegmentTable {
    epoch: u64,
    segments: Vec<Region>,
}
impl SegmentTable {
    pub fn empty() -> Self {
        Self {
            epoch: 0,
            segments: Vec::new(),
        }
    }
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    pub fn segments(&self) -> &[Region] {
        &self.segments
    }
    pub fn push(&mut self, region: Region) {
        self.segments.push(region);
    }
    /// Swaps `old` for `new` in place of the copy; returns false when `old` is not listed.
    pub fn replace(&mut self, old: Region, new: Region) -> bool {
        match self.segments.iter_mut().find(|r| **r == old) {
            Some(slot) => {
                *slot = new;
                true
            }
            None => false,
        }
    }
    pub fn remove(&mut self, region: Region) -> bool {
        let before = self.segments.len();
        self.segments.retain(|r| *r != region);
        self.segments.len() != before
    }
    /// Encoded size of the device-side table: epoch, count, then one entry per segment.
    pub fn encoded_len(&self) -> u64 {
        16 + 24 * self.segments.len() as u64
    }
}
/// Versions published by the host and the epoch each reader reported having read.
pub struct EpochTracker {
    published: AtomicU64,
    readers: Box<[AtomicU64]>,
}
impl EpochTracker {
    pub fn new(readers: usize) -> Self {
        Self {
            published: AtomicU64::new(0),
            readers: (0..readers).map(|_| AtomicU64::new(0)).collect(),
        }
    }
    pub fn published(&self) -> u64 {
        self.published.load(Ordering::Acquire)
    }
    fn advance(&self, epoch: u64) {
        self.published.store(epoch, Ordering::Release);
    }
    /// A finished batch read `epoch`; later batches of the reader read the same or a newer one.
    pub fn report(&self, reader: usize, epoch: u64) {
        self.readers[reader].fetch_max(epoch, Ordering::AcqRel);
    }
    /// For a reader with no batch in flight: whatever it runs next reads the published root or
    /// a newer one, so it no longer holds anything older.
    pub fn quiesce(&self, reader: usize) {
        self.report(reader, self.published());
    }
    pub fn reader_min(&self) -> u64 {
        self.readers
            .iter()
            .map(|r| r.load(Ordering::Acquire))
            .min()
            .unwrap_or_else(|| self.published())
    }
}
/// Writes a table into device-visible memory.
pub trait TableSink: Send + Sync {
    /// Returns only once the content is complete and visible to the device, i.e. after the
    /// copy-stream event has been awaited or the mapped stores are released.
    fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError>;
}
pub trait TableSwap {
    /// Copy of the current table with the next epoch.
    fn build_next(&self) -> SegmentTable;
    /// Makes the content visible, then swaps the root with release semantics. Returns the epoch.
    fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError>;
    /// Frees versions retired up to `upto` once every reader has reached them.
    fn reclaim(&self, upto: u64) -> usize;
}
/// Root and epoch seen by one launch or one persistent batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub root: u64,
    pub epoch: u64,
}
/// Pins the version read at batch start; reports its epoch when dropped.
pub struct BatchGuard<'a> {
    epochs: &'a EpochTracker,
    reader: usize,
    snapshot: Snapshot,
}
impl BatchGuard<'_> {
    pub fn snapshot(&self) -> Snapshot {
        self.snapshot
    }
}
impl Drop for BatchGuard<'_> {
    fn drop(&mut self) {
        self.epochs.report(self.reader, self.snapshot.epoch);
        tracing::trace!(
            reader = self.reader, epoch = self.snapshot.epoch, "reader epoch"
        );
    }
}
struct Current {
    table: Arc<SegmentTable>,
    region: Option<Region>,
}
pub struct SwapManager<'a, 'b> {
    arena: &'a IndexArena<'b>,
    sink: Box<dyn TableSink + 'a>,
    root: RootPointer,
    epochs: EpochTracker,
    current: Mutex<Current>,
    publish_gate: Mutex<()>,
}
impl<'a, 'b> SwapManager<'a, 'b> {
    pub fn new(
        arena: &'a IndexArena<'b>,
        sink: Box<dyn TableSink + 'a>,
        readers: usize,
    ) -> Self {
        Self::with_root(arena, sink, RootPointer::new(EMPTY_ROOT), readers)
    }
    pub fn with_root(
        arena: &'a IndexArena<'b>,
        sink: Box<dyn TableSink + 'a>,
        root: RootPointer,
        readers: usize,
    ) -> Self {
        root.publish(EMPTY_ROOT);
        Self {
            arena,
            sink,
            root,
            epochs: EpochTracker::new(readers),
            current: Mutex::new(Current {
                table: Arc::new(SegmentTable::empty()),
                region: None,
            }),
            publish_gate: Mutex::new(()),
        }
    }
    pub fn root(&self) -> &RootPointer {
        &self.root
    }
    pub fn epochs(&self) -> &EpochTracker {
        &self.epochs
    }
    pub fn current(&self) -> Arc<SegmentTable> {
        self.current.lock().table.clone()
    }
    /// Normal launches: the root is passed as a launch argument, so the version stays constant
    /// for the whole launch and a swap takes effect with the next one.
    pub fn snapshot(&self) -> Snapshot {
        let cur = self.current.lock();
        Snapshot {
            root: self.root.load(),
            epoch: cur.table.epoch(),
        }
    }
    /// Persistent kernel: one acquire load per batch, never per work item.
    pub fn begin_batch(&self, reader: usize) -> BatchGuard<'_> {
        BatchGuard {
            epochs: &self.epochs,
            reader,
            snapshot: self.snapshot(),
        }
    }
}
impl TableSwap for SwapManager<'_, '_> {
    fn build_next(&self) -> SegmentTable {
        let cur = self.current.lock();
        let mut next = (*cur.table).clone();
        next.epoch = cur.table.epoch() + 1;
        next
    }
    fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError> {
        let _span = crate::resident::telemetry::manager_cycle_span(table.epoch())
            .entered();
        let _gate = self.publish_gate.lock();
        let published = self.current.lock().table.epoch();
        if table.epoch() != published + 1 {
            return Err(ManagerError::StaleMapping {
                base: table.epoch() - 1,
                published,
            });
        }
        let region = self.sink.write(&table)?;
        let epoch = table.epoch();
        let segments = table.segments().len();
        let previous = {
            let mut cur = self.current.lock();
            self.root.publish(region.offset);
            self.epochs.advance(epoch);
            cur.table = Arc::new(table);
            cur.region.replace(region)
        };
        if let Some(old) = previous {
            self.arena.retire(old, epoch)?;
        }
        tracing::debug!(epoch, segments, offset = region.offset, "table published");
        Ok(epoch)
    }
    fn reclaim(&self, upto: u64) -> usize {
        let reached = self.epochs.reader_min().min(upto);
        let freed = self.arena.reclaim(reached);
        tracing::debug!(freed, reached, "reclaim");
        freed
    }
}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use crate::resident::memory::{ArenaAllocator, ArenaKind, DeviceKind};
    struct HostSink<'a, 'b>(&'a IndexArena<'b>);
    impl TableSink for HostSink<'_, '_> {
        fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {
            self.0.alloc_segment(table.encoded_len().max(256), 256)
        }
    }
    fn alloc() -> ArenaAllocator {
        ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, 1 << 16)
    }
    fn seg(offset: u64) -> Region {
        Region {
            arena: crate::resident::memory::ArenaId(0),
            offset,
            len: 64,
            gen: 0,
        }
    }
    #[test]
    fn publish_swaps_root_and_advances_epoch() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 1);
        assert_eq!(mgr.root().load(), EMPTY_ROOT);
        let mut t = mgr.build_next();
        t.push(seg(0));
        assert_eq!(mgr.publish(t).unwrap(), 1);
        assert_ne!(mgr.root().load(), EMPTY_ROOT);
        assert_eq!(mgr.current().segments().len(), 1);
        assert_eq!(mgr.epochs().published(), 1);
    }
    #[test]
    fn concurrent_builds_on_the_same_base_are_rejected() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 1);
        let a = mgr.build_next();
        let b = mgr.build_next();
        mgr.publish(a).unwrap();
        assert!(matches!(mgr.publish(b), Err(ManagerError::StaleMapping { .. })));
    }
    #[test]
    fn old_table_is_freed_only_after_all_readers_reach_the_new_epoch() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 2);
        mgr.publish(mgr.build_next()).unwrap();
        let first_root = mgr.root().load();
        let slow = mgr.begin_batch(1);
        assert_eq!(slow.snapshot().root, first_root);
        mgr.epochs().quiesce(0);
        mgr.publish(mgr.build_next()).unwrap();
        mgr.epochs().quiesce(0);
        assert_eq!(mgr.reclaim(u64::MAX), 0);
        drop(slow);
        assert_eq!(mgr.reclaim(u64::MAX), 0);
        mgr.epochs().quiesce(1);
        assert_eq!(mgr.reclaim(u64::MAX), 1);
    }
    #[test]
    fn reclaim_respects_the_requested_bound() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 1);
        for _ in 0..3 {
            mgr.publish(mgr.build_next()).unwrap();
        }
        mgr.epochs().quiesce(0);
        assert_eq!(mgr.reclaim(2), 1);
        assert_eq!(mgr.reclaim(3), 1);
    }
    #[test]
    fn launch_snapshot_is_constant_until_the_next_launch() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 1);
        mgr.publish(mgr.build_next()).unwrap();
        let launch = mgr.snapshot();
        mgr.publish(mgr.build_next()).unwrap();
        assert_eq!(launch.epoch, 1);
        let next = mgr.snapshot();
        assert_eq!(next.epoch, 2);
        assert_ne!(next.root, launch.root);
    }
    #[test]
    fn copy_is_independent_of_the_published_table() {
        let alloc = alloc();
        let arena = IndexArena::new(&alloc, 0);
        let mgr = SwapManager::new(&arena, Box::new(HostSink(&arena)), 1);
        let mut t = mgr.build_next();
        t.push(seg(0));
        mgr.publish(t).unwrap();
        let mut next = mgr.build_next();
        assert!(next.replace(seg(0), seg(128)));
        assert_eq!(mgr.current().segments(), & [seg(0)]);
        assert_eq!(next.segments(), & [seg(128)]);
    }
}
#[cfg(loom)]
mod loom_tests {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;
    #[test]
    fn content_is_visible_once_the_root_is_observed() {
        loom::model(|| {
            let content = Arc::new(AtomicU64::new(0));
            let root = Arc::new(RootPointer::new(EMPTY_ROOT));
            let writer = {
                let (content, root) = (content.clone(), root.clone());
                thread::spawn(move || {
                    content.store(42, Ordering::Relaxed);
                    root.publish(7);
                })
            };
            if root.load() == 7 {
                assert_eq!(content.load(Ordering::Relaxed), 42);
            }
            writer.join().unwrap();
        });
    }
    #[test]
    fn retired_version_is_not_reclaimed_while_a_reader_holds_it() {
        loom::model(|| {
            let epochs = Arc::new(EpochTracker::new(1));
            let root = Arc::new(RootPointer::new(1));
            let live = Arc::new(AtomicU64::new(1));
            let host = {
                let (epochs, root, live) = (epochs.clone(), root.clone(), live.clone());
                thread::spawn(move || {
                    root.publish(2);
                    epochs.advance(2);
                    if epochs.reader_min() >= 2 {
                        live.store(0, Ordering::Release);
                    }
                })
            };
            let held = root.load();
            if held == 1 {
                assert_eq!(live.load(Ordering::Acquire), 1);
            }
            epochs.report(0, held);
            host.join().unwrap();
        });
    }
}
#[cfg(all(test, feature = "cuda", not(loom)))]
mod device_tests {
    use super::*;
    use crate::resident::subengine::DeviceHandle;
    use cudarc::driver::{LaunchConfig, PushKernelArg};
    use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
    use std::time::Duration;
    const PROBE: &str = r#"
extern "C" __global__ void probe(const unsigned long long* root,
                                const unsigned long long* data,
                                unsigned long long* out,
                                unsigned long long empty) {
    unsigned long long v = empty;
    for (unsigned long long i = 0; i < (1ull << 31) && v == empty; ++i) {
        asm volatile("ld.acquire.sys.u64 %0, [%1];" : "=l"(v) : "l"(root) : "memory");
    }
    unsigned long long d;
    asm volatile("ld.relaxed.sys.u64 %0, [%1];" : "=l"(d) : "l"(data) : "memory");
    out[0] = v;
    out[1] = d;
}
"#;
    #[test]
    #[ignore = "requires a CUDA device"]
    fn device_sees_content_written_before_the_host_release_store() {
        let dev = DeviceHandle::cuda(0).unwrap();
        let ctx = dev.cuda_context().unwrap().clone();
        let stream = ctx.new_stream().unwrap();
        let ptx = compile_ptx_with_opts(
                PROBE,
                CompileOptions {
                    arch: Some("compute_75"),
                    ..Default::default()
                },
            )
            .unwrap();
        let probe = ctx.load_module(ptx).unwrap().load_function("probe").unwrap();
        let mut host = unsafe { ctx.alloc_pinned::<u64>(2) }.unwrap();
        let base = host.as_mut_ptr().unwrap();
        let root = unsafe { RootPointer::from_mapped(base) };
        let data = unsafe { AtomicU64::from_ptr(base.add(1)) };
        let (root_dev, data_dev) = (base as u64, base as u64 + 8);
        let mut out = stream.alloc_zeros::<u64>(2).unwrap();
        for round in 1..=200u64 {
            root.publish(EMPTY_ROOT);
            data.store(0, Ordering::Relaxed);
            let mut b = stream.launch_builder(&probe);
            b.arg(&root_dev).arg(&data_dev).arg(&mut out).arg(&EMPTY_ROOT);
            unsafe { b.launch(LaunchConfig::for_num_elems(1)) }.unwrap();
            std::thread::sleep(Duration::from_micros(round * 37 % 3000));
            data.store(round * 1000, Ordering::Relaxed);
            root.publish(round);
            stream.synchronize().unwrap();
            assert_eq!(
                stream.clone_dtoh(& out).unwrap(), [round, round * 1000], "round {round}"
            );
        }
    }
    const PROBE_DEVICE: &str = r#"
extern "C" __global__ void probe_device(const unsigned long long* root,
                                       const unsigned long long* table,
                                       unsigned long long* out,
                                       unsigned long long empty) {
    unsigned long long v = empty;
    for (unsigned long long i = 0; i < (1ull << 31) && v == empty; ++i) {
        asm volatile("ld.acquire.sys.u64 %0, [%1];" : "=l"(v) : "l"(root) : "memory");
    }
    out[0] = v;
    out[1] = *(const volatile unsigned long long*)table;
}
"#;
    #[test]
    #[ignore = "requires a CUDA device"]
    fn copy_stream_event_orders_device_table_before_the_root() {
        use cudarc::driver::sys::CUdevice_attribute::CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED;
        let dev = DeviceHandle::cuda(0).unwrap();
        let ctx = dev.cuda_context().unwrap().clone();
        let native = ctx
            .attribute(CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED)
            .unwrap();
        eprintln!("host native atomics supported: {native}");
        struct TrackingOff(Arc<cudarc::driver::CudaContext>);
        impl Drop for TrackingOff {
            fn drop(&mut self) {
                unsafe { self.0.enable_event_tracking() };
            }
        }
        unsafe { ctx.disable_event_tracking() };
        let _tracking = TrackingOff(ctx.clone());
        let kernel_stream = ctx.new_stream().unwrap();
        let copy_stream = ctx.new_stream().unwrap();
        let ptx = compile_ptx_with_opts(
                PROBE_DEVICE,
                CompileOptions {
                    arch: Some("compute_75"),
                    ..Default::default()
                },
            )
            .unwrap();
        let probe = ctx.load_module(ptx).unwrap().load_function("probe_device").unwrap();
        let mut root_cell = unsafe { ctx.alloc_pinned::<u64>(1) }.unwrap();
        let mut staging = unsafe { ctx.alloc_pinned::<u64>(1) }.unwrap();
        let root_ptr = root_cell.as_mut_ptr().unwrap();
        let root = unsafe { RootPointer::from_mapped(root_ptr) };
        let root_dev = root_ptr as u64;
        let mut table = copy_stream.alloc_zeros::<u64>(1).unwrap();
        let mut out = kernel_stream.alloc_zeros::<u64>(2).unwrap();
        for round in 1..=200u64 {
            root.publish(EMPTY_ROOT);
            copy_stream.memcpy_htod(&[0u64], &mut table).unwrap();
            copy_stream.synchronize().unwrap();
            let mut b = kernel_stream.launch_builder(&probe);
            b.arg(&root_dev).arg(&table).arg(&mut out).arg(&EMPTY_ROOT);
            unsafe { b.launch(LaunchConfig::for_num_elems(1)) }.unwrap();
            staging.as_mut_slice().unwrap()[0] = round * 7;
            copy_stream.memcpy_htod(&staging, &mut table).unwrap();
            let done = copy_stream.record_event(None).unwrap();
            done.synchronize().unwrap();
            root.publish(round);
            kernel_stream.synchronize().unwrap();
            assert_eq!(
                kernel_stream.clone_dtoh(& out).unwrap(), [round, round * 7],
                "round {round}"
            );
        }
    }
}
