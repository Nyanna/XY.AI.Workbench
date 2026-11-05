use super::*;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct FakeGpuEvent {
    ready: AtomicBool,
    fail: AtomicBool,
    queries: Arc<AtomicUsize>,
}

impl GpuEventBackend for FakeGpuEvent {
    fn query(&self) -> Result<bool, String> {
        self.queries.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            return Err("boom".into());
        }
        Ok(self.ready.load(Ordering::SeqCst))
    }
    fn synchronize(&self) -> Result<(), String> {
        if self.fail.load(Ordering::SeqCst) {
            Err("sync boom".into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct FakeStream {
    fail: bool,
}

impl StreamBackend for FakeStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        if self.fail {
            return Err("no record".into());
        }
        Ok(Box::new(FakeGpuEvent {
            ready: AtomicBool::new(true),
            fail: AtomicBool::new(false),
            queries: Arc::new(AtomicUsize::new(0)),
        }))
    }
}

#[test]
fn device_handle_basics() {
    assert_eq!(DeviceHandle::Cpu.kind(), DeviceKind::Cpu);
    assert_eq!(DeviceHandle::Gpu(2).kind(), DeviceKind::Gpu);
    assert_eq!(DeviceHandle::Gpu(2).ordinal(), Some(2));
    assert_eq!(DeviceHandle::Cpu.ordinal(), None);
    assert_ne!(DeviceHandle::Gpu(0), DeviceHandle::Cpu);
    assert_eq!(DeviceHandle::Cpu.to_string(), "cpu");
    assert_eq!(DeviceHandle::Gpu(1).to_string(), "gpu:1");
}

#[test]
fn event_error_display() {
    let e = EventError {
        device: DeviceHandle::Gpu(3),
        cause: "x".into(),
    };
    assert_eq!(e.to_string(), "event error on gpu:3: x");
}

#[test]
fn cpu_event_signal_and_wait() {
    let (ev, sig) = Event::cpu();
    assert_eq!(ev.is_complete(), Ok(false));
    assert_eq!(ev.to_string(), "Event(cpu, pending)");
    let waiters: Vec<_> = (0..3)
        .map(|_| {
            let e = ev.clone();
            thread::spawn(move || e.wait())
        })
        .collect();
    thread::sleep(Duration::from_millis(20));
    sig.signal();
    sig.signal();
    for w in waiters {
        assert_eq!(w.join().unwrap(), Ok(()));
    }
    assert_eq!(ev.is_complete(), Ok(true));
    assert_eq!(ev.wait(), Ok(()));
    assert_eq!(ev.to_string(), "Event(cpu, complete)");
}

#[test]
fn gpu_event_query_caches_complete() {
    let queries = Arc::new(AtomicUsize::new(0));
    let backend = FakeGpuEvent {
        ready: AtomicBool::new(true),
        fail: AtomicBool::new(false),
        queries: queries.clone(),
    };
    let ev = Event::gpu(1, Box::new(backend));
    assert_eq!(ev.device(), DeviceHandle::Gpu(1));
    assert_eq!(ev.is_complete(), Ok(true));
    assert_eq!(ev.is_complete(), Ok(true));
    assert_eq!(queries.load(Ordering::SeqCst), 1);
    assert_eq!(ev.to_string(), "Event(gpu:1, complete)");
}

#[test]
fn gpu_event_errors_not_cached() {
    let backend = FakeGpuEvent {
        ready: AtomicBool::new(true),
        fail: AtomicBool::new(true),
        queries: Arc::new(AtomicUsize::new(0)),
    };
    let ev = Event::gpu(0, Box::new(backend));
    let err = ev.is_complete().unwrap_err();
    assert_eq!(err.device, DeviceHandle::Gpu(0));
    assert_eq!(err.cause, "boom");
    assert_eq!(ev.to_string(), "Event(gpu:0, failed: boom)");
    assert_eq!(ev.wait().unwrap_err().cause, "sync boom");
    assert!(ev.is_complete().is_err());
}

#[test]
fn mock_event_lifecycle() {
    let (ev, ctl) = Event::mock();
    assert_eq!(ev.is_complete(), Ok(false));
    let clone = ev.clone();
    let h = thread::spawn(move || clone.wait());
    thread::sleep(Duration::from_millis(20));
    ctl.complete();
    assert_eq!(h.join().unwrap(), Ok(()));
    assert_eq!(ev.is_complete(), Ok(true));
    ctl.reset();
    assert_eq!(ev.is_complete(), Ok(false));
    ctl.fail("bad");
    let err = ev.wait().unwrap_err();
    assert_eq!(err.device, DeviceHandle::Cpu);
    assert_eq!(err.cause, "bad");
    assert_eq!(ev.to_string(), "Event(cpu, failed: bad)");
}

#[test]
fn mock_fail_wakes_waiters() {
    let (ev, ctl) = Event::mock();
    let e = ev.clone();
    let h = thread::spawn(move || e.wait());
    thread::sleep(Duration::from_millis(20));
    ctl.fail("late");
    assert_eq!(h.join().unwrap().unwrap_err().cause, "late");
}

#[test]
fn lane_presets_and_buffers() {
    let q = LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu);
    assert_eq!(q.priority, Priority::High);
    assert_eq!(q.param_block_size, 256);
    assert_eq!(q.workspace_size, 0);
    for k in [
        LaneKind::Insert,
        LaneKind::Update,
        LaneKind::CopyH2D,
        LaneKind::CopyD2H,
    ] {
        assert_eq!(LaneConfig::preset(k, DeviceHandle::Cpu).priority, Priority::Normal);
    }
    let mut lane = Lane::new(q, None);
    assert_eq!(lane.params().len(), 256);
    assert!(lane.params().iter().all(|b| *b == 0));
    lane.params_mut()[0] = 7;
    lane.ensure_workspace(8);
    lane.workspace_mut()[3] = 9;
    lane.ensure_workspace(4);
    assert_eq!(lane.workspace().len(), 8);
    lane.ensure_workspace(16);
    assert_eq!(lane.workspace()[3], 9);
    assert_eq!(lane.workspace()[15], 0);
    assert_eq!(lane.params().len(), 256);
    assert!(lane.stream().is_none());
    assert_eq!(lane.kind(), LaneKind::Query);
    assert_eq!(lane.device(), DeviceHandle::Cpu);
}

#[test]
fn lane_record_event() {
    let cpu_lane = Lane::new(LaneConfig::preset(LaneKind::Insert, DeviceHandle::Cpu), None);
    assert_eq!(cpu_lane.record_event().unwrap_err(), LaneError::NoStreamBackend);
    assert!(cpu_lane.last_event().is_none());

    let cfg = LaneConfig::preset(LaneKind::CopyH2D, DeviceHandle::Gpu(2));
    let lane = Lane::new(cfg.clone(), Some(Box::new(FakeStream { fail: false })));
    let ev = lane.record_event().unwrap();
    assert_eq!(ev.device(), DeviceHandle::Gpu(2));
    assert!(lane.last_event().is_some());

    let bad = Lane::new(cfg, Some(Box::new(FakeStream { fail: true })));
    match bad.record_event().unwrap_err() {
        LaneError::Record(e) => {
            assert_eq!(e.device, DeviceHandle::Gpu(2));
            assert_eq!(e.cause, "no record");
        }
        other => panic!("unexpected {other:?}"),
    }
}
