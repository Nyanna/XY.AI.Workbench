//! Logical memory arenas by lifetime class and device memory budget checks.

use std::fmt;
use std::sync::{Mutex, MutexGuard};

pub const MIB: u64 = 1024 * 1024;
pub const DEFAULT_HEADROOM: u64 = 512 * MIB;

const ARENA_COUNT: usize = 6;

/// Stable numeric IDs; order equals ID order. New variants only at the end with new IDs.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum ArenaKind {
    Weights = 0,
    IndexSegments = 1,
    Workspace = 2,
    Staging = 3,
    ParameterBlock = 4,
    MappedHost = 5,
}

impl ArenaKind {
    pub const ALL: [ArenaKind; ARENA_COUNT] = [
        ArenaKind::Weights,
        ArenaKind::IndexSegments,
        ArenaKind::Workspace,
        ArenaKind::Staging,
        ArenaKind::ParameterBlock,
        ArenaKind::MappedHost,
    ];

    pub fn id(self) -> u32 {
        self as u32
    }

    /// Unknown id yields `None`.
    pub fn from_id(id: u32) -> Option<ArenaKind> {
        Self::ALL.get(usize::try_from(id).ok()?).copied()
    }

    /// True for Weights, IndexSegments, Workspace and ParameterBlock.
    pub fn is_device(self) -> bool {
        matches!(
            self,
            ArenaKind::Weights
                | ArenaKind::IndexSegments
                | ArenaKind::Workspace
                | ArenaKind::ParameterBlock
        )
    }

    fn slot(self) -> usize {
        self as usize
    }
}

/// Address = 64-bit offset within an arena.
pub type ArenaOffset = u64;

/// Configured capacities in bytes. Default: all zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArenaCapacities {
    bytes: [u64; ARENA_COUNT],
}

impl ArenaCapacities {
    pub fn get(&self, k: ArenaKind) -> u64 {
        self.bytes[k.slot()]
    }

    pub fn set(&mut self, k: ArenaKind, bytes: u64) {
        self.bytes[k.slot()] = bytes;
    }

    /// Saturating sum over the device arenas; host arenas do not count.
    pub fn device_need(&self) -> u64 {
        ArenaKind::ALL
            .iter()
            .filter(|k| k.is_device())
            .fold(0u64, |acc, k| acc.saturating_add(self.get(*k)))
    }

    /// `index - index / max(generations, 1)`: the share of the index arena that
    /// is not the largest single generation.
    pub fn generation_reserve(&self, generations: u32) -> u64 {
        let idx = self.get(ArenaKind::IndexSegments);
        idx - idx / u64::from(generations.max(1))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemInfo {
    pub free: u64,
    pub total: u64,
}

/// Replaceable memory query, implemented by a device adapter or a test mock.
pub trait MemoryQuery: Send + Sync {
    fn query(&self) -> Result<MemInfo, String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BudgetError {
    MemoryQueryFailed { reason: String },
    DoesNotFit { requested: u64, usable: u64 },
}

impl fmt::Display for BudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BudgetError::MemoryQueryFailed { reason } => {
                write!(f, "memory query failed: {reason}")
            }
            BudgetError::DoesNotFit { requested, usable } => {
                write!(
                    f,
                    "does not fit: requested {requested} bytes, usable {usable} bytes"
                )
            }
        }
    }
}

impl std::error::Error for BudgetError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budget {
    pub free: u64,
    pub total: u64,
    pub generation_reserve: u64,
}

impl Budget {
    pub fn from_query(q: &dyn MemoryQuery, generation_reserve: u64) -> Result<Budget, BudgetError> {
        let info = q
            .query()
            .map_err(|reason| BudgetError::MemoryQueryFailed { reason })?;
        Ok(Budget {
            free: info.free,
            total: info.total,
            generation_reserve,
        })
    }

    /// `free - generation_reserve`, never negative.
    pub fn usable(&self) -> u64 {
        self.free.saturating_sub(self.generation_reserve)
    }
}

fn check_fit(requested: u64, free: u64, headroom: u64) -> Result<(), BudgetError> {
    let usable = free.saturating_sub(headroom);
    if requested <= usable {
        Ok(())
    } else {
        Err(BudgetError::DoesNotFit { requested, usable })
    }
}

/// Passes iff `device_need <= free.saturating_sub(headroom)`.
pub fn fit_check(caps: &ArenaCapacities, free: u64, headroom: u64) -> Result<(), BudgetError> {
    check_fit(caps.device_need(), free, headroom)
}

/// Like `fit_check`, with `cow_extra` bytes held twice during copy-on-write added.
pub fn fit_check_cow(
    caps: &ArenaCapacities,
    cow_extra: u64,
    free: u64,
    headroom: u64,
) -> Result<(), BudgetError> {
    check_fit(caps.device_need().saturating_add(cow_extra), free, headroom)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arena {
    pub kind: ArenaKind,
    pub capacity: u64,
}

impl Arena {
    /// `off + len <= capacity`, overflow-safe.
    pub fn contains(&self, off: ArenaOffset, len: u64) -> bool {
        off.checked_add(len).is_some_and(|end| end <= self.capacity)
    }
}

struct State {
    caps: ArenaCapacities,
    arenas: [Option<Arena>; ARENA_COUNT],
}

/// Thread-safe arena set; arenas are created lazily and exactly once.
pub struct ArenaSet {
    state: Mutex<State>,
}

impl ArenaSet {
    pub fn new(caps: ArenaCapacities) -> Self {
        ArenaSet {
            state: Mutex::new(State {
                caps,
                arenas: [None; ARENA_COUNT],
            }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn capacities(&self) -> ArenaCapacities {
        self.lock().caps
    }

    /// Affects only arenas that are not yet created.
    pub fn set_capacity(&self, k: ArenaKind, bytes: u64) {
        self.lock().caps.set(k, bytes);
    }

    /// Returns the arena, creating it on first call. Device arenas must fit
    /// `budget.usable()`; host arenas are not checked. Nothing is created on error.
    pub fn ensure(&self, k: ArenaKind, budget: &Budget) -> Result<Arena, BudgetError> {
        let mut st = self.lock();
        if let Some(a) = st.arenas[k.slot()] {
            return Ok(a);
        }
        let capacity = st.caps.get(k);
        if k.is_device() {
            let usable = budget.usable();
            if capacity > usable {
                return Err(BudgetError::DoesNotFit {
                    requested: capacity,
                    usable,
                });
            }
        }
        let arena = Arena { kind: k, capacity };
        st.arenas[k.slot()] = Some(arena);
        Ok(arena)
    }

    pub fn get(&self, k: ArenaKind) -> Option<Arena> {
        self.lock().arenas[k.slot()]
    }
}

#[cfg(test)]
#[path = "test/arenas.rs"]
mod tests;
