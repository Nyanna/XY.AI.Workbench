# Specification: Auslastungsmeldung (build order 67)

## 1. Task, restrictions and boundaries
Implement the Rust module `auslastungsmeldung` using only `std`. It reports the load of one sub-engine instance, which is one resource entry on one device. A report has four fields:
- kind
- device
- capacity
- smoothed throughput in entries per second

The report is the basis for the Processor's work allocation.

Out of scope:
- Allocating, taking or queueing work. The work comes from the Processor's queues.
- Deciding how much work goes where.
- Submitting work to lanes or events.
- Any behavior not listed here.

Forbidden: `unsafe`, global state, I/O, other crates.

## 2. Context
- The Processor reads reports from several sub-engines. It does this from multiple threads, and it decides the allocation itself.
- A sub-engine owns one `LoadMeter`. It sets the capacity and records the completed entries with their elapsed time. The Processor, or any other thread, reads the report.
- "Art" (kind) is the `ResourceKind` of the sub-engine (Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder). It comes from the Resource-Requirements module.
- Device is the `DeviceHandle` from the Device/Lane/Event module.
- Capacity means how many entries the sub-engine can accept now. It is not a measurement and not a queue.

## 3. Interface
```rust
use std::time::Duration;
use device::DeviceHandle;
use resource_requirements::ResourceKind;

/// Snapshot of the load. Plain value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadReport {
    pub kind: ResourceKind,
    pub device: DeviceHandle,
    /// Entries the sub-engine can accept now.
    pub capacity: usize,
    /// Smoothed throughput in entries per second. Always finite and >= 0.0.
    pub throughput: f64,
}

/// Read side. The Processor uses this trait.
pub trait LoadReporting: Send + Sync {
    fn report(&self) -> LoadReport;
}

/// Default smoothing factor.
pub const DEFAULT_ALPHA: f64 = 0.2;

pub struct LoadMeter { /* interior synchronization */ }
// static assertion: LoadMeter: Send + Sync + 'static

impl LoadMeter {
    /// alpha is clamped to [0.01, 1.0]. NaN becomes DEFAULT_ALPHA.
    /// Initial throughput is 0.0.
    pub fn new(kind: ResourceKind, device: DeviceHandle, capacity: usize) -> Self;
    pub fn with_alpha(kind: ResourceKind, device: DeviceHandle, capacity: usize, alpha: f64) -> Self;

    pub fn kind(&self) -> ResourceKind;
    pub fn device(&self) -> DeviceHandle;

    /// Replaces the capacity.
    pub fn set_capacity(&self, capacity: usize);
    pub fn capacity(&self) -> usize;

    /// Records that `entries` were processed in `elapsed`. Updates the smoothed throughput.
    pub fn record(&self, entries: usize, elapsed: Duration);

    pub fn throughput(&self) -> f64;
}
impl LoadReporting for LoadMeter {}
```
All methods take `&self`. There is no error type.

## 4. Behavior and invariants
**Report**
- `report()` returns `kind` and `device` as given at construction. They never change.
- `report()` returns capacity and throughput from one consistent state. It is never a mix of two concurrent updates.
- A fresh meter reports `throughput == 0.0` and the initial capacity.

**Capacity**
- `set_capacity(n)` makes later `capacity()` and `report().capacity` return `n`. `0` is valid and means the meter can accept nothing.
- The meter never changes capacity on its own, and `record` does not touch it.

**Throughput**
- The sample is `s = entries as f64 / elapsed.as_secs_f64()`.
- The first valid sample sets the throughput to `s`.
- Each later valid sample sets `t = alpha * s + (1 - alpha) * t`.
- A sample is ignored and changes nothing if `elapsed` is zero, or if `s` is not finite.
- `entries == 0` with `elapsed > 0` is valid. It yields `s = 0.0` and lowers the average.
- Throughput is never negative, NaN or infinite.
- No time-based decay happens. Without `record` calls the value stays unchanged.
- The meter never reads a clock. The caller passes the duration.

**Concurrency**
- All methods may be called concurrently from any number of threads.
- Each `record` is atomic with respect to other `record` calls. No update is lost and every valid sample enters the average exactly once. The order between concurrent samples is unspecified.
- `report()` and the getters do not block on long operations. They only wait for short critical sections.
- A poisoned lock is recovered by taking the inner state. Nothing panics because of poisoning.
- The meter allocates no work, holds no queue and never calls back into other modules.

**Errors**
- No method returns an error or panics for any input, including `usize::MAX` entries or a `Duration::MAX` duration.

## 5. Relied-on elements
From Device und Lane und Event (unchanged):
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceHandle { Cpu, Gpu(u32) }
```

From Resource-Requirements (unchanged):
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind { Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder }
```
Both types are `Copy`, `Send + Sync + 'static`.