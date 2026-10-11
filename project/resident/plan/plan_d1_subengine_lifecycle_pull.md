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
