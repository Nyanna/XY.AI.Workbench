Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe F aus `resident_planplan.md`.

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
SystemInit:  id=2fd17d89-cf5c-4d1a-ac56-0cbcdf46003b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by reading the key files to understand the task: the planning document and the context for Group F.
Control Request:
```yaml
id: 64b3-1
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_f.md
  reason: Reading the master planning document and Group F context to understand what implementation plans need to be created.
```
/answer 64b3-1 allow
ReasoningToken: 96
Text:
 I have the planning document and Group F context. Now let me read the primary source document `resident_precision.md` and the cross-cutting context `context_h_q.md`, plus the D6 source (`resident_subengine.md` §4.5) which F1 depends on.
Control Request:
```yaml
id: 64b3-2
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_h_q.md
  reason: "Reading the core precision/calibration source document (primary source for Group F) and the cross-cutting Q context (error taxonomy, logging, tests, feature flags) that all F packages must follow."
```
/answer 64b3-2 allow
ReasoningToken: 853
Text:
 I now have everything I need: the metaplan (Group F = F1–F5), the detailed source `resident_precision.md`, the Group‑F research context, and the cross‑cutting Q context. The note about `resident_index.md` only confirms that Group F's sources are `resident_precision.md` (+ `resident_subengine.md` §4.5 for D6), not an index/TOC file.

I'll create one overview plan plus one detailed implementation plan per package (F1–F5) in the `plan` directory.
Control Request:
```yaml
id: 64b3-6
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f0_overview.md
    mode: replace
    content: |
      # Umsetzungsplan Gruppe F — Präzision & Kalibrierung (Übersicht)

      > Stufe-2-Umsetzungsplan zum Metaplan `resident_planplan.md`, Gruppe F.
      > Primärquelle: `resident_precision.md`. Recherche-/Kontextquelle: `context/context_f.md`.
      > Querschnitt (Fehler/Logging/Tests/Feature-Flags): `context/context_h_q.md` (Q1–Q4).
      >
      > **Hinweis zur Quellenlage:** `resident_index.md` ist die Dokumentation des Vektorindex-Subsystems (Gruppe E) und **keine** Inhalts-/Indexdatei des Projekts. Gruppe F bezieht ihre Fachvorgaben ausschließlich aus `resident_precision.md`; `resident_subengine.md` §4.5 (D6) liefert die Kernel-/Scoring-Pfade, gegen die die Capability-Achsen validiert werden.

      ---

      ## 1. Zweck & Scope der Gruppe

      Gruppe F implementiert die **Präzisions-/Kalibrierungsschicht** der Resident-Engine: Jede Subengine (CPU, CUDA) erhebt ihre Rechenpfade (`CapabilitySet`), ein `Calibrator` misst sie interleaved gegen die fp32-Referenz, leitet aus der Pareto-Front **unbenannte Profile** (`Profile { position ∈ [0,1], config }`) ab, ein Task wird über `TaskParams.precision ∈ [0,1]` auf das nächstliegende Profil gematcht, und ein `CalibrationStore` persistiert das Ergebnis pro `Modell × Subengine`.

      ### Leitprinzipien (aus `resident_precision.md`, verbindlich)
      - Subengines, Kernel, Modelle, Indizes sind **nicht gekoppelt**. Kein dtype-/Rundungsvertrag zwischen CPU und GPU.
      - Varianz/Indeterminismus sind erlaubt. Ergebnisse sind **akzeptierte Variablen**, keine Garantien. Keine Bit-Reproduzierbarkeit (Tests prüfen Schranken, nicht Gleichheit).
      - Präzision ist ein **Wunsch**, keine Zusicherung. Ein Wunsch ist immer erfüllbar (nächstliegendes aktives Profil).
      - Kalibrierung darf den Nutzer **nie** blockieren/unterbrechen (niedrige Priorität, Zeit-/Wiederholungs-Obergrenze, kein Warten auf GPU-Preemption).
      - **Keine** Durchsatzwerte aus der Kalibrierung für die Lastverteilung (Pull bleibt selbstorganisiert über realen Fortschritt).
      - Die einzige harte Bindung Index↔Modell ist **Modell-ID + Dimension** (im Index-Header); das Profil ist höchstens optionale Diagnose.

      ### Ausdrücklich NICHT Teil von Gruppe F
      - Chunking, Query-Formulierung, Reranking, Kandidaten-k, Indexstruktur (→ Suchschicht).
      - Index-Persistenzformat / VRAM-Spiegelung (→ Hauptkonzept / Gruppe E/G).
      - Nutzung der gemessenen Zeiten für Scheduling/Backpressure (→ Processor Gruppe C, nicht F).

      ---

      ## 2. Paketüberblick & Reihenfolge

      | Paket | Thema | Datei | Hängt ab von |
      |------|-------|-------|--------------|
      | F1 | CapabilitySet-Erhebung | `plan_f1_capabilityset.md` | B6 (Typen), D6 (Kernel-/Scoring-Pfade) |
      | F2 | Calibrator (Messung, Pruning, Pareto) | `plan_f2_calibrator.md` | F1 |
      | F3 | Profilbildung (Spread/Bogenlänge) | `plan_f3_profilbildung.md` | F2 |
      | F4 | Task-Matching (`precision → Profil`) | `plan_f4_task_matching.md` | F3, B6, C1 |
      | F5 | CalibrationStore / Persistenz | `plan_f5_calibration_store.md` | F3 |

      **Bearbeitungssequenz:** `F1 → F2 → F3 → {F4, F5}` (F4 und F5 sind nach F3 unabhängig und parallelisierbar).

      **Reifegrad der Recherche (siehe `context_f.md`):** F1/Q1–2 und F2/Q6 sind **[V]** (quellenbelegt), alle übrigen Antworten **[W]** (Fachwissen, vor Implementierung verifizieren). Alle `[W]`/`[prüfen]`-Punkte sind in den Paketplänen als *Offene Verifikation* geführt und müssen VOR Festschreibung einer Konstanten/API geprüft werden.

      ---

      ## 3. Gemeinsamer Modul-/Typschnitt

      Empfohlener Modulbaum (Crate der Engine, Modul `precision`):

      ```
      precision/
        mod.rs          // Re-Exports, Doku der Skala (0=fast, 1=precision)
        capability.rs   // F1: CapabilitySet, Achsen/Werte-Katalog, CapabilityProbe-Trait
        capability_cpu.rs   // F1: CPU-Probe (raw-cpuid, sysinfo, half)
        capability_cuda.rs  // F1: GPU-Probe (cudarc)            [cfg(feature = "cuda")]
        measure.rs      // F2: Measure-Trait + Cfg + Sample/MeasureResult
        measure_cuda.rs // F2: reale Messung (CUDA-Events)        [cfg(feature = "cuda")]
        measure_mock.rs // F2/Q3: FakeMeasure (seeded RNG, Skripte) [cfg(test)]
        calibrate.rs    // F2: Calibrator, Aufzählung/Pruning/Pareto
        profile.rs      // F3: Profile, Profilbildung (Normierung, Bogenlänge, Linkage)
        matching.rs     // F4: match_profile(precision, &[Profile])
        store.rs        // F5: CalibrationStore, Fingerprint, Zustände, Serde-Schema
        error.rs        // Q1: PrecisionError (thiserror, #[non_exhaustive])
      ```

      ### Gemeinsame Verträge (einmal definieren, alle Pakete nutzen sie)

      ```rust
      /// Welche Subengine. Erweiterbar (weitere Backends später).
      #[non_exhaustive]
      pub enum EngineKind { Cpu, Cuda /* , ... */ }

      /// Messabstraktion (F2) — EINZIGER Weg, wie der Calibrator Zeit+Abweichung bekommt.
      /// Reale CUDA-Messung und Mock implementieren dasselbe Trait → GPU-frei testbar.
      pub trait Measure {
          /// Misst eine Kombination: liefert robuste Zeit (Median/Min) und die
          /// Abweichung gegen die fp32-Referenz desselben Testsets.
          fn measure(&mut self, cfg: &Cfg) -> Result<MeasureResult, PrecisionError>;
      }

      /// Capability-Erhebung (F1) — pro Engine eine Implementierung.
      pub trait CapabilityProbe {
          fn probe(&self) -> Result<CapabilitySet, PrecisionError>;
      }
      ```

      B6 besitzt formal `CapabilitySet` und `Profile { position: f32, config }`. Falls B6 zum Startzeitpunkt noch nicht vorliegt, definiert F1 diese Typen provisorisch in `capability.rs`/`profile.rs` und markiert sie mit `// TODO(B6): konsolidieren`. **Keine** festen Profilnamen, **kein** geschlossenes Profil-Enum (harte Vorgabe).

      ---

      ## 4. Querschnitt Q für alle F-Pakete

      ### Q1 · Fehler (`thiserror`)
      Ein **modul-lokales** Enum `PrecisionError` (`#[non_exhaustive]`), das an der Engine-API per `#[from]` in den Top-Level-Fehler aggregiert wird (kein Mega-Enum; vgl. `context_h_q.md` Q1/26). Pflicht-Varianten aus `resident_precision.md`:
      ```rust
      #[derive(thiserror::Error, Debug)]
      #[non_exhaustive]
      pub enum PrecisionError {
          #[error("combination incompatible: {reason}")]
          Incompatible { reason: &'static str },          // F2: Maximalabweichung/degeneriert
          #[error("calibration did not stabilize (best-effort used)")]
          CalibrationUnstable,                            // F2: Obergrenze erreicht
          #[error("no active profile for model {model_id}")]
          NoActiveProfile { model_id: String },           // F3/F4: Modell gesperrt
          #[error("invalid precision value (NaN)")]
          InvalidPrecision,                               // F4: NaN im Task-Wunsch
          // ergänzend, intern/konfig:
          #[error("capability probe failed: {0}")]
          Capability(String),                             // F1
          #[error("calibration store io/serde error")]
          Store(#[source] std::io::Error),                // F5
      }
      ```
      `CalibrationUnstable` ist **kein** Abbruch des Betriebs — es markiert ein persistiertes, aber unsicheres Ergebnis. `NoActiveProfile` tritt **lazy** beim ersten Task auf (siehe `resident_precision.md`, „Lazy-Initialisierung“): die Subengine wird für dieses Modell markiert, Folgetasks wiederholen die Prüfung nicht.

      ### Q2 · Logging (`tracing`)
      - Span pro Kalibrierzyklus `calibrate{engine, model_id}`; Unter-Span pro Kombination.
      - Zu loggen (Vorgabe `resident_precision.md`): Ergebnis pro Kombination, **verworfene** Kombinationen mit Grund, abgeleitete Profile (Anzahl + Positionen), **gewähltes Profil pro Task auf `debug`**.
      - CUDA-Host-Callbacks: **keine** `tracing`-Aufrufe im Callback; Daten per Channel/Atomic an einen Logging-Thread (vgl. Q2/32).

      ### Q3 · Tests / Mocks
      - `FakeMeasure` (manueller Fake, nicht `mockall`) hinter `Measure`: seeded RNG + Skripte **stabil / Ausreißer / driftend / bimodal / nie stabil** (vgl. Q3/37). Zeit über injizierten `Clock`-Trait abstrahieren.
      - `FakeCapabilityProbe` liefert deterministische Achsenkataloge für F2/F3-Tests.
      - Kein echter GPU-Zugriff in Default-Tests (Feature `cuda` aus). Logik läuft gegen die Traits.
      - Pflicht-Testmatrizen je Paket siehe jeweiliger Paketplan (Zusammenfassung in `resident_precision.md`, Abschnitt „Tests“).

      ### Q4 · Feature-Flags
      - `default = []`, reale CUDA-Pfade hinter `cuda` (additiv, vgl. Q4/38–40). `cfg(feature="cuda")` **nur** in `*_cuda.rs` und an der Factory, die Probe/Measure konstruiert.
      - Kalibrierung und CPU-Kernel laufen in einem **eigenen Thread-Pool** (`std::thread::Builder` + `thread_priority = Min`), **nicht** im tokio-Runtime-Thread (Vorgabe `resident_precision.md`; vgl. Q3/Q4 zum Pool).
      - No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests ausführen; `cuda`-Build mindestens per `cargo check` prüfen.

      ---

      ## 5. Globale Definition of Done (Gruppe F)
      1. `precision`-Modul kompiliert mit **und** ohne `cuda`-Feature.
      2. F1–F5 implementiert, jeweils eigene DoD (siehe Paketpläne) erfüllt.
      3. Vollständige Kalibrier-Pipeline ohne GPU per `FakeMeasure` durchlaufbar: Erhebung → Messung → Pruning → Pareto → Profile → Matching → Persistenz (Round-Trip) → Reload.
      4. Alle Pflicht-Tests aus `resident_precision.md`/`context_f.md` grün, mit ≥ 3 Seeds für die Stabilitätslogik.
      5. Jede `[V]`-abgeleitete Hardware-Konstante (z. B. `dp4a`-Verfügbarkeit, fehlende Tensor-Cores auf TU116) ist als Defaulttabelle hinterlegt und per Kommentar auf die Quelle/offene Verifikation verweisend dokumentiert.
      6. Keine Kopplung an Scheduler/Processor-Durchsatz; keine festen Profilnamen; `precision`-Skala-Semantik (0=fast, 1=precision) in Typ-Doku dokumentiert.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f1_capabilityset.md
    mode: replace
    content: |
      # Umsetzungsplan F1 — CapabilitySet-Erhebung

      > Gruppe F · Paket F1. Quelle: `resident_precision.md` (Abschnitte „Capability-Listen statt Profile“, „Umsetzungsvorgaben“). Kontext: `context/context_f.md` §F1 (Q1–Q5). Querschnitt: `plan_f0_overview.md` §4.
      > **Hängt ab von:** B6 (Typen `CapabilitySet`, `Profile`), D6 (`resident_subengine.md` §4.5 — reale Kernel-/Scoring-Pfade). **Liefert an:** F2.

      ## 1. Zweck & Ergebnis
      Jede Subengine erhebt ihr `CapabilitySet`: **pro Parameterachse die tatsächlich verfügbaren Werte**. Ergebnis ist (a) ein erweiterbarer Achsen-/Wertekatalog, (b) ein `CapabilityProbe`-Trait mit CPU- und CUDA-Implementierung, (c) eine Abgleichregel gegen die D6-Kernel-Registry (nur Werte, für die ein Kernel existiert, werden gelistet).

      ## 2. Scope-Abgrenzung
      - F1 **misst nicht** und bewertet nicht (das ist F2). F1 ist **optimistisch**: es listet, was Hardware + Kernel-Registry anbieten. Unbrauchbare Kombinationen verwirft F2 (NaN/Inf, Maximalabweichung, dominiert).
      - F1 definiert **keine** Profile und keine Reihenfolge.
      - Achsen/Werte müssen **pro Hardware erweiterbar** sein, **ohne** Änderung an Fassade (C1) oder Task-Schnittstelle (harte Vorgabe).

      ## 3. Zieltypen (Skizze; B6 ist formaler Eigentümer)
      ```rust
      #[non_exhaustive]
      pub enum EngineKind { Cpu, Cuda }

      /// Achsen-Identität ist offen (String-Newtype), damit neue Achsen ohne
      /// Enum-Änderung (= ohne API-Bruch) hinzukommen können.
      #[derive(Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
      pub struct AxisId(pub &'static str);

      /// Ein Achsenwert ist getaggt, damit heterogene Achsen (dtype, Flag, Zahl)
      /// in einem Katalog leben. Serde-fähig für F5-Persistenz.
      #[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
      pub enum AxisValue {
          Dtype(DType),       // f32, f16, (bf16), int8 ...  — DType-Enum aus B2
          Accum(DType),       // Akkumulator-dtype
          Flag(bool),         // z. B. Fusion an/aus
          UInt(u32),          // Batch-, Tile-, Thread-Zahl
          Scoring(ScoringPath)// f32-FMA | f16x2-FMA/f32-Akku | int8-dp4a/i32-Akku | cpu-int8 ...
      }

      pub struct CapabilityAxis { pub id: AxisId, pub values: Vec<AxisValue> }

      pub struct CapabilitySet { pub engine: EngineKind, pub axes: Vec<CapabilityAxis> }

      pub trait CapabilityProbe { fn probe(&self) -> Result<CapabilitySet, PrecisionError>; }
      ```
      `ScoringPath` ist ein erweiterbares Enum (`#[non_exhaustive]`), dessen Varianten **1:1 den in D6 registrierten Kernel-Varianten** entsprechen.

      ## 4. Achsen-/Wertekatalog (Default, aus `context_f.md` §F1)
      ### GPU / CUDA (Zielkarte GTX 1660, TU116, sm_75)
      | Achse (`AxisId`) | Default-Werte | Belegt |
      |---|---|---|
      | `"operands"` | `{f32, f16}` | [V] native FP16 (2× FP32-Rate, `half2`) |
      | `"accumulator"` | `{f32, f16}` | [V] |
      | `"scoring"` | `{f32-FMA, f16x2-FMA/f32-Akku, int8-dp4a/i32-Akku}` | [V] `dp4a` ⇔ CC ≥ 6.1, sm_75 erfüllt |
      | `"fusion"` | `{an, aus}` | — |
      | `"batch"` | konfigurierbare Menge (z. B. `{1,8,32,…}`) | — |
      | `"tile"` | konfigurierbare Menge | — |

      **Harte Regeln (sm_75 / TU116):**
      - **Keine Tensor-Cores** → **kein** WMMA/MMA-Matrixpfad in den Katalog aufnehmen. [V: TU116 ist eigenes Die ohne TC]
      - **Tensor-Core-Erkennung NIE nur aus Compute Capability** ableiten: TU116 meldet CC 7.5, hat aber keine TC. Ein naiver `major >= 7`-Check (wie in pandrs/hive-gpu) wäre hier falsch-positiv. [V] → TC-Flag per **Whitelist/Mikrobenchmark**, nicht per CC.
      - **bf16** ist auf sm_75 nicht nativ → als *emuliert/dominiert* behandeln, nicht als eigener Schnellpfad listen. [W]
      - `dp4a`-**Durchsatz** (Rate vs. FP32) auf TU116 ist **unbelegt** → Wert in Katalog aufnehmen, aber dessen Nutzen **F2 messen lassen**, nicht annehmen.

      ### CPU
      | Achse | Default-Werte | Belegt |
      |---|---|---|
      | `"operands"` | `{f32, f16→f32}` | [W] `half`-Crate, F16C auf x86 |
      | `"scoring"` | `{f32, int8}` | [W] AVX2 `_mm256_maddubs_epi16` / VNNI |
      | `"threads"` | `{1 … n_logical}` (sysinfo) | [W] Skalierung bis Speicherbandbreite |
      | `"tile"` | konfigurierbare Menge (L1/L2-Passung) | [W] |

      ## 5. Umsetzungsschritte
      1. **Typen/Trait** in `capability.rs` anlegen (ggf. B6 konsolidieren). `AxisId` als `&'static str`-Newtype, Werte getaggt, serde-fähig (für F5).
      2. **CUDA-Probe** (`capability_cuda.rs`, `cfg(feature="cuda")`):
         - CC lesen: `cudarc::driver::CudaContext::new(ordinal)?.compute_capability() -> (major, minor)` (cudarc ≥ 0.19) [V]. Fallback roh: `result::device::get_attribute(dev, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR/MINOR)` [V].
         - Ableitungen: FP16-schnell ⇔ CC ≥ 5.3; `dp4a` ⇔ CC ≥ 6.1 [V]. TC ⇔ CC ≥ 7.0 **UND** nicht in TU116-Whitelist.
         - Achsen aus einer **`hardware_table`** (CC/Device-Name → erlaubte Werte) erzeugen, danach mit D6-`KernelRegistry` schneiden (nur Werte mit existierendem Kernel).
      3. **CPU-Probe** (`capability_cpu.rs`): `raw-cpuid` (Brand, Flags AVX2/F16C/VNNI), `sysinfo` (logische Kerne). Achsen entsprechend füllen (`int8` nur bei VNNI/AVX2, `f16→f32` nur bei F16C).
      4. **Registry-Abgleich (D6):** Funktion `intersect_with_kernels(set, &KernelRegistry) -> CapabilitySet`. Entfernt Werte, für die kein Kernel registriert ist. Dokumentiert, dass F1 nie Werte liefert, die D6 nicht ausführen kann.
      5. **Konfig-Achsen (batch/tile):** aus Engine-Config (nicht aus Hardware) befüllen; Defaults als Zweierpotenzen; Obergrenze an die konfigurierte Maximalsequenzlänge koppeln (F2 testet Verarbeitbarkeit bis dorthin).
      6. **Factory** `make_probe(engine, cfg) -> Box<dyn CapabilityProbe>` — einzige `cfg`-Verzweigung für CUDA.

      ## 6. Querschnitt
      - **Fehler (Q1):** `PrecisionError::Capability(String)` bei fehlgeschlagener Treiber-/CPUID-Abfrage. Kein Panic bei fehlender GPU — Probe liefert dann nur die CPU-Engine.
      - **Logging (Q2):** `info!` mit `engine`, `device_name`, `cc`, Anzahl Achsen/Werte; `debug!` je Achse die gelisteten Werte und welche per Registry-Schnitt entfielen.
      - **Feature-Flags (Q4):** CUDA-Probe komplett hinter `cuda`; ohne Feature existiert nur die CPU-Probe. `cudarc`-Safe-API liefert **keine** Runtime-/Driver-Version [V] → das ist F5 (NVML/`cuDriverGetVersion`), nicht F1.

      ## 7. Teststrategie (Q3)
      - `FakeCapabilityProbe` mit festgelegtem Katalog (für F2/F3-Tests).
      - CPU-Probe-Test: auf Host mit/ohne AVX2 korrekte Achsenmengen (Feature-Erkennung mocken via injizierbarer `CpuFeatures`-Quelle).
      - Registry-Schnitt-Test: Wert ohne Kernel wird entfernt.
      - Erweiterbarkeits-Test: neue Achse/neuer Wert lässt sich hinzufügen, ohne bestehende Signaturen zu ändern (Kompilationsnachweis).
      - **Kein** echter GPU-Test im Default-Lauf.

      ## 8. Offene Verifikation / Risiken
      - `[prüfen]` `cudarc`-Version & genaue API (`compute_capability()` vs. `result::device::get_attribute`). Quellen: docs.rs prism-q/numr/pandrs/hive-gpu.
      - `[unverifiziert]` `dp4a`-Durchsatz auf TU116, WMMA-Verhalten (AnandTech-Spekulation) → nur messen, nie annehmen.
      - `[W]` candle-dtype-Pfade (f16 cuBLAS-Compute-Typ, bf16-Emulation auf sm_75) vor Nutzung im Katalog gegen `candle-kernels`/`cuda_backend` prüfen.
      - Whitelist der TU116-Dies (fehlende TC) pflegbar halten; alternativ Mikrobenchmark-Hook für TC-Flag.

      ## 9. Definition of Done
      - `CapabilityProbe` für CPU implementiert und getestet; CUDA-Probe hinter `cuda` baubar (`cargo check`).
      - Default-Katalog GPU/CPU gemäß Tabelle, inkl. ausdrücklichem Ausschluss von Tensor-Cores/bf16 auf sm_75 mit Quellenkommentar.
      - Registry-Schnitt gegen D6 implementiert; Achsen/Werte erweiterbar ohne API-Bruch (Test vorhanden).
      - Liefert ein `CapabilitySet`, das F2 direkt aufzählen kann.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f2_calibrator.md
    mode: replace
    content: |
      # Umsetzungsplan F2 — Calibrator (Messung, Pruning, Pareto)

      > Gruppe F · Paket F2. Quelle: `resident_precision.md` (Abschnitte „Calibrator“, „Messmethodik“, „Kombinatorische Explosion begrenzen“). Kontext: `context/context_f.md` §F2 (Q6–Q14). Querschnitt: `plan_f0_overview.md` §4.
      > **Hängt ab von:** F1. **Liefert an:** F3 (Pareto-Front), F5 (zu persistierende Rohergebnisse).

      ## 1. Zweck & Ergebnis
      Der `Calibrator` zählt Kombinationen aus dem `CapabilitySet` auf, misst jede als **Mini-Benchmark** interleaved gegen die fp32-Referenz, verwirft ungültige, bildet die **Pareto-Front (Zeit, Abweichung)** und übergibt sie an F3. Ergebnis: Kalibrierungsablauf-Spezifikation + `Measure`-Trait + reale/mock Messung + Pareto-Auswahl.

      ## 2. Zieltypen (Skizze)
      ```rust
      pub struct Cfg { pub combo: Vec<(AxisId, AxisValue)>, pub seq_lens: Vec<usize> }

      pub struct MeasureResult {
          pub time: std::time::Duration, // robuste Zeit (Median oder Min)
          pub deviation: f64,            // Abweichung vs. fp32-Referenz desselben Testsets
          pub valid: Validity,           // Ok | Rejected(reason)
          pub stable: bool,              // false ⇒ als `unstable` markieren
      }
      pub enum Validity { Ok, NonFinite, Degenerate, OverMaxDeviation }

      pub trait Measure { fn measure(&mut self, cfg: &Cfg) -> Result<MeasureResult, PrecisionError>; }

      pub struct CalibratorConfig {
          pub max_deviation: f64,   // konfigurierbare Maximalabweichung (= inkompatibel)
          pub min_reps: u32,        // ≥ 3
          pub window_ms: f64,       // Zielfensterlänge je Messung (≥ ~1 ms)
          pub cv_threshold: f64,    // Streuungsschwelle (Variationskoeffizient)
          pub hard_cap: HardCap,    // max. Wiederholungen ODER Zeitbudget
          pub slice_budget_ms: f64, // ≤ ~5–10 ms je GPU-Launch (keine Preemption)
      }

      pub struct ParetoPoint { pub cfg: Cfg, pub time: Duration, pub deviation: f64, pub stable: bool }
      ```

      ## 3. Ablauf (verbindlich aus `resident_precision.md`)
      1. **Aufzählen** der Kombinationen aus `CapabilitySet` (kartesisch) **mit Pruning**.
      2. **Messen** jeder Kombination als Mini-Benchmark; **gleichzeitig** Abweichung vs. fp32-Referenz erfassen.
      3. **Verwerfen**: NaN/Inf; degeneriert (konstante Vektoren, Norm außerhalb des plausiblen Bereichs); Abweichung > `max_deviation` (⇒ inkompatibel, **kein** Laufzeitfehler, nur nicht aktiviert).
      4. **Pareto-Front** aus Zeit und Abweichung bilden.
      5. Profile ableiten → **F3**.
      6. Persistieren → **F5** (inkl. `unstable`, Maximalabweichungs-Verwerfungen, Pruning-Ergebnisse).

      ## 4. Kombinatorik begrenzen (verlustfrei bzgl. Pareto)
      - **Separable Achsen getrennt messen** (z. B. Batch/Tile unabhängig von dtype), danach **nur Gewinner kombinieren** — OFAT, aber mit **Verifikationsmessung** der kombinierten Gewinner, weil Wechselwirkungen auftreten (typisch: Tile×dtype, Fusion×Akkumulator). [context §F2/Q12, W]
      - **Dominierte Kombinationen früh abbrechen** (bereits langsamer **und** ungenauer als eine bekannte). Da **Abweichung früh billig** und **Zeit teuer** ist: **Abweichung zuerst** messen, dann bei Dominanz Zeit-Samples sparen. [§F2/Q11]
      - Nur die Pareto-Front wird benötigt ⇒ Pruning ist verlustfrei bzgl. Ergebnis.

      ## 5. Messmethodik
      ### Allgemein (`resident_precision.md`)
      - **≥ 3 Wiederholungen** je Kombination.
      - **Interleaved** messen (nicht A komplett, dann B) → Lastschwankungen treffen alle gleich. Reihenfolgeeffekte per **ABBA**/Randomisierung dämpfen. [§F2/Q8]
      - Nur **Relationen** zählen; Stabilitätsprüfung auf dem **Verhältnis** zwischen Kombinationen, nicht auf Einzelzeiten.
      - Robuste Statistik: **Median oder Minimum** (Last verzerrt nur nach oben; Minimum = Untergrenze der Hardwarezeit). Mittelwert nur bei Gesamtdurchsatz. [§F2/Q9]
      - **Adaptiv** nach Streuung (CV oder `(max−min)/median`): klein ⇒ stabil/fertig; intern verrauscht ⇒ mehr Samples je Wiederholung; Wiederholungen driften, Samples stabil ⇒ ganzen Durchlauf wiederholen.
      - **Abbruch bei Stabilität, nicht nach fester Zeit.** Harte Obergrenze (max. Wiederholungen/Zeitbudget): besten Zwischenstand verwenden, als **`unstable`** loggen+persistieren, **nicht blockieren, nicht abbrechen**. `unstable` darf später (manuell) nachgemessen werden.
      - **Niedrige Thread-Priorität**; Testset mit **Längenvarianz bis zur Maximalsequenzlänge** (prüft Verarbeitbarkeit: Speicher/Overflow), **nicht** Retrieval-Qualität.

      ### GPU-spezifisch (korrektes Benchmarking, context §F2/Q6 [V])
      - Launches sind **asynchron**; ein Host-Timer ohne Sync misst nur die Submit-Zeit. **CUDA-Events auf dem Stream** stempeln und blenden Launch-Overhead aus.
      - Vor Messung `synchronize`; **Warmup** (JIT, Caches, Clock-Ramp); **L2-Flush** zwischen Läufen; für **kurze** Kernel mehrere Launches/CUDA-Graph pro Messfenster und durch Anzahl teilen.
      - Iterationszahl pro Kalibrierlauf auf **Zielfensterlänge** (`window_ms`, z. B. ≥ 1 ms) skalieren.
      - **Clock-Fixierung** (`nvidia-smi -lgc`) braucht Admin und wird von GeForce teils nicht unterstützt [W] → **Fallback: Warmup + Streuungskriterium**.
      - Fallstrick: implizite CUDA-Init verfälscht Host-Timer → vor der Messung einmal initialisieren.

      ### Keine GPU-Preemption (context §F2/Q7 [W])
      - Zeitbudget je Launch (`slice_budget_ms` ≤ ~5–10 ms), kleine Problemgrößen, in **viele kurze Launches** slicen. Zwischen Slices Prüfpunkt (Atomic-Flag/Queue-Länge), ob ein Produktions-Task wartet → ggf. **abbrechen**. Gesamtbudget der Kalibrierung begrenzen.

      ## 6. Verwerfungskriterien (context §F2/Q10 [W])
      - **NonFinite:** `!x.is_finite()` über alle Elemente (GPU-Reduktion).
      - **Degenerate:** Varianz ≈ 0 (konstante Vektoren) oder L2-Norm außerhalb `[ε, K]`.
      - **Abweichungsmetrik:** Embeddings `1 − cos(a, ref)`, zusätzlich relativer Fehler `‖a−ref‖/‖ref‖`; Scoring zusätzlich **Rang-Metrik** (Top-k-Overlap / Kendall τ).
      - **Richtwerte (projektspezifisch festlegen):** fp16 ~1e-3…1e-2 Cosinus-Abweichung, int8 ~1e-2. In `CalibratorConfig.max_deviation` konfigurierbar.

      ## 7. Pareto-Auswahl
      - 2D-Algorithmus: nach Zeit sortieren, linear durchlaufen, Punkt behalten, wenn Abweichung < bisheriges Minimum → `O(n log n)`. [§F2/Q11]
      - Ergebnis: `Vec<ParetoPoint>` (sortiert), mit `stable`-Flag je Punkt.

      ## 8. Thread-Pool & Priorität (context §F2/Q13 [W])
      - Eigener Pool via `std::thread::Builder`, `thread_priority::set_current_thread_priority(ThreadPriority::Min)` beim Start; **getrennt** von der tokio-Runtime (Vorgabe `resident_precision.md`).
      - Hinweis: CPU-Priorität beeinflusst **GPU-Konkurrenz nicht** → deshalb zusätzlich die Slice-/Abbruch-Logik aus §5 (keine Preemption).

      ## 9. Querschnitt
      - **Fehler (Q1):** `Incompatible { reason }` wird **nicht** geworfen — Verwerfung ist ein normaler Ausgang (`Validity`). `CalibrationUnstable` nur als Markierung/Log, nie als Betriebsabbruch.
      - **Logging (Q2):** Span `calibrate{engine, model_id}`; je Kombination `debug!(cfg, time, deviation, stable)`; verworfene mit Grund (`info!`); am Ende Pareto-Größe. CUDA-Callbacks: Daten per Channel, **nicht** im Callback loggen.
      - **Feature-Flags (Q4):** reale Messung (`measure_cuda.rs`) hinter `cuda`; Default-Tests nutzen `FakeMeasure`.

      ## 10. Teststrategie (Q3, `resident_precision.md` „Tests“)
      - **`FakeMeasure`** hinter `Measure`: Basiswert + Rauschen aus **seeded RNG** (`StdRng::seed_from_u64`), Skripte: **stabil / Ausreißer / driftend / bimodal / nie stabil**; injizierter `Clock`-Trait.
      - **Stabilitätslogik:** nie-stabil ⇒ Obergrenze erreicht ⇒ `unstable` + bester Zwischenstand (mehrere Seeds). Konvergenz bei geringem Rauschen. Ausreißer werden von Median/Min absorbiert.
      - **Pareto:** konstruierte Punkte, dominierte werden verworfen; Monotonie der Front.
      - **Verwerfung:** NaN/Inf, degeneriert, > `max_deviation`.
      - **Pruning:** separable Achsen getrennt + Gewinner-Verifikation führt zum selben Front-Ergebnis wie Vollenumeration (kleiner Fall).
      - **Kein** GPU nötig.

      ## 11. Offene Verifikation / Risiken
      - `[V, aber projektspezifisch]` Abweichungsschwellen (fp16/int8) müssen gegen reale e5-small-Embeddings festgelegt werden (Verweis G1).
      - `[W]` Clock-Locking auf GeForce nicht verlässlich → Fallback-Streuungskriterium ist Pflichtpfad.
      - `[W]` Separabilitätsannahme (OFAT): Wechselwirkungen Tile×dtype / Fusion×Akku → Verifikationsmessung der Gewinner nicht weglassen.
      - `cudarc` Event-/Graph-API & `cuEventQuery` vor realer Messung prüfen (vgl. `context_h_q.md`).

      ## 12. Definition of Done
      - `Measure`-Trait + `FakeMeasure` + reale CUDA-Messung (hinter `cuda`) implementiert.
      - Ablauf 1–4 + Pruning + Pareto umgesetzt; `unstable`-Pfad ohne Blockieren.
      - Slice-/Abbruch-Logik (keine Preemption) und eigener Thread-Pool mit niedriger Priorität.
      - Pflicht-Tests (Stabilität, Pareto, Verwerfung, Pruning) grün mit ≥ 3 Seeds.
      - Liefert `Vec<ParetoPoint>` an F3 und Rohergebnisse (inkl. Verwerfungen/Pruning/`unstable`) an F5.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f3_profilbildung.md
    mode: replace
    content: |
      # Umsetzungsplan F3 — Profilbildung

      > Gruppe F · Paket F3. Quelle: `resident_precision.md` (Abschnitt „Profilbildung“). Kontext: `context/context_f.md` §F3 (Q15–Q18). Querschnitt: `plan_f0_overview.md` §4.
      > **Hängt ab von:** F2 (Pareto-Front). **Liefert an:** F4 (Profile zum Matchen), F5 (zu persistierende Profile).

      ## 1. Zweck & Ergebnis
      Aus der Pareto-Front leitet F3 **unbenannte Profile** ab: `Profile { position: f32 ∈ [0,1], config }`. Die Anzahl ist variabel (1…n) und ergibt sich **allein aus dem Spread** der Front. Der **Spread-Schwellwert in normierter Skala** ist die **einzige** Konfiguration. Ergebnis: Funktion `pareto_front → Vec<Profile>` + Positionsberechnung über Bogenlänge.

      ## 2. Zieltypen (Skizze; B6 ist formaler Eigentümer von `Profile`)
      ```rust
      pub struct Profile {
          pub position: f32,   // p ∈ [0,1], 0 = schnellstes, 1 = genauestes
          pub config: Cfg,     // die zugehörige Kombination (aus F1/F2)
      }

      pub struct ProfileConfig { pub spread_threshold: f64 } // EINZIGER Parameter, normierte Skala

      pub fn derive_profiles(front: &[ParetoPoint], cfg: &ProfileConfig) -> Vec<Profile>;
      ```

      ## 3. Verfahren (verbindlich aus `resident_precision.md`)
      1. **Normieren:** Zeit und Abweichung jeweils auf `[0,1]`. Zeit **logarithmisch** (`log(t)`), da Größenordnungsspannen. [context §F3/Q15]
         - Fallstricke absichern: Division durch 0 bei **Einpunkt-Front** (Konvention, siehe 4); Ausreißer dominieren Min/Max; Skalenwahl ändert Abstände → Skala fix dokumentieren.
      2. **Bogenlänge** entlang der Front: `s_i = Σ_{j≤i} ‖P_j − P_{j−1}‖` (euklidisch im normierten Raum), `p_i = s_i / s_n`. **Robuster als Rang**, weil echte Tradeoff-Sprünge erhalten bleiben (Rang verzerrt Abstände). [§F3/Q16]
      3. **Redundanz zusammenfassen:** 1D-**Single-Linkage entlang `p`** — Nachbarn mit `Δp < spread_threshold` verschmelzen. Repräsentant = schnellster (oder mittlerer) Punkt des Clusters. **Schwelle ist der einzige Parameter.** [§F3/Q17]
      4. **Positionen setzen:** schnellstes Profil `p = 0`, genauestes `p = 1`, übrige proportional zur kumulierten Distanz. Bei genau **einem** Profil gilt `p = 0.5` per Konvention.
      5. **Redundante Profile nicht aktivieren:** praktisch identische Zeit **und** identisches Ergebnis (z. B. derselbe Emulationspfad) → nicht aktivieren.
      6. **Ordnung/Herabstufung:** Profil-Ordnung ergibt sich aus der Messung. Der **umgekehrte Fall** (reduzierte Genauigkeit, die durch Emulation **langsamer** ist) wird erkannt und **herabgestuft oder nicht aktiviert**.

      ## 4. Emulations-/Dominanzerkennung (context §F3/Q18 [W])
      - Eine Kombination ist **dominiert**, wenn sie **langsamer und nicht genauer** als der fp32-Referenzpfad ist → aus der Front entfernen.
      - Zusatz-Plausibilität: „**Zeit ≥ Zeit des höherpräzisen Pfads**“ bei geringerer Präzision ⇒ Emulation ⇒ herabstufen/nicht aktivieren.
      - (Die eigentliche Pareto-Dominanz filtert F2 bereits; F3 wendet die Regel nochmals nach der Zusammenfassung an, um redundante Emulationspunkte sicher auszuschließen.)

      ## 5. Normierungs-Robustheit (Pflicht-Sonderfälle)
      - **Einpunkt-Front:** direkt ein Profil mit `p = 0.5`, keine Normierung nötig.
      - **Zwei identische Koordinaten:** Spannweite 0 in einer Achse → diese Achse trägt 0 zur Bogenlänge bei (keine Division durch 0 erzwingen; Guard einbauen).
      - Normierte Skala ist **modell- und hardwareunabhängig** → `spread_threshold` gilt engineübergreifend gleich, obwohl Profile pro Engine gebildet werden.

      ## 6. Sperre ohne aktives Profil
      - Bleibt nach Bildung/Aktivierung **kein** Profil übrig (alles verworfen/inaktiv), ist die Engine **für dieses Modell gesperrt** (`NoActiveProfile` tritt dann lazy im Matching/ersten Task auf, siehe F4).

      ## 7. Querschnitt
      - **Fehler (Q1):** F3 selbst wirft i. d. R. nicht; eine leere aktive Profilmenge wird als Zustand an F4/F5 weitergegeben (→ `NoActiveProfile` dort).
      - **Logging (Q2):** `info!` mit Anzahl abgeleiteter Profile und deren Positionen; `debug!` je Cluster (verschmolzene Punkte, Repräsentant, Grund für Nicht-Aktivierung).
      - **Feature-Flags (Q4):** rein rechnerisch, **GPU-frei**, kein `cfg`.

      ## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
      - **Spread-Schwellwert:** steuert Profilanzahl (kleine Schwelle → mehr Profile; große → Zusammenfassung).
      - **Zusammenfassen redundanter Punkte:** nahe Punkte (Δp < Schwelle) ergeben ein Profil; Repräsentant korrekt gewählt.
      - **Positionsberechnung über Bogenlänge:** konstruierte Front mit ungleichen Abständen → Positionen folgen kumulierter Distanz, nicht dem Rang.
      - **Einzelprofil:** `p = 0.5`.
      - **Emulation/Herabstufung:** langsamer-und-ungenauer-Punkt wird entfernt/nicht aktiviert.
      - Alle Tests **deterministisch**, ohne GPU.

      ## 9. Offene Verifikation / Risiken
      - `[W]` gesamter Abschnitt F3 ist Fachwissen ohne frische Quellenprüfung → Normierungs-/Bogenlängen-Formeln und die Single-Linkage-Schwelle an echten Fronten (aus F2-Läufen auf e5-small) validieren.
      - Wahl „log(Zeit)“ vs. lineare Zeit beeinflusst Abstände/Positionen → fixieren und dokumentieren.
      - Repräsentantenwahl (schnellster vs. mittlerer) konsistent festlegen (empfohlen: schnellster, damit `p`-Richtung monoton bleibt).

      ## 10. Definition of Done
      - `derive_profiles` implementiert: Normierung (log-Zeit) → Bogenlänge → Linkage → Positionen, inkl. Sonderfälle (Einpunkt, identische Koordinaten).
      - `spread_threshold` ist der einzige Konfigparameter, in normierter Skala.
      - Emulations-/Herabstufungsregel umgesetzt; redundante Profile nicht aktiviert.
      - Pflicht-Tests grün. Liefert `Vec<Profile>` an F4 und F5.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f4_task_matching.md
    mode: replace
    content: |
      # Umsetzungsplan F4 — Task-Matching (`precision → Profil`)

      > Gruppe F · Paket F4. Quelle: `resident_precision.md` (Abschnitt „Task-Parameter und Matching“). Kontext: `context/context_f.md` §F4 (Q19–Q21). Querschnitt: `plan_f0_overview.md` §4.
      > **Hängt ab von:** F3 (Profile), B6 (Typen), C1 (Fassade/`TaskParams`). **Liefert an:** Subengine-Pull (D1) / Scorer — welches Profil ein Task bekommt.

      ## 1. Zweck & Ergebnis
      F4 bildet den **Wunsch** `TaskParams.precision: f32` (Skala **0 = fast, 1 = precision**) auf das **nächstliegende aktive Profil** der jeweiligen Subengine ab. Ergebnis: eine deterministische Matching-Funktion + Klemm-/NaN-Regeln + Tie-Break.

      ## 2. Verbindliche Semantik (`resident_precision.md`)
      - `TaskParams.precision: f32`, Skala **0 = fast, 1 = precision**. Orientierung **muss** in Doku **und** Typ-Dokumentation ausdrücklich stehen.
      - Werte außerhalb `[0,1]` werden **geklemmt**; **`NaN` ist ein Fehler** (`InvalidPrecision`).
      - Matching: Profil mit **minimalem `|t − p|`**. Bei Gleichstand gewinnt **deterministisch das genauere** Profil (höheres `p`).
      - Jede Subengine matcht gegen ihre **eigenen** Profile; die Skala ist **relativ zur Engine** (`t = 0.5` ist auf CPU und GPU nicht dasselbe Genauigkeitsniveau).
      - Ein Wunsch ist **immer erfüllbar** (es gibt immer ein nächstliegendes aktives Profil). **Kein** Fehler und **kein** Warten wegen nicht verfügbarem Profil — **außer** die Engine hat für dieses Modell gar kein aktives Profil (dann `NoActiveProfile`, lazy; siehe §5).

      ## 3. Zielsignatur (Skizze)
      ```rust
      /// Gibt den Index/das Profil mit minimalem |t - p| zurück.
      /// Fehler nur bei NaN (InvalidPrecision) oder leerer aktiver Profilmenge (NoActiveProfile).
      pub fn match_profile<'a>(
          precision: f32,
          profiles: &'a [Profile],   // aktive Profile EINER Engine für EIN Modell
          model_id: &str,
      ) -> Result<&'a Profile, PrecisionError>;
      ```

      ## 4. Implementierungsregeln (context §F4/Q20–Q21 [W])
      1. **NaN zuerst abfangen:** `if precision.is_nan() { return Err(InvalidPrecision) }` — **vor** `clamp`, denn `f32::clamp` gibt bei NaN NaN zurück (Rust-Verhalten vor Implementierung gegen Doku **verifizieren**). `±∞` wird von `clamp` auf die Grenzen `[0,1]` geklemmt.
      2. **Klemmen:** `let t = precision.clamp(0.0, 1.0);`.
      3. **Leere aktive Profilmenge:** `Err(NoActiveProfile { model_id })`.
      4. **Nächstes Profil:** `min_by` über `(t − p).abs()`, Vergleich mit `total_cmp`. **Distanz in f64 rechnen**, um f32-Rundungs-Gleichstände zu vermeiden.
      5. **Tie-Break zum genaueren:** `d.total_cmp(&d2).then(p2.total_cmp(&p1))` (bei gleicher Distanz gewinnt höheres `p`).

      ```rust
      let t = precision as f64; // bereits geklemmt
      profiles.iter()
          .min_by(|a, b| {
              let da = (t - a.position as f64).abs();
              let db = (t - b.position as f64).abs();
              da.total_cmp(&db).then(b.position.total_cmp(&a.position))
          })
          .ok_or_else(|| PrecisionError::NoActiveProfile { model_id: model_id.into() })
      ```

      ## 5. Lazy-Sperre (aus `resident_precision.md` „Laufzeit-Fehlerbehandlung“)
      - Ein inkompatibles/gesperrtes Modell fällt **erst beim ersten Task** auf. Der Fehler (`NoActiveProfile`) geht **sauber an den wartenden Task** zurück; die Subengine wird für dieses Modell **markiert**, damit Folgetasks die Prüfung nicht wiederholen.
      - **Kein** Profil-Prädikat im Pull/Claim und **keine** Konsistenzbedingung zwischen Tasks (Vorgabe „Pull-Mechanismus“).

      ## 6. Einordnung zum Task-Parameterobjekt (C1)
      - `precision: f32` ist ein Feld des erweiterbaren `TaskParams`-Objekts der Fassade (C1). F4 liefert nur die Matching-Funktion; die Fassade/Planner reicht den Wert durch. **Keine** Zusicherung nach außen (Wunsch-Semantik).
      - Analogien zur Orientierung (keine 1:1-Vorlage): HNSW `ef_search` / IVF `nprobe`, Faiss-Autotuning, TensorRT-Präzisionsmodi, llama.cpp-Quantisierungsstufen. [context §F4/Q19]

      ## 7. Querschnitt
      - **Fehler (Q1):** `InvalidPrecision` (NaN), `NoActiveProfile { model_id }` (keine aktiven Profile). Sonst nie Fehler/Warten.
      - **Logging (Q2):** **gewähltes Profil pro Task auf `debug`** (Vorgabe): `debug!(task_id, t, chosen_p = profile.position)`.
      - **Feature-Flags (Q4):** rein rechnerisch, GPU-frei.

      ## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
      - **Klemmen:** `-0.3 → 0.0`, `1.7 → 1.0`, `+∞ → 1.0`, `−∞ → 0.0`.
      - **NaN-Fehler:** `NaN → Err(InvalidPrecision)` (vor Klemmen).
      - **Tie-Break zum genaueren:** zwei Profile gleich weit von `t` → höheres `p` gewinnt, deterministisch (mehrfach ausgeführt gleich).
      - **Einzelprofil:** jedes `t` matcht das einzige Profil (`p = 0.5`).
      - **Leere Profilmenge:** `Err(NoActiveProfile)`.
      - **f64-Distanz:** konstruierte Fälle, in denen f32-Distanz einen Gleichstand erzeugen würde, f64 aber nicht (und umgekehrt der Tie-Break greift).

      ## 9. Offene Verifikation / Risiken
      - `[prüfen]` `f32::clamp`-Verhalten bei NaN gegen aktuelle Rust-Doku (Kontext markiert dies ausdrücklich als zu verifizieren).
      - `[W]` gesamter F4-Abschnitt ist Fachwissen → Tie-Break-Determinismus mit mehreren gleichweit entfernten Profilen testen.

      ## 10. Definition of Done
      - `match_profile` implementiert mit NaN-Fehler-vor-Klemmen, f64-Distanz, Tie-Break zum genaueren.
      - `precision`-Skala-Semantik (0=fast, 1=precision) in Typ-Doku dokumentiert.
      - Lazy-`NoActiveProfile`-Pfad vorbereitet (Markierung des gesperrten Modells an der Subengine).
      - Pflicht-Tests grün.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f5_calibration_store.md
    mode: replace
    content: |
      # Umsetzungsplan F5 — CalibrationStore / Persistenz

      > Gruppe F · Paket F5. Quelle: `resident_precision.md` (Abschnitt „Persistenz (CalibrationStore)“, „Laufzeit-Fehlerbehandlung“). Kontext: `context/context_f.md` §F5 (Q22–Q25). Querschnitt: `plan_f0_overview.md` §4.
      > **Hängt ab von:** F3 (Profile), F1 (CapabilitySet für Rohdaten). **Liefert an:** Subengine-Init (lädt Profile beim ersten Task-Bedarf).

      ## 1. Zweck & Ergebnis
      Der `CalibrationStore` persistiert das Kalibrierergebnis **pro `Modell × Subengine`** auf derselben Maschine (Datei), erkennt über einen **Fingerprint** die Zustände `Calibrated | Stale | Missing` und steuert das Verhalten (laden / im Hintergrund neu kalibrieren / konservativer Default). Ergebnis: Persistenz-Spezifikation + Store-API + Fingerprint + Serde-Schema.

      ## 2. Verbindliche Vorgaben (`resident_precision.md`)
      - Kalibrierung läuft **initial einmal** und wird persistiert; **Erstkalibrierung darf lang sein**.
      - Gilt **pro Modell × Subengine** (Verhalten hängt von Dimension und Sequenzlänge ab).
      - **Fingerprint:** GPU-Modell, Treiber-/CUDA-Version, CPU-Modell, Modell-ID, Engine-Version.
      - **Zustände:** `Calibrated` (laden/verwenden), `Stale` (Fingerprint abweichend: mit altem Ergebnis starten), `Missing` (konservativer Default = genaueste gültige Kombination + Warnung, **oder** explizites `calibrate`-Kommando).
      - **Persistiert werden auch:** `unstable`-Markierung, Maximalabweichungs-**Verwerfungen** und **Pruning-Ergebnisse** (für Diagnose und Nachmessung).

      ## 3. Zieltypen (Skizze)
      ```rust
      pub enum StoreState { Calibrated, Stale, Missing }

      pub struct Fingerprint {
          pub schema_version: u32,     // Klartext, NICHT nur im Hash
          pub gpu_name: Option<String>,
          pub gpu_cc: Option<(u32,u32)>,
          pub gpu_vram: Option<u64>,
          pub driver_version: Option<String>,
          pub cuda_driver_version: Option<String>,
          pub cuda_runtime_version: Option<String>,
          pub cpu_brand: String,
          pub cpu_features: Vec<String>,  // AVX2/F16C/VNNI ...
          pub cpu_threads: usize,
          pub model_id: String,           // + Hash
          pub engine_version: String,
      }

      pub struct CalibrationRecord {
          pub fingerprint: Fingerprint,
          pub profiles: Vec<Profile>,          // aus F3
          pub unstable: bool,                  // F2-Markierung
          #[serde(default)] pub rejections: Vec<RejectedCombo>,  // Maximalabweichung/degeneriert
          #[serde(default)] pub pruning: Vec<PrunedCombo>,       // Diagnose
      }

      pub trait CalibrationStore {
          fn load(&self, model_id: &str, engine: EngineKind) -> std::io::Result<Option<CalibrationRecord>>;
          fn state(&self, rec: &Option<CalibrationRecord>, current: &Fingerprint) -> StoreState;
          fn save_atomic(&self, rec: &CalibrationRecord) -> std::io::Result<()>; // Temp + rename
      }
      ```

      ## 4. Fingerprint-Erhebung (context §F5/Q22–Q23)
      - **GPU/Treiber/CUDA:** NVML via `nvml-wrapper` (`device.name()`, `sys_driver_version()`, `sys_cuda_driver_version()`); **CC** via `cudarc` (F1). Fallback bei fehlendem NVML (z. B. Windows ohne DLL): `cudarc::driver::sys::cuDriverGetVersion`.
      - **CPU:** `raw-cpuid` (Brand-String, Feature-Flags AVX2/F16C/VNNI), `sysinfo` (Kernzahl).
      - **Schlüssel** = Hash (empfohlen **blake3**) über **kanonisch serialisierte** Felder, **plus `schema_version` im Klartext** (nicht nur im Hash). Zustände: `Calibrated` (Hash gleich), `Stale` (Hash weicht ab), `Missing` (keine Datei). [§F5/Q23]

      ## 5. Serde-Format (context §F5/Q24 [W])
      - **JSON** (debugbar) mit `schema_version`; `#[serde(default)]` für neue Felder (vorwärtskompatibel), **unbekannte Felder tolerieren**. (`postcard`/`bincode` wären kompakter, aber nicht selbstbeschreibend → verworfen für die Diagnosefähigkeit.)
      - `unstable`, `rejections` (Maximalabweichungs-Verwerfungen) und `pruning` als **explizite Felder** (Pflicht für Diagnose/Nachmessung).
      - Ablageort: pro Maschine, Dateiname/Pfad enthält `model_id` + `engine` + `schema_version`.

      ## 6. Zustandsverhalten (context §F5/Q25 + `resident_precision.md`)
      - **`Calibrated`:** laden und verwenden.
      - **`Stale`:** alte Ergebnisse **sofort nutzen**, im **Hintergrund** neu kalibrieren, dann **atomar ersetzen** (Temp-Datei + `rename`). Nie den Nutzer blockieren.
      - **`Missing`:** **konservativer Default** = genaueste gültige Kombination (fp32-Referenz), **Warnung loggen**, Kalibrierung im Hintergrund anstoßen; **manuelle** Nachkalibrierung via CLI/API-Flag (`calibrate`).
      - Hintergrund-Kalibrierung läuft im **eigenen Thread-Pool** mit niedriger Priorität (F2), **nicht** im tokio-Runtime-Thread; sie darf den Betrieb nie blockieren.

      ## 7. Querschnitt
      - **Fehler (Q1):** `PrecisionError::Store(std::io::Error)` für IO/Serde; `Stale`/`Missing` sind **Zustände**, keine Fehler. `NoActiveProfile` entsteht nicht hier, sondern im Matching (F4), wenn ein geladenes/Default-Record keine aktiven Profile hat.
      - **Logging (Q2):** `warn!` bei `Missing` (konservativer Default aktiv); `info!` bei `Stale` (Hintergrund-Rekalibrierung gestartet) und beim atomaren Ersetzen; Fingerprint-Hash + `schema_version` im Log.
      - **Feature-Flags (Q4):** NVML/`cudarc`-Felder des Fingerprints hinter `cuda` optional; ohne GPU bleiben die GPU-Felder `None`, der CPU-Teil bleibt gültig. Store-Logik selbst ist GPU-frei testbar.

      ## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
      - **Fingerprint-Wechsel → `Stale`:** abweichendes Feld (z. B. Treiberversion) ⇒ Hash weicht ab ⇒ `Stale`; altes Record bleibt nutzbar.
      - **Fehlende Datei → `Missing`:** `load` liefert `None` ⇒ `state == Missing` ⇒ konservativer Default + Warnung.
      - **Round-Trip:** `save_atomic` → `load` liefert identisches Record (inkl. `unstable`, `rejections`, `pruning`).
      - **Vorwärtskompatibilität:** JSON mit zusätzlichem unbekanntem Feld lädt ohne Fehler (`#[serde(default)]`, unbekannte toleriert).
      - **Atomarität:** `save_atomic` schreibt Temp + `rename`; ein abgebrochener Schreibvorgang lässt das alte Record intakt (simulierbar).
      - **`unstable`-Persistenz:** ein in F2 als `unstable` markiertes Ergebnis wird gespeichert und wieder geladen.
      - Alle Tests ohne GPU (GPU-Felder `None`).

      ## 9. Offene Verifikation / Risiken
      - `[prüfen]` `nvml-wrapper`-API (`sys_driver_version`, `sys_cuda_driver_version`) und Verfügbarkeit (Windows ohne DLL → Fallback `cuDriverGetVersion`).
      - `[W]` gesamter F5-Abschnitt ist Fachwissen → Fingerprint-Feldauswahl/Serde-Schema vor Festschreibung bestätigen.
      - Kanonische Serialisierung für den Hash festlegen (Feldreihenfolge stabil!), sonst instabile Hashes.
      - Dateirechte/Ablageort pro OS; `rename` muss auf dem Zielvolume atomar sein (gleiches Verzeichnis für Temp-Datei).

      ## 10. Definition of Done
      - `CalibrationStore` (load/state/save_atomic), `Fingerprint`, Serde-Schema (JSON + `schema_version` + `#[serde(default)]`) implementiert.
      - Fingerprint-Erhebung mit NVML/`cudarc`/`raw-cpuid`/`sysinfo` + NVML-Fallback; GPU-Felder optional hinter `cuda`.
      - Zustandsverhalten `Calibrated/Stale/Missing` inkl. atomarem Ersetzen und Hintergrund-Rekalibrierung ohne Blockieren.
      - `unstable`/`rejections`/`pruning` persistiert. Pflicht-Tests grün.
  reason: "Creating the Group F implementation plans (overview + F1–F5) grounded in resident_precision.md, context_f.md and the cross-cutting Q context, so each agent has a self-contained, actionable work package."
```
/answer 64b3-6 allow
Result Stats: id=2fd17d89-cf5c-4d1a-ac56-0cbcdf46003b, total: 152726, in: 123, out: 35004, reason: 7895, read: 29252, write: 117599
ReasoningToken: 7877
ReasoningToken: 18
Die Umsetzungspläne für Gruppe F (Präzision & Kalibrierung) sind erstellt. Ich habe aus `resident_planplan.md` (Gruppe F = F1–F5) die Pakete entnommen und sie mit der Fachquelle `resident_precision.md`, der Recherche `context/context_f.md` und dem Querschnitt `context/context_h_q.md` (Q1–Q4) zu agentenfertigen Plänen ausgearbeitet.

## Erstellte Dateien in `.../resident/plan/`

| Datei | Inhalt |
|---|---|
| `plan_f0_overview.md` | Gruppenübersicht: Scope/Leitprinzipien, Modul-/Typschnitt (`precision`-Modul), gemeinsame Traits (`Measure`, `CapabilityProbe`), Querschnitt Q1–Q4, Sequenz `F1→F2→F3→{F4,F5}`, globale Definition of Done |
| `plan_f1_capabilityset.md` | `CapabilitySet`-Erhebung: erweiterbarer Achsen-/Wertekatalog (GPU/CPU), `CapabilityProbe` via cudarc/raw-cpuid, Abgleich gegen D6-Kernel-Registry |
| `plan_f2_calibrator.md` | Aufzählung/Pruning, interleaved Mini-Benchmarks, GPU-Event-Messung, Verwerfungskriterien, Pareto-Front, `unstable`-Pfad, eigener Thread-Pool |
| `plan_f3_profilbildung.md` | Normierung (log-Zeit) → Bogenlänge → Single-Linkage, Positionen `p∈[0,1]`, Emulations-/Herabstufungsregel, Spread-Schwellwert als einziger Parameter |
| `plan_f4_task_matching.md` | `precision→Profil`: NaN-Fehler vor Klemmen, f64-Distanz, Tie-Break zum genaueren, Lazy-`NoActiveProfile` |
| `plan_f5_calibration_store.md` | Fingerprint (NVML/cudarc/CPUID), Zustände `Calibrated/Stale/Missing`, atomares Ersetzen, Serde-JSON-Schema, Persistenz inkl. `unstable`/Verwerfungen/Pruning |

## Durchgängige Prinzipien in allen Plänen
- **Quellenlage geklärt:** `resident_index.md` wurde als Vektorindex-Doku (Gruppe E) behandelt, **nicht** als Index-/Inhaltsdatei. Gruppe-F-Fachvorgaben stammen aus `resident_precision.md`; D6 (`resident_subengine.md` §4.5) liefert die Kernel-/Scoring-Pfade für F1.
- **Abhängigkeiten abgebildet:** B6 (Typen), D6 (Kernel), C1 (Fassade/`TaskParams`) sind je Paket als Vorbedingung vermerkt.
- **Belegstatus übernommen:** `[V]`-belegte Fakten (z. B. TU116/sm_75 ohne Tensor-Cores, `dp4a` ab CC 6.1, CUDA-Event-Benchmarking) als Defaults hinterlegt; alle `[W]`/`[prüfen]`-Punkte explizit unter *Offene Verifikation* geführt, bevor Konstanten/APIs festgeschrieben werden.
- **Querschnitt Q** (thiserror-Fehler, tracing-Logging, Mock-/GPU-freie Tests, `cuda`-Feature-Flag, eigener Thread-Pool) je Paket konkretisiert, inkl. der Pflicht-Testmatrizen aus `resident_precision.md`.

Empfohlene Bearbeitungsreihenfolge für die ausführenden Agenten: **F1 → F2 → F3 → {F4, F5}** (F4 und F5 nach F3 parallelisierbar).
