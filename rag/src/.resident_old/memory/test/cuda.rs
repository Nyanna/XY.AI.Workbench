use super::*;
use crate::resident::subengine::{LaneClass, LaneConfig};
fn manager() -> (DeviceHandle, CudaMemoryManager) {
    let dev = DeviceHandle::cuda(0).unwrap();
    let sizes = ArenaSizes {
        weights: 1 << 20,
        index_segment: 2 << 20,
        workspace: 1 << 20,
        staging: 0,
        param_block: 4096,
        mapped: 0,
    };
    let m = CudaMemoryManager::new(&dev, sizes, 0, 2).unwrap();
    (dev, m)
}
#[test]
#[ignore = "requires a CUDA device"]
fn upload_copy_download_roundtrip() {
    let (dev, m) = manager();
    let scratch = m.alloc_in(ArenaKind::ParamBlock, 256, 8).unwrap();
    let lane = dev.open_lane(LaneClass::Index, LaneConfig::new(scratch)).unwrap();
    let ctx = dev.cuda_context().unwrap();
    let stream = ctx.new_stream().unwrap();
    let ring = m.staging_ring(2, 64).unwrap();
    let a = m.alloc_in(ArenaKind::Workspace, 64, 16).unwrap();
    let b = m.alloc_in(ArenaKind::IndexSegment, 64, 16).unwrap();
    let slot = ring.acquire().unwrap();
    ring.with_slot(slot, |p| p.as_mut_slice().unwrap()[..64].copy_from_slice(&[7u8; 64]))
        .unwrap();
    let host: Vec<u8> = ring
        .with_slot(slot, |p| p.as_slice().unwrap()[..64].to_vec())
        .unwrap();
    unsafe { m.upload(&stream, &host, a).unwrap() };
    m.copy_d2d(&stream, a, b).unwrap();
    let mut out = vec![0u8; 64];
    unsafe { m.download(&stream, b, &mut out).unwrap() };
    stream.synchronize().unwrap();
    assert_eq!(out, vec![7u8; 64]);
    assert!(m.copy_d2d(& stream, a, a).is_err());
    assert!(m.budget().unwrap().total > 0);
    drop(lane);
}
#[test]
fn oversized_plan_is_rejected_before_allocating() {
    let Ok(dev) = DeviceHandle::cuda(0) else { return };
    let sizes = ArenaSizes {
        weights: u64::MAX / 2,
        ..Default::default()
    };
    assert!(
        matches!(CudaMemoryManager::new(& dev, sizes, 0, 2),
        Err(MemoryError::BudgetExceeded { .. }))
    );
}
#[test]
#[ignore = "requires a CUDA device"]
fn context_footprint_and_pool_support() {
    let dev = DeviceHandle::cuda(0).unwrap();
    let ctx = dev.cuda_context().unwrap();
    let mib = |b: usize| b >> 20;
    let (free_ctx, total) = ctx.mem_get_info().unwrap();
    let pools = ctx
        .attribute(sys::CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED)
        .unwrap();
    eprintln!("after context: free {} MiB / total {} MiB", mib(free_ctx), mib(total));
    eprintln!("memory pools supported: {pools}");
    let candle = candle_core::Device::new_cuda(0).unwrap();
    let a = candle_core::Tensor::zeros((256, 256), candle_core::DType::F32, &candle)
        .unwrap();
    a.matmul(&a).unwrap();
    candle.synchronize().unwrap();
    let (free_libs, _) = ctx.mem_get_info().unwrap();
    eprintln!(
        "after cublas matmul: free {} MiB (libraries cost {} MiB)", mib(free_libs),
        mib(free_ctx.saturating_sub(free_libs))
    );
    assert_eq!(pools, 1);
}
#[test]
#[ignore = "requires a CUDA device"]
fn index_arena_copy_on_write_on_device() {
    use crate::resident::manager::arena::IndexArena;
    let (dev, m) = manager();
    let arena = IndexArena::new(m.arenas().get(ArenaKind::IndexSegment), 0);
    let ctx = dev.cuda_context().unwrap();
    let stream = ctx.new_stream().unwrap();
    let ring = m.staging_ring(1, 512).unwrap();
    let old = arena.alloc_segment(512, 1).unwrap();
    let ticket = arena.begin_cow(old).unwrap();
    let slot = ring.acquire().unwrap();
    let host: Vec<u8> = ring
        .with_slot(
            slot,
            |p| {
                p.as_mut_slice().unwrap().fill(9);
                p.as_slice().unwrap().to_vec()
            },
        )
        .unwrap();
    unsafe { m.upload(&stream, &host, ticket.new_region()).unwrap() };
    let mut out = vec![0u8; 512];
    unsafe { m.download(&stream, ticket.new_region(), &mut out).unwrap() };
    stream.synchronize().unwrap();
    assert_eq!(out, host);
    ticket.commit(1).unwrap();
    assert_eq!(arena.reclaim(1), 1);
}
