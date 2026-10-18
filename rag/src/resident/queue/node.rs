//! Append-only chain node. Public API hides the sentinel used by [`super::Queue`]
//! as a stable head anchor so the head pointer itself never needs a CAS.

use std::sync::Arc;

use arc_swap::ArcSwapOption;
use parking_lot::Mutex;

use super::state::TaskState;

enum Content<P, D, O, E> {
    Sentinel,
    Real {
        params: P,
        state: Arc<TaskState<O, E>>,
        payload: Mutex<Option<D>>,
    },
}

pub struct Node<P, D, O, E> {
    content: Content<P, D, O, E>,
    next: ArcSwapOption<Node<P, D, O, E>>,
}

impl<P, D, O, E> Node<P, D, O, E> {
    pub(crate) fn sentinel() -> Arc<Self> {
        Arc::new(Self {
            content: Content::Sentinel,
            next: ArcSwapOption::from(None),
        })
    }

    pub fn new(params: P, state: Arc<TaskState<O, E>>, data: D) -> Arc<Self> {
        Arc::new(Self {
            content: Content::Real {
                params,
                state,
                payload: Mutex::new(Some(data)),
            },
            next: ArcSwapOption::from(None),
        })
    }

    /// `None` for the sentinel; otherwise the immutable filter metadata.
    pub fn params(&self) -> Option<&P> {
        match &self.content {
            Content::Real { params, .. } => Some(params),
            Content::Sentinel => None,
        }
    }

    pub fn task_state(&self) -> Option<&Arc<TaskState<O, E>>> {
        match &self.content {
            Content::Real { state, .. } => Some(state),
            Content::Sentinel => None,
        }
    }

    /// Attempts `QUEUED -> TAKEN`; on success takes the payload. Never blocks,
    /// never awaits under the payload lock.
    pub fn claim(&self) -> Option<D> {
        match &self.content {
            Content::Real { state, payload, .. } => {
                if state.claim() {
                    payload.lock().take()
                } else {
                    None
                }
            }
            Content::Sentinel => None,
        }
    }

    pub(crate) fn peek_next(&self) -> Option<Arc<Node<P, D, O, E>>> {
        (*self.next.load()).clone()
    }

    /// CAS-appends `new` as this node's successor. `Ok` on success; `Err`
    /// returns whatever is actually linked so the caller can advance and retry.
    pub(crate) fn try_link_next(
        &self,
        new: Arc<Node<P, D, O, E>>,
    ) -> Result<(), Arc<Node<P, D, O, E>>> {
        let prev = self.next.compare_and_swap(&None::<Arc<Node<P, D, O, E>>>, Some(new));
        match &*prev {
            None => Ok(()),
            Some(existing) => Err(existing.clone()),
        }
    }
}

impl<P, D, O, E> Drop for Node<P, D, O, E> {
    fn drop(&mut self) {
        // Iterative: the default recursive drop would overflow the stack on
        // chains with tens of thousands of nodes.
        let mut cur = self.next.swap(None);
        while let Some(arc) = cur {
            match Arc::try_unwrap(arc) {
                Ok(node) => cur = node.next.swap(None),
                Err(_shared) => break, // another owner takes over the remainder
            }
        }
    }
}

/// Traverses live nodes (sentinel excluded) in age order. Holds one `Arc<Node>`
/// per step so a concurrently unlinked node stays valid; never blocks.
pub struct Iter<P, D, O, E> {
    next: Option<Arc<Node<P, D, O, E>>>,
}

impl<P, D, O, E> Iter<P, D, O, E> {
    pub(crate) fn from_sentinel(sentinel: Arc<Node<P, D, O, E>>) -> Self {
        Self { next: Some(sentinel) }
    }
}

impl<P, D, O, E> Iterator for Iter<P, D, O, E> {
    type Item = Arc<Node<P, D, O, E>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cur = self.next.take()?;
            self.next = cur.peek_next();
            if cur.params().is_some() {
                return Some(cur);
            }
            // Only the initial sentinel hits this branch; loop once more.
        }
    }
}
