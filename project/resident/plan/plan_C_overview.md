# Umsetzungspläne Gruppe C — Orchestrierung (Übersicht)

Stand: 10.10.2026 · Zielsprache Rust · GPU über `candle` + `cudarc` · Projekt `/home/user/xyan/xy.ai.workbench/rag`

Diese Übersicht bündelt die Stufe‑2‑Umsetzungspläne der Gruppe C aus `resident_planplan.md`.
Grundlage ist die Recherche in `context/context_c.md` (Belegstufen **[B]** belegt · **[D]** Drittquelle · **[W]** Standardwissen/unverifiziert).
Primärquellen: `resident.md` (Fassade, Kontrollfluss, Queue-Logik), `resident_processor.md`, `resident_planner.md`.

## Pakete und Dateien

| Paket | Rolle | Datei | Hängt ab von |
|---|---|---|---|
| **C1** | Fassade „Resident" — Einstiegspunkt, Config/Init, Task-Einreihung, Index-Verwaltung, Ergebnisabholung | `plan_C1_fassade.md` | B1 (Queue/Task), B3 (Op/Value), B4 (Index-Trait) |
| **C2** | Processor — Queues, Worker-Parkplatz, Worker-Loop, GPU-Completion, Leases, Backpressure, Eviction | `plan_C2_processor.md` | B1, B3, C3 |
| **C3** | Planner — zustandslose Task→Operation-Expansion, Device-Wahl, `materialize` | `plan_C3_planner.md` | B3 (Op/Value), B6 (CapabilitySet/Profile) |

## Umsetzungsreihenfolge (laut Metaplan)

```
C1  →  C3  →  C2
```

Begründung: Die Fassade (C1) definiert die öffentliche Task-API und die Datentypen, die der Planner konsumiert.
Der Planner (C3) ist eine reine Funktion ohne Laufzeitabhängigkeiten und lässt sich isoliert bauen/testen.
Der Processor (C2) orchestriert beide und braucht Planner-Ausgabe + Fassaden-Queue, kommt daher zuletzt.

## Komponentenbeziehung (Soll-Architektur)

```
            ┌──────────────────────────── Resident (C1, Fassade) ───────────────────────────┐
            │  Config → init() baut Processor + spawnt Worker-std::threads                   │
            │  submit(task) → push in Task-Queue (B1)      add_index(Arc<dyn Index>) (B4)     │
            │  TaskHandle (IntoFuture / wait_timeout / Drop=Cancel)                           │
            └───────────────┬───────────────────────────────────────────────┬────────────────┘
                            │ Task-Queue (geteilt)                           │ Residency-Tabelle
                            ▼                                                 ▼
            ┌──────────── Processor (C2) ───────────┐              Index-ID → Zustand/Leases
            │ Task-Queue + 3 Op-Queues (CPU/GPU/sh) │
            │ Worker-Parkplatz, Notify, Starved-Hints│  ── expand(task, &ExpandContext) ──▶  Planner (C3)
            │ Worker-Loop (GPU-Vorrang), Leases      │  ◀── Vec<Operation> (requires/materialize) ──
            │ GPU-Completion (Callback + cuEventQuery)│
            └────────────────────────────────────────┘
```

- Fassade und Processor teilen sich **eine** Task-Queue (Datentyp aus B1). Die Fassade reiht ein, Subengine-Worker ziehen per `claim`.
- Der Processor ruft den Planner synchron und zustandslos für die Expansion (C2 → C3).
- Subengines (Gruppe D) sind hier **nicht** Teil der Pläne; C2 definiert nur die Schnittstellen, über die Subengines Operationen claimen und Ergebnisse zurückmelden.

## Querschnitt (gilt für C1–C3)

- **Q1 Fehler (`thiserror` 2.0.21):** ein Fehler-Enum je Modul (`ConfigError`, `PlannerError`, `OrchestrationError`, `CapabilityError`), oben via `#[from]` zusammengeführt, öffentliche Enums `#[non_exhaustive]`, `Cancelled` als eigene Variante. Fan-out an mehrere Warter: `Arc<TaskError>` oder `Clone`. [B]
- **Q2 Logging (`tracing` 0.1.44):** ein Span pro Task-ID (bei `submit` erzeugt, im Task-Zustand gehalten), ein Span pro Worker-Thread-Lebenszeit (nicht pro Zyklus); Hot-Path (`claim`/`launch`/`complete`) nur Events auf `trace`/`debug`. `Instrument` statt `Span::enter()` über `.await`. Bibliothek ruft **nie** `set_global_default`. [B/W]
- **Q3 Tests/Mocks:** No-GPU-Pfad immer lauffähig; `proptest`/`proptest-state-machine` für Invarianten und Lebenszyklen; Kernel-/Mess-/Completion-Mocks. [B/W]
- **Q4 Feature-Flag `cuda`:** reale CUDA-Pfade hinter `#[cfg(feature = "cuda")]`, orchestrierungsseitige Logik ist ohne GPU übersetzbar und testbar. Trait `Completion` immer kompiliert, `ManualCompletion`-Mock immer verfügbar. [D/W]

## Vorbedingungen aus anderen Gruppen (Eingangsverträge)

Diese Pläne konsumieren Typen aus Gruppe B. Sind diese noch nicht gebaut, definiert der jeweilige C-Plan einen **minimalen Platzhalter-Trait/-Typ** mit identischer Signatur, der später gegen die B-Implementierung getauscht wird:
- **B1** `Queue`, `Node`, `Task`, `TaskState` (lock-free Liste, `claim`=CAS `Queued→Taken`).
- **B3** `Operation`, `requires()→Requirement`, `materialize:bool`, `Value::Host|Device(handle)`.
- **B4** `Index`-Trait (objekt-sicher: `stamp/read/apply/changed/capabilities/...`).
- **B6** `CapabilitySet`, `Profile` (nur von C3 benötigt, Matching selbst ist Gruppe F).

## Offene Risiken, die in Gruppe C nur abgesichert (nicht gelöst) werden

1. Keine belastbare Quelle zu „launch-bound GPU-Worker"-Dimensionierung (Q13) → Worker-Zahl als Parameter, per Messung bestimmen.
2. `launch_host_function`-Signatur und candle-Stream-Erzeugung nur in der **gepinnten** Version verifizierbar (Q12) → als erster Schritt in C2 gegenprüfen.
3. Starved-Hint-Verfall und Shared-Queue ohne Head-of-Line-Blocking (Q17/Q21) sind Eigenentwürfe [W] → mit Lasttests absichern.
4. Ringbuffer-/Graph-Capture-Modi bleiben Ausblick (Gruppe H); C2 sichert nur Erweiterungspunkte.
