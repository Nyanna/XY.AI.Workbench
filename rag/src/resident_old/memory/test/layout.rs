use super::*;

#[test]
fn contiguous_byte_strides() {
    let l = Layout::contiguous(&[2, 3, 4], DType::F16);
    assert_eq!(l.strides, vec![24, 8, 2]);
    assert_eq!(l.byte_len(), 48);
    assert!(l.is_contiguous());
}

#[test]
fn views_do_not_copy() {
    let l = Layout::contiguous(&[2, 3], DType::F32);
    let t = l.transpose(0, 1);
    assert_eq!(t.shape, vec![3, 2]);
    assert_eq!(t.strides, vec![4, 12]);
    assert!(!t.is_contiguous());
    assert_eq!(l.narrow(1, 2).shape, vec![2, 2]);
}
