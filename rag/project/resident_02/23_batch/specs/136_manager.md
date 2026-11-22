# Manager: Technical Specification (build order 136)

Crate module `manager`.

## 1. Task, restrictions and boundaries

**Task.** The module `manager` provides one thing: the trait `LazyIndex`. It is the lazy access to the index that a sub-engine component holds before the index is open.

**Restrictions.**
- There is no slot manager in this module. The mapping of entries to slots, the residency, the update queue, the upload and the eviction of the GPU live in `gpu_manager` (138) and `index_ops` (140). The CPU reads entries directly from the `Index` (137). Neither side has a second manager.
- The module contains no state, no threads, no IO and no `unsafe`. It has no globals.
- Crates: `index_contract` only.

## 2. Context

- **`gpu_manager` (138):** `GpuManager` gets its index as `Arc<dyn LazyIndex>`. In production this is `index_ops::ManagedIndex`, an adapter over `manager_lifecycle::IndexManager`.
- **`cpu_subengine` (137):** `FileLazyIndex` implements the trait. It opens the index file on the first successful `get`.
- **Index.** `index_contract::Index` is the authority. `LazyIndex` only decides when it becomes available.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::Index;

/// Lazily activated index. `get` activates on its first successful call and then returns the same Arc.
/// A failure is not cached. Callers never call it on a lookup path.
pub trait LazyIndex: Send + Sync {
    fn get(&self) -> Result<Arc<dyn Index>, String>;
}
```

## 4. Behavior and invariants

- `get` returns the same `Arc` after its first success. Two `Arc`s from two successful calls point to the same instance (`Arc::ptr_eq`).
- A failed `get` leaves no state. The next call tries again. The error is the `Display` text of the cause.
- Concurrent callers of the first `get` get the same instance. At most one activation succeeds, and no lock is held by the caller while it runs.
- An implementation never panics on valid use.
- Implementations are `Send + Sync`.

## 5. Relied-on elements

- **`index_contract`:** `Index: Send + Sync`.
