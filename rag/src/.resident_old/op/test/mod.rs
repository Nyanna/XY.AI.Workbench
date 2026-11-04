use super::*;
use crate::resident::error::PlannerError;
use crate::resident::memory::region::ArenaId;
use crate::resident::memory::Region;
fn desc(kind: OpKind) -> Arc<dyn Op> {
    let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Either);
    Arc::new(OpDesc::new(kind, OpParams::new(req)))
}
fn err() -> ResidentError {
    PlannerError::InvalidConfig {
        reason: "x".into(),
    }
        .into()
}
#[test]
fn params_clone_hash_eq() {
    use std::collections::HashSet;
    let p = OpParams::new(
        Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu),
    );
    let mut set = HashSet::new();
    set.insert(p.clone());
    assert!(set.contains(& p));
    let mut q = p.clone();
    q.top_k = Some(5);
    assert_ne!(p, q);
}
#[test]
fn requires_comes_from_params() {
    let op = desc(OpKind::Embed);
    assert_eq!(op.requires(), op.params().requires);
}
#[test]
fn task_done_only_at_zero() {
    let ctx = TaskCtx::new(1, 0, 3);
    assert!(! ctx.finish_op());
    assert!(! ctx.finish_op());
    assert!(ctx.finish_op());
    assert_eq!(ctx.open_ops(), 0);
}
#[test]
fn first_error_cancels_and_is_kept() {
    let ctx = TaskCtx::new(1, 0, 2);
    assert!(ctx.fail(err()));
    assert!(! ctx.fail(err()));
    assert!(ctx.is_cancelled());
    assert!(ctx.error().is_some());
    ctx.finish_op();
    assert!(! ctx.finish_op());
}
#[test]
fn output_set_once_and_device_value_holds_lease() {
    let ctx = TaskCtx::new(1, 0, 1);
    let node = OperationNode::new(
        desc(OpKind::Tokenize),
        Value::Host(HostData::Text(vec![])),
        true,
        ctx,
    );
    assert!(node.set_output(Value::Host(HostData::F32(vec![1.0]))).is_ok());
    assert!(node.set_output(Value::Host(HostData::F32(vec![]))).is_err());
    let pin = PinCount::new();
    let region = Region {
        arena: ArenaId(0),
        offset: 0,
        len: 8,
        gen: 1,
    };
    let v = Value::Device(Lease::new(region, pin.clone(), None));
    assert_eq!(pin.get(), 1);
    drop(v);
    assert!(pin.is_unpinned());
}
#[test]
fn affinity_is_non_exhaustive() {
    let a = Affinity::Device(DeviceId(0));
    #[allow(unreachable_patterns)]
    let _ = match a {
        Affinity::Handle(_) | Affinity::Device(_) => {}
        _ => {}
    };
}
