use super::*;

#[test]
fn alloc_align_and_exhaust() {
    let mut f = FreeList::new(100);
    assert_eq!(f.alloc(10, 1), Some(0));
    assert_eq!(f.alloc(10, 16), Some(16));
    assert_eq!(f.alloc(200, 1), None);
}

#[test]
fn coalesces_neighbours() {
    let mut f = FreeList::new(30);
    let a = f.alloc(10, 1).unwrap();
    let b = f.alloc(10, 1).unwrap();
    let c = f.alloc(10, 1).unwrap();
    f.free(a, 10);
    f.free(c, 10);
    f.free(b, 10);
    assert_eq!(f.available(), 30);
    assert_eq!(f.alloc(30, 1), Some(0));
}

#[test]
fn fragmentation_blocks_large_alloc() {
    let mut f = FreeList::new(30);
    let _a = f.alloc(10, 1).unwrap();
    let b = f.alloc(10, 1).unwrap();
    let _c = f.alloc(10, 1).unwrap();
    f.free(b, 10);
    assert_eq!(f.available(), 10);
    assert_eq!(f.alloc(20, 1), None);
}
