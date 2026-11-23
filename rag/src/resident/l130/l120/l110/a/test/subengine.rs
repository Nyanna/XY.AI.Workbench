use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;
use crate::resident::l130::l120::l110::l100::l090::a::gpu_completion::TakenVia;
use crate::resident::l130::l120::l110::l100::a::plan_contract::QueueKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    ModelId, ResourceKind, ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::CapabilitySet;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    DeviceHandle, Event, Lane, LaneConfig, LaneKind,
};
use crate::resident::l130::l120::l110::l100::l090::l080::subengine_loop::IterationOutcome;
fn assert_send_sync<T: Send + Sync>() {}
struct StubSubengine {
    name: &'static str,
    starts: AtomicUsize,
}
impl Subengine for StubSubengine {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn start(&self) -> Result<(), String> {
        self.starts.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn capabilities(&self) -> Vec<(QueueKind, CapabilitySet)> {
        Vec::new()
    }
    fn offers(&self) -> Vec<ResourceKind> {
        Vec::new()
    }
    fn announce(&self, _req: &ResourceRequirements) {}
    fn activate(&self) -> Result<(), String> {
        Ok(())
    }
    fn is_loaded(&self, _kind: ResourceKind, _model: Option<&ModelId>) -> bool {
        false
    }
}
fn stub(name: &'static str) -> Arc<StubSubengine> {
    Arc::new(StubSubengine {
        name,
        starts: AtomicUsize::new(0),
    })
}
struct CountingFeed {
    pulls: Arc<AtomicUsize>,
}
impl FeedStep for CountingFeed {
    fn feed(&mut self, _now: Instant) -> IterationOutcome {
        let n = self.pulls.fetch_add(1, Ordering::SeqCst) + 1;
        IterationOutcome {
            claimed: n,
            ..IterationOutcome::default()
        }
    }
}
fn lane_with_tag(tag: u64) -> FinishedLane {
    FinishedLane {
        lane: Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu), None),
        event: Event::mock().0,
        tag,
        result: Ok(()),
        via: TakenVia::Callback,
    }
}
struct Fixture {
    handle: SubengineHandle,
    builds: Arc<AtomicUsize>,
    pulls: Arc<AtomicUsize>,
    handled: Arc<std::sync::Mutex<Vec<u64>>>,
}
fn fixture(device: DeviceKind, owned_tag: u64) -> Fixture {
    let builds = Arc::new(AtomicUsize::new(0));
    let pulls = Arc::new(AtomicUsize::new(0));
    let handled = Arc::new(std::sync::Mutex::new(Vec::new()));
    let (b, p, h) = (builds.clone(), pulls.clone(), handled.clone());
    let feeds: FeedFactory = Arc::new(move || {
        b.fetch_add(1, Ordering::SeqCst);
        vec![Box::new(CountingFeed { pulls : p.clone() }) as Box < dyn FeedStep >]
    });
    let router: FinishedRouter = Arc::new(move |lane: FinishedLane| {
        if lane.tag == owned_tag {
            h.lock().unwrap().push(lane.tag);
            Ok(())
        } else {
            Err(lane)
        }
    });
    Fixture {
        handle: SubengineHandle::new(stub("stub"), device, feeds, router),
        builds,
        pulls,
        handled,
    }
}
#[test]
fn handle_is_send_and_sync() {
    assert_send_sync::<SubengineHandle>();
}
#[test]
fn exposes_device_kind() {
    assert_eq!(fixture(DeviceKind::Gpu, 1).handle.device(), DeviceKind::Gpu);
    assert_eq!(fixture(DeviceKind::Cpu, 1).handle.device(), DeviceKind::Cpu);
}
#[test]
fn subengine_is_the_wrapped_instance_and_forwarded_unchanged() {
    let inner = stub("wrapped");
    let dynamic: Arc<dyn Subengine> = inner.clone();
    let feeds: FeedFactory = Arc::new(Vec::new);
    let router: FinishedRouter = Arc::new(Err);
    let handle = SubengineHandle::new(dynamic.clone(), DeviceKind::Cpu, feeds, router);
    assert!(Arc::ptr_eq(handle.subengine(), & dynamic));
    assert_eq!(handle.subengine().name(), "wrapped");
    handle.subengine().start().unwrap();
    assert_eq!(inner.starts.load(Ordering::SeqCst), 1);
}
#[test]
fn feeds_for_worker_builds_a_fresh_set_per_call() {
    let fx = fixture(DeviceKind::Gpu, 1);
    assert_eq!(fx.builds.load(Ordering::SeqCst), 0);
    let first = fx.handle.feeds_for_worker();
    let second = fx.handle.feeds_for_worker();
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(fx.builds.load(Ordering::SeqCst), 2);
}
#[test]
fn feeds_of_different_workers_share_the_same_underlying_state() {
    let fx = fixture(DeviceKind::Cpu, 1);
    let mut a = fx.handle.feeds_for_worker();
    let mut b = fx.handle.feeds_for_worker();
    let now = Instant::now();
    assert_eq!(a[0].feed(now).claimed, 1);
    assert_eq!(b[0].feed(now).claimed, 2);
    assert_eq!(a[0].feed(now).claimed, 3);
    assert_eq!(fx.pulls.load(Ordering::SeqCst), 3);
}
#[test]
fn empty_feed_set_is_allowed() {
    let feeds: FeedFactory = Arc::new(Vec::new);
    let router: FinishedRouter = Arc::new(Err);
    let handle = SubengineHandle::new(stub("s"), DeviceKind::Cpu, feeds, router);
    assert!(handle.feeds_for_worker().is_empty());
}
#[test]
fn owned_lane_is_handled() {
    let fx = fixture(DeviceKind::Gpu, 7);
    assert!(fx.handle.on_finished(lane_with_tag(7)).is_ok());
    assert_eq!(* fx.handled.lock().unwrap(), vec![7]);
}
#[test]
fn foreign_lane_is_returned_unchanged() {
    let fx = fixture(DeviceKind::Gpu, 7);
    let back = fx.handle.on_finished(lane_with_tag(9)).err().unwrap();
    assert_eq!(back.tag, 9);
    assert_eq!(back.via, TakenVia::Callback);
    assert!(back.result.is_ok());
    assert!(fx.handled.lock().unwrap().is_empty());
}
#[test]
fn lane_is_offered_to_the_next_handle_after_rejection() {
    let first = fixture(DeviceKind::Gpu, 1);
    let second = fixture(DeviceKind::Gpu, 2);
    let lane = first.handle.on_finished(lane_with_tag(2)).err().unwrap();
    assert!(second.handle.on_finished(lane).is_ok());
    assert_eq!(* second.handled.lock().unwrap(), vec![2]);
}
#[test]
fn usable_concurrently_from_many_threads() {
    let fx = Arc::new(fixture(DeviceKind::Gpu, 5));
    let joins: Vec<_> = (0..4)
        .map(|_| {
            let fx = fx.clone();
            thread::spawn(move || {
                let mut feeds = fx.handle.feeds_for_worker();
                feeds[0].feed(Instant::now());
                assert!(fx.handle.on_finished(lane_with_tag(5)).is_ok());
                assert!(fx.handle.on_finished(lane_with_tag(6)).is_err());
            })
        })
        .collect();
    for j in joins {
        j.join().unwrap();
    }
    assert_eq!(fx.builds.load(Ordering::SeqCst), 4);
    assert_eq!(fx.pulls.load(Ordering::SeqCst), 4);
    assert_eq!(fx.handled.lock().unwrap().len(), 4);
}
