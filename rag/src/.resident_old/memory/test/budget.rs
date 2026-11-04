use super::*;

#[test]
fn kind_ids_roundtrip() {
    for k in ArenaKind::ALL {
        assert_eq!(ArenaKind::from_id(k.id()), Some(k));
    }
    assert_eq!(ArenaKind::from_id(ArenaId(99)), None);
}

#[test]
fn generation_reserve_holds_back_inactive_generations() {
    let s = ArenaSizes { index_segment: 1000, ..Default::default() };
    assert_eq!(s.generation_reserve(2), 500);
    assert_eq!(s.generation_reserve(1), 0);
    assert_eq!(s.generation_reserve(0), 0);
}

#[test]
fn staging_does_not_count_against_device() {
    let s = ArenaSizes { weights: 10, staging: 1 << 40, ..Default::default() };
    assert_eq!(s.device_total(), 10);
}

#[test]
fn fit_respects_headroom() {
    let s = ArenaSizes { weights: 90, ..Default::default() };
    assert!(check_fit(&s, 100, 10).is_ok());
    assert!(matches!(
        check_fit(&s, 100, 11),
        Err(MemoryError::BudgetExceeded { requested: 90, usable: 89 })
    ));
}

#[test]
fn usable_subtracts_reserve() {
    let b = Budget { free: 100, total: 200, reserved_for_generations: 150 };
    assert_eq!(b.usable(), 0);
}
