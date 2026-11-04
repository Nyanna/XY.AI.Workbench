use super::*;

#[test]
fn first_fit_and_split() {
    let mut f = FreeSpace::new(100);
    assert_eq!(f.allocate(10, 0), Some(0));
    assert_eq!(f.allocate(10, 1), Some(10));
    assert_eq!(f.free_bytes(), 80);
    f.free(0, 10).unwrap();
    assert_eq!(f.allocate(5, 1), Some(0));
    assert_eq!(f.allocate(5, 1), Some(5));
}

#[test]
fn alignment_non_power_of_two() {
    let mut f = FreeSpace::new(100);
    assert_eq!(f.allocate(1, 1), Some(0));
    assert_eq!(f.allocate(10, 7), Some(7));
    assert_eq!(f.free_bytes(), 89);
    assert_eq!(f.allocate(1, 1), Some(1));
}

#[test]
fn zero_and_overflow() {
    let mut f = FreeSpace::new(100);
    assert_eq!(f.allocate(0, 1), None);
    assert_eq!(f.allocate(u64::MAX, 1), None);
    assert_eq!(f.allocate(1, u64::MAX), Some(0));
    assert_eq!(f.allocate(1, u64::MAX), None);
    assert_eq!(f.free_bytes(), 99);
    let mut z = FreeSpace::new(0);
    assert_eq!(z.allocate(1, 1), None);
}

#[test]
fn free_errors() {
    let mut f = FreeSpace::new(100);
    assert_eq!(f.free(0, 0), Err(FreeError::InvalidRange));
    assert_eq!(f.free(u64::MAX, 2), Err(FreeError::InvalidRange));
    assert_eq!(f.free(90, 11), Err(FreeError::InvalidRange));
    assert_eq!(f.free(10, 5), Err(FreeError::NotAllocated));
    let a = f.allocate(50, 1).unwrap();
    assert_eq!(f.free(a + 40, 20), Err(FreeError::NotAllocated));
    assert_eq!(f.free_bytes(), 50);
}

#[test]
fn coalesce_both_sides() {
    let mut f = FreeSpace::new(30);
    let a = f.allocate(10, 1).unwrap();
    let b = f.allocate(10, 1).unwrap();
    let c = f.allocate(10, 1).unwrap();
    f.free(a, 10).unwrap();
    f.free(c, 10).unwrap();
    f.free(b, 10).unwrap();
    assert_eq!(f, FreeSpace::new(30));
    assert_eq!(f.free_bytes(), f.capacity());
}

#[test]
fn partial_and_combined_free() {
    let mut f = FreeSpace::new(30);
    f.allocate(30, 1).unwrap();
    f.free(5, 5).unwrap();
    f.free(0, 5).unwrap();
    f.free(10, 20).unwrap();
    assert_eq!(f, FreeSpace::new(30));
}
