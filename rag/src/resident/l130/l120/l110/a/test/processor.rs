use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::Channel;
use std::sync::atomic::AtomicUsize;
use std::thread;
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
fn submit_pushes_and_notifies() {
    let p = make(vec![], false);
    let n = p.notification().epoch();
    let a = p.task_queue().epoch(Channel::Available);
    let params = TaskParams {
        priority: 3,
        ..Default::default()
    };
    let t = p.submit(7, params.clone());
    assert_eq!(t.params(), & params);
    assert_ne!(p.notification().epoch(), n);
    assert_ne!(p.task_queue().epoch(Channel::Available), a);
    assert_eq!(p.task_queue().len(), 1);
    let claim = p.task_queue().take().unwrap();
    assert_eq!(claim.meta(), & params);
}
#[test]
fn expand_routes_in_order() {
    let p = make(vec![QueueKind::Gpu, QueueKind::Cpu, QueueKind::Gpu], false);
    let t = p.submit(1, TaskParams::default());
    let n = p.notification().epoch();
    let tickets = p.expand(&t).unwrap();
    assert_eq!(tickets.len(), 3);
    assert_eq!(tickets[1].0, QueueKind::Cpu);
    assert_eq!(p.ops_queue(QueueKind::Gpu).len(), 2);
    assert_eq!(p.ops_queue(QueueKind::Cpu).len(), 1);
    assert_eq!(p.ops_queue(QueueKind::Shared).len(), 0);
    assert_eq!(p.notification().epoch(), n.wrapping_add(1));
    let c = p.ops_queue(QueueKind::Gpu).take().unwrap();
    assert_eq!(* c.meta(), 0);
}
#[test]
fn expand_empty_and_error() {
    let p = make(vec![], false);
    let t = p.submit(1, TaskParams::default());
    let n = p.notification().epoch();
    assert!(p.expand(& t).unwrap().is_empty());
    assert_eq!(p.notification().epoch(), n);
    let p = make(vec![QueueKind::Cpu], true);
    let t = p.submit(1, TaskParams::default());
    let n = p.notification().epoch();
    assert!(matches!(p.expand(& t), Err(ExpandError::Planner(PErr))));
    assert_eq!(p.notification().epoch(), n);
    assert_eq!(p.ops_queue(QueueKind::Cpu).len(), 0);
    assert_eq!(p.pool().registered(), 0);
}
#[test]
fn park_returns_on_changed_epoch_and_timeout() {
    let p = make(vec![], false);
    let id = p.pool().register();
    let seen = p.notification().epoch();
    p.notification().notify();
    assert_ne!(p.pool().park(id, seen, None), seen);
    let seen = p.notification().epoch();
    let start = Instant::now();
    assert_eq!(p.pool().park(id, seen, Some(Duration::from_millis(50))), seen);
    assert!(start.elapsed() >= Duration::from_millis(50));
    assert_eq!(p.pool().park(id, seen, Some(Duration::ZERO)), seen);
    assert_eq!(p.pool().parked(), 0);
    p.pool().deregister(WorkerId(999));
    assert_eq!(p.pool().park(WorkerId(999), seen, None), seen);
}
#[test]
fn notify_wakes_parked_workers() {
    let p = make(vec![], false);
    let mut hs = vec![];
    for _ in 0..3 {
        let p = p.clone();
        let id = p.pool().register();
        hs.push(
            thread::spawn(move || {
                let seen = p.notification().epoch();
                p.pool().park(id, seen, None) != seen
            }),
        );
    }
    while p.pool().parked() < 3 {
        thread::sleep(Duration::from_millis(5));
    }
    p.submit(1, TaskParams::default());
    for h in hs {
        assert!(h.join().unwrap());
    }
}
#[test]
fn idle_epoch_rules() {
    let p = make(vec![], false);
    let i0 = p.idle().epoch();
    let a = p.pool().register();
    let b = p.pool().register();
    let seen = p.notification().epoch();
    let h = {
        let p = p.clone();
        thread::spawn(move || p.pool().park(a, seen, None))
    };
    while p.pool().parked() < 1 {
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(p.idle().epoch(), i0);
    let h2 = {
        let p = p.clone();
        thread::spawn(move || p.pool().park(b, seen, None))
    };
    assert_ne!(p.idle().wait_change(i0, Some(Duration::from_secs(5))), i0);
    p.notification().notify();
    h.join().unwrap();
    h2.join().unwrap();
    assert_eq!(
        p.idle().wait_change(p.idle().epoch(), Some(Duration::from_millis(20))), p.idle()
        .epoch()
    );
}
#[test]
fn idle_independent_of_queue_contents() {
    let p = make(vec![], false);
    let i0 = p.idle().epoch();
    p.submit(1, TaskParams::default());
    let a = p.pool().register();
    let seen = p.notification().epoch();
    p.pool().park(a, seen, Some(Duration::ZERO));
    assert_eq!(p.idle().epoch(), i0.wrapping_add(1));
    p.pool().deregister(a);
    assert_eq!(p.idle().epoch(), i0.wrapping_add(1));
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
