# Umsetzungsplan Gruppe D — Subengine-Ressourcen (Überblick)

**Zweck dieses Dokuments:** Stufe-2-Umsetzungsplan für Gruppe D aus `resident_planplan.md`. Es bündelt die gemeinsamen Entscheidungen, die Reihenfolge, die Prototyp-Gates und die Querschnittskonventionen (Q) für die acht Pakete D1–D8. Jedes Paket hat eine eigene Plandatei (`plan_d1_*` … `plan_d8_*`), die isoliert durch einen Agenten abgearbeitet werden kann.

**Zielprojekt:** Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag` (Engine `resident`).
**Zielhardware (Referenz):** GTX 1660 (TU116, sm75, 22 SMs, 6 GB VRAM).

**Quellen:** `resident_planplan.md` (Paketschnitt), `context/context_d.md` (destillierte Recherche, Belegmarken `[V]`/`[U]`/`[prüfen]`), `resident_subengine.md` (primäre Designquelle, §1–§11), `resident.md` (Basiskonzept), `resident_first.md` (Modell e5-small), `context/context_h_q.md` (Querschnitt Q1–Q4).

> Hinweis: `resident_index.md` betrifft die Implementierung des **Vektorindexes** (nicht eine Dateiliste/Inhaltsverzeichnis). Er überlappt mit D7 und Gruppe E; für Gruppe D nur dort relevant, wo Segment-Repräsentation und Sync berührt werden.

---

## 1. Gegenstand von Gruppe D

Gruppe D beschreibt, **welche Ressourcen eine Subengine (CPU bzw. GPU/CUDA) besitzt**, wie sie initialisiert, geleast und entladen werden, wie sie rechnen und wie sie an Index-Segmente gebunden sind. Die Subengine ist das **Compute-Substrat**; Chunking, Kompression, Cascade-Policy und Retrieval-Qualität bleiben außerhalb (Suchschicht).

| Paket | Inhalt | Plandatei |
|------|--------|-----------|
| D1 | Gemeinsamer Lebenszyklus, Leases, Lazy-Init, Aggregation, Pull/Selbstorganisation | `plan_d1_subengine_lifecycle_pull.md` |
| D2 | Device & Ausführung: `DeviceHandle`, `ExecContext`/Lane, Streams, Events | `plan_d2_device_execcontext.md` |
| D3 | GPU-Speicher & Arenen, Budget, Pinned Staging, dtype | `plan_d3_gpu_memory_arenas.md` |
| D4 | Modell-Ressourcen: `ModelSpec`, `LoadedEncoder` (Candle-Adapter), Projektionskopf, L2, Batching | `plan_d4_model_encoder.md` |
| D5 | Tokenizer & Normalizer (CPU, geteilt), Modi, Bucketing | `plan_d5_tokenizer_normalizer.md` |
| D6 | Kernels & Scorer: `KernelRegistry`, `MaxSimOp`, Top-k, Work-Slice | `plan_d6_kernels_scorer.md` |
| D7 | Index-Segmente in der Subengine: `SegmentView`/Slot, Sync, Rückabbildung | `plan_d7_index_segments.md` |
| D8 | CPU-vs-GPU-Unterschiede & inkrementeller Ausbaupfad (§8/§9) | `plan_d8_cpu_gpu_buildout.md` |

---

## 2. Abhängigkeiten & empfohlene Reihenfolge

Abhängigkeiten aus dem Metaplan (Gruppe B/C sind Voraussetzung):

- **B1** Queue/Task-Primitive, **B2** Region/Layout/Memory, **B3** Op/Value, **B4** Index-Trait, **B5** Kernel-Abstraktion+Mock, **B6** CapabilitySet — müssen als Verträge stehen (mindestens als Trait-Stubs), bevor D baut.
- **C2/C3** Processor/Planner liefern die Op-Queues, aus denen die Subengine zieht (siehe Revisionshinweis §3).

Reihenfolge innerhalb D (topologisch):

```
D1 (Lifecycle/Leases/Pull)
  → D2 (Device/ExecContext)        [braucht D1]
    → D3 (GPU-Speicher/Arenen)     [braucht B2, D2; konsolidiert mit E3]
      → D4 (Encoder)               [braucht D2, D3]
D5 (Tokenizer/Normalizer)          [braucht D1; CPU-only, parallel möglich]
D6 (Kernels/Scorer)                [braucht B5, D2, D3]
D7 (Index-Segmente)                [braucht B4, D3; überlappt Gruppe E]
D8 (CPU/GPU-Diff & Ausbaupfad)     [braucht D1–D7, konsolidiert]
```

---

## 3. Gemeinsame, verbindliche Entscheidungen (gelten für alle D-Pakete)

1. **Revision Planner/Processor (aus `resident.md`):** Die direkte Verwendung von **Tasks** in Subengines ist **veraltet**. Der **Planner** expandiert Task→Op-Folge, der **Processor** hält die drei Op-Queues (CPU/GPU/shared) und den Worker-Parkplatz. Die Subengine liefert **Ressourcen + Ausführung** und signalisiert **freie Kapazität/Durchsatz**; die eigentliche Claim-/Reihenfolge-Logik liegt im Processor. Das in `resident_subengine.md` §7 beschriebene `claim(filter, budget)` wird in D1 als **Kapazitäts-/Durchsatz-Schnittstelle zum Processor** reinterpretiert (keine eigene Task-Queue in der Subengine).
2. **Invarianten (`resident_subengine.md` §2, ab Phase 1 verbindlich):** kein `candle_core::Tensor` und kein Raw-Pointer in öffentlichen Signaturen (nur `Region`+`Layout`); jede GPU-Arbeit über einen `ExecContext`, nie Default-Stream; jede Berechnung ist ein `Op` mit expliziten Regions; Kernels nur über `KernelRegistry`; Ressourcen nur über **Leases**. Candle ist Adapter an **genau einer** Stelle (Schicht 4, D4).
3. **dtype-Politik:** GPU `F16`, CPU `F32`. Index-dtype pro Index fest. Gemischte CPU/GPU-Indizierung nur mit definierter **Toleranz** (kein exakter Gleichheitstest), erwartete Cosinus-Abweichung f16↔f32 ≈ 1e-3 (`context_d` #13/#31).
4. **cudarc-Version:** Maßgeblich ist die von `candle-core` gezogene `cudarc`-Version. Im Projekt **dieselbe pinnen** oder besser den Re-Export `candle_core::cuda_backend::cudarc` verwenden. Vorab-Check: `cargo tree -i cudarc` und `cargo tree -d` (`context_d` #32). **Blockierend für D2/D3/D4/D6.**
5. **Modell jetzt:** dense **e5-small** (Cosine, Mean-Pooling+Maske+L2), `BertModel` statt `xlm_roberta` (`resident_first.md`, G1). **ColBERT/MaxSim** bleibt Infrastruktur + Wechselpfad, nicht Standardpfad.
6. **GPU-Tokenizer (D5-Widerspruch aufgelöst):** CPU-Tokenizer → `u32`-IDs → H2D → candle `index_select`. Kein ID→Adresse-Gather-Kernel. `resident_subengine.md` („nur Ausblick") gewinnt gegenüber `resident.md` (`context_d` #19).
7. **Index-Konsistenzmodell (D7, Empfehlung festgelegt):** **immutable Segmente + Generations-Manifest** (Lucene/Tantivy-Prinzip), Publikation per `arc-swap`, Leser pinnen die Generation per Lease, Freigabe erst wenn Leases==0 **und** alle Kernel-Stream-Events abgeschlossen. VRAM-Budget für 2 Generationen einplanen (CoW-Spitze). Diese Festlegung ist in D7 zu bestätigen (`context_d` #27).
8. **Speicher-Konsolidierung D3↔E3:** Es gibt **einen** `ArenaAllocator`/`MemoryManager`-Vertrag. D3 definiert ihn für die GPU-Subengine; E3 (Gruppe E) referenziert ihn ohne Dopplung. Keine zweite parallele Arena-Beschreibung.

---

## 4. Prototyp-Gates (vor Design-Festlegung, `resident_subengine.md` §10 + `context_d`)

Diese Prototypen sind **Gating** und in den jeweiligen Paketen als erster Schritt verankert:

- **G-ALIGN** (D2): cudarc-Version in candle == Projektversion (`cargo tree`). *(#32)*
- **G-STREAM** (D2): candle-`CudaDevice` mit eigenem Stream; Event auf candles Stream aufzeichnen/abwarten (`new_with_stream` prüfen). *(#5)*
- **G-PTR** (D2/D3): Device-Pointer eines Candle-Tensors (`CudaStorage`→`CudaSlice`→`device_ptr`) im eigenen Kernel über den Primary Context lesbar. *(#6)*
- **G-WRAP** (D3, **größtes Risiko**): Fremde Arena-Allokation als `CudaStorage` einbinden, **ohne dass Candle sie beim Drop freigibt** (`wrap_cuda_slice` + Drop-Verhalten, `upgrade_device_ptr`/`leak`). Fällt der Test negativ aus → Fallback „Arena nur für eigene Kernel, candle-Tensoren über candle-Allokation, D2D-Copy". *(#9/#33)*
- **G-GRAPH** (D8/H4): CUDA-Graph-Capture über Candle-Ops auf eigenem Stream (nur falls Phase-4 relevant). *(#16)*
- **G-CKPT** (D4): ColBERT-/Projektionskopf-Ort/Format, `safetensors` vorhanden oder Konvertierung. *(#15/#16/#17)* — für e5 entschärft.
- **G-MAXSIM** (D6): MaxSim-Kachel-Kette vs. fused Kernel messen (Bandbreite/Latenz, reale Workloads). *(#22)*

---

## 5. Querschnitt Q (für alle D-Pakete anzuwenden)

- **Q1 Fehler (`thiserror` 2.x [prüfen]):** Ein Enum **pro Modul** (klein, `#[non_exhaustive]`, strukturierte Felder, keine Strings), Top-Level-Enum aggregiert per `#[from]`. Interne Varianten (z. B. `StaleMapping`) in `pub(crate)`-Enums, an der API-Grenze per `From`/`map_err` abbilden. D-relevante Varianten u. a.: `TransferFailed`, `ArenaExhausted`, `Incompatible`, `IndexUnavailable`, `KernelLoad`, `DeviceInit`, `StaleMapping`(intern). Keine `anyhow::Error` in öffentlicher API.
- **Q2 Logging (`tracing`):** `#[tracing::instrument(skip_all, fields(...))]`; Span pro **Task-ID / Ressource / Manager-Zyklus**. Span-Propagation über `spawn`/`spawn_blocking`/Threads explizit (`Span::current()`+`in_scope`). In **CUDA-Host-Callbacks keine CUDA-API und kein direktes Loggen** — Daten per Channel/Atomic an einen Logger-Thread geben. Hochfrequente Pfade auf `trace`.
- **Q3 Tests/Mocks:** Logik gegen Traits testen; **No-GPU-Pfad** als Default. Kernel-Mock (B5) und Mess-Mock (manuelle Fakes, `context_d`-Q33). Nebenläufigkeit mit `loom`/`shuttle` (D1 Lease/Evict, D7 Sync). Deterministische Zeit via `tokio::time::pause` / `Clock`-Trait.
- **Q4 Feature-Flags:** `cuda` ist **additiv**, Default **ohne**. `cfg(feature="cuda")` nur an Modulgrenzen (Backend-Factory). Durchreichen: `cuda = ["candle-core/cuda", "dep:cudarc"]`. CI-Matrix mit/ohne `cuda`; No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests laufen.

---

## 6. Vorgeschlagener Modulschnitt (final in A2)

```
rag/src/
  error.rs                 # Top-Level-Fehler (Q1), aggregiert Modul-Enums
  subengine/               # D1: Subengine-Trait, Lifecycle, Lease, Aggregation
  device/                  # D2: DeviceHandle, ExecContext/Lane (cuda hinter Flag)
  memory/                  # B2+D3/E3: MemoryManager, Region, Layout, ArenaAllocator, Staging
  model/                   # D4: ModelSpec, Registry, LoadedEncoder, ProjectionHead, candle_adapter
  tokenize/                # D5: Tokenizer, Normalizer, Modi, Bucketing (CPU)
  kernel/                  # B5+D6: Kernel-Trait, KernelRegistry, MaxSimOp
  segment/                 # D7: SegmentView, Slot-Klassen, Mapping, Sync
```

Alle CUDA-konkreten Submodule hinter `#[cfg(feature = "cuda")]`; die Logik bleibt über Traits GPU-frei testbar.

---

## 7. Definition of Done (Gruppe D gesamt)

- D1–D7 jeweils als Modul mit Traits, CPU-Implementierung und (hinter `cuda`) CUDA-Implementierung; Mocks vorhanden.
- Alle Prototyp-Gates (§4) durchlaufen und dokumentiert; Fallbacks bei negativem Ergebnis festgehalten.
- Invarianten §2 durch API-Review bestätigt (kein `Tensor`/Raw-Pointer in öffentlichen Signaturen).
- No-GPU-Build grün (alle Unit-Tests), `cuda`-Build mindestens `cargo check` grün.
- D8 führt die §8-Unterschiede und den §9-Ausbaupfad in konkrete Stufen über.
