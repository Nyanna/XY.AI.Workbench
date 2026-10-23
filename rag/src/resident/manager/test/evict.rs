use super::*;

const CAP: Occupancy = Occupancy { used: 0, capacity: 1000 };

fn used(n: u64) -> Occupancy {
    Occupancy { used: n, ..CAP }
}

fn policy() -> EvictPolicy {
    EvictPolicy::new(0.9, 0.7, Duration::from_secs(10))
}

fn filled(n: u32, base: Instant) -> Evictor<u32> {
    let mut e = Evictor::new(policy());
    for k in 0..n {
        e.insert(k, 100, Tier::Vector, 1.0, base);
    }
    e
}

#[test]
fn hysteresis_prevents_thrashing() {
    let base = Instant::now();
    let mut e = filled(10, base);
    assert!(e.relieve(used(900)).is_empty());
    assert!(e.relieve(used(800)).is_empty());

    let out = e.relieve(used(1000));
    assert_eq!(out.len(), 3);
    assert!(out.iter().all(|v| v.reason == EvictReason::Pressure));
    assert_eq!(e.tracked_bytes(), 700);

    assert!(e.relieve(used(700)).is_empty());
    assert!(e.relieve(used(850)).is_empty());
}

#[test]
fn summaries_are_never_victims() {
    let base = Instant::now();
    let mut e = Evictor::new(policy());
    e.insert(1u32, 500, Tier::Summary, 1000.0, base);
    e.insert(2u32, 100, Tier::Vector, 1.0, base);

    let out = e.relieve(used(1000));
    assert_eq!(out.iter().map(|v| v.key).collect::<Vec<_>>(), vec![2]);
    assert!(e.contains(&1));

    assert!(matches!(
        e.make_room(900, used(1000)),
        Err(ManagerError::ArenaExhausted { requested: 900, .. })
    ));
    assert!(e.expire(base + Duration::from_secs(10_000)).is_empty());
    assert!(e.contains(&1));
}

#[test]
fn pinned_entries_survive_until_released() {
    let base = Instant::now();
    let mut e = Evictor::new(policy());
    e.insert(1u32, 600, Tier::Vector, 1.0, base);
    let pin = e.pin(&1).unwrap();

    assert!(e.relieve(used(1000)).is_empty());
    assert!(e.expire(base + Duration::from_secs(60)).is_empty());
    assert!(e.make_room(500, used(1000)).is_err());
    assert!(e.contains(&1));

    drop(pin);
    assert_eq!(e.relieve(used(1000)).len(), 1);
}

#[test]
fn idle_expiry_follows_last_use() {
    let base = Instant::now();
    let at = |s| base + Duration::from_secs(s);
    let mut e = Evictor::new(policy());
    e.insert(1u32, 100, Tier::Vector, 1.0, base);
    e.insert(2u32, 100, Tier::Vector, 1.0, base);

    assert!(e.expire(at(9)).is_empty());
    assert!(e.touch(&1, at(9)));
    let out = e.expire(at(15));
    assert_eq!(out.iter().map(|v| v.key).collect::<Vec<_>>(), vec![2]);
    assert_eq!(out[0].reason, EvictReason::Idle);
    assert!(e.expire(at(18)).is_empty());
    assert_eq!(e.expire(at(19)).len(), 1);
    assert!(e.is_empty());
}

#[test]
fn costly_reloads_outlive_cheap_ones() {
    let base = Instant::now();
    let mut e = Evictor::new(policy());
    e.insert(1u32, 100, Tier::Vector, 1.0, base);
    e.insert(2u32, 100, Tier::Vector, 50.0, base);
    e.insert(3u32, 100, Tier::Vector, 5.0, base);

    let out = e.make_room(150, used(1000)).unwrap();
    assert_eq!(out.iter().map(|v| v.key).collect::<Vec<_>>(), vec![1, 3]);
    assert!(out.iter().all(|v| v.reason == EvictReason::NeedSpace));
    assert!(e.contains(&2));
}

#[test]
fn size_counts_against_value() {
    let base = Instant::now();
    let mut e = Evictor::new(policy());
    e.insert(1u32, 400, Tier::Vector, 10.0, base);
    e.insert(2u32, 50, Tier::Vector, 10.0, base);

    let out = e.make_room(1, used(1000)).unwrap();
    assert_eq!(out[0].key, 1);
}

#[test]
fn frequent_use_raises_priority() {
    let base = Instant::now();
    let mut e = Evictor::new(policy());
    e.insert(1u32, 100, Tier::Vector, 1.0, base);
    e.insert(2u32, 100, Tier::Vector, 1.0, base);
    for _ in 0..5 {
        e.touch(&1, base);
    }
    let out = e.make_room(1, used(1000)).unwrap();
    assert_eq!(out[0].key, 2);
}

#[test]
fn make_room_is_all_or_nothing() {
    let base = Instant::now();
    let mut e = filled(3, base);
    assert!(e.make_room(10_000, used(1000)).is_err());
    assert_eq!(e.len(), 3);
    assert!(e.make_room(50, used(500)).unwrap().is_empty());
}

#[test]
fn tick_combines_expiry_and_pressure() {
    let base = Instant::now();
    let mut e = filled(10, base);
    e.touch(&9, base + Duration::from_secs(20));
    let out = e.tick(base + Duration::from_secs(20), used(1000));
    assert_eq!(out.len(), 9);
    assert!(out.iter().all(|v| v.reason == EvictReason::Idle));
    assert!(e.contains(&9));
}

#[test]
fn occupancy_excludes_reserve_and_headroom() {
    let b = ArenaBudget {
        capacity: 1000,
        reserve: 100,
        live: 300,
        retired: 50,
        cow_headroom: 200,
        available: 350,
    };
    assert_eq!(Occupancy::from_budget(&b), Occupancy { used: 350, capacity: 700 });
}

#[test]
fn policy_is_normalised() {
    let p = EvictPolicy::new(f64::NAN, 2.0, Duration::ZERO);
    assert!(p.low_watermark <= p.high_watermark && p.high_watermark <= 1.0);
    assert!(EvictPolicy::vram().idle_ttl > EvictPolicy::ram().idle_ttl);
}
