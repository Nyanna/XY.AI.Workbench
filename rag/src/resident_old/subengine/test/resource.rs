use super::*;
#[test]
fn ewma_primes_then_smooths() {
    let mut e = Ewma::new(0.5);
    assert_eq!(e.get(), None);
    assert_eq!(e.observe(10.0), 10.0);
    assert_eq!(e.observe(20.0), 15.0);
}
#[test]
fn batch_size_is_clamped_and_starts_at_min() {
    let c = PullController::new(PullConfig {
        min_batch: 2,
        max_batch: 8,
        ..Default::default()
    });
    assert_eq!(c.batch_size(), 2);
    c.observe(1000, 1.0);
    assert_eq!(c.batch_size(), 8);
    let slow = PullController::new(PullConfig {
        min_batch: 2,
        max_batch: 8,
        ..Default::default()
    });
    slow.observe(1, 1000.0);
    assert_eq!(slow.batch_size(), 2);
}
#[test]
fn noisy_samples_do_not_oscillate_batch_size() {
    let c = PullController::new(PullConfig {
        target_batch_ms: 100.0,
        min_batch: 1,
        max_batch: 256,
        ..Default::default()
    });
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut sizes = Vec::new();
    for _ in 0..200 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let jitter = 0.8 + (state % 400) as f32 / 1000.0;
        c.observe(10, 10.0 * jitter);
        sizes.push(c.batch_size() as i64);
    }
    let tail = &sizes[100..];
    let spread = tail.iter().max().unwrap() - tail.iter().min().unwrap();
    assert!(spread <= 40, "spread {spread}");
    assert!(c.throughput() > 0.0);
}
#[test]
fn capacity_free_saturates() {
    assert_eq!(Capacity { in_flight : 5, max_in_flight : 3 } .free(), 0);
    assert_eq!(Capacity { in_flight : 1, max_in_flight : 3 } .free(), 2);
}
#[test]
fn coalesce_groups_in_first_seen_order() {
    let g = coalesce([("a", 1), ("b", 2), ("a", 3)], |x| x.0);
    assert_eq!(g, vec![("a", vec![("a", 1), ("a", 3)]), ("b", vec![("b", 2)])]);
}
