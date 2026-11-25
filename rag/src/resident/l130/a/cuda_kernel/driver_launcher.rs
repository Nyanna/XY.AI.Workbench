//! Enqueues kernel launches on the stream of a lane without waiting for completion.

use std::ffi::c_void;
use std::sync::Arc;

use cudarc::driver::result as drv;

use super::super::cuda_backend::stream_of;
use super::driver_loader::DriverFunction;
use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;
use l070::l060 as l060;
use l060::l050 as l050;
use l050::l040 as l040;
use l040::l030 as l030;
use l030::l020 as l020;
use l020::l010 as l010;

use l010::device_lane_event::Lane;
use l070::a::kernel_register::KernelFunction;
use l090::a::gpu_kernel::{GpuLauncher, LaunchConfig};

#[derive(Debug, Default, Clone, Copy)]
pub struct DriverLauncher;

impl GpuLauncher for DriverLauncher {
    fn launch(
        &self,
        function: &Arc<dyn KernelFunction>,
        lane: &Lane,
        cfg: &LaunchConfig,
        params: &[u8],
    ) -> Result<(), String> {
        let function = function
            .as_any()
            .downcast_ref::<DriverFunction>()
            .ok_or_else(|| "function was not loaded by the driver loader".to_string())?;
        let stream = lane.stream().ok_or_else(|| "lane has no stream".to_string())?;
        let (ctx, raw_stream) = stream_of(stream)
            .ok_or_else(|| "lane stream was not created by the CUDA backend".to_string())?;
        if ctx.ordinal() != function.context().ordinal() {
            return Err("function and lane stream belong to different devices".to_string());
        }
        ctx.bind_to_thread().map_err(|e| e.to_string())?;
        let mut block = params.to_vec();
        let mut args = [block.as_mut_ptr().cast::<c_void>()];
        unsafe {
            drv::launch_kernel(
                function.raw(),
                (cfg.grid, 1, 1),
                (cfg.block, 1, 1),
                cfg.dynamic_shared,
                raw_stream,
                &mut args,
            )
        }
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
#[path = "test/driver_launcher.rs"]
mod tests;
