use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use super::kernel_scoring::Metric;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    kernel_resource_key, CpuScoreKernel, Kernel, KernelError, KernelProvider, Variant,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::cpu_compute::{
    dot_with, sq_dist_with, Isa,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
type CpuKernelKey = (String, DType);
struct LiveKernel {
    handle: Arc<dyn CpuScoreKernel>,
    last_used: Instant,
}
struct KernelHandle(Arc<dyn CpuScoreKernel>);
impl CpuScoreKernel for KernelHandle {
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]) {
        self.0.score(query, rows, dim, out);
    }
}
#[derive(Default)]
struct KernelState {
    defined: HashMap<CpuKernelKey, Arc<dyn CpuScoreKernel>>,
    live: HashMap<CpuKernelKey, LiveKernel>,
}
/// Register of CPU score kernels. Kernels are defined once and become live on first request.
pub struct CpuKernelRegister {
    state: Mutex<KernelState>,
}
impl CpuKernelRegister {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(KernelState::default()),
        }
    }
    pub fn with_builtin() -> Self {
        let register = Self::new();
        let isa = Isa::detect();
        for metric in [Metric::Dot, Metric::NegL2Sq] {
            let kernel: Arc<dyn CpuScoreKernel> = Arc::new(BuiltinKernel {
                metric,
                isa,
            });
            register
                .register(metric.kernel_name(), DType::F32, kernel)
                .expect("builtin kernel names are unique");
        }
        register
    }
    pub fn register(
        &self,
        name: &str,
        dtype: DType,
        kernel: Arc<dyn CpuScoreKernel>,
    ) -> Result<(), KernelError> {
        let mut st = lock(&self.state);
        let key = (name.to_string(), dtype);
        if st.defined.contains_key(&key) {
            return Err(
                KernelError::Unavailable(
                    format!("kernel '{name}' for {dtype:?} is already registered"),
                ),
            );
        }
        st.defined.insert(key, kernel);
        Ok(())
    }
    pub fn request_at(
        &self,
        name: &str,
        dtype: DType,
        now: Instant,
    ) -> Result<Arc<dyn CpuScoreKernel>, KernelError> {
        let mut st = lock(&self.state);
        let key = (name.to_string(), dtype);
        if let Some(live) = st.live.get_mut(&key) {
            live.last_used = now;
            return Ok(Arc::clone(&live.handle));
        }
        let Some(defined) = st.defined.get(&key).cloned() else {
            return Err(KernelError::UnknownKernel(format!("{name} ({dtype:?})")));
        };
        let handle: Arc<dyn CpuScoreKernel> = Arc::new(KernelHandle(defined));
        st.live
            .insert(
                key,
                LiveKernel {
                    handle: Arc::clone(&handle),
                    last_used: now,
                },
            );
        Ok(handle)
    }
    /// Drops unused live kernels idle for at least `ttl`. Returns the `kernel_resource_key` of each dropped kernel.
    pub fn evict_expired(&self, now: Instant, ttl: Duration) -> Vec<String> {
        let mut st = lock(&self.state);
        let expired: Vec<CpuKernelKey> = st
            .live
            .iter()
            .filter(|(_, live)| {
                Arc::strong_count(&live.handle) == 1
                    && now.saturating_duration_since(live.last_used) >= ttl
            })
            .map(|(key, _)| key.clone())
            .collect();
        for key in &expired {
            st.live.remove(key);
        }
        let mut keys: Vec<String> = expired
            .into_iter()
            .map(|(name, dtype)| kernel_resource_key(&name, dtype))
            .collect();
        keys.sort();
        keys
    }
}
impl Default for CpuKernelRegister {
    fn default() -> Self {
        Self::new()
    }
}
impl KernelProvider for CpuKernelRegister {
    fn request(
        &self,
        name: &str,
        dtype: DType,
        _variant: Variant,
    ) -> Result<Kernel, KernelError> {
        self.request_at(name, dtype, Instant::now()).map(Kernel::Cpu)
    }
    fn is_resident(&self, name: &str, dtype: DType) -> bool {
        lock(&self.state).live.contains_key(&(name.to_string(), dtype))
    }
}
struct BuiltinKernel {
    metric: Metric,
    isa: Isa,
}
impl CpuScoreKernel for BuiltinKernel {
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]) {
        for (i, slot) in out.iter_mut().enumerate() {
            let row = i
                .checked_mul(dim)
                .and_then(|start| {
                    start.checked_add(dim).and_then(|end| rows.get(start..end))
                });
            *slot = match row {
                Some(row) if row.len() == query.len() => {
                    row_score(self.isa, self.metric, query, row)
                }
                _ => f32::NAN,
            };
        }
    }
}
fn row_score(isa: Isa, metric: Metric, query: &[f32], row: &[f32]) -> f32 {
    match metric {
        Metric::Dot => dot_with(isa, query, row),
        Metric::NegL2Sq => -sq_dist_with(isa, query, row),
    }
}
#[cfg(test)]
#[path = "test/cpu_kernels.rs"]
mod tests;
