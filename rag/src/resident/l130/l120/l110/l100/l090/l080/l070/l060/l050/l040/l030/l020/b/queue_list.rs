use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use arc_swap::{ArcSwap, ArcSwapOption};
use tokio::sync::Notify;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::EntryState;
pub type NotifyChannel = Arc<Notify>;
pub struct QueueNode<M, P, T, E> {
    meta: Option<M>,
    state: Option<Arc<EntryState<P, T, E>>>,
    next: ArcSwapOption<QueueNode<M, P, T, E>>,
}
impl<M, P, T, E> QueueNode<M, P, T, E> {
    fn sentinel() -> Self {
        Self {
            meta: None,
            state: None,
            next: ArcSwapOption::empty(),
        }
    }
    fn entry(meta: M, state: Arc<EntryState<P, T, E>>) -> Self {
        Self {
            meta: Some(meta),
            state: Some(state),
            next: ArcSwapOption::empty(),
        }
    }
    pub fn meta(&self) -> Option<&M> {
        self.meta.as_ref()
    }
    pub fn state(&self) -> Option<&Arc<EntryState<P, T, E>>> {
        self.state.as_ref()
    }
    pub fn next(&self) -> Option<Arc<QueueNode<M, P, T, E>>> {
        self.next.load_full()
    }
    pub fn is_sentinel(&self) -> bool {
        self.state.is_none()
    }
}
impl<M, P, T, E> super::queue_iter::QueueNode for QueueNode<M, P, T, E>
where
    M: Send + Sync + 'static,
    P: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    type Payload = P;
    type Output = T;
    type Error = E;
    type Meta = M;
    fn is_sentinel(&self) -> bool {
        self.state.is_none()
    }
    fn next(&self) -> Option<Arc<Self>> {
        self.next.load_full()
    }
    fn entry(&self) -> &Arc<EntryState<P, T, E>> {
        self.state.as_ref().expect("entry requested on sentinel node")
    }
    fn meta(&self) -> &M {
        self.meta.as_ref().expect("meta requested on sentinel node")
    }
}
impl<M, P, T, E> Drop for QueueNode<M, P, T, E> {
    fn drop(&mut self) {
        let mut cur = self.next.swap(None);
        while let Some(node) = cur {
            match Arc::into_inner(node) {
                Some(inner) => cur = inner.next.swap(None),
                None => break,
            }
        }
    }
}
/// Sentinel-list queue_list: `enqueue` appends a node, consumers walk it via `head()` and
/// `queue_iter` and claim through the entry state. Not the take-based queue in
/// `l020::l010::queue` (`push`/`take`/`take_where`), which is a different design.
pub struct Queue<M, P, T, E> {
    head: Arc<QueueNode<M, P, T, E>>,
    tail_hint: ArcSwap<QueueNode<M, P, T, E>>,
    channels: Vec<NotifyChannel>,
    tail_hint_enabled: AtomicBool,
}
impl<M, P, T, E> Queue<M, P, T, E> {
    pub fn new(channels: Vec<NotifyChannel>) -> Self {
        let head = Arc::new(QueueNode::sentinel());
        Self {
            tail_hint: ArcSwap::from(Arc::clone(&head)),
            head,
            channels,
            tail_hint_enabled: AtomicBool::new(true),
        }
    }
    pub fn enqueue(
        &self,
        meta: M,
        state: Arc<EntryState<P, T, E>>,
    ) -> Arc<QueueNode<M, P, T, E>> {
        let new = Arc::new(QueueNode::entry(meta, state));
        let mut cur = self.tail_hint.load_full();
        loop {
            while let Some(next) = cur.next.load_full() {
                cur = next;
            }
            let expected: Option<Arc<QueueNode<M, P, T, E>>> = None;
            let prev = cur.next.compare_and_swap(&expected, Some(Arc::clone(&new)));
            match &*prev {
                None => break,
                Some(observed) => cur = Arc::clone(observed),
            }
        }
        if self.tail_hint_enabled.load(Ordering::Acquire) {
            self.tail_hint.store(Arc::clone(&new));
        }
        for channel in &self.channels {
            channel.notify_one();
        }
        new
    }
    pub fn head(&self) -> Arc<QueueNode<M, P, T, E>> {
        Arc::clone(&self.head)
    }
    pub fn first(&self) -> Option<Arc<QueueNode<M, P, T, E>>> {
        self.head.next()
    }
    pub fn set_tail_hint_update(&self, enabled: bool) {
        self.tail_hint_enabled.store(enabled, Ordering::Release);
    }
}
#[cfg(test)]
#[path = "test/queue_list.rs"]
mod tests;
