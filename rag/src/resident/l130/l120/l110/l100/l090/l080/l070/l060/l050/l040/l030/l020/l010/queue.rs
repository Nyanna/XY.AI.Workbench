//! Generic unbounded lock-free MPMC queue of stateful work items.
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use crossbeam::epoch::{self, Atomic, Guard, Owned, Shared};
const PENDING: u8 = 0;
const RUNNING: u8 = 1;
const SUCCEEDED: u8 = 2;
const FAILED: u8 = 3;
const ABANDONED: u8 = 4;
/// Item lifecycle. Transitions only forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Pending,
    Running,
    Succeeded,
    Failed,
    Abandoned,
}
impl State {
    fn from_raw(raw: u8) -> State {
        match raw {
            PENDING => State::Pending,
            RUNNING => State::Running,
            SUCCEEDED => State::Succeeded,
            FAILED => State::Failed,
            _ => State::Abandoned,
        }
    }
    pub fn is_final(self) -> bool {
        matches!(self, State::Succeeded | State::Failed | State::Abandoned)
    }
}
/// Per-instance notification channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Available,
    Completed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultError {
    NotReady,
    Taken,
    Abandoned,
}
/// Epoch counter with parking support. Bumping never blocks unless a waiter exists.
struct Notifier {
    epoch: AtomicU64,
    waiters: AtomicUsize,
    lock: Mutex<()>,
    cv: Condvar,
}
impl Notifier {
    fn new() -> Self {
        Notifier {
            epoch: AtomicU64::new(0),
            waiters: AtomicUsize::new(0),
            lock: Mutex::new(()),
            cv: Condvar::new(),
        }
    }
    fn epoch(&self) -> u64 {
        self.epoch.load(SeqCst)
    }
    fn bump(&self) {
        self.epoch.fetch_add(1, SeqCst);
        if self.waiters.load(SeqCst) > 0 {
            let _guard = self.lock.lock().unwrap_or_else(|e| e.into_inner());
            self.cv.notify_all();
        }
    }
    fn wait_change(&self, seen: u64, timeout: Option<Duration>) -> u64 {
        let current = self.epoch();
        if current != seen || timeout == Some(Duration::ZERO) {
            return current;
        }
        let deadline = timeout.and_then(|t| Instant::now().checked_add(t));
        self.waiters.fetch_add(1, SeqCst);
        let mut guard = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        let result = loop {
            let current = self.epoch();
            if current != seen {
                break current;
            }
            match deadline {
                None => {
                    guard = self.cv.wait(guard).unwrap_or_else(|e| e.into_inner());
                }
                Some(deadline) => {
                    let now = Instant::now();
                    if now >= deadline {
                        break current;
                    }
                    guard = self
                        .cv
                        .wait_timeout(guard, deadline - now)
                        .unwrap_or_else(|e| e.into_inner())
                        .0;
                }
            }
        };
        drop(guard);
        self.waiters.fetch_sub(1, SeqCst);
        result
    }
}
struct Channels {
    available: Notifier,
    completed: Notifier,
}
struct Item<M, P, R, E> {
    meta: M,
    state: AtomicU8,
    payload: UnsafeCell<Option<P>>,
    result: UnsafeCell<Option<Result<R, E>>>,
    result_taken: AtomicBool,
    notifier: Notifier,
}
unsafe impl<M: Send + Sync, P: Send, R: Send, E: Send> Send for Item<M, P, R, E> {}
unsafe impl<M: Send + Sync, P: Send, R: Send, E: Send> Sync for Item<M, P, R, E> {}
impl<M, P, R, E> Item<M, P, R, E> {
    fn state(&self) -> u8 {
        self.state.load(Acquire)
    }
    fn try_claim(&self) -> bool {
        self.state.compare_exchange(PENDING, RUNNING, SeqCst, Relaxed).is_ok()
    }
}
struct Node<M, P, R, E> {
    item: Option<Arc<Item<M, P, R, E>>>,
    next: Atomic<Node<M, P, R, E>>,
}
/// Take-based work queue: `push` returns a `Ticket`, consumers `take`/`take_where` a `Claim`.
/// Not the sentinel-list queue_list in `l020::b::queue` (`enqueue`/`head`/`iter_active`), which is a
/// different design for a different consumer set.
pub struct Queue<M, P, R, E> {
    head: Atomic<Node<M, P, R, E>>,
    tail: Atomic<Node<M, P, R, E>>,
    pending: AtomicUsize,
    channels: Arc<Channels>,
}
impl<M, P, R, E> Queue<M, P, R, E>
where
    M: Send + Sync + 'static,
    P: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    pub fn new() -> Self {
        let sentinel = Owned::new(Node {
            item: None,
            next: Atomic::null(),
        });
        let guard = unsafe { epoch::unprotected() };
        let sentinel = sentinel.into_shared(guard);
        let head = Atomic::null();
        let tail = Atomic::null();
        head.store(sentinel, Relaxed);
        tail.store(sentinel, Relaxed);
        Queue {
            head,
            tail,
            pending: AtomicUsize::new(0),
            channels: Arc::new(Channels {
                available: Notifier::new(),
                completed: Notifier::new(),
            }),
        }
    }
    /// Enqueue at the tail.
    pub fn push(&self, meta: M, payload: P) -> Ticket<M, P, R, E> {
        let item = Arc::new(Item {
            meta,
            state: AtomicU8::new(PENDING),
            payload: UnsafeCell::new(Some(payload)),
            result: UnsafeCell::new(None),
            result_taken: AtomicBool::new(false),
            notifier: Notifier::new(),
        });
        self.pending.fetch_add(1, SeqCst);
        {
            let guard = &epoch::pin();
            let mut new = Owned::new(Node {
                item: Some(item.clone()),
                next: Atomic::null(),
            });
            loop {
                let tail = self.tail.load(Acquire, guard);
                let tail_ref = unsafe { tail.deref() };
                let next = tail_ref.next.load(Acquire, guard);
                if !next.is_null() {
                    let _ = self
                        .tail
                        .compare_exchange(tail, next, Release, Relaxed, guard);
                    continue;
                }
                match tail_ref
                    .next
                    .compare_exchange(Shared::null(), new, Release, Relaxed, guard)
                {
                    Ok(linked) => {
                        let _ = self
                            .tail
                            .compare_exchange(tail, linked, Release, Relaxed, guard);
                        break;
                    }
                    Err(err) => new = err.new,
                }
            }
        }
        self.channels.available.bump();
        Ticket { item }
    }
    /// Claim the oldest Pending item.
    pub fn take(&self) -> Option<Claim<M, P, R, E>> {
        self.take_where(|_| true)
    }
    /// Claim the oldest Pending item accepted by `pred`.
    pub fn take_where<F: FnMut(&M) -> bool>(
        &self,
        mut pred: F,
    ) -> Option<Claim<M, P, R, E>> {
        let guard = &epoch::pin();
        let head = self.head.load(Acquire, guard);
        let mut cur = unsafe { head.deref() }.next.load(Acquire, guard);
        while let Some(node) = unsafe { cur.as_ref() } {
            if let Some(item) = node.item.as_ref() {
                if item.state() == PENDING && pred(&item.meta) && item.try_claim() {
                    self.pending.fetch_sub(1, SeqCst);
                    let payload = unsafe { (*item.payload.get()).take() };
                    let claim = Claim {
                        item: item.clone(),
                        payload,
                        channels: self.channels.clone(),
                        done: false,
                    };
                    self.advance_head(guard);
                    return Some(claim);
                }
            }
            cur = node.next.load(Acquire, guard);
        }
        self.advance_head(guard);
        None
    }
    /// Retire leading nodes whose items are no longer Pending.
    fn advance_head(&self, guard: &Guard) {
        loop {
            let head = self.head.load(Acquire, guard);
            let next = unsafe { head.deref() }.next.load(Acquire, guard);
            let Some(next_ref) = (unsafe { next.as_ref() }) else {
                return;
            };
            if let Some(item) = next_ref.item.as_ref() {
                if item.state() == PENDING {
                    return;
                }
            }
            let tail = self.tail.load(Acquire, guard);
            if tail == head {
                let _ = self.tail.compare_exchange(head, next, Release, Relaxed, guard);
            }
            if self.head.compare_exchange(head, next, Release, Relaxed, guard).is_ok() {
                unsafe { guard.defer_destroy(head) };
            }
        }
    }
    /// Number of Pending items.
    pub fn len(&self) -> usize {
        self.pending.load(SeqCst)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Current event counter of a channel.
    pub fn epoch(&self, ch: Channel) -> u64 {
        self.notifier(ch).epoch()
    }
    /// Block until the channel epoch differs from `seen` or the timeout expires.
    pub fn wait_change(&self, ch: Channel, seen: u64, timeout: Option<Duration>) -> u64 {
        self.notifier(ch).wait_change(seen, timeout)
    }
    fn notifier(&self, ch: Channel) -> &Notifier {
        match ch {
            Channel::Available => &self.channels.available,
            Channel::Completed => &self.channels.completed,
        }
    }
}
impl<M, P, R, E> Default for Queue<M, P, R, E>
where
    M: Send + Sync + 'static,
    P: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<M, P, R, E> Drop for Queue<M, P, R, E> {
    fn drop(&mut self) {
        unsafe {
            let guard = epoch::unprotected();
            let mut cur = self.head.load(Relaxed, guard);
            while !cur.is_null() {
                let owned = cur.into_owned();
                let next = owned.next.load(Relaxed, guard);
                if let Some(item) = owned.item.as_ref() {
                    if item
                        .state
                        .compare_exchange(PENDING, ABANDONED, SeqCst, Relaxed)
                        .is_ok()
                    {
                        drop((*item.payload.get()).take());
                        item.notifier.bump();
                    }
                }
                drop(owned);
                cur = next;
            }
        }
    }
}
/// Producer-side handle to one item.
pub struct Ticket<M, P, R, E> {
    item: Arc<Item<M, P, R, E>>,
}
impl<M, P, R, E> Clone for Ticket<M, P, R, E> {
    fn clone(&self) -> Self {
        Ticket { item: self.item.clone() }
    }
}
impl<M, P, R, E> Ticket<M, P, R, E> {
    pub fn state(&self) -> State {
        State::from_raw(self.item.state())
    }
    pub fn meta(&self) -> &M {
        &self.item.meta
    }
    /// Move the result out (once).
    pub fn take_result(&self) -> Result<Result<R, E>, ResultError> {
        match self.item.state() {
            PENDING | RUNNING => Err(ResultError::NotReady),
            ABANDONED => Err(ResultError::Abandoned),
            _ => {
                if self.item.result_taken.swap(true, SeqCst) {
                    return Err(ResultError::Taken);
                }
                match unsafe { (*self.item.result.get()).take() } {
                    Some(result) => Ok(result),
                    None => Err(ResultError::Taken),
                }
            }
        }
    }
    /// Block until the item is final or the timeout expires.
    pub fn wait(&self, timeout: Option<Duration>) -> State {
        let deadline = timeout.and_then(|t| Instant::now().checked_add(t));
        loop {
            let seen = self.item.notifier.epoch();
            let state = self.state();
            if state.is_final() {
                return state;
            }
            let remaining = match (timeout, deadline) {
                (Some(_), Some(d)) => {
                    let rem = d.saturating_duration_since(Instant::now());
                    if rem.is_zero() {
                        return state;
                    }
                    Some(rem)
                }
                _ => None,
            };
            self.item.notifier.wait_change(seen, remaining);
        }
    }
}
/// Exclusive consumer-side ownership of one Running item.
pub struct Claim<M, P, R, E> {
    item: Arc<Item<M, P, R, E>>,
    payload: Option<P>,
    channels: Arc<Channels>,
    done: bool,
}
impl<M, P, R, E> Claim<M, P, R, E> {
    pub fn meta(&self) -> &M {
        &self.item.meta
    }
    pub fn take_payload(&mut self) -> Option<P> {
        self.payload.take()
    }
    pub fn ticket(&self) -> Ticket<M, P, R, E> {
        Ticket { item: self.item.clone() }
    }
    /// Store the result and publish the final state.
    pub fn complete(mut self, result: Result<R, E>) {
        let state = if result.is_ok() { SUCCEEDED } else { FAILED };
        self.finish(state, Some(result));
    }
    fn finish(&mut self, state: u8, result: Option<Result<R, E>>) {
        if self.done {
            return;
        }
        self.done = true;
        if let Some(result) = result {
            unsafe { *self.item.result.get() = Some(result) };
        }
        self.item.state.store(state, Release);
        self.item.notifier.bump();
        self.channels.completed.bump();
    }
}
impl<M, P, R, E> Drop for Claim<M, P, R, E> {
    fn drop(&mut self) {
        self.finish(ABANDONED, None);
    }
}
#[cfg(test)]
#[path = "test/queue.rs"]
mod tests;
