//! Work-slice scheduling: bounded launches sized from observed throughput, with yield points only
//! at slice boundaries.

use std::time::Duration;

use super::SliceRange;
use crate::resident::subengine::Ewma;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopReason {
    Preempted,
    Deadline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceOutcome {
    pub done: u32,
    /// Work left when stopped early; resume from here.
    pub remaining: Option<SliceRange>,
    pub stopped: Option<StopReason>,
}

/// Chooses slice lengths so one launch lasts about `target`.
#[derive(Debug)]
pub struct SliceSizer {
    target: Duration,
    min: u32,
    max: u32,
    rate: Ewma,
}

impl SliceSizer {
    pub const DEFAULT_TARGET: Duration = Duration::from_millis(2);

    pub fn new(target: Duration, min: u32, max: u32) -> Self {
        let min = min.max(1);
        Self { target, min, max: max.max(min), rate: Ewma::new(0.3) }
    }

    /// Slots per second; `None` before the first observation.
    pub fn throughput(&self) -> Option<f32> {
        self.rate.get()
    }

    /// Starts at the minimum until a throughput sample exists.
    pub fn next_len(&self) -> u32 {
        match self.rate.get() {
            None => self.min,
            Some(rate) => {
                let slots = f64::from(rate) * self.target.as_secs_f64();
                (slots as u64).clamp(u64::from(self.min), u64::from(self.max)) as u32
            }
        }
    }

    pub fn record(&mut self, slots: u32, elapsed: Duration) {
        if slots == 0 {
            return;
        }
        let secs = elapsed.as_secs_f64().max(1e-6);
        self.rate.observe((f64::from(slots) / secs) as f32);
    }
}

impl Default for SliceSizer {
    fn default() -> Self {
        Self::new(Self::DEFAULT_TARGET, 16, 1 << 20)
    }
}

/// Runs `range` as consecutive slices. `exec` returns the elapsed time of the slice it ran.
/// `should_yield` is polled after each completed slice, never inside one. At least one slice always
/// completes, so the caller makes progress even under permanent preemption pressure.
pub fn run_sliced<E>(
    range: SliceRange,
    sizer: &mut SliceSizer,
    budget: Option<Duration>,
    mut should_yield: impl FnMut() -> bool,
    mut exec: impl FnMut(SliceRange) -> Result<Duration, E>,
) -> Result<SliceOutcome, E> {
    let mut rest = range;
    let mut done = 0u32;
    let mut spent = Duration::ZERO;
    while !rest.is_empty() {
        let (head, tail) = rest.split_at(sizer.next_len());
        let elapsed = exec(head)?;
        sizer.record(head.len(), elapsed);
        done += head.len();
        spent += elapsed;
        rest = tail;
        if rest.is_empty() {
            break;
        }
        let stopped = if budget.is_some_and(|b| spent >= b) {
            Some(StopReason::Deadline)
        } else if should_yield() {
            Some(StopReason::Preempted)
        } else {
            None
        };
        if stopped.is_some() {
            return Ok(SliceOutcome { done, remaining: Some(rest), stopped });
        }
    }
    Ok(SliceOutcome { done, remaining: None, stopped: None })
}

#[cfg(test)]
#[path = "test/slice.rs"]
mod tests;
