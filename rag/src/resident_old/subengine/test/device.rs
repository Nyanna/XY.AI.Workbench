use super::*;
use crate::resident::memory::{ArenaId, Region};
fn cfg() -> LaneConfig {
    LaneConfig::new(Region {
        arena: ArenaId(0),
        offset: 0,
        len: 1024,
        gen: 0,
    })
}
#[test]
fn cpu_handle_opens_cpu_lanes() {
    let dev = DeviceHandle::cpu();
    assert_eq!(dev.kind(), DeviceKind::Cpu);
    assert_eq!(dev.init_cost(), Duration::ZERO);
    let lane = dev.open_lane(LaneClass::Query, cfg()).unwrap();
    assert_eq!(lane.device(), DeviceKind::Cpu);
    assert_eq!(lane.class(), LaneClass::Query);
    lane.synchronize().unwrap();
    dev.warmup().unwrap();
}
#[cfg(not(feature = "cuda"))]
#[test]
fn cuda_handle_unavailable_without_feature() {
    assert!(matches!(DeviceHandle::cuda(0), Err(DeviceError::CudaUnavailable)));
}
#[cfg(feature = "cuda")]
#[test]
fn project_cudarc_is_the_one_candle_uses() {
    use std::any::TypeId;
    assert_eq!(
        TypeId::of::< candle_core::cuda_backend::cudarc::driver::CudaContext > (),
        TypeId::of::< cudarc::driver::CudaContext > ()
    );
}
#[cfg(feature = "cuda")]
#[test]
#[ignore = "requires a CUDA device"]
fn cuda_lanes_order_work_through_events() {
    let dev = DeviceHandle::cuda(0).unwrap();
    dev.warmup().unwrap();
    let producer = dev.open_lane(LaneClass::Copy, cfg()).unwrap();
    let consumer = dev.open_lane(LaneClass::Query, cfg()).unwrap();
    let event = producer.record_event().unwrap();
    consumer.wait(&event).unwrap();
    consumer.synchronize().unwrap();
    assert!(event.is_complete());
    assert_eq!(consumer.device(), DeviceKind::Gpu);
}
