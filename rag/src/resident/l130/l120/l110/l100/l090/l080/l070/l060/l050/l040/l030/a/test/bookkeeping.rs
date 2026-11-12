use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::{
    ArenaCapacities, Budget,
};

fn setup(cap: u64) -> Bookkeeping {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::Weights, cap);
    caps.set(ArenaKind::Staging, cap);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    set.ensure(ArenaKind::Weights, &budget).unwrap();
    set.ensure(ArenaKind::Staging, &budget).unwrap();
    Bookkeeping::new(set)
}

const W: ArenaKind = ArenaKind::Weights;

#[test]
fn first_fit_and_errors() {
    let b = setup(100);
    assert_eq!(b.allocate(W, 0), Err(AllocError::ZeroLength));
    assert_eq!(
        b.allocate(ArenaKind::Workspace, 1),
        Err(AllocError::ArenaNotCreated {
            arena: ArenaKind::Workspace
        })
    );
    let a = b.allocate(W, 40).unwrap();
    let c = b.allocate(W, 40).unwrap();
    assert_eq!((a.offset, c.offset), (0, 40));
    assert_eq!(
        b.allocate(W, 30),
        Err(AllocError::OutOfSpace {
            arena: W,
            requested: 30,
            free: 20
        })
    );
    b.free(W, &a).unwrap();
    let d = b.allocate(W, 10).unwrap();
    assert_eq!(d.offset, 0);
    assert_eq!(b.live_bytes(W), 50);
}

#[test]
fn fragmentation_reports_total_free() {
    let b = setup(30);
    let x = b.allocate(W, 10).unwrap();
    let _y = b.allocate(W, 10).unwrap();
    let z = b.allocate(W, 10).unwrap();
    b.free(W, &x).unwrap();
    b.free(W, &z).unwrap();
    assert_eq!(
        b.allocate(W, 20),
        Err(AllocError::OutOfSpace {
            arena: W,
            requested: 20,
            free: 20
        })
    );
}

#[test]
fn generations_and_liveness() {
    let b = setup(100);
    let r0 = b.allocate(W, 10).unwrap();
    assert_eq!(r0.generation, 0);
    assert!(b.is_live(&r0));
    b.free(W, &r0).unwrap();
    assert!(!b.is_live(&r0));
    assert_eq!(b.free(W, &r0), Err(FreeError::AlreadyFreed));
    let r1 = b.allocate(W, 10).unwrap();
    assert_eq!(r1.generation, 1);
    assert!(b.is_live(&r1) && !b.is_live(&r0));
    assert_eq!(b.free(W, &r0), Err(FreeError::Mismatch));
    let s = b.allocate(ArenaKind::Staging, 10).unwrap();
    assert_eq!(s.generation, 0);
}

#[test]
fn strict_free_errors() {
    let b = setup(100);
    let r = b.allocate(W, 10).unwrap();
    assert_eq!(
        b.free(ArenaKind::Staging, &r),
        Err(FreeError::ForeignArena {
            expected: ArenaKind::Staging,
            found: W
        })
    );
    let other_len = Region { len: 5, ..r };
    assert_eq!(b.free(W, &other_len), Err(FreeError::Mismatch));
    let unknown = Region { offset: 50, ..r };
    assert_eq!(b.free(W, &unknown), Err(FreeError::UnknownRegion));
    assert!(b.is_live(&r));
}

#[test]
fn release_or_ignore_and_resolve() {
    let b = setup(100);
    let r = b.allocate(W, 10).unwrap();
    assert_eq!(b.resolve(&r), Some((0, 10)));
    assert_eq!(b.release_or_ignore(&r), ReleaseOutcome::Released);
    assert_eq!(b.release_or_ignore(&r), ReleaseOutcome::IgnoredStale);
    assert_eq!(b.resolve(&r), None);
    assert_eq!(b.live_bytes(W), 0);
}

#[test]
fn fault_injection() {
    let b = setup(100);
    b.inject_alloc_failure(W, 2);
    assert_eq!(b.allocate(W, 0), Err(AllocError::ZeroLength));
    assert!(b.allocate(ArenaKind::Staging, 5).is_ok());
    assert_eq!(
        b.allocate(W, 5),
        Err(AllocError::OutOfSpace {
            arena: W,
            requested: 5,
            free: 100
        })
    );
    assert!(b.allocate(W, 5).is_err());
    let r = b.allocate(W, 5).unwrap();
    assert_eq!((r.offset, r.generation), (0, 0));
    b.inject_alloc_failure(W, 5);
    b.clear_alloc_failure(W);
    assert!(b.allocate(W, 5).is_ok());
}

#[test]
fn zero_capacity() {
    let b = setup(0);
    assert_eq!(
        b.allocate(W, 1),
        Err(AllocError::OutOfSpace {
            arena: W,
            requested: 1,
            free: 0
        })
    );
}

#[test]
fn concurrent_no_overlap_and_single_free() {
    let b = Arc::new(setup(1000));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let b = b.clone();
            std::thread::spawn(move || {
                (0..20).filter_map(|_| b.allocate(W, 10).ok()).collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all: Vec<Region> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all.sort_by_key(|r| r.offset);
    for w in all.windows(2) {
        assert!(w[0].offset + w[0].len <= w[1].offset);
    }
    assert!(b.live_bytes(W) <= 1000);

    let r = all[0];
    let results: Vec<_> = (0..4)
        .map(|_| {
            let b = b.clone();
            std::thread::spawn(move || b.free(W, &r))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert!(results
        .iter()
        .filter_map(|r| r.as_ref().err())
        .all(|e| *e == FreeError::AlreadyFreed));
}
