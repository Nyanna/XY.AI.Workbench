//! Generic lock-free queue: sentinel head, CAS append, lazy logical unlink.
use std::sync::Arc;
use arc_swap::ArcSwapOption;
use tokio::sync::Notify;
use super::node::{Iter, Node};
use crate::resident::failpoint;
use super::state::TaskState;
/// Generic over `P` (filter metadata) and `D` (payload); `O`/`E` are the
/// result/error types carried by each node's [`TaskState`]. Reused as-is for
/// op-queues, whose completion is tracked externally (task-op counter) rather
/// than through `TaskState::result` (see `task` submodule).
pub struct Queue<P, D, O, E> {
    head: Arc<Node<P, D, O, E>>,
    tail: ArcSwapOption<Node<P, D, O, E>>,
    relays: Vec<Arc<Notify>>,
}
impl<P, D, O, E> Queue<P, D, O, E> {
    /// `relays` are notified (append-relay) on every push; one per subengine.
    pub fn new(relays: Vec<Arc<Notify>>) -> Self {
        Self {
            head: Node::sentinel(),
            tail: ArcSwapOption::from(None),
            relays,
        }
    }
    pub fn push(
        &self,
        params: P,
        state: Arc<TaskState<O, E>>,
        data: D,
    ) -> Arc<Node<P, D, O, E>> {
        let node = Node::new(params, state, data);
        self.append(node.clone());
        for relay in &self.relays {
            relay.notify_one();
        }
        tracing::debug!("push");
        node
    }
    fn append(&self, new: Arc<Node<P, D, O, E>>) {
        let mut cur = self.tail_hint();
        loop {
            if let Some(next) = cur.peek_next() {
                cur = next;
                continue;
            }
            match cur.try_link_next(new.clone()) {
                Ok(()) => {
                    if !failpoint::hit("queue.skip_tail_store", self as *const Self as u64) {
                        self.tail.store(Some(new));
                    }
                    return;
                }
                Err(existing) => cur = existing,
            }
        }
    }
    fn tail_hint(&self) -> Arc<Node<P, D, O, E>> {
        (*self.tail.load()).clone().unwrap_or_else(|| self.head.clone())
    }
    /// Non-blocking traversal, age order, sentinel excluded.
    pub fn iter(&self) -> Iter<P, D, O, E> {
        Iter::from_sentinel(self.head.clone())
    }
    /// Like [`Self::iter`], skipping logically unlinked nodes (`Done`/`Cancelled`).
    pub fn iter_live(&self) -> impl Iterator<Item = Arc<Node<P, D, O, E>>> {
        self.iter().filter(|n| n.is_live())
    }

    pub fn claim(&self, node: &Node<P, D, O, E>) -> Option<D> {
        node.claim()
    }
}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use super::super::state::TaskState;
    fn task_state() -> Arc<TaskState<u32, ()>> {
        Arc::new(TaskState::new())
    }
    #[test]
    fn push_claim_takes_payload_once() {
        let q: Queue<&str, u32, u32, ()> = Queue::new(vec![]);
        let node = q.push("p", task_state(), 99);
        assert_eq!(q.claim(& node), Some(99));
        assert_eq!(q.claim(& node), None);
    }
    #[test]
    fn iter_yields_age_order_and_skips_sentinel() {
        let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);
        q.push(1, task_state(), 1);
        q.push(2, task_state(), 2);
        q.push(3, task_state(), 3);
        let seen: Vec<i32> = q.iter().map(|n| *n.params().unwrap()).collect();
        assert_eq!(seen, vec![1, 2, 3]);
    }
    #[test]
    fn push_notifies_all_relays() {
        let relay = Arc::new(Notify::new());
        let q: Queue<(), (), u32, ()> = Queue::new(vec![relay.clone()]);
        q.push((), task_state(), ());
        let notified = relay.notified();
        tokio::pin!(notified);
        assert!(notified.as_mut().enable());
    }
    #[test]
    fn drop_long_chain_does_not_overflow_stack() {
        let q: Queue<u32, u32, u32, ()> = Queue::new(vec![]);
        for i in 0..100_000u32 {
            q.push(i, task_state(), i);
        }
        drop(q);
    }

    #[test]
    fn unlinked_nodes_are_skipped_by_live_iteration() {
        let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);
        let a = q.push(1, task_state(), 1);
        let b = q.push(2, task_state(), 2);
        q.push(3, task_state(), 3);
        a.task_state().unwrap().cancel();
        assert!(q.claim(&b).is_some());
        b.task_state().unwrap().complete(Ok(0));
        let live: Vec<i32> = q.iter_live().map(|n| *n.params().unwrap()).collect();
        assert_eq!(live, vec![3]);
        assert_eq!(q.iter().count(), 3);
    }

    #[test]
    fn stale_tail_hint_keeps_order() {
        let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);
        let key = &q as *const _ as u64;
        let _fp = failpoint::arm("queue.skip_tail_store", key, 2);
        for i in 0..4 {
            q.push(i, task_state(), i);
        }
        let seen: Vec<i32> = q.iter().map(|n| *n.params().unwrap()).collect();
        assert_eq!(seen, vec![0, 1, 2, 3]);
    }

    #[test]
    fn concurrent_pushes_are_all_linked() {
        let q: Arc<Queue<u32, u32, u32, ()>> = Arc::new(Queue::new(vec![]));
        let handles: Vec<_> = (0..4u32)
            .map(|t| {
                let q = q.clone();
                std::thread::spawn(move || {
                    for i in 0..250 {
                        q.push(t * 1000 + i, task_state(), 0);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let mut per_thread = [0u32; 4];
        for n in q.iter() {
            let v = *n.params().unwrap();
            let (t, i) = ((v / 1000) as usize, v % 1000);
            assert_eq!(i, per_thread[t], "per-producer order must be preserved");
            per_thread[t] += 1;
        }
        assert_eq!(per_thread, [250; 4]);
    }

    proptest::proptest! {
        #[test]
        fn iteration_matches_push_order(items in proptest::collection::vec(proptest::num::i32::ANY, 0..200)) {
            let q: Queue<i32, (), u32, ()> = Queue::new(vec![]);
            for &i in &items {
                q.push(i, task_state(), ());
            }
            let seen: Vec<i32> = q.iter().map(|n| *n.params().unwrap()).collect();
            proptest::prop_assert_eq!(seen, items);
        }
    }
}
