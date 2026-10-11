# Umsetzungsplan B3 · Operation-/Op-Modell

| | |
|---|---|
| **Paket** | B3 (Gruppe B) |
| **Abhängt von** | B1 (Queue/Node), B2 (`Region`/`DeviceKind`) |
| **Blockiert** | C1 (Fassade), C2 (Processor/Op-Queues), C3 (Planner-Ausgabe) |
| **Ergebnis** | Op-/Value-Vertrag: `Op`-Trait, `OpParams`, `Requirements`, `Value`, `Lease`, impliziter Graph via Reihenfolge |
| **Quellen** | `../resident_planner.md` (Ausgabe `Operation`), `../resident_subengine.md` (Task ≠ Op, §4.2 Region), `../resident_processor.md` (Datenübergabe/Leases), `../context/context_b.md` §B3 (12–14) |

---

## 1. Zweck & Abgrenzung

`Operation` ist die **datenhafte Beschreibung** eines Schritts auf einem Device. Sie trägt unveränderliche `params` mit `requires()` (Ressourcenart, Modell, Index, erlaubte Engine), ein `materialize: bool` und `Value`-Ein/Ausgaben. Der **Graph ist implizit** durch die Reihenfolge der Operationen eines Tasks.

**Scharfe Trennung (`resident_subengine.md`):**
- **Task** = Nutzeranfrage (Host-Typen, Daten + Parameter), Queue-Knoten mit `Arc<TaskState>` (B1).
- **Op** = Schritt auf einem Device (Regions + Layouts). Ein **Planner** (C3) expandiert Task → Pipeline aus Ops. Op bleibt **reine Beschreibung/Daten**.

**Nicht Teil von B3:** Expansionsregeln (C3), Worker-Loop/Completion/Backpressure (C2). B3 liefert die Typen, die Planner erzeugt und Processor/Subengine konsumieren.

---

## 2. Zu implementierende Typen

### 2.1 Op-Trait (Op als Daten)

```rust
pub trait Op: Send + Sync {
    fn kind(&self) -> OpKind;        // stabiler Bezeichner
    fn params(&self) -> &OpParams;   // unveränderlich; Clone + Hash + Eq
    fn requires(&self) -> Requirements;
}

#[non_exhaustive]
pub enum OpKind { Tokenize, Embed, Pool, L2Norm, Similarity, TopK, WriteIndex, Infer /* … */ }
```
Muster (Vorbild candle `UnaryOp`/`BinaryOp`): Op als **Enum** oder `Arc<dyn Op>` + `params`. Der Scheduler matcht `requires() ⊆ worker.capabilities()` (Node-Selector-Prinzip).

### 2.2 Requirements & ResourceKind

```rust
#[non_exhaustive]
pub struct Requirements {
    pub resource: ResourceKind,       // Tokenizer | Encoder | Scorer | Index | CrossEncoder …
    pub model:    Option<ModelId>,
    pub index:    Option<IndexId>,
    pub engine:   EngineSelector,     // Cpu | Gpu | Either (requires() bestimmt Engine eindeutig)
    pub caps:     CapabilitySet,      // aus B6 (leer erlaubt)
    pub affinity: Option<Affinity>,   // Erweiterungspunkt, s. u.
}

#[non_exhaustive]
pub enum ResourceKind { Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder }

#[non_exhaustive]
pub enum EngineSelector { Cpu, Gpu, Either }
```
- Device-Wahl (Planner, C3): Stage ohne Device-Festlegung trägt nur die **Ressourcenart**; die Zuordnung entscheidet der **Claim** der Subengine. Derzeit: zwei Engines, jede Ressourcenart höchstens einmal pro Device → `requires()` bestimmt die Engine eindeutig.
- **Affinität = Erweiterungspunkt** (keine aktive Logik in Phase 1):
  ```rust
  #[non_exhaustive]
  pub enum Affinity { Handle(Region), Device(DeviceId) }
  ```
  Scheduler fragt nur `requires()`; neue Varianten ändern **Fassade/Task nicht** (context_b §B3.14).

### 2.3 Value & Lease (RAII)

```rust
pub enum Value { Host(HostData), Device(DeviceHandle) }
```
- **Kein `Tensor` im `Value`.** `DeviceHandle` ist eine **Lease** auf eine `Region` (B2), kein Tensor.
- Transfers sind **explizit** als eigene Op modelliert (`into_host`/Device→Host nur über eine Op), um ungewollte Rücktransfers in Hot-Paths zu vermeiden.
- **Lease als RAII** (bevorzugt vor manuellen Zählern):
  ```rust
  pub struct Lease { handle: Region, pin: Arc<PinCount> }   // Drop dekrementiert PinCount
  ```
  - Evict nur bei `pin == 0` (Processor/Subengine). Erzeugung prüft `gen` (Debug-Assertion beim Claim, kein eigener Pfad).
  - **`Drop` darf nicht blockieren/awaiten:** asynchrone Freigabe über eine Freigabe-Queue (Channel), nicht Arbeit im `Drop`.
  - Offenes Handle = Lease auf Arena **und** Ressource, Use-Zähler > 0 → Ressource wird nicht evicted. Lease endet mit der letzten konsumierenden Operation oder im Cancel-/Timeout-Pfad.

### 2.4 Operation-Knoten (Instanziierung von B1)

```rust
pub struct OperationNode {
    pub op:       Arc<dyn Op>,
    pub input:    Value,
    pub output:   OnceLock<Value>,     // gesetzt beim Fertigwerden
    pub materialize: bool,             // Ergebnis in Host zurückholen (true) oder Device-Handle halten (false)
    pub task:     Arc<TaskCtx>,        // gemeinsamer Zähler offener Ops + Task-ID/Priorität
}
```
- `materialize` setzt der Planner (C3): `true`, wenn CPU-Nachfolger / Task-Ende / Index-Spiegelung; `false`, wenn GPU-Nachfolger (Handle bleibt im VRAM).
- Abschluss: `TaskCtx` führt den Zähler offener Operationen; bei 0 → Task `Done`. Erster Fehler → `Cancelled` + `Err`, übrige Ops scheitern beim Claim.
- Als Queue-Payload: `Queue<OpParams, Value>` (B1), eingereiht in die passende Op-Queue (CPU/GPU/shared) durch C2.

---

## 3. Fehler & Logging

- Q1: `Incompatible` (Capability nicht erfüllbar), Config-/Planner-Fehler (ungültige Params). B3 definiert die Varianten, die C3 wirft.
- Q2: `tracing`-Span pro Task-ID; Events: Op erzeugt (kind, requires), materialize-Entscheidung, Lease auf/zu.

## 4. Teststrategie (Q3, ohne GPU)

- `requires()` ⊆ Capabilities: Match true/false (nutzt B6 `CapabilitySet::satisfies`).
- `OpParams`: `Clone`/`Hash`/`Eq` stabil (für Cache-/Graph-Schlüssel).
- **Lease-RAII:** Drop dekrementiert `PinCount`; Evict erst bei 0; Early-Return/Panic lässt keinen Pin hängen.
- `Value`: kein impliziter Device→Host-Transfer; Transfer nur über explizite Op.
- Affinität: neue `Affinity`-Variante bricht Fassaden-/Task-API nicht (`#[non_exhaustive]`-Testfall).
- Op-Zähler: Pipeline aus n Ops → Task erst bei 0 `Done`; erster Fehler cancelt Rest.

## 5. Offene Punkte (→ A3 / C3)

1. `Op` als geschlossenes Enum vs. `Arc<dyn Op>` (Dispatch-Kosten vs. Offenheit) — context_b empfiehlt datenhaftes Enum, `dyn` nur für Laufzeitwahl.
2. Handle-/Lease-Repräsentation final an B2 `Region`+`gen` koppeln.
3. Freigabe-Queue-Mechanik für async Lease-Drop (Channel-Typ) mit C2 abstimmen.

## 6. Definition of Done

- `Op`-Trait, `OpKind`, `OpParams`, `Requirements`/`ResourceKind`/`EngineSelector`/`Affinity`, `Value`, `Lease`, `OperationNode` implementiert.
- Kein `Tensor`/Pointer in `Value`/öffentlicher API; Lease-RAII getestet.
- Erweiterungspunkt Affinität vorhanden, aber inaktiv; baut ohne `cuda`/`candle`.
