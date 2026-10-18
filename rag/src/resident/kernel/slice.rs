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
mod tests {
    use super::*;

    fn range(n: u32) -> SliceRange {
        SliceRange { segment: 0, slot_from: 0, slot_to: n }
    }

    fn sizer() -> SliceSizer {
        SliceSizer::new(Duration::from_millis(2), 10, 1000)
    }

    #[test]
    fn size_adapts_to_throughput_within_bounds() {
        let mut s = sizer();
        assert_eq!(s.next_len(), 10);
        s.record(100, Duration::from_millis(1));
        assert_eq!(s.next_len(), 200);
        let mut fast = sizer();
        fast.record(100, Duration::from_micros(1));
        assert_eq!(fast.next_len(), 1000);
        let mut slow = sizer();
        slow.record(1, Duration::from_secs(1));
        assert_eq!(slow.next_len(), 10);
    }

    #[test]
    fn slices_cover_range_exactly_once() {
        let mut seen = Vec::new();
        let out = run_sliced::<()>(range(95), &mut sizer(), None, || false, |r| {
            seen.push((r.slot_from, r.slot_to));
            Ok(Duration::from_millis(2))
        })
        .unwrap();
        assert_eq!(out.done, 95);
        assert!(out.remaining.is_none());
        assert_eq!(seen.first().unwrap().0, 0);
        assert_eq!(seen.last().unwrap().1, 95);
        assert!(seen.windows(2).all(|w| w[0].1 == w[1].0));
    }

    #[test]
    fn preemption_happens_only_at_boundaries_and_resumes() {
        let mut calls = 0;
        let mut ran = Vec::new();
        let out = run_sliced::<()>(
            range(100),
            &mut sizer(),
            None,
            || {
                calls += 1;
                true
            },
            |r| {
                ran.push(r);
                Ok(Duration::from_millis(2))
            },
        )
        .unwrap();
        assert_eq!(out.stopped, Some(StopReason::Preempted));
        assert_eq!(ran.len(), 1);
        assert_eq!(calls, 1);
        let rest = out.remaining.unwrap();
        assert_eq!(rest.slot_from, out.done);
        assert_eq!(rest.slot_to, 100);
    }

    #[test]
    fn deadline_stops_between_slices() {
        let out = run_sliced::<()>(
            range(1000),
            &mut sizer(),
            Some(Duration::from_millis(3)),
            || false,
            |_| Ok(Duration::from_millis(2)),
        )
        .unwrap();
        assert_eq!(out.stopped, Some(StopReason::Deadline));
        assert!(out.remaining.is_some());
    }

    #[test]
    fn exec_error_propagates() {
        let r = run_sliced(range(10), &mut sizer(), None, || false, |_| Err::<Duration, _>("boom"));
        assert_eq!(r.unwrap_err(), "boom");
    }
}
