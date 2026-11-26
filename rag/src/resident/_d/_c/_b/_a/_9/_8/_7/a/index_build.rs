//! Index build over a pre-allocated segment: the kernel writes only into spans
//! handed out by a cursor, and nothing allocates after `begin`.
use std::borrow::Borrow;
use std::fmt;
use super::super::_6::a::segment_arena::{
    BumpSegment, Readers, Segment, SegmentArena, SegmentArenaError,
};
use super::super::_6::_5::_4::a::allocator::Allocator;
use super::super::_6::_5::_4::_3::a::region_handle::{
    Generation, RegionError, RegionHandle,
};
/// Writable span granted up front; `offset` is an absolute arena offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WriteSpan {
    pub offset: u64,
    pub len: u64,
}
impl WriteSpan {
    /// `offset + len`; overflow-free because it was checked at grant time.
    pub fn end(&self) -> u64 {
        self.offset.saturating_add(self.len)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexBuildError {
    Arena(SegmentArenaError),
    Handle(RegionError),
}
impl fmt::Display for IndexBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexBuildError::Arena(e) => write!(f, "index build arena error: {e}"),
            IndexBuildError::Handle(e) => write!(f, "index build handle error: {e}"),
        }
    }
}
impl std::error::Error for IndexBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IndexBuildError::Arena(e) => Some(e),
            IndexBuildError::Handle(e) => Some(e),
        }
    }
}
impl From<SegmentArenaError> for IndexBuildError {
    fn from(e: SegmentArenaError) -> Self {
        IndexBuildError::Arena(e)
    }
}
impl From<RegionError> for IndexBuildError {
    fn from(e: RegionError) -> Self {
        IndexBuildError::Handle(e)
    }
}
/// Result of a finished build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuiltIndex {
    /// The whole pre-allocated segment, to be released through the arena.
    pub segment: Segment,
    /// Bytes from `segment.offset` granted by the cursor, including padding.
    pub written: u64,
}
impl BuiltIndex {
    pub fn handle(
        &self,
        generation: Generation,
    ) -> Result<RegionHandle, IndexBuildError> {
        Ok(
            RegionHandle::new(
                self.segment.region.arena(),
                self.segment.offset,
                self.written,
                generation,
            )?,
        )
    }
}
/// Single-writer index build. Has no `Drop` effect: without `finish` or
/// `abort` the segment stays allocated.
pub struct IndexBuild<'a, A: Borrow<Allocator>> {
    arena: &'a SegmentArena<A>,
    bump: BumpSegment,
}
impl<'a, A: Borrow<Allocator>> IndexBuild<'a, A> {
    pub fn begin(
        arena: &'a SegmentArena<A>,
        capacity: u64,
    ) -> Result<Self, IndexBuildError> {
        let bump = arena.allocate_bump(capacity)?;
        Ok(IndexBuild { arena, bump })
    }
    pub fn reserve(
        &mut self,
        len: u64,
        align: u64,
    ) -> Result<WriteSpan, IndexBuildError> {
        let offset = self.bump.claim(len, align)?;
        Ok(WriteSpan { offset, len })
    }
    pub fn written(&self) -> u64 {
        self.bump.written()
    }
    pub fn remaining(&self) -> u64 {
        self.bump.remaining()
    }
    pub fn segment(&self) -> &Segment {
        self.bump.segment()
    }
    pub fn finish(self) -> BuiltIndex {
        let (segment, written) = self.bump.into_parts();
        BuiltIndex { segment, written }
    }
    pub fn abort(self, readers: Readers) -> Result<(), IndexBuildError> {
        let (segment, _) = self.bump.into_parts();
        self.arena.release(&segment, readers)?;
        Ok(())
    }
}
#[cfg(test)]
#[path = "test/index_build.rs"]
mod tests;
