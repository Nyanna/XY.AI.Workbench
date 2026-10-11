# Umsetzungsplan Gruppe F — Präzision & Kalibrierung (Übersicht)

> Stufe-2-Umsetzungsplan zum Metaplan `resident_planplan.md`, Gruppe F.
> Primärquelle: `resident_precision.md`. Recherche-/Kontextquelle: `context/context_f.md`.
> Querschnitt (Fehler/Logging/Tests/Feature-Flags): `context/context_h_q.md` (Q1–Q4).

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

Hier -> `/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_00_overview.md`

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
