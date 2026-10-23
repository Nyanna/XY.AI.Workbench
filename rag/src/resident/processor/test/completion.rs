use std::sync::atomic::AtomicUsize;
use std::sync::Barrier;
use super::*;
fn board(lanes: usize) -> (Arc<LaneBoard>, Arc<ManualCompletion>) {
    (
        Arc::new(LaneBoard::new(lanes, Arc::new(Notify::new()))),
        Arc::new(ManualCompletion::new(lanes)),
    )
}
#[test]
fn missed_callback_is_picked_up_by_poll() {
    let (b, c) = board(1);
    let lane = b.reserve().unwrap();
    b.launch(lane, &*c);
    assert_eq!(b.reap(&* c, | _, _ | panic!("not finished")), 0);
    c.finish(lane, LaneOutcome::Success);
    let mut seen = vec![];
    assert_eq!(b.reap(&* c, | l, o | seen.push((l, o))), 1);
    assert_eq!(seen, vec![(lane, LaneOutcome::Success)]);
    assert!(b.is_idle(lane));
}
#[test]
fn callback_without_poll_completes() {
    let (b, c) = board(1);
    let lane = b.reserve().unwrap();
    b.launch(lane, &*c);
    b.signal(lane);
    assert_eq!(b.reap(&* c, | _, o | assert_eq!(o, LaneOutcome::Success)), 1);
}
#[test]
fn failure_is_reported_by_poll() {
    let (b, c) = board(1);
    let lane = b.reserve().unwrap();
    b.launch(lane, &*c);
    c.finish(lane, LaneOutcome::Failed);
    assert_eq!(b.reap(&* c, | _, o | assert_eq!(o, LaneOutcome::Failed)), 1);
}
#[test]
fn repeated_signals_complete_lane_once() {
    let (b, c) = board(1);
    let lane = b.reserve().unwrap();
    b.launch(lane, &*c);
    b.signal(lane);
    b.signal(lane);
    assert_eq!(b.reap(&* c, | _, _ | {}), 1);
    assert_eq!(b.reap(&* c, | _, _ | {}), 0);
}
#[test]
fn reserved_lane_is_not_reaped_and_can_abort() {
    let (b, c) = board(1);
    let lane = b.reserve().unwrap();
    assert!(b.reserve().is_none());
    b.signal(lane);
    assert_eq!(b.reap(&* c, | _, _ | {}), 0);
    b.abort(lane);
    assert!(b.is_idle(lane));
}
#[test]
fn concurrent_callback_and_pollers_process_once() {
    for _ in 0..200 {
        let (b, c) = board(1);
        let lane = b.reserve().unwrap();
        b.launch(lane, &*c);
        c.finish(lane, LaneOutcome::Success);
        let count = Arc::new(AtomicUsize::new(0));
        let gate = Arc::new(Barrier::new(4));
        let mut threads = vec![];
        for t in 0..4 {
            let (b, c, count, gate) = (
                b.clone(),
                c.clone(),
                count.clone(),
                gate.clone(),
            );
            threads
                .push(
                    std::thread::spawn(move || {
                        gate.wait();
                        if t == 0 {
                            b.signal(lane);
                        }
                        b.reap(
                            &*c,
                            |_, _| {
                                count.fetch_add(1, Ordering::SeqCst);
                            },
                        );
                    }),
                );
        }
        for t in threads {
            t.join().unwrap();
        }
        b.reap(
            &*c,
            |_, _| {
                count.fetch_add(1, Ordering::SeqCst);
            },
        );
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
