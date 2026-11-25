use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ExecutionEnvironment;

use super::super::config::default_kernel_names;
use super::super::fixture::MockBackend;

fn caps(values: &[(Axis, Value)]) -> CapabilitySet {
    values.iter().cloned().collect()
}

type Axis = crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::Axis;
type Value = crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::Value;

fn half_and_single() -> CapabilitySet {
    caps(&[(axes::operands(), values::fp32()), (axes::operands(), values::fp16())])
}

fn single_only() -> CapabilitySet {
    caps(&[(axes::operands(), values::fp32())])
}

fn scorer() -> ResourceRequirements {
    ResourceRequirements::new(ResourceKind::Scorer, ExecutionEnvironment::Any)
}

fn policy() -> Fp16FirstPolicy {
    Fp16FirstPolicy::new(DType::F16, default_kernel_names())
}

fn gpu() -> DeviceHandle {
    DeviceHandle::Gpu(0)
}

#[test]
fn fp16_is_the_default_path() {
    let path = policy().select(&scorer(), &half_and_single(), gpu()).unwrap();
    assert_eq!(path.kernel, "score_dot");
    assert_eq!(path.dtype, DType::F16);
    assert_eq!(path.variant, Variant::default());
    assert_eq!(path.lane, LaneKind::Query);
}

#[test]
fn nothing_is_chosen_for_the_cpu_or_for_unknown_kinds() {
    assert!(policy().select(&scorer(), &half_and_single(), DeviceHandle::Cpu).is_none());
    let encoder = ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::Any);
    assert!(policy().select(&encoder, &half_and_single(), gpu()).is_none());
}

#[test]
fn fp32_only_requirements_get_fp32() {
    let req = scorer().with_capability(axes::operands(), values::fp32());
    let path = policy().select(&req, &half_and_single(), gpu()).unwrap();
    assert_eq!(path.dtype, DType::F32);
}

#[test]
fn requirements_that_accept_both_keep_fp16() {
    let req = scorer()
        .with_capability(axes::operands(), values::fp32())
        .with_capability(axes::operands(), values::fp16());
    let path = policy().select(&req, &half_and_single(), gpu()).unwrap();
    assert_eq!(path.dtype, DType::F16);
    let req = scorer().with_capability(axes::operands(), values::fp16());
    let path = policy().select(&req, &half_and_single(), gpu()).unwrap();
    assert_eq!(path.dtype, DType::F16);
}

#[test]
fn without_fp16_the_path_falls_back_to_fp32() {
    let path = policy().select(&scorer(), &single_only(), gpu()).unwrap();
    assert_eq!(path.dtype, DType::F32);
}

#[test]
fn without_any_float_operand_there_is_no_path() {
    assert!(policy().select(&scorer(), &CapabilitySet::new(), gpu()).is_none());
    let int8 = caps(&[(axes::scoring(), values::int8_dp4a())]);
    assert!(policy().select(&scorer(), &int8, gpu()).is_none());
}

#[test]
fn a_configured_fp32_default_is_honoured() {
    let policy = Fp16FirstPolicy::new(DType::F32, default_kernel_names());
    let path = policy.select(&scorer(), &half_and_single(), gpu()).unwrap();
    assert_eq!(path.dtype, DType::F32);
}

#[test]
fn bf16_is_never_chosen() {
    let policy = Fp16FirstPolicy::new(DType::BF16, default_kernel_names());
    for caps in [half_and_single(), single_only()] {
        let path = policy.select(&scorer(), &caps, gpu()).unwrap();
        assert_eq!(path.dtype, DType::F32);
    }
}

#[test]
fn the_kernel_name_comes_from_the_map() {
    let names = std::collections::HashMap::from([
        (ResourceKind::Scorer, "other_kernel".to_string()),
        (ResourceKind::Encoder, "encode".to_string()),
    ]);
    let policy = Fp16FirstPolicy::new(DType::F16, names);
    let path = policy.select(&scorer(), &half_and_single(), gpu()).unwrap();
    assert_eq!(path.kernel, "other_kernel");
    let encoder = ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::Any);
    assert_eq!(policy.select(&encoder, &half_and_single(), gpu()).unwrap().kernel, "encode");
}

#[test]
fn lane_factory_creates_lanes_with_a_stream_of_the_device() {
    let backend = MockBackend::single();
    let factory = GpuLaneFactory::new(Arc::clone(&backend) as Arc<dyn GpuBackend>, 0);
    for kind in [LaneKind::Query, LaneKind::Insert, LaneKind::CopyH2D] {
        let cfg = LaneConfig::preset(kind, DeviceHandle::Gpu(0));
        let lane = factory.create(&cfg).unwrap();
        assert_eq!(lane.kind(), kind);
        assert_eq!(lane.device(), DeviceHandle::Gpu(0));
        assert!(lane.stream().is_some());
    }
    let streams = lock(&backend.streams).clone();
    assert_eq!(
        streams,
        vec![(0, Priority::High), (0, Priority::Normal), (0, Priority::Normal)]
    );
}

#[test]
fn lane_factory_reports_stream_errors() {
    let backend = MockBackend::single();
    backend.streams_before_failure.store(0, std::sync::atomic::Ordering::SeqCst);
    let factory = GpuLaneFactory::new(Arc::clone(&backend) as Arc<dyn GpuBackend>, 0);
    let cfg = LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0));
    assert!(factory.create(&cfg).is_err());
}
