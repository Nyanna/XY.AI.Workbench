use super::*;
use std::sync::Mutex;

struct TestNode {
    sentinel: bool,
    entry: Arc<EntryState<u32, u32, String>>,
    meta: u32,
    next: Mutex<Option<Arc<TestNode>>>,
}

impl QueueNode for TestNode {
    type Payload = u32;
    type Output = u32;
    type Error = String;
    type Meta = u32;

    fn is_sentinel(&self) -> bool {
        self.sentinel
    }
    fn next(&self) -> Option<Arc<Self>> {
        self.next.lock().unwrap().clone()
    }
    fn entry(&self) -> &Arc<EntryState<u32, u32, String>> {
        &self.entry
    }
    fn meta(&self) -> &u32 {
        &self.meta
    }
}

fn node(sentinel: bool, meta: u32) -> Arc<TestNode> {
    Arc::new(TestNode {
        sentinel,
        entry: if sentinel {
            EntryState::sentinel()
        } else {
            EntryState::new(meta)
        },
        meta,
        next: Mutex::new(None),
    })
}

fn build(n: u32) -> Vec<Arc<TestNode>> {
    let mut nodes = vec![node(true, 0)];
    for i in 1..=n {
        let nd = node(false, i);
        *nodes.last().unwrap().next.lock().unwrap() = Some(nd.clone());
        nodes.push(nd);
    }
    nodes
}

#[test]
fn empty_queue_yields_nothing() {
    let nodes = build(0);
    assert_eq!(iter_all(&nodes[0]).count(), 0);
    assert_eq!(iter_active(&nodes[0]).count(), 0);
}

#[test]
fn all_in_order_with_meta() {
    let nodes = build(4);
    let metas: Vec<u32> = iter_all(&nodes[0]).map(|i| *i.meta()).collect();
    assert_eq!(metas, vec![1, 2, 3, 4]);
}

#[test]
fn active_skips_cancelled_and_all_keeps_them() {
    let nodes = build(4);
    assert!(nodes[2].entry.cancel());
    let all: Vec<u32> = iter_all(&nodes[0]).map(|i| *i.meta()).collect();
    let active: Vec<u32> = iter_active(&nodes[0]).map(|i| *i.meta()).collect();
    assert_eq!(all, vec![1, 2, 3, 4]);
    assert_eq!(active, vec![1, 3, 4]);
}

#[test]
fn fused_and_sees_late_append_before_end() {
    let nodes = build(1);
    let mut it = iter_all(&nodes[0]);
    assert!(it.next().is_some());
    let extra = node(false, 9);
    *nodes[1].next.lock().unwrap() = Some(extra);
    assert_eq!(*it.next().unwrap().meta(), 9);
    assert!(it.next().is_none());
    let later = node(false, 10);
    *nodes[1].next.lock().unwrap().as_ref().unwrap().next.lock().unwrap() = Some(later);
    assert!(it.next().is_none());
}

#[test]
fn item_state_reflects_changes() {
    let nodes = build(1);
    let item = iter_active(&nodes[0]).next().unwrap();
    assert_eq!(item.state(), QueueState::Enqueued);
    item.entry().cancel();
    assert_eq!(item.state(), QueueState::Cancelled);
}

#[test]
fn iterators_are_send() {
    fn assert_send<T: Send>() {}
    assert_send::<AllIter<TestNode>>();
    assert_send::<ActiveIter<TestNode>>();
}
