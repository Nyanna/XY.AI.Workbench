# Backpressure und Expansionssteuerung (module `backpressure`, build order 130)

## 1. Task, restrictions and boundaries

**Task.** Implement the Rust module `backpressure`. It does three things:
- It holds the Idle-Notifications as a **set per sub-engine kind** (`ResourceKind`), not as a queue.
- It lets a notification expire on a hit or when the resource is busy.
- It decides whether a task may be expanded now, and which waiting task is chosen first. A task may be expanded only if there is an Idle-Notification or the operations queue is empty. A task whose first operation needs an underserved resource is preferred.

**Restrictions.**
- The module does not expand tasks, call the Planner, or touch queues. It does not claim or enqueue anything.
- It does not run sub-engine loops. It does not read clocks and has no timers.
- It has no I/O, no globals and no `unsafe`. It uses only `std` and the types of the dependencies.
- It invents no further policies. There is no priority interpretation, no consumption of notifications by expansion, and no time-based expiry.

## 2. Context

- **Subengine-Loop** produces `IterationOutcome{claimed, inits_started, idle: Vec<IdleNotice>, evicted}` per iteration. `idle` is non-empty only if `claimed == 0`. Each `IdleNotice` is a free resource with its kind and requirements, i.e. the resource kind and parameters.
- **Auslastungsmeldung** provides `LoadReport{kind, device, capacity, throughput}`. `capacity == 0` means the sub-engine accepts nothing, i.e. the resource is busy.
- **Planner** reports the resource kind of the first operation of a task kind in advance (`first_resource_kind(&TaskSpec) -> Option<ResourceKind>`, `first_resource_kind_of_task`). The caller passes this value in. This module never calls the Planner.
- **Resource-Requirements** provides `ResourceKind`, the sub-engine kind and the key of the "Art".
- **Queue-Iteration** is not called directly. The caller derives "operations queue empty" from it, e.g. `iter_active(head).next().is_none()` on the operations queues, and passes it in as `bool`.
- **Caller** (the engine or processor feed side): it calls `report_iteration` after each `run_iteration`, `observe_load` for load reports, and `select` before each expansion. The caller then expands the chosen task through the Planner.

## 3. Interface

```rust
use resource_requirements::ResourceKind;
use subengine_loop::{IdleNotice, IterationOutcome};
use auslastungsmeldung::LoadReport;

/// A task waiting for expansion, with the Planner's advance report for its first operation.
#[derive(Debug, Clone)]
pub struct Candidate<T> {
    pub item: T,
    /// Result of Planner::first_resource_kind(_of_task). None = unknown / not reported.
    pub first_kind: Option<ResourceKind>,
}

/// Idle-Notification sets per sub-engine kind plus expansion decisions.
#[derive(Debug, Default)]
pub struct ExpansionControl { /* private: Mutex-protected map ResourceKind -> set of IdleNotice */ }
// static assertion: ExpansionControl: Send + Sync + 'static

impl ExpansionControl {
    pub fn new() -> Self;

    /// Feed one finished iteration of a sub-engine of `kind`.
    pub fn report_iteration(&self, kind: ResourceKind, outcome: &IterationOutcome);

    /// Feed a load report. capacity == 0 means the resource is busy.
    pub fn observe_load(&self, report: &LoadReport);

    /// Marks one resource of `kind` busy (e.g. Loading/Backoff/capacity gone).
    pub fn mark_busy(&self, kind: ResourceKind, key: &str);

    /// Current notifications of one kind (order unspecified, no duplicates).
    pub fn notifications(&self, kind: ResourceKind) -> Vec<IdleNotice>;
    /// All current notifications.
    pub fn snapshot(&self) -> Vec<IdleNotice>;
    /// Kinds with at least one notification = underserved kinds.
    pub fn underserved(&self) -> Vec<ResourceKind>;
    pub fn has_idle(&self) -> bool;

    /// true iff operations_queue_empty || has_idle().
    pub fn may_expand(&self, operations_queue_empty: bool) -> bool;

    /// Chooses the index in `candidates` to expand next, or None if expansion is not allowed
    /// (or candidates is empty). Does not change any state.
    pub fn select<T>(&self, operations_queue_empty: bool, candidates: &[Candidate<T>]) -> Option<usize>;
}
```

`IdleNotice.key` is the `ResourceKey` of `subengine_loop`/`resource_lifecycle` and is used only through `IdleNotice` equality. In `mark_busy` it is matched as `&str`.

## 4. Behavior and invariants

**Set semantics**
- Notifications are stored as a set per `ResourceKind`, not as a queue. There is no ordering and no count.
- Within one kind, a notice is identified by its `key`. Inserting a notice with an existing key **replaces** the old one, so no key occurs twice.
- A notification is never created from anything except `report_iteration`.

**`report_iteration(kind, outcome)`**
- `outcome.claimed > 0` is a **hit**: it removes **all** notifications of `kind`. Nothing from `outcome.idle` is inserted.
- `outcome.claimed == 0`: every notice in `outcome.idle` is inserted under `kind`. The `kind` parameter decides the set, not `notice.kind`. Existing notifications of that kind that are not in `idle` stay.
- `claimed == 0` with an empty `idle` (no node visited) changes nothing.
- `inits_started` and `evicted` are ignored.

**Expiry on busy**
- `mark_busy(kind, key)` removes the notification of that kind with that key. An unknown key is a no-op.
- `observe_load(r)` with `r.capacity == 0` removes **all** notifications of `r.kind`. The report does not identify the resource key. Reports with `capacity > 0` change nothing and create no notification.
- Notifications otherwise never expire: no time decay, and expansion does not consume them.

**Expansion permission**
- `may_expand(empty)` is `empty || has_idle()`.
- `select` returns `None` if `candidates` is empty or `may_expand(empty)` is false.
- Otherwise `select` returns the index of the **first** candidate whose `first_kind` is `Some(k)` with `k` in `underserved()`. If there is none, it returns index `0`, the oldest candidate. This fallback also applies to candidates with `first_kind == None`.
- Candidate order is the caller's fairness order, and `select` keeps it among equals.
- When permission comes from an empty queue with no notifications, no kind is underserved, so the result is `Some(0)`.
- `select` is pure and repeatable for an unchanged state.

**Concurrency**
- All methods take `&self` and can run from any thread concurrently. State sits behind one short-lived `Mutex`, and no lock is held across callbacks (there are none).
- A poisoned lock is recovered by taking the inner state, and nothing panics because of poisoning.
- Each method sees a consistent state. `select` and `may_expand` read one snapshot. A decision may be outdated right after it returns, which is accepted.

**Errors**
- There are no error values and no panics for any input.

## 5. Relied-on elements

- `subengine_loop`:
  - `IdleNotice{key: ResourceKey, kind: ResourceKind, requirements: ResourceRequirements}`. It is `Debug + Clone + PartialEq + Eq`.
  - `IterationOutcome{claimed: usize, inits_started: usize, idle: Vec<IdleNotice>, evicted: Vec<ResourceKey>}`.
  - `idle` is filled only if `claimed == 0`.
  - `progress() == claimed > 0`.
- `auslastungsmeldung`: `LoadReport{kind: ResourceKind, device: DeviceHandle, capacity: usize, throughput: f64}`. `capacity == 0` means it accepts nothing.
- `resource_requirements`: `ResourceKind` is `#[non_exhaustive]` and `Copy + Eq + Hash + Ord`. Handle it only through equality and hashing, with no exhaustive matches.
- Planner (used by the caller, not here): `first_resource_kind(&TaskSpec) -> Option<ResourceKind>`. For Embedding it is `Tokenizer`, for a Similarity with a scorer it is `Scorer`, for a Similarity with a cross-encoder it is `Tokenizer`, and for `TaskSpec::Other` it is `None`. It equals the `resource.kind()` of the first operation produced.
- Queue-Iteration (used by the caller): `iter_active(head)` is non-blocking, and an empty iteration means no active entry.