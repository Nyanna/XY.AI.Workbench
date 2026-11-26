use std::collections::HashSet;

use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, ModelId, ResourceKind,
};

fn reqs() -> ResourceRequirements {
    ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::GpuOnly)
        .with_model(ModelId::new("m"))
}

#[test]
fn params_defaults_and_builders() {
    let p = OperationParams::new(reqs());
    assert_eq!(p.top_k(), None);
    assert_eq!(p.dimension(), None);
    assert_eq!(p.mode(), None);
    assert_ne!(p, p.clone().with_top_k(3));
    assert_ne!(p.clone().with_top_k(3), p.clone().with_top_k(4));
    assert_eq!(p.clone().with_dimension(8).dimension(), Some(8));
    assert_eq!(p.requirements(), &reqs());
}

#[test]
fn mode_distinguishes_params() {
    let p = OperationParams::new(reqs());
    let query = p.clone().with_mode(EmbeddingMode::Query);
    let passage = p.clone().with_mode(EmbeddingMode::Passage);
    assert_eq!(query.mode(), Some(EmbeddingMode::Query));
    assert_ne!(query, passage);
    assert_ne!(query, p);
}

#[test]
fn descriptor_is_key() {
    let a = OperationDescriptor::new(OperationKind::TopK, OperationParams::new(reqs()));
    let b = OperationDescriptor::new(OperationKind::TopK, OperationParams::new(reqs()));
    let c = OperationDescriptor::new(OperationKind::Pooling, OperationParams::new(reqs()));
    assert_eq!(a, b);
    assert_ne!(a, c);
    let mut set = HashSet::new();
    set.insert(a.clone());
    assert!(set.contains(&b));
    assert!(!set.contains(&c));
    assert_eq!(a.requirements(), a.params().requirements());
}

#[test]
fn ram_data_shapes() {
    let hits = RamData::HitList(vec![(EntryId(1), 0.5)]);
    assert_eq!(hits.shape(), DataShape::HitList);
    let result = RamData::ScoredHitResult(ScoredHitResult {
        scored_count: 1,
        dropped_count: 0,
        hits: vec![],
    });
    assert_eq!(result.shape(), DataShape::ScoredHitResult);
}

#[test]
fn send_sync() {
    fn check<T: Send + Sync>() {}
    check::<OperationDescriptor>();
    check::<OperationParams>();
    check::<RamData>();
}
