use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{
    ExecutionEnvironment, ResourceKind, ResourceRequirements,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::TaskParams;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::QueueState;
struct TestOp {
    requirements: ResourceRequirements,
}
impl TestOp {
    fn new(kind: ResourceKind) -> Self {
        Self {
            requirements: ResourceRequirements::new(kind, ExecutionEnvironment::Any),
        }
    }
}
impl HasResourceRequirements for TestOp {
    fn resource_requirements(&self) -> &ResourceRequirements {
        &self.requirements
    }
}
impl OperationKind for TestOp {
    fn operation_kind(&self) -> &'static str {
        "test-op"
    }
}
type Node = OperationNode<TestOp, u32, String, String, String>;
type TestTask = Task<String, String, String>;
fn new_task() -> TestTask {
    Task::new("data".to_string(), TaskParams::default())
}
fn claimed_task() -> TestTask {
    let task = new_task();
    task.claim().unwrap();
    task
}
fn node_with(task: TestTask) -> Node {
    OperationNode::new(7, TestOp::new(ResourceKind::Encoder), 42, true, task)
}
fn handler<F>(f: F) -> ContinuationHandler<TestOp, u32, String, String, String>
where
    F: Fn(&String, &TestTask) -> Vec<Node> + Send + Sync + 'static,
{
    Arc::new(f)
}
fn result_of(task: &TestTask) -> Option<Result<String, String>> {
    task.result()
        .map(|r| match r {
            Ok(v) => Ok(v.clone()),
            Err(_) => Err("error".to_string()),
        })
}
#[test]
fn new_node_has_given_parts_and_empty_output() {
    let node = node_with(new_task());
    assert_eq!(* node.input(), 42);
    assert!(node.materialize());
    assert_eq!(node.operation().operation_kind(), "test-op");
    assert!(node.output().is_none());
    assert!(! node.has_continuation());
    assert_eq!(node.task().state(), QueueState::Enqueued);
}
#[test]
fn set_output_is_write_once() {
    let node = node_with(new_task());
    assert_eq!(node.set_output("first".to_string()), Ok(()));
    assert_eq!(node.set_output("second".to_string()), Err("second".to_string()));
    assert_eq!(node.output().map(String::as_str), Some("first"));
    assert!(node.task().result().is_none());
}
#[test]
fn concurrent_set_output_has_single_winner() {
    let node = Arc::new(node_with(new_task()));
    let wins = Arc::new(AtomicUsize::new(0));
    let threads: Vec<_> = (0..8)
        .map(|i| {
            let node = Arc::clone(&node);
            let wins = Arc::clone(&wins);
            std::thread::spawn(move || {
                let value = format!("v{i}");
                match node.set_output(value.clone()) {
                    Ok(()) => {
                        wins.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(back) => assert_eq!(back, value),
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    assert_eq!(wins.load(Ordering::SeqCst), 1);
    assert!(node.output().is_some());
}
#[test]
fn advance_without_output_fails_without_side_effects() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let node = node_with(claimed_task())
        .with_continuation(
            handler(move |_, _| {
                counter.fetch_add(1, Ordering::SeqCst);
                Vec::new()
            }),
        );
    assert_eq!(node.advance().err(), Some(AdvanceError::OutputNotSet));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(node.task().state(), QueueState::Claimed);
}
#[test]
fn advance_without_handler_completes_task() {
    let node = node_with(claimed_task());
    node.set_output("out".to_string()).unwrap();
    assert!(node.advance().unwrap().is_empty());
    assert_eq!(result_of(node.task()), Some(Ok("out".to_string())));
}
#[test]
fn advance_with_empty_handler_result_completes_task() {
    let node = node_with(claimed_task()).with_continuation(handler(|_, _| Vec::new()));
    node.set_output("out".to_string()).unwrap();
    assert!(node.advance().unwrap().is_empty());
    assert_eq!(result_of(node.task()), Some(Ok("out".to_string())));
}
#[test]
fn advance_with_follow_up_nodes_does_not_complete_task() {
    let node = node_with(claimed_task())
        .with_continuation(
            handler(|out, task| {
                assert_eq!(out, "out");
                vec![
                    OperationNode::new(8, TestOp::new(ResourceKind::Scorer), 1, false,
                    task.clone()), OperationNode::new(9,
                    TestOp::new(ResourceKind::Index), 2, true, task.clone()),
                ]
            }),
        );
    node.set_output("out".to_string()).unwrap();
    let next = node.advance().unwrap();
    assert_eq!(next.len(), 2);
    assert_eq!(* next[0].input(), 1);
    assert!(! next[0].materialize());
    assert_eq!(* next[1].input(), 2);
    assert!(next[1].output().is_none());
    assert_eq!(node.task().state(), QueueState::Claimed);
    assert!(node.task().result().is_none());
}
#[test]
fn advance_chain_end_on_unclaimed_task_reports_error() {
    let node = node_with(new_task());
    node.set_output("out".to_string()).unwrap();
    assert_eq!(
        node.advance().err(),
        Some(AdvanceError::Complete(CompleteError::NotClaimed(QueueState::Enqueued)))
    );
}
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "completed more than once")]
fn repeated_chain_end_panics_in_debug_builds() {
    let node = node_with(claimed_task());
    node.set_output("out".to_string()).unwrap();
    node.advance().unwrap();
    let _ = node.advance();
}
#[cfg(not(debug_assertions))]
#[test]
fn repeated_chain_end_reports_already_completed() {
    let node = node_with(claimed_task());
    node.set_output("out".to_string()).unwrap();
    node.advance().unwrap();
    assert_eq!(
        node.advance().err(),
        Some(AdvanceError::Complete(CompleteError::NotClaimed(QueueState::Completed)))
    );
}
#[test]
fn advance_invokes_handler_on_every_call() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let node = node_with(claimed_task())
        .with_continuation(
            handler(move |_, task| {
                counter.fetch_add(1, Ordering::SeqCst);
                vec![
                    OperationNode::new(1, TestOp::new(ResourceKind::Tokenizer), 0, false,
                    task.clone(),)
                ]
            }),
        );
    node.set_output("out".to_string()).unwrap();
    assert_eq!(node.advance().unwrap().len(), 1);
    assert_eq!(node.advance().unwrap().len(), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
#[test]
fn latest_handler_wins() {
    let mut node = node_with(claimed_task());
    node.set_continuation(
        handler(|_, task| {
            vec![
                OperationNode::new(1, TestOp::new(ResourceKind::Tokenizer), 1, false,
                task.clone(),)
            ]
        }),
    );
    assert!(node.has_continuation());
    node.set_continuation(
        handler(|_, task| {
            vec![
                OperationNode::new(2, TestOp::new(ResourceKind::Tokenizer), 2, false,
                task.clone(),)
            ]
        }),
    );
    node.set_output("out".to_string()).unwrap();
    let next = node.advance().unwrap();
    assert_eq!(* next[0].input(), 2);
}
#[test]
fn shared_handler_can_serve_several_nodes_concurrently() {
    let shared = handler(|out, task| {
        vec![
            OperationNode::new(1, TestOp::new(ResourceKind::Scorer), out.len() as u32,
            false, task.clone(),)
        ]
    });
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let shared = Arc::clone(&shared);
            std::thread::spawn(move || {
                let node = node_with(claimed_task()).with_continuation(shared);
                node.set_output("abc".to_string()).unwrap();
                let next = node.advance().unwrap();
                assert_eq!(* next[0].input(), 3);
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
#[test]
fn node_is_send_and_sync() {
    fn assert_send_sync<X: Send + Sync>() {}
    assert_send_sync::<Node>();
}
struct CaptureLogger {
    records: Mutex<Vec<(log::Level, String)>>,
}
impl log::Log for CaptureLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        self.records.lock().unwrap().push((record.level(), record.args().to_string()));
    }
    fn flush(&self) {}
}
static LOGGER: CaptureLogger = CaptureLogger {
    records: Mutex::new(Vec::new()),
};
#[test]
fn new_logs_one_debug_record() {
    if log::set_logger(&LOGGER).is_err() {
        return;
    }
    log::set_max_level(log::LevelFilter::Debug);
    let _node: Node = OperationNode::new(
        987_654_321,
        TestOp::new(ResourceKind::CrossEncoder),
        0,
        true,
        new_task(),
    );
    let records = LOGGER.records.lock().unwrap();
    let matching: Vec<_> = records
        .iter()
        .filter(|(_, msg)| msg.contains("987654321"))
        .collect();
    assert_eq!(matching.len(), 1);
    let (level, msg) = matching[0];
    assert_eq!(* level, log::Level::Debug);
    assert!(msg.contains("test-op"));
    assert!(msg.contains("CrossEncoder"));
    assert!(msg.contains("true"));
}
