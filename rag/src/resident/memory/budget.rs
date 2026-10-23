//! Arena classes and the VRAM budget.

use super::error::MemoryError;
use super::region::ArenaId;

/// Arena classes by lifetime.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArenaKind {
    Weights,
    IndexSegment,
    Workspace,
    Staging,
    ParamBlock,
}

impl ArenaKind {
    pub const ALL: [ArenaKind; 5] = [
        ArenaKind::Weights,
        ArenaKind::IndexSegment,
        ArenaKind::Workspace,
        ArenaKind::Staging,
        ArenaKind::ParamBlock,
    ];

    pub const fn id(self) -> ArenaId {
        ArenaId(self as u32)
    }

    pub fn from_id(id: ArenaId) -> Option<Self> {
        Self::ALL.get(id.0 as usize).copied()
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// Capacity per arena class in bytes. `index_segment` covers all resident generations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArenaSizes {
    pub weights: u64,
    pub index_segment: u64,
    pub workspace: u64,
    pub staging: u64,
    pub param_block: u64,
}

impl ArenaSizes {
    pub fn get(&self, kind: ArenaKind) -> u64 {
        match kind {
            ArenaKind::Weights => self.weights,
            ArenaKind::IndexSegment => self.index_segment,
            ArenaKind::Workspace => self.workspace,
            ArenaKind::Staging => self.staging,
            ArenaKind::ParamBlock => self.param_block,
        }
    }

    /// Bytes that live on the device; staging is pinned host memory.
    pub fn device_total(&self) -> u64 {
        self.weights
            .saturating_add(self.index_segment)
            .saturating_add(self.workspace)
            .saturating_add(self.param_block)
    }

    /// Index bytes held back for generations other than the active one.
    pub fn generation_reserve(&self, generations: u32) -> u64 {
        let g = u64::from(generations.max(1));
        self.index_segment - self.index_segment / g
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budget {
    pub free: u64,
    pub total: u64,
    pub reserved_for_generations: u64,
}

impl Budget {
    pub fn usable(&self) -> u64 {
        self.free.saturating_sub(self.reserved_for_generations)
    }
}

/// Free/total device memory in bytes.
pub trait BudgetProbe: Send + Sync {
    fn probe(&self) -> Result<(u64, u64), MemoryError>;
}

/// Memory kept free for the driver context and other processes (compositor).
pub const DEFAULT_HEADROOM: u64 = 512 << 20;

/// Fails when the planned device arenas do not fit into the probed free memory.
pub fn check_fit(sizes: &ArenaSizes, free: u64, headroom: u64) -> Result<(), MemoryError> {
    let usable = free.saturating_sub(headroom);
    let requested = sizes.device_total();
    if requested > usable {
        return Err(MemoryError::BudgetExceeded { requested, usable });
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/budget.rs"]
mod tests;
