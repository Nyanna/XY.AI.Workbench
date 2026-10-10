Lies zunächst alle Dokumente in `/home/user/xyan/xy.ai.workbench/project/resident` ein.

```json
{
  "id" : "call_fecd700b-dd1a-44b0-aa8a-cdcc668509d5",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
```

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe C zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=3fc1e8bf-ea64-4900-aa78-63a4715ae14f, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente im Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-5
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  reason: "Einlesen aller Projekt-Dokumente im resident-Verzeichnis, um den Kontext zu verstehen und die Fragen für Gruppe C aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-5 allow
Result Stats: id=3fc1e8bf-ea64-4900-aa78-63a4715ae14f, total: 61838, in: 119, out: 8913, reason: 3064, read: 1684, write: 52806
ReasoningToken: 66
ReasoningToken: 2998
Ich habe alle acht Dokumente eingelesen. Gruppe C (Orchestrierung) umfasst die Pakete **C1 Fassade**, **C2 Processor** und **C3 Planner**. Nachfolgend die Fragenliste für die Web-Recherche, abgeleitet aus den `Zu klären`-Punkten, den Zwecken und Abhängigkeiten dieser drei Pakete sowie dem im Projekt festgelegten Stack (`tokio`, `candle`/`candle-core` mit `cuda`, `cudarc`, `tokenizers`, `crossbeam`/`parking_lot`, `arc-swap`, `thiserror`, `tracing`, `parquet`/mmap).

---

# Web-Recherche-Fragen — Gruppe C (Orchestrierung)

## Kontext für den recherchierenden Agenten
Zielsprache: Rust. Es geht um eine nebenläufige „Resident"-Engine mit Fassade, einem Processor (Orchestrierung mit Lock-free-Pull-Queues) und einem zustandslosen Planner (Task→Op-Expansion). GPU über `candle` + `cudarc`. Gesucht sind **belegte Best Practices, API-Semantik und Pattern-Empfehlungen** (mit Quellen/Links), keine fertige Implementierung. Pro Antwort möglichst: Empfehlung, Begründung, Trade-offs, Quelle, Versions-/Feature-Hinweise.

---

## C1 · Fassade „Resident" (Einstiegspunkt/API)

1. **Config-vor-Init-Lebenszyklus:** Welche Rust-Patterns erzwingen die Reihenfolge `konfigurieren → initialisieren → bereit` möglichst zur Compile-Zeit (Typestate-Pattern, Builder mit verbrauchendem `build()`/`init()`)? Vor-/Nachteile für eine öffentliche Bibliotheks-Fassade.
2. **Erweiterbare Parameterobjekte:** Wie gestaltet man Parameter-Structs in Rust so, dass spätere Felder **ohne Breaking Change** ergänzt werden können? Vergleich von `#[non_exhaustive]`, Builder-Pattern, `Option`-Feldern + `Default`, versionierten Enums. Welcher Ansatz passt für die Embedding-/Similarity-/Inferenz-Parameter?
3. **Query/Passage-Unterscheidung:** Idiomatische Modellierung eines Enums (`Query | Passage`) innerhalb eines erweiterbaren Parameterobjekts, inkl. späterer Erweiterung auf Cross-Encoder-Paare (Query+Chunk gemeinsam). Patterns für „Enum + zusätzliche Settings".
4. **Lazy-Fassade ohne Ressourcenverbrauch:** Welche Mechanismen stellen sicher, dass die Fassade nach `init()` noch **keine** Modelle/Executoren lädt (`OnceCell`/`tokio::sync::OnceCell`, `get_or_try_init`, `Lazy`, Semaphore für parallele Inits)? Unterschiede `once_cell` vs. `std::sync::OnceLock` vs. `tokio::sync::OnceCell` für async-Init.
5. **Executor mit Mindest-Idle-Threads:** Wie konfiguriert man einen Thread-/Task-Pool mit einer Mindestzahl idle laufender Threads? Möglichkeiten in `tokio` (Runtime-Builder, Worker-/Blocking-Pool) vs. eigenem Pool (`rayon`, `threadpool`). Wie hält man min. Idle-Threads vor?
6. **Ergebnisabholung per Event/Notify:** Vergleich der `tokio`-Primitive `oneshot`, `Notify`, `watch`, `broadcast` für das Muster „Ergebnis fertig → kann abgeholt werden". Welches Primitive für Einzelergebnis pro Task, welches für „bereit"-Signalisierung an mehrere Warter? Empfohlene Reihenfolge `notified().enable()` → `state.load()` → warten (Lost-Wakeup-Vermeidung).
7. **Eine API für Server/Daemon **und** CLI:** Patterns, um dieselbe Engine sowohl in einem lang laufenden async-Daemon als auch als kurzlebiges CLI synchron nutzbar zu machen (sync/async-Brücke, `Runtime::block_on`, optionaler Blocking-Wrapper). Fallstricke beim Teilen einer tokio-Runtime.
8. **Index-Übergabe/-Besitz:** Idiomatik für die Übergabe eines Index-Objekts an die Engine als Trait-Objekt mit geteiltem Besitz (`Arc<dyn Index + Send + Sync>`), während das Index-Objekt seinen **eigenen** Mutex/Persistenz besitzt. Patterns für „Engine verwaltet Residency, besitzt aber nicht die Persistenz".
9. **Task-Queueing-API-Ergonomie:** Wie gestalten vergleichbare Rust-Inferenz-/Job-Engines die öffentliche Methode zum Einreihen eines Tasks + Rückgabe eines Awaitables/Handles (Future vs. Handle mit `.await`/`.wait()` + Timeout)? Beispiele aus bestehenden Crates.

---

## C2 · Processor (Orchestrierung)

10. **`cuLaunchHostFunc`-Restriktionen:** Welche Operationen sind laut CUDA-Doku **innerhalb** des Host-Callbacks verboten (insb. CUDA-API-Aufrufe, Synchronisation)? Belegen, dass der Callback nur „Completion-Eintrag ablegen + Notify" tun darf. Offizielle Quelle (CUDA Programming/Driver API).
11. **`cuEventQuery` als Fallback:** Semantik von `cuEventQuery` (Polling ohne Blockieren, Rückgabewerte). Wie kombiniert man Callback-getriebene Completion **und** `cuEventQuery`-Polling, ohne eine Lane doppelt zu verarbeiten (CAS-/Claim-Pattern)? Gibt es dokumentierte Fälle verpasster/verzögerter Host-Callbacks?
12. **Zugang über `candle`/`cudarc`:** Sind `cuLaunchHostFunc`, Events und `cuEventQuery` über `cudarc` (bzw. das von `candle` genutzte `cudarc`) erreichbar? Welche API-Pfade/Typen, und wie greift man auf den von `candle` genutzten Stream/Event zu (Stream-Sharing)?
13. **Worker-Zahl-Dimensionierung:** Best Practices zur Zahl der Worker-Threads = CPU-Kerne als Obergrenze gegen Kontextwechsel; wie dimensioniert man „launch-bound" GPU-Worker (die nur Kernel starten und zurückkehren) gegenüber CPU-Workern? Messmethodik/Empfehlungen aus der Literatur.
14. **Höhere Priorität für GPU-Worker:** Wie setzt man unter Linux Thread-Prioritäten für bestimmte Worker (GPU inkl. CPU-bound Tokenizer/Normalisierung) höher als für CPU-Engine-Worker? Rust-Crates/`libc`-Wege (`setpriority`, `sched_setscheduler`), Portabilität und Rechte-Anforderungen.
15. **`candle`/`rayon`-Blocking im async-Kontext:** Wie nutzt der `candle`-CPU-Backend `rayon`, und warum darf blockierende Compute **nicht** auf tokio-Runtime-Threads laufen? Empfohlene Trennung (`spawn_blocking`, dedizierter Pool). Bestätigt „Blockieren eines Workers = natürliches Rückdrucksignal" als tragfähiges Muster?
16. **Notify-Topologie / Fan-out:** Patterns, um ein Queue-Append-Signal als Relay auf das `Notify` **jeder** Subengine-Schleife zu verteilen. `Notify::notify_waiters` vs. `notify_one`; `watch`-Kanal als Level-Signal; Vermeidung von Lost-Wakeups und Thundering-Herd.
17. **Backpressure / Starved-Hints:** Belegte Patterns für Backpressure in Pull-/Work-Stealing-Schedulern. Wie modelliert man „Starved(resource_kind)"-Hints als verfallendes Set (kein Queue), das bei Pick oder belegter Ressource erlischt? Referenz-Implementierungen (z. B. Scheduler in `tokio`, `rayon`, Job-Systeme).
18. **Mindestpuffer an Operationen:** Übliche Heuristiken, wie viele voraus-expandierte Operationen eine Pipeline vorrätig halten soll, damit sie nicht „trockenläuft", ohne zu überfüllen. Parameterwahl ohne feste Füllschwelle.
19. **Lease-/Handle-Pattern für Device-Speicher:** Patterns für Leases mit Use-Zähler (`Region`+`gen`), die Eviction verhindern (`in_flight==0` + Leases 0 + TTL). Vergleich mit RAII-Guards, Generationszählern zur Handle-Entwertung nach Evict/Reload. Beispiele aus GPU-Ressourcenmanagern.
20. **Oneshot + Notify-Kombination für Abschluss:** Empfohlenes Muster, Task-Abschluss sowohl per `oneshot` (Ergebnis) als auch per Event/Notify zu signalisieren; Umgang mit abgebrochenen (`Cancelled`) Tasks, Freigabe der Leases im Cancel-/Timeout-Pfad.
21. **Shared-Queue-Semantik:** Gibt es belegte Muster für eine „angelegte, leere" dritte Queue (CPU/GPU/shared), die zuerst die eigene, dann die shared Queue prüft (Priorität/Reihenfolge), ohne Head-of-Line-Blocking?

---

## C3 · Planner (reine Taskexpansion)

22. **Konfigurationsformat der Expansionsregeln:** Vergleich deklarativer Formate (RON, TOML, YAML, JSON via `serde`) vs. code-basierter Registry für „Regel pro Modell/Tasktyp". Welches Format eignet sich für eine erweiterbare „Registry-Eintrag pro Modell/Tasktyp" (Lesbarkeit, Validierung, Hot-Reload, Typprüfung)?
23. **Registry-Pattern in Rust:** Idiomatische Umsetzung einer Registry `Tasktyp/Modell → Expansionsfunktion` (z. B. `HashMap<Key, Box<dyn Fn...>>`, `inventory`/`linkme` für statische Registrierung, Enum-Dispatch). Trade-offs für Testbarkeit/Erweiterbarkeit.
24. **Pipeline/Op-Graph-Darstellung:** Wie stellen vergleichbare ML-/Daten-Pipelines eine Op-Folge als **impliziten** Graph (Reihenfolge der Ops eines Tasks) dar, statt eines expliziten DAG? Vor-/Nachteile, Beispiele.
25. **`materialize`-Entscheidung (Host vs. Device):** Belegte Heuristiken/Patterns, wann Zwischenergebnisse zwischen Pipeline-Stages im Device-Speicher bleiben vs. in den Host zurückgeholt werden (GPU-Nachfolger → Handle behalten; CPU-Nachfolger/Task-Ende/Index-Spiegelung → materialisieren).
26. **Aufgeschobene Device-Wahl:** Patterns, bei denen eine Stage nur die **Ressourcenart** deklariert und die konkrete Device-Zuordnung erst beim Claim der Subengine erfolgt (statt im Planner). Wie hält man das erweiterbar für spätere Handle-Affinität in `requires()`?
27. **Zähler offener Operationen / Abschluss:** Robuste Patterns für einen atomaren „Anzahl offener Ops"-Zähler pro Task (Dekrement, bei 0 `Done`), inkl. korrekter Reihenfolge „Folgeops einreihen **vor** Dekrement des Vorgängers" zur Vermeidung von vorzeitigem Abschluss. Fehler-Propagation (erster Fehler → `Cancelled`/`Err`).
28. **Gewichtung des Starved-Hints bei der Expansion:** Gibt es belegte Ansätze, wie ein Auslastungs-/Starvation-Hinweis die Reihenfolge/Präferenz einer zustandslosen Expansion beeinflussen kann, **ohne** die Reinheit (gleiche Eingabe → gleiche Ops) zu verletzen (Präferenz, kein Vertrag)?
29. **Testbarkeit reiner Expansion:** Bewährte Methoden zum Testen reiner/zustandsloser Funktionen mit Registry-gesteuertem Verhalten in Rust (Golden-Tests, Property-based Testing mit `proptest`, Mock-Capabilities der Subengines).

---

## Gruppenübergreifend (Querschnitt für C)

30. **Fehler-Taxonomie mit `thiserror`:** Best Practices für eine schichtenübergreifende, aber modulare Fehler-Hierarchie (Planner-/Config-/Capability-Fehler, Orchestrierungsfehler) mit `thiserror`, inkl. Fehlerweitergabe an den wartenden Task über `oneshot`.
31. **`tracing`-Span-Granularität:** Empfehlungen für Span-Struktur „pro Task-ID / pro Ressource / pro Worker-Loop-Zyklus" bei stark nebenläufigem Code; Instrumentierung von `claim`/`launch`/Completion, Vermeidung von Span-Overhead im Hot-Path.
32. **Feature-Flag `cuda`:** Patterns, um CUDA-abhängige Orchestrierungspfade (GPU-Completion, Callback, Event-Query) hinter einem Cargo-Feature zu kapseln, sodass die Orchestrierungslogik **ohne GPU** (Mock-Kernel/Mock-Completion) testbar bleibt. Beispiele für `cfg`-Abstraktion + Trait-Mock.

---

Soll ich diese Liste zusätzlich als Markdown-Datei (z. B. `resident_groupC_research_questions.md`) im Projektordner ablegen? Mir steht aktuell nur Lesezugriff zur Verfügung — für das Schreiben müsste ich wissen, ob ein entsprechendes Vorgehen/Tool gewünscht ist.
