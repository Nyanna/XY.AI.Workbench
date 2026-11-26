use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::task::Wake;
use std::thread;

struct CountWaker(AtomicUsize);

impl Wake for CountWaker {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn counting() -> (Arc<CountWaker>, Waker) {
    let c = Arc::new(CountWaker(AtomicUsize::new(0)));
    (c.clone(), Waker::from(c))
}

struct ThreadWaker(thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = Box::pin(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
        thread::park();
    }
}

#[test]
fn notify_before_first_poll_is_observed() {
    let n = CompletionNotifier::new();
    let mut sub = n.subscribe();
    n.notify_all();
    let (_, w) = counting();
    let mut cx = Context::from_waker(&w);
    assert_eq!(Pin::new(&mut sub).poll(&mut cx), Poll::Ready(()));
    assert_eq!(Pin::new(&mut sub).poll(&mut cx), Poll::Ready(()));
}

#[test]
fn notify_before_subscribe_is_ignored() {
    let n = CompletionNotifier::new();
    n.notify_all();
    let mut sub = n.subscribe();
    let (_, w) = counting();
    let mut cx = Context::from_waker(&w);
    assert_eq!(Pin::new(&mut sub).poll(&mut cx), Poll::Pending);
}

#[test]
fn wakes_all_waiters_and_latest_waker() {
    let n = CompletionNotifier::new();
    let mut a = n.subscribe();
    let mut b = n.subscribe();
    let (ca, wa) = counting();
    let (ca2, wa2) = counting();
    let (cb, wb) = counting();
    assert_eq!(Pin::new(&mut a).poll(&mut Context::from_waker(&wa)), Poll::Pending);
    assert_eq!(Pin::new(&mut a).poll(&mut Context::from_waker(&wa2)), Poll::Pending);
    assert_eq!(Pin::new(&mut b).poll(&mut Context::from_waker(&wb)), Poll::Pending);
    n.notify_all();
    assert_eq!(ca.0.load(Ordering::SeqCst), 0);
    assert_eq!(ca2.0.load(Ordering::SeqCst), 1);
    assert_eq!(cb.0.load(Ordering::SeqCst), 1);
}

#[test]
fn drop_deregisters() {
    let n = CompletionNotifier::new();
    let mut a = n.subscribe();
    let (_, w) = counting();
    let _ = Pin::new(&mut a).poll(&mut Context::from_waker(&w));
    drop(a);
    assert!(n.shared.lock().slots.is_empty());
}

#[test]
fn already_terminal_is_ready_immediately() {
    let n = CompletionNotifier::new();
    let mut fut = Box::pin(n.wait_for_terminal(|| Some(7)));
    let (_, w) = counting();
    assert_eq!(fut.as_mut().poll(&mut Context::from_waker(&w)), Poll::Ready(7));
    drop(fut);
    assert!(n.shared.lock().slots.is_empty());
}

#[test]
fn spurious_wakeup_rechecks() {
    let n = CompletionNotifier::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let mut fut = Box::pin(n.wait_for_terminal(move || {
        if c.fetch_add(1, Ordering::SeqCst) >= 1 {
            Some(())
        } else {
            None
        }
    }));
    let (cw, w) = counting();
    let mut cx = Context::from_waker(&w);
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
    n.notify_all();
    assert_eq!(cw.0.load(Ordering::SeqCst), 1);
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(()));
}

#[test]
fn cross_thread_wait() {
    let n = CompletionNotifier::new();
    let flag = Arc::new(AtomicBool::new(false));
    let (n2, f2) = (n.clone(), flag.clone());
    let t = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(20));
        f2.store(true, Ordering::SeqCst);
        n2.notify_all();
    });
    let f = flag.clone();
    block_on(n.wait_for_terminal(move || f.load(Ordering::SeqCst).then_some(1)));
    t.join().unwrap();
}

#[test]
fn dropping_one_waiter_does_not_affect_others() {
    let n = CompletionNotifier::new();
    let mut a = n.subscribe();
    let mut b = n.subscribe();
    let (_, wa) = counting();
    let (cb, wb) = counting();
    let _ = Pin::new(&mut a).poll(&mut Context::from_waker(&wa));
    let _ = Pin::new(&mut b).poll(&mut Context::from_waker(&wb));
    drop(a);
    n.notify_all();
    assert_eq!(cb.0.load(Ordering::SeqCst), 1);
}

#[test]
fn subscription_outlives_notifier() {
    let n = CompletionNotifier::new();
    let mut sub = n.subscribe();
    drop(n);
    let (_, w) = counting();
    assert_eq!(Pin::new(&mut sub).poll(&mut Context::from_waker(&w)), Poll::Pending);
}

#[test]
fn auto_traits() {
    fn send_sync<T: Send + Sync + Unpin>() {}
    fn is_send<T: Send>(_: &T) {}
    send_sync::<CompletionNotifier>();
    send_sync::<Subscription>();
    let n = CompletionNotifier::new();
    is_send(&n.wait_for_terminal(|| Some(1u8)));
}
