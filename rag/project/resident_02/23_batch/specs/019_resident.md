# Resident: Technical Specification (build order 19)

## 1. Task, restrictions and boundaries

**Task.** Resident is a long-running engine, written in Rust, for **Embedding** (text → vector) and **Similarity** (nearest-neighbour search of a query vector against an index). A RAG application can start it in-process. It can also run as a server process, as a daemon, or be used through the CLI.

**Structure.** It has two sub-engines, **CPU** and **GPU (CUDA)**. Each sub-engine has its own workers. The workers pull tasks from queues (pull principle). Nothing pushes work to a worker.

**Guiding principles to enforce:**
- Self-organization: workers decide themselves what to pull and when.
- Decoupling: the submitter and the executor share nothing except the queue and the result slot.
- Non-blocking: `submit_*` never waits for computation.
- Lean core: the core contains only the queues, the workers, the backend traits and the result slots.
- Lazy resources: no thread, GPU context, model or memory pool exists before the first task needs it.

**Restrictions (must not be implemented):**
- No iteration, cascade, recursion, parent/child relation between tasks, task dependencies or intermediate state between tasks. Each task is atomic and independent. Such behavior belongs to addons.
- No guarantee on results. This covers determinism, completeness or size of the candidate set, result quality, and freshness. Variance from fusion, atomics and hardware rounding is allowed. Stale results are not errors.
- No locks (`Mutex`, `RwLock`) on data paths: submit, pull, compute, result delivery. Use lock-free structures: atomics, MPMC lock-free queues, and `ArcSwap`-style or epoch-based snapshots for the index. Locks are allowed only on cold paths (init, shutdown, lazy-start arbitration, preferably via `Once`/atomics).
- Do not implement the model itself, the index structure, tokenization or the CUDA kernels. Applications or addons supply them through the backend traits in §3. The core only schedules.
- No persistence, no retry, no priorities, no result caching.

**Dependencies:** none. Use only the Rust std library and general crates (lock-free queues, serde/JSON for the CLI and server).

## 2. Context

Resident is a base engine near the top of the build order. RAG applications and addons (iteration, cascades, re-ranking, quality control) sit above it. They build every higher-level behavior from independent Resident tasks. Accuracy and completeness are the responsibility of those outer layers. Resident's task is throughput and utilization.

## 3. Interface

```rust
use std::{sync::Arc, time::Duration};

pub type Vector = Vec<f32>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target { Cpu, Gpu, Any }   // Any: either sub-engine may pull it

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidentError {
    Overloaded,            // target queue full; task not accepted
    ShutDown,              // engine shut down
    BackendUnavailable,    // no backend registered for the target, or lazy init failed (e.g. no CUDA)
    Backend(String),       // backend reported an error for this task
    Cancelled,             // task dropped at shutdown before execution
    Timeout,               // wait_timeout elapsed (the task stays pending)
}

/// Supplied by the application/addon. One instance per sub-engine; shared by its workers.
pub trait EmbedBackend: Send + Sync + 'static {
    /// Dimension of the produced vectors.
    fn dim(&self) -> usize;
    /// Called at most once, lazily, on the first task for this sub-engine, from a worker.
    fn warm_up(&self) -> Result<(), String> { Ok(()) }
    /// Embeds a batch. Output length must equal input length.
    fn embed(&self, texts: &[String]) -> Result<Vec<Vector>, String>;
}

/// A searchable index. `snapshot()` must be lock-free and cheap.
pub trait SimilarityIndex: Send + Sync + 'static {
    /// Returns up to `k` (id, score) pairs, higher score = more similar.
    /// May return fewer than `k`, in any order guarantee-free sense except
    /// that the returned vec is sorted by descending score.
    fn search(&self, query: &[f32], k: usize, device: Target) -> Result<Vec<Hit>, String>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit { pub id: u64, pub score: f32 }

#[derive(Clone, Debug)]
pub struct Config {
    pub cpu_workers: usize,          // default: available_parallelism(); 0 disables CPU sub-engine
    pub gpu_workers: usize,          // default: 1; 0 disables GPU sub-engine
    pub queue_capacity: usize,       // per queue, default 4096
    pub max_batch: usize,            // max embed tasks merged per backend call, default 32
}
impl Default for Config { /* values as commented above */ }

pub struct ResidentBuilder { /* private */ }
impl ResidentBuilder {
    pub fn new(config: Config) -> Self;
    pub fn cpu_embedder(self, b: Arc<dyn EmbedBackend>) -> Self;
    pub fn gpu_embedder(self, b: Arc<dyn EmbedBackend>) -> Self;
    pub fn index(self, name: &str, idx: Arc<dyn SimilarityIndex>) -> Self; // registered before build
    pub fn build(self) -> Resident;   // infallible, spawns nothing, allocates only the queues' bookkeeping
}

pub struct Resident { /* private */ }
// Resident: Send + Sync; all methods take &self.
impl Resident {
    /// Non-blocking. Never computes on the caller's thread.
    pub fn submit_embed(&self, text: String, target: Target) -> Result<Ticket<Vector>, ResidentError>;
    pub fn submit_search(&self, index: &str, query: Vector, k: usize, target: Target)
        -> Result<Ticket<Vec<Hit>>, ResidentError>;
    /// Replaces/adds an index atomically (lock-free swap). Running searches keep the old one.
    pub fn set_index(&self, name: &str, idx: Arc<dyn SimilarityIndex>);
    pub fn stats(&self) -> Stats;
    /// Stops accepting tasks, lets workers finish the task in hand, cancels queued tasks, joins threads.
    pub fn shutdown(&self);
}
impl Drop for Resident { /* calls shutdown */ }

pub struct Ticket<T> { /* private; completion slot, Send */ }
impl<T> Ticket<T> {
    pub fn try_take(&mut self) -> Option<Result<T, ResidentError>>;      // non-blocking
    pub fn wait(self) -> Result<T, ResidentError>;                        // blocks the caller only
    pub fn wait_timeout(&mut self, d: Duration) -> Result<T, ResidentError>; // Err(Timeout) keeps the ticket valid
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub cpu_started: bool, pub gpu_started: bool,
    pub queued: usize, pub completed: u64, pub failed: u64, pub rejected: u64,
}
```

**CLI binary `resident`** (JSON on stdout, errors on stderr, nonzero exit code on error):
- `resident serve --listen <addr> [--daemon]`: server. It accepts newline-delimited JSON requests on the TCP address or on a Unix socket path, one JSON response line per request.
  - Requests: `{"op":"embed","text":..,"target":"cpu|gpu|any"}` and `{"op":"search","index":..,"query":[..],"k":N,"target":..}`.
  - Responses: `{"ok":..}` or `{"err":"<ResidentError variant>"}`.
  - `--daemon` detaches the process from the terminal.
- `resident embed <text> [--target ..]`
- `resident search <index> <vector-json> -k N`

Backends and indexes for the binary are provided through a registration hook (`pub fn register_defaults(b: ResidentBuilder) -> ResidentBuilder`, supplied by the embedding application or crate feature). Without registered backends, the binary returns `BackendUnavailable`.

## 4. Behavior and invariants

**Lifecycle and laziness**
- After `build()` returns, `submit_*` is immediately usable. Before the first submit, no worker thread exists, no backend `warm_up` has run, and no GPU context has been created.
- The first accepted task for a sub-engine starts that sub-engine's workers (once, race-free via atomics). Then `warm_up` runs once on a worker. If `warm_up` fails, that sub-engine is marked failed, and its queued and future tasks complete with `BackendUnavailable`. This does not affect the other sub-engine.
- A sub-engine with 0 workers or no backend counts as not present. A `submit` with `Target::Cpu` or `Target::Gpu` for a sub-engine that is not present returns `Err(BackendUnavailable)` immediately.
- `Target::Any` is accepted if at least one sub-engine is present.

**Queues and pull**
- There is one queue per sub-engine (Cpu, Gpu) plus one shared queue for `Any`. They are bounded and lock-free MPMC. A worker first pulls from its own queue, then from the shared queue. It never pulls from the other sub-engine's queue.
- The shared queue may only be pulled by started workers. If only one sub-engine is present, `Any` goes to that one.
- Submission to a full queue returns `Err(Overloaded)` immediately. It never blocks and never evicts.
- A worker only pulls when it is idle. Idle workers park (without busy-spinning) and are woken by submit.
- Workers may merge up to `max_batch` pending embed tasks from the queue into one `embed` call. Each task gets its own result. If a batch call fails, every task in it gets `Backend(msg)`. No other task is affected.
- Search tasks are executed individually against the index snapshot taken at execution start.

**Task independence**
- Tasks have no ids, parents, dependencies or ordering guarantees. Completion order may differ from submission order.
- A worker holds no state between tasks that influences results.

**Results**
- Each ticket completes exactly once, with a value or an error. Completion uses a lock-free slot. `wait` blocks only the calling thread.
- An embed result has length `backend.dim()` unless the backend violates its contract. If the backend returns the wrong output length, the affected tasks get `Backend(..)`.
- No guarantee of reproducibility, ranking stability, candidate-set size (`≤ k`, possibly empty) or completeness. Stale results (from an index replaced during execution) are valid.
- A search with an unknown index name returns `Backend("unknown index")` through the ticket. It does not fail at submit.
- A search with `k == 0` completes with an empty vec without calling the index.

**Shutdown**
- After `shutdown()`, `submit_*` returns `Err(ShutDown)`. Queued, not-yet-started tasks complete with `Cancelled`. Tasks in progress complete normally. `shutdown` is idempotent and joins all workers.
- A dropped `Ticket` neither cancels nor blocks the task. The result is discarded.
- A panic in a backend call is caught per task or batch. Those tasks get `Backend("panic")` and the worker continues.

**Concurrency**
- All `Resident` methods are safe to call concurrently from any thread. No locks on submit, pull, compute or result delivery.
- `set_index` takes effect for searches that start afterwards. Searches already running are unaffected.
- `Stats` counters are atomic and approximate. They give no consistency guarantee between fields.

**Server/CLI**
- The server handles each connection independently. It never blocks the accept loop on computation. Overload maps to `{"err":"Overloaded"}`.
- On SIGTERM or SIGINT it calls `shutdown()` and exits with code 0.

## 5. Relied-on elements

None (no dependencies).