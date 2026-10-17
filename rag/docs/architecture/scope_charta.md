# Scope Charta: Resident Engine vs. Search Layer

> Package **A1′** (Group A — Foundation & Context). Replaces the completed package A1
> ("distill the thesis"). Binding reference for C1 (Facade API) and E2 (Scope Resolver);
> also informs G1/G3 (dense default path vs. ColBERT/MaxSim switch path).
>
> Sources: `resident_planplan.md` (Kernbefund, Abgleich-Tabelle, Zentrale Divergenz,
> Auswirkungen auf den Metaplan), `resident.md` (Klarstellung, Ausblick), `resident_index.md`
> (Leitprinzipien, Nicht Teil), `resident_precision.md` (Nicht Teil).

## 1. Purpose

This charta fixes the boundary between the "Resident" engine and the outer search/retrieval
("search layer"). It is **blocking** for C1 (facade API) and E2 (scope resolver): any later
design that pulls quality/chunking concerns into the engine, or that omits a required hook,
violates this charta and must be corrected before proceeding.

The engine is **substrate, not a retrieval algorithm**. It computes on whatever candidate set
it is handed; it guarantees neither completeness nor size of that set.

## 2. Boundary Table

Status legend: **K** = core/implemented · **I** = infrastructure/hook prepared ·
**A** = outlook/later, inside the engine · **E** = deliberately outside the engine ·
**—** = not addressed.

| Thesis (§) | Aspect | Status | Evidence | Responsibility |
|---|---|---|---|---|
| §2 | Disaggregation tokenizer/weights/kernel | **K** | `resident.md` Stack; `subengine` §3; `KernelRegistry` §4.5 | Engine |
| §3 | Residency (no per-request setup, lazy+TTL) | **K** | `resident.md` control flow; `subengine` §6 lifecycle | Engine |
| §3 | Ring-buffer producer/consumer compute | **A** | `resident.md` Ausblick; `resident_index.md` "Ringbuffer-Modus (geplant)"; `subengine` §4.6 | Engine (deferred, see §4 Divergence) |
| §3 | Multiple models resident | **K** (restricted) | fixed set before init, model-ID-keyed, lazy | Engine; dynamic add not planned |
| §3 | Batching optional / streaming | partly **K** | batching is real mechanic (token budget); streaming equivalence is outlook only | Engine |
| §4 | MaxSim = geometry, model-decoupled | **K** (principle) | `resident_precision.md` "not coupled"; `resident_index.md` "kernel only knows segments"; `MaxSimOp` | Engine |
| §4 | ColBERT multi-vector model | **A** | `resident_first.md` switch path; "no ColBERT module in candle-transformers" | Engine (switch path, not default) |
| §4 | Position instead of relation (index stores positions) | **K** | index stores entries/vectors; relation is outer layer | Engine stores positions; relation = search layer |
| §5 | Sliding window instead of chunking | **A** | `resident.md`/`planner` "gleitende Indizierung (Ausblick)" | Engine (data model only, prepared) |
| §5 | Index data model for multi-vector/window | **I** | `resident_index.md` variable-length entry; `subengine` slot classes | Engine (hook present) |
| §5 | Chunking itself | **E** | `resident_index.md`/`resident_precision.md` "Nicht Teil … Suchschicht" | Search layer |
| §6 | Frequency compression (JPEG/DCT) | **E / —** | mentioned only as outer-layer prefilter (FFT/compression domain as scope) | Search layer (H5) |
| §6.1 | Two axes / semantic spectrogram / wavelet | **—** | not addressed at all | Unaddressed (H5) |
| §7 | Cascaded refinement (fractal) | **I / A** | `resident_index.md` "Addons (Iteration/Kaskade/Rekursion)"; funnel "later"; scope resolver/centroid | Engine (addon pattern only, not core) (H3) |
| §9 | Parameter/activation duality, mutable weights | **—** | plan: weights immutable (weights arena long-lived); only index bidirectionally mutable | Deliberate divergence; not engine scope (H6) |
| §9 | Online adaptation (bandit, reward stream) | **—** | not addressed | Unaddressed (H6) |
| §9.1 | Bandit as meta-router over cascade | **—** | not addressed | Unaddressed (H6) |
| §10 | Drift monitor / re-embed trigger | **I / E** | stamps/change-notify support re-embed; monitor itself is outside | Hook via `changed()`/stamps; monitor = search layer |
| §10 | Hierarchical index (coarse/fine, frequency bands) | **I** | `resident_index.md` `children`/`subtree`/`summary` generic; frequency bands not | Engine (generic structure open) |
| §10 | Bridge to pylate (reference/validation) | partly **—** | `resident_first.md`: reference comparison with sentence-transformers (not pylate) | Different reference used |
| §2/§10 | Stack (safetensors/tokenizers/candle/cudarc/Parquet) | **K** | full stack + tokio/crossbeam/mmap | Engine |

## 3. Engine Delivers

- Resident compute: embedding/similarity inference, kept warm (lazy-loaded, TTL-evicted).
- Index substrate with streaming sync RAM ↔ Index ↔ VRAM (index is RAM-primary, VRAM is a
  derived, bidirectionally mutable mirror; the index object itself owns persistence/mmap
  outside the engine).
- Model-decoupled MaxSim geometry (kernel knows only segments, not the owning model/profile).
- Residency and eviction management for models, weights, and kernels.
- Scope resolution over declared access methods + resolver (`Scope`/`AccessMethod`), operating
  on a candidate set handed to it — without guaranteeing its completeness or size.
- Lock-free pull-task-queue concurrency model (Processor/Planner/Subengine), replacing the
  thesis's ring-buffer producer/consumer model for Phase 1 (see §4).
- Index change notification only ("what changed"); what the engine/manager does with that is
  manager/engine concern, not index concern.

## 4. Outside the Engine (Search Layer)

- Chunking.
- Query formulation.
- Candidate-k / rerank policy.
- Retrieval quality / completeness guarantees.
- Approximative prefilters (centroid, FFT, compression) — these lose edge matches by nature
  and are an outer-layer decision, not an engine concern.
- Drift monitor (consumes engine hooks, lives outside).
- Signal compression (FFT/wavelet, semantic spectrogram) — H5, not engine scope at all.
- Mutable weights / online bandit adaptation — H6, not engine scope at all.
- Index persistence format and persistence timing (owned by the index object itself).
- Iteration/cascade/recursion and parent-child task relationships as *core* engine behavior
  (only permitted as an addon pattern, never as scheduler-level task relationship/fan-out+join).

## 5. Central Divergence: Ring Buffer vs. Pull Queue (§3)

**Thesis §3** states its central answer to concurrency is "not classical multithreading but a
producer–consumer architecture over ring buffers."

**This plan's answer instead:** a **lock-free pull queue + Processor/Planner/Subengine**
(`resident.md` queue logic, `resident_processor.md`, `resident_planner.md`). The ring buffer is
retained as an **optional persistent-kernel mode for the scorer only** (`subengine` §4.6:
"useful for the scorer, not for the encoder").

The goal (residency, no per-request setup overhead) is identical to the thesis; the path for
Phase 1 is deliberately different. **This is a documented implementation decision, not an
inconsistency** — it must not be read as a 1:1 implementation of thesis §3.

## 6. Hook Obligations (Phase 1 must not foreclose)

Phase 1 implementation must keep the following extension points open, each tied to a later
outlook position (H1–H6):

- **H1 — GPU ring-buffer mode** (§3, status A): extensible parameter object, separate
  embedding/MaxSim rings, acquire-load-pointer in the manager discipline (E4). The pull-queue
  model must not structurally preclude a later persistent-kernel/ring-buffer scorer mode.
- **H2 — Sliding-window indexing** (§5, status A/I): index data model (variable-length entry,
  slot classes) is already the hook; kernel/pipeline follow later.
- **H3 — Funnel/PLAID + cross-encoder** (§7, status I/A): realizable via addons + centroid
  scope resolver; op exchange behind the same trait.
- **H4 — Graph-capture / persistent scorer** (§3/§9, status A): same hook family as H1.
- **H5 — Signal compression** (FFT/wavelet, semantic spectrogram; §6/§6.1, status E/—):
  **not engine scope.** Obligation: `AccessMethod`/`Scope`/`summary()` must not foreclose a
  later "frequency/compression domain" as a *search-layer prefilter*. No engine workstream.
- **H6 — Mutable weights / online bandit** (§9/§9.1, status —): **not engine scope.** Weights
  stay immutable (long-lived weights arena); only the index is bidirectionally mutable. If ever
  pursued, it becomes its own new workstream (reward queue, mutable weights arena); Phase 1 only
  commits to "not building this out structurally".

## 7. References

- C1 (Facade API) and E2 (Scope & Resolver) **must** plan against this charta.
- G1/G3: dense e5-small (cosine) is the standard path; MaxSim/ColBERT exists only as kernel
  infrastructure / switch path (H3/H4 adjacent), not the default.

## 8. Acceptance Check

1. Boundary table reproduces every row of the metaplan Abgleich-Tabelle. ✅
2. Every hook obligation (H1–H6) is tied to a concrete later extension. ✅
3. Ring-buffer ↔ pull-queue divergence is documented as a decision, not an inconsistency. ✅
4. C1 and E2 reference this charta as their planning basis (see §7). ✅
