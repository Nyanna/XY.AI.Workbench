# Addons: iteration, cascades, recursion

See `scope_charta.md` for the engine/search-layer boundary.

## Core

The core owns exactly two things a cascade needs:

- The task queue: any task, whoever submits it, goes through `Resident::submit_*`, the planner and the engine unchanged.
- Open parameter objects: `EmbeddingParams`, `SimilarityParams` and `InferenceParams` are `#[non_exhaustive]` and carry an `Extensions` map (one value per type, cheap to clone). Queue and planner move it untouched.

The core has no iteration, cascade, recursion, parent/child relation or intermediate task state. A task produced by an addon is indistinguishable from an application task.

## Addon

Everything iterative lives outside the core and only submits tasks:

- Continuation: when a task finishes, the addon derives follow-up tasks from its output and submits them to the same queue, with copied or changed parameters (for example a `Scope::Set` built from the hit cells).
- Correlation: an addon-defined type stored in `Extensions` (for example a correlation id) is copied to follow-ups. The core never interprets it.
- Placement of follow-ups is steered by the existing pull filters, not by the addon.

The reference cascade (summary hits -> member scope) is exercised in `resident/facade/test/addon.rs`.

## Search layer

Strategy belongs to the layer above the engine: funnel shapes (candidate generation, centroid pruning, exact re-ranking), candidate counts, quality targets. Addons may implement such shapes, but the engine does not know them.

## Open points

- `Extensions` is not serializable. If parameters must be persisted, add a name-keyed (de)serialize registry or a `serde_json::Value` fallback.
