use super::super::super::l040::l030::l020::l010::layout::{DType, Layout};
use super::*;
fn assert_thread_safe<T: Send + Sync + 'static>() {}
#[test]
fn types_are_send_sync_static() {
    assert_thread_safe::<GpuBackend>();
    assert_thread_safe::<PinnedBuffer>();
    assert_thread_safe::<StagingRing>();
    assert_thread_safe::<BackendError>();
}
const TEST_MIB: u64 = 1024 * 1024;
fn caps() -> ArenaCapacities {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::Weights, 8 * TEST_MIB);
    caps.set(ArenaKind::IndexSegments, 8 * TEST_MIB);
    caps.set(ArenaKind::Workspace, 8 * TEST_MIB);
    caps.set(ArenaKind::Staging, 4 * TEST_MIB);
    caps.set(ArenaKind::ParameterBlock, TEST_MIB);
    caps.set(ArenaKind::MappedHost, 2 * TEST_MIB);
    caps
}
/// `None` when no usable Nvidia device is present.
fn backend() -> Option<GpuBackend> {
    match GpuBackend::new(DeviceHandle::Gpu(0), caps(), TEST_MIB) {
        Ok(b) => Some(b),
        Err(BackendError::DeviceMemoryUnavailable(reason)) => {
            eprintln!("skipped, no device: {reason}");
            None
        }
        Err(e) => panic!("unexpected error: {e}"),
    }
}
fn pinned(b: &GpuBackend, data: &[u8]) -> PinnedBuffer {
    let mut ring = b.create_staging_ring(1, data.len()).unwrap();
    let mut buf = std::mem::replace(&mut ring.buffers[0], PinnedBuffer::empty());
    buf.as_mut_slice().copy_from_slice(data);
    buf
}
#[test]
fn cpu_device_is_unavailable() {
    let err = GpuBackend::new(DeviceHandle::Cpu, caps(), 0).unwrap_err();
    assert!(matches!(err, BackendError::DeviceMemoryUnavailable(_)));
}
#[test]
fn error_display_texts() {
    let e = BackendError::Transfer {
        direction: TransferDirection::D2D,
        reason: "same arena".to_string(),
    };
    assert_eq!(e.to_string(), "transfer D2D failed: same arena");
    assert_eq!(BackendError::StaleRegion.to_string(), "stale Region");
    assert_eq!(
        BackendError::DeviceMemoryUnavailable("x".into()).to_string(),
        "Device-Speicher nicht verfügbar: x"
    );
}
#[test]
fn generation_beyond_u32_is_stale() {
    let handle = RegionHandle::new(
            ArenaKind::Weights,
            0,
            256,
            Generation(u64::from(u32::MAX) + 1),
        )
        .unwrap();
    assert_eq!(to_region(& handle), Err(BackendError::StaleRegion));
    let ok = handle.with_generation(Generation(7));
    let region = to_region(&ok).unwrap();
    assert_eq!(
        (region.arena, region.offset, region.len, region.generation),
        (ArenaKind::Weights, 0, 256, 7)
    );
}
#[test]
fn empty_ring_has_no_index() {
    let ring = StagingRing::from_buffers(Vec::new());
    assert_eq!(ring.len(), 0);
    assert_eq!(ring.next_index(), None);
    assert!(ring.get(0).is_none());
}
#[test]
fn ring_indices_wrap_round_robin() {
    let ring = StagingRing::from_buffers(
        vec![PinnedBuffer::empty(), PinnedBuffer::empty(), PinnedBuffer::empty(),],
    );
    let seen: Vec<_> = (0..7).filter_map(|_| ring.next_index()).collect();
    assert_eq!(seen, vec![0, 1, 2, 0, 1, 2, 0]);
    assert_eq!(ring.buffer_size(), 0);
    assert!(ring.get(2).unwrap().is_empty());
}
#[test]
fn creation_reports_layout_and_zeroed_state() {
    let Some(b) = backend() else { return };
    let c = b.capacities();
    assert_eq!(c.get(ArenaKind::Staging), 0);
    assert_eq!(c.get(ArenaKind::Weights), 8 * TEST_MIB);
    assert!(b.mapped_device_addr().is_some());
    assert_eq!(b.device(), DeviceHandle::Gpu(0));
    assert_eq!(b.available(ArenaKind::Staging), Ok(0));
    assert_eq!(b.available(ArenaKind::Weights), Ok(8 * TEST_MIB));
    let budget = b.budget().unwrap();
    assert!(budget.total >= budget.free && budget.total > 0);
    assert_eq!(budget.generation_reserve, 0);
}
#[test]
fn oversized_request_does_not_fit() {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::Weights, u64::MAX / 2);
    match GpuBackend::new(DeviceHandle::Gpu(0), caps, 0) {
        Err(BackendError::Budget(_)) | Err(BackendError::DeviceMemoryUnavailable(_)) => {}
        other => panic!("unexpected: {other:?}"),
    }
}
#[test]
fn allocation_rounds_and_validates() {
    let Some(b) = backend() else { return };
    let r = b.allocate(ArenaKind::Weights, 100, 16).unwrap();
    assert_eq!(r.len(), 256);
    assert_eq!(r.offset() % 256, 0);
    for (size, align) in [(0, 1), (8, 0), (8, 3), (8, 512), (u64::MAX, 1)] {
        assert_eq!(
            b.allocate(ArenaKind::Weights, size, align),
            Err(BackendError::AllocationFailed)
        );
    }
    assert!(
        matches!(b.allocate(ArenaKind::Weights, 16 * TEST_MIB, 1),
        Err(BackendError::OutOfMemory { .. }))
    );
    assert!(
        matches!(b.allocate(ArenaKind::Staging, 256, 1), Err(BackendError::OutOfMemory {
        .. }))
    );
    b.free(&r).unwrap();
    assert_eq!(b.free(& r), Err(BackendError::StaleRegion));
}
#[test]
fn upload_download_roundtrip_pinned_and_pageable() {
    let Some(b) = backend() else { return };
    let region = b.allocate(ArenaKind::Weights, 1024, 1).unwrap();
    let data: Vec<u8> = (0..1024u32).map(|i| (i % 251) as u8).collect();
    b.upload(&region, HostSource::Pageable(&data)).unwrap().wait().unwrap();
    let out = pinned(&b, &vec![0xAA; 1024]);
    b.download(&region, &out).unwrap().wait().unwrap();
    assert_eq!(out.as_slice(), & data[..]);
    let src = pinned(&b, &vec![0x5C; 512]);
    b.upload(&region, HostSource::Pinned(&src)).unwrap().wait().unwrap();
    let out = pinned(&b, &vec![0; 1024]);
    b.download(&region, &out).unwrap().wait().unwrap();
    assert!(out.as_slice() [..512].iter().all(|& v | v == 0x5C));
    assert_eq!(& out.as_slice() [512..], & data[512..]);
}
#[test]
fn fresh_blocks_are_zeroed() {
    let Some(b) = backend() else { return };
    let region = b.allocate(ArenaKind::Workspace, 512, 1).unwrap();
    let out = pinned(&b, &vec![0xFF; 512]);
    b.download(&region, &out).unwrap().wait().unwrap();
    assert!(out.as_slice().iter().all(|& v | v == 0));
}
#[test]
fn transfer_error_ordering() {
    let Some(b) = backend() else { return };
    let a = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    let a2 = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    let w = b.allocate(ArenaKind::Workspace, 256, 1).unwrap();
    let same = b.copy_d2d(&a, &a2, 16).unwrap_err();
    assert_eq!(
        same, BackendError::Transfer { direction : TransferDirection::D2D, reason :
        "same arena".to_string() }
    );
    assert!(
        matches!(b.copy_d2d(& a, & w, 257), Err(BackendError::Transfer { direction :
        TransferDirection::D2D, .. }))
    );
    assert!(
        matches!(b.upload(& a, HostSource::Pageable(& [0u8; 300])),
        Err(BackendError::Transfer { direction : TransferDirection::H2D, .. }))
    );
    let big = pinned(&b, &vec![0; 300]);
    assert!(
        matches!(b.download(& a, & big), Err(BackendError::Transfer { direction :
        TransferDirection::D2H, .. }))
    );
    b.free(&w).unwrap();
    assert_eq!(b.copy_d2d(& a, & w, 16).unwrap_err(), BackendError::StaleRegion);
    assert_eq!(
        b.upload(& w, HostSource::Pageable(& [0u8; 4])).unwrap_err(),
        BackendError::StaleRegion
    );
}
#[test]
fn d2d_copies_and_zero_length_returns_event() {
    let Some(b) = backend() else { return };
    let src = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    let dst = b.allocate(ArenaKind::IndexSegments, 256, 1).unwrap();
    let data: Vec<u8> = (0..256u32).map(|i| i as u8).collect();
    b.upload(&src, HostSource::Pageable(&data)).unwrap();
    b.copy_d2d(&src, &dst, 256).unwrap().wait().unwrap();
    let out = pinned(&b, &vec![0; 256]);
    b.download(&dst, &out).unwrap().wait().unwrap();
    assert_eq!(out.as_slice(), & data[..]);
    assert!(b.copy_d2d(& src, & dst, 0).unwrap().wait().is_ok());
}
#[test]
fn mapped_access_checks_in_order() {
    let Some(b) = backend() else { return };
    let m = b.allocate(ArenaKind::MappedHost, 512, 1).unwrap();
    b.mapped_write(&m, 10, &[1, 2, 3]).unwrap();
    let mut got = [0u8; 3];
    b.mapped_read(&m, 10, &mut got).unwrap();
    assert_eq!(got, [1, 2, 3]);
    b.mapped_write(&m, 0, &[]).unwrap();
    let w = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    assert_eq!(
        b.mapped_read(& w, 0, & mut got), Err(BackendError::ArenaMismatch { expected :
        ArenaKind::MappedHost, found : ArenaKind::Weights })
    );
    assert_eq!(
        b.mapped_write(& m, 510, & [0; 4]),
        Err(BackendError::Access(AccessError::OutOfBounds { offset : 510, len : 4,
        region_len : 512 }))
    );
    assert!(
        matches!(b.mapped_write(& m, usize::MAX, & [0; 4]),
        Err(BackendError::Access(AccessError::OutOfBounds { .. })))
    );
    b.free(&m).unwrap();
    assert_eq!(b.mapped_read(& m, 0, & mut got), Err(BackendError::StaleRegion));
}
#[test]
fn deferred_free_keeps_region_live_until_event_completes() {
    let Some(b) = backend() else { return };
    let r = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    let (event, control) = Event::mock();
    let before = b.available(ArenaKind::Weights).unwrap();
    b.free_deferred(&r, event).unwrap();
    assert_eq!(b.available(ArenaKind::Weights).unwrap(), before);
    control.complete();
    let again = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    assert_eq!(again.offset(), r.offset());
}
#[test]
fn staging_ring_buffers_are_zeroed_and_sized() {
    let Some(b) = backend() else { return };
    let ring = b.create_staging_ring(3, 4096).unwrap();
    assert_eq!((ring.len(), ring.buffer_size()), (3, 4096));
    assert!(ring.get(2).unwrap().as_slice().iter().all(|& v | v == 0));
    assert!(ring.get(3).is_none());
    assert_eq!(b.create_staging_ring(0, 64).unwrap().len(), 0);
    let zero = b.create_staging_ring(2, 0).unwrap();
    assert_eq!(zero.get(0).unwrap().len(), 0);
    assert!(
        matches!(b.create_staging_ring(usize::MAX, 2),
        Err(BackendError::DeviceMemoryUnavailable(_)))
    );
}
#[test]
fn wrap_tensor_views_device_memory() {
    let Some(b) = backend() else { return };
    let region = b.allocate(ArenaKind::Weights, 16, 4).unwrap();
    let values = [1.0f32, 2.0, 3.0, 4.0];
    let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
    b.upload(&region, HostSource::Pageable(&bytes)).unwrap().wait().unwrap();
    let layout = Layout::contiguous(&[4], DType::F32);
    let tensor = unsafe { b.wrap_tensor(&region, &layout) }.unwrap();
    assert_eq!(tensor.dims(), & [4]);
    assert_eq!(tensor.to_host_bytes().unwrap(), bytes);
    drop(tensor);
    b.free(&region).unwrap();
    assert!(
        matches!(unsafe { b.wrap_tensor(& region, & layout) },
        Err(BackendError::StaleRegion))
    );
}
#[test]
fn shutdown_invalidates_everything_and_is_idempotent() {
    let Some(b) = backend() else { return };
    let r = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
    let m = b.allocate(ArenaKind::MappedHost, 256, 1).unwrap();
    b.shutdown();
    b.shutdown();
    assert_eq!(b.mapped_device_addr(), None);
    assert_eq!(
        b.upload(& r, HostSource::Pageable(& [1])).unwrap_err(),
        BackendError::StaleRegion
    );
    assert_eq!(b.mapped_write(& m, 0, & [1]), Err(BackendError::StaleRegion));
    assert_eq!(b.available(ArenaKind::Weights), Ok(0));
}
#[test]
fn concurrent_transfers_are_covered_by_their_events() {
    let Some(b) = backend() else { return };
    let b = Arc::new(b);
    let handles: Vec<_> = (0..4u8)
        .map(|i| {
            let b = Arc::clone(&b);
            std::thread::spawn(move || {
                let r = b.allocate(ArenaKind::Weights, 256, 1).unwrap();
                let data = vec![i + 1; 256];
                b.upload(&r, HostSource::Pageable(&data)).unwrap();
                let out = pinned(&b, &vec![0; 256]);
                b.download(&r, &out).unwrap().wait().unwrap();
                assert!(out.as_slice().iter().all(|& v | v == i + 1));
                b.free(&r).unwrap();
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
