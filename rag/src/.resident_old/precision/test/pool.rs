use super::*;

#[test]
fn jobs_return_their_result() {
    let pool = LowPriorityPool::new("test-pool", 2).unwrap();
    let handles: Vec<_> = (0..8).map(|i| pool.submit(move || i * 2)).collect();
    let got: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(got, (0..8).map(|i| i * 2).collect::<Vec<_>>());
}

#[test]
fn a_panicking_job_does_not_kill_the_worker() {
    let pool = LowPriorityPool::new("test-pool", 1).unwrap();
    let bad = pool.submit(|| -> i32 { panic!("boom") });
    assert!(bad.join().is_none());
    assert_eq!(pool.submit(|| 7).join(), Some(7));
}

#[test]
fn workers_run_apart_from_the_caller() {
    let pool = LowPriorityPool::new("test-pool", 1).unwrap();
    let caller = std::thread::current().id();
    let worker = pool.submit(|| std::thread::current().id()).join().unwrap();
    assert_ne!(caller, worker);
}

#[cfg(target_os = "linux")]
#[test]
fn workers_run_at_lowest_priority() {
    let pool = LowPriorityPool::new("test-pool", 1).unwrap();
    let nice = pool
        .submit(|| unsafe {
            let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
            libc::getpriority(libc::PRIO_PROCESS, tid)
        })
        .join()
        .unwrap();
    assert_eq!(nice, 19);
}

#[cfg(target_os = "linux")]
#[test]
fn lowering_priority_is_scoped_to_the_calling_thread() {
    let before = unsafe { libc::getpriority(libc::PRIO_PROCESS, 0) };
    std::thread::spawn(|| assert!(lower_current_thread_priority())).join().unwrap();
    let after = unsafe { libc::getpriority(libc::PRIO_PROCESS, 0) };
    assert_eq!(before, after);
}

#[test]
fn dropping_the_pool_waits_for_queued_jobs() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    let done = Arc::new(AtomicU32::new(0));
    {
        let pool = LowPriorityPool::new("test-pool", 1).unwrap();
        for _ in 0..5 {
            let done = done.clone();
            drop(pool.submit(move || {
                done.fetch_add(1, Ordering::SeqCst);
            }));
        }
    }
    assert_eq!(done.load(Ordering::SeqCst), 5);
}
