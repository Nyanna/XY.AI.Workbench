use super::*;

#[test]
fn policy() {
    assert_eq!(compute_dtype(DeviceKind::Gpu), DType::F16);
    assert_eq!(compute_dtype(DeviceKind::Cpu), DType::F32);
}

#[test]
fn realloc_bumps_gen_and_stale_is_rejected() {
    let a = BookkeepingArena::new(ArenaId(1), 64);
    let r1 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();
    a.free(r1);
    let r2 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();
    assert_eq!(r2.offset, r1.offset);
    assert_eq!(r2.gen, r1.gen + 1);
    assert!(a.try_free(r1).is_err());
    assert!(a.is_live(r2));
    assert!(a.try_free(r2).is_ok());
}

#[test]
fn failpoint_injects_exhaustion_without_consuming_space() {
    let a = BookkeepingArena::new(ArenaId(9001), 64);
    let guard = failpoint::arm("memory.alloc", 9001, 1);
    assert!(matches!(a.alloc(DeviceKind::Cpu, 8, 1), Err(IndexError::ArenaExhausted { .. })));
    assert_eq!(a.available(), 64);
    assert!(a.alloc(DeviceKind::Cpu, 8, 1).is_ok());
    drop(guard);
}

proptest::proptest! {
    #[test]
    fn live_regions_never_overlap(ops in proptest::collection::vec((1u64..24, proptest::bool::ANY), 1..80)) {
        let a = BookkeepingArena::new(ArenaId(1), 256);
        let mut live: Vec<Region> = Vec::new();
        for (bytes, free_one) in ops {
            if free_one && !live.is_empty() {
                let r = live.swap_remove(0);
                a.free(r);
            } else if let Ok(r) = a.alloc(DeviceKind::Cpu, bytes, 1) {
                for o in &live {
                    proptest::prop_assert!(r.offset + r.len <= o.offset || o.offset + o.len <= r.offset);
                }
                proptest::prop_assert!(r.offset + r.len <= 256);
                live.push(r);
            }
        }
        let used: u64 = live.iter().map(|r| r.len).sum();
        proptest::prop_assert_eq!(a.available(), 256 - used);
    }
}

#[test]
fn exhaustion() {
    let a = BookkeepingArena::new(ArenaId(0), 32);
    let _r = a.alloc(DeviceKind::Gpu, 24, 1).unwrap();
    assert!(matches!(
        a.alloc(DeviceKind::Gpu, 16, 1),
        Err(IndexError::ArenaExhausted { requested: 16, available: 8 })
    ));
}
