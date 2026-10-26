use super::*;
use crate::resident::memory::ArenaId;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
fn cfg() -> LaneConfig {
    LaneConfig::new(Region {
        arena: ArenaId(1),
        offset: 8,
        len: 256,
        gen: 3,
    })
}
fn lane(class: LaneClass) -> CpuLane {
    CpuLane::new(class, cfg()).unwrap()
}
#[test]
fn lane_class_is_open_for_long_lived_variants() {
    fn priority_or_default(class: LaneClass) -> i32 {
        match class {
            LaneClass::Query | LaneClass::Index | LaneClass::Copy => class.priority(),
            #[allow(unreachable_patterns)]
            _ => 0,
        }
    }
    assert_eq!(priority_or_default(LaneClass::Query), LaneClass::Query.priority());
}
#[test]
fn jobs_run_in_submission_order() {
    let lane = lane(LaneClass::Query);
    let log = Arc::new(Mutex::new(Vec::new()));
    for i in 0..32 {
        let log = log.clone();
        lane.submit(
                Box::new(move || {
                    log.lock().push(i);
                    Ok(())
                }),
            )
            .unwrap();
    }
    lane.synchronize().unwrap();
    assert_eq!(* log.lock(), (0..32).collect::< Vec < _ >> ());
}
#[test]
fn event_completes_after_prior_jobs() {
    let lane = lane(LaneClass::Index);
    let hits = Arc::new(AtomicUsize::new(0));
    let h = hits.clone();
    lane.submit(
            Box::new(move || {
                std::thread::sleep(Duration::from_millis(20));
                h.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }),
        )
        .unwrap();
    let event = lane.record_event().unwrap();
    event.synchronize().unwrap();
    assert!(event.is_complete());
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}
#[test]
fn wait_orders_work_across_lanes() {
    let producer = lane(LaneClass::Copy);
    let consumer = lane(LaneClass::Query);
    let value = Arc::new(AtomicUsize::new(0));
    let seen = Arc::new(AtomicUsize::new(usize::MAX));
    let v = value.clone();
    producer
        .submit(
            Box::new(move || {
                std::thread::sleep(Duration::from_millis(30));
                v.store(7, Ordering::SeqCst);
                Ok(())
            }),
        )
        .unwrap();
    let event = producer.record_event().unwrap();
    consumer.wait(&event).unwrap();
    let (v, s) = (value.clone(), seen.clone());
    consumer
        .submit(
            Box::new(move || {
                s.store(v.load(Ordering::SeqCst), Ordering::SeqCst);
                Ok(())
            }),
        )
        .unwrap();
    consumer.synchronize().unwrap();
    assert_eq!(seen.load(Ordering::SeqCst), 7);
}
#[test]
fn first_failure_is_reported_once() {
    let lane = lane(LaneClass::Index);
    lane.submit(Box::new(|| Err(DeviceError::LaneClosed))).unwrap();
    lane.submit(Box::new(|| panic!("boom"))).unwrap();
    assert!(matches!(lane.synchronize(), Err(DeviceError::LaneClosed)));
    assert!(lane.synchronize().is_ok());
}
#[test]
fn drop_drains_pending_jobs() {
    let hits = Arc::new(AtomicUsize::new(0));
    {
        let lane = lane(LaneClass::Index);
        let h = hits.clone();
        lane.submit(
                Box::new(move || {
                    h.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }),
            )
            .unwrap();
    }
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}
#[test]
fn priorities_rank_query_above_copy_above_index() {
    assert!(LaneClass::Query.priority() < LaneClass::Copy.priority());
    assert!(LaneClass::Copy.priority() < LaneClass::Index.priority());
}
#[test]
fn param_block_aligns_and_bounds() {
    let mut p = ParamBlock::new(16);
    assert_eq!(p.push(1u8).unwrap(), 0);
    assert_eq!(p.push(2u32).unwrap(), 4);
    assert_eq!(p.push(3u64).unwrap(), 8);
    assert_eq!(p.as_bytes().len(), 16);
    assert!(
        matches!(p.push(1u8), Err(DeviceError::ParamOverflow { capacity : 16, .. }))
    );
    p.clear();
    assert_eq!(p.push(1.5f32).unwrap(), 0);
}
#[test]
fn scratch_respects_alignment_bounds_and_reset() {
    let s = ScratchArena::new(cfg().scratch);
    let a = s.alloc(10, 16).unwrap();
    assert_eq!(a.offset % 16, 0);
    assert_eq!((a.arena, a.gen), (ArenaId(1), 3));
    let b = s.alloc(10, 1).unwrap();
    assert!(b.offset >= a.offset + a.len);
    assert!(matches!(s.alloc(1024, 1), Err(DeviceError::ScratchExhausted { .. })));
    assert!(matches!(s.alloc(1, 3), Err(DeviceError::InvalidAlign { align : 3 })));
    s.reset();
    assert_eq!(s.used(), 0);
    assert!(s.alloc(256 - 8, 1).is_ok());
}
#[test]
fn recording_lane_logs_calls() {
    let mut lane = RecordingLane::new(LaneClass::Query, cfg());
    lane.submit(Box::new(|| Ok(()))).unwrap();
    let e = lane.record_event().unwrap();
    lane.wait(&e).unwrap();
    lane.synchronize().unwrap();
    lane.param_block().push(1u32).unwrap();
    assert_eq!(lane.calls(), vec!["submit", "record", "wait", "sync"]);
}
#[test]
fn lanes_work_as_trait_objects() {
    let mut lanes: Vec<Box<dyn ExecContext>> = vec![
        Box::new(lane(LaneClass::Query)), Box::new(RecordingLane::new(LaneClass::Copy,
        cfg())),
    ];
    for l in &mut lanes {
        l.param_block().push(1u32).unwrap();
        l.synchronize().unwrap();
    }
}
