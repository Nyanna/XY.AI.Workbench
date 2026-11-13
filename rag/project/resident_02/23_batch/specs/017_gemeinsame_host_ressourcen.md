# Specification: Gemeinsame Host-Ressourcen (build order 17)

## 1. Task, restrictions and boundaries

Implement a Rust library module that provides **CPU-side resources shared by the CPU sub-engine and the GPU sub-engine**. Two kinds are provided:

- a pool of reusable host byte buffers, and
- a registry of shared, immutable, typed host objects.

All public types must be safe to use from multiple threads at once.

Restrictions:
- There are no dependencies on other modules. Use only `std`. No `unsafe` unless it is unavoidable. No GPU API types.
- The module does not schedule work, run kernels or do device transfers. It also does not interpret buffer contents.
- The module does not define engine-specific resource types. Consumers store their own types through the generic registry.
- The module does no I/O or logging.

## 2. Context

The system has a CPU sub-engine and a GPU sub-engine. Both need host-memory resources: staging and scratch buffers, and shared read-only data such as tables and configuration. This module is the single place where those resources are created and shared. Both sub-engines hold an `Arc<HostResources>` (or a reference to it) and call it concurrently. Nothing else is assumed about other modules.

## 3. Interface

```rust
use std::sync::Arc;
use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostResourceError {
    /// Pool limit would be exceeded.
    PoolExhausted { requested: usize, available: usize },
    /// Requested size is 0 or larger than the pool's max_buffer_size.
    InvalidSize(usize),
    /// Registry key exists with a different value type.
    TypeMismatch { key: String },
    /// Registry key already present (insert_new).
    AlreadyExists { key: String },
}
impl std::fmt::Display for HostResourceError { /* ... */ }
impl std::error::Error for HostResourceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolConfig {
    /// Max total bytes held by live (checked-out) buffers. Idle pooled buffers do not count.
    pub max_total_bytes: usize,
    /// Max size of a single buffer.
    pub max_buffer_size: usize,
}
impl Default for PoolConfig { /* max_total_bytes = 1 GiB, max_buffer_size = 256 MiB */ }

/// Exclusive handle to a zero-initialised byte buffer. Returned to the pool on Drop.
pub struct PooledBuffer { /* private */ }
impl PooledBuffer {
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn as_slice(&self) -> &[u8];
    pub fn as_mut_slice(&mut self) -> &mut [u8];
    /// Convert into an immutable, cheaply clonable shared view. Buffer returns to pool when last clone drops.
    pub fn freeze(self) -> SharedBuffer;
}

/// Immutable shared view; Clone is O(1).
#[derive(Clone)]
pub struct SharedBuffer { /* private */ }
impl SharedBuffer {
    pub fn len(&self) -> usize;
    pub fn as_slice(&self) -> &[u8];
}

pub struct BufferPool { /* private */ }
impl BufferPool {
    pub fn new(config: PoolConfig) -> Self;
    pub fn acquire(&self, len: usize) -> Result<PooledBuffer, HostResourceError>;
    pub fn config(&self) -> PoolConfig;
    /// Bytes currently held by live PooledBuffer/SharedBuffer handles.
    pub fn bytes_in_use(&self) -> usize;
}

pub struct ResourceRegistry { /* private */ }
impl ResourceRegistry {
    pub fn new() -> Self;
    /// Fails with AlreadyExists if key is present (any type).
    pub fn insert_new<T: Any + Send + Sync>(&self, key: &str, value: T) -> Result<Arc<T>, HostResourceError>;
    /// Returns existing value, or creates it with `init` exactly once per key even under races.
    /// TypeMismatch if the key exists with another type.
    pub fn get_or_insert_with<T: Any + Send + Sync, F: FnOnce() -> T>(&self, key: &str, init: F) -> Result<Arc<T>, HostResourceError>;
    /// None if absent or of a different type.
    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>>;
    /// Removes the key; existing Arc clones stay valid. Returns whether it was present.
    pub fn remove(&self, key: &str) -> bool;
    pub fn len(&self) -> usize;
}
impl Default for ResourceRegistry { /* new() */ }

/// Top-level container shared by both sub-engines.
pub struct HostResources {
    pub buffers: BufferPool,
    pub registry: ResourceRegistry,
}
impl HostResources {
    pub fn new(config: PoolConfig) -> Self;
    pub fn shared(config: PoolConfig) -> Arc<HostResources>;
}
impl Default for HostResources { /* new(PoolConfig::default()) */ }

// Required auto-trait guarantees (assert with compile-time tests):
// HostResources, BufferPool, ResourceRegistry, PooledBuffer, SharedBuffer: Send + Sync
```

## 4. Behavior and invariants

**Buffers**
- `acquire(len)` returns a buffer of exactly `len` bytes, all zero. Recycled memory is re-zeroed, so no data from earlier users is visible.
- `len == 0` or `len > max_buffer_size` gives `InvalidSize(len)`.
- If `bytes_in_use + len > max_total_bytes`, it gives `PoolExhausted { requested: len, available: max_total_bytes - bytes_in_use }`. Acquire never blocks.
- `bytes_in_use` increases by `len` on acquire. It decreases by `len` when the `PooledBuffer` drops, or when the last `SharedBuffer` clone drops after `freeze`. `freeze` does not change the count.
- Dropping a buffer makes its memory reusable by later `acquire` calls. Reuse is an optimisation and not observable except through zeroing.
- The accounting is exact under concurrency: `bytes_in_use` never exceeds `max_total_bytes`, and returns to 0 once all handles are dropped.
- A `PooledBuffer` is exclusive (`&mut` access, no aliasing). A `SharedBuffer` is read-only and can be sent to and read from any thread.
- A buffer handle may outlive the `BufferPool` value it came from. The handle keeps the shared internal state alive, for example through an `Arc`, and dropping it never panics.

**Registry**
- Keys are case-sensitive strings. The empty key is allowed.
- Values are stored as `Arc<T>` and are immutable. Use interior mutability in `T` if mutation is needed.
- `get_or_insert_with` runs `init` at most once per key. Concurrent callers for the same key receive the same `Arc` (`Arc::ptr_eq` is true). Callers for different keys do not block each other for longer than the map's own lock. `init` must not call back into the same registry. Such a call may deadlock, and callers are responsible for avoiding it.
- If `init` panics, nothing is stored and the registry stays usable. The registry's locks must not be poisoned permanently, so recover with `into_inner` on poison.
- `insert_new` on an existing key returns `AlreadyExists` and leaves the stored value unchanged.
- `remove` never invalidates `Arc`s that were already handed out. A later `get` after `remove` returns `None`.

**General**
- No operation panics on valid input. Poisoned locks are recovered rather than propagated.
- All operations are linearizable per object. No global ordering across the pool and the registry is promised.

## 5. Relied-on elements

None. This module has no dependencies.