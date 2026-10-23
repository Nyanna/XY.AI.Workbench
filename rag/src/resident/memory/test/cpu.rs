use super::*;

fn mgr() -> CpuMemoryManager {
    CpuMemoryManager::new(
        ArenaSizes { weights: 64, index_segment: 128, workspace: 64, staging: 0, param_block: 16 },
        2,
    )
}

#[test]
fn write_read_roundtrip_and_stale_rejection() {
    let m = mgr();
    let r = m.alloc_in(ArenaKind::Workspace, 8, 8).unwrap();
    m.write(r, &[1, 2, 3, 4]).unwrap();
    let mut out = [0u8; 4];
    m.read(r, &mut out).unwrap();
    assert_eq!(out, [1, 2, 3, 4]);
    m.free(r).unwrap();
    assert!(m.read(r, &mut out).is_err());
    assert!(m.write(r, &[0]).is_err());
}

#[test]
fn oversized_access_is_rejected() {
    let m = mgr();
    let r = m.alloc_in(ArenaKind::Workspace, 4, 4).unwrap();
    assert!(m.write(r, &[0; 5]).is_err());
}

#[test]
fn copy_across_and_within_arenas() {
    let m = mgr();
    let a = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
    let b = m.alloc_in(ArenaKind::IndexSegment, 4, 4).unwrap();
    let c = m.alloc_in(ArenaKind::Weights, 4, 4).unwrap();
    m.write(a, &[9, 8, 7, 6]).unwrap();
    m.copy(a, b).unwrap();
    m.copy(a, c).unwrap();
    let mut out = [0u8; 4];
    m.read(b, &mut out).unwrap();
    assert_eq!(out, [9, 8, 7, 6]);
    m.read(c, &mut out).unwrap();
    assert_eq!(out, [9, 8, 7, 6]);
}

#[test]
fn budget_reserves_the_second_generation() {
    let m = mgr();
    let before = m.budget().unwrap();
    assert_eq!(before.total, 272);
    assert_eq!(before.reserved_for_generations, 64);
    m.alloc_in(ArenaKind::IndexSegment, 32, 8).unwrap();
    assert_eq!(m.budget().unwrap().free, 240);
}
