//! Loads compiled kernel images into a CUDA context and resolves their entry points.

use std::any::Any;
use std::ffi::CString;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use cudarc::driver::result as drv;
use cudarc::driver::{sys, CudaContext};

use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;
use l070::l060 as l060;
use l060::l050 as l050;
use l050::l040 as l040;
use l040::l030 as l030;
use l030::l020 as l020;
use l020::l010 as l010;

use l010::target_env::ComputeCapability;
use l070::a::kernel_register::{
    CodeImage, CodeKind, KernelFunction, LoadFailure, ModuleLoader,
};

fn text(e: drv::DriverError) -> String {
    e.to_string()
}

struct ModuleOwner {
    ctx: Arc<CudaContext>,
    raw: sys::CUmodule,
}

unsafe impl Send for ModuleOwner {}
unsafe impl Sync for ModuleOwner {}

impl Drop for ModuleOwner {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            let _ = unsafe { drv::module::unload(self.raw) };
        }
    }
}

/// A kernel entry point; keeps its module loaded for as long as it lives.
pub struct DriverFunction {
    symbol: String,
    raw: sys::CUfunction,
    module: Arc<ModuleOwner>,
}

unsafe impl Send for DriverFunction {}
unsafe impl Sync for DriverFunction {}

impl DriverFunction {
    pub(super) fn raw(&self) -> sys::CUfunction {
        self.raw
    }

    pub(super) fn context(&self) -> &Arc<CudaContext> {
        &self.module.ctx
    }
}

impl fmt::Debug for DriverFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DriverFunction({})", self.symbol)
    }
}

impl KernelFunction for DriverFunction {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct DriverLoader {
    ctx: Arc<CudaContext>,
    capability: ComputeCapability,
}

impl DriverLoader {
    /// Retains the primary context of device `ordinal`.
    pub fn new(ordinal: u32) -> Result<DriverLoader, String> {
        let ctx = catch_unwind(AssertUnwindSafe(|| CudaContext::new(ordinal as usize)))
            .map_err(|_| "CUDA driver could not be loaded".to_string())?
            .map_err(text)?;
        DriverLoader::from_context(ctx)
    }

    pub fn from_context(ctx: Arc<CudaContext>) -> Result<DriverLoader, String> {
        let (major, minor) = ctx.compute_capability().map_err(text)?;
        Ok(DriverLoader {
            ctx,
            capability: ComputeCapability::new(major as u32, minor as u32),
        })
    }

    pub fn context(&self) -> &Arc<CudaContext> {
        &self.ctx
    }
}

impl ModuleLoader for DriverLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        Some(self.capability)
    }

    fn load(
        &self,
        image: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        let name = CString::new(symbol).map_err(|e| LoadFailure::Other(e.to_string()))?;
        self.ctx
            .bind_to_thread()
            .map_err(|e| LoadFailure::Other(text(e)))?;
        let mut data = image.bytes.clone();
        data.push(0);
        let raw = unsafe { drv::module::load_data(data.as_ptr().cast()) }
            .map_err(|e| {
                if image.kind == CodeKind::Ptx {
                    LoadFailure::PtxRejected(text(e))
                } else {
                    LoadFailure::Other(text(e))
                }
            })?;
        let module = Arc::new(ModuleOwner {
            ctx: Arc::clone(&self.ctx),
            raw,
        });
        let function = unsafe { drv::module::get_function(raw, name) }
            .map_err(|e| {
                if e.0 == sys::CUresult::CUDA_ERROR_NOT_FOUND {
                    LoadFailure::MissingSymbol(symbol.to_string())
                } else {
                    LoadFailure::Other(text(e))
                }
            })?;
        Ok(
            Arc::new(DriverFunction {
                symbol: symbol.to_string(),
                raw: function,
                module,
            }),
        )
    }
}

#[cfg(test)]
#[path = "test/driver_loader.rs"]
mod tests;
