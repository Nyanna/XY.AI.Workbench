use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::thread::JoinHandle;
use crate::resident::facade::TaskQueue;
use super::config::ProcessorConfig;
use super::hub::{Engine, Expander, Hub, Step, Wakeups};
use super::ops::Side;
pub struct Processor {
    hub: Arc<Hub>,
    workers: Vec<JoinHandle<()>>,
}
impl Processor {
    /// `wakeups.relays()` must be the relays of `tasks` so submissions wake the workers.
    /// `gpu` is required when `cfg.gpu_workers > 0`.
    pub fn start(
        cfg: ProcessorConfig,
        tasks: Arc<TaskQueue>,
        wakeups: Wakeups,
        expander: Arc<dyn Expander>,
        cpu: Arc<dyn Engine>,
        gpu: Option<Arc<dyn Engine>>,
    ) -> std::io::Result<Self> {
        let gpu_workers = if gpu.is_some() { cfg.gpu_workers } else { 0 };
        let cfg = ProcessorConfig {
            gpu_workers,
            ..cfg
        };
        let lower_cpu = (gpu_workers > 0 && cfg.cpu_nice != 0).then_some(cfg.cpu_nice);
        let hub = Arc::new(Hub::new(cfg.clone(), tasks, wakeups, expander));
        let mut workers = Vec::with_capacity(cfg.cpu_workers + gpu_workers);
        for index in 0..cfg.cpu_workers {
            workers.push(spawn(&hub, Side::Cpu, index, cpu.clone(), lower_cpu)?);
        }
        if let Some(gpu) = gpu {
            for index in 0..gpu_workers {
                workers.push(spawn(&hub, Side::Gpu, index, gpu.clone(), None)?);
            }
        }
        Ok(Self { hub, workers })
    }
    pub fn hub(&self) -> &Arc<Hub> {
        &self.hub
    }
    pub fn shutdown(&mut self) {
        self.hub.begin_shutdown();
        for _ in 0..self.workers.len().max(1) {
            self.hub.wakeups().cpu.notify_one();
            self.hub.wakeups().gpu.notify_one();
        }
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}
impl Drop for Processor {
    fn drop(&mut self) {
        self.shutdown();
    }
}
fn spawn(
    hub: &Arc<Hub>,
    side: Side,
    index: usize,
    engine: Arc<dyn Engine>,
    nice: Option<i32>,
) -> std::io::Result<JoinHandle<()>> {
    let hub = hub.clone();
    let name = format!(
        "resident-{}-{index}", if side == Side::Gpu { "gpu" } else { "cpu" }
    );
    std::thread::Builder::new()
        .name(name)
        .spawn(move || {
            let span = tracing::info_span!("worker", ? side, index);
            let _guard = span.enter();
            if let Some(nice) = nice {
                lower_priority(nice);
            }
            run(&hub, side, &*engine);
        })
}
fn run(hub: &Hub, side: Side, engine: &dyn Engine) {
    let wake = hub.wakeups().side(side).clone();
    loop {
        let notified = wake.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if hub.is_shutdown() {
            break;
        }
        match catch_unwind(AssertUnwindSafe(|| pass(hub, side, engine))) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(_) => tracing::error!("worker pass panicked"),
        }
        futures::executor::block_on(notified);
    }
}
/// One prioritised pass: device results, device feeding, host work, then expansion.
pub(super) fn pass(hub: &Hub, side: Side, engine: &dyn Engine) -> bool {
    if engine.harvest(hub) || engine.feed(hub) {
        return true;
    }
    match engine.step(hub) {
        Step::Worked => return true,
        Step::Idle(Some(kind)) => hub.starved(side).mark(kind),
        Step::Idle(None) => {}
    }
    hub.expand_if_needed(side)
}
#[cfg(target_os = "linux")]
fn lower_priority(nice: i32) {
    let rc = unsafe {
        let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
        libc::setpriority(libc::PRIO_PROCESS, tid, nice)
    };
    if rc != 0 {
        tracing::warn!(
            nice, error = % std::io::Error::last_os_error(),
            "could not lower worker priority"
        );
    }
}
#[cfg(not(target_os = "linux"))]
fn lower_priority(_nice: i32) {}
