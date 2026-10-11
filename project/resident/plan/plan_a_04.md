# Umsetzungsplan A4 — Querschnittskonventionen festlegen

> Teil von Gruppe A (Fundament & Kontext).
> Quellen: `resident_planplan.md` (Paket A4 + Querschnitt Q1–Q4), `context_a.md` (Paket A4
> Nr. 16–18), Fehler-/Logging-Abschnitte aller Dokumente (`resident.md`, `resident_index.md`,
> `resident_processor.md`, `resident_precision.md`).

## 1. Ziel & Ergebnistyp

Einheitliche Konventionen für **Fehler** (`thiserror`), **Logging** (`tracing`) und
**Test/Mocks** festlegen, dazu passendes **Basis-Scaffolding** anlegen. Das Dokument speist die
Querschnitte Q1–Q4 und wird je Gruppe fortgeschrieben.

**Ergebnistyp:** Konventionsdokument + minimales, kompilierendes Basis-Scaffolding
(`error.rs`, `telemetry.rs`, Mock-Konvention, Feature-Flag-Konvention).

## 2. Eingaben & Quellen

- `context_a.md` Paket A4 Nr. 16–18 (alle **[W]**): Fehler-Taxonomie, tracing-Spans, Traits+Mocking.
- `resident_planplan.md` Querschnitt **Q1** (konkrete Fehlervarianten-Liste), **Q2**, **Q3**, **Q4**.
- Fehler-/Logging-Hinweise der Fachdokumente (z. B. `resident_index.md` → `UnsupportedScope`,
  `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern), `TransferFailed`;
  `resident.md` → Init-/Launch-Fehler + Backoff + Logging mit Task-ID).
- A2-Modulstruktur (Zielmodule `error.rs`/`telemetry.rs`, Crate-Grenzen).

## 3. Abhängigkeiten

- **Abhängt von:** A2.
- **Blockiert / speist:** alle Gruppen (Q1–Q4 begleiten durchgehend); unmittelbar relevant für
  C2 (Backpressure/Fehler), E (Sync-Fehler), F (Kalibrierungsfehler).

## 4. Vorannahmen / verifizierter Stand (context_a.md)

- thiserror **2.0.20**, tracing **0.1.44**, tracing-subscriber **0.3.23**, mockall **0.15.0** **[V]**.
- Konventionsempfehlungen **[W]** (nicht gegen Primärquellen geprüft) — vor Festschreibung kurz
  gegen aktuelle Crate-Doku gegenchecken (thiserror 2.0 Format-Syntax, tracing-instrument).

## 5. Arbeitsschritte (für den Agenten)

### 5.1 Q1 — Fehler-Taxonomie (`thiserror`)

1. **Ein Fehler-Enum pro Schicht** (z. B. `AdapterError`, `OrchestratorError`, `ManagerError`,
   `CalibrationError`); obere Schichten wrappen untere mit `#[from]`/`#[source]`.
2. Öffentliche Enums `#[non_exhaustive]`; alle Fehler **`Send + Sync + 'static`** (für
   `async`-Grenzen/`JoinHandle`/Channels). Kontext (z. B. `task_id`) als **Felder**, nicht String.
3. **Gemeinsame Taxonomie-Basis** verankern (aus Q1), zunächst als Varianten/Platzhalter, in den
   jeweiligen Gruppen zu füllen:
   `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern),
   `TransferFailed`, `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`,
   plus Planner-/Config-/Capability-Fehler.
4. **Retry-Klassifikation:** `is_retryable()` — OOM/kritisch vs. wiederholbar trennen
   (für C2-Backoff). Worker-Pool liefert `Result<T, E>` per Channel, nie `panic`; Panics via
   `JoinError` abfangen. `anyhow` nur in Binary/`main`.

### 5.2 Q2 — Logging (`tracing`)

1. **Span-Granularität:** ein Span **pro Task** (Feld `task_id`), Child-Spans pro
   Verarbeitungszyklus/Batch und **pro Manager-Zyklus**; Ressource (Device/Index) als **Feld**,
   nicht als eigener Span.
2. **Async-Regel:** `#[tracing::instrument(skip_all, fields(task_id = %id))]` bzw.
   `.instrument(span)` auf Futures; **kein** `span.enter()` über `.await`.
3. **Thread/`spawn_blocking`-Regel:** Span klonen, im Closure `span.in_scope(...)` /
   `let _g = span.enter();` (Kontext propagiert nicht automatisch). GPU-Aufrufe über
   dedizierten Thread/`spawn_blocking`, nicht als async-Task.
4. **Subscriber:** `tracing-subscriber` mit `EnvFilter`; hochfrequente Kernel-Level-Events auf
   `trace!`. Definieren, welche Ereignisse je Gruppe geloggt werden (z. B. Manager:
   Scan-Ergebnis, Lade-/Eviction-Entscheidung, verworfene/`outdated`-Ergebnisse, Arena-Auslastung).

### 5.3 Q3 — Teststrategie & Mocks

1. **Schmale Traits** für GPU-/Mess-Schnittstellen (z. B. `Kernel` aus B5, Mess-Trait aus F2);
   Generics (statischer Dispatch) im Hot-Path, `dyn` nur an Rändern.
2. **Mock-Strategie:** `mockall` (`#[automock]`) für Orchestrator-Tests; für deterministische
   Fakes (fester Embedding-Output, simulierter Fehler) handgeschriebene `FakeGpu`/`FakeKernel`
   (einfacher bei async/`Send`). Sichtbarkeit `#[cfg(any(test, feature = "mock"))]`.
3. **Testmatrix-Vorgaben** je Paket referenzieren (B1 Queue-Semantik, E5/E6 Sync, F2–F5
   Kalibrierung) — als Konvention, dass jedes Paket seine Matrix mitliefert.
4. **No-GPU-Pfad:** GPU-Integrationstests mit `#[ignore]` oder eigenem Feature; CI ohne GPU bleibt grün.

### 5.4 Q4 — Feature-Flags

1. Konvention aus A2 übernehmen und festschreiben: reale CUDA-Pfade hinter `cuda`,
   Backend-Auswahl per `#[cfg]`:
   `#[cfg(feature = "cuda")] type DefaultBackend = CudaBackend;`
   `#[cfg(not(feature = "cuda"))] type DefaultBackend = CpuBackend;`
2. `cuda` nie in `default`; `dep:`-Syntax für optionale GPU-Crates.

### 5.5 Basis-Scaffolding anlegen

1. `resident-core/src/error.rs`: Taxonomie-Basis + Trait/Helfer `is_retryable()`, Re-Export in
   `lib.rs`.
2. `resident-core/src/telemetry.rs`: Span-Helfer + Subscriber-Init (`EnvFilter`), Feld-Namen
   (`task_id`, `resource`) als Konstanten.
3. Mock-Konventionsstub + `mock`-Feature in `Cargo.toml` verdrahten.

## 6. Zu erzeugende Artefakte

- `rag/docs/architecture/conventions.md` (Q1–Q4 vollständig, mit den Regeln aus 5.1–5.4).
- `rag/crates/resident-core/src/error.rs` + `telemetry.rs` (kompilierend, leere/Platzhalter-Varianten).
- `mock`-Feature in `resident-core/Cargo.toml`.

## 7. Offene Punkte / Verifikation vor Abschluss

- **[W]→verifizieren:** thiserror-2.0 Format-Syntax (Raw-Identifier `{r#type}`, Display-Bounds)
  gegen aktuelle Doku; `#[tracing::instrument]`-Signatur gegen tracing 0.1.44.
- Prüfen, dass alle Q1-Varianten aus dem Metaplan im `error.rs`-Basisgerüst **adressiert** sind
  (als Variante oder dokumentierter TODO der Zielgruppe).
- Gegencheck mit A3: Lade-/Kompatibilitätsfehler (`Incompatible`, `TransferFailed`) sind abgedeckt.

## 8. Akzeptanzkriterien (Definition of Done)

1. `conventions.md` deckt Q1–Q4 vollständig ab (Fehler, Logging, Test/Mocks, Feature-Flags).
2. `error.rs` + `telemetry.rs` kompilieren im GPU-freien Build (`--no-default-features`).
3. Alle Q1-Fehlervarianten aus dem Metaplan sind verankert (Variante oder zugewiesener TODO).
4. Mock-Konvention + `mock`-Feature funktionieren (Beispiel-Fake kompiliert außerhalb von `#[test]`).

## 9. Querschnitt (Q1–Q4)

- Dieses Paket **ist** die initiale Fixierung von Q1–Q4; nachfolgende Gruppen schreiben die
  Taxonomie, Logging-Ereignisse und Testmatrizen in ihren Modulen fort.
