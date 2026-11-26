use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn counter() -> (Arc<AtomicUsize>, Wake) {
    let count = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&count);
    let wake: Wake = Arc::new(move || {
        c.fetch_add(1, Ordering::SeqCst);
    });
    (count, wake)
}

#[test]
fn creates_requested_handles() {
    let (_, wake) = counter();
    let topo = NotificationTopology::new(3, wake);
    assert_eq!(topo.len(), 3);
    assert!(topo.notifier(2).is_some());
    assert!(topo.notifier(3).is_none());
}

#[test]
fn zero_subengines_still_wakes_on_push() {
    let (count, wake) = counter();
    let topo = NotificationTopology::new(0, wake);
    assert_eq!(topo.len(), 0);
    assert!(topo.is_empty());
    topo.notify_push();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn each_event_wakes_once() {
    let (count, wake) = counter();
    let topo = NotificationTopology::new(2, wake);
    let n = topo.notifier(1).unwrap();
    n.notify_result();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    n.notify_init_done();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    n.notify_eviction_timer();
    assert_eq!(count.load(Ordering::SeqCst), 3);
    topo.notify_push();
    assert_eq!(count.load(Ordering::SeqCst), 4);
}

#[test]
fn iteration_wakes_only_with_progress() {
    let (count, wake) = counter();
    let topo = NotificationTopology::new(1, wake);
    let n = topo.notifier(0).unwrap();
    n.notify_iteration(false);
    assert_eq!(count.load(Ordering::SeqCst), 0);
    n.notify_iteration(true);
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn concurrent_notifications_are_not_lost() {
    let (count, wake) = counter();
    let topo = Arc::new(NotificationTopology::new(4, wake));
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let topo = Arc::clone(&topo);
            std::thread::spawn(move || {
                let n = topo.notifier(i).unwrap();
                for _ in 0..100 {
                    n.notify_result();
                    topo.notify_push();
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(count.load(Ordering::SeqCst), 800);
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn types_are_send_sync() {
    assert_send_sync::<SubengineNotifier>();
    assert_send_sync::<NotificationTopology>();
}
