use super::*;

#[test]
fn monotonic_clock_does_not_decrease() {
    let clock = MonotonicClock::new();
    let mut last = clock.now_ns();
    for _ in 0..1000 {
        let now = clock.now_ns();
        assert!(now >= last);
        last = now;
    }
}

#[test]
fn never_pending_is_false() {
    assert!(!NeverPending.productive_waiting());
}

#[test]
fn pending_flag_clones_share_state() {
    let flag = PendingFlag::new();
    assert!(!flag.get());
    let other = flag.clone();
    other.set(true);
    assert!(flag.get());
    assert!(flag.productive_waiting());
    flag.set(false);
    assert!(!other.get());
}

#[test]
fn default_hooks_never_interrupt() {
    let hooks = Hooks::default();
    assert!(!hooks.pending.productive_waiting());
    let a = hooks.clock.now_ns();
    assert!(hooks.clock.now_ns() >= a);
}
