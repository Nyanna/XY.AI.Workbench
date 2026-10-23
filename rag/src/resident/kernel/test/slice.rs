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
