use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::Channel;
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Debug)]
struct PErr;
impl std::fmt::Display for PErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "perr")
    }
}
impl std::error::Error for PErr {}
struct TestPlanner {
    kinds: Vec<QueueKind>,
    fail: bool,
    calls: AtomicUsize,
}
impl Planner<u32, u32, (), u32, u32> for TestPlanner {
    type Error = PErr;
    fn expand(&self, _t: &Task<u32, u32, ()>) -> Result<Vec<PlannedOp<u32, u32>>, PErr> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(PErr);
        }
        Ok(
            self
                .kinds
                .iter()
                .enumerate()
                .map(|(i, k)| PlannedOp {
                    kind: *k,
                    meta: i as u32,
                    values: i as u32 * 10,
                })
                .collect(),
        )
    }
}
type P = Processor<u32, u32, (), u32, u32, (), (), TestPlanner>;
fn make(kinds: Vec<QueueKind>, fail: bool) -> Arc<P> {
    Processor::new(TestPlanner {
        kinds,
        fail,
        calls: AtomicUsize::new(0),
    })
}
#[test]
fn submit_pushes_into_task_queue() {
    let p = make(vec![], false);
    let a = p.task_queue().epoch(Channel::Available);
    let params = TaskParams {
        priority: 3,
        ..Default::default()
    };
    let t = p.submit(7, params.clone());
    assert_eq!(t.params(), & params);
    assert_ne!(p.task_queue().epoch(Channel::Available), a);
    assert_eq!(p.task_queue().len(), 1);
    let claim = p.task_queue().take().unwrap();
    assert_eq!(claim.meta(), & params);
}
#[test]
fn expand_routes_in_order() {
    let p = make(vec![QueueKind::Gpu, QueueKind::Cpu, QueueKind::Gpu], false);
    let t = p.submit(1, TaskParams::default());
    let tickets = p.expand(&t).unwrap();
    assert_eq!(tickets.len(), 3);
    assert_eq!(tickets[1].0, QueueKind::Cpu);
    assert_eq!(p.ops_queue(QueueKind::Gpu).len(), 2);
    assert_eq!(p.ops_queue(QueueKind::Cpu).len(), 1);
    assert_eq!(p.ops_queue(QueueKind::Shared).len(), 0);
    let c = p.ops_queue(QueueKind::Gpu).take().unwrap();
    assert_eq!(* c.meta(), 0);
}
#[test]
fn expand_empty_and_error() {
    let p = make(vec![], false);
    let t = p.submit(1, TaskParams::default());
    assert!(p.expand(& t).unwrap().is_empty());
    let p = make(vec![QueueKind::Cpu], true);
    let t = p.submit(1, TaskParams::default());
    assert!(matches!(p.expand(& t), Err(ExpandError::Planner(PErr))));
    assert_eq!(p.ops_queue(QueueKind::Cpu).len(), 0);
}
#[test]
fn queues_are_independent() {
    let p = make(vec![], false);
    let e = p.ops_queue(QueueKind::Cpu).epoch(Channel::Available);
    p.submit(1, TaskParams::default());
    p.ops_queue(QueueKind::Gpu).push(1, 1);
    assert_eq!(p.ops_queue(QueueKind::Cpu).epoch(Channel::Available), e);
    assert!(std::ptr::eq(p.ops_queue(QueueKind::Gpu), p.ops_queue(QueueKind::Gpu)));
    assert!(! std::ptr::eq(p.ops_queue(QueueKind::Gpu), p.ops_queue(QueueKind::Shared)));
}
