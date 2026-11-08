use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
static_assertions::assert_impl_all!(Region : Send, Sync);
static_assertions::assert_impl_all!(TensorRef : Send, Sync);
static_assertions::assert_impl_all!(AccessError : Send, Sync);
static_assertions::assert_impl_all!(TensorRefError : Send, Sync);
const _: fn() = || {
    fn is_static<T: 'static>() {}
    is_static::<Region>();
    is_static::<TensorRef>();
    is_static::<AccessError>();
};
#[derive(Debug)]
struct MemBackend {
    bytes: Mutex<Vec<u8>>,
    calls: AtomicUsize,
    fail: Option<String>,
}
impl MemBackend {
    fn new(len: usize) -> Arc<MemBackend> {
        Arc::new(MemBackend {
            bytes: Mutex::new(vec![0; len]),
            calls: AtomicUsize::new(0),
            fail: None,
        })
    }
    fn failing(len: usize, text: &str) -> Arc<MemBackend> {
        Arc::new(MemBackend {
            bytes: Mutex::new(vec![0; len]),
            calls: AtomicUsize::new(0),
            fail: Some(text.to_string()),
        })
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        self.bytes.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some(t) = &self.fail {
            return Err(AccessError::Backend(t.clone()));
        }
        let b = self.bytes.lock().unwrap();
        dst.copy_from_slice(&b[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some(t) = &self.fail {
            return Err(AccessError::Backend(t.clone()));
        }
        let mut b = self.bytes.lock().unwrap();
        b[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}
fn region(len: usize) -> (Region, Arc<MemBackend>) {
    let b = MemBackend::new(len);
    (Region::new(b.clone()), b)
}
fn layout(shape: &[usize], strides: &[usize], dtype: DType) -> Layout {
    Layout::new(shape.to_vec(), strides.to_vec(), dtype).unwrap()
}
#[test]
fn region_len_and_is_empty() {
    let (r, _) = region(8);
    assert_eq!(r.len(), 8);
    assert!(! r.is_empty());
    let (e, _) = region(0);
    assert!(e.is_empty());
}
#[test]
fn write_then_read_roundtrip() {
    let (r, _) = region(16);
    r.write(4, &[1, 2, 3, 4]).unwrap();
    let mut out = [0u8; 6];
    r.read(3, &mut out).unwrap();
    assert_eq!(out, [0, 1, 2, 3, 4, 0]);
}
#[test]
fn read_your_writes_through_clone() {
    let (r, _) = region(8);
    let c = r.clone();
    r.write(0, &[9, 8, 7]).unwrap();
    let mut out = [0u8; 3];
    c.read(0, &mut out).unwrap();
    assert_eq!(out, [9, 8, 7]);
    assert!(r.same_region(& c));
}
#[test]
fn same_region_distinguishes_backends() {
    let (a, _) = region(4);
    let (b, _) = region(4);
    assert!(! a.same_region(& b));
}
#[test]
fn full_region_access_is_allowed() {
    let (r, _) = region(4);
    r.write(0, &[1, 2, 3, 4]).unwrap();
    let mut out = [0u8; 4];
    r.read(0, &mut out).unwrap();
    assert_eq!(out, [1, 2, 3, 4]);
}
#[test]
fn out_of_bounds_read_does_not_call_backend() {
    let (r, b) = region(4);
    let mut out = [0u8; 3];
    let err = r.read(2, &mut out).unwrap_err();
    assert_eq!(err, AccessError::OutOfBounds { offset : 2, len : 3, region_len : 4 });
    assert_eq!(b.calls(), 0);
}
#[test]
fn out_of_bounds_write_leaves_bytes_untouched() {
    let (r, b) = region(4);
    let err = r.write(3, &[1, 1]).unwrap_err();
    assert!(matches!(err, AccessError::OutOfBounds { .. }));
    assert_eq!(b.calls(), 0);
    let mut out = [0xffu8; 4];
    r.read(0, &mut out).unwrap();
    assert_eq!(out, [0; 4]);
}
#[test]
fn offset_overflow_is_out_of_bounds() {
    let (r, b) = region(4);
    let mut out = [0u8; 2];
    let err = r.read(usize::MAX, &mut out).unwrap_err();
    assert_eq!(
        err, AccessError::OutOfBounds { offset : usize::MAX, len : 2, region_len : 4 }
    );
    assert!(r.write(usize::MAX - 1, & [0, 0, 0]).is_err());
    assert_eq!(b.calls(), 0);
}
#[test]
fn zero_length_access_is_noop_without_backend_call() {
    let (r, b) = region(4);
    r.read(4, &mut []).unwrap();
    r.write(0, &[]).unwrap();
    r.write(4, &[]).unwrap();
    assert_eq!(b.calls(), 0);
}
#[test]
fn zero_length_access_past_end_fails() {
    let (r, b) = region(4);
    let err = r.read(5, &mut []).unwrap_err();
    assert_eq!(err, AccessError::OutOfBounds { offset : 5, len : 0, region_len : 4 });
    assert!(r.write(usize::MAX, & []).is_err());
    assert_eq!(b.calls(), 0);
}
#[test]
fn backend_errors_pass_through_unchanged() {
    let b = MemBackend::failing(8, "transfer failed");
    let r = Region::new(b.clone());
    let mut out = [0u8; 2];
    assert_eq!(
        r.read(0, & mut out).unwrap_err(), AccessError::Backend("transfer failed".into())
    );
    assert_eq!(
        r.write(0, & [1]).unwrap_err(), AccessError::Backend("transfer failed".into())
    );
    assert_eq!(b.calls(), 2);
}
#[test]
fn access_error_display() {
    let e = AccessError::OutOfBounds {
        offset: 1,
        len: 2,
        region_len: 3,
    };
    assert_eq!(
        e.to_string(), "Zugriff außerhalb der Region: offset=1 len=2 region_len=3"
    );
    assert_eq!(
        AccessError::Backend("x".into()).to_string(), "Speicherzugriff fehlgeschlagen: x"
    );
    let _: &dyn std::error::Error = &e;
}
#[test]
fn concurrent_disjoint_writes_do_not_interfere() {
    let (r, _) = region(8 * 1024);
    let handles: Vec<_> = (0..8u8)
        .map(|i| {
            let r = r.clone();
            std::thread::spawn(move || {
                let chunk = vec![i + 1; 1024];
                for _ in 0..50 {
                    r.write(i as usize * 1024, &chunk).unwrap();
                    let mut out = vec![0u8; 1024];
                    r.read(i as usize * 1024, &mut out).unwrap();
                    assert_eq!(out, chunk);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    for i in 0..8usize {
        let mut out = vec![0u8; 1024];
        r.read(i * 1024, &mut out).unwrap();
        assert!(out.iter().all(|& v | v == i as u8 + 1));
    }
}
#[test]
fn tensor_new_contiguous_fits_exactly() {
    let (r, _) = region(24);
    let t = TensorRef::new(r.clone(), Layout::contiguous(&[2, 3], DType::F32)).unwrap();
    assert_eq!(t.offset_bytes(), 0);
    assert_eq!(t.end_bytes(), 24);
    assert_eq!(t.dtype(), DType::F32);
    assert_eq!(t.layout().shape(), & [2, 3]);
    assert!(t.region().same_region(& r));
}
#[test]
fn tensor_exceeding_region_is_rejected() {
    let (r, _) = region(23);
    let err = TensorRef::new(r, Layout::contiguous(&[2, 3], DType::F32)).unwrap_err();
    assert_eq!(
        err, TensorRefError::ExceedsRegion { required_end : 24, region_len : 23 }
    );
}
#[test]
fn tensor_with_offset_checks_end() {
    let (r, _) = region(32);
    let l = Layout::contiguous(&[2, 3], DType::F32);
    let t = TensorRef::with_offset(r.clone(), l.clone(), 8).unwrap();
    assert_eq!(t.offset_bytes(), 8);
    assert_eq!(t.end_bytes(), 32);
    assert_eq!(
        TensorRef::with_offset(r, l, 9).unwrap_err(), TensorRefError::ExceedsRegion {
        required_end : 33, region_len : 32 }
    );
}
#[test]
fn tensor_offset_overflow_reports_max() {
    let (r, _) = region(32);
    let err = TensorRef::with_offset(r, Layout::contiguous(&[2], DType::F32), usize::MAX)
        .unwrap_err();
    assert_eq!(
        err, TensorRefError::ExceedsRegion { required_end : usize::MAX, region_len : 32 }
    );
}
#[test]
fn tensor_stride_overflow_reports_max() {
    let (r, _) = region(32);
    let l = layout(&[usize::MAX, 2], &[usize::MAX, 1], DType::U8);
    let err = TensorRef::new(r, l).unwrap_err();
    assert_eq!(
        err, TensorRefError::ExceedsRegion { required_end : usize::MAX, region_len : 32 }
    );
}
#[test]
fn rank_zero_span_is_element_size() {
    let (r, _) = region(8);
    let t = TensorRef::with_offset(r.clone(), Layout::contiguous(&[], DType::I64), 0)
        .unwrap();
    assert_eq!(t.end_bytes(), 8);
    assert!(TensorRef::with_offset(r, Layout::contiguous(& [], DType::I64), 1).is_err());
}
#[test]
fn empty_tensor_valid_when_offset_within_region() {
    let (r, _) = region(4);
    let l = Layout::contiguous(&[0, 5], DType::F32);
    let t = TensorRef::with_offset(r.clone(), l.clone(), 4).unwrap();
    assert_eq!(t.end_bytes(), 4);
    assert_eq!(
        TensorRef::with_offset(r, l, 5).unwrap_err(), TensorRefError::ExceedsRegion {
        required_end : 5, region_len : 4 }
    );
}
#[test]
fn empty_tensor_in_empty_region() {
    let (r, _) = region(0);
    let t = TensorRef::new(r, Layout::contiguous(&[0], DType::U8)).unwrap();
    assert_eq!(t.end_bytes(), 0);
}
#[test]
fn non_contiguous_span_uses_strides() {
    let (r, _) = region(64);
    let l = layout(&[3, 2], &[16, 4], DType::F32);
    let t = TensorRef::new(r, l).unwrap();
    assert_eq!(t.end_bytes(), 40);
}
#[test]
fn non_contiguous_span_exceeding_region_fails() {
    let (r, _) = region(39);
    let l = layout(&[3, 2], &[16, 4], DType::F32);
    assert_eq!(
        TensorRef::new(r, l).unwrap_err(), TensorRefError::ExceedsRegion { required_end :
        40, region_len : 39 }
    );
}
#[test]
fn transpose_keeps_offset_and_region() {
    let (r, _) = region(64);
    let t = TensorRef::with_offset(r.clone(), Layout::contiguous(&[2, 3], DType::F32), 8)
        .unwrap();
    let tt = t.transpose(0, 1).unwrap();
    assert_eq!(tt.layout().shape(), & [3, 2]);
    assert_eq!(tt.layout().strides(), & [4, 12]);
    assert_eq!(tt.offset_bytes(), 8);
    assert_eq!(tt.end_bytes(), t.end_bytes());
    assert!(tt.region().same_region(& r));
}
#[test]
fn transpose_layout_error_is_wrapped() {
    let (r, _) = region(64);
    let t = TensorRef::new(r, Layout::contiguous(&[2, 3], DType::F32)).unwrap();
    assert_eq!(
        t.transpose(0, 2).unwrap_err(), TensorRefError::Layout(LayoutError::DimOutOfRange
        { dim : 2, rank : 2 })
    );
}
#[test]
fn narrow_moves_offset_by_start_stride() {
    let (r, _) = region(64);
    let t = TensorRef::with_offset(r.clone(), Layout::contiguous(&[4, 3], DType::F32), 4)
        .unwrap();
    let n = t.narrow(0, 1, 2).unwrap();
    assert_eq!(n.layout().shape(), & [2, 3]);
    assert_eq!(n.offset_bytes(), 4 + 12);
    assert_eq!(n.end_bytes(), 16 + 24);
    assert!(n.region().same_region(& r));
    let c = t.narrow(1, 2, 1).unwrap();
    assert_eq!(c.offset_bytes(), 4 + 8);
    assert_eq!(c.end_bytes(), 12 + 3 * 12 + 4);
}
#[test]
fn narrow_views_address_the_same_bytes() {
    let (r, _) = region(24);
    let t = TensorRef::new(r.clone(), Layout::contiguous(&[3, 2], DType::F32)).unwrap();
    let n = t.narrow(0, 2, 1).unwrap();
    n.region().write(n.offset_bytes(), &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    let mut out = [0u8; 8];
    r.read(16, &mut out).unwrap();
    assert_eq!(out, [1, 2, 3, 4, 5, 6, 7, 8]);
}
#[test]
fn narrow_layout_errors_are_wrapped() {
    let (r, _) = region(64);
    let t = TensorRef::new(r, Layout::contiguous(&[4, 3], DType::F32)).unwrap();
    assert_eq!(
        t.narrow(0, 3, 2).unwrap_err(),
        TensorRefError::Layout(LayoutError::NarrowOutOfRange { dim : 0, start : 3, len :
        2, size : 4 })
    );
    assert!(
        matches!(t.narrow(5, 0, 1).unwrap_err(),
        TensorRefError::Layout(LayoutError::DimOutOfRange { .. }))
    );
}
#[test]
fn narrow_to_empty_at_end_is_valid() {
    let (r, _) = region(48);
    let t = TensorRef::new(r, Layout::contiguous(&[4, 3], DType::F32)).unwrap();
    let n = t.narrow(0, 4, 0).unwrap();
    assert_eq!(n.offset_bytes(), 48);
    assert_eq!(n.end_bytes(), 48);
}
#[test]
fn narrow_offset_overflow_reports_max() {
    let (r, _) = region(64);
    let l = layout(&[2, 0], &[1 << (usize::BITS - 1), 1], DType::U8);
    let t = TensorRef::new(r, l).unwrap();
    assert_eq!(
        t.narrow(0, 2, 0).unwrap_err(), TensorRefError::ExceedsRegion { required_end :
        usize::MAX, region_len : 64 }
    );
}
#[test]
fn tensor_ref_error_conversion_and_display() {
    let e: TensorRefError = LayoutError::DimOutOfRange {
        dim: 1,
        rank: 0,
    }
        .into();
    assert!(matches!(e, TensorRefError::Layout(_)));
    assert!(! e.to_string().is_empty());
    assert!(std::error::Error::source(& e).is_some());
    let x = TensorRefError::ExceedsRegion {
        required_end: 5,
        region_len: 4,
    };
    assert!(x.to_string().contains("required_end=5"));
    assert!(std::error::Error::source(& x).is_none());
}
#[test]
fn tensor_clone_shares_region() {
    let (r, b) = region(8);
    let t = TensorRef::new(r, Layout::contiguous(&[2], DType::F32)).unwrap();
    let c = t.clone();
    assert!(c.region().same_region(t.region()));
    assert_eq!(b.calls(), 0);
}
