Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe D aus `resident_planplan.md`.

* `resident_index.md` ist nur relevant für die Implementierung eines Vektorindexes und keine Indexdatei.

Projektpläne:
```json
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
Kontext Dateien für Gruppen:
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident/context",
      "entries" : [ ".:", "context_h_q.md", "context_a.md", "context_b.md", "context_c.md", "context_d.md", "context_e.md", "context_f.md", "context_g.md" ]
    } ]
  }
}
```
SystemInit:  id=c468822d-f77a-4c85-a5de-9b647eaf6612, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by reading the key files to understand the task structure. Let me read the planning document and the Group D context file first.
Control Request:
```yaml
id: 0f9c-13
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_d.md
  reason: Read the main planning document for groups and the Group D context to understand what implementation plans are needed.
```
/answer 0f9c-13 allow
ReasoningToken: 266
Text:
 I have a clear picture of Group D (Subengine-Ressourcen, packages D1–D8). Now let me read the primary source documents referenced for Group D to ground the implementation plans accurately.
Control Request:
```yaml
id: 0f9c-14
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  reason: Read the primary source for Group D (resident_subengine.md) plus the base concept (resident.md) and model notes (resident_first.md) to ground the per-package implementation plans.
```
/answer 0f9c-14 allow
ReasoningToken: 568
Text:
 Let me read the cross-cutting context (Q: errors/logging/tests/feature-flags) so the Group D plans reference consistent conventions.
Control Request:
```yaml
id: 0f9c-15
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_h_q.md
  reason: "Read the cross-cutting H/Q context so the Group D implementation plans reference consistent error taxonomy, logging, test/mock and feature-flag conventions."
```
/answer 0f9c-15 allow
ReasoningToken: 3408
Text:
 I now have everything I need. I'll create the Group D implementation plans (one overview + one actionable plan per package D1–D8) in the `plan` directory, grounded in `resident_planplan.md` (package definitions), `context_d.md` (resolved research), the primary source `resident_subengine.md`, and the cross-cutting Q conventions.
Control Request:
```yaml
id: 0f9c-16
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_overview.md
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d1_subengine_lifecycle_pull.md
    mode: replace
    content: |
      # D1 — Subengine-Gemeinsames: Lebenszyklus, Leases, Lazy-Init, Pull

      **Zweck:** Gemeinsamer Vertrag beider Subengines (CPU, GPU): Invarianten, Ressourcen-Lebenszyklus, Leases, Lazy-Init mit Semaphore, Aggregation und die Kapazitäts-/Pull-Schnittstelle zum Processor.

      **Einordnung:** Abhängt von B1 (Task/Queue-Primitive), B2 (Region/Memory), B3 (Op/Value). Downstream: D2–D7 bauen darauf. Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §2, §6, §7; `resident.md` (zwei Subengines, Kontrollfluss, Queue-Logik); `context/context_d.md` D1 (#1–#4).

      ---

      ## Festgelegte Entscheidungen

      - **Revisionshinweis (verbindlich):** Direkte Task-Nutzung in der Subengine ist veraltet. Der **Processor** (C2) hält die Op-Queues und ruft die Subengine-Ressourcen über Worker; der **Planner** (C3) erzeugt die Ops. D1 definiert daher **keine eigene Task-Queue**, sondern:
        - einen **Subengine-/Resource-Lifecycle** (Lazy-Init, Lease, Evict),
        - eine **Kapazitäts-/Durchsatz-Schnittstelle** (`claim(filter, budget)` reinterpretiert als „gib mir bis zu `budget` passende Arbeit für diese Ressource"), die vom Processor bedient wird.
      - **Lazy-Init (`[V]`, #1):** `tokio::sync::OnceCell::get_or_try_init(async …)`. Bei `Err` bleibt die Zelle uninitialisiert (erneut versuchbar). Globaler `Arc<Semaphore>` begrenzt **parallele Inits über verschiedene Zellen**; das Permit wird **innerhalb** der Init-Closure gehalten. Backoff per `Instant` neben der Zelle.
      - **Leases (`[V]`, #4):** RAII-Guard, `Drop` dekrementiert `AtomicUsize`. Neue Leases während `Evicting` gesperrt.
      - **Eviction (#4):** Bedingung `in_flight==0 && leases==0 && last_used+TTL < now`, unter Lock **Double-Check**. Referenzmuster `moka` (TTI/TTL). VRAM-TTL deutlich länger als RAM-TTL.
      - **Pull/Selbstorganisation (`[U]`, #2):** kein Standard-Crate. Batchgröße ∝ EWMA-Durchsatz (`alpha≈0.2`) mit Min/Max-Klammern; EWMA glätten, Obergrenze setzen (verhindert Last-Oszillation). Verwandt: Guided Self-Scheduling, Work-Stealing.
      - **Claim-Filter (`[U]`, #3):** getrennte Queues je Priorität/Modell (im Processor); Verfügbarkeitsprädikate (z. B. „Modell nicht gesperrt") als Atomic-Flag vor dem Pop; Skip-Scan unter `parking_lot::Mutex` ist bei < 10⁴ Tasks ausreichend.

      ---

      ## Zu erstellende Artefakte (`rag/src/subengine/`)

      - `mod.rs`
        - `trait Subengine` (objektsicher): `device() -> &DeviceHandle` (D2), `capacity() -> Capacity`, `throughput() -> f32` (EWMA), `resource(ResourceKey) -> &ResourceCell`, `run_ops(batch, lease)`.
        - `enum SubengineKind { Cpu, Cuda }`.
      - `lifecycle.rs`
        - `enum ResourceState { Unloaded, Loading, Resident, Leased(usize), Idle(Instant), Evicting, Failed }` als Zustandsautomat (intern per `AtomicU8` + Zusatzfelder).
        - `struct ResourceCell<R>` mit `OnceCell<R>`, `AtomicUsize leases`, `AtomicUsize in_flight`, `last_used: Mutex<Instant>`, `backoff_until: Mutex<Option<Instant>>`.
        - `fn get_or_init(&self, init: impl Future) -> Result<Lease<R>, EngineError>` (Semaphore-Permit in der Closure).
        - `struct Lease<R>` (RAII, Drop → `leases -= 1`, `last_used = now`).
        - `struct InitLimiter(Arc<Semaphore>)` global.
      - `resource.rs`
        - `trait Resource`: `key() -> ResourceKey`, `init_cost_hint()`, `evictable(now) -> bool`.
        - `fn evict_idle(&self, now)` mit Double-Check unter Lock.
        - `struct Ewma { value: f32, alpha: f32 }` + `fn observe(&mut self, sample)`.
        - Aggregations-Helfer: wartende Arbeit auf gleichem Modell/Index zusammenziehen (Hook für Processor).

      ---

      ## Arbeitsschritte

      1. **Zustandsautomat** `ResourceState` samt zulässiger Übergänge dokumentieren (`Unloaded→Loading→Resident→Leased(n)→Idle→Evicting→Unloaded`, plus `*→Failed`). Übergänge per CAS auf `AtomicU8`.
      2. **Lazy-Init** über `OnceCell::get_or_try_init` + globalen `Semaphore`; Fehlerpfad lässt Zelle leer, setzt `backoff_until`.
      3. **Lease** als RAII-Guard; `leases`/`in_flight` als Atomics; neue Leases bei `Evicting` ablehnen (`IndexUnavailable`/`ResourceEvicting`).
      4. **Eviction** mit Double-Check implementieren; TTL pro Ressourcenklasse konfigurierbar (VRAM≫RAM).
      5. **EWMA-Durchsatz + Kapazität**: `throughput()`/`capacity()` liefern dem Processor die Batchgröße (Min/Max-geklammert).
      6. **`claim`-Reinterpretation**: Schnittstelle definieren, über die der Processor passende Ops (Filter: Tasktyp, Index-Affinität, Prioritätsklasse) bis `budget` an diese Ressource übergibt; Starvation-Schutz/Query-Überholen bleiben im Processor.
      7. **Aggregation**: beim Laden einer Ressource wartende Arbeit desselben Modells/Index zusammenziehen (Hook, keine eigene Queue).
      8. **Fehler/Backoff**: Init-/Launch-Fehler → `Failed`/Backoff, strukturierter Fehler, Span-Logging.

      ---

      ## Fehler (Q1) & Logging (Q2)

      - `error.rs` im Modul: `enum SubengineError { ResourceInit{source}, ResourceEvicting, Backoff{until}, CapacityExhausted, Incompatible{..} }` (`#[non_exhaustive]`).
      - Span pro Ressource (`resource = %key`) und pro Zyklus; Events: `loading`, `resident`, `lease_acquired/released`, `evicting`, `failed`. Hochfrequentes `claim`-Scanning auf `trace`.

      ## Feature-Flags (Q4)

      - D1 ist **GPU-frei** (reine Lifecycle-/Lease-Logik) und baut ohne `cuda`. Konkrete GPU-Ressourcen werden erst in D2/D3 eingehängt.

      ## Tests (Q3)

      - Zustandsautomat: alle Übergänge inkl. verbotener; `Failed`-Retry.
      - Lease/Evict-Nebenläufigkeit mit `loom`/`shuttle` (Double-Check-Race, Lease während Evicting).
      - Lazy-Init: nur **ein** Initialisierer bei parallelen Aufrufern; Semaphore begrenzt parallele Inits; Fehler lässt Zelle leer.
      - EWMA: Konvergenz, Klammerung, keine Oszillation bei verrauschten Samples (seeded RNG).
      - Deterministische Zeit via `tokio::time::pause`/`Clock`-Trait für TTL.

      ## Definition of Done

      - `Subengine`-Trait + `ResourceCell`/`Lease`/`evict_idle` implementiert und getestet (No-GPU).
      - Lifecycle-Invarianten geprüft; Revisionshinweis in Code-Doku verankert; `claim`-Schnittstelle mit Processor (C2) abgestimmt.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d2_device_execcontext.md
    mode: replace
    content: |
      # D2 — Device & Ausführung (`DeviceHandle`, `ExecContext`/Lane)

      **Zweck:** Ausführungskontext je Subengine: `DeviceHandle`, `ExecContext`/Lane (Stream, Priorität, Events, ParamBlock, Scratch), Lane-Klassen (Query/Index/Copy), CUDA-Init-Kosten.

      **Einordnung:** Abhängt von D1. Downstream: D3 (Arenen/Staging an Lanes), D4 (Encoder läuft auf Lane), D6 (Kernel-Launch auf Lane), D7 (Copy-Lane für Segment-Sync). Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §4.1, §2 (ExecContext-Invariante); `context/context_d.md` D2 (#5–#8); `context_h_q.md` (#7 cudarc/candle).

      ---

      ## Festgelegte Entscheidungen

      - **Invariante:** Jede GPU-Arbeit läuft über einen `ExecContext`, **nie** auf dem Default-Stream (§2).
      - **Eigener Stream (`[U]`, #5):** candle-core ≥0.9 nutzt cudarc ≥0.16 (`CudaContext`/`CudaStream`). `CudaDevice::new_with_stream(ordinal)` **prüfen** (G-STREAM). Events über cudarc: `stream.record_event(flags) → CudaEvent`, `event.synchronize()`, `other_stream.wait(&event)`. Candle-Ops laufen auf dem **Device-Stream** — nicht annehmen, dass sie auf dem Default-Stream bleiben.
      - **Primary Context / Pointer (`[V/U]`, #6):** cudarc retained den Primary Context; candle teilt ihn **bei identischer cudarc-Version** (siehe G-ALIGN). Pointer: `CudaStorage → CudaSlice<T> → DevicePtr::device_ptr(&stream)`. Stream-Sync **manuell per Event**.
      - **Init-Kosten (`[U]`, #7):** erste Context-Erzeugung 0,2–2 s (kalt mehr). Context beim Start **warmlaufen** (Dummy-Alloc), `CUDA_MODULE_LOADING=LAZY` setzen, Modell-Init separat messen. Lazy-Init (D1) muss diese Kosten einplanen.
      - **Lane/Prioritäten (`[U]`, #8):** cudarc-safe hat **keine** Prioritäts-API → `sys::cuStreamCreateWithPriority` + `cuCtxGetStreamPriorityRange`. Turing unterstützt Stream-Prioritäten (z. B. 0…−5), wirken nur aufs **Block-Scheduling**, keine Preemption laufender Blöcke.
      - **ExecContext =** `{ stream, events, pinned staging (D3), ParamBlock (Device-Buffer), scratch-Arena }`.
      - **CPU-Lane:** Lane = Worker-Gruppe im eigenen Pool, **gleiche Schnittstelle**, ohne Streams/Staging.

      ---

      ## Zu erstellende Artefakte (`rag/src/device/`)

      - `mod.rs`
        - `struct DeviceHandle` — Ordinal bzw. CPU; hält (hinter `cuda`) `CudaContext`/Device.
        - `trait ExecContext` (objektsicher, GPU-frei): `submit(op)`, `record_event() -> EventHandle`, `wait(&EventHandle)`, `param_block() -> &mut ParamBlock`, `scratch() -> &ScratchArena`.
        - `enum LaneClass { Query, Index, Copy }` + Prioritäts-Mapping.
        - `struct EventHandle` (opak; kein Raw-Handle nach außen).
        - `struct ParamBlock` (kleine feste Region für dynamische Kernel-Parameter; aus D3-Arena).
      - `cpu.rs`: `CpuLane` (Worker-Gruppe) implementiert `ExecContext` ohne Streams; Events = no-op/Join-Barrieren.
      - `cuda.rs` (hinter `#[cfg(feature = "cuda")]`):
        - `CudaLane { stream, priority, events }` über cudarc; Prioritäts-Stream via `sys::cuStreamCreateWithPriority`.
        - `fn warmup(&self)` (Dummy-Alloc, Modul-Load).
        - Event-Record/Wait-Mapping; Device-Pointer-Zugriff nur **intern**.

      ---

      ## Prototyp-Gates (zuerst)

      1. **G-ALIGN:** `cargo tree -i cudarc` / `-d` → identische cudarc-Version candle↔Projekt; sonst `candle_core::cuda_backend::cudarc` re-exportieren.
      2. **G-STREAM:** `CudaDevice`/`new_with_stream` + Event record/wait auf candles Stream verifizieren.
      3. **G-PTR:** Device-Pointer eines Candle-Tensors in eigenem Trivial-Kernel lesen (Primary Context geteilt).

      Ergebnis jedes Gates dokumentieren; bei Fehlschlag Fallback (eigener Context/manuelle Pointer-Pflege) festhalten.

      ---

      ## Arbeitsschritte

      1. Gates G-ALIGN/G-STREAM/G-PTR durchführen und dokumentieren.
      2. `DeviceHandle` für CPU und CUDA; Context-Warmup + `CUDA_MODULE_LOADING=LAZY`.
      3. `ExecContext`-Trait definieren (GPU-frei), `CpuLane` + `CudaLane` implementieren.
      4. Lane-Klassen Query/Index/Copy mit Prioritäten anlegen; Übergaben **nur** per Event.
      5. `ParamBlock`/`ScratchArena` an Lane binden (Regions aus D3).
      6. Init-Kosten messen (Context, Modell separat) und als Metrik an D1-Lazy-Init melden.

      ## Fehler (Q1) & Logging (Q2)

      - `enum DeviceError { Init{source}, StreamCreate, EventRecord, PriorityUnsupported, VersionMismatch }`.
      - Span pro Lane (`lane = ?class`, `stream_prio = n`); Events `warmup`, `event_record/wait`. **Keine CUDA-API in Host-Callbacks** (#32).

      ## Feature-Flags (Q4)

      - `cuda.rs` komplett hinter `cuda`. Backend-Factory wählt `CpuLane`/`CudaLane`. No-GPU-Build nutzt nur `CpuLane`.

      ## Tests (Q3)

      - `CpuLane` voll gegen `ExecContext`-Trait (Reihenfolge, Event-Barrieren als Join).
      - CUDA-Tests nur unter `cuda`-Feature + vorhandener GPU (ignorierbar in CI ohne GPU).
      - Mock-Lane für Downstream (D4/D6) ohne echten Stream.

      ## Definition of Done

      - Gates dokumentiert; `ExecContext`-Trait + CPU/CUDA-Lanes; Prioritäts-Streams gesetzt; Warmup vorhanden; keine Raw-Pointer/`Tensor` in öffentlichen Signaturen.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d3_gpu_memory_arenas.md
    mode: replace
    content: |
      # D3 — GPU-Speicher & Arenen

      **Zweck:** Arenen nach Lebensdauer (Weights/Index-Segment/Workspace/Staging/ParamBlock), VRAM-Budget via `cuMemGetInfo`, stream-geordnete Wiederverwendung, Pinned Staging, dtype pro Device. **Konsolidiert mit E3** (eine Arena-Spezifikation).

      **Einordnung:** Abhängt von B2 (Region/Layout/MemoryManager), D2 (Lane/Stream/Events). Downstream: D4 (Weights/Workspace), D6 (Workspace/ParamBlock), D7 (Index-Segment-Arena/Staging). Überlappt E3 (Gruppe E) — **dort nur referenzieren, nicht neu beschreiben**.

      **Quellen:** `resident_subengine.md` §4.2; `resident_index.md` (Arena/Manager-Disziplin, via E3); `context/context_d.md` D3 (#9–#13, #10 Pools, #32/#33).

      ---

      ## Festgelegte Entscheidungen

      - **Region-Modell (B2):** `Region{arena, offset, len, gen}`, kein Pointer nach außen; `gen` entwertet Handles nach Evict/Reload. `Layout{shape, strides, dtype}` mit eigenem DType-Enum.
      - **Arenen nach Lebensdauer (§4.2):** Weights (langlebig), Index-Segment (gestreamt/evictable), Workspace/Scratch pro Lane (kurzlebig), Staging (Pinned Host), ParamBlock pro Lane (klein, fest).
      - **Budget (`[V/U]`, #12):** `cudarc::driver::result::mem_get_info() → (free,total)`; **nicht** fix 6 GB. Realistisch nutzbar ~5–5,5 GB (Context 250–400 MB, Compositor 100–500 MB). Vor Ort messen. **2 Generationen** einplanen (CoW-Spitze, E3/D7).
      - **Kein malloc/free im Betrieb (`[V/U]`, #10):** cudarc ≥0.12 nutzt `cuMemAllocAsync`/`cuMemFreeAsync` bei Pool-Support (Turing: ja); `cuMemPoolSetAttribute(RELEASE_THRESHOLD = u64::MAX)` behält Speicher. Zusätzlich **Bump-Arena** (ein großer Alloc, Offsets) für eigene Scratch-Buffer.
      - **Pinned Staging (`[U]`, #11):** `ctx.alloc_pinned::<T>(n) → PinnedHostSlice`, `stream.memcpy_htod/dtoh` async. **Ring von Regionen**, jede mit Event; Region erst nach `event.synchronize()`/Completion wiederverwenden.
      - **Stream-geordnete Wiederverwendung (§4.2):** Region wird erst frei/überschrieben, wenn alle nutzenden Streams ihr Event erreicht haben — Logik im **MemoryManager**, nicht in Ops.
      - **dtype (`[V/U]`, #13):** GPU `F16`, CPU `F32`. BF16 auf Turing meiden (nicht nativ). Attention-Maske f16-sicher: **−1e4 statt −inf**. Erwartete Cosinus-Abweichung f16↔f32 ≈ 1e-3.
      - **Fremde Allokation in candle (`[U]`, kritisch, #9/#33):** `CudaSlice` ist **owning** → Drop gibt frei. Empfehlung (Default): **Arena-Speicher nur für eigene Kernel**; candle-Tensoren über candles eigene (stream-ordered Pool-)Allokation, Daten per **D2D-Copy** austauschen. `wrap_cuda_slice`/`upgrade_device_ptr`+`leak` nur nach erfolgreichem **G-WRAP**-Prototyp.

      ---

      ## Prototyp-Gate (zuerst, blockierend)

      - **G-WRAP:** Arena-Sub-Range als `CudaStorage` in candle einbinden **ohne Doppel-Free beim Tensor-Drop**. Prüfen: `wrap_cuda_slice(CudaSlice, CudaDevice)` vs. `stream.upgrade_device_ptr` + `CudaSlice::leak()`; beachten, dass Sub-Range nur `CudaView` (borrowed) ergibt, das candle nicht als Storage nimmt.
        - **Positiv:** Zero-Copy `encode_into` direkt in Arena möglich (Phase 3+).
        - **Negativ (Fallback):** candle rechnet in eigenem Pool, `WriteSlotOp` kopiert D2D in die Arena. **Design darauf auslegen, dass der Fallback trägt.**

      ---

      ## Zu erstellende Artefakte (`rag/src/memory/`)

      - `mod.rs`
        - `trait MemoryManager`: `alloc(dev, bytes, align) -> Region`, `free(Region)`, `resolve(Region) -> *internal*`, `budget() -> Budget`.
        - `enum ArenaKind { Weights, IndexSegment, Workspace, Staging, ParamBlock }`.
        - `struct Budget { free, total, reserved_for_generations }`.
      - `arena.rs`
        - `struct ArenaAllocator` — Bump/Free-List, Offsets `u64`, `gen`-Entwertung; pro `ArenaKind` eine Instanz. **Gemeinsamer Vertrag für D3 und E3.**
        - Stream-geordnete Freigabe: `release_after(region, event)`; Rückgabe erst nach Event.
      - `staging.rs` (hinter `cuda`)
        - `struct StagingRing { regions: Vec<PinnedRegion>, events: Vec<EventHandle> }`, `acquire()`/`publish(event)`.
      - `cuda.rs` (hinter `cuda`)
        - `CudaMemoryManager` über cudarc: Pool-Config (`RELEASE_THRESHOLD`), `mem_get_info`, `alloc_pinned`, D2D-Copy-Helfer.
      - `cpu.rs`
        - `CpuMemoryManager` — einfache RAM-Arena, kein Staging/Stream (CPU-Pfad).

      ---

      ## Arbeitsschritte

      1. **G-WRAP** durchführen, Ergebnis + gewählten Pfad (Zero-Copy vs. D2D-Fallback) dokumentieren.
      2. `MemoryManager`-Trait + `Region`/`Budget`; `gen`-Entwertung implementieren.
      3. `ArenaAllocator` pro `ArenaKind`; stream-geordnete Freigabe (`release_after`).
      4. CUDA-Backend: Pool-Konfiguration, `mem_get_info`, Budgetreservierung für 2 Generationen.
      5. `StagingRing` (Pinned) mit Event-pro-Region.
      6. dtype-Politik verankern (f16/f32), f16-sichere Maskenkonstante (−1e4) als Modulkonstante.
      7. Abgleich mit **E3**: sicherstellen, dass E3 diesen `ArenaAllocator` referenziert (keine Dopplung).

      ## Fehler (Q1) & Logging (Q2)

      - `enum MemoryError { ArenaExhausted{arena, requested, free}, TransferFailed{dir, source}, StaleRegion{gen}, BudgetProbe }`.
      - Span pro Arena/Allokation (`arena = ?kind`, `offset`, `len`, `gen`); Budget-Probes auf `debug`, Alloc/Free auf `trace`.

      ## Feature-Flags (Q4)

      - `staging.rs`/`cuda.rs` hinter `cuda`; `CpuMemoryManager` immer verfügbar. Factory wählt Backend.

      ## Tests (Q3)

      - CPU-Arena: Alloc/Free/Offsets/`gen`-Entwertung, `ArenaExhausted`.
      - Stream-geordnete Freigabe gegen Mock-Events (keine Freigabe vor Event).
      - Staging-Ring: keine Wiederverwendung vor Event-Completion (Mock).
      - Budget-Mock: Reservierung 2 Generationen respektiert.
      - CUDA-D2D/Pinned nur unter `cuda` + GPU.

      ## Definition of Done

      - G-WRAP dokumentiert; `MemoryManager`/`ArenaAllocator`/`StagingRing` implementiert; Budget inkl. 2-Generationen-Reserve; dtype-Politik verankert; mit E3 konsolidiert (eine Spezifikation).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d4_model_encoder.md
    mode: replace
    content: |
      # D4 — Modell-Ressourcen (Encoder)

      **Zweck:** `ModelSpec`+Registry, `LoadedEncoder` (Candle-Adapter als **einzige** Candle-Stelle), `ProjectionHead`, L2-Kernel, `encode_into`/`WriteSlotOp`, Batching per Token-Budget/Bucketing.

      **Einordnung:** Abhängt von D2 (Lane), D3 (Weights-/Workspace-Arena, Staging). Downstream: D6 (Scorer liest Slot-Embeddings), D7 (Encoder schreibt in Slots), G1 (konkrete e5-Config). Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §3, §4.3; `resident.md` (Modelle, Stack); `resident_first.md` (e5-small); `context/context_d.md` D4 (#14–#18), #32/#33/#34.

      ---

      ## Festgelegte Entscheidungen

      - **Modell jetzt:** dense **e5-small** (G1), `BertModel` (candle-transformers), **nicht** `xlm_roberta`. ColBERT = Wechselpfad.
      - **Candle = Adapter an genau einer Stelle (§2):** nur in `model/candle_adapter.rs`. Kein `Tensor` in öffentlichen Signaturen; Ein-/Ausgabe über `Region`+`Layout`.
      - **Namen (`[U]`, #14):** e5-small speichert BERT-Schlüssel ohne `bert.`-Präfix (`embeddings.*`, `encoder.layer.N.*`). `BertModel::load` hat Fallback über `config.model_type` → meist kein Mapping nötig. Prüfen: `config.json` (`model_type:"bert"`, `vocab_size≈250037`, `hidden_size 384`, 12 Layer); Pooler-Gewichte werden **nicht** gebraucht.
      - **safetensors (`[V/U]`, #15):** HF-Repo enthält `model.safetensors`. Falls nicht: `save_pretrained(safe_serialization=True)` oder Candle `VarBuilder::from_pth`. mmap, Zero-Copy auf Rohbytes, pro Engine in deren Weights-Arena.
      - **f16-Laden (#13):** `VarBuilder::from_mmaped_safetensors(&[..], DType::F16, &dev)`; Attention-Maske f16-sicher (**−1e4**).
      - **Projektionskopf (`[U]`, #16/#17):** candle-transformers hat **kein ColBERT-Textmodul** → Backbone + eigener `candle_nn::Linear` aus `VarBuilder`, danach L2 pro Token. Kopf-Ort je Checkpoint: `jina-colbert-v2` `linear.weight` (1024→128, ohne Bias) im Hauptcheckpoint; `answerai-colbert-small-v1` separates `1_Dense/` (384→96). Tensor-Namen per `safetensors`-Header **vorab lesen**. Für e5: **kein** Projektionskopf (384-Dim direkt), stattdessen Mean-Pooling+L2.
      - **Pooling/L2 (#33, e5):** Mean-Pooling über `attention_mask`, dann L2. L2 als **eigener Kernel am Encoder-Ende** (einmal beim Schreiben, nicht im Scorer). Query und Doc gleich behandeln. Referenzwerte aus Python (`sentence-transformers`) als Fixtures; Erwartung f32 Cosinus ≥ 0,9999, f16 ≈ 1e-3.
      - **Präfix (#34):** `query: `/`passage: ` Pflicht (Model Card warnt vor Qualitätsverlust); für symmetrische Aufgaben `query: ` beidseitig. Präfix ist **Anwendungslogik vor `encode`** (D5), nicht Tokenizer-Funktion.
      - **`encode_into` (§4.3):** Phase 1 rechnet Candle; dünner Kernel `WriteSlotOp` liest Device-Pointer des Candle-Tensors (G-PTR) und schreibt **normiert + dtype-konvertiert** in den Slot. Tensor bleibt bis zum End-Event am Leben. **Fallback bei negativem G-WRAP:** D2D-Copy in Slot statt Zero-Copy.
      - **Batching (`[V]`, #18):** nach Länge sortieren, Buckets (32/64/128/256/512), Batch bis `batch*max_len ≤ Token-Budget`, Padding mit `pad_token_id` (XLM-R: 1), Maske 0/1, Zusammenbau `Tensor::stack`/`from_vec`.

      ---

      ## Zu erstellende Artefakte (`rag/src/model/`)

      - `mod.rs`
        - `struct ModelSpec { arch_id, dim, pooling, prefixes, dtype, max_seq_len, head: Option<HeadSpec>, paths }` (reine Daten, besitzt nichts).
        - `struct ModelRegistry` — `"bert" → BertEncoder`, `"maxsim" → MaxSimScorer` (Scorer in D6). Neues Modell = Registry-Eintrag.
        - `trait Encoder`: `encode_into(tokens: Region, out: Region, lane: &dyn ExecContext) -> Result<EncodeEvent, ModelError>`; `dim()`, `dtype()`.
        - `struct LoadedEncoder` — Architektur (Code) + Gewichte (Region in Weights-Arena), entladbar (D1-Lease).
        - `struct ProjectionHead { linear }` (optional; nur ColBERT-Pfad).
      - `candle_adapter.rs` (hinter `cuda` für GPU-Pfad; CPU-Pfad `candle-core` CPU)
        - Backbone-Load (`BertModel::load` + `VarBuilder::from_mmaped_safetensors`), Forward, Pointer-Zugriff für `WriteSlotOp`. **Einzige** Stelle mit `candle_core::Tensor`.
      - `pooling.rs`: Mean-Pooling über Maske; L2-Kernel (CPU + CUDA via D6-Registry).
      - `batch.rs`: Token-Budget-Batcher, Längen-Bucketing, Padding/Maske.

      ---

      ## Prototyp-Gates (zuerst)

      - **G-CKPT:** `safetensors`-Header lesen; e5 `model.safetensors`+`config.json` vorhanden/kompatibel; (für ColBERT-Pfad) Kopf-Ort/Format.
      - **G-PTR** (aus D2) als Voraussetzung für `WriteSlotOp`.

      ---

      ## Arbeitsschritte

      1. `ModelSpec`/`ModelRegistry` + `Encoder`-Trait definieren (GPU-frei).
      2. `candle_adapter`: e5 `BertModel` laden (f16 GPU / f32 CPU), Config-Checks (model_type/vocab/hidden/layers), Pooler ignorieren.
      3. Mean-Pooling+L2 implementieren; Referenz-Fixtures aus `sentence-transformers` ablegen; Cosinus-Toleranz prüfen.
      4. `encode_into` + `WriteSlotOp` (normiert + dtype-konvertiert in Slot); Tensor-Lebensdauer bis End-Event; Fallback D2D falls G-WRAP negativ.
      5. Batching per Token-Budget/Bucketing; Präfix-Hook (von D5 geliefert).
      6. (Vorbereitet, nicht aktiv) `ProjectionHead` + per-Token-L2 für ColBERT-Wechselpfad; Namen per safetensors-Header.

      ## Fehler (Q1) & Logging (Q2)

      - `enum ModelError { Load{source}, WeightNameMismatch{expected}, UnsupportedArch, Incompatible{..}, EncodeFailed{source} }`.
      - Span pro Modell (`model_id`, `dtype`, `batch_tokens`); Encode-Events `loaded`, `batch`, `write_slot`. Große Argumente nie loggen (`skip_all`).

      ## Feature-Flags (Q4)

      - GPU-Forward + `WriteSlotOp`-CUDA hinter `cuda`; CPU-Forward über candle-CPU immer verfügbar. `Encoder`-Trait entkoppelt Logik.

      ## Tests (Q3)

      - Golden-Tests e5: f32 vs. Python-Referenz (Cosinus ≥ 0,9999), f16-Toleranz (~1e-3).
      - Batcher: Bucketing, Token-Budget-Einhaltung, Padding/Maske korrekt.
      - `WriteSlotOp` gegen Mock-Slot/Mock-Lane (normiert, richtiger dtype).
      - Fehlerpfade: fehlende Gewichte, Namensmismatch.

      ## Definition of Done

      - e5-Encoder lädt und kodiert korrekt (Golden-Tests grün); `encode_into`/`WriteSlotOp` schreibt in Slots; Candle nur im Adapter; ColBERT-Pfad als Registry-/Head-Hook vorbereitet.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d5_tokenizer_normalizer.md
    mode: replace
    content: |
      # D5 — Tokenizer & Normalizer (CPU, geteilt)

      **Zweck:** `tokenizers`-Crate, genau eine Instanz, Query/Doc-Modus (Marker/`[MASK]`/doc_maxlen), Normalizer CPU, Ausgabe `u32`-IDs + Attention-Maske, Längen-Bucketing. **Widerspruch GPU-Tokenizer aufgelöst.**

      **Einordnung:** Abhängt von D1 (Lifecycle/Lease der geteilten CPU-Ressource). Downstream: D4 (liefert IDs+Maske+Präfix), G1. CPU-only, kann parallel zu D2/D3 entstehen. Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §3; `resident_first.md` (Tokenizer); `resident.md` (GPU-Tokenizer-Idee); `context/context_d.md` D5 (#19–#21), #20/#34.

      ---

      ## Festgelegte Entscheidungen

      - **Widerspruch aufgelöst (`[V-Argumentation]`, #19):** **Kein** GPU-Tokenizer, **kein** ID→Adresse-Gather-Kernel. Begründung: Tokenisierung ist String-Verarbeitung (CPU); der Embedding-Lookup ist ein `seq×384`-Gather, gegenüber 12 Transformer-Layern unmessbar; ein eigener Gather-Kernel spart nichts, bricht candle-Kompatibilität und erzwingt Adresspflege bei Modell-Eviction. **`resident_subengine.md` („nur Ausblick") gewinnt gegenüber `resident.md`.** Pipeline: **CPU-Tokenizer → `u32`-IDs → H2D → `index_select`.**
      - **tokenizers (`[V]`, #20):** `Encoding::get_ids() -> &[u32]`, `get_attention_mask() -> &[u32]`, `get_type_ids()`. **Normalisierung liegt im Tokenizer** (`tokenizer.json`: Normalizer → Pre-Tokenizer → Model → Post-Processor) — nicht separat nachbauen. Batch: `encode_batch` + `with_padding`/`with_truncation`.
      - **Genau eine Instanz:** geteilte CPU-Ressource beider Subengines (D1-Lease), modellspezifisch oder geteilt.
      - **Modi (`[U]`, #21):**
        - **e5:** `"query: "`/`"passage: "` als **Klartext vor** der Eingabe; Special Tokens `<s>…</s>` fügt der Tokenizer hinzu.
        - **ColBERT (Wechselpfad):** Marker (`[Q]`/`[D]` bzw. Prefix-Tokens je Checkpoint); Query-Padding mit `[MASK]` bis `query_maxlen` (32); Dokument-Kürzung auf `doc_maxlen`. Das ist **Anwendungslogik nach `encode`**, keine Tokenizer-Funktion; je Checkpoint-Doku prüfen.
      - **Präfix (#34):** Pflicht für e5; für symmetrische Aufgaben `query: ` beidseitig. Falsches/fehlendes Präfix verschlechtert Qualität deutlich → Tests im Query- und Index-Pfad.
      - **Bucketing:** Padding auf gemeinsame Länge im Batch, Längen-Bucketing (koordiniert mit D4-Batcher).

      ---

      ## Zu erstellende Artefakte (`rag/src/tokenize/`)

      - `mod.rs`
        - `struct SharedTokenizer` — eine `tokenizers::Tokenizer`-Instanz (geteilt, D1-Lease).
        - `enum EncodeMode { Query, Passage }` (e5) / erweiterbar für ColBERT-Marker.
        - `struct TokenBatch { ids: Vec<u32>, mask: Vec<u32>, seq_len, offsets_per_task }`.
        - `fn encode(mode, texts: &[&str]) -> Result<TokenBatch, TokenizeError>` — Präfix anwenden, `encode_batch`, Padding/Truncation, Bucketing.
      - `modes.rs`
        - e5-Präfixlogik; ColBERT-Marker/`[MASK]`-Padding/`doc_maxlen` (vorbereitet, Wechselpfad).
      - `normalize.rs`
        - Verweis: Normalisierung bleibt **im** Tokenizer; nur optionale **vorgeschaltete** externe Chunk-Textnormalisierung als separater, konfigurierbarer Hook (nicht Teil der Engine-Kernlogik).

      ---

      ## Arbeitsschritte

      1. `SharedTokenizer` aus `tokenizer.json` laden; genau eine Instanz, als D1-Ressource mit Lease.
      2. `EncodeMode` + Präfix-Anwendung (e5 `query:`/`passage:`).
      3. `encode`/`encode_batch` mit `with_padding`/`with_truncation`; Ausgabe `u32`-IDs + Maske + Offsets pro Task.
      4. Längen-Bucketing mit D4-Batcher abstimmen (gemeinsame Buckets).
      5. ColBERT-Modus (Marker, `[MASK]`-Padding bis `query_maxlen`, `doc_maxlen`) als vorbereiteten Pfad implementieren, inaktiv für e5.
      6. Dokumentieren, dass GPU-Tokenizer/Adressmapping **nicht** umgesetzt wird (Widerspruch-Auflösung in Code-Doku).

      ## Fehler (Q1) & Logging (Q2)

      - `enum TokenizeError { Load{source}, EncodeFailed{source}, EmptyInput, Truncated{at} }`.
      - Span pro Batch (`mode`, `n_texts`, `seq_len`); Events `encoded`, `bucketed`. Texte **nicht** loggen (PII/Größe), nur Längen.

      ## Feature-Flags (Q4)

      - Komplett **GPU-frei**; baut ohne `cuda`. IDs gehen erst in D4 (H2D) auf die GPU.

      ## Tests (Q3)

      - IDs/Maske gegen Referenz-Encodings (Fixtures) für e5 (query/passage).
      - Präfix-Effekt: falsches/fehlendes Präfix erkennbar (Test prüft, dass Präfix tatsächlich vorangestellt wird).
      - Bucketing/Padding: korrekte `seq_len`, Maske 0/1, Offsets pro Task.
      - ColBERT-Modus: `[MASK]`-Padding bis `query_maxlen`, `doc_maxlen`-Kürzung.

      ## Definition of Done

      - Eine geteilte Tokenizer-Instanz; e5 Query/Passage korrekt; Bucketing mit D4 abgestimmt; GPU-Tokenizer-Verzicht dokumentiert; ColBERT-Modus vorbereitet.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d6_kernels_scorer.md
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d7_index_segments.md
    mode: replace
    content: |
      # D7 — Index-Segmente in der Subengine

      **Zweck:** `SegmentView`/Slot/Slot-Klassen, Schreiben/Upsert/Gültigkeitsbit, Mapping `(segment,slot)→EntryRef`, Segment-Basistabelle, bidirektionale Sync. **Brücke zu Gruppe E.**

      > Hinweis: „Index" meint hier den **Vektorindex** (RAG), keine Dateiliste. D7 überlappt stark mit Gruppe E (Index-Management); E referenziert die hier festgelegte Segment-Repräsentation.

      **Einordnung:** Abhängt von B4 (Index-Trait), D3 (Index-Segment-Arena/Staging). Downstream: D6 (Kernel schreibt/liest Slots), Gruppe E (E1/E5/E8 bauen darauf). Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §4.4; `resident_index.md` (Segment/Slot/Sync, via Gruppe E); `context/context_d.md` D7 (#27–#29).

      ---

      ## Festgelegte Entscheidungen

      - **Konsistenzmodell (Empfehlung festgelegt, in D7 bestätigen, `[V-Konzept]`, #27):** **immutable Segmente + Generations-Manifest** (Lucene/Tantivy-Prinzip). Host baut neues Segment/Manifest, publiziert per **`arc-swap`**; Leser **pinnen die Generation per Lease** (D1). Altes Segment wird erst freigegeben, wenn **Leases==0 UND alle Kernel-Stream-Events abgeschlossen** (D3 stream-geordnete Freigabe). Alternativen (nicht gewählt): Epoch-Reclamation (`crossbeam-epoch`), Double-Buffering. **VRAM-Budget für 2 Generationen** (CoW-Spitze, D3/E3).
      - **SegmentView (§4.4):** Basisadresse als `Region`, Slot-Zahl, Stride, dtype, **Generation**.
      - **Slot-Klassen (`[U]`, #28):** Adresse = `base + slot*stride(class)`; getrennte Längen-Arrays + **Valid-Bitmap**; Gültigkeit/Generationsnummer je Slot. Slot = feste Größe `T_max × dim × dtype`; Klassen (z. B. 32/64/128 Tokens) halten den Kernel **lookup-frei**.
      - **Padding (§4.4):** gültige Token-Zahl je Slot; Padding mit **−inf/−1e4** maskiert (nicht 0).
      - **Upsert (#28):** neuen Slot schreiben → **stream-geordnet** Valid-Bit setzen → altes Bit löschen (**Tombstone**). **Kein In-Place** während laufender Kernels. Kompaktierung durch **Segment-Rebuild** (vgl. vLLM-PagedAttention-Blöcke).
      - **Optionales Feld pro Token:** Centroid-ID/Code (für späteren Funnel; vom Kernel ignoriert, wenn ungenutzt).
      - **Kernel kennt nur Slots (§4.4):** Ergebnis `(segment_gen, slot, score)`. **Rückabbildung `(segment,slot)→EntryRef`** (Chunk/Zeile/Bereich/Datei) macht der **Manager auf der CPU**. Treffer aus entladenen Segmenten werden **verworfen oder remapped**. Segment-Basisadressen gehen als **kleine Tabelle über Segmente** (nicht Einträge) an den Kernel.
      - **Bidirektionale Sync (`[U]`, #29):** Segment-Ereignisse („hinzugefügt/geändert") über `tokio::sync::mpsc`/`broadcast`; Kopie auf der **Copy-Lane** mit Pinned-Staging (D2/D3); **Publikation erst nach Event-Completion**; Version/Generation im Event.

      ---

      ## Zu erstellende Artefakte (`rag/src/segment/`)

      - `mod.rs`
        - `struct SegmentView { base: Region, slots: u32, stride: u32, class: SlotClass, dtype: DType, gen: u64 }`.
        - `enum SlotClass { T32, T64, T128, .. }` (konfigurierbar) + `fn stride(class, dim, dtype)`.
        - `struct SegmentTable` — kleine Basistabelle über Segmente (an Kernel übergeben).
        - `struct Manifest` — Generations-Manifest, publiziert per `arc_swap::ArcSwap<Manifest>`.
      - `slots.rs`
        - `struct SlotMeta { valid: bool, token_count: u16, gen: u64, centroid: Option<u32> }`; Valid-Bitmap; Upsert (write→set-valid→tombstone), stream-geordnet.
      - `mapping.rs`
        - `struct SlotMap` — `(segment_gen, slot) → EntryRef`; Rückabbildung **Fixed** (Arithmetik) / **Variable** (Binärsuche über Offsets). Entladene Segmente → verwerfen/remappen.
      - `sync.rs`
        - Segment-Ereigniskanal (`mpsc`/`broadcast`), Copy-Lane-Anbindung (Staging), Publikation nach Event, Generation im Event. Konvergenz mit Index-Objekt (Gruppe E).

      ---

      ## Arbeitsschritte

      1. `SegmentView`/`SlotClass`/`SegmentTable` definieren; Generation durchgängig.
      2. Konsistenzmodell (immutable Segmente + Generations-Manifest, `arc-swap`) implementieren; Leases pinnen Generation; Freigabe erst Leases==0 ∧ Events fertig (mit D3 koppeln).
      3. Slot-Schreiben/Upsert/Tombstone **stream-geordnet**; Valid-Bitmap; Padding-Maskierung.
      4. `SlotMap`-Rückabbildung (Fixed/Variable); Umgang mit entladenen Segmenten.
      5. Segment-Basistabelle an Kernel (D6) übergeben.
      6. Bidirektionale Sync über Ereigniskanal + Copy-Lane (D2) + Staging (D3); Publikation nach Event.
      7. **Konsistenzmodell bestätigen** (offene Festlegung aus Metaplan/§4.4) und mit Gruppe E (E5/E6/E8) abstimmen.

      ## Fehler (Q1) & Logging (Q2)

      - `enum SegmentError { StaleGeneration{gen}, SlotOutOfRange, MappingMissing, SyncFailed{source} }`; intern `StaleMapping` (`pub(crate)`), an API-Grenze abgebildet.
      - Span pro Segment/Generation (`segment_gen`, `slot_class`); Events `segment_added/changed`, `upsert`, `tombstone`, `remap`. Sync-Pfad auf `debug`.

      ## Feature-Flags (Q4)

      - Slot-/Mapping-/Manifest-Logik **GPU-frei** (RAM-Modell, testbar ohne GPU). Copy-Lane/Staging-Pfad hinter `cuda`.

      ## Tests (Q3)

      - Upsert/Tombstone/Valid-Bitmap: korrekte Sichtbarkeit; kein In-Place während „laufender Kernel" (Mock-Events).
      - `SlotMap`: Fixed-Arithmetik und Variable-Binärsuche; entladene Segmente verworfen/remappt.
      - Generations-Reclamation: Freigabe erst Leases==0 ∧ Events fertig (Mock-Events, `loom`/`shuttle` für Race).
      - Sync: Publikation erst nach Event-Completion; ABA/Generation-Wechsel zwischen Lesen und CAS (Failpoints).

      ## Definition of Done

      - `SegmentView`/Slot-Klassen/Manifest implementiert; immutable-Segmente-Modell bestätigt und mit Gruppe E abgestimmt; Rückabbildung + bidirektionale Sync getestet (No-GPU-Modell grün).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d8_cpu_gpu_buildout.md
    mode: replace
    content: |
      # D8 — CPU-vs-GPU-Unterschiede & Ausbaupfad

      **Zweck:** Unterschiede (§8) konsolidieren und den API-bruchfreien Ausbaupfad (§9, Phasen 1–5) in konkrete Planungsstufen überführen.

      **Einordnung:** Abhängt von D1–D7 (konsolidierend). Downstream: steuert die Reihenfolge der GPU-Optimierungen und sichert die Erweiterungspunkte (Gruppe H). Querschnitt Q1–Q4.

      **Quellen:** `resident_subengine.md` §8, §9, §4.6, §10; `context/context_d.md` D8 (#30–#31), #32/#33; `context_h_q.md` (#15–#18 Graph-Capture, #1–#7 Ringbuffer).

      ---

      ## Festgelegte Entscheidungen

      - **Unterschiede CPU/GPU (§8):**
        - **CPU:** mehr Speicher; alles CPU; Scorer selbst (SIMD, D6); Encoder über Candle-CPU; Index per mmap lesbar; **kein** Staging/Streams.
        - **GPU:** begrenzter VRAM; Streams/Staging/Transfers; Tokenizer/Normalizer CPU-gebunden (Bottleneck); Batching nötig zum Streamen/Swappen von Segmenten/Kernels.
        - **Gemeinsam:** dieselben Traits (`Op`, `Region`, `ExecContext`), dieselbe Lease-Logik, dieselben Task-Pipelines.
      - **Numerik (`[U]`, #30/#31):** f32 (CPU) vs f16 (GPU) → Cosinus-Differenz 1e-3…1e-4, Ranking-Unterschiede nur bei Beinahe-Gleichstand. CPU-f16 in candle ist langsam → **CPU f32**. Gemischte Indizes nur mit Toleranz; idealerweise **dtype pro Index** festlegen. cuBLAS-Akkumulation ≠ candle-Ops → **Golden-Tests je Phase**.
      - **Trait-Hygiene (#30):** `Op`/`Region`/`ExecContext` **nie** an `candle_core::Tensor` leaken, sonst bricht die API beim Ersetzen von candle. Event-Ordnung zwischen Candle- und eigenen Kerneln **explizit** (candle nutzt eigenen Allokator/Stream).
      - **Ausbaupfad (§9, verbindliche Stufenreihenfolge):**
        1. **Phase 1:** `Region`/`Op`/`ExecContext` als dünne Hüllen um Candle; MaxSim als Kachel-Kette; Engine-Gerüst, Lazy-Init, Leases. *(D1–D7 Grundlevel)*
        2. **Phase 2:** eigene Streams, eigener MaxSim-Kernel (NVRTC), Slice-Scheduling. *(D2/D6)*
        3. **Phase 3:** eigene Arena für Gewichte/Workspace, `encode_into` ohne Copy (abhängig von G-WRAP). *(D3/D4)*
        4. **Phase 4:** Graph-Decorator, Persistent-Scorer mit Ring. *(§4.6, H4/H1 — nur Erweiterungspunkte sichern)*
        5. **Phase 5:** optional eigener Encoder-Forward (cuBLAS + wenige Kernels) hinter demselben `Op`. *(D4)*

      ---

      ## Erweiterungspunkte sichern (Gruppe H — nur Hooks, keine Detailplanung)

      - **H1 Ringbuffer/Persistent-Kernel (§4.6):** Zustand (Head/Tail/Stop-Flag/Zähler) in eigener Region; Grid ≤ SMs×Occupancy; Stop-Flag nur am Scheibenende; Acquire-Load `ld.acquire.sys` (sm75 ok). **Hook:** eigener Stream + feste Ring-Slots im Manager vorsehen (D2/D7). Sinnvoll für Scorer, nicht Encoder.
      - **H4 Graph-Capture (§4.6):** Launcher-Decorator `Direct`/`Capture`, Cache-Schlüssel `(Op, Shape-Bucket, Regions, dtype, Stream)`; Invalidierung bei `gen`-Wechsel; feste Ring-Slots. **Hook:** Launcher so bauen, dass Capture später eingehängt werden kann. **G-GRAPH** vor Aktivierung.
      - Allgemein: Parameterobjekte erweiterbar, `requires()`-Affinität offen, Op-Austausch hinter gleichem Trait (gilt für alle H).

      ---

      ## Zu erstellende Artefakte

      - `docs`/Code-Doku: **Stufen-/Phasenplan** (Phase 1–5) mit Zuordnung zu D-Paketen und Abnahmekriterien je Phase.
      - `kernel/launcher.rs`: Launcher mit Decorator-Punkt (`Direct` jetzt, `Capture` als Hook).
      - Golden-Test-Harness: Referenzwerte je Phase (f32-Referenz, f16-Toleranz), damit Phasen-Austausch (Candle→eigener Kernel) verifizierbar bleibt.
      - `compat.rs` (oder Review-Checkliste): Prüfung, dass öffentliche Signaturen frei von `Tensor`/Raw-Pointern sind.

      ---

      ## Arbeitsschritte

      1. §8-Unterschiede als CPU/GPU-Fähigkeitsmatrix festhalten (Speicher, Streams, Staging, Scorer-Impl, dtype).
      2. Phasen 1–5 als Meilensteine mit Abnahmekriterien definieren; D-Pakete zuordnen.
      3. Golden-Test-Harness (f32-Referenz + Toleranzen) aufsetzen; je Phase wiederverwendbar.
      4. Launcher mit Decorator-Punkt (Capture-Hook) und festen Ring-Slots vorbereiten.
      5. Trait-Hygiene-Review: keine `Tensor`/Pointer-Leaks; Event-Ordnung Candle↔eigene Kernel explizit.
      6. H-Hooks (H1/H4) als Erweiterungspunkte dokumentieren; **keine** Detailimplementierung.
      7. Prototyp-Gates verbuchen: G-WRAP (Phase 3), G-GRAPH (Phase 4), G-MAXSIM (Phase 2).

      ## Fehler (Q1) & Logging (Q2)

      - Keine neuen Fehlertypen; D8 konsolidiert. Phasenübergänge als `info`-Events; Golden-Test-Abweichungen als strukturierte Diagnose (nicht als Panic).

      ## Feature-Flags (Q4)

      - Phasen 2–5 sind GPU-spezifisch (hinter `cuda`). Phase 1 CPU-Pfad + Candle-CPU bleibt ohne `cuda` voll funktionsfähig und testbar.

      ## Tests (Q3)

      - Golden-Tests je Phase (CPU-f32-Referenz; f16-Toleranz ~1e-3; kein exakter Gleichheitstest).
      - API-Kompatibilitätstest: Austausch Candle-Op ↔ eigener Kernel ändert öffentliche Signatur nicht.
      - Phasen-Regression: Phase-2-Kernel liefert gleiche Rangfolge wie Phase-1-Kette (innerhalb Toleranz).

      ## Definition of Done

      - Phasenplan 1–5 dokumentiert und an D-Pakete gebunden; Golden-Test-Harness vorhanden; H1/H4-Hooks gesichert; Trait-Hygiene bestätigt; Prototyp-Gates den Phasen zugeordnet.
  reason: "Create the Group D overview plan plus the eight per-package implementation plans (D1–D8) as actionable agent plans, grounded in resident_planplan.md, context_d.md, resident_subengine.md and the Q cross-cutting conventions."
```
/answer 0f9c-16 allow
Result Stats: id=c468822d-f77a-4c85-a5de-9b647eaf6612, total: 186445, in: 125, out: 39693, reason: 2509, read: 71764, write: 146627
ReasoningToken: 2483
ReasoningToken: 26
Erledigt. Ich habe die Umsetzungspläne für Gruppe D (Subengine-Ressourcen) in `/home/user/xyan/xy.ai.workbench/project/resident/plan/` erstellt — ein Überblicksdokument plus je eine agenten-abarbeitbare Plandatei pro Paket D1–D8.

## Erstellte Dateien

| Datei | Inhalt |
|------|--------|
| `plan_d_overview.md` | Gemeinsame Entscheidungen, Reihenfolge/Abhängigkeiten, Prototyp-Gates, Querschnitt Q1–Q4, Modulschnitt, DoD |
| `plan_d1_subengine_lifecycle_pull.md` | Lebenszyklus, Leases (RAII), Lazy-Init (`OnceCell`+Semaphore), Eviction (Double-Check), EWMA-Pull |
| `plan_d2_device_execcontext.md` | `DeviceHandle`, `ExecContext`/Lane (Streams, Prioritäten, Events), Init-Kosten |
| `plan_d3_gpu_memory_arenas.md` | Arenen, Budget (`cuMemGetInfo`), Pinned Staging, dtype, stream-geordnete Freigabe |
| `plan_d4_model_encoder.md` | `ModelSpec`/Registry, `LoadedEncoder` (Candle-Adapter), L2, `encode_into`/`WriteSlotOp`, Batching |
| `plan_d5_tokenizer_normalizer.md` | CPU-Tokenizer, Query/Doc-Modi, Bucketing |
| `plan_d6_kernels_scorer.md` | `KernelRegistry`, `MaxSimOp`, Cosine, CPU-SIMD, Work-Slice, Top-k |
| `plan_d7_index_segments.md` | `SegmentView`/Slot-Klassen, Konsistenzmodell, Mapping, bidirektionale Sync |
| `plan_d8_cpu_gpu_buildout.md` | CPU/GPU-Unterschiede, Ausbaupfad Phasen 1–5, H-Hooks |

## Wichtigste eingearbeitete Festlegungen (aus `context_d.md`)

- **Revisionshinweis**: Direkte Task-Nutzung in Subengines ist veraltet → Planner/Processor-Schicht; D1 definiert keine eigene Task-Queue, sondern Lifecycle + Kapazitäts-/Pull-Schnittstelle.
- **D5-Widerspruch aufgelöst**: CPU-Tokenizer + `index_select`, kein GPU-Adressmapping (`resident_subengine.md` gewinnt).
- **D7-Konsistenzmodell** festgelegt (zu bestätigen): immutable Segmente + Generations-Manifest via `arc-swap`, Freigabe erst bei Leases==0 ∧ Events fertig.
- **D3↔E3** als eine Arena-Spezifikation konsolidiert.
- **Prototyp-Gates** als blockierende erste Schritte verankert, v. a. **G-WRAP** (`wrap_cuda_slice`/Drop — größtes Risiko) und **G-ALIGN** (cudarc-Versionsgleichheit).
- Jedes Paket enthält Artefakte (vorgeschlagene Modulpfade), Arbeitsschritte, Q1–Q4 (Fehler/Logging/Tests/Feature-Flags) und Definition of Done.

Zu `resident_index.md` habe ich den Hinweis berücksichtigt: Er betrifft den **Vektorindex** (nicht eine Dateiliste) und wurde nur dort referenziert, wo D7 die Segment-Repräsentation und Sync berührt.

Zwei Punkte sind in den Plänen bewusst als **zu bestätigen** markiert, da `resident_planplan.md`/`context_d.md` sie offenlassen: das D7-Konsistenzmodell (Empfehlung übernommen) und der vorgeschlagene Modulschnitt (final in Paket A2). Falls du für diese eine verbindliche Vorgabe hast, passe ich die Pläne entsprechend an.
