use super::*;
use std::thread;

#[test]
fn new_defaults() {
    let c = TaskContext::<String>::new(7, -3, 2);
    assert_eq!(c.id(), 7);
    assert_eq!(c.priority(), -3);
    assert_eq!(c.open_operations(), 2);
    assert!(!c.is_cancelled());
    assert!(c.first_error().is_none());
}

#[test]
fn last_completion_returns_true_once() {
    let c = TaskContext::<String>::new(1, 0, 1);
    c.register_follow_ups(0);
    c.register_follow_ups(2);
    assert_eq!(c.open_operations(), 3);
    assert!(!c.complete_operation());
    assert!(!c.complete_operation());
    assert!(c.complete_operation());
    assert_eq!(c.open_operations(), 0);
}

#[test]
fn cancelled_never_completes() {
    let c = TaskContext::<String>::new(1, 0, 2);
    assert!(c.fail("a".into()));
    assert!(!c.fail("b".into()));
    assert!(c.is_cancelled());
    assert_eq!(c.first_error().map(String::as_str), Some("a"));
    c.register_follow_ups(1);
    assert!(!c.complete_operation());
    assert!(!c.complete_operation());
    assert!(!c.complete_operation());
    assert_eq!(c.open_operations(), 0);
}

#[test]
fn fail_after_success_still_true() {
    let c = TaskContext::<String>::new(1, 0, 1);
    assert!(c.complete_operation());
    assert!(c.fail("late".into()));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn underflow_panics_in_debug() {
    let c = TaskContext::<String>::new(1, 0, 0);
    c.complete_operation();
}

#[test]
#[cfg(not(debug_assertions))]
fn underflow_returns_false_in_release() {
    let c = TaskContext::<String>::new(1, 0, 0);
    assert!(!c.complete_operation());
    assert_eq!(c.open_operations(), 0);
}

#[test]
fn concurrent_completion_single_winner() {
    let n = 64;
    let c = TaskContext::<String>::new(1, 0, n);
    let handles: Vec<_> = (0..n)
        .map(|_| {
            let c = Arc::clone(&c);
            thread::spawn(move || c.complete_operation())
        })
        .collect();
    let wins = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|b| *b)
        .count();
    assert_eq!(wins, 1);
}

#[test]
fn concurrent_fail_single_winner() {
    let c = TaskContext::<usize>::new(1, 0, 1);
    let handles: Vec<_> = (0..16)
        .map(|i| {
            let c = Arc::clone(&c);
            thread::spawn(move || c.fail(i))
        })
        .collect();
    let wins = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|b| *b)
        .count();
    assert_eq!(wins, 1);
    assert!(c.is_cancelled());
    assert!(c.first_error().is_some());
}
