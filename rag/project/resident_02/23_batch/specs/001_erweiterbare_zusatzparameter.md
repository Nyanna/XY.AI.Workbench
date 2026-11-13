# Spezifikation: Erweiterbare Zusatzparameter (Build-Order 1)

## 1. Task, restrictions and boundaries

Implement in Rust a typed extension map, `Extensions`. It is the only means by which addon tasks carry additional parameters, for example the scope of a follow-up task.

- The map holds at most one value per type.
- It is cheap to clone: clones share the stored values.
- It is `Send + Sync`.
- Restrictions:
  - The core never interprets the values.
  - There is no serialization, no persistence, no named keys and no iteration over values.
  - The module has no dependencies other than `std`. Don't add external crates.
  - The module has no error type. All operations are infallible.

## 2. Context

The core passes `Extensions` unchanged through the queue and through planning, as part of a task's parameter object. Addons put their own typed values in and read them back by type. Because the map crosses threads, it must be `Send + Sync`. Because tasks are cloned, cloning must not duplicate the values. Neighboring modules only use this public interface.

## 3. Interface

```rust
use std::any::Any;

#[derive(Clone, Default)]
pub struct Extensions { /* private; values held as shared (Arc) type-erased entries keyed by TypeId */ }

impl Extensions {
    pub fn new() -> Self;

    /// Stores `value`; replaces an existing value of type T.
    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, value: T);

    /// Value of type T, or None.
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T>;

    /// Removes the value of type T; true if one was present.
    pub fn remove<T: Any + Send + Sync + 'static>(&mut self) -> bool;

    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl std::fmt::Debug for Extensions {} // manual impl; prints at least len; must not require T: Debug
```

`Extensions` must satisfy `Send + Sync + 'static`. Add a compile-time assertion for this, for example `fn _assert<T: Send + Sync>() {}`.

## 4. Behavior and invariants

- `new()` and `default()` return an empty map: `len() == 0` and `is_empty() == true`.
- `insert::<T>(v)`, then `get::<T>()`, returns `Some(&v)`.
- A second `insert` of the same `T` replaces the value, and `len` is unchanged. Values of different types coexist, and `len` counts the distinct types.
- `get::<T>()` returns `None` if no `T` is stored. Types are distinguished exactly by `TypeId`. For example, `String` and `&'static str` are different types, and `u32` and `u64` are different types.
- `remove::<T>()` returns `true` if a `T` was present and removed. It returns `false` and changes nothing otherwise. After a removal, `get::<T>()` returns `None` and `len` decreases by 1.
- `is_empty() == (len() == 0)` always holds.
- `clone()` shares the stored values (`Arc`) and does not clone `T`. A `T` that is not `Clone` is therefore fine, and `Extensions::clone` imposes no `Clone` bound on stored values.
  - The clone has a map of its own. A later `insert` or `remove` on one map does not affect the other.
  - A value present at the time of cloning is the very same instance in both maps. `get` on both returns references to the same address.
  - A value is dropped when the last map holding it is dropped.
- Values are immutable once stored. There is no `get_mut`. Replacement only happens through `insert`.
- Concurrency:
  - `Extensions` may be moved to other threads and shared by reference across threads. Concurrent `get`, `len` and `is_empty` on a shared reference are safe.
  - Mutation requires `&mut self`. There is no internal locking and no interior mutability.
  - Clones can be used independently on different threads.
- Passing the map through queue and planning leaves its content unchanged.

## 5. Relied-on elements

None.