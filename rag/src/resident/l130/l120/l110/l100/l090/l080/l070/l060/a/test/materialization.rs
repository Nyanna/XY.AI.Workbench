use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::{DType, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

static_assertions::assert_impl_all!(Materialization: Send, Sync);
static_assertions::assert_impl_all!(DeviceResult: Send, Sync);
static_assertions::assert_impl_all!(Materialized: Send, Sync);
static_assertions::assert_impl_all!(MaterializeError: Send, Sync);

#[derive(Debug)]
struct Counting {
    data: Mutex<Vec<u8>>,
    reads: AtomicUsize,
    fail: bool,
}

impl Counting {
    fn new(data: Vec<u8>, fail: bool) -> Arc<Counting> {
        Arc::new(Counting { data: Mutex::new(data), reads: AtomicUsize::new(0), fail })
    }
}

impl RegionBackend for Counting {
    fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(AccessError::Backend("boom".into()));
        }
        dst.copy_from_slice(&self.data.lock().unwrap()[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.data.lock().unwrap()[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}

fn handle() -> RegionHandle {
    RegionHandle::new(ArenaKind::Workspace, 0, 64, Generation(1)).unwrap()
}

fn device(backend: &Arc<Counting>, layout: Layout) -> DeviceResult {
    let region = Region::new(backend.clone());
    DeviceResult::new(handle(), TensorRef::new(region, layout).unwrap())
}

#[test]
fn keep_on_device_does_not_touch_backend() {
    let b = Counting::new((0..16).collect(), false);
    let r = device(&b, Layout::contiguous(&[4], DType::F32));
    let m = materialize(Materialization::KeepOnDevice, r.clone()).unwrap();
    assert!(m.is_device() && !m.is_host());
    assert_eq!(m.handle(), Some(r.handle()));
    assert!(m.tensor().region().same_region(r.tensor().region()));
    assert_eq!(b.reads.load(Ordering::SeqCst), 0);
}

#[test]
fn to_host_copies_bytes_independently() {
    let b = Counting::new((0..16).collect(), false);
    let src = device(&b, Layout::contiguous(&[2, 2], DType::F32));
    let m = materialize(Materialization::ToHost, src.clone()).unwrap();
    assert!(m.is_host());
    assert_eq!(m.handle(), None);
    let t = m.tensor();
    assert_eq!(t.offset_bytes(), 0);
    assert_eq!(t.layout(), src.tensor().layout());
    assert_eq!(t.region().len(), 16);
    assert!(!t.region().same_region(src.tensor().region()));
    src.tensor().region().write(0, &[99]).unwrap();
    let mut out = [0u8; 16];
    t.region().read(0, &mut out).unwrap();
    assert_eq!(out.to_vec(), (0..16).collect::<Vec<u8>>());
}

#[test]
fn to_host_keeps_strides_and_gap_bytes() {
    let b = Counting::new((0..32).collect(), false);
    let layout = Layout::new(vec![2, 2], vec![12, 4], DType::F32).unwrap();
    let src = device(&b, layout.clone());
    let m = materialize(Materialization::ToHost, src).unwrap();
    let t = m.tensor();
    assert_eq!(t.layout(), &layout);
    assert_eq!(t.region().len(), 12 + 4 + 4);
    let mut out = vec![0u8; 20];
    t.region().read(0, &mut out).unwrap();
    assert_eq!(out, (0..20).collect::<Vec<u8>>());
}

#[test]
fn to_host_empty_does_not_read() {
    let b = Counting::new(vec![0; 8], false);
    let src = device(&b, Layout::contiguous(&[0], DType::F32));
    let m = materialize(Materialization::ToHost, src).unwrap();
    assert_eq!(m.tensor().region().len(), 0);
    assert_eq!(b.reads.load(Ordering::SeqCst), 0);
}

#[test]
fn to_host_propagates_backend_error() {
    let b = Counting::new(vec![0; 8], true);
    let src = device(&b, Layout::contiguous(&[2], DType::F32));
    let e = materialize(Materialization::ToHost, src).unwrap_err();
    assert_eq!(e, MaterializeError::Access(AccessError::Backend("boom".into())));
    assert!(!e.to_string().is_empty());
}

#[test]
fn host_buffer_read_your_writes() {
    let region = Region::new(Arc::new(HostBuffer::new(vec![0; 4])));
    region.write(1, &[7, 8]).unwrap();
    let mut out = [0u8; 4];
    region.read(0, &mut out).unwrap();
    assert_eq!(out, [0, 7, 8, 0]);
    assert!(matches!(region.read(3, &mut [0u8; 2]), Err(AccessError::OutOfBounds { .. })));
}
