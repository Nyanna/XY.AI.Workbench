//! Per-arena region bookkeeping: first-fit allocation, generations and liveness.
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use log::{debug, warn};
use super::region_handle::{Generation, RegionHandle};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::{
    ArenaKind, ArenaOffset, ArenaSet,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocError {
    /// Out of space or injected fault. `free` is capacity minus live bytes.
    OutOfSpace { arena: ArenaKind, requested: u64, free: u64 },
    /// The arena has not been created via `ArenaSet::ensure`.
    ArenaNotCreated { arena: ArenaKind },
    /// `len == 0`.
    ZeroLength,
}
impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocError::OutOfSpace { arena, requested, free } => {
                write!(
                    f,
                    "out of space in arena {arena:?}: requested {requested} bytes, {free} free"
                )
            }
            AllocError::ArenaNotCreated { arena } => {
                write!(f, "arena {arena:?} has not been created")
            }
            AllocError::ZeroLength => write!(f, "zero-length allocation"),
        }
    }
}
impl std::error::Error for AllocError {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FreeError {
    ForeignArena { expected: ArenaKind, found: ArenaKind },
    UnknownRegion,
    AlreadyFreed,
    Mismatch,
}
impl fmt::Display for FreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreeError::ForeignArena { expected, found } => {
                write!(f, "region belongs to arena {found:?}, expected {expected:?}")
            }
            FreeError::UnknownRegion => write!(f, "unknown region"),
            FreeError::AlreadyFreed => write!(f, "region already freed"),
            FreeError::Mismatch => {
                write!(f, "region does not match the current allocation")
            }
        }
    }
}
impl std::error::Error for FreeError {}
/// Result of the ignoring release.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseOutcome {
    Released,
    IgnoredStale,
}
#[derive(Default)]
struct ArenaState {
    /// offset -> (len, generation) of live regions.
    live: BTreeMap<u64, (u64, Generation)>,
    /// (offset, len) -> last assigned generation; never forgotten.
    last_gen: HashMap<(u64, u64), Generation>,
    live_bytes: u64,
    inject: u32,
}
impl ArenaState {
    fn find_first_fit(&self, capacity: u64, len: u64) -> Option<u64> {
        let mut cursor = 0u64;
        for (&off, &(l, _)) in &self.live {
            if off.saturating_sub(cursor) >= len {
                return Some(cursor);
            }
            cursor = off.saturating_add(l);
        }
        (capacity.saturating_sub(cursor) >= len).then_some(cursor)
    }
    fn is_live(&self, r: &RegionHandle) -> bool {
        self.live.get(&r.offset()) == Some(&(r.len(), r.generation()))
    }
    fn remove(&mut self, r: &RegionHandle) {
        self.live.remove(&r.offset());
        self.live_bytes -= r.len();
    }
}
/// Thread-safe; `Send + Sync`.
pub struct Bookkeeping {
    arenas: Arc<ArenaSet>,
    state: Mutex<HashMap<ArenaKind, ArenaState>>,
}
impl Bookkeeping {
    pub fn new(arenas: Arc<ArenaSet>) -> Self {
        Bookkeeping {
            arenas,
            state: Mutex::new(HashMap::new()),
        }
    }
    fn lock(&self) -> MutexGuard<'_, HashMap<ArenaKind, ArenaState>> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
    pub fn allocate(
        &self,
        arena: ArenaKind,
        len: u64,
    ) -> Result<RegionHandle, AllocError> {
        let capacity = match self.arenas.get(arena) {
            Some(a) => a.capacity,
            None => {
                return Err(AllocError::ArenaNotCreated {
                    arena,
                });
            }
        };
        if len == 0 {
            return Err(AllocError::ZeroLength);
        }
        let mut guard = self.lock();
        let st = guard.entry(arena).or_default();
        let free = capacity.saturating_sub(st.live_bytes);
        if st.inject > 0 {
            st.inject -= 1;
            return Err(AllocError::OutOfSpace {
                arena,
                requested: len,
                free,
            });
        }
        let offset = match st.find_first_fit(capacity, len) {
            Some(o) => o,
            None => {
                return Err(AllocError::OutOfSpace {
                    arena,
                    requested: len,
                    free,
                });
            }
        };
        let generation = match st.last_gen.get(&(offset, len)) {
            Some(g) => g.next().unwrap_or(Generation::INITIAL),
            None => Generation::INITIAL,
        };
        let handle = match RegionHandle::new(arena, offset, len, generation) {
            Ok(handle) => handle,
            Err(_) => {
                return Err(AllocError::OutOfSpace {
                    arena,
                    requested: len,
                    free,
                });
            }
        };
        st.last_gen.insert((offset, len), generation);
        st.live.insert(offset, (len, generation));
        st.live_bytes += len;
        drop(guard);
        debug!("allocated arena={arena:?} offset={offset} len={len}");
        Ok(handle)
    }
    pub fn free(
        &self,
        arena: ArenaKind,
        region: &RegionHandle,
    ) -> Result<(), FreeError> {
        if region.arena() != arena {
            return Err(FreeError::ForeignArena {
                expected: arena,
                found: region.arena(),
            });
        }
        let mut guard = self.lock();
        let st = guard.get_mut(&arena).ok_or(FreeError::UnknownRegion)?;
        let live_at = st.live.get(&region.offset()).copied();
        if let Some((len, generation)) = live_at {
            if len == region.len() {
                if generation != region.generation() {
                    return Err(FreeError::Mismatch);
                }
                st.remove(region);
                drop(guard);
                debug!(
                    "freed arena={arena:?} offset={} len={}", region.offset(), region
                    .len()
                );
                return Ok(());
            }
        }
        if let Some(&last) = st.last_gen.get(&(region.offset(), region.len())) {
            return if last == region.generation() {
                Err(FreeError::AlreadyFreed)
            } else {
                Err(FreeError::Mismatch)
            };
        }
        if live_at.is_some() {
            return Err(FreeError::Mismatch);
        }
        Err(FreeError::UnknownRegion)
    }
    pub fn release_or_ignore(&self, region: &RegionHandle) -> ReleaseOutcome {
        let released = {
            let mut guard = self.lock();
            match guard.get_mut(&region.arena()) {
                Some(st) if st.is_live(region) => {
                    st.remove(region);
                    true
                }
                _ => false,
            }
        };
        if released {
            debug!(
                "released arena={:?} offset={} len={}", region.arena(), region.offset(),
                region.len()
            );
            ReleaseOutcome::Released
        } else {
            warn!(
                "ignoring stale region arena={:?} offset={} len={} generation={}", region
                .arena(), region.offset(), region.len(), region.generation().0
            );
            ReleaseOutcome::IgnoredStale
        }
    }
    pub fn is_live(&self, region: &RegionHandle) -> bool {
        self.lock().get(&region.arena()).is_some_and(|st| st.is_live(region))
    }
    pub fn resolve(&self, region: &RegionHandle) -> Option<(ArenaOffset, u64)> {
        if self.is_live(region) {
            Some((region.offset(), region.len()))
        } else {
            warn!(
                "resolve of stale region arena={:?} offset={} len={} generation={}",
                region.arena(), region.offset(), region.len(), region.generation().0
            );
            None
        }
    }
    /// The next `count` calls of `allocate` on `arena` fail with `OutOfSpace`.
    pub fn inject_alloc_failure(&self, arena: ArenaKind, count: u32) {
        self.lock().entry(arena).or_default().inject = count;
    }
    pub fn clear_alloc_failure(&self, arena: ArenaKind) {
        if let Some(st) = self.lock().get_mut(&arena) {
            st.inject = 0;
        }
    }
    pub fn live_bytes(&self, arena: ArenaKind) -> u64 {
        self.lock().get(&arena).map_or(0, |st| st.live_bytes)
    }
}
#[cfg(test)]
#[path = "test/bookkeeping.rs"]
mod tests;
