# Umsetzungsplan B1 · Queue & Task-Primitive

| | |
|---|---|
| **Paket** | B1 (Gruppe B — Kern-Datentypen & Verträge) |
| **Abhängt von** | A4 (Querschnittskonventionen Q1–Q4) |
| **Blockiert** | C1 (Fassade), C2 (Processor: Task-Queue + 3 Op-Queues), D1, E9 |
| **Ergebnis** | Datentyp- & Zustandsautomaten-Spezifikation + Rust-Implementierung der lock-freien Queue und Task-Primitive |
| **Quellen** | `../resident.md` (Queue Logik, Task, Notify-Topologie), `../resident_processor.md` (Queues, Worker), `../context/context_b.md` §B1 (1–7) |

---

## 1. Zweck & Abgrenzung

Eine **einzige** einfach verkettete, lock-freie Liste: nur **Append** (CAS auf `tail.next`) und **Unlink**. Reihenfolge = Alter. Keine Version, kein Snapshot, kein Lock. Priorität, Ressource, Modell, Modus sind **Metadata in `params`** — die Queue kennt sie nicht.

Die Struktur wird mehrfach instanziiert: eine **Task-Queue** und drei **Op-Queues** (CPU/GPU/shared — Letztere angelegt, leer, ohne Logik; siehe `resident_processor.md`). B1 liefert den **generischen** Queue-/Node-Baustein plus die konkrete **Task-Instanziierung**. Die Operation-Instanziierung folgt in B3/C2.

**Nicht Teil von B1:** Worker-Loop, Batching, Eviction, Expansion (C2/C3). B1 liefert nur Datentypen, Zustandsautomat, Notify-Relay-Hook.

**Invarianten:** Kein `unsafe` außerhalb von `arc-swap`. Iterierer blockieren nie. Drop iterativ.

---

## 2. Crates (Pins kommen aus A3)

- `arc-swap` (≥ 1.6, wegen Miri-/Stacked-Borrows-Fixes) — `ArcSwapOption<Node>` als Link.
- `tokio` (`sync`-Feature) — `Notify`, später `watch`.
- `parking_lot` — `Mutex` für `payload` (kein Poisoning, lock-freie Umgebung). Alternativ `std::sync::Mutex` mit Poison-Recovery (A3-Entscheidung).
- `std::sync::OnceLock` (stable ≥ 1.70) für das Set-once-Ergebnis.

---

## 3. Zu implementierende Typen

### 3.1 Zustandsautomat (`ClaimState`)

```rust
const QUEUED: u8 = 0; const TAKEN: u8 = 1; const DONE: u8 = 2; const CANCELLED: u8 = 3;

pub struct TaskState {
    state:  AtomicU8,                          // QUEUED | TAKEN | DONE | CANCELLED
    result: OnceLock<Result<Output, Error>>,   // genau einmal gesetzt durch Claim-Inhaber
    ready:  Notify,                            // weckt wartende Aufrufer
}
```

Übergänge (alle **ein** Schritt → `compare_exchange` *strong*, `AcqRel`/`Acquire`):
- `claim`: `QUEUED → TAKEN`. Einzige Stelle, an der Cancel relevant ist (scheitert bei `CANCELLED`/`TAKEN`/`DONE`).
- Fertigstellung: **erst** `result.set(out)`, **dann** `state.store(DONE, Release)`. Alleiniger Eigentümer (Taken) → `store` genügt. Danach `ready.notify_waiters()`.
- `cancel`: `QUEUED → CANCELLED`, danach `ready.notify_waiters()`. War schon `TAKEN`: kein Effekt. `TAKEN → CANCELLED` ist **nicht** erlaubt (laufende Arbeit); falls je nötig, separates Flag `cancel_requested`.
- Leser/Waiter: `state.load(Acquire)` — Release auf `DONE` + Acquire garantiert Sichtbarkeit von `result`. **Kein `SeqCst`.**
- Zweites `result.set` → `Err(value)`: als Invariantenverletzung `debug_assert!` + `tracing::warn!`.

### 3.2 Node & Link

```rust
pub struct Node<P, D> {
    pub params: P,                        // unveränderlich, zum Filtern VOR dem Claim
    pub state:  Arc<TaskState>,           // Handle des Aufrufers
    payload:    parking_lot::Mutex<Option<D>>, // beim Claim per take() entnommen
    next:       ArcSwapOption<Node<P, D>>,     // Append/Unlink per CAS
}
```

- **Append** (`append(tail, new)`): `tail.next.compare_and_swap(&None, Some(new.clone()))`; Swap fand statt, wenn der Rückgabe-`Guard` pointer-gleich `None` ist (Vergleich ist **Pointer-Identität**). Konflikt → Tail vorlaufen und erneut.
- **Payload-Übergabe:** `let data = node.payload.lock().take();` **nur nach gewonnenem Claim-CAS**. Lock kurz halten, Daten außerhalb des Locks verarbeiten, **kein `await` unter dem Lock**.
- Rückverweise (falls je nötig) nur `Weak`; `next` ist der einzige starke Zeiger → keine Zyklen.

### 3.3 Queue mit Sentinel-Tail

```rust
pub struct Queue<P, D> {
    head:   Arc<Node<P, D>>,   // Dummy-Sentinel; echte Knoten hängen dahinter
    relays: Vec<Arc<Notify>>,  // Notify jeder Subengine (Append-Relay)
}
```

- `push(params, state, data)`: Knoten anlegen, am Tail anhängen, danach **jedes** `relays`-Notify `notify_one()`-en (Append-Relay der Notify-Topologie).
- `iter()`: traversiert per `next.load()` (**Guard**, kein `load_full()`) — vermeidet Atomic-RMW-Contention auf geteilter Cacheline. Hält pro Schritt ein `Arc<Node>` → Pointer-ABA ausgeschlossen, Use-after-free unmöglich (kein Epoch/Hazard-Pointer nötig).
- `claim(node) -> bool`: delegiert an `node.state`-CAS `QUEUED → TAKEN`.

### 3.4 Unlink — **Entscheidung A3 vorbereiten, Default festlegen**

Reines CAS-Unlink einer einfach verketteten Liste ist ohne Markierung **nicht korrekt** (Harris-Problem: paralleles Aushängen von B und Anhängen/Aushängen hinter B verliert Änderungen). Default dieses Plans:

1. **Logisches Löschen** über `state` (`CANCELLED`/`TAKEN`/`DONE`); physisches Aushängen nur **lazy** durch Traversierer.
2. **Niemals den Sentinel-/Tail-Knoten aushängen.**
3. Alternativ (A3): Unlink unter kurzem `Mutex` (Queue-Mutation selten, Durchlauf lock-frei per `load`).

> `// TODO(A3): logisches Löschen + Dummy-Tail (Empfehlung) vs. Mutex-geschützte Mutation endgültig festlegen (context_b §B1.1, offener Punkt 3).`

### 3.5 Iterativer Drop (Pflicht)

```rust
impl<P, D> Drop for Node<P, D> {
    fn drop(&mut self) {
        let mut cur = self.next.swap(None);
        while let Some(arc) = cur {
            match Arc::try_unwrap(arc) {
                Ok(mut node) => cur = node.next.swap(None),
                Err(_shared) => break, // anderer Besitzer übernimmt den Rest
            }
        }
    }
}
```
Ohne dies: Rekursionstiefe = Kettenlänge → Stack-Overflow bei Zehntausenden Knoten.

---

## 4. Aufrufer- & Engine-Protokoll (Notify)

**Aufrufer** (wartet auf Ergebnis) — Reihenfolge gegen verlorene Wakeups:
```rust
let notified = state.ready.notified();
tokio::pin!(notified);
notified.as_mut().enable();                 // registrieren VOR dem Prüfen
match state.state.load(Acquire) {
    DONE | CANCELLED => { /* fertig */ }
    _ => notified.await,                    // sonst warten
}
```
Timeout = eigene Deadline des Aufrufers; `Cancelled` lokal abgeleitet. `notify_waiters()` (bei Fertig/Cancel) hat **kein** Permit → `Notified` muss vor der Prüfung existieren.

**Notify-Topologie** (aus `resident.md`), B1 liefert die Hooks:
| Quelle | Wirkung |
|---|---|
| Queue-Append | Relay auf das Notify **jeder** Subengine (`relays`) |
| Result / Init fertig / Evict-Timer | eigenes Notify (von C2 ausgelöst) |
| Iteration mit Fortschritt | eigenes Notify |

---

## 5. Wiederverwendung für Op-Queues (Hinweis an C2/B3)

`Queue<P, D>` ist über `params: P` und `payload: D` generisch. Instanzen:
- **Task-Queue:** `P = TaskParams`, `D = TaskData`, `state = Arc<TaskState>` mit `Result<Output, Error>`.
- **Op-Queues (CPU/GPU/shared):** `P = OpParams` (B3), `D = Value`-Eingabe; der Abschluss läuft über den Task-Op-Zähler (C2/B3), nicht über `TaskState::result`.
Die **shared**-Queue wird angelegt, bleibt leer und ohne Logik; Subengine prüft erst die eigene, dann shared.

---

## 6. Fehler & Logging

- Fehler via gemeinsame Taxonomie (Q1). B1 selbst wirft kaum typisierte Fehler; `result` transportiert `Result<Output, Error>` des Tasks.
- Doppel-Set von `result`, unerwarteter CAS-Zustand → `debug_assert!` + `tracing::warn!`.
- `tracing`-Span pro Task-ID (Q2); Events: `push`, `claim` (erfolgreich/abgelehnt mit aktuellem Zustand), `complete`, `cancel`.

---

## 7. Teststrategie (Q3, ohne GPU)

- **Zustandsautomat:** jeder erlaubte/verbotene Übergang; `claim` nach `cancel` scheitert; `cancel` nach `claim` wirkungslos; `Taken → Cancelled` abgelehnt.
- **Sichtbarkeit:** `complete` setzt `result` vor `DONE`; Leser sieht nach `load(Acquire)==DONE` immer das Ergebnis (loom-Modell oder gezielte Thread-Tests).
- **Nebenläufigkeit:** `loom`-Test (Feature-gated) für Append + Claim + Cancel; mehrere Claimer, genau einer gewinnt.
- **Notify:** kein verlorenes Wakeup (Completion zwischen `enable()` und `await`); `notify_waiters` ohne Wartende + anschließende Zustandsprüfung.
- **Payload:** `take()` genau einmal; nach Claim ist `payload` `None`.
- **Drop:** Kette mit ≥ 100 000 Knoten droppt ohne Stack-Overflow (iterativ).
- **ABA/Retention:** ausgehängter Knoten mit lebendem Durchläufer bleibt gültig; dieselbe ID neu eingereiht erhält neuen Knoten/Zustand (Zustand über Automat, nicht Pointer).

---

## 8. Offene Punkte (→ A3 / Spezifikation)

1. Unlink-Strategie: logisches Löschen + Dummy-Tail (Default) vs. Mutex-Mutation.
2. `parking_lot::Mutex` vs. `std::sync::Mutex` (+ Poison-Recovery) für `payload`.
3. Exakte Pins: `arc-swap`, `tokio`, `parking_lot` (A3).

## 9. Definition of Done

- `Queue<P,D>`, `Node`, `TaskState`/`ClaimState` implementiert, kein `unsafe` außer `arc-swap`.
- Append/Claim/Complete/Cancel/Unlink + iterativer Drop + Notify-Relay vorhanden.
- Testmatrix §7 grün, inkl. `loom`-Test (Feature-gated) und Drop-Stresstest.
- Task- und Op-Instanziierung dokumentiert (für C2/B3).
