# Specification: Profilauswahl (build order 39)

## 1. Task, restrictions and boundaries

Implement a Rust library module with three parts:
- It selects, from the profiles of one subengine, the profile closest to a clamped precision wish.
- It rejects an invalid precision.
- It holds a per-subengine latch. With no active profile, the first task fails and the latch locks the model for that subengine. The lock can be released again.

Restrictions:
- There is no I/O, no hardware or model knowledge, and no profile names.
- The module does not derive profiles and does not execute tasks.
- It has no global mutable state.
- Allowed dependencies: Pareto-Front und Profilableitung (`PrecisionProfile`) and `log`.

## 2. Context

- The subengine (GPU or CPU) obtains its unnamed `PrecisionProfile`s from `derive_profiles`. Each has a `position` in [0,1] (0 = fastest, 1 = most precise) and a `config`.
- The facade and task interface hold one `ProfileSelector` per subengine. For each task they pass the wish (0..1) and the current profile list of that subengine to `select`.
- The scale is relative to the subengine: the same wish (e.g. 0.5) picks different configurations on GPU and CPU, because each has its own profile list.
- This module does not interpret `config`.

## 3. Interface

```rust
use pareto_profiles::PrecisionProfile;   // from "Pareto-Front und Profilableitung"

#[derive(Clone, Debug, PartialEq)]
pub enum SelectionError {
    /// Wish is NaN, or a profile position is NaN or outside [0,1].
    InvalidPrecision,
    /// No active profile (first failing task) or latch already set (later tasks).
    NoActiveProfile,
}
impl std::fmt::Display for SelectionError {}
impl std::error::Error for SelectionError {}

/// Clamp wish to [0,1]. NaN -> Err(InvalidPrecision). ±inf is clamped.
pub fn clamp_wish(wish: f64) -> Result<f64, SelectionError>;

/// Per-subengine selector with latch. One instance per subengine.
#[derive(Debug, Default)]
pub struct ProfileSelector { /* latch state */ }

impl ProfileSelector {
    pub fn new() -> Self;   // unlocked

    /// Select the profile closest to the clamped wish.
    pub fn select<'a>(
        &self,
        wish: f64,
        profiles: &'a [PrecisionProfile],
    ) -> Result<&'a PrecisionProfile, SelectionError>;

    /// True iff at least one active profile exists (`!profiles.is_empty()`).
    /// Pure; does not latch, does not validate.
    pub fn is_satisfiable(profiles: &[PrecisionProfile]) -> bool;

    pub fn is_locked(&self) -> bool;

    /// Release the latch. Returns true iff it was locked before (feedback).
    pub fn unlock(&self) -> bool;
}
```

`ProfileSelector` is `Send + Sync`, and all methods take `&self`.

## 4. Behavior and invariants

**Order of checks in `select`:**
1. The wish is clamped with `clamp_wish`. NaN returns `Err(InvalidPrecision)`. This does not touch the latch, whether it is locked or not.
2. If the latch is set, return `Err(NoActiveProfile)` immediately. The profiles are not examined.
3. If `profiles` is empty, set the latch, log once at `log::warn!`, and return `Err(NoActiveProfile)`. This is the first failing task.
4. If any profile has a position that is NaN or outside [0,1], return `Err(InvalidPrecision)`. This does not latch. The whole list is validated before selecting.
5. Return the profile with the smallest `|position − clamped_wish|`. On an exact tie in distance, take the one with the larger position (more precise). If positions are also equal, take the first in the list.

**Clamp:**
- Values <0 become 0 and values >1 become 1. Values within [0,1] are unchanged.
- `-0.0` may be returned as is.

**Latch:**
- The latch is set only by step 3.
- While it is set, every `select` with a non-NaN wish fails with `NoActiveProfile`, even if `profiles` is non-empty again. Only `unlock` clears it.
- `unlock` returns `true` if it was set and `false` otherwise. After `unlock`, the next `select` evaluates normally. With still empty profiles it latches again.
- The latch of one selector does not affect other selectors (other subengines).
- Invalid wishes and invalid precision never latch.

**Satisfiable:** the wish counts as satisfiable as long as one active profile exists. No wish value makes it unsatisfiable.

**Relative scale:** only the positions of the passed profile list matter. There is no cross-subengine normalization.

**Concurrency:**
- Thread-safe. Concurrent `select`, `is_locked` and `unlock` calls on a shared `&ProfileSelector` are allowed.
- The latch must be poison-free. An atomic bool is recommended. If a `Mutex` is used, a poisoned lock is recovered (`into_inner`) and never panics or fails.
- When several threads concurrently see an empty list on an unlocked selector, all fail with `NoActiveProfile`, and the latch ends up set.
- No function panics.

## 5. Relied-on elements

From Pareto-Front und Profilableitung:
```rust
pub struct PrecisionProfile {
    pub position: f64,                   // in [0,1]; 0 = fastest, 1 = most precise
    pub config: BTreeMap<Axis, Value>,
}  // Clone, Debug, PartialEq, Send + Sync
```
- `derive_profiles` returns profiles ordered by ascending position, in [0,1]. A single profile has position 0.5. The list may be empty. This is not an error there, and it is the case that triggers the latch here.
- This module does not rely on the ordering of the list (it does a linear scan) and revalidates the positions.

The Capability-Modell is not used directly. `Axis` and `Value` appear only as the types inside `PrecisionProfile.config`.