use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::notification_topology::NotificationTopology;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, LaneConfig, LaneKind, MockEventControl,
};
use std::sync::atomic::AtomicUsize;
use std::thread;

fn tracker() -> (GpuCompletion, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&count);
    let topo = NotificationTopology::new(
        1,
        Arc::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
        }),
    );
    (GpuCompletion::new(topo.notifier(0).unwrap()), count)
}

fn lane() -> Lane {
    Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None)
}

fn submit(gc: &GpuCompletion, tag: u64) -> (CompletionCallback, MockEventControl) {
    let (event, ctl) = Event::mock();
    (gc.submit(lane(), event, tag), ctl)
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn types_are_send_and_sync() {
    assert_send::<GpuCompletion>();
    assert_sync::<GpuCompletion>();
    assert_send::<CompletionCallback>();
    assert_send::<FinishedLane>();
}

#[test]
fn empty_poll_returns_nothing() {
    let (gc, _) = tracker();
    assert!(gc.poll().is_empty());
    assert_eq!(gc.in_flight(), 0);
}

#[test]
fn pending_lane_stays_tracked() {
    let (gc, count) = tracker();
    let (_cb, _ctl) = submit(&gc, 1);
    assert!(gc.poll().is_empty());
    assert_eq!(gc.in_flight(), 1);
    assert_eq!(count.load(Ordering::SeqCst), 0);
}

#[test]
fn callback_hands_over_lane() {
    let (gc, count) = tracker();
    let (cb, _ctl) = submit(&gc, 7);
    cb.fire();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    let done = gc.poll();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].tag, 7);
    assert_eq!(done[0].via, TakenVia::Callback);
    assert!(done[0].result.is_ok());
    assert_eq!(gc.in_flight(), 0);
    assert!(gc.poll().is_empty());
}

#[test]
fn missed_callback_is_found_by_event_query() {
    let (gc, count) = tracker();
    let (_cb, ctl) = submit(&gc, 3);
    ctl.complete();
    let done = gc.poll();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].via, TakenVia::EventQuery);
    assert!(done[0].result.is_ok());
    assert_eq!(count.load(Ordering::SeqCst), 0);
}

#[test]
fn failed_event_still_hands_over_lane() {
    let (gc, _) = tracker();
    let (_cb, ctl) = submit(&gc, 4);
    ctl.fail("boom");
    let done = gc.poll();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].via, TakenVia::EventQuery);
    assert_eq!(done[0].result.as_ref().unwrap_err().cause, "boom");
    assert_eq!(gc.in_flight(), 0);
}

#[test]
fn done_slot_makes_no_event_query() {
    let (gc, _) = tracker();
    let (cb, ctl) = submit(&gc, 5);
    ctl.fail("ignored");
    cb.fire();
    let done = gc.poll();
    assert_eq!(done[0].via, TakenVia::Callback);
    assert!(done[0].result.is_ok());
}

#[test]
fn late_callback_after_take_is_harmless_but_notifies() {
    let (gc, count) = tracker();
    let (cb, ctl) = submit(&gc, 6);
    ctl.complete();
    assert_eq!(gc.poll().len(), 1);
    cb.fire();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert!(gc.poll().is_empty());
    assert_eq!(gc.in_flight(), 0);
}

#[test]
fn dropped_callback_does_nothing() {
    let (gc, count) = tracker();
    let (cb, _ctl) = submit(&gc, 8);
    drop(cb);
    assert_eq!(count.load(Ordering::SeqCst), 0);
    assert!(gc.poll().is_empty());
    assert_eq!(gc.in_flight(), 1);
}

#[test]
fn finished_lanes_are_returned_in_submit_order() {
    let (gc, _) = tracker();
    let (cb0, _c0) = submit(&gc, 0);
    let (_cb1, c1) = submit(&gc, 1);
    let (_cb2, _c2) = submit(&gc, 2);
    let (cb3, _c3) = submit(&gc, 3);
    cb3.fire();
    c1.complete();
    cb0.fire();
    let tags: Vec<u64> = gc.poll().iter().map(|f| f.tag).collect();
    assert_eq!(tags, vec![0, 1, 3]);
    assert_eq!(gc.in_flight(), 1);
}

#[test]
fn callback_and_poll_race_hands_over_exactly_once() {
    let (gc, _) = tracker();
    let gc = Arc::new(gc);
    let n = 200u64;
    let mut callbacks = Vec::new();
    let mut ctls = Vec::new();
    for tag in 0..n {
        let (cb, ctl) = submit(&gc, tag);
        callbacks.push(cb);
        ctls.push(ctl);
    }
    let firing = thread::spawn(move || {
        for cb in callbacks {
            cb.fire();
        }
        ctls
    });
    let pollers: Vec<_> = (0..3)
        .map(|_| {
            let gc = Arc::clone(&gc);
            thread::spawn(move || {
                let mut seen = Vec::new();
                while seen.len() < n as usize {
                    let got = gc.poll();
                    if got.is_empty() && gc.in_flight() == 0 {
                        break;
                    }
                    seen.extend(got.into_iter().map(|f| f.tag));
                }
                seen
            })
        })
        .collect();
    let _ctls = firing.join().unwrap();
    let mut all: Vec<u64> = pollers
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all.sort_unstable();
    assert_eq!(all, (0..n).collect::<Vec<_>>());
    assert_eq!(gc.in_flight(), 0);
}

#[test]
fn fire_works_on_another_thread() {
    let (gc, count) = tracker();
    let (cb, _ctl) = submit(&gc, 9);
    thread::spawn(move || cb.fire()).join().unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(gc.poll().len(), 1);
}
