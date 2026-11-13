# Specification: Pareto-Front und Profilableitung (build order 26)

## 1. Task, restrictions and boundaries

Implement a pure Rust library module with two parts:
1. **Pareto front and positions.** Compute the front over time and deviation, both minimized. Compute positions on a normalized scale.
2. **Profile derivation.** Derive unnamed precision profiles from calibration measurements using the merge threshold.

Restrictions:
- No I/O except log output through the `log` crate, no global mutable state, no hardware or model knowledge, and no profile names.
- Allowed dependencies: the Capability-Modell (`Axis`, `Value`) and `log`.
- The only tuning parameter is the merge threshold on the normalized scale (default 0.1). It does not depend on model or hardware.
- The number of profiles comes only from the spread of the front. There is no target count.

## 2. Context

The subengine runs calibration measurements. A measurement is one configuration (one value per axis) with measured time and deviation. This module turns them into the precision profiles that the subengine provides. Profiles carry no name. They carry a position in [0,1] and one value per axis. The facade and the task interface consume the profiles. They interpret the positions and the configurations, and this module does not.

## 3. Interface

```rust
use std::collections::BTreeMap;
use capability_model::{Axis, Value};   // Name aliases from Capability-Modell

pub const DEFAULT_MERGE_THRESHOLD: f64 = 0.1;

/// One calibration measurement.
#[derive(Clone, Debug, PartialEq)]
pub struct Measurement {
    pub config: BTreeMap<Axis, Value>,   // exactly one value per axis
    pub time: f64,                       // lower is better
    pub deviation: f64,                  // lower is better
}

/// Unnamed precision profile.
#[derive(Clone, Debug, PartialEq)]
pub struct PrecisionProfile {
    pub position: f64,                   // in [0,1]; 0 = fastest, 1 = most precise
    pub config: BTreeMap<Axis, Value>,   // value per axis
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProfileError { InvalidThreshold(f64) }   // non-finite or < 0
impl std::fmt::Display for ProfileError {}
impl std::error::Error for ProfileError {}

/// Front over (time, deviation) of number pairs (time, deviation).
pub fn pareto_front_pairs(points: &[(f64, f64)]) -> Vec<(f64, f64)>;

/// Generic front. `key` returns (time, deviation). Returns clones of the kept elements.
pub fn pareto_front<T: Clone>(items: &[T], key: impl Fn(&T) -> (f64, f64)) -> Vec<T>;

/// Positions in [0,1], one per input point, in input order.
/// `points` are (time, deviation), expected to be a front (ascending time).
pub fn positions(points: &[(f64, f64)], log_time: bool) -> Vec<f64>;

/// Derive profiles with the given merge threshold. Ordered by ascending position.
pub fn derive_profiles(
    measurements: &[Measurement],
    merge_threshold: f64,
) -> Result<Vec<PrecisionProfile>, ProfileError>;
```

All functions are free functions without state and are `Send + Sync`-safe. `Measurement` and `PrecisionProfile` are `Send + Sync`.

## 4. Behavior and invariants

**Front**
- Points with a non-finite time or deviation (NaN, ±inf) are dropped first.
- The result is sorted by ascending time.
- A point is kept only if its deviation is strictly smaller than the deviation of every kept point with smaller or equal time. Equal-time points are ordered by deviation, then by input order. Of those, only the first can survive. Exact duplicates keep the first.
- Hence in the result, time is non-decreasing and deviation is strictly decreasing.
- Empty input or all points non-finite gives an empty result.
- `pareto_front` gives the same result as `pareto_front_pairs` on the keys. It drops elements whose key is non-finite and preserves the element values.

**Positions**
- Axes are min-max normalized to [0,1] over the given points. With `log_time`, the time axis is `ln(time)` before normalization, and times ≤ 0 are clamped to `f64::MIN_POSITIVE`. An axis with range 0 normalizes to 0 for all points.
- The position of a point is the cumulative Euclidean arc length along the normalized polyline in input order, divided by the total length. The first position is 0 and the last is 1.
- Empty input gives an empty vec. One point gives `[0.5]`. If the total length is 0 and n ≥ 2, the distribution is uniform: `i/(n-1)`.
- Output length equals input length. Positions are non-decreasing.

**Threshold**
- `merge_threshold` that is NaN, infinite or negative gives `Err(InvalidThreshold)`. 0 is valid and merges nothing.

**Derivation** (`derive_profiles`)
1. Drop measurements that are non-finite or have `time <= 0`. Compute the front (generic variant over the measurements). If it is empty, return `Ok(vec![])`.
2. **Filter.** On the front, take `ln(time)` min-max normalized to `t̂` (range 0 gives all 0). Remove a front point i if some slower and more precise front point j > i exists with `t̂_j − t̂_i < threshold`. That means it is less precise without being significantly faster. This covers identical emulation paths and slow emulated precision. The last front point is never removed. Log every removal at `log::info!` with config, time and deviation. If every point is removed, return an empty result, though the rule makes this impossible.
3. **Group.** Compute `positions(remaining, log_time = true)`. Walk the remaining points in order. A neighbor with a position gap `< threshold` to the previous point joins its group (chained). Otherwise it starts a new group.
4. **Representative.** For each group, choose the member with the lowest deviation, the most precise front point. If that is not determinable, choose the fastest.
5. **Re-position.** Compute `positions(representatives, log_time = true)`. Each profile is `{position, config}`, with the representative's config copied unchanged.
- A single remaining representative gets position 0.5.
- The result is ordered by ascending position. Positions are in [0,1]. It is deterministic and does not depend on the input order, except for ties of exactly equal (time, deviation), where the first input wins.
- The profile count follows only from the front and the threshold. No other input affects it.
- A threshold of 0.1 on the normalized scale gives at most about 10 profiles.
- If no usable profile exists (no finite measurements), the result is `Ok(vec![])`. This is not an error.

**Concurrency / errors**
- All functions are pure and reentrant, and no function panics. The only error is `ProfileError::InvalidThreshold`.

## 5. Relied-on elements

From Capability-Modell:
- `Name`, with `Axis = Name` and `Value = Name`. They are `Clone + Ord + Eq + Hash + Debug`, `Send + Sync`, with cheap `Arc<str>` clone. Ordering is byte-wise lexicographic by string content, so a `BTreeMap<Axis, Value>` iterates deterministically.
- This module defines no axis names or value names. It copies configurations through unchanged.