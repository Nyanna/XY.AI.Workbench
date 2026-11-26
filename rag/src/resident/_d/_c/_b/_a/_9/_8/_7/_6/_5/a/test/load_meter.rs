use super::*;
use std::sync::Arc;
use std::thread;
fn meter() -> LoadMeter {
    LoadMeter::new(ResourceKind::Encoder, DeviceHandle::Gpu(1), 8)
}
fn secs(s: u64) -> Duration {
    Duration::from_secs(s)
}
#[test]
fn fresh_meter_reports_initial_state() {
    let m = meter();
    let r = m.report();
    assert_eq!(r.kind, ResourceKind::Encoder);
    assert_eq!(r.device, DeviceHandle::Gpu(1));
    assert_eq!(r.capacity, 8);
    assert_eq!(r.throughput, 0.0);
    assert_eq!(m.kind(), ResourceKind::Encoder);
    assert_eq!(m.device(), DeviceHandle::Gpu(1));
}
#[test]
fn capacity_is_replaced_and_untouched_by_record() {
    let m = meter();
    m.set_capacity(0);
    assert_eq!(m.capacity(), 0);
    m.record(100, secs(1));
    assert_eq!(m.capacity(), 0);
    m.set_capacity(42);
    assert_eq!(m.report().capacity, 42);
}
#[test]
fn first_sample_sets_throughput_then_smooths() {
    let m = meter();
    m.record(100, secs(1));
    assert_eq!(m.throughput(), 100.0);
    m.record(200, secs(1));
    let expected = 0.2 * 200.0 + 0.8 * 100.0;
    assert!((m.throughput() - expected).abs() < 1e-9);
}
#[test]
fn zero_elapsed_is_ignored() {
    let m = meter();
    m.record(100, Duration::ZERO);
    assert_eq!(m.throughput(), 0.0);
    m.record(10, secs(1));
    m.record(100, Duration::ZERO);
    assert_eq!(m.throughput(), 10.0);
}
#[test]
fn zero_entries_is_valid_sample() {
    let m = meter();
    m.record(0, secs(1));
    assert_eq!(m.throughput(), 0.0);
    m.record(100, secs(1));
    assert_eq!(m.throughput(), 100.0 * 0.2 + 0.0);
    m.record(0, secs(1));
    assert!((m.throughput() - 16.0).abs() < 1e-9);
}
#[test]
fn zero_sample_first_still_seeds() {
    let m = meter();
    m.record(0, secs(1));
    m.record(100, secs(1));
    assert!((m.throughput() - 20.0).abs() < 1e-9);
}
#[test]
fn alpha_is_clamped() {
    let k = ResourceKind::Scorer;
    let d = DeviceHandle::Cpu;
    let hi = LoadMeter::with_alpha(k, d, 1, 5.0);
    hi.record(10, secs(1));
    hi.record(30, secs(1));
    assert_eq!(hi.throughput(), 30.0);
    let lo = LoadMeter::with_alpha(k, d, 1, -1.0);
    lo.record(100, secs(1));
    lo.record(0, secs(1));
    assert!((lo.throughput() - 99.0).abs() < 1e-9);
    let nan = LoadMeter::with_alpha(k, d, 1, f64::NAN);
    nan.record(100, secs(1));
    nan.record(200, secs(1));
    assert!((nan.throughput() - 120.0).abs() < 1e-9);
    let inf = LoadMeter::with_alpha(k, d, 1, f64::INFINITY);
    inf.record(10, secs(1));
    inf.record(30, secs(1));
    assert_eq!(inf.throughput(), 30.0);
}
#[test]
fn extreme_inputs_stay_finite() {
    let m = meter();
    m.record(usize::MAX, Duration::from_nanos(1));
    assert!(m.throughput().is_finite() && m.throughput() > 0.0);
    m.record(usize::MAX, Duration::MAX);
    m.record(0, Duration::MAX);
    m.record(usize::MAX, Duration::from_nanos(1));
    let t = m.report().throughput;
    assert!(t.is_finite() && t >= 0.0);
}
#[test]
fn no_decay_without_records() {
    let m = meter();
    m.record(50, secs(1));
    thread::sleep(Duration::from_millis(5));
    assert_eq!(m.throughput(), 50.0);
}
#[test]
fn trait_object_reports() {
    let m: Arc<dyn LoadReporting> = Arc::new(meter());
    assert_eq!(m.report().capacity, 8);
}
#[test]
fn poisoned_lock_is_recovered() {
    let m = Arc::new(meter());
    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || {
            let _guard = m2.state.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert!(m.state.is_poisoned());
    m.set_capacity(3);
    m.record(10, secs(1));
    let r = m.report();
    assert_eq!(r.capacity, 3);
    assert_eq!(r.throughput, 10.0);
}
#[test]
fn concurrent_records_all_count() {
    let m = Arc::new(
        LoadMeter::with_alpha(ResourceKind::Index, DeviceHandle::Cpu, 0, 0.5),
    );
    let mut handles = Vec::new();
    for i in 0..8usize {
        let m = Arc::clone(&m);
        handles
            .push(
                thread::spawn(move || {
                    for _ in 0..1000 {
                        m.record(100, secs(1));
                        m.set_capacity(i);
                        let r = m.report();
                        assert!(r.throughput.is_finite() && r.throughput >= 0.0);
                        assert!(r.capacity < 8);
                    }
                }),
            );
    }
    for h in handles {
        h.join().unwrap();
    }
    assert!((m.throughput() - 100.0).abs() < 1e-9);
}
