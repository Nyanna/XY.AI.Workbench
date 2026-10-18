use std::time::Duration;

use crate::resident::facade::WorkerConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ProcessorConfig {
    pub cpu_workers: usize,
    pub gpu_workers: usize,
    /// Lower waterline of queued operations before the planner is asked for more.
    pub min_op_buffer: usize,
    /// Nice value for CPU workers while GPU workers exist; `0` disables.
    pub cpu_nice: i32,
    pub starved_ttl: Duration,
}

impl ProcessorConfig {
    pub fn new(cpu_workers: usize, gpu_workers: usize) -> Self {
        Self {
            cpu_workers,
            gpu_workers,
            min_op_buffer: 2,
            cpu_nice: 5,
            starved_ttl: Duration::from_millis(200),
        }
    }

    pub fn with_min_op_buffer(mut self, n: usize) -> Self {
        self.min_op_buffer = n;
        self
    }

    pub fn with_cpu_nice(mut self, nice: i32) -> Self {
        self.cpu_nice = nice;
        self
    }

    pub fn with_starved_ttl(mut self, ttl: Duration) -> Self {
        self.starved_ttl = ttl;
        self
    }
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

impl From<WorkerConfig> for ProcessorConfig {
    fn from(w: WorkerConfig) -> Self {
        Self::new(w.cpu, w.gpu)
    }
}
