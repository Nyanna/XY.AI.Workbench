use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use std::collections::{BTreeMap, HashMap};
static_assertions::assert_impl_all!(ResourceRequirements : Send, Sync);
static_assertions::assert_impl_all!(ResourceKind : Send, Sync);
static_assertions::assert_impl_all!(ExecutionEnvironment : Send, Sync);
static_assertions::assert_impl_all!(ModelId : Send, Sync);
static_assertions::assert_impl_all!(IndexId : Send, Sync);
static_assertions::assert_impl_all!(Affinity : Send, Sync);
fn base() -> ResourceRequirements {
    ResourceRequirements::new(ResourceKind::Encoder, ExecutionEnvironment::Any)
}
#[test]
fn new_has_defaults() {
    let r = base();
    assert!(r.model().is_none() && r.index().is_none() && r.affinity().is_none());
    assert!(r.capabilities().is_empty());
}
#[test]
fn environment_accepts() {
    assert!(ExecutionEnvironment::Any.accepts(DeviceKind::Cpu));
    assert!(ExecutionEnvironment::Any.accepts(DeviceKind::Gpu));
    assert!(ExecutionEnvironment::CpuOnly.accepts(DeviceKind::Cpu));
    assert!(! ExecutionEnvironment::CpuOnly.accepts(DeviceKind::Gpu));
    assert!(ExecutionEnvironment::GpuOnly.accepts(DeviceKind::Gpu));
    assert!(! ExecutionEnvironment::GpuOnly.accepts(DeviceKind::Cpu));
}
#[test]
fn satisfiability_ignores_affinity() {
    let a = base().with_capability("a", "x");
    let b = a.clone().with_affinity(Affinity::Device(DeviceHandle::Gpu(1)));
    let dev = CapabilitySet::new().with("a", "x");
    assert!(a.is_satisfied_by(DeviceKind::Cpu, & dev));
    assert!(b.is_satisfied_by(DeviceKind::Cpu, & dev));
    assert!(! a.is_satisfied_by(DeviceKind::Cpu, & CapabilitySet::new()));
    assert!(base().is_satisfied_by(DeviceKind::Gpu, & CapabilitySet::new()));
}
#[test]
fn builder_order_and_merge() {
    let x = base().with_capability("a", "1").with_capability("b", "2");
    let y = base()
        .with_capabilities(CapabilitySet::new().with("b", "2"))
        .with_capability("a", "1");
    assert_eq!(x, y);
    assert_eq!(x.cmp(& y), Ordering::Equal);
    let z = x.clone().with_model(ModelId::new("m1")).with_model(ModelId::new("m2"));
    assert_eq!(z.model().unwrap().as_str(), "m2");
}
#[test]
fn ordering_rules() {
    let none = base();
    let some = base().with_model(ModelId::from("m"));
    assert!(none < some);
    let r = Affinity::Region(
        RegionHandle::new(ArenaKind::Weights, 0, 1, Generation(0)).unwrap(),
    );
    let c = Affinity::Device(DeviceHandle::Cpu);
    let g0 = Affinity::Device(DeviceHandle::Gpu(0));
    let g1 = Affinity::Device(DeviceHandle::Gpu(1));
    assert!(r < c && c < g0 && g0 < g1);
    assert!(base() < base().with_affinity(c));
    assert!(
        ResourceRequirements::new(ResourceKind::Tokenizer, ExecutionEnvironment::Any) <
        ResourceRequirements::new(ResourceKind::Index, ExecutionEnvironment::CpuOnly)
    );
}
#[test]
fn usable_as_map_key() {
    let k = base().with_index(IndexId::new("i"));
    let mut h = HashMap::new();
    h.insert(k.clone(), 1);
    let mut b = BTreeMap::new();
    b.insert(k.clone(), 2);
    assert_eq!(h[& k], 1);
    assert_eq!(b[& k], 2);
    assert_eq!(format!("{}", IndexId::from(String::from("i"))), "i");
}
struct Op(ResourceRequirements);
impl HasResourceRequirements for Op {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.0
    }
}
#[test]
fn accessor_trait() {
    let op = Op(base());
    assert_eq!(op.resource_requirements(), op.resource_requirements());
}
