# Umsetzungsplan Q2 · Logging (`tracing`)

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben.

## 1. Zweck

Einheitliches strukturiertes Logging/Tracing: Spans pro Task-ID, Ressource und Manager-Zyklus; stabile Feldnamen; korrekte Propagation über async-/Thread-/CUDA-Callback-Grenzen; Initialisierung nur im Binary.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q2)

- **Spans/Felder:** `#[tracing::instrument(skip_all, fields(task_id = %id))]` auf Funktionen; `skip_all` verhindert Loggen großer Argumente, Felder explizit. Span pro Task/Ressource/Manager-Zyklus, **stabile Feldnamen**. Strukturierte Events `info!(bytes = n, "msg")`. Hochfrequente Pfade auf `trace`/`debug`.
- **Propagation:** Futures `fut.instrument(span)`; gespawnte Tasks `tokio::spawn(fut.instrument(Span::current()))`; synchron `span.in_scope(|| …)`; **`Entered`-Guard nie über `.await` halten**. `spawn_blocking`/Threads übernehmen den Span nicht automatisch → `let span = Span::current(); spawn_blocking(move || span.in_scope(|| …))`.
- **Subscriber:** `tracing-subscriber` mit `EnvFilter` (z. B. `RUST_LOG=engine=debug,engine::gpu=trace`), `fmt()` menschenlesbar, `.json()` (Feature `json`) für Produktion; `tracing-appender::non_blocking` (Guard am Leben halten). **Initialisierung im Binary, nie in der Library.**
- **Nicht-tokio-Threads / CUDA-Callbacks:** `tracing` ist threadsicher bei globalem Subscriber; Span explizit übergeben. **CUDA-Host-Callbacks (`cuLaunchHostFunc`): keine CUDA-API darin, nur kurze Arbeit**; zum Loggen Daten per Channel/Atomic an einen Logging-Thread geben.

## 3. Verbindliche Konvention (Umsetzung)

**Stabile Feldnamen (Span-Felder):** `task_id`, `resource_kind`, `model_id`, `index_id`, `segment_gen`, `slot`, `lane`, `manager_cycle`, `op`, `precision`, `profile`. Immer dieselben Namen, damit Filter/JSON-Pipelines stabil sind.

**Span-Topologie:**
- **Task-Span** pro Task-ID (Fassade/Planner/Processor; `resident_planner.md`/`resident_processor.md`: „Span pro Task-ID").
- **Ressourcen-Span** pro geleaster Ressource (`resident_subengine.md` §6: „Span pro Task-ID und Ressource").
- **Manager-Zyklus-Span** pro Kommunikationsslot/Scan-Zyklus (`resident_index.md`: „Spans pro Task und pro Manager-Zyklus").

**Ereignis-Katalog (was pro Gruppe geloggt wird):**
- **Queue/Processor (B1/C2):** Claim/Complete/Cancel, Batch-Bildung, Starved-Hint-Erzeugung/-Verfall, GPU-Completion (Callback/`cuEventQuery`) — hochfrequent → `trace`.
- **Subengine/Lifecycle (D):** Lazy-Init-Start/-Ende, Lease-Zähler, Eviction-Entscheidung, `Failed`/Backoff, GPU-Init-Fallback auf CPU.
- **Index/Manager (E):** Scan-Ergebnis (geladen/veraltet/entfernt), Lade-/Eviction-Entscheidungen, verworfene und `outdated`-Ergebnisse, Arena-Auslastung (`resident_index.md`).
- **Präzision (F):** Kalibrierungsergebnis pro Kombination, verworfene Kombinationen mit Grund, abgeleitete Profile (Anzahl/Positionen), gewähltes Profil pro Task auf `debug` (`resident_precision.md`).

**Propagations-Regeln (verbindlich):**
- Worker-Pool/`spawn_blocking` (CPU-Kernel, Kalibrierung, blockierende Index-Reads): Span per `Span::current()` einfangen und `in_scope`. Kalibrierung/CPU-Kernel laufen im eigenen Thread-Pool (`resident_precision.md`).
- GPU-Completion-Callback: **kein** CUDA-/Log-schweres Tun; nur Completion-Eintrag + Notify (`resident_processor.md`). Logging erst im aufräumenden Worker (Schritt 1 der Worker-Loop) mit dem dort übergebenen Span.
- Kein `Entered`-Guard über `.await`.

**Bibliotheks-Regel:** Die Engine-Crate setzt **keinen** globalen Subscriber und keine Appender; das macht ausschließlich das Binary/der Daemon/CLI (`resident.md`: Server/Daemon/CLI).

## 4. Umsetzungsschritte

1. `#[tracing::instrument(skip_all, fields(...))]`-Konvention mit den stabilen Feldnamen etablieren (Helper/Doku).
2. Span-Erzeugung an den drei Topologie-Punkten (Task/Ressource/Manager-Zyklus).
3. Propagations-Helfer für `spawn_blocking`/Threads (`Span::current()` + `in_scope`).
4. Completion-Callback: Channel/Atomic → Logging im Worker.
5. Binary-Setup-Beispiel (EnvFilter + fmt/json + non_blocking) separat vom Lib-Code.
6. Hochfrequente Ereignisse auf `trace`/`debug`.

## 5. Tests

- `tracing-test` o. Ä.: Span-Felder vorhanden und korrekt benannt bei Task-/Manager-Pfaden.
- `spawn_blocking`-Pfad trägt die Task-ID (Span-Propagation).
- Lib initialisiert keinen globalen Subscriber (API-/Sichtbarkeits-Check).

## 6. Offene Verifikationen ([prüfen])

- `tracing`/`tracing-subscriber`/`tracing-appender`-Versionen und `json`-Feature in der Projekt-`Cargo.toml`.

## 7. Definition of Done

- Span-Topologie + stabile Feldnamen + Ereignis-Katalog + Propagations-Regeln sind umgesetzt und dokumentiert; Lib setzt keinen Subscriber; Completion-Callback loggt nicht selbst; Tests aus §5 grün.

## Result

Q2 (Logging) ist umgesetzt:

- **`Cargo.toml`**: `tracing-appender` ergänzt, `tracing-subscriber/env-filter` explizit, neues `json`-Feature (→ `tracing-subscriber/json`).
- **`src/resident/telemetry.rs`**: vollständiger stabiler Feldnamen-Satz (`task_id`, `resource_kind`, `model_id`, `index_id`, `segment_gen`, `slot`, `lane`, `manager_cycle`, `op`, `precision`, `profile`); Span-Topologie-Konstruktoren `task_span`/`resource_span`/`manager_cycle_span`; `init_tracing_non_blocking` (EnvFilter + fmt/json + non-blocking Appender, `WorkerGuard`); Moduldoc mit Propagations- und Callback-Regel; Tests für Feldnamen, Span-Felder und Thread-Propagation.
- **`src/bin/xyrag.rs`**: Subscriber-Setup (`init_tracing_non_blocking`, Guard gehalten) ausschließlich im Binary verdrahtet.
- **`docs/architecture/conventions.md`**: Q2-Abschnitt auf den vollen Stand (Feldnamen, Topologie, Callback-Regel, Subscriber-Setup) aktualisiert.

Instrumentierung der konkreten Ereigniskataloge (Queue/Processor/Subengine/Index/Precision) folgt, sobald die jeweiligen Gruppen (B–F) implementiert sind — diese Module sind aktuell nur Platzhalter.
