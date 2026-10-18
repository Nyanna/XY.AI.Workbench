//! Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it?

use std::sync::Arc;

use candle_core::cuda_backend::CudaStorage;
use candle_core::op::BackpropOp;
use candle_core::{Device, Storage, Tensor};
use cudarc::driver::{CudaContext, CudaSlice, CudaStream, DevicePtr};

const ARENA_ELEMS: usize = 64;
const VIEW_OFFSET: usize = 16;
const VIEW_ELEMS: usize = 16;

struct Fixture {
    dev: Device,
    ctx: Arc<CudaContext>,
    stream: Arc<CudaStream>,
    arena: CudaSlice<f32>,
}

impl Fixture {
    fn new() -> Self {
        let dev = Device::new_cuda(0).unwrap();
        let Device::Cuda(cd) = &dev else { unreachable!() };
        let stream = cd.cuda_stream();
        let ctx = stream.context().clone();
        let host: Vec<f32> = (0..ARENA_ELEMS).map(|i| i as f32).collect();
        let arena = stream.clone_htod(&host).unwrap();
        Self { dev, ctx, stream, arena }
    }

    /// Tensor over `arena[VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS]` whose storage owns the range.
    fn wrapped_view(&self) -> Tensor {
        let Device::Cuda(cd) = &self.dev else { unreachable!() };
        let (base, _g) = self.arena.device_ptr(&self.stream);
        let at = base + (VIEW_OFFSET * 4) as u64;
        let slice = unsafe { self.stream.upgrade_device_ptr::<f32>(at, VIEW_ELEMS) };
        let storage = CudaStorage::wrap_cuda_slice(slice, cd.clone());
        Tensor::from_storage(Storage::Cuda(storage), (VIEW_ELEMS,), BackpropOp::none(), false)
    }

    fn arena_host(&self) -> Vec<f32> {
        self.stream.clone_dtoh(&self.arena).unwrap()
    }
}

#[test]
#[ignore = "requires a CUDA device"]
fn candle_reads_the_wrapped_range_and_forgetting_it_is_clean() {
    let f = Fixture::new();
    let view = f.wrapped_view();
    let seen = view.to_vec1::<f32>().unwrap();
    assert_eq!(seen, (VIEW_OFFSET..VIEW_OFFSET + VIEW_ELEMS).map(|i| i as f32).collect::<Vec<_>>());
    std::mem::forget(view);
    f.ctx.synchronize().unwrap();
    f.ctx.check_err().unwrap();
    assert_eq!(f.arena_host()[0], 0.0);
}

#[test]
#[ignore = "requires a CUDA device"]
fn dropping_a_wrapped_tensor_poisons_the_context() {
    let f = Fixture::new();
    drop(f.wrapped_view());
    let synced = f.ctx.synchronize();
    println!("synchronize after drop: {synced:?}");
    assert!(synced.is_err() || f.ctx.check_err().is_err());
}

#[test]
#[ignore = "requires a CUDA device"]
fn candle_copies_allocate_their_own_storage() {
    let f = Fixture::new();
    let view = f.wrapped_view();
    let copy = view.copy().unwrap();
    let ptr_of = |t: &Tensor| -> u64 {
        let (st, _) = t.storage_and_layout();
        let Storage::Cuda(s) = &*st else { unreachable!() };
        let slice = s.as_cuda_slice::<f32>().unwrap();
        let (ptr, guard) = slice.device_ptr(&f.stream);
        drop(guard);
        ptr
    };
    let (base, _g) = f.arena.device_ptr(&f.stream);
    let out = ptr_of(&copy);
    assert!(out < base || out >= base + (ARENA_ELEMS * 4) as u64);
    std::mem::forget(view);
}
