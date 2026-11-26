pub mod write_pointer_region;
use std::borrow::Borrow;
use std::fmt;
use log::warn;
use self::write_pointer_region::WriteResult;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::{
    Readers, Segment, SegmentArena, SegmentArenaError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::Allocator;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::kernel_batch::{
    BatchError, BatchExecutor, BatchId, BatchInputs, BatchInterface, LaunchOutcome,
    UsedRange,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::{
    RegionError, RegionHandle,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecConfig {
    /// Length of each step range in bytes (> 0).
    pub range_len: u64,
    /// Maximum number of ranges reserved per execution (> 0).
    pub max_ranges: usize,
}
/// Implemented by the backend: write-pointer result of the range a step filled.
pub trait StepProgress: Send + Sync {
    fn write_result(&self, range: RegionHandle) -> Result<WriteResult, String>;
}
#[derive(Clone, Copy, Debug)]
pub struct BatchInput {
    pub region: RegionHandle,
}
#[derive(Debug)]
pub struct BatchSuccess {
    /// From `end`, in assignment order (one entry per step).
    pub ranges: Vec<UsedRange>,
    /// Reserved segments in assignment order; owned by the caller afterwards.
    pub regions: Vec<Segment>,
    /// Number of partial answers (not an error).
    pub partials: u32,
    /// `hits` of the final `Hits` answer.
    pub hits: u64,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ExecError {
    /// `max_ranges` was already reached before a step.
    ArenaExhausted { max_ranges: usize },
    InvalidConfig,
    Arena(SegmentArenaError),
    Region(RegionError),
    Batch(BatchError),
    Progress(String),
    ProgressOutOfRange { processed: u64, input_len: u64 },
}
impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::ArenaExhausted { .. } => write!(f, "Arena erschöpft"),
            ExecError::InvalidConfig => write!(f, "invalid execution config"),
            ExecError::Arena(e) => write!(f, "{e}"),
            ExecError::Region(e) => write!(f, "region handle: {e}"),
            ExecError::Batch(e) => write!(f, "batch: {e}"),
            ExecError::Progress(e) => write!(f, "step progress: {e}"),
            ExecError::ProgressOutOfRange { processed, input_len } => {
                write!(f, "processed {processed} bytes exceed input length {input_len}")
            }
        }
    }
}
impl std::error::Error for ExecError {}
const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = {
    assert_send_sync_static::<ExecConfig>();
    assert_send_sync_static::<BatchInput>();
    assert_send_sync_static::<BatchSuccess>();
    assert_send_sync_static::<ExecError>();
};
/// Runs an already begun batch to its hits. Always ends the batch; on error all
/// reserved segments are released, on success they belong to the caller.
pub fn execute_embedding_batch<E, A>(
    batch: &BatchInterface<E>,
    arena: &SegmentArena<A>,
    progress: &dyn StepProgress,
    id: BatchId,
    input: BatchInput,
    config: &ExecConfig,
) -> Result<BatchSuccess, ExecError>
where
    E: BatchExecutor,
    A: Borrow<Allocator>,
{
    let mut segments: Vec<Segment> = Vec::new();
    let steps = run_steps(batch, arena, progress, id, input, config, &mut segments);
    let ended = batch.end(id);
    match (steps, ended) {
        (Ok((partials, hits)), Ok(ranges)) => {
            Ok(BatchSuccess {
                ranges,
                regions: segments,
                partials,
                hits,
            })
        }
        (Ok(_), Err(e)) => {
            release_all(arena, &segments);
            Err(ExecError::Batch(e))
        }
        (Err(e), _) => {
            release_all(arena, &segments);
            Err(e)
        }
    }
}
fn run_steps<E, A>(
    batch: &BatchInterface<E>,
    arena: &SegmentArena<A>,
    progress: &dyn StepProgress,
    id: BatchId,
    input: BatchInput,
    config: &ExecConfig,
    segments: &mut Vec<Segment>,
) -> Result<(u32, u64), ExecError>
where
    E: BatchExecutor,
    A: Borrow<Allocator>,
{
    if config.range_len == 0 || config.max_ranges == 0 {
        return Err(ExecError::InvalidConfig);
    }
    let mut partials: u32 = 0;
    let mut current = input.region;
    loop {
        if segments.len() >= config.max_ranges {
            return Err(ExecError::ArenaExhausted {
                max_ranges: config.max_ranges,
            });
        }
        let seg = arena.allocate(config.range_len).map_err(ExecError::Arena)?;
        segments.push(seg);
        let handle = RegionHandle::new(
                seg.region.arena(),
                seg.offset,
                seg.len,
                seg.region.generation(),
            )
            .map_err(ExecError::Region)?;
        batch.assign_ranges(id, &[handle]).map_err(ExecError::Batch)?;
        let inputs = BatchInputs {
            regions: vec![current],
        };
        match batch.run(id, &inputs).map_err(ExecError::Batch)? {
            LaunchOutcome::Hits { hits } => return Ok((partials, hits)),
            LaunchOutcome::Partial { .. } => {
                partials = partials.saturating_add(1);
                let processed = progress
                    .write_result(handle)
                    .map_err(ExecError::Progress)?
                    .valid_len();
                if processed > current.len() {
                    return Err(ExecError::ProgressOutOfRange {
                        processed,
                        input_len: current.len(),
                    });
                }
                let offset = current
                    .offset()
                    .checked_add(processed)
                    .ok_or(ExecError::Region(RegionError::Overflow))?;
                current = RegionHandle::new(
                        current.arena(),
                        offset,
                        current.len() - processed,
                        current.generation(),
                    )
                    .map_err(ExecError::Region)?;
            }
        }
    }
}
fn release_all<A: Borrow<Allocator>>(arena: &SegmentArena<A>, segments: &[Segment]) {
    for seg in segments {
        if let Err(e) = arena.release(seg, Readers::None) {
            warn!("releasing embedding batch segment failed: {e}");
        }
    }
}
#[cfg(test)]
#[path = "test/embedding_batch.rs"]
mod tests;
