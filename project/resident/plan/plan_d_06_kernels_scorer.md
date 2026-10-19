Setze D6 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor

# D6 — Kernels & Scorer

**Zweck:** `KernelRegistry` (NVRTC/CUBIN/Candle), Compile-Time- vs. ParamBlock-Parameter, `MaxSimOp` (Kachel-Kette → fused), Top-k-Reduktion/Merge, Work-Slice/Preemption, Query-Batching.

**Einordnung:** Abhängt von B5 (Kernel-Trait+Mock), D2 (Lane/ParamBlock/Events), D3 (Workspace/Arena). Downstream: G3 (Similarity/Cosine), D7 (Kernel schreibt/liest Segmente). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §4.5; `resident_index.md` (Kernel-Schnittstelle, via B5); `context/context_d.md` D6 (#22–#26).

---

## Festgelegte Entscheidungen

- **Standardpfad e5 = Cosine** (G3). MaxSim ist Infrastruktur/Wechselpfad (ColBERT). Beide hinter demselben Scorer-Op.
- **KernelRegistry (§4.5, `[V/U]`, #23):** `HashMap<(&'static str name, DType, u32 sm), CudaFunction>`. Quellen: **NVRTC** (`compile_ptx_with_opts(src, CompileOptions{ arch: Some("sm_75"), .. })` → `ctx.load_module(ptx)` → `get_func`), **CUBIN**-Fallback bei zu altem Treiber (PTX `compute_75`), **Candle-Ops** als weitere Implementierung. Candles eigene Kernel zur Build-Zeit mit `CUDA_COMPUTE_CAP=75`.
- **Konstanten vs. Parameter (§4.5):** Compile-Time (`-D`/Varianten als eigene Funktionen): dtype, `dim`, Tile-Größe. Dynamisch über **ParamBlock** (feste Region): Slot-Indizes, Zähler, Segmenttabelle, Query-Länge, Head/Tail.
- **MaxSim (`[U]`, #22):** Phase 1 **Candle-Kette** (`matmul → masked_fill → max → sum`); Sim-Matrix je Dokument klein (32×128 f16 = 8 KB), speichergebunden, cuBLAS/Tensor-Cores effizient. **Fused Kernel** lohnt erst, wenn Profil Sim-Matrix-Traffic/Launch-Overhead zeigt → **G-MAXSIM messen**, dann austauschen (gleicher Op).
- **Padding (`[V/U]`, #25):** Dokumentseite vor dem Max mit **−1e4** maskieren (nicht −inf → NaN-Risiko bei 0·−inf in f16); Query-`[MASK]`-Tokens zählen mit (Query-Augmentation). PLAID teilt dasselbe Scoring.
- **CPU-SIMD (`[U]`, #24):** Schleife über Query-Tokens, je Token laufendes Maximum über Doc-Tokens; `std::arch` (AVX2/FMA)/`wide`/`pulp` oder Autovektorisierung über `chunks_exact(8)`; `half` für f16. **Keine** Slot-übergreifende Matrix.
- **Top-k (§4.5):** pro Slice lokal reduzieren, **Merge auf der CPU**.
- **Work-Slice (`[U]`, #26):** Kernel in begrenzte Launches (~1–5 ms) teilen; Deadline zwischen Launches prüfen; Event je Slice; **Preemption nur an Slice-Grenzen**; hohe Prio gewinnt nur beim Block-Scheduling. Referenzen: Clockwork/PipeSwitch/REEF.
- **Query-Batching (§4.5):** ein Tile gegen alle wartenden Queries scoren (Lesekosten amortisieren).

---

## Zu erstellende Artefakte (`rag/src/kernel/`)

- `mod.rs` (baut auf B5)
  - `trait Kernel` (B5): Batch-Start/-Ende, Ergebnisse `KernelHit{offset, score}`, geschriebene Bereiche, Fortschrittszähler; `SegmentDesc{base, count, layout}`, `Layout{Fixed{stride}|Variable{offsets}}`.
  - `trait Scorer`: `score(query_batch, SliceRange, lane, param_block) -> Vec<KernelHit>`.
- `registry.rs`
  - `struct KernelRegistry` — `(name, dtype, sm) → Function`; NVRTC-Compile + CUBIN-Fallback; Candle-Impl als Eintrag. Lazy, über D1-Lease entladbar.
- `maxsim.rs`
  - `struct MaxSimOp` — `SliceRange{segment, slot_from, slot_to}` + Query-Batch → Scores je Slot.
  - Phase-1-Impl: Candle-Kachel-Kette (laufendes Max über Token-Kacheln, Summe über Query-Tokens); Zwischenmatrix durch Kachelgröße begrenzt.
  - Phase-2-Hook: fused NVRTC-Kernel hinter demselben Op.
- `cosine.rs`
  - e5-Standardpfad: Skalarprodukt auf L2-normierten Vektoren (Cosine = Dot), Scope-Subset, `top_k`.
- `cpu_simd.rs`
  - CPU-Scorer (SIMD), laufendes Max je Query-Token.
- `slice.rs`
  - Work-Slice-Scheduler: Zeitbudget, adaptive Größe aus Durchsatz, Preemption an Grenzen, Query-Überholen.
- `topk.rs`
  - Lokale Reduktion je Slice + CPU-Merge.

---

## Prototyp-Gate (zuerst)

- **G-MAXSIM:** Kachel-Kette vs. fused Kernel mit realen Workloads messen (Bandbreite, Latenz, Launch-Overhead). Entscheidet, wann Phase-2-Fused eingeführt wird.

---

## Arbeitsschritte

1. `Scorer`/`Kernel`-Traits (auf B5) + `KernelRegistry` (NVRTC+CUBIN+Candle).
2. Cosine-Scorer (e5-Standardpfad) auf L2-normierten Vektoren; Scope-Subset + `top_k`.
3. MaxSim Phase-1 (Candle-Kette) mit f16-sicherer Maskierung (−1e4), Query-`[MASK]` zählt mit.
4. CPU-SIMD-Scorer (laufendes Max je Query-Token).
5. Work-Slice-Scheduler (Zeitbudget, adaptiv, Preemption an Grenzen, Query-Überholen).
6. Query-Batching (ein Tile gegen wartende Queries); Top-k lokal + CPU-Merge.
7. G-MAXSIM messen; bei Bedarf fused NVRTC-Kernel hinter demselben Op ergänzen.

## Fehler (Q1) & Logging (Q2)

- `enum KernelError { Compile{log}, ModuleLoad, FuncNotFound{name}, LaunchFailed{source}, ParamBlockOverflow }`.
- Span pro Scoring-Op (`op`, `slice`, `n_queries`, `sm`); Compile-Events auf `info`, Launches auf `trace`. Fortschrittszähler als Metrik.

## Feature-Flags (Q4)

- NVRTC/CUBIN/CUDA-Launch hinter `cuda`; CPU-SIMD-Scorer + Candle-CPU-MaxSim immer verfügbar. B5-Kernel-Mock für GPU-freie Tests.

## Tests (Q3)

- CPU-Scorer gegen Referenz-Scores (Cosine/MaxSim) mit kleinen Fixtures.
- Padding/Maske: −1e4 verhindert NaN; Query-`[MASK]` zählt mit; Doc-Padding maskiert.
- Work-Slice: Preemption nur an Grenzen; Query überholt Indexing-Slice (deterministische Hooks).
- Top-k-Merge korrekt über mehrere Slices.
- Kernel-Mock (B5) für Launch-/Fortschrittslogik ohne GPU.
- CUDA/NVRTC-Compile nur unter `cuda` + GPU.

## Definition of Done

- Cosine-Standardpfad (e5) + MaxSim-Kachel-Kette lauffähig; CPU-SIMD-Pfad; Work-Slice/Top-k; KernelRegistry (NVRTC+CUBIN+Candle); G-MAXSIM gemessen und Entscheidung zu fused dokumentiert.

## Result

D6 is implemented under `rag/src/resident/kernel/`, not `rag/src/kernel/` as the plan says, because that is where the existing B5 kernel module lives.

I ran the full no-GPU suite (270 tests, all passing) and the kernel tests with `candle` and with `cuda`. The `cuda` run used the GTX 1660 SUPER and included the ignored GPU tests, which passed. I did not re-run the full suite with `cuda` after adding the G-MAXSIM test. `Cargo.toml` is unchanged.

- **Scorers:**
  - **Cosine (e5):** the standard dense path, a dot product on normalised vectors.
  - **CPU MaxSim:** SIMD-friendly, and an empty document scores −1e4 per query token.
  - **Candle MaxSim:** the tile chain with −1e4 masking on padded columns, behind the `candle` feature.
  - **Behind one op:** the Candle chain and the CPU path both sit behind `MaxSimOp`.
- **Top-k and slicing:**
  - **Top-k merge:** ties go to the lower offset and NaN scores are dropped.
  - **Slice scheduler:** it sizes slices from throughput and checks for preemption and the deadline only between slices, so at least one slice always completes.
- **`KernelRegistry`:** a lazy table keyed by name, dtype, sm and a variant (the embedding dimension), which is one more key field than the plan lists. It implements `Resource`, and loading is serialised per key.
- **NVRTC and CUBIN:** the CUDA loader compiles PTX for `compute_<sm>` and falls back to a CUBIN. The CUBIN path loads and runs on the GPU.
- **CUDA kernels:** cosine and a fused MaxSim.
- **Errors:** `KernelError` is added to `ResidentError`.
- **`Scorer` return type:** `Scorer::score` returns top-k hits per query rather than a single `Vec<KernelHit>`, so query batching keeps each query's results separate.

**G-MAXSIM:** on 4096 documents × 128 tokens, 32 query tokens, dim 128, f16, the Candle chain took 21.6 ms. My fused kernel took 67.1 ms. The fused kernel is correct but about 3× slower, since it doesn't use tensor cores. I kept the Candle chain and wrote the result and decision into `plan_d_06_kernels_scorer.md`.

**Not done:** kernel launches pass plain arguments rather than going through the lane's `ParamBlock`. Resolving arena regions to device pointers, and scoring from device-resident documents, are left for D7. The Candle chain currently scores from host data.

### Zusätzlich

## Implementation status

Code lives in `rag/src/resident/kernel/`: `registry.rs`, `cosine.rs`, `maxsim.rs`, `cpu_simd.rs`, `slice.rs`, `topk.rs`, `slots.rs`, `error.rs`, `cuda.rs` (NVRTC/CUBIN, `cuda` feature). `Scorer` returns slice-local top-k per query (`Vec<Vec<KernelHit>>`) so query batching keeps per-query results.

## G-MAXSIM result (GTX 1660 SUPER, sm75, f16)

Workload: 4096 documents x 128 tokens, 32 query tokens, dim 128, whole slice per pass.

| Implementation | Time per pass |
|---|---|
| Candle chain (`broadcast_matmul -> mask -> max -> sum`, cuBLAS) | 21.6 ms |
| Fused NVRTC kernel (scalar FMA, one block per document) | 67.1 ms |

Decision: keep the Candle chain as the MaxSim implementation. The fused kernel is correct (matches CPU reference) but 3x slower because it does not use tensor cores. Revisit only with a tensor-core (WMMA/mma) fused kernel that beats the chain; it plugs in behind `MaxSimOp::with_slots`.

## Open points

- Kernel launches take plain kernel arguments; mirroring dynamic values through the lane `ParamBlock` and resolving arena `Region`s to device pointers are part of the segment integration (D7).
- The Candle chain scores from host `SlotSource` data; device-resident documents arrive with D7.
