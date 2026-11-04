use super::testutil::*;
use super::*;
use crate::resident::op::ResourceKind;

fn queues(has_gpu: bool) -> OpQueues {
    OpQueues::new(Arc::new(Notify::new()), Arc::new(Notify::new()), has_gpu)
}

fn job(run: &Arc<TaskRun>, resource: ResourceKind, engine: EngineSelector) -> OpJob {
    OpJob::new(node(run.ctx(), resource, engine), run.clone())
}

fn any(_: &OpParams) -> bool {
    true
}

#[test]
fn routes_by_engine_and_rejects_missing_gpu() {
    let q = queues(false);
    let (run, _) = run(1);
    run.ctx().add_ops(2);
    q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Either)).unwrap();
    let err = q.push(job(&run, ResourceKind::Encoder, EngineSelector::Gpu)).unwrap_err();
    assert!(matches!(err, OrchestrationError::EngineUnavailable { .. }));
    assert_eq!(q.queued(Side::Cpu), 1);
    assert_eq!(q.queued(Side::Gpu), 0);
}

#[test]
fn own_queue_precedes_shared_and_fifo_holds() {
    let q = queues(true);
    let (run, _) = run(1);
    run.ctx().add_ops(3);
    q.push_shared(job(&run, ResourceKind::Scorer, EngineSelector::Either));
    q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
    q.push(job(&run, ResourceKind::Normalizer, EngineSelector::Cpu)).unwrap();
    let kinds: Vec<_> = (0..3)
        .map(|_| q.claim(Side::Cpu, &any).unwrap().params().requires.resource)
        .collect();
    assert_eq!(kinds, [ResourceKind::Tokenizer, ResourceKind::Normalizer, ResourceKind::Scorer]);
    assert_eq!(q.total_queued(), 0);
}

#[test]
fn foreign_head_does_not_block_shared_queue() {
    let q = queues(true);
    let (run, _) = run(1);
    run.ctx().add_ops(3);
    q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
    q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
    q.push_shared(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu));
    let cpu_only = |p: &OpParams| p.requires.engine == EngineSelector::Cpu;
    let got = q.claim(Side::Cpu, &cpu_only).unwrap();
    assert_eq!(got.params().requires.resource, ResourceKind::Tokenizer);
    assert!(q.claim(Side::Cpu, &cpu_only).is_none());
    assert_eq!(q.total_queued(), 2);
}

#[test]
fn cancelled_task_ops_are_dropped_at_claim() {
    let q = queues(false);
    let (run, _) = run(1);
    run.ctx().add_ops(2);
    q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
    q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
    run.fail(crate::resident::error::PlannerError::UnknownModel { model: "m".into() }.into());
    assert!(q.claim(Side::Cpu, &any).is_none());
    assert_eq!(run.ctx().open_ops(), 0);
}

#[test]
fn last_op_completes_task_once() {
    let (run, state) = run(7);
    run.ctx().add_ops(2);
    run.op_done();
    assert!(state.result().is_none());
    run.op_done();
    assert_eq!(state.result().unwrap().as_ref().unwrap(), &Output::Scores(vec![1.0]));
}

#[test]
fn followups_counted_before_predecessor_closes() {
    let (run, state) = run(7);
    run.ctx().add_ops(1);
    run.ctx().add_ops(2);
    run.op_done();
    assert!(state.result().is_none());
    run.op_done();
    assert!(state.result().is_none());
    run.op_done();
    assert!(state.result().is_some());
}

#[test]
fn first_error_wins_and_completes_exactly_once() {
    let (run, state) = run(7);
    run.ctx().add_ops(3);
    run.fail(crate::resident::error::PlannerError::UnknownModel { model: "a".into() }.into());
    run.fail(crate::resident::error::PlannerError::UnknownModel { model: "b".into() }.into());
    run.skip();
    run.skip();
    run.skip();
    let msg = format!("{:?}", state.result().unwrap());
    assert!(msg.contains("Engine") && msg.contains("\"a\""), "{msg}");
    assert!(run.ctx().is_cancelled());
}
