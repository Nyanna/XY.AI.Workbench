use std::fmt;

use super::super::l020::a::arenas::{Arena, ArenaKind, ArenaOffset};

/// Monotonic generation counter of a region; assigned by the caller/backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(pub u64);

impl Generation {
    pub const INITIAL: Generation = Generation(0);

    /// Successor; `None` on overflow (no wrapping).
    pub fn next(self) -> Option<Generation> {
        self.0.checked_add(1).map(Generation)
    }
}

/// Pointer-free region handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionHandle {
    arena: ArenaKind,
    offset: ArenaOffset,
    len: u64,
    generation: Generation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionError {
    /// `offset + len` overflows `u64`.
    Overflow,
    /// Region does not lie completely within the arena.
    OutOfBounds { offset: u64, len: u64, capacity: u64 },
    /// Handle arena kind differs from the supplied arena.
    ArenaMismatch { handle: ArenaKind, arena: ArenaKind },
}

impl fmt::Display for RegionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionError::Overflow => write!(f, "region offset + length overflows u64"),
            RegionError::OutOfBounds { offset, len, capacity } => write!(
                f,
                "region [offset {offset}, len {len}) exceeds arena capacity {capacity}"
            ),
            RegionError::ArenaMismatch { handle, arena } => write!(
                f,
                "arena mismatch: handle refers to {handle:?}, got {arena:?}"
            ),
        }
    }
}

impl std::error::Error for RegionError {}

impl RegionHandle {
    /// Without arena context; only checks `offset + len`.
    pub fn new(
        arena: ArenaKind,
        offset: ArenaOffset,
        len: u64,
        generation: Generation,
    ) -> Result<RegionHandle, RegionError> {
        offset.checked_add(len).ok_or(RegionError::Overflow)?;
        Ok(RegionHandle { arena, offset, len, generation })
    }

    /// Like `new`, additionally validated against the arena.
    pub fn new_in(
        arena: &Arena,
        offset: ArenaOffset,
        len: u64,
        generation: Generation,
    ) -> Result<RegionHandle, RegionError> {
        offset.checked_add(len).ok_or(RegionError::Overflow)?;
        if !arena.contains(offset, len) {
            return Err(RegionError::OutOfBounds {
                offset,
                len,
                capacity: arena.capacity,
            });
        }
        Ok(RegionHandle { arena: arena.kind, offset, len, generation })
    }

    pub fn arena(&self) -> ArenaKind {
        self.arena
    }

    pub fn offset(&self) -> ArenaOffset {
        self.offset
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// Exclusive end: `offset + len` (never overflows, see `new`).
    pub fn end(&self) -> u64 {
        self.offset + self.len
    }

    /// True when the generation equals `current`.
    pub fn is_current(&self, current: Generation) -> bool {
        self.generation == current
    }

    /// Same region with a new generation.
    pub fn with_generation(&self, generation: Generation) -> RegionHandle {
        RegionHandle { generation, ..*self }
    }

    /// Checks kind and bounds against an arena; generation is ignored.
    pub fn check_in(&self, arena: &Arena) -> Result<(), RegionError> {
        if arena.kind != self.arena {
            return Err(RegionError::ArenaMismatch {
                handle: self.arena,
                arena: arena.kind,
            });
        }
        if !arena.contains(self.offset, self.len) {
            return Err(RegionError::OutOfBounds {
                offset: self.offset,
                len: self.len,
                capacity: arena.capacity,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/region_handle.rs"]
mod tests;
