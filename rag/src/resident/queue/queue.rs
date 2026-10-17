//! Generic lock-free queue: sentinel head, CAS append, lazy logical unlink.
use std::sync::Arc;
use arc_swap::ArcSwapOption;
use tokio::sync::Notify;
use super::node::{Iter, Node};
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
                    self.tail.store(Some(new));
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
    pub fn claim(&self, node: &Node<P, D, O, E>) -> Option<D> {
        node.claim()
    }
}
#[cfg(test)]
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
}
