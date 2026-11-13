# Spec: Abschluss-Warten (Completion Waiting)

## 1. Task, restrictions and boundaries
Implement a small, runtime-agnostic Rust library module that lets any number of async tasks wait for a unit of work to reach a terminal state (completed or cancelled) without blocking a thread and without losing a notification.

- Dependencies: none. Use only `std` (`Mutex`, `Arc`, `Waker`, `Future`). Do not depend on tokio, async-std or other runtime crates.
- The module does not know about the work-state type, results, or cancel logic. It provides only the notification primitive and a generic wait loop. The owner of the state calls `notify_all()` after every transition into a terminal state, completion and successful cancel alike. The owner also supplies the state check as a closure.
- No timeouts, no result storage, no state machine, no spawning of tasks or threads.
- Use no `unsafe`.

## 2. Context
Each unit of work has a state that is protected elsewhere and can end as completed or cancelled. Callers need to wait for that end and then fetch results.
- The state-owning module holds one `CompletionNotifier` per work unit. After it sets a terminal state, it calls `notify_all()`.
- Waiters call `wait_for_terminal` with a closure that reads the state.
- The race to avoid is a completion between "read state" and "begin waiting". The fix is to register for notification before reading the state.

## 3. Interface
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Cheap to clone; all clones share the same notifier. Send + Sync.
#[derive(Clone, Default, Debug)]
pub struct CompletionNotifier { /* Arc<Mutex<..>> internally */ }

impl CompletionNotifier {
    pub fn new() -> Self;

    /// Wake all current waiters and all registered Subscriptions (their next wait resolves). Never blocks. Callable from any thread, sync context, and repeatedly.
    pub fn notify_all(&self);

    /// Register for notification NOW. Any notify_all() occurring after this call
    /// returns is guaranteed to be observed by the returned Subscription.
    pub fn subscribe(&self) -> Subscription;

    /// Core loop:
    ///   loop { let sub = self.subscribe(); if let Some(s) = check() { return s; } sub.await; }
    /// `check` returns Some(terminal_state) if the work is completed/cancelled, otherwise None.
    /// Subscription happens before each check, so no notification is lost.
    pub async fn wait_for_terminal<S, F>(&self, check: F) -> S
    where
        F: FnMut() -> Option<S>;
}

/// Registered interest. Awaiting it resolves once a notify_all() has occurred since subscribe().
/// Send + Sync + Unpin. Dropping it, even while pending, deregisters it and leaks nothing.
#[must_use]
pub struct Subscription { /* .. */ }

impl Future for Subscription {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>;
}
```
The returned future of `wait_for_terminal` is `Send` whenever `F` and `S` are `Send`. No error type is needed because no operation can fail.

## 4. Behavior and invariants
- **No lost wakeups:** If `notify_all()` happens at any time after `subscribe()` returned, awaiting that `Subscription` completes. This holds even if the notify happens before the first poll, so a Subscription never needs to be polled before the notify.
- **Notifications before `subscribe()`** do not affect the new Subscription. The state check covers them.
- **Multiple waiters:** One `notify_all()` wakes every pending Subscription and every pending `wait_for_terminal` call, with none left out. Waiters are independent, and dropping or cancelling one does not affect the others.
- **Already terminal:** If `check()` returns `Some` on the first call, `wait_for_terminal` returns without ever suspending (it is ready on the first poll).
- **Spurious or non-terminal wakeups:** If a wakeup arrives but `check()` returns `None`, `wait_for_terminal` subscribes again and re-checks. It then waits again and never returns without `Some`.
- **Waker handling:** `poll` stores or updates the latest `Waker`, so re-polling with a different waker wakes the new one. Polling after completion returns `Ready(())` again.
- **Non-blocking:** `notify_all` and `poll` hold the internal lock only briefly. They never call wakers while holding the lock, and never block waiting for other tasks. Handle a poisoned mutex by continuing with its inner value (no panic).
- **Cancel safety:** Dropping a `wait_for_terminal` future at any await point leaves the notifier consistent and leaks no waker registrations. `check` is called only inside the poll of that future, never on a separate thread.
- **Notifier lifetime:** A Subscription keeps the shared state alive (Arc) and stays valid if all `CompletionNotifier` handles are dropped. It then simply stays pending forever unless notified earlier.
- **Ordering contract for the owner:** The owner must make the terminal state visible to `check()` (state write happens-before `notify_all()`). A waiter that is woken must then see that state.

## 5. Relied-on elements
None (no dependencies).