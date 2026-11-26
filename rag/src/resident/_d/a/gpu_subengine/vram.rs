use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, PoisonError};

use log::warn;

use super::backend::GpuBackend3;
use super::config::GpuConfig;
use super::deps::*;
use super::error::GpuError;

/// Result of a successful reservation; lives as long as the arenas.
pub(super) struct Reserved2 {
    regions: HashMap<ArenaKind, AccessRegion>,
    bookkeeping: Arc<Bookkeeping>,
    allocators: Arc<AllocatorSet>,
    weights: Arena,
}

impl Reserved2 {
    pub(super) fn allocators(&self) -> &Arc<AllocatorSet> {
        &self.allocators
    }
}

enum Phase3 {
    Idle,
    Reserving,
    Ready(Arc<Reserved2>),
}

/// Resets a reservation that did not finish, so that waiters and later calls retry.
struct ReserveGuard<'a> {
    owner: &'a VramManager,
    armed: bool,
}

impl Drop for ReserveGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            *lock(&self.owner.phase) = Phase3::Idle;
            self.owner.changed.notify_all();
        }
    }
}

/// Weights of one model in the Weights arena. Dropping the last `Arc` frees the extent.
/// Whoever uses the tensors on the device keeps the `Arc` until its events are complete.
pub struct WeightSet {
    weights: LoadedWeights,
    extent: Option<RegionHandle>,
    bytes: u64,
    owner: Option<Arc<Reserved2>>,
}

impl WeightSet {
    pub fn tensors(&self) -> &LoadedWeights {
        &self.weights
    }

    /// Reserved extent length.
    pub fn bytes(&self) -> u64 {
        self.bytes
    }
}

impl Drop for WeightSet {
    fn drop(&mut self) {
        let (Some(extent), Some(owner)) = (self.extent.take(), self.owner.take()) else {
            return;
        };
        let Some(allocator) = owner.allocators.shared(ArenaKind::Weights) else {
            return;
        };
        if let Err(e) = allocator.free(&extent) {
            warn!("weight extent at {} could not be freed: {e}", extent.offset());
        }
    }
}

static_assertions::assert_impl_all!(WeightSet : Send, Sync);

/// Manages the VRAM of one GPU: reserves the six arenas once and places model weights.
pub struct VramManager {
    ordinal: u32,
    backend: Arc<dyn GpuBackend3>,
    capacities: ArenaCapacities,
    headroom: u64,
    generations: u32,
    phase: Mutex<Phase3>,
    changed: Condvar,
}

static_assertions::assert_impl_all!(VramManager : Send, Sync);

fn align_up5(value: u64) -> Option<u64> {
    value.checked_add(WEIGHT_ALIGN - 1).map(|v| v & !(WEIGHT_ALIGN - 1))
}

/// End of the last selected tensor under the layout rule of `WeightFile::copy_to_arena`,
/// relative to an extent start that is aligned to `WEIGHT_ALIGN`.
fn required_size(file: &WeightFile, select: &dyn Fn(&str) -> bool) -> Option<u64> {
    let mut cursor = 0u64;
    for info in file.tensors() {
        if !select(&info.name) {
            continue;
        }
        let start = align_up5(cursor)?;
        cursor = start.checked_add(info.data_end - info.data_begin)?;
    }
    Some(cursor)
}

impl VramManager {
    pub(super) fn new(ordinal: u32, backend: Arc<dyn GpuBackend3>, cfg: &GpuConfig) -> Arc<Self> {
        Arc::new(VramManager {
            ordinal,
            backend,
            capacities: cfg.arenas,
            headroom: cfg.headroom,
            generations: cfg.generations.max(1),
            phase: Mutex::new(Phase3::Idle),
            changed: Condvar::new(),
        })
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// Idempotent. Reserves all six arenas once. A failure is not cached.
    pub fn ensure_arenas(&self) -> Result<(), GpuError> {
        self.reserved().map(|_| ())
    }

    pub fn arenas_ready(&self) -> bool {
        matches!(&*lock(&self.phase), Phase3::Ready(_))
    }

    /// Region of a reserved arena (clone); `None` before `ensure_arenas`.
    pub fn arena_region(&self, kind: ArenaKind) -> Option<AccessRegion> {
        match &*lock(&self.phase) {
            Phase3::Ready(r) => r.regions.get(&kind).cloned(),
            _ => None,
        }
    }

    pub(super) fn reserved(&self) -> Result<Arc<Reserved2>, GpuError> {
        let mut phase = lock(&self.phase);
        loop {
            match &*phase {
                Phase3::Ready(r) => return Ok(Arc::clone(r)),
                Phase3::Reserving => {
                    phase = self
                        .changed
                        .wait(phase)
                        .unwrap_or_else(PoisonError::into_inner);
                }
                Phase3::Idle => break,
            }
        }
        *phase = Phase3::Reserving;
        drop(phase);
        let mut guard = ReserveGuard {
            owner: self,
            armed: true,
        };
        let reserved = Arc::new(self.reserve_all()?);
        *lock(&self.phase) = Phase3::Ready(Arc::clone(&reserved));
        guard.armed = false;
        self.changed.notify_all();
        Ok(reserved)
    }

    fn reserve_all(&self) -> Result<Reserved2, GpuError> {
        let query = self.backend.memory_query(self.ordinal);
        let budget = Budget::from_query(
                &*query,
                self.capacities.generation_reserve(self.generations),
            )
            .map_err(GpuError::Budget)?;
        fit_check(&self.capacities, budget.free, self.headroom).map_err(GpuError::Budget)?;
        let set = Arc::new(ArenaSet::new(self.capacities));
        let mut regions = HashMap::with_capacity(ArenaKind::ALL.len());
        for kind in ArenaKind::ALL {
            let arena = set.ensure(kind, &budget).map_err(GpuError::Budget)?;
            let region = self
                .backend
                .reserve_arena(self.ordinal, arena)
                .map_err(GpuError::Backend)?;
            regions.insert(kind, region);
        }
        let arena_of = |kind: ArenaKind| {
            set.get(kind)
                .ok_or_else(|| GpuError::Backend(format!("arena {kind:?} is missing")))
        };
        let weights = arena_of(ArenaKind::Weights)?;
        let segments = arena_of(ArenaKind::IndexSegments)?;
        let bookkeeping = Arc::new(Bookkeeping::new(Arc::clone(&set)));
        let allocators = AllocatorSet::new(
                Arc::clone(&bookkeeping),
                &[(weights, DeviceKind::Gpu), (segments, DeviceKind::Gpu)],
            )
            .map_err(|e| GpuError::InvalidConfig(e.to_string()))?;
        Ok(Reserved2 {
            regions,
            bookkeeping,
            allocators: Arc::new(allocators),
            weights,
        })
    }

    /// Copies the selected tensors of a mapped safetensors file into the Weights arena.
    pub fn stage_weights(
        &self,
        file: &WeightFile,
        select: &dyn Fn(&str) -> bool,
    ) -> Result<Arc<WeightSet>, GpuError> {
        let reserved = self.reserved()?;
        let overflow = || {
            GpuError::Weights(WeightError::CapacityExceeded {
                required_end: u64::MAX,
                arena_capacity: reserved.weights.capacity,
                region_len: 0,
            })
        };
        let required = required_size(file, select).ok_or_else(overflow)?;
        if required == 0 {
            return Ok(
                Arc::new(WeightSet {
                    weights: LoadedWeights {
                        tensors: Vec::new(),
                        end_offset: 0,
                    },
                    extent: None,
                    bytes: 0,
                    owner: None,
                }),
            );
        }
        let reserve_len = align_up5(required).ok_or_else(overflow)?;
        let allocator = reserved
            .allocators
            .shared(ArenaKind::Weights)
            .ok_or(GpuError::Memory(AllocatorError::AllocationFailed))?;
        let region = reserved
            .regions
            .get(&ArenaKind::Weights)
            .ok_or_else(|| GpuError::Backend("weights arena is not reserved".to_string()))?;
        let extent = allocator.allocate(reserve_len).map_err(GpuError::Memory)?;
        let release = |extent: &RegionHandle| {
            if let Err(e) = allocator.free(extent) {
                warn!("weight extent at {} could not be freed: {e}", extent.offset());
            }
        };
        if extent.offset() % WEIGHT_ALIGN != 0 {
            release(&extent);
            return Err(GpuError::Memory(AllocatorError::AllocationFailed));
        }
        let loaded = match file
            .copy_to_arena(&reserved.weights, region, extent.offset(), select)
        {
            Ok(loaded) => loaded,
            Err(e) => {
                release(&extent);
                return Err(GpuError::Weights(e));
            }
        };
        Ok(
            Arc::new(WeightSet {
                weights: loaded,
                extent: Some(extent),
                bytes: reserve_len,
                owner: Some(Arc::clone(&reserved)),
            }),
        )
    }

    /// Sum of the live extent sizes.
    pub fn weights_used(&self) -> u64 {
        match &*lock(&self.phase) {
            Phase3::Ready(r) => r.bookkeeping.live_bytes(ArenaKind::Weights),
            _ => 0,
        }
    }

    pub fn weights_capacity(&self) -> u64 {
        self.capacities.get(ArenaKind::Weights)
    }
}

#[cfg(test)]
#[path = "test/vram.rs"]
mod tests;
