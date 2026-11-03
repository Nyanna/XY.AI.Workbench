use super::*;
use std::collections::HashSet;
use std::sync::Mutex;
use std::thread;

type Q = Queue<u32, u32, u32, String>;

#[test]
fn fifo_single_consumer() {
    let q = Q::new();
    for i in 0..100 {
        q.push(i, i * 2);
    }
    assert_eq!(q.len(), 100);
    for i in 0..100 {
        let mut c = q.take().unwrap();
        assert_eq!(*c.meta(), i);
        assert_eq!(c.take_payload(), Some(i * 2));
        assert_eq!(c.take_payload(), None);
        c.complete(Ok(i));
    }
    assert!(q.take().is_none());
    assert!(q.is_empty());
}

#[test]
fn take_where_skips_without_touching() {
    let q = Q::new();
    let t0 = q.push(1, 0);
    let t1 = q.push(2, 0);
    let t2 = q.push(2, 0);
    let c = q.take_where(|m| *m == 2).unwrap();
    assert!(Arc::ptr_eq(&c.ticket().item, &t1.item));
    assert_eq!(t0.state(), State::Pending);
    assert_eq!(t2.state(), State::Pending);
    assert!(q.take_where(|m| *m == 9).is_none());
    assert_eq!(q.len(), 2);
    let c0 = q.take().unwrap();
    assert_eq!(*c0.meta(), 1);
}

#[test]
fn result_lifecycle() {
    let q = Q::new();
    let t = q.push(0, 0);
    assert_eq!(t.take_result(), Err(ResultError::NotReady));
    let c = q.take().unwrap();
    assert_eq!(t.state(), State::Running);
    c.complete(Ok(7));
    assert_eq!(t.state(), State::Succeeded);
    assert_eq!(t.take_result(), Ok(Ok(7)));
    assert_eq!(t.take_result(), Err(ResultError::Taken));

    let t = q.push(0, 0);
    q.take().unwrap().complete(Err("bad".to_string()));
    assert_eq!(t.state(), State::Failed);
    assert_eq!(t.take_result(), Ok(Err("bad".to_string())));
}

#[test]
fn dropped_claim_is_abandoned() {
    let q = Q::new();
    let t = q.push(0, 0);
    let e = q.epoch(Channel::Completed);
    drop(q.take().unwrap());
    assert_eq!(t.state(), State::Abandoned);
    assert_eq!(t.take_result(), Err(ResultError::Abandoned));
    assert_ne!(q.epoch(Channel::Completed), e);
}

#[test]
fn queue_drop_abandons_pending() {
    let q = Q::new();
    let t = q.push(5, 0);
    drop(q);
    assert_eq!(t.wait(Some(Duration::from_millis(10))), State::Abandoned);
    assert_eq!(*t.meta(), 5);
}

#[test]
fn ticket_wait_wakes() {
    let q = Arc::new(Q::new());
    let t = q.push(0, 0);
    assert_eq!(t.wait(Some(Duration::ZERO)), State::Pending);
    let q2 = q.clone();
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        q2.take().unwrap().complete(Ok(1));
    });
    assert_eq!(t.wait(None), State::Succeeded);
    h.join().unwrap();
}

#[test]
fn wait_change_no_lost_wakeup() {
    let q = Arc::new(Q::new());
    let seen = q.epoch(Channel::Available);
    assert_eq!(q.wait_change(Channel::Available, seen, Some(Duration::ZERO)), seen);
    let q2 = q.clone();
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        q2.push(0, 0);
    });
    let now = q.wait_change(Channel::Available, seen, None);
    assert_ne!(now, seen);
    h.join().unwrap();
}

#[test]
fn predicate_panic_keeps_item_pending() {
    let q = Q::new();
    let t = q.push(0, 0);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        q.take_where(|_| panic!("boom"));
    }));
    assert!(r.is_err());
    assert_eq!(t.state(), State::Pending);
    assert!(q.take().is_some());
}

#[test]
fn concurrent_no_duplicates_no_loss() {
    const PRODUCERS: u32 = 4;
    const PER: u32 = 5000;
    let q = Arc::new(Q::new());
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = Vec::new();
    for p in 0..PRODUCERS {
        let q = q.clone();
        handles.push(thread::spawn(move || {
            for i in 0..PER {
                q.push(p * PER + i, 0);
            }
        }));
    }
    let total = (PRODUCERS * PER) as usize;
    let mut consumers = Vec::new();
    for _ in 0..4 {
        let q = q.clone();
        let seen = seen.clone();
        consumers.push(thread::spawn(move || {
            let mut local = Vec::new();
            loop {
                if seen.lock().unwrap().len() + local.len() >= total && q.is_empty() {
                    break;
                }
                match q.take() {
                    Some(c) => {
                        local.push(*c.meta());
                        c.complete(Ok(0));
                    }
                    None => thread::yield_now(),
                }
                if local.len() % 256 == 0 {
                    let mut s = seen.lock().unwrap();
                    for v in local.drain(..) {
                        assert!(s.insert(v), "duplicate {v}");
                    }
                }
            }
            let mut s = seen.lock().unwrap();
            for v in local.drain(..) {
                assert!(s.insert(v), "duplicate {v}");
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    for c in consumers {
        c.join().unwrap();
    }
    assert_eq!(seen.lock().unwrap().len(), total);
}
