use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WorkerConfig {
    pub cpu_workers: usize,
    pub gpu_workers: usize,
    /// Lower waterline of queued operations before the planner is asked for more.
    pub min_op_buffer: usize,
    pub candle_rayon_threads: Option<usize>,
    /// Nice value for CPU workers while GPU workers exist; `0` disables.
    pub cpu_nice: i32,
    pub starved_ttl: Duration,
    /// Tasks past this age are failed with a timeout when one of their operations is next claimed.
    pub task_timeout: Option<Duration>,
}

impl WorkerConfig {
    pub fn new(cpu_workers: usize, gpu_workers: usize) -> Self {
        Self {
            cpu_workers,
            gpu_workers,
            min_op_buffer: 2,
            candle_rayon_threads: None,
            cpu_nice: 5,
            starved_ttl: Duration::from_millis(200),
            task_timeout: None,
        }
    }

    pub fn with_min_op_buffer(mut self, n: usize) -> Self {
        self.min_op_buffer = n;
        self
    }

    pub fn with_candle_rayon_threads(mut self, n: usize) -> Self {
        self.candle_rayon_threads = Some(n);
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

    pub fn with_task_timeout(mut self, timeout: Duration) -> Self {
        self.task_timeout = Some(timeout);
        self
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self::new(1, 0)
    }
}
