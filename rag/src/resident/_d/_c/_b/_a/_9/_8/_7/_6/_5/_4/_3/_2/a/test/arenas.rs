use super::*;
use std::sync::Arc;
struct Mock(Result<MemInfo, String>);
impl MemoryQuery for Mock {
    fn query(&self) -> Result<MemInfo, String> {
        self.0.clone()
    }
}
fn budget(free: u64, reserve: u64) -> Budget {
    Budget {
        free,
        total: free.saturating_mul(2),
        generation_reserve: reserve,
    }
}
fn caps(w: u64, i: u64, ws: u64, st: u64, pb: u64, mh: u64) -> ArenaCapacities {
    let mut c = ArenaCapacities::default();
    c.set(ArenaKind::Weights, w);
    c.set(ArenaKind::IndexSegments, i);
    c.set(ArenaKind::Workspace, ws);
    c.set(ArenaKind::Staging, st);
    c.set(ArenaKind::ParameterBlock, pb);
    c.set(ArenaKind::MappedHost, mh);
    c
}
#[test]
fn ids_and_list() {
    assert_eq!(ArenaKind::ALL.len(), 6);
    for (i, k) in ArenaKind::ALL.iter().enumerate() {
        assert_eq!(k.id(), i as u32);
        assert_eq!(ArenaKind::from_id(k.id()), Some(* k));
    }
    assert_eq!(ArenaKind::ALL[0], ArenaKind::Weights);
    assert_eq!(ArenaKind::ALL[5], ArenaKind::MappedHost);
    assert_eq!(ArenaKind::from_id(6), None);
    assert_eq!(ArenaKind::from_id(u32::MAX), None);
}
#[test]
fn device_classification() {
    let dev: Vec<_> = ArenaKind::ALL.iter().filter(|k| k.is_device()).collect();
    assert_eq!(
        dev, vec![& ArenaKind::Weights, & ArenaKind::IndexSegments, &
        ArenaKind::Workspace, & ArenaKind::ParameterBlock]
    );
    assert!(! ArenaKind::Staging.is_device());
    assert!(! ArenaKind::MappedHost.is_device());
}
#[test]
fn default_capacities_zero() {
    let c = ArenaCapacities::default();
    for k in ArenaKind::ALL {
        assert_eq!(c.get(k), 0);
    }
    assert_eq!(c.device_need(), 0);
}
#[test]
fn device_need_ignores_host_and_saturates() {
    assert_eq!(caps(1, 2, 3, 100, 4, 200).device_need(), 10);
    assert_eq!(caps(u64::MAX, 1, 1, 0, 1, 0).device_need(), u64::MAX);
    assert_eq!(
        caps(u64::MAX, u64::MAX, u64::MAX, 0, u64::MAX, 0).device_need(), u64::MAX
    );
}
#[test]
fn generation_reserve_values() {
    let c = caps(0, 900, 0, 0, 0, 0);
    assert_eq!(c.generation_reserve(0), 0);
    assert_eq!(c.generation_reserve(1), 0);
    assert_eq!(c.generation_reserve(3), 600);
    assert_eq!(c.generation_reserve(2), 450);
    assert_eq!(caps(0, 10, 0, 0, 0, 0).generation_reserve(3), 7);
    assert_eq!(
        caps(0, u64::MAX, 0, 0, 0, 0).generation_reserve(u32::MAX) <= u64::MAX, true
    );
    for g in 0..20 {
        assert!(c.generation_reserve(g) <= 900);
    }
}
#[test]
fn budget_usable() {
    assert_eq!(budget(100, 30).usable(), 70);
    assert_eq!(budget(100, 100).usable(), 0);
    assert_eq!(budget(10, 100).usable(), 0);
}
#[test]
fn budget_from_query() {
    let ok = Mock(Ok(MemInfo { free: 5, total: 9 }));
    let b = Budget::from_query(&ok, 2).unwrap();
    assert_eq!(b, Budget { free : 5, total : 9, generation_reserve : 2 });
    let bad = Mock(Err("driver lost".to_string()));
    assert_eq!(
        Budget::from_query(& bad, 0), Err(BudgetError::MemoryQueryFailed { reason :
        "driver lost".to_string() })
    );
}
#[test]
fn fit_check_boundaries() {
    let c = caps(10, 20, 30, 999, 40, 999);
    assert!(fit_check(& c, 600, 500).is_ok());
    assert!(fit_check(& c, 100, 0).is_ok());
    assert_eq!(
        fit_check(& c, 99, 0), Err(BudgetError::DoesNotFit { requested : 100, usable : 99
        })
    );
    assert_eq!(
        fit_check(& c, 100, 1), Err(BudgetError::DoesNotFit { requested : 100, usable :
        99 })
    );
}
#[test]
fn fit_check_headroom_exceeds_free() {
    assert!(fit_check(& ArenaCapacities::default(), 10, 20).is_ok());
    assert_eq!(
        fit_check(& caps(1, 0, 0, 0, 0, 0), 10, 20), Err(BudgetError::DoesNotFit {
        requested : 1, usable : 0 })
    );
}
#[test]
fn cow_extra_is_added() {
    let c = caps(10, 20, 0, 0, 0, 0);
    assert!(super::fit_check_cow(& c, 5, 35, 0).is_ok());
    assert_eq!(
        super::fit_check_cow(& c, 6, 35, 0), Err(BudgetError::DoesNotFit { requested :
        36, usable : 35 })
    );
    assert_eq!(
        super::fit_check_cow(& c, u64::MAX, 100, 10), Err(BudgetError::DoesNotFit {
        requested : u64::MAX, usable : 90 })
    );
}
#[test]
fn default_headroom_value() {
    assert_eq!(DEFAULT_HEADROOM, 512 * 1024 * 1024);
}
#[test]
fn error_display() {
    let e = BudgetError::MemoryQueryFailed {
        reason: "x".into(),
    };
    assert!(e.to_string().contains('x'));
    let e = BudgetError::DoesNotFit {
        requested: 7,
        usable: 3,
    };
    let s = e.to_string();
    assert!(s.contains('7') && s.contains('3'));
    let _: &dyn std::error::Error = &e;
}
#[test]
fn arena_contains() {
    let a = Arena {
        kind: ArenaKind::Weights,
        capacity: 100,
    };
    assert!(a.contains(0, 100));
    assert!(a.contains(100, 0));
    assert!(! a.contains(0, 101));
    assert!(! a.contains(101, 0));
    assert!(! a.contains(u64::MAX, 2));
    assert!(! a.contains(1, u64::MAX));
    let z = Arena {
        kind: ArenaKind::Staging,
        capacity: 0,
    };
    assert!(z.contains(0, 0));
    assert!(! z.contains(0, 1));
}
#[test]
fn lazy_creation_and_idempotence() {
    let set = ArenaSet::new(caps(10, 20, 0, 5, 0, 0));
    assert_eq!(set.get(ArenaKind::Weights), None);
    let b = budget(1000, 0);
    let a = set.ensure(ArenaKind::Weights, &b).unwrap();
    assert_eq!(a, Arena { kind : ArenaKind::Weights, capacity : 10 });
    for k in ArenaKind::ALL {
        assert_eq!(set.get(k).is_some(), k == ArenaKind::Weights);
    }
    set.set_capacity(ArenaKind::Weights, 500);
    assert_eq!(set.capacities().get(ArenaKind::Weights), 500);
    let tight = budget(0, 0);
    assert_eq!(set.ensure(ArenaKind::Weights, & tight).unwrap(), a);
    assert_eq!(set.get(ArenaKind::Weights), Some(a));
}
#[test]
fn device_budget_check_and_retry() {
    let set = ArenaSet::new(caps(0, 100, 0, 0, 0, 0));
    let b = budget(100, 40);
    assert_eq!(
        set.ensure(ArenaKind::IndexSegments, & b), Err(BudgetError::DoesNotFit {
        requested : 100, usable : 60 })
    );
    assert_eq!(set.get(ArenaKind::IndexSegments), None);
    set.set_capacity(ArenaKind::IndexSegments, 60);
    let a = set.ensure(ArenaKind::IndexSegments, &b).unwrap();
    assert_eq!(a.capacity, 60);
}
#[test]
fn host_arenas_skip_budget() {
    let set = ArenaSet::new(caps(0, 0, 0, 1 << 40, 0, 1 << 41));
    let b = budget(0, 0);
    assert_eq!(set.ensure(ArenaKind::Staging, & b).unwrap().capacity, 1 << 40);
    assert_eq!(set.ensure(ArenaKind::MappedHost, & b).unwrap().capacity, 1 << 41);
}
#[test]
fn zero_capacity_arena() {
    let set = ArenaSet::new(ArenaCapacities::default());
    let a = set.ensure(ArenaKind::Workspace, &budget(0, 0)).unwrap();
    assert_eq!(a.capacity, 0);
    assert!(a.contains(0, 0));
}
#[test]
fn concurrent_ensure_creates_once() {
    let set = Arc::new(ArenaSet::new(caps(0, 0, 64, 0, 0, 0)));
    let handles: Vec<_> = (0..8)
        .map(|i| {
            let set = Arc::clone(&set);
            std::thread::spawn(move || {
                set.ensure(ArenaKind::Workspace, &budget(1000, 0)).unwrap();
                set.set_capacity(ArenaKind::Workspace, 100 + i);
                set.ensure(ArenaKind::Workspace, &budget(0, 0)).unwrap()
            })
        })
        .collect();
    let results: Vec<Arena> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(results.iter().all(| a | a.capacity == 64));
}
#[test]
fn send_sync() {
    fn assert_ss<T: Send + Sync>() {}
    assert_ss::<ArenaSet>();
}
