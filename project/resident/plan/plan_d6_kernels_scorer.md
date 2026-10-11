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
