use super::*;
fn eid(n: u64) -> EntryId {
    EntryId(n)
}
#[test]
fn types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<AddrLayout>();
    check::<SlotMap>();
    check::<KernelHit>();
    check::<AddrLayoutError>();
}
#[test]
fn fixed_errors() {
    assert_eq!(FixedLayout::new(0, 0, 3), Err(AddrLayoutError::ZeroStride));
    assert_eq!(FixedLayout::new(u64::MAX - 5, 4, 2), Err(AddrLayoutError::Overflow));
    assert!(FixedLayout::new(u64::MAX - 8, 4, 2).is_ok());
}
#[test]
fn fixed_lookup() {
    let l = AddrLayout::Fixed(FixedLayout::new(100, 10, 3).unwrap());
    assert_eq!(l.len(), 3);
    assert_eq!(l.slot_of(Addr(99)), None);
    assert_eq!(l.slot_of(Addr(100)), Some(Slot5(0)));
    assert_eq!(l.slot_of(Addr(109)), Some(Slot5(0)));
    assert_eq!(l.slot_of(Addr(110)), Some(Slot5(1)));
    assert_eq!(l.slot_of(Addr(129)), Some(Slot5(2)));
    assert_eq!(l.slot_of(Addr(130)), None);
    assert_eq!(l.slot_range(Slot5(1)), Some(110..120));
    assert_eq!(l.slot_range(Slot5(3)), None);
}
#[test]
fn lengths_offsets_and_lookup() {
    let v = VariableLayout::from_lengths(10, &[4, 0, 0, 6, 0]).unwrap();
    assert_eq!(v.offsets().unwrap(), & [10, 14, 14, 14, 20, 20]);
    let l = AddrLayout::Variable(v);
    assert_eq!(l.len(), 5);
    assert_eq!(l.slot_of(Addr(9)), None);
    assert_eq!(l.slot_of(Addr(10)), Some(Slot5(0)));
    assert_eq!(l.slot_of(Addr(13)), Some(Slot5(0)));
    assert_eq!(l.slot_of(Addr(14)), Some(Slot5(3)));
    assert_eq!(l.slot_of(Addr(19)), Some(Slot5(3)));
    assert_eq!(l.slot_of(Addr(20)), None);
    assert_eq!(l.slot_range(Slot5(1)), Some(14..14));
    assert_eq!(l.slot_range(Slot5(5)), None);
}
#[test]
fn lengths_empty_and_overflow() {
    let v = VariableLayout::from_lengths(7, &[]).unwrap();
    assert_eq!(v.offsets().unwrap(), & [7]);
    assert!(AddrLayout::Variable(v).is_empty());
    assert_eq!(
        VariableLayout::from_lengths(u64::MAX, & [1]), Err(AddrLayoutError::Overflow)
    );
    assert_eq!(
        VariableLayout::from_lengths(0, & [u64::MAX, 1]), Err(AddrLayoutError::Overflow)
    );
}
#[test]
fn extents_lookup_in_input_order() {
    let v = VariableLayout::from_extents(
            &[
                Extent { start: 50, len: 10 },
                Extent { start: 0, len: 5 },
                Extent { start: 5, len: 0 },
                Extent { start: 5, len: 20 },
            ],
        )
        .unwrap();
    assert!(v.offsets().is_none());
    let l = AddrLayout::Variable(v);
    assert_eq!(l.len(), 4);
    assert_eq!(l.slot_of(Addr(0)), Some(Slot5(1)));
    assert_eq!(l.slot_of(Addr(5)), Some(Slot5(3)));
    assert_eq!(l.slot_of(Addr(24)), Some(Slot5(3)));
    assert_eq!(l.slot_of(Addr(25)), None);
    assert_eq!(l.slot_of(Addr(55)), Some(Slot5(0)));
    assert_eq!(l.slot_of(Addr(60)), None);
    assert_eq!(l.slot_range(Slot5(0)), Some(50..60));
    assert_eq!(l.slot_range(Slot5(2)), Some(5..5));
}
#[test]
fn extents_errors() {
    assert_eq!(
        VariableLayout::from_extents(& [Extent { start : 0, len : 10 }, Extent { start :
        9, len : 2 }]), Err(AddrLayoutError::Overlap)
    );
    assert_eq!(
        VariableLayout::from_extents(& [Extent { start : u64::MAX, len : 2 }]),
        Err(AddrLayoutError::Overflow)
    );
    assert!(
        VariableLayout::from_extents(& [Extent { start : 3, len : 0 }, Extent { start :
        0, len : 10 }]).is_ok()
    );
}
#[test]
fn choose_rules() {
    assert!(matches!(choose_layout(0, & [], 1000).unwrap(), AddrLayout::Variable(_)));
    assert!(
        matches!(choose_layout(0, & [0, 0], 1000).unwrap(), AddrLayout::Variable(_))
    );
    assert!(matches!(choose_layout(0, & [8, 8], 0).unwrap(), AddrLayout::Fixed(_)));
    assert!(matches!(choose_layout(0, & [8, 7], 0).unwrap(), AddrLayout::Variable(_)));
    assert!(matches!(choose_layout(0, & [8, 7], 63).unwrap(), AddrLayout::Fixed(_)));
    assert!(matches!(choose_layout(0, & [8, 7], 62).unwrap(), AddrLayout::Variable(_)));
    let big = u32::MAX as u64 + 1;
    assert!(
        matches!(choose_layout(0, & [big, big], 0).unwrap(), AddrLayout::Variable(_))
    );
    match choose_layout(5, &[8, 4], 500).unwrap() {
        AddrLayout::Fixed(f) => assert_eq!(f.stride(), 8),
        _ => panic!("expected fixed"),
    }
    assert_eq!(choose_layout(u64::MAX, & [1, 2], 0), Err(AddrLayoutError::Overflow));
}
#[test]
fn slot_map_resolution() {
    let l = choose_layout(0, &[4, 8, 2], 0).unwrap();
    assert_eq!(
        l.clone().into_slot_map(vec![eid(1)]), Err(AddrLayoutError::CountMismatch {
        expected : 3, got : 1 })
    );
    let m = l.into_slot_map(vec![eid(10), eid(20), eid(30)]).unwrap();
    assert_eq!(m.layout().len(), 3);
    assert_eq!(m.entry_of(Addr(3)), Some(eid(10)));
    assert_eq!(m.entry_of(Addr(4)), Some(eid(20)));
    assert_eq!(m.entry_of(Addr(11)), Some(eid(20)));
    assert_eq!(m.entry_of(Addr(12)), Some(eid(30)));
    assert_eq!(m.entry_of(Addr(14)), None);
    assert_eq!(m.entry_of_slot(Slot5(3)), None);
    let hit = KernelHit {
        addr: Addr(13),
        score: 0.5,
    };
    assert_eq!(m.resolve(hit), Some((eid(30), 0.5)));
    assert_eq!(m.resolve(KernelHit { addr : Addr(99), score : 1.0 }), None);
}
#[test]
fn error_display() {
    assert!(! AddrLayoutError::Overlap.to_string().is_empty());
    assert!(
        AddrLayoutError::CountMismatch { expected : 1, got : 2 } .to_string()
        .contains('2')
    );
}
