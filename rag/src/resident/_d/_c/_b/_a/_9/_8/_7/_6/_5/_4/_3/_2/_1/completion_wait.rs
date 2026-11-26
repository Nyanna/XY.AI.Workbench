//! Runtime-agnostic completion notification primitive and generic wait loop.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

#[derive(Debug, Default)]
struct Slot3 {
    notified: bool,
    waker: Option<Waker>,
}

#[derive(Debug, Default)]
struct Inner2 {
    next_id: u64,
    slots: HashMap<u64, Slot3>,
}

#[derive(Debug, Default)]
struct Shared2 {
    inner: Mutex<Inner2>,
}

impl Shared2 {
    fn lock(&self) -> MutexGuard<'_, Inner2> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Cheap to clone; all clones share the same notifier.
#[derive(Clone, Default, Debug)]
pub struct CompletionNotifier {
    shared: Arc<Shared2>,
}

impl CompletionNotifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wakes all current waiters and marks all registered subscriptions as notified.
    pub fn notify_all(&self) {
        let wakers: Vec<Waker> = {
            let mut inner = self.shared.lock();
            inner
                .slots
                .values_mut()
                .filter_map(|slot| {
                    slot.notified = true;
                    slot.waker.take()
                })
                .collect()
        };
        for waker in wakers {
            waker.wake();
        }
    }

    /// Registers interest now; any later `notify_all` is observed by the result.
    pub fn subscribe(&self) -> Subscription {
        let mut inner = self.shared.lock();
        let id = inner.next_id;
        inner.next_id = inner.next_id.wrapping_add(1);
        inner.slots.insert(id, Slot3::default());
        drop(inner);
        Subscription {
            shared: Arc::clone(&self.shared),
            id,
        }
    }

    /// Subscribes before each `check` call, so no terminal notification is lost.
    pub async fn wait_for_terminal<S, F>(&self, mut check: F) -> S
    where
        F: FnMut() -> Option<S>,
    {
        loop {
            let sub = self.subscribe();
            if let Some(state) = check() {
                return state;
            }
            sub.await;
        }
    }
}

/// Registered interest; resolves once `notify_all` has occurred since subscription.
#[must_use]
#[derive(Debug)]
pub struct Subscription {
    shared: Arc<Shared2>,
    id: u64,
}

impl Future for Subscription {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut inner = self.shared.lock();
        match inner.slots.get_mut(&self.id) {
            None => Poll::Ready(()),
            Some(slot) if slot.notified => Poll::Ready(()),
            Some(slot) => {
                match &slot.waker {
                    Some(existing) if existing.will_wake(cx.waker()) => {}
                    _ => slot.waker = Some(cx.waker().clone()),
                }
                Poll::Pending
            }
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let removed = self.shared.lock().slots.remove(&self.id);
        drop(removed);
    }
}

#[cfg(test)]
#[path = "test/completion_wait.rs"]
mod tests;
