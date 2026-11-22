# Spezifikation: Residency-Steuerung (Build-Order 125)

Crate-Modul `residency`.

## 1. Task, restrictions and boundaries

**Aufgabe**

- Indizes registrieren, ohne etwas zu laden (`register`).
- Beim ersten Bedarf das Backend eines Index anlegen, genau einmal je Index (`ensure`). Das Backend ist der GPU-Manager samt Koordinator.
- `ensure` und `reconcile` an das Backend weiterreichen, je Index serialisiert.
- Die Gap-Arbeit aller aktiven Backends bündeln (`run_gap`).
- Einen Index abmelden und sein Backend herunterfahren (`unregister`).

**Grenzen**

- **Lazy:** `register` ruft weder `IndexManager::index` oder `activate` noch die Backend-Fabrik auf. Es liest nichts aus dem Index und allokiert nichts. Nur `ensure` legt das Backend an. `reconcile`, `pending`, `is_resident` und `run_gap` aktivieren nie.
- Keine Segmente, keine Slots, kein Mapping, keine Eviction, keine Update-Queue, kein Reconcile-Vergleich und keine Tabellen-Publikation. Das alles gehört dem GPU-Manager (138) und dem Koordinator (140), die das `ResidencyBackend` implementieren. Dieses Modul kennt `Manifest`, `AddressMap`, `MappingStore`, `UpdateQueue`, `SegmentStore` und `TablePublisher` nicht.
- Keine Scope-Auflösung. Sie liegt im Backend (`ScopeResolverRegistry`).
- Keine Policy, wann `ensure`, `reconcile` oder `run_gap` aufgerufen werden. Keine automatische Verdrängung. Verdrängen ist Sache der `Eviction` im Backend.
- Es gibt keine Threads, Timer oder Hintergrundarbeit. Das Modul schreibt nie in den Index.
- Kein `unsafe`. Typen der Abhängigkeiten werden nicht neu definiert.

## 2. Context

- Pro registriertem Index gibt es einen Eintrag mit dem `IndexManager` (er hält den Index lazy, 050), der `IndexSpec` und dem Backend. Das Backend entsteht erst beim ersten `ensure`.
- Der Aufrufer legt den `IndexManager` an und hängt den Index an, wenn ein Task ihn braucht. Er ist die einzige Quelle des Index für das Backend.
- Das Backend wird von einer Fabrik gebaut, die die Subengine von außen liefert. Die Fabrik baut für einen Index den `GpuManager` (138) und den `IndexOperations`-Koordinator (140) mit den gemeinsamen Strukturen der Subengine.
- Der Index gewinnt immer. Stale Reads und Torn Reads gegenüber nebenläufigen Writes sind akzeptiert (Konsistenzmodell).
- Die Reihenfolge beim Laden (erst Daten in Slots schreiben, dann das Segment veröffentlichen) ist eine Eigenschaft des Backends.
- Dieses Modul hängt nur von Typen ab, die vor 125 definiert sind. Das Backend ist ein Trait, damit 140 es später implementieren kann.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::EntryId;
use scopes::Scope;
use manager_lifecycle::IndexManager;
use resource_requirements::IndexId;
use slot_layout::SlotClass;
use layout::DType;

/// Geometrie und Segmentgröße eines Index. Wird unverändert an die Fabrik gereicht.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexSpec {
    pub class: SlotClass,      // Token-Obergrenze je Slot
    pub dim: u32,              // > 0
    pub dtype: DType,
    pub segment_bytes: u64,    // Länge je angefordertem Segment, >= Slot-Größe
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EnsureSummary {
    pub loaded: u64,
    pub already_resident: u64,
    pub missing: u64,          // im Index nicht vorhanden
    pub failed: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ReconcileSummary {
    pub removed: u64,
    pub reloaded: u64,
    pub current: usize,
}

/// Pro Index, wird von 140 implementiert. Send + Sync, alle Methoden &self.
pub trait ResidencyBackend: Send + Sync {
    /// Löst den Scope auf und lädt die noch nicht residenten Entries.
    fn ensure(&self, scope: &Scope) -> Result<EnsureSummary, String>;
    /// Ein Abgleich mit dem Index. Ok(None): der Index existiert nicht mehr.
    fn reconcile(&self) -> Result<Option<ReconcileSummary>, String>;
    /// Steht ein ungesehenes Index-Signal an? Blockiert nie.
    fn pending(&self) -> bool;
    fn is_resident(&self, entry: EntryId) -> bool;
    /// Gap-Arbeit. Rückgabe: Zahl der erledigten Einheiten.
    fn run_gap(&self) -> usize;
    /// Gibt alle Segmente frei. Rückgabe: Zahl der freigegebenen Segmente.
    fn shutdown(&self) -> Result<usize, String>;
}

/// Von der Subengine geliefert.
pub trait BackendFactory: Send + Sync {
    fn create(&self, id: &IndexId, spec: &IndexSpec, index: Arc<IndexManager>)
        -> Result<Arc<dyn ResidencyBackend>, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidencyError {
    UnknownIndex(IndexId),
    AlreadyRegistered(IndexId),
    InvalidSpec,                         // dim == 0 oder segment_bytes < Slot-Größe
    Backend(String),
}
impl std::fmt::Display for ResidencyError {}
impl std::error::Error for ResidencyError {}

/// Send + Sync; alle Methoden &self.
pub struct ResidencyController { /* Registry RwLock<HashMap<IndexId, Arc<Slot>>> */ }

impl ResidencyController {
    pub fn new(factory: Arc<dyn BackendFactory>) -> ResidencyController;

    /// Registriert, lädt nichts, aktiviert nichts.
    pub fn register(&self, id: IndexId, index: Arc<IndexManager>, spec: IndexSpec) -> Result<(), ResidencyError>;
    /// Fährt das Backend herunter (falls aktiv) und entfernt die Registrierung. Bei Fehler bleibt sie bestehen.
    pub fn unregister(&self, id: &IndexId) -> Result<usize, ResidencyError>;

    /// Legt beim ersten Aufruf das Backend an und lädt die noch nicht residenten Entries des Scopes.
    pub fn ensure(&self, id: &IndexId, scope: &Scope) -> Result<EnsureSummary, ResidencyError>;
    /// Gleicht ab, falls das Backend aktiv ist. Sonst ein leeres Ergebnis ohne Aktivierung.
    pub fn reconcile(&self, id: &IndexId) -> Result<ReconcileSummary, ResidencyError>;
    pub fn pending(&self, id: &IndexId) -> Result<bool, ResidencyError>;

    /// In der Gap zwischen Batches aufrufen. Summe der `run_gap`-Ergebnisse aller aktiven Backends.
    pub fn run_gap(&self) -> usize;

    pub fn is_resident(&self, id: &IndexId, entry: EntryId) -> bool;
    pub fn is_active(&self, id: &IndexId) -> bool;
    pub fn registered_count(&self) -> usize;
    pub fn active_count(&self) -> usize;
}
```

`ResidencyController`, `IndexSpec`, die Summaries und `ResidencyError` sind `Send + Sync` (statisch geprüft).

## 4. Behavior and invariants

**register / unregister**

- `dim == 0` oder `segment_bytes < slot_layout::slot_size(class, dim, dtype)` (oder ein Fehler daraus) ergibt `InvalidSpec`.
- Eine bereits registrierte ID ergibt `AlreadyRegistered`.
- `register` ruft weder `IndexManager::index` noch `BackendFactory::create` auf. Danach ist `is_active == false`, `is_resident == false` für alle Entries, und `active_count` ist unverändert.
- Jeder Aufruf mit unbekannter ID ergibt `UnknownIndex`, außer `is_resident` und `is_active`, die `false` liefern.
- `unregister` ruft `backend.shutdown()`, falls das Backend aktiv ist, und gibt dessen Zahl zurück (sonst 0). Ein Fehler ergibt `Backend(msg)`, und die Registrierung bleibt bestehen.

**ensure(id, scope)**

1. Der Eintrag wird unter dem Registry-Lock nur geklont. Der Index-Lock des Eintrags serialisiert `ensure` und `reconcile` dieses Index. Verschiedene Indizes laufen parallel.
2. Ist noch kein Backend da, ruft `ensure` die Fabrik genau einmal auf (unter dem Index-Lock, sodass gleichzeitige Aufrufer dasselbe Backend bekommen).
   - Ein Fabrikfehler ergibt `Backend(msg)`. Er wird nicht gecacht, und der nächste Aufruf versucht es erneut.
3. `backend.ensure(scope)`. Ein Fehler ergibt `Backend(msg)`.
4. Wiederholtes `ensure` desselben Scopes lädt nichts neu (`loaded == 0`).

**reconcile / pending / is\_resident / run\_gap**

- `reconcile` ruft `backend.reconcile()` nur bei aktivem Backend. Ohne Backend ist das Ergebnis `ReconcileSummary::default()`, ohne Aktivierung. `Ok(None)` des Backends (Index nicht mehr vorhanden) wird zu `ReconcileSummary::default()`.
- `pending` ist `backend.pending()` und blockiert nie. Ohne Backend ist es `false`.
- `is_resident` ist `backend.is_resident(entry)`. Ohne Backend ist es `false`.
- `run_gap` ruft `backend.run_gap()` für jedes aktive Backend und summiert. Es ist idempotent und parallel sicher.

**Nebenläufigkeit und Fehler**

- Lock-Reihenfolge: Registry-Lock (nur kurz, Arc klonen) → Index-Lock des Eintrags → Aufrufe im Backend. Unter dem Registry-Lock laufen keine Aufrufe in andere Module.
- `run_gap` und `pending` nehmen den Index-Lock nicht, damit sie nie auf ein laufendes `ensure` warten.
- Vergiftete Locks werden übernommen. Es gibt keine Panics bei gültigen Eingaben.

## 5. Relied-on elements

- **`index_contract`:** `EntryId(pub u64)`.
- **`manager_lifecycle` (050):** `IndexManager` (`Send + Sync`) mit `attach(Arc<dyn Index>) -> bool`, `index() -> Result<Arc<dyn Index>, ManagerError>` und `run_cycle`. Es hält den Index lazy. `register` ruft nichts davon auf.
- **`scopes`:** `Scope` (`All`, `Range`, `Set`, `Subtree`, `Prefix`, `Centroid`; `#[non_exhaustive]`).
- **`resource_requirements`:** `IndexId` (`Clone + Eq + Hash`).
- **`slot_layout`:** `SlotClass` und `slot_size(class, dim, dtype) -> Result<u32, SlotError>`.
- **`layout`:** `DType`.
- **Nicht genutzt (nur Abgrenzung):** `manifest`, `address_map`, `update_queue`, `segments`, `consistency`, `table_swap`.
- **Logging:** `log`-Crate (`warn!`).
