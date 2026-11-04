//! Resident: lazy pull-based engine for embedding and similarity tasks.

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering::*};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{self, JoinHandle, Thread};
use std::time::{Duration, Instant};

use arc_swap::ArcSwap;
use serde_json::{json, Value};

use super::queue::{self, Queue, ResultError};

pub type Vector = Vec<f32>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Cpu,
    Gpu,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidentError {
    Overloaded,
    ShutDown,
    BackendUnavailable,
    Backend(String),
    Cancelled,
    Timeout,
}

impl ResidentError {
    pub fn variant_name(&self) -> &'static str {
        match self {
            ResidentError::Overloaded => "Overloaded",
            ResidentError::ShutDown => "ShutDown",
            ResidentError::BackendUnavailable => "BackendUnavailable",
            ResidentError::Backend(_) => "Backend",
            ResidentError::Cancelled => "Cancelled",
            ResidentError::Timeout => "Timeout",
        }
    }
}

impl std::fmt::Display for ResidentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResidentError::Backend(m) => write!(f, "Backend: {m}"),
            other => f.write_str(other.variant_name()),
        }
    }
}

impl std::error::Error for ResidentError {}

pub trait EmbedBackend: Send + Sync + 'static {
    fn dim(&self) -> usize;
    fn warm_up(&self) -> Result<(), String> {
        Ok(())
    }
    fn embed(&self, texts: &[String]) -> Result<Vec<Vector>, String>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub id: u64,
    pub score: f32,
}

pub trait SimilarityIndex: Send + Sync + 'static {
    fn search(&self, query: &[f32], k: usize, device: Target) -> Result<Vec<Hit>, String>;
}

#[derive(Clone, Debug)]
pub struct Config {
    pub cpu_workers: usize,
    pub gpu_workers: usize,
    pub queue_capacity: usize,
    pub max_batch: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cpu_workers: thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            gpu_workers: 1,
            queue_capacity: 4096,
            max_batch: 32,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub cpu_started: bool,
    pub gpu_started: bool,
    pub queued: usize,
    pub completed: u64,
    pub failed: u64,
    pub rejected: u64,
}

// ---------------------------------------------------------------------------
// Work items, lanes and tickets
// ---------------------------------------------------------------------------

enum Work {
    Embed(String),
    Search { index: String, query: Vector, k: usize },
}

enum Output {
    Vector(Vector),
    Hits(Vec<Hit>),
}

type WorkQueue = Queue<(), Work, Output, ResidentError>;
type QueueTicket = queue::Ticket<(), Work, Output, ResidentError>;
type Claim = queue::Claim<(), Work, Output, ResidentError>;

/// Bounded view over the unbounded queue.
struct Lane {
    queue: WorkQueue,
    reserved: AtomicUsize,
    capacity: usize,
}

impl Lane {
    fn new(capacity: usize) -> Self {
        Lane { queue: Queue::new(), reserved: AtomicUsize::new(0), capacity }
    }

    fn push(&self, work: Work) -> Option<QueueTicket> {
        if self.reserved.fetch_add(1, SeqCst) >= self.capacity {
            self.reserved.fetch_sub(1, SeqCst);
            return None;
        }
        Some(self.queue.push((), work))
    }

    fn take(&self) -> Option<Claim> {
        let claim = self.queue.take()?;
        self.reserved.fetch_sub(1, SeqCst);
        Some(claim)
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

pub struct Ticket<T> {
    inner: QueueTicket,
    convert: fn(Output) -> Option<T>,
    taken: bool,
}

impl<T> Ticket<T> {
    pub fn try_take(&mut self) -> Option<Result<T, ResidentError>> {
        if self.taken {
            return None;
        }
        let out = match self.inner.take_result() {
            Ok(r) => r,
            Err(ResultError::NotReady) | Err(ResultError::Taken) => return None,
            Err(ResultError::Abandoned) => Err(ResidentError::Cancelled),
        };
        self.taken = true;
        Some(out.and_then(|o| {
            (self.convert)(o).ok_or_else(|| ResidentError::Backend("result type mismatch".into()))
        }))
    }

    pub fn wait(mut self) -> Result<T, ResidentError> {
        if self.taken {
            return Err(ResidentError::Cancelled);
        }
        self.inner.wait(None);
        self.try_take().unwrap_or(Err(ResidentError::Cancelled))
    }

    pub fn wait_timeout(&mut self, d: Duration) -> Result<T, ResidentError> {
        if self.taken {
            return Err(ResidentError::Cancelled);
        }
        if !self.inner.wait(Some(d)).is_final() {
            return Err(ResidentError::Timeout);
        }
        self.try_take().unwrap_or(Err(ResidentError::Cancelled))
    }
}

// ---------------------------------------------------------------------------
// Engine internals
// ---------------------------------------------------------------------------

enum Job {
    Embed(Claim, String),
    Search { claim: Claim, index: String, query: Vector, k: usize },
}

impl Job {
    fn from_claim(mut claim: Claim) -> Job {
        match claim.take_payload() {
            Some(Work::Embed(text)) => Job::Embed(claim, text),
            Some(Work::Search { index, query, k }) => Job::Search { claim, index, query, k },
            None => Job::Embed(claim, String::new()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Cpu,
    Gpu,
}

impl Kind {
    fn other(self) -> Kind {
        match self {
            Kind::Cpu => Kind::Gpu,
            Kind::Gpu => Kind::Cpu,
        }
    }
    fn target(self) -> Target {
        match self {
            Kind::Cpu => Target::Cpu,
            Kind::Gpu => Target::Gpu,
        }
    }
}

const ENGINE_NEW: u8 = 0;
const ENGINE_RUNNING: u8 = 1;
const ENGINE_FAILED: u8 = 2;

struct Engine {
    backend: Option<Arc<dyn EmbedBackend>>,
    workers: usize,
    lane: Lane,
    state: AtomicU8,
    spawning: AtomicBool,
    threads: OnceLock<Vec<Thread>>,
    warm: OnceLock<Result<(), String>>,
}

impl Engine {
    fn new(backend: Option<Arc<dyn EmbedBackend>>, workers: usize, capacity: usize) -> Self {
        let backend = if workers > 0 { backend } else { None };
        Engine {
            backend,
            workers,
            lane: Lane::new(capacity),
            state: AtomicU8::new(ENGINE_NEW),
            spawning: AtomicBool::new(false),
            threads: OnceLock::new(),
            warm: OnceLock::new(),
        }
    }
    fn usable(&self) -> bool {
        self.backend.is_some() && self.state.load(SeqCst) != ENGINE_FAILED
    }
    fn wake(&self) {
        if let Some(ts) = self.threads.get() {
            for t in ts {
                t.unpark();
            }
        }
    }
}

type IndexMap = HashMap<String, Arc<dyn SimilarityIndex>>;

struct Inner {
    cpu: Engine,
    gpu: Engine,
    shared: Lane,
    indexes: ArcSwap<IndexMap>,
    max_batch: usize,
    shut: AtomicBool,
    handles: Mutex<Vec<JoinHandle<()>>>,
    completed: AtomicU64,
    failed: AtomicU64,
    rejected: AtomicU64,
}

impl Inner {
    fn engine(&self, kind: Kind) -> &Engine {
        match kind {
            Kind::Cpu => &self.cpu,
            Kind::Gpu => &self.gpu,
        }
    }

    fn finish(&self, claim: Claim, result: Result<Output, ResidentError>) {
        match &result {
            Ok(_) => self.completed.fetch_add(1, Relaxed),
            Err(_) => self.failed.fetch_add(1, Relaxed),
        };
        claim.complete(result);
    }

    fn sweep(&self, lane: &Lane, err: &ResidentError) {
        while let Some(claim) = lane.take() {
            self.finish(claim, Err(err.clone()));
        }
    }

    fn sweep_all(&self, err: &ResidentError) {
        self.sweep(&self.cpu.lane, err);
        self.sweep(&self.gpu.lane, err);
        self.sweep(&self.shared, err);
    }

    fn wake_all(&self) {
        self.cpu.wake();
        self.gpu.wake();
    }

    fn ensure_started(self: &Arc<Self>, kind: Kind) {
        let eng = self.engine(kind);
        if eng
            .state
            .compare_exchange(ENGINE_NEW, ENGINE_RUNNING, SeqCst, SeqCst)
            .is_err()
        {
            return;
        }
        eng.spawning.store(true, SeqCst);
        if self.shut.load(SeqCst) {
            eng.spawning.store(false, SeqCst);
            return;
        }
        let mut spawned = Vec::with_capacity(eng.workers);
        for i in 0..eng.workers {
            let inner = Arc::clone(self);
            let name = match kind {
                Kind::Cpu => format!("resident-cpu-{i}"),
                Kind::Gpu => format!("resident-gpu-{i}"),
            };
            match thread::Builder::new()
                .name(name)
                .spawn(move || worker_loop(inner, kind))
            {
                Ok(h) => spawned.push(h),
                Err(_) => break,
            }
        }
        if spawned.is_empty() {
            eng.state.store(ENGINE_FAILED, SeqCst);
            eng.spawning.store(false, SeqCst);
            self.sweep(&eng.lane, &ResidentError::BackendUnavailable);
            return;
        }
        let threads: Vec<Thread> = spawned.iter().map(|h| h.thread().clone()).collect();
        self.handles.lock().unwrap().extend(spawned);
        let _ = eng.threads.set(threads);
        eng.spawning.store(false, SeqCst);
        eng.wake();
    }

    fn on_engine_failed(self: &Arc<Self>, kind: Kind) {
        let eng = self.engine(kind);
        eng.state.store(ENGINE_FAILED, SeqCst);
        self.sweep(&eng.lane, &ResidentError::BackendUnavailable);
        let other = self.engine(kind.other());
        if other.usable() {
            if other.state.load(SeqCst) == ENGINE_NEW && self.shared.len() > 0 {
                self.ensure_started(kind.other());
            }
        } else {
            self.sweep(&self.shared, &ResidentError::BackendUnavailable);
        }
    }

    fn route(&self, target: Target) -> Result<Kind, ResidentError> {
        if self.shut.load(SeqCst) {
            return Err(ResidentError::ShutDown);
        }
        match target {
            Target::Cpu if self.cpu.usable() => Ok(Kind::Cpu),
            Target::Gpu if self.gpu.usable() => Ok(Kind::Gpu),
            Target::Any => {
                let (c, g) = (self.cpu.usable(), self.gpu.usable());
                let running = |e: &Engine| e.state.load(SeqCst) == ENGINE_RUNNING;
                if c && running(&self.cpu) {
                    Ok(Kind::Cpu)
                } else if g && running(&self.gpu) {
                    Ok(Kind::Gpu)
                } else if c {
                    Ok(Kind::Cpu)
                } else if g {
                    Ok(Kind::Gpu)
                } else {
                    Err(ResidentError::BackendUnavailable)
                }
            }
            _ => Err(ResidentError::BackendUnavailable),
        }
    }

    fn submit<T>(
        self: &Arc<Self>,
        target: Target,
        work: Work,
        convert: fn(Output) -> Option<T>,
    ) -> Result<Ticket<T>, ResidentError> {
        let result = self.submit_inner(target, work, convert);
        if result.is_err() {
            self.rejected.fetch_add(1, Relaxed);
        }
        result
    }

    fn submit_inner<T>(
        self: &Arc<Self>,
        target: Target,
        work: Work,
        convert: fn(Output) -> Option<T>,
    ) -> Result<Ticket<T>, ResidentError> {
        let kind = self.route(target)?;
        let lane = if target == Target::Any {
            &self.shared
        } else {
            &self.engine(kind).lane
        };
        let inner = lane.push(work).ok_or(ResidentError::Overloaded)?;
        self.ensure_started(kind);
        self.engine(kind).wake();
        if self.shut.load(SeqCst) {
            self.sweep_all(&ResidentError::Cancelled);
        } else if target == Target::Any {
            if !self.cpu.usable() && !self.gpu.usable() {
                self.sweep(&self.shared, &ResidentError::BackendUnavailable);
            }
        } else if self.engine(kind).state.load(SeqCst) == ENGINE_FAILED {
            self.sweep(&self.engine(kind).lane, &ResidentError::BackendUnavailable);
        }
        Ok(Ticket { inner, convert, taken: false })
    }

    fn pull(&self, kind: Kind) -> Option<Job> {
        self.engine(kind)
            .lane
            .take()
            .or_else(|| self.shared.take())
            .map(Job::from_claim)
    }

    fn execute(&self, kind: Kind, backend: &Arc<dyn EmbedBackend>, first: Job) {
        match first {
            Job::Search { claim, index, query, k } => {
                self.run_search(kind, claim, &index, &query, k)
            }
            Job::Embed(claim, text) => {
                let mut batch = vec![(claim, text)];
                let mut extra = None;
                while batch.len() < self.max_batch {
                    match self.pull(kind) {
                        Some(Job::Embed(c, t)) => batch.push((c, t)),
                        Some(other) => {
                            extra = Some(other);
                            break;
                        }
                        None => break,
                    }
                }
                self.run_embed(backend, batch);
                if let Some(Job::Search { claim, index, query, k }) = extra {
                    self.run_search(kind, claim, &index, &query, k);
                }
            }
        }
    }

    fn run_embed(&self, backend: &Arc<dyn EmbedBackend>, items: Vec<(Claim, String)>) {
        let (claims, texts): (Vec<Claim>, Vec<String>) = items.into_iter().unzip();
        let out = catch_unwind(AssertUnwindSafe(|| backend.embed(&texts)));
        let checked: Result<Vec<Vector>, String> = match out {
            Err(_) => Err("panic".into()),
            Ok(Err(m)) => Err(m),
            Ok(Ok(v)) if v.len() != claims.len() => Err("backend returned wrong batch length".into()),
            Ok(Ok(v)) => Ok(v),
        };
        match checked {
            Err(m) => {
                for c in claims {
                    self.finish(c, Err(ResidentError::Backend(m.clone())));
                }
            }
            Ok(vecs) => {
                let dim = backend.dim();
                for (c, v) in claims.into_iter().zip(vecs) {
                    if v.len() == dim {
                        self.finish(c, Ok(Output::Vector(v)));
                    } else {
                        let e = ResidentError::Backend("wrong vector dimension".into());
                        self.finish(c, Err(e));
                    }
                }
            }
        }
    }

    fn run_search(&self, kind: Kind, claim: Claim, index: &str, query: &[f32], k: usize) {
        if k == 0 {
            self.finish(claim, Ok(Output::Hits(Vec::new())));
            return;
        }
        let snapshot = self.indexes.load();
        let Some(idx) = snapshot.get(index) else {
            let e = ResidentError::Backend("unknown index".into());
            self.finish(claim, Err(e));
            return;
        };
        let out = catch_unwind(AssertUnwindSafe(|| idx.search(query, k, kind.target())));
        let result = match out {
            Err(_) => Err(ResidentError::Backend("panic".into())),
            Ok(Err(m)) => Err(ResidentError::Backend(m)),
            Ok(Ok(mut hits)) => {
                hits.truncate(k);
                Ok(Output::Hits(hits))
            }
        };
        self.finish(claim, result);
    }
}

fn worker_loop(inner: Arc<Inner>, kind: Kind) {
    let eng = inner.engine(kind);
    let Some(backend) = eng.backend.clone() else {
        return;
    };
    let warm = eng.warm.get_or_init(|| {
        match catch_unwind(AssertUnwindSafe(|| backend.warm_up())) {
            Ok(r) => r,
            Err(_) => Err("panic".to_string()),
        }
    });
    if warm.is_err() {
        inner.on_engine_failed(kind);
        return;
    }
    loop {
        if inner.shut.load(SeqCst) {
            break;
        }
        match inner.pull(kind) {
            Some(job) => inner.execute(kind, &backend, job),
            None => {
                if inner.shut.load(SeqCst) {
                    break;
                }
                thread::park();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Public facade
// ---------------------------------------------------------------------------

pub struct ResidentBuilder {
    config: Config,
    cpu: Option<Arc<dyn EmbedBackend>>,
    gpu: Option<Arc<dyn EmbedBackend>>,
    indexes: IndexMap,
}

impl ResidentBuilder {
    pub fn new(config: Config) -> Self {
        ResidentBuilder { config, cpu: None, gpu: None, indexes: HashMap::new() }
    }

    pub fn cpu_embedder(mut self, b: Arc<dyn EmbedBackend>) -> Self {
        self.cpu = Some(b);
        self
    }

    pub fn gpu_embedder(mut self, b: Arc<dyn EmbedBackend>) -> Self {
        self.gpu = Some(b);
        self
    }

    pub fn index(mut self, name: &str, idx: Arc<dyn SimilarityIndex>) -> Self {
        self.indexes.insert(name.to_string(), idx);
        self
    }

    pub fn build(self) -> Resident {
        let cap = self.config.queue_capacity.max(1);
        let inner = Inner {
            cpu: Engine::new(self.cpu, self.config.cpu_workers, cap),
            gpu: Engine::new(self.gpu, self.config.gpu_workers, cap),
            shared: Lane::new(cap),
            indexes: ArcSwap::from_pointee(self.indexes),
            max_batch: self.config.max_batch.max(1),
            shut: AtomicBool::new(false),
            handles: Mutex::new(Vec::new()),
            completed: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            rejected: AtomicU64::new(0),
        };
        Resident { inner: Arc::new(inner) }
    }
}

pub struct Resident {
    inner: Arc<Inner>,
}

fn into_vector(o: Output) -> Option<Vector> {
    match o {
        Output::Vector(v) => Some(v),
        _ => None,
    }
}

fn into_hits(o: Output) -> Option<Vec<Hit>> {
    match o {
        Output::Hits(h) => Some(h),
        _ => None,
    }
}

impl Resident {
    pub fn submit_embed(&self, text: String, target: Target) -> Result<Ticket<Vector>, ResidentError> {
        self.inner.submit(target, Work::Embed(text), into_vector)
    }

    pub fn submit_search(
        &self,
        index: &str,
        query: Vector,
        k: usize,
        target: Target,
    ) -> Result<Ticket<Vec<Hit>>, ResidentError> {
        let work = Work::Search { index: index.to_string(), query, k };
        self.inner.submit(target, work, into_hits)
    }

    pub fn set_index(&self, name: &str, idx: Arc<dyn SimilarityIndex>) {
        self.inner.indexes.rcu(|current| {
            let mut next: IndexMap = (**current).clone();
            next.insert(name.to_string(), Arc::clone(&idx));
            next
        });
    }

    pub fn stats(&self) -> Stats {
        let i = &self.inner;
        Stats {
            cpu_started: i.cpu.state.load(Relaxed) != ENGINE_NEW,
            gpu_started: i.gpu.state.load(Relaxed) != ENGINE_NEW,
            queued: i.cpu.lane.len() + i.gpu.lane.len() + i.shared.len(),
            completed: i.completed.load(Relaxed),
            failed: i.failed.load(Relaxed),
            rejected: i.rejected.load(Relaxed),
        }
    }

    pub fn shutdown(&self) {
        let i = &self.inner;
        i.shut.store(true, SeqCst);
        while i.cpu.spawning.load(SeqCst) || i.gpu.spawning.load(SeqCst) {
            thread::yield_now();
        }
        i.wake_all();
        let handles: Vec<JoinHandle<()>> = std::mem::take(&mut *i.handles.lock().unwrap());
        for h in handles {
            let _ = h.join();
        }
        i.sweep_all(&ResidentError::Cancelled);
    }
}

impl Drop for Resident {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// ---------------------------------------------------------------------------
// Line protocol handler
// ---------------------------------------------------------------------------

fn parse_target(v: &Value) -> Option<Target> {
    match v.get("target").and_then(Value::as_str) {
        None | Some("any") => Some(Target::Any),
        Some("cpu") => Some(Target::Cpu),
        Some("gpu") => Some(Target::Gpu),
        _ => None,
    }
}

fn err_line(e: &str) -> String {
    json!({ "err": e }).to_string()
}

/// Handles one newline-delimited JSON request and returns one response line.
pub fn handle_request(resident: &Resident, line: &str) -> String {
    let Ok(v) = serde_json::from_str::<Value>(line) else {
        return err_line("BadRequest");
    };
    let Some(target) = parse_target(&v) else {
        return err_line("BadRequest");
    };
    match v.get("op").and_then(Value::as_str) {
        Some("embed") => {
            let Some(text) = v.get("text").and_then(Value::as_str) else {
                return err_line("BadRequest");
            };
            match resident
                .submit_embed(text.to_string(), target)
                .and_then(|t| t.wait())
            {
                Ok(vec) => json!({ "ok": vec }).to_string(),
                Err(e) => err_line(e.variant_name()),
            }
        }
        Some("search") => {
            let index = v.get("index").and_then(Value::as_str);
            let query: Option<Vec<f32>> = v
                .get("query")
                .and_then(Value::as_array)
                .and_then(|a| a.iter().map(|x| x.as_f64().map(|f| f as f32)).collect());
            let k = v.get("k").and_then(Value::as_u64);
            let (Some(index), Some(query), Some(k)) = (index, query, k) else {
                return err_line("BadRequest");
            };
            match resident
                .submit_search(index, query, k as usize, target)
                .and_then(|t| t.wait())
            {
                Ok(hits) => {
                    let list: Vec<Value> = hits
                        .iter()
                        .map(|h| json!({ "id": h.id, "score": h.score }))
                        .collect();
                    json!({ "ok": list }).to_string()
                }
                Err(e) => err_line(e.variant_name()),
            }
        }
        _ => err_line("BadRequest"),
    }
}

#[cfg(test)]
#[path = "test/resident.rs"]
mod tests;
