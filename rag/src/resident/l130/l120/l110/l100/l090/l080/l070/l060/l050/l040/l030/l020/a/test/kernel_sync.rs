use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    LaneConfig, LaneKind,
};
use std::sync::atomic::AtomicUsize;

fn lane() -> Lane {
    Lane::new(LaneConfig::preset(LaneKind::Update, DeviceHandle::Cpu), None)
}

fn ok_change(log: &Arc<Mutex<Vec<usize>>>, id: usize) -> Change {
    let log = log.clone();
    Box::new(move |_| {
        log.lock().push(id);
        Ok(())
    })
}

const LONG: Duration = Duration::from_secs(5);

#[test]
fn nothing_runs_without_gap() {
    let sync = KernelSync::new(4);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let h = sync.submit(&l, ok_change(&log, 1)).unwrap();
    assert_eq!(h.status(), ChangeStatus::Pending);
    assert_eq!(sync.pending(&l), 1);
    assert!(log.lock().is_empty());
}

#[test]
fn fifo_and_event_signalled() {
    let sync = KernelSync::new(8);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let hs: Vec<_> = (0..3)
        .map(|i| sync.submit(&l, ok_change(&log, i)).unwrap())
        .collect();
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 3);
    sync.gap_end(&l);
    assert_eq!(*log.lock(), vec![0, 1, 2]);
    for h in &hs {
        assert_eq!(h.status(), ChangeStatus::Applied);
        assert!(h.event().is_complete().unwrap());
    }
    assert_eq!(sync.pending(&l), 0);
}

#[test]
fn lanes_are_independent() {
    let sync = KernelSync::new(8);
    let a = lane();
    let b = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    sync.submit(&a, ok_change(&log, 1)).unwrap();
    let hb = sync.submit(&b, ok_change(&log, 2)).unwrap();
    assert_eq!(sync.gap_begin(&a, GapKind::CommSlot, LONG), 1);
    assert_eq!(hb.status(), ChangeStatus::Pending);
    assert_eq!(*log.lock(), vec![1]);
}

#[test]
fn queue_full_and_cancel_frees_slot() {
    let sync = KernelSync::new(1);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let h = sync.submit(&l, ok_change(&log, 1)).unwrap();
    assert_eq!(
        sync.submit(&l, ok_change(&log, 2)).err(),
        Some(SyncError::QueueFull)
    );
    assert!(h.cancel());
    assert!(!h.cancel());
    assert_eq!(h.status(), ChangeStatus::Cancelled);
    assert!(h.event().is_complete().unwrap());
    sync.submit(&l, ok_change(&log, 3)).unwrap();
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 1);
    assert_eq!(*log.lock(), vec![3]);
}

#[test]
fn zero_budget_applies_nothing() {
    let sync = KernelSync::new(4);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let h = sync.submit(&l, ok_change(&log, 1)).unwrap();
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, Duration::ZERO), 0);
    sync.gap_end(&l);
    assert_eq!(h.status(), ChangeStatus::Pending);
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 1);
}

#[test]
fn budget_exhaustion_keeps_order() {
    let sync = KernelSync::new(8);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let slow_log = log.clone();
    sync.submit(
        &l,
        Box::new(move |_| {
            std::thread::sleep(Duration::from_millis(30));
            slow_log.lock().push(0);
            Ok(())
        }),
    )
    .unwrap();
    sync.submit(&l, ok_change(&log, 1)).unwrap();
    sync.submit(&l, ok_change(&log, 2)).unwrap();
    assert_eq!(
        sync.gap_begin(&l, GapKind::BatchEnd, Duration::from_millis(10)),
        1
    );
    sync.gap_end(&l);
    assert_eq!(sync.pending(&l), 2);
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 2);
    assert_eq!(*log.lock(), vec![0, 1, 2]);
}

#[test]
fn failure_and_panic_do_not_stop_gap() {
    let sync = KernelSync::new(8);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let h1 = sync
        .submit(&l, Box::new(|_| Err(ChangeError::Failed("x".into()))))
        .unwrap();
    let h2 = sync.submit(&l, Box::new(|_| panic!("boom"))).unwrap();
    let h3 = sync.submit(&l, ok_change(&log, 3)).unwrap();
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 3);
    assert_eq!(h1.status(), ChangeStatus::Failed(ChangeError::Failed("x".into())));
    assert_eq!(h2.status(), ChangeStatus::Failed(ChangeError::Failed("boom".into())));
    assert_eq!(h3.status(), ChangeStatus::Applied);
}

#[test]
fn nested_gap_begin_is_noop_and_gap_end_reopens() {
    let sync = Arc::new(KernelSync::new(8));
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    sync.gap_end(&l);
    let inner = Arc::new(AtomicUsize::new(99));
    let (s2, i2) = (sync.clone(), inner.clone());
    let lp = &l as *const Lane as usize;
    sync.submit(
        &l,
        Box::new(move |ctx| {
            assert_eq!(lp, ctx.lane as *const Lane as usize);
            i2.store(s2.gap_begin(ctx.lane, ctx.kind, LONG), Ordering::SeqCst);
            Ok(())
        }),
    )
    .unwrap();
    assert_eq!(sync.gap_begin(&l, GapKind::CommSlot, LONG), 1);
    assert_eq!(inner.load(Ordering::SeqCst), 0);
    sync.gap_end(&l);
    sync.submit(&l, ok_change(&log, 1)).unwrap();
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 1);
}

#[test]
fn close_cancels_pending_and_rejects_submit() {
    let sync = KernelSync::new(4);
    let l = lane();
    let log = Arc::new(Mutex::new(Vec::new()));
    let h = sync.submit(&l, ok_change(&log, 1)).unwrap();
    sync.close();
    assert_eq!(h.status(), ChangeStatus::Cancelled);
    assert!(h.event().is_complete().unwrap());
    assert_eq!(
        sync.submit(&l, ok_change(&log, 2)).err(),
        Some(SyncError::Closed)
    );
    assert_eq!(sync.gap_begin(&l, GapKind::BatchEnd, LONG), 0);
}

#[test]
fn remaining_is_bounded_by_budget() {
    let sync = KernelSync::new(4);
    let l = lane();
    let seen = Arc::new(Mutex::new(None));
    let s = seen.clone();
    sync.submit(
        &l,
        Box::new(move |ctx| {
            *s.lock() = Some((ctx.remaining(), ctx.kind, *ctx.device));
            Ok(())
        }),
    )
    .unwrap();
    sync.gap_begin(&l, GapKind::CommSlot, Duration::from_secs(1));
    let (rem, kind, dev) = seen.lock().take().unwrap();
    assert!(rem <= Duration::from_secs(1));
    assert_eq!(kind, GapKind::CommSlot);
    assert_eq!(dev, DeviceHandle::Cpu);
}
