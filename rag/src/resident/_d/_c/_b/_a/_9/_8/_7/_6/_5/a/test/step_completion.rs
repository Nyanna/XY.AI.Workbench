use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::operation_node::{
    ContinuationHandler, HasOperationKind,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, HasResourceRequirements, ResourceKind, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::operation::OperationKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::b::task_context::TaskId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    CompleteError, Task, TaskParams, TaskState,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::State2;
struct TestOp {
    requirements: ResourceRequirements,
}
impl TestOp {
    fn new() -> Self {
        Self {
            requirements: ResourceRequirements::new(
                ResourceKind::Encoder,
                ExecutionEnvironment::Any,
            ),
        }
    }
}
impl HasResourceRequirements for TestOp {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
impl HasOperationKind for TestOp {
    fn operation_kind(&self) -> OperationKind {
        OperationKind::Embedding
    }
}
type TestTask = Task<String, String, String>;
type Node = OperationNode<TestOp, u32, String, String, String>;
type Payload = StepPayload<u32, TestOp, u32, String, String, String>;
type Context = TaskContext<String>;
struct Fixture {
    task: TestTask,
    context: Arc<Context>,
}
fn fixture(open: usize) -> Fixture {
    let task = Task::new("data".to_string(), TaskParams::default());
    task.claim().unwrap();
    Fixture {
        task,
        context: TaskContext::new(TaskId(1), 0, open),
    }
}
fn node(task: &TestTask, input: u32) -> Node {
    OperationNode::new(TaskId(1), TestOp::new(), input, false, task.clone())
}
fn handler<F>(f: F) -> ContinuationHandler<TestOp, u32, String, String, String>
where
    F: Fn(&String, &TestTask) -> Vec<Node> + Send + Sync + 'static,
{
    Arc::new(f)
}
fn steps(
    fx: &Fixture,
    range: std::ops::Range<u32>,
) -> Vec<PlannedStep<u32, TestOp, u32, String, String, String>> {
    range
        .map(|i| PlannedStep {
            placement: i,
            node: node(&fx.task, i),
        })
        .collect()
}
fn chain(fx: &Fixture, len: u32) -> Payload {
    StepPayload::new(node(&fx.task, 0), Arc::clone(&fx.context), steps(fx, 1..len))
}
struct RecordingSink {
    context: Arc<Context>,
    items: Mutex<Vec<(Option<u32>, Payload)>>,
    open_at_submit: Mutex<Vec<usize>>,
}
impl RecordingSink {
    fn new(context: &Arc<Context>) -> Self {
        Self {
            context: Arc::clone(context),
            items: Mutex::new(Vec::new()),
            open_at_submit: Mutex::new(Vec::new()),
        }
    }
}
impl FollowUpSink<u32, Payload> for RecordingSink {
    fn submit(&self, placement: Option<u32>, payload: Payload) {
        self.open_at_submit.lock().unwrap().push(self.context.open_operations());
        self.items.lock().unwrap().push((placement, payload));
    }
}
struct QueueSink {
    queue: Queue<u32, Payload, (), ()>,
}
impl FollowUpSink<u32, Payload> for QueueSink {
    fn submit(&self, placement: Option<u32>, payload: Payload) {
        self.queue.push(placement.unwrap_or(u32::MAX), payload);
    }
}
#[test]
fn planned_chain_is_released_step_by_step() {
    let fx = fixture(1);
    let sink = QueueSink { queue: Queue::new() };
    sink.submit(Some(0), chain(&fx, 3));
    assert_eq!(sink.queue.len(), 1);
    for i in 0..3u32 {
        let mut claim = sink.queue.take().expect("a runnable step");
        let mut payload = claim.take_payload().unwrap();
        assert_eq!(* payload.node().input(), i);
        let report = payload.finish(format!("out{i}"), &sink).unwrap();
        claim.complete(Ok(()));
        assert_eq!(report.task_finished, i == 2);
        assert_eq!(report.discarded, false);
        assert_eq!(sink.queue.len(), usize::from(i < 2));
    }
    assert_eq!(
        fx.task.result().map(| r | r.ok().cloned()), Some(Some("out2".to_string()))
    );
    assert_eq!(fx.context.open_operations(), 0);
}
#[test]
fn new_with_rest_clears_chain_end_flag() {
    let fx = fixture(1);
    assert!(! chain(& fx, 2).node().ends_chain());
    assert!(chain(& fx, 1).node().ends_chain());
}
#[test]
fn new_with_empty_rest_keeps_the_flag() {
    let fx = fixture(1);
    let kept: Payload = StepPayload::new(
        node(&fx.task, 0).with_ends_chain(false),
        Arc::clone(&fx.context),
        Vec::new(),
    );
    assert!(! kept.node().ends_chain());
}
#[test]
fn replace_rest_returns_old_rest_and_updates_flag() {
    let fx = fixture(1);
    let mut payload = chain(&fx, 3);
    assert_eq!(payload.rest().len(), 2);
    let old = payload.replace_rest(Vec::new());
    assert_eq!(old.len(), 2);
    assert_eq!(old[0].placement, 1);
    assert!(payload.rest().is_empty());
    assert!(payload.node().ends_chain());
    let none = payload.replace_rest(old);
    assert!(none.is_empty());
    assert_eq!(payload.rest().len(), 2);
    assert!(! payload.node().ends_chain());
    assert_eq!(fx.context.open_operations(), 1);
}
#[test]
fn finish_submits_run_time_nodes_after_registering_them() {
    let fx = fixture(1);
    let n = node(&fx.task, 0)
        .with_continuation(
            handler(|_, task| {
                vec![
                    OperationNode::new(TaskId(1), TestOp::new(), 10, false, task
                    .clone()), OperationNode::new(TaskId(1), TestOp::new(), 11, false,
                    task.clone()),
                ]
            }),
        );
    let mut payload = StepPayload::new(n, Arc::clone(&fx.context), Vec::new());
    let sink = RecordingSink::new(&fx.context);
    let report = payload.finish("out".to_string(), &sink).unwrap();
    assert_eq!(report.follow_ups, 2);
    assert!(! report.task_finished);
    assert_eq!(* sink.open_at_submit.lock().unwrap(), vec![3, 3]);
    assert_eq!(fx.context.open_operations(), 2);
    let items = sink.items.lock().unwrap();
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(| (placement, _) | placement.is_none()));
    assert_eq!(* items[0].1.node().input(), 10);
    assert!(! items[0].1.node().ends_chain());
    assert!(items[1].1.node().ends_chain());
    assert!(fx.task.result().is_none());
}
#[test]
fn finish_submits_planned_next_before_run_time_nodes() {
    let fx = fixture(1);
    let n = node(&fx.task, 0)
        .with_continuation(
            handler(|_, task| {
                vec![
                    OperationNode::new(TaskId(1), TestOp::new(), 10, false, task.clone())
                ]
            }),
        );
    let mut payload = StepPayload::new(n, Arc::clone(&fx.context), steps(&fx, 1..3));
    let sink = RecordingSink::new(&fx.context);
    let report = payload.finish("out".to_string(), &sink).unwrap();
    assert_eq!(report.follow_ups, 2);
    assert_eq!(* sink.open_at_submit.lock().unwrap(), vec![3, 3]);
    assert_eq!(fx.context.open_operations(), 2);
    let items = sink.items.lock().unwrap();
    assert_eq!(items[0].0, Some(1));
    assert_eq!(items[0].1.rest().len(), 1);
    assert!(! items[0].1.node().ends_chain());
    assert_eq!(items[1].0, None);
    assert!(! items[1].1.node().ends_chain());
    assert!(payload.rest().is_empty());
}
#[test]
fn finish_on_cancelled_context_discards_the_result() {
    let fx = fixture(1);
    fx.context.fail("boom".to_string());
    let mut payload = chain(&fx, 3);
    let sink = RecordingSink::new(&fx.context);
    let report = payload.finish("out".to_string(), &sink).unwrap();
    assert!(report.discarded);
    assert_eq!(report.follow_ups, 0);
    assert!(payload.node().output().is_none());
    assert!(payload.rest().is_empty());
    assert!(sink.items.lock().unwrap().is_empty());
    assert_eq!(fx.context.open_operations(), 0);
}
#[test]
fn finish_with_output_already_set_changes_nothing() {
    let fx = fixture(1);
    let mut payload = chain(&fx, 3);
    payload.node().set_output("early".to_string()).unwrap();
    let sink = RecordingSink::new(&fx.context);
    assert_eq!(
        payload.finish("late".to_string(), & sink).err(),
        Some(StepError::OutputAlreadySet)
    );
    assert_eq!(payload.rest().len(), 2);
    assert_eq!(fx.context.open_operations(), 1);
    assert!(sink.items.lock().unwrap().is_empty());
}
#[test]
fn finish_with_failing_advance_changes_no_counter() {
    let task: TestTask = Task::new("data".to_string(), TaskParams::default());
    let context = TaskContext::new(TaskId(1), 0, 1);
    let mut payload = StepPayload::new(node(&task, 0), Arc::clone(&context), Vec::new());
    let sink = RecordingSink::new(&context);
    assert_eq!(
        payload.finish("out".to_string(), & sink).err(),
        Some(StepError::Advance(AdvanceError::Complete(CompleteError::NotClaimed(TaskState::Enqueued))))
    );
    assert_eq!(context.open_operations(), 1);
    assert!(sink.items.lock().unwrap().is_empty());
}
#[test]
fn first_fail_completes_the_task_and_drops_the_rest() {
    let fx = fixture(2);
    let mut first = chain(&fx, 3);
    let mut second: Payload = StepPayload::new(
        node(&fx.task, 9),
        Arc::clone(&fx.context),
        Vec::new(),
    );
    let calls = AtomicUsize::new(0);
    let report = first
        .fail(
            "err".to_string(),
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                0
            },
        );
    assert!(! report.task_finished);
    assert!(first.rest().is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(fx.task.state(), TaskState::Completed);
    assert!(fx.task.result().unwrap().is_err());
    assert_eq!(fx.context.open_operations(), 1);
    let report = second.fail("later".to_string(), || 0);
    assert!(! report.task_finished);
    assert_eq!(fx.context.open_operations(), 0);
    assert_eq!(fx.context.first_error().map(String::as_str), Some("err"));
}
#[test]
fn fail_counts_withdrawn_items() {
    let fx = fixture(3);
    let mut payload = chain(&fx, 1);
    let report = payload.fail("err".to_string(), || 2);
    assert!(! report.task_finished);
    assert_eq!(fx.context.open_operations(), 0);
}
#[test]
fn fail_on_completed_task_does_not_panic() {
    let fx = fixture(1);
    fx.task.complete(Ok("done".to_string())).unwrap();
    let mut payload = chain(&fx, 1);
    payload.fail("err".to_string(), || 0);
    assert_eq!(fx.task.state(), TaskState::Completed);
    assert_eq!(fx.context.open_operations(), 0);
}
fn filled_queue() -> (
    Queue<u32, u32, (), ()>,
    Vec<
        crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::queue::Ticket<
            u32,
            u32,
            (),
            (),
        >,
    >,
) {
    let queue = Queue::new();
    let tickets = [1, 2, 1, 3, 1]
        .into_iter()
        .enumerate()
        .map(|(i, meta)| queue.push(meta, i as u32))
        .collect();
    (queue, tickets)
}
#[test]
fn withdraw_pending_removes_only_matching_pending_items() {
    let (queue, tickets) = filled_queue();
    let claimed = queue.take_where(|m| *m == 1).unwrap();
    assert_eq!(withdraw_pending(& queue, | m | * m == 1), 2);
    assert_eq!(tickets[0].state(), State2::Running);
    assert_eq!(tickets[2].state(), State2::Abandoned);
    assert_eq!(tickets[4].state(), State2::Abandoned);
    assert_eq!(queue.len(), 2);
    assert_eq!(withdraw_pending(& queue, | m | * m == 1), 0);
    let mut first = queue.take().unwrap();
    let mut second = queue.take().unwrap();
    assert_eq!(first.take_payload(), Some(1));
    assert_eq!(second.take_payload(), Some(3));
    drop(claimed);
}
#[test]
fn withdraw_planned_reduces_open_operations() {
    let (queue, _tickets) = filled_queue();
    let context: Arc<Context> = TaskContext::new(TaskId(1), 0, 4);
    assert_eq!(withdraw_planned(& queue, | m | * m == 1, & context), 3);
    assert_eq!(context.open_operations(), 1);
    assert_eq!(queue.len(), 2);
}
