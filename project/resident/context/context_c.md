# Kontext Gruppe C – Orchestrierung (Resident / Processor / Planner)

Stand: 10.10.2026 · Zielsprache Rust · GPU über `candle` + `cudarc`

## Lesehilfe

Jeder Punkt trägt eine Belegstufe:

- **[B]** belegt: in dieser Recherche aus Primärquelle gelesen (Link vorhanden).
- **[D]** Drittquelle: belegt nur durch Open-Source-Code/Blog Dritter, Qualität variiert.
- **[W]** Standardwissen/Design-Empfehlung: **nicht** in dieser Recherche gegen eine Quelle geprüft. Vor Übernahme verifizieren.

Geprüfte Versionen: tokio 1.53.2 · cudarc 0.19.10 · candle-core 0.9.1 (Quellseite; evtl. nicht aktuellste) · rayon 1.12.0 · thiserror 2.0.21 · tracing 0.1.44 · proptest 1.10.0 · std 1.99.0 · CUDA Driver API Doku 13.4 (Stand 14.09.2026).

---

## Wichtigste Befunde (Kurzfassung)

1. **Host-Callback ist hart eingeschränkt** [B]: keine CUDA-API-Aufrufe, keine Synchronisation, die von ausstehender CUDA-Arbeit abhängt. Außerdem blockiert der Callback **nachfolgende Arbeit im Stream**. Er darf nur „Eintrag ablegen + Notify".
2. **Callback wird bei Fehler im CUDA-Kontext nicht aufgerufen** [B]. Das ist der dokumentierte Grund für den `cuEventQuery`-Fallback.
3. **`CudaEvent::is_complete()` liefert bei CUDA-Fehlern `false`** [B]. Für Polling `try_is_complete()` nehmen, sonst wird ein Fehler als „noch nicht fertig" gelesen.
4. **tokio hat keinen „Mindest-Idle-Threads"-Schalter** [B]. Persistente Worker-Loops gehören auf eigene `std::thread`s.
5. **`Notify`: `notify_one` speichert max. 1 Permit, `notify_waiters` keins** [B]. Für Fan-out + Lost-Wakeup-Freiheit `Notified` vor der Zustandsprüfung erzeugen/`enable()`n.
6. Für Q17–Q21, Q24–Q26, Q28 wurde **keine belastbare Primärquelle** gefunden (siehe Lücken am Ende).

---

## C1 · Fassade „Resident"

**1 · Config-vor-Init-Lebenszyklus** [B/W]
- Empfehlung: Builder mit **verbrauchendem** `build()`/`init()`, das direkt `Resident` (bereit) liefert. Typestate (`Resident<Konfiguriert>` → `Resident<Bereit>`) nur, wenn es wirklich mehrere nutzbare Zwischenzustände gibt.
- Begründung: Typestate = Zustand im Typ, Übergänge konsumieren `self`, falsche Operationen kompilieren nicht (Cliffle).
- Trade-off [W]: Typparameter „infiziert" Signaturen und erschwert `dyn`/Speichern in Strukturen; für öffentliche Bibliotheks-API reicht ein verbrauchender Builder meist.
- Quelle: https://cliffle.com/blog/rust-typestate · Crate-Hilfe: `typed-builder`.

**2 · Erweiterbare Parameterobjekte** [B]
- Empfehlung: `#[non_exhaustive]` Struct + `Default` + Konstruktor/Setter (konsumierend). Felder als `Option`/mit Default.
- Wichtig: Außerhalb des Crates sind `#[non_exhaustive]`-Structs **nicht per Struct-Ausdruck konstruierbar** (auch nicht mit `..Default::default()`), Matching braucht `..`. Ohne Konstruktor/Builder ist die Struct also unbenutzbar.
- `#[non_exhaustive]` auf Enums: Konstruktion ok, Matching braucht Wildcard-Arm; neue Varianten sind dann kein Breaking Change.
- Versionierte Enums (`ParamsV1/V2`) [W]: unnötig schwer, nur bei Serialisierungsverträgen sinnvoll.
- API-Guidelines: private Felder (C-STRUCT-PRIVATE), Sealed Traits (C-SEALED).
- Quellen: https://doc.rust-lang.org/reference/attributes/type_system.html · https://rust-lang.github.io/api-guidelines/future-proofing.html

**3 · Query/Passage-Enum + Settings** [W]
- Vorschlag: `#[non_exhaustive] enum TextRole { Query, Passage }` als **Feld** der Embedding-Parameter (nicht Parameter pro Variante). Cross-Encoder-Paare als eigener Eingabetyp (`PairInput { query, chunk }`) mit eigenem Parameterobjekt, nicht als weitere Enum-Variante des Texttyps.
- Begründung: Rolle und Zusatzsettings bleiben orthogonal; Erweiterung ohne Breaking Change dank `non_exhaustive`.
- Keine externe Quelle geprüft.

**4 · Lazy-Fassade ohne Ressourcenverbrauch** [B]
- `tokio::sync::OnceCell::get_or_try_init`: gleichzeitige Aufrufer warten auf **einen** Init; bei Fehler/Abbruch/Panic wird der Versuch verworfen und ein wartender Aufrufer versucht erneut. Rekursive Initialisierung → Deadlock.
- `std::sync::OnceLock`: stabil seit 1.70, `get_or_init` blockierend (sync), nie vergiftet; **`get_or_try_init` ist in std 1.99.0 weiterhin nightly** (`once_cell_try`, #109737). Reentranz-Init → Deadlock.
- Empfehlung: async-Modell-/Executor-Init → `tokio::sync::OnceCell` pro Ressource (Modell, Executor). Parallele Inits begrenzen mit `Semaphore`. `init()` der Fassade legt nur leere Zellen an.
- `once_cell`-Crate (`sync::OnceCell`, hat stabiles `get_or_try_init`) [W]: in dieser Recherche nicht geprüft.
- Quellen: https://docs.rs/tokio/latest/tokio/sync/struct.OnceCell.html · https://doc.rust-lang.org/std/sync/struct.OnceLock.html

**5 · Executor mit Mindest-Idle-Threads** [B]
- tokio `Builder`: `worker_threads` (Default = Kerne; Worker sind dauerhaft), `max_blocking_threads` (Default 512), `thread_keep_alive` (Default 10 s). **Keine Option für minimale Idle-Threads im Blocking-Pool.** Die Blocking-Queue hat keinen Backpressure.
- `spawn_blocking`-Doku: lang laufende/persistente Loops → **dedizierter `std::thread`**.
- rayon `ThreadPoolBuilder::num_threads` garantiert nur „höchstens" diese Zahl; `start_handler`/`spawn_handler` erlauben Namen/Priorität pro Thread. Ob rayon alle Threads sofort startet, ist nicht dokumentiert [W].
- Empfehlung: Worker-Loops als eigene benannte `std::thread`s (Mindestzahl = Anzahl gestarteter Loops, sie warten per Notify/Park).
- Quellen: https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html · https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html · https://docs.rs/rayon/latest/rayon/struct.ThreadPoolBuilder.html

**6 · Ergebnisabholung per Event/Notify** [B]
- `oneshot`: genau ein Wert, ein Sender, ein Empfänger → Einzelergebnis pro Task. Alternativ `JoinHandle`, wenn das Ergebnis die letzte Aktion der Task ist.
- `watch`: nur der **letzte** Wert wird gehalten, viele Empfänger → Level-Signal „bereit/Zustand" für mehrere Warter.
- `broadcast`: jeder Empfänger sieht **jeden** Wert (Fan-out von Events).
- `Notify`: kein Datenträger; `notify_one` speichert max. 1 Permit, `notify_waiters` keins.
- Lost-Wakeup-Reihenfolge (mpmc-Beispiel der Doku): `notified()` erzeugen → `tokio::pin!` → `enable()` → Zustand prüfen → erst dann `.await`. Ein `Notified` ist für `notify_waiters` ab Erzeugung registriert, für `notify_one` erst nach Poll/`enable()`.
- Quellen: https://docs.rs/tokio/latest/tokio/sync/index.html · https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html

**7 · Eine API für Daemon und CLI** [B]
- Tokio-Doku „Bridging": (a) `Runtime` im Wrapper halten + `block_on`, (b) `Runtime::spawn`/`Handle`, (c) Runtime in eigenem Thread + `mpsc` (Actor-Stil, `blocking_send`).
- Fallstricke (Doku): `current_thread`-Runtime arbeitet **nur während `block_on`**; Hintergrund-Tasks frieren danach ein → für Engine mit Hintergrund-Loops `multi_thread` oder eigener Runtime-Thread.
- Empfehlung: Kern async, dünner `blocking`-Wrapper mit eigener Runtime (nicht die des Daemons teilen). `block_on` innerhalb einer laufenden Runtime panikt [W, bekannt, nicht neu geprüft].
- Quelle: https://tokio.rs/tokio/topics/bridging

**8 · Index-Übergabe/-Besitz** [W]
- Empfehlung: `Arc<dyn Index + Send + Sync>`; Engine hält eine eigene Residency-Tabelle (Index-ID → Zustand/Leases) und kein Persistenz-Handle. Der Index kapselt Mutex/Persistenz selbst; Engine ruft nur Trait-Methoden.
- Trait objekt-sicher halten (keine Generics in Methoden). Keinen `std::sync::Mutex`-Guard über `.await` halten.
- Keine externe Quelle geprüft.

**9 · Task-Queueing-API-Ergonomie** [D/W]
- Muster in Open-Source-Engines: Anfrage-Enum mit eingebettetem `reply: oneshot::Sender<Result<…>>` über `mpsc` an Worker (z. B. `oxillama-server` `queue.rs`, `embellama` Worker-Pool mit `response_tx: oneshot::Sender`, `hibachi` Batcher mit Queue + `notify()`).
- Empfehlung: `submit()` gibt ein **Handle** zurück, das `IntoFuture` implementiert (`.await`) und zusätzlich `wait_timeout(d)`/`try_get()` bietet; `Drop` ohne Ergebnis = Cancel (siehe Q20).
- Hinweis: Quellen sind kleine Drittcrates, keine etablierten Referenzen.
- Quellen: https://docs.rs/oxillama-server/0.1.3/src/oxillama_server/queue.rs.html · https://docs.rs/crate/hibachi/latest/source/src/feedforward/batcher.rs · https://docs.rs/crate/embellama/latest/source/implementation_plan_server.md

---

## C2 · Processor (Orchestrierung)

**10 · `cuLaunchHostFunc`-Restriktionen** [B]
- Wörtlich sinngemäß (Driver API 6.16): Die Host-Funktion läuft nach der bereits eingereihten Arbeit und **blockiert nachfolgende Arbeit** im Stream. Sie **darf keine CUDA-API-Aufrufe** machen (kann `CUDA_ERROR_NOT_PERMITTED` liefern, ist aber nicht garantiert) und **keine Synchronisation**, die von ausstehender, nicht früher erzwungener CUDA-Arbeit abhängt.
- Host-Funktionen ohne erzwungene Reihenfolge (z. B. unabhängige Streams) laufen in **undefinierter Reihenfolge** und **können serialisiert** werden.
- `cuLaunchHostFunc_v2(…, syncMode)` existiert in Doku 13.4 (neu, Details nicht geprüft).
- Folgerung für das Design: Callback = Completion-Eintrag (lock-free) ablegen + `Notify` auslösen. Nichts weiter.
- Quelle: https://docs.nvidia.com/cuda/cuda-driver-api/cuda_driver_api/group__CUDA__EXEC.html
- Drittquelle mit Kommentar zur Einschränkung („keine CUDA-Aufrufe, keine meisten Locks, kein Blockieren"): `atomr-accel-cuda` `completion/host_fn.rs`.
- Forum-Aussage, **ungeprüft/niedrige Qualität**: ein Callback-Thread pro Device. Nicht als Fakt übernehmen.

**11 · `cuEventQuery` als Fallback** [B]
- `cuEventQuery`: `CUDA_SUCCESS` wenn alle erfassten Arbeiten fertig, `CUDA_ERROR_NOT_READY` wenn nicht; nicht blockierend. Kann außerdem Fehler früherer asynchroner Launches zurückgeben. Ein nie aufgezeichnetes Event gilt als fertig (`SUCCESS`).
- Event mit `CU_EVENT_DISABLE_TIMING` (ohne BLOCKING_SYNC) ist laut Doku am schnellsten für `cuEventQuery`.
- Dokumentierter Verlustfall: Der Host-Callback wird bei Fehler im CUDA-Kontext **nicht** aufgerufen (Unterschied zu `cuStreamAddCallback`). → Poll-Fallback ist nötig. Weitere „verzögerte Callbacks" sind nur als Serialisierung/undefinierte Reihenfolge dokumentiert, keine Sonderfälle gefunden.
- Doppelverarbeitung vermeiden [W, eigenes Muster]: pro Lane ein `AtomicU8`-Zustand; Callback **und** Poller versuchen `compare_exchange(Launched → Completing)`; nur der Gewinner verarbeitet.
- cudarc: `CudaEvent::try_is_complete() -> Result<bool, DriverError>` (`Ok(false)` nur bei `NOT_READY`); `is_complete()` gibt bei Fehlern `false` zurück → **`try_is_complete` verwenden**.
- Quellen: https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__EVENT.html · https://docs.rs/cudarc/latest/cudarc/driver/safe/struct.CudaEvent.html

**12 · Zugang über `candle`/`cudarc`** [B/D]
- cudarc 0.19.10: `CudaStream::cu_stream()` (Rohhandle, nicht zerstören), `record_event(flags) -> CudaEvent`, `wait(&CudaEvent)`, `join`, `fork`; `CudaEvent::cu_event()`, `record`, `synchronize`, `try_is_complete`. `CudaStream`/`CudaEvent` sind `Send + Sync`.
- Host-Funktion: **kein sicherer Wrapper auf `CudaStream`**; erreichbar über `cudarc::driver::result::stream::launch_host_function(stream.cu_stream(), trampoline, arg)` [D: so verwendet in `atomr-accel-cuda` 0.10.0, Trampoline gibt `Box<oneshot::Sender>` frei]. Signatur im Quelltext der gepinnten Version prüfen.
- candle-core 0.9.1: `CudaDevice::cuda_stream() -> Arc<CudaStream>`; `cudarc` wird re-exportiert (`candle::cuda_backend::cudarc`). **Genau dieselbe cudarc-Version aus candle verwenden** (nicht separat dazuziehen).
- Offene Unstimmigkeit: Quellen zeigen für candle teils `context.new_stream()`, teils `default_stream()` (ältere Doku). Im gepinnten candle-Code prüfen.
- Stream-Sharing: Ein Host-Callback auf dem candle-Stream blockiert dort alle Folge-Kernel → Callback minimal halten. Für Stream-Wechsel das Event/Wait-Muster nutzen (`kaio-candle` macht das so zwischen candle- und eigenem Stream).
- Quellen: https://docs.rs/cudarc/latest/cudarc/driver/safe/struct.CudaStream.html · https://docs.rs/atomr-accel-cuda/latest/src/atomr_accel_cuda/completion/host_fn.rs.html · https://docs.rs/crate/candle-core/0.9.1/source/src/cuda_backend/device.rs · https://docs.rs/crate/kaio-candle/0.2.0/source/src/bridge.rs

**13 · Worker-Zahl-Dimensionierung** [B/Lücke]
- Belegt (Ryhl): CPU-gebundene Berechnung ist am effizientesten bei Threadzahl = Kernzahl; `spawn_blocking` hat viel mehr Threads und ist dafür „suboptimal".
- **Keine Literatur** zu „launch-bound GPU-Workern" gefunden. Empfehlung: GPU-Worker-Zahl = Streams/Devices (klein, z. B. 1–2 pro Device), CPU-Worker = Kerne − (GPU-Worker + Reserve). Per Messung festlegen: Auslastung der Worker-Threads, Launch-Latenz, GPU-Idle-Anteil [W].
- Quelle: https://ryhl.io/blog/async-what-is-blocking/

**14 · Höhere Priorität für GPU-Worker (Linux)** [B/W]
- `sched_setscheduler(2)`: Linux-Scheduling-Attribute sind **pro Thread**; `pid=0` = aufrufender Thread. `SCHED_OTHER/BATCH/IDLE` nur mit Priorität 0; `SCHED_FIFO/RR` Priorität 1–99; ohne Berechtigung `EPERM`. Details zu Rechten: `sched(7)`, `capabilities(7)`.
- Empfehlung [W]: Nice-Wert per Thread über `libc::setpriority(PRIO_PROCESS, gettid, nice)`. **Besser die CPU-Engine-Worker herunterstufen** (höherer Nice-Wert), das braucht keine Privilegien; Heraufstufen der GPU-Worker braucht `CAP_SYS_NICE`/`RLIMIT_NICE`. Echtzeit-Policies vermeiden (Aushungern anderer Threads). Fehler (`EPERM`) nur loggen, hinter `cfg(target_os = "linux")`.
- Setzen im Worker-Start: eigener Thread selbst, oder rayon `start_handler`, oder tokio `on_thread_start`.
- Quelle: https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html

**15 · `candle`/`rayon`-Blocking im async-Kontext** [B/Lücke]
- Belegt: Async-Code soll nie lange ohne `.await` bleiben (Faustregel 10–100 µs). Für CPU-Compute: rayon (+ `oneshot` zurück an tokio), dedizierter Thread oder `spawn_blocking`.
- `spawn_blocking`-Tasks sind nach Start **nicht abbrechbar**; Runtime-Shutdown wartet auf sie; Pool ist groß (512) → Semaphore zum Begrenzen.
- **Nicht verifiziert:** wie `candle-core` rayon im CPU-Backend nutzt (`Cargo.toml`/`cpu_backend` des gepinnten Releases prüfen). Praktisch relevant: rayon-Globalpool und eigene Pools können sich Kerne teilen.
- „Blockieren des Workers = natürliches Rückdruck": **tragfähig nur für dedizierte Non-Runtime-Threads mit beschränkter Queue**. Auf tokio-Worker-Threads blockieren ist das Gegenteil (Starvation). Expliziten Rückdruck über beschränkte `mpsc` (Kapazität = Rückdruck laut tokio-Doku) oder `Semaphore` bevorzugen.
- Quellen: https://ryhl.io/blog/async-what-is-blocking/ · https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html

**16 · Notify-Topologie / Fan-out** [B/W]
- `notify_waiters`: weckt alle **bereits registrierten** Warter, speichert kein Permit. `notify_one`: weckt den ersten Warter oder speichert 1 Permit.
- Muster A: Pro Subengine-Schleife ein eigenes `Notify`; ein Relay ruft nach jedem Queue-Append `notify_one()` auf jedes (Permit-Speicherung verhindert Lost-Wakeups bei einzelnem Konsumenten).
- Muster B [W, API-Semantik aus tokio-`sync`-Doku, `watch`-Details nicht separat geprüft]: ein `watch<u64>`-Epochenzähler als Level-Signal; jede Schleife `changed().await` → keine Lost-Wakeups, kein Thundering-Herd-Problem durch Permit-Verlust, aber Schleife muss danach die Queue leer lesen.
- Mehrere Konsumenten pro `Notify`: Check-Reihenfolge aus Q6 einhalten (`enable()` vor Zustandsprüfung).

**17 · Backpressure / Starved-Hints** [B/Lücke]
- Belegt: tokio-Scheduler-Fairness über `global_queue_interval` (Default 31, current-thread) und `event_interval` (61): kleinere Werte = fairer, mehr Synchronisation; bounded `mpsc`-Kapazität steuert Backpressure; `spawn_blocking`-Queue hat keinen.
- „Starved(resource_kind)" als verfallendes Set: **keine Referenzimplementierung gefunden.** Eigenes Design [W]: Bitmaske/`AtomicU32` pro Ressourcenart mit Zeitstempel/Epoche; Bit wird beim Pick dieser Art oder bei Belegung gelöscht; nie als Queue modellieren.
- Quelle: https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html

**18 · Mindestpuffer an Operationen** [W, keine Quelle]
- Faustregel: Puffer ≈ (Refill-Latenz des Planners / Zeit pro Op) + 1, adaptiv nachgeführt (untere Wasserlinie; keine obere Füllschwelle erzwingen, Planner läuft nur bei Unterschreitung).

**19 · Lease-/Handle-Pattern für Device-Speicher** [D/W]
- Beleg für Generationszähler: `atomr-accel-cuda` beschreibt „generation-validated buffers".
- Empfehlung [W]: RAII-`Lease` (Drop = Use-Zähler −1) + `gen` im Handle; Evict nur bei `in_flight == 0 && leases == 0 && TTL abgelaufen`; Reload erhöht `gen`, alte Handles scheitern beim Auflösen mit `Stale`. Zugriff nur über `Lease::get() -> &Region`, nie rohe Pointer weitergeben.
- cudarc-Sicherheitsmodell: `CudaSlice` hält `Arc<CudaContext>`; Freigabe beim Drop ([B], cudarc-Moduldoku).

**20 · Oneshot + Notify für Abschluss; Cancel** [B/W]
- `oneshot` für Ergebnis, `Notify`/`watch` nur als „abholbereit"-Signal an Dritte (Scheduler/Mehrfachwarter). Ergebnis zuerst in den Zustand/Sender, **dann** Signal.
- Sender-Drop ohne Senden macht `recv` zum Fehler → diesen als `Cancelled` abbilden (Beispiel `atomr-accel-cuda`: „callback dropped without firing" → typed Error).
- Cancel/Timeout-Pfad [W]: Leases per RAII in Task-Zustand halten, damit sie auch bei Drop/Panic freigegeben werden; bereits gestartete GPU-Ops nicht abbrechbar → nach Completion Ergebnis verwerfen und Leases freigeben.

**21 · Shared-Queue-Semantik** [B-Analogie/W]
- Analogie: tokio-Scheduler prüft lokale Queue bevorzugt, die globale alle `global_queue_interval` Ticks (Fairness gegen Aushungern).
- Empfehlung [W]: eigene Queue zuerst, jede N-te Runde die Shared-Queue zuerst; beim Pop aus Shared nur nehmen, was zur eigenen Capability passt (`requires()`), sonst liegen lassen (kein Head-of-Line-Blocking: nicht vom Kopf blockieren, sondern bis zu K Einträge überspringen).

---

## C3 · Planner

**22 · Konfigurationsformat der Expansionsregeln** [W]
- Empfehlung: TOML (lesbar, Kommentare) via `serde` + **Validierung nach dem Deserialisieren** (Registry-Check: unbekannte Ops/Ressourcenarten → Fehler). RON, wenn Enums mit Daten häufig sind. Reine code-basierte Registry für Regeln, die Logik brauchen.
- Hot-Reload: Datei-Watcher (`notify`) + atomarer Austausch (`ArcSwap`/`RwLock<Arc<…>>`); Planner bleibt rein, weil er pro Aufruf einen Snapshot bekommt.
- Keine Quelle in dieser Recherche geprüft.

**23 · Registry-Pattern in Rust** [B/W]
- `inventory` 0.3: Registrierung zur Laufzeit „vor main" (constructor-basiert); `collect!` muss im selben Crate wie der Plugin-Typ stehen; `iter::<T>()` liefert `&'static T`; auf WebAssembly rufen manche Linker die Konstruktoren nicht auf. Alternative `linkme` ohne Life-before-main.
- Empfehlung [W]: **explizite Registry** (`HashMap<(Modell, Tasktyp), Box<dyn Expander>>` bzw. Enum-Dispatch) als Standard → deterministisch, leicht mockbar. `inventory`/`linkme` nur, wenn Fremdcrates sich selbst registrieren sollen. `fn`-Pointer/Trait-Objekte statt `Fn`-Closures mit Zustand, damit Expansion rein bleibt.
- Quelle: https://github.com/dtolnay/inventory

**24 · Impliziter Op-Graph** [W, keine Quelle]
- Empfehlung: geordnete `Vec<Op>` pro Task (Reihenfolge = Abhängigkeit), Op hat nur `requires()`; Verzweigung nur als „Folgeops erzeugen beim Abschluss". Vorteil: einfach, deterministisch testbar. Nachteil: keine automatische Parallelität zwischen unabhängigen Ops (bewusst).

**25 · `materialize`-Entscheidung** [W, keine Quelle]
- Regel: Zwischenergebnis bleibt Device-Handle, wenn der Nachfolger GPU-Ressource verlangt; materialisieren (Host-Kopie) bei CPU-Nachfolger, Task-Ende oder Index-Spiegelung. Entscheidung im Planner per Op-Metadaten, nicht im Worker.
- Hinweis aus cudarc: D2H-Kopien sind stream-geordnet; Host-Daten erst nach Completion lesen.

**26 · Aufgeschobene Device-Wahl** [W]
- Op deklariert nur `ResourceKind` + optionale Affinität (`requires() -> Requirement { kind, affinity: Option<HandleId> }`, `#[non_exhaustive]`). Die Subengine bindet beim Claim ein konkretes Device. Handle-Affinität später ergänzbar, ohne Planner zu ändern.

**27 · Zähler offener Operationen / Abschluss** [W, Standardwissen]
- `AtomicUsize pending`: neue Folgeops erst **zählen und einreihen** (`fetch_add(n, Relaxed)` + Enqueue), **danach** den Vorgänger abschließen (`fetch_sub(1, AcqRel) == 1` → `Done`). So erreicht der Zähler nie vorzeitig 0.
- Fehler: erstes Fehlerergebnis per `compare_exchange` auf ein `AtomicU8`-Status (`Running → Failed/Cancelled`) setzen; weitere Ops dieses Tasks beim Claim überspringen; `Done` nur wenn `pending == 0`.
- Parallelbeispiel aus std: `Arc` nutzt dasselbe Release/Acquire-Dekrementmuster.

**28 · Starved-Hint ohne Verletzung der Reinheit** [W, keine Quelle]
- Hint als **Eingabe** der Expansion (`expand(task, &ExpandContext { starved: … })`) behandeln, nicht als versteckten Zustand: gleiche Eingabe + gleicher Hint → gleiche Ops. Der Hint darf nur zwischen **äquivalenten** Varianten wählen (Reihenfolge/Präferenz), nie den Vertrag (Op-Menge/Ergebnis) ändern.

**29 · Testbarkeit reiner Expansion** [B/W]
- `proptest` 1.10.0 für Invarianten (z. B. „Op-Menge unabhängig von Hint", „Zähler endet bei 0"); `proptest-state-machine` für Lebenszyklus-Sequenzen (Referenz-Zustandsautomat gegen Implementierung).
- Golden-Tests für Expansionen pro Modell/Tasktyp (z. B. mit `insta`) [W]; Mock-Capabilities als Trait-Impl.
- Quellen: https://proptest-rs.github.io/proptest/intro.html · https://docs.rs/crate/proptest-state-machine/latest

---

## Querschnitt

**30 · Fehler-Taxonomie mit `thiserror`** [B]
- `thiserror` 2.0.21: erscheint nicht in der öffentlichen API (Wechsel zu Handimplementierung ist kein Breaking Change); `#[from]` impliziert `#[source]`; `#[error(transparent)]` leitet Display/source weiter; opaker öffentlicher Fehler (`pub struct PublicError(#[from] ErrorRepr)`) hält die Repräsentation änderbar.
- Empfehlung [W]: ein Fehler-Enum je Modul (`PlannerError`, `ConfigError`, `CapabilityError`, `OrchestrationError`), oben zusammengeführt mit `#[from]`; `#[non_exhaustive]` auf öffentlichen Enums. Über `oneshot` `Result<T, TaskError>` senden; wenn derselbe Fehler an mehrere Warter geht: `Arc<TaskError>` oder `Clone`. `Cancelled` als eigene Variante (nicht als String).
- Quelle: https://docs.rs/thiserror/latest/thiserror/

**31 · `tracing`-Span-Granularität** [B/W]
- Belegt: `Span::enter()`-Guard über `.await` hinweg erzeugt **falsche Traces** → für Futures `Instrument` nutzen, `in_scope` für Sync-Code. Spans/Events werden gar nicht erst erzeugt, wenn kein Subscriber Interesse hat; statische Level-Features (`max_level_*`, `release_max_level_*`) entfernen Aufrufe zur Compile-Zeit. Bibliotheken dürfen `set_global_default` **nicht** aufrufen.
- Empfehlung [W]: ein Span pro Task-ID (bei `submit` erzeugt, im Task-Zustand gehalten, Felder via `field::Empty` + `record`), ein Span pro Worker-Thread-Lebenszeit (nicht pro Zyklus), im Hot-Path (`claim`/`launch`/`complete`) nur **Events** mit `task_id`/`lane` als Felder auf `trace`/`debug`.
- Quelle: https://docs.rs/tracing/latest/tracing/

**32 · Feature-Flag `cuda`** [D/W]
- Muster: Trait `Completion` (orchestrierungsseitig, immer kompiliert) mit Implementierungen `HostFnCompletion`/`PolledCompletion` hinter `#[cfg(feature = "cuda")]` und einer `ManualCompletion` (Mock, auslösbar im Test) immer verfügbar. Beispiel `atomr-accel-cuda`: `CompletionStrategy`-Trait mit `HostFn`/`Polled`/`Sync`.
- `candle-core` selbst nutzt `dummy_cuda_backend` als Ersatz, wenn CUDA nicht einkompiliert ist (docs.rs-Auszug).
- cudarc lädt die CUDA-Bibliotheken per `libloading` (Dependency); Details zum Feature-Layout im `Cargo.toml` der gepinnten Version prüfen [W].
- Quelle: https://docs.rs/crate/atomr-accel-cuda/latest/source/src/completion/mod.rs

---

## Offene Lücken (keine belastbare Quelle gefunden)

- Q13: Literatur zur Dimensionierung launch-bound GPU-Worker.
- Q15: tatsächliche rayon-Nutzung in `candle-core` CPU-Backend.
- Q17/Q21: Referenzimplementierung für verfallende Starved-Sets und Shared-Queue ohne Head-of-Line-Blocking.
- Q18, Q24–Q26, Q28: nur Design-Empfehlungen, keine externen Belege.
- Q19: Lease-Muster nur indirekt belegt (Generationszähler bei `atomr-accel-cuda`).
- Q12: Signatur von `launch_host_function` und candle-Stream-Erzeugung in der **gepinnten** Version gegenprüfen.
- Q1, Q3, Q8, Q22: Empfehlungen ohne geprüfte Primärquelle (Typestate-Beleg aus Cliffle-Blog).