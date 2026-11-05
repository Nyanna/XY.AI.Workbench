use std::ptr;
use std::sync::Arc;
use arc_swap::ArcSwapOption;
use log::debug;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::{
    EntryState, QueueState,
};
/// Atomic, shareable pointer slot to an optional node.
pub struct Link<P, T, E> {
    slot: ArcSwapOption<Node<P, T, E>>,
}
impl<P, T, E> Link<P, T, E> {
    pub fn empty() -> Self {
        Link {
            slot: ArcSwapOption::const_empty(),
        }
    }
    pub fn new(node: Arc<Node<P, T, E>>) -> Self {
        Link {
            slot: ArcSwapOption::new(Some(node)),
        }
    }
    /// Clone of the current target, `None` if empty.
    pub fn load(&self) -> Option<Arc<Node<P, T, E>>> {
        self.slot.load_full()
    }
    /// Unconditional atomic swap; returns the previous target.
    pub fn swap(&self, new: Option<Arc<Node<P, T, E>>>) -> Option<Arc<Node<P, T, E>>> {
        self.slot.swap(new)
    }
    /// Compare-and-swap by pointer identity (`None` matches `None`).
    pub fn compare_and_swap(
        &self,
        expected: Option<&Arc<Node<P, T, E>>>,
        new: Option<Arc<Node<P, T, E>>>,
    ) -> Result<(), Option<Arc<Node<P, T, E>>>> {
        let expected_ptr: *const Node<P, T, E> = expected
            .map_or(ptr::null(), Arc::as_ptr);
        let previous = self.slot.compare_and_swap(expected_ptr, new);
        let previous_ptr: *const Node<P, T, E> = (*previous)
            .as_ref()
            .map_or(ptr::null(), Arc::as_ptr);
        if previous_ptr == expected_ptr {
            Ok(())
        } else {
            Err(arc_swap::Guard::into_inner(previous))
        }
    }
}
impl<P, T, E> Default for Link<P, T, E> {
    fn default() -> Self {
        Self::empty()
    }
}
pub struct Node<P, T, E> {
    entry: Arc<EntryState<P, T, E>>,
    next: Link<P, T, E>,
}
impl<P, T, E> Node<P, T, E> {
    pub fn new(entry: Arc<EntryState<P, T, E>>) -> Arc<Self> {
        Arc::new(Node { entry, next: Link::empty() })
    }
    pub fn entry(&self) -> &Arc<EntryState<P, T, E>> {
        &self.entry
    }
    /// The successor slot.
    pub fn next(&self) -> &Link<P, T, E> {
        &self.next
    }
}
impl<P, T, E> Drop for Node<P, T, E> {
    fn drop(&mut self) {
        let mut current = self.next.swap(None);
        while let Some(arc) = current {
            match Arc::into_inner(arc) {
                Some(node) => {
                    current = node.next.swap(None);
                }
                None => break,
            }
        }
    }
}
/// Best-effort removal of `node` from the chain behind `pred`.
/// Returns true only if this call's compare-and-swap removed it.
pub fn unlink<P, T, E>(pred: &Link<P, T, E>, node: &Arc<Node<P, T, E>>) -> bool {
    if matches!(node.entry().state(), QueueState::Enqueued) {
        return false;
    }
    let successor = match node.next().load() {
        Some(successor) => successor,
        None => return false,
    };
    let removed = pred.compare_and_swap(Some(node), Some(successor)).is_ok();
    if !removed {
        debug!("unlink skipped: predecessor no longer points to node");
    }
    removed
}
#[cfg(test)]
#[path = "test/queue_reclaim.rs"]
mod tests;
