# Spezifikation: Manager-Lebenszyklus (Build-Order 50)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Das Crate-Modul `manager_lifecycle` stellt `IndexManager` bereit. Er verwaltet genau einen Index unter einer festen `IndexId` und tut dafür Folgendes:
- Er hängt den Index erst auf Anforderung an (Attach).
- Er aktiviert den Manager einmalig mit einer Init-Funktion (Once-Semantik).
- Er macht Aktivität und Zahl residenter Entries über eine Schnittstelle abfragbar.
- Er führt Zyklen in einem Diagnose-Span mit der Index-ID aus.

**Grenzen:**
- Keine eigene Index-Implementierung, keine Suche, keine Persistenz, kein Scheduling. Der Manager startet keine Zyklen selbst. Aufrufer rufen `run_cycle` auf.
- Kein Detach, keine Index-Factory und keine Retry-Logik. Die Aufrufer entscheiden, wann sie `activate` erneut aufrufen.
- Der Manager erzeugt und hält keine Subscriptions. Er ruft `Index::subscribe` nicht auf.
- Externe Abhängigkeit: nur `tracing` (Spans und Warnungen). Außerdem `index_contract` und `std`.

## 2. Kontext
- Der Manager ist ein Verbraucher des `index_contract::Index`-Traits, das nur über `Arc<dyn Index>` genutzt wird.
- Ein Task, der den Index braucht, hängt ihn per `attach` an und aktiviert den Manager per `activate`. Danach führt er Arbeit über `run_cycle` aus.
- Bis zum Attach hält der Manager nur die `IndexId` und den Zustand. Er hält keinen Index, keine Ressourcen und keine Subscriptions.
- `IndexManager` ist `Send + Sync`. Er wird typischerweise als `Arc<IndexManager>` geteilt.

## 3. Interface
```rust
use std::sync::Arc;
use index_contract::Index;

/// Kennung des Index. Billig zu klonen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndexId(pub Arc<str>);
impl std::fmt::Display for IndexId { /* gibt den Inhalt aus */ }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagerError {
    /// Kein Index angehängt.
    IndexUnavailable { index_id: IndexId },
    /// Init-Funktion schlug fehl (Text aus ihrem Err).
    InitFailed { index_id: IndexId, reason: String },
}
impl std::fmt::Display for ManagerError { /* Text enthält index_id */ }
impl std::error::Error for ManagerError {}

/// Momentaufnahme von Aktivität und Residenz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagerStatus {
    pub index_id: IndexId,
    pub attached: bool,
    pub active: bool,
    /// Anzahl residenter Entries = `index.stamps().len()`; 0 ohne angehängten Index.
    pub resident_entries: usize,
}

pub struct IndexManager { /* intern */ }

impl IndexManager {
    pub fn new(index_id: IndexId) -> Self;          // nicht angehängt, inaktiv
    pub fn index_id(&self) -> &IndexId;

    /// true = angenommen. Ist schon ein Index angehängt: bleibt er, false, `tracing::warn!` mit index_id.
    pub fn attach(&self, index: Arc<dyn Index>) -> bool;

    /// Der angehängte Index, sonst `IndexUnavailable { index_id }`.
    pub fn index(&self) -> Result<Arc<dyn Index>, ManagerError>;

    /// Once-Aktivierung. `init` erhält den angehängten Index.
    pub fn activate<F>(&self, init: F) -> Result<(), ManagerError>
    where F: FnOnce(&Arc<dyn Index>) -> Result<(), String>;

    /// Aktivität und residente Entries aus einer Schnittstelle.
    pub fn status(&self) -> ManagerStatus;

    /// Führt `f` in einem Diagnose-Span aus (siehe §4).
    pub fn run_cycle<R, F>(&self, name: &str, f: F) -> Result<R, ManagerError>
    where F: FnOnce(&Arc<dyn Index>) -> R;
}
```
`IndexManager: Send + Sync`.

## 4. Behavior und Invarianten
**Attach**
- Nach `new` gilt `attached == false`, `active == false`, `resident_entries == 0`.
- Das erste `attach` liefert `true` und speichert den Index.
- Jedes weitere `attach` liefert `false`, ändert nichts und gibt eine `warn!`-Meldung mit der Index-ID aus. Das gilt auch, wenn der Manager schon aktiv ist, und auch für denselben `Arc`.
- Bei gleichzeitigen `attach`-Aufrufen gewinnt genau einer mit `true`.

**Zugriff**
- `index()`, `activate` und `run_cycle` ohne angehängten Index liefern `Err(IndexUnavailable { index_id })`. `f` und `init` werden dann nicht aufgerufen.

**Aktivierung (Once)**
- Zustände: Inaktiv → Initialisierend → Aktiv.
- `activate` im Zustand Aktiv liefert sofort `Ok(())`. `init` wird nicht erneut aufgerufen.
- Im Zustand Inaktiv führt genau ein Aufrufer sein `init` aus. Während der Initialisierung blockieren gleichzeitige Aufrufer, ohne ihr eigenes `init` zu nutzen, und warten auf dasselbe Ergebnis.
- Bei `Ok(())` von `init` wird der Manager aktiv, und alle Wartenden erhalten `Ok(())`.
- Bei `Err(reason)` oder bei einem Panic in `init` bleibt der Manager inaktiv. Der Initialisierer und alle zu diesem Versuch Wartenden erhalten `Err(InitFailed { index_id, reason })`. Bei einem Panic lautet `reason` "init panicked", und das Panic wird nicht weitergereicht. Ein späterer `activate`-Aufruf startet einen neuen Versuch.
- Ein Fehlschlag hinterlässt keinen Halbzustand: `status().active == false`.
- `init` läuft ohne gehaltene interne Sperre, die `status`, `index` oder `attach` blockiert.

**Status**
- `status()` blockiert nicht auf eine laufende Initialisierung. Währenddessen gilt `active == false`.
- `resident_entries` kommt aus `Index::stamps().len()`. Das berührt keine Vektordaten und ist ein Snapshot.
- `status()` enthält immer die `index_id`.

**Zyklen**
- `run_cycle` öffnet ein `tracing`-Span mit dem Namen `"index_cycle"` und den Feldern `index_id` (Display der ID) und `cycle` (= `name`). `f` läuft innerhalb des Spans, und der Span endet danach, auch bei Panic von `f`.
- Panics von `f` werden nicht abgefangen.
- `run_cycle` setzt keine Aktivierung voraus. Es verlangt nur einen angehängten Index.
- Gleichzeitige `run_cycle`-Aufrufe sind erlaubt und werden nicht serialisiert. Kein Aufruf hält eine interne Sperre, während `f` läuft.

**Nebenläufigkeit:** Alle Methoden sind `&self` und von beliebig vielen Threads aufrufbar. Verklemmungen sind ausgeschlossen, weil kein Callback (`init`, `f`) unter einer internen Sperre läuft.

## 5. Relied-on elements
Aus `index_contract`, unverändert genutzt:
```rust
pub trait Index: Send + Sync {
    fn stamps(&self) -> Vec<(EntryId, Stamp)>; // Snapshot, aufsteigend nach EntryId, ohne Duplikate, berührt keine Vektordaten
    fn subscribe(&self) -> StampObserver;      // vom Manager NICHT aufgerufen
    /* weitere Methoden unverändert, hier ungenutzt */
}
```
Nutzung: `Arc<dyn Index>` (`Index` ist object-safe, `Send + Sync`). Alle `&self`-Methoden sind nebenläufig aufrufbar, ohne externes Locking.