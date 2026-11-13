# Spezifikation: Lade-Aggregation und Single-Flight (Build-Order 124)

Crate-Modul `load_aggregation`.

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- **Aggregation:** Requests gegen denselben Index werden über die Union ihrer Scopes gebündelt. Jeder Index wird genau einmal geladen, die Indizes nebenläufig. Der Collector sammelt die Requests.
- **Single-Flight:** Gleichzeitige Requests mit gleichem Key teilen einen Ladevorgang samt Ergebnis, auch im Fehlerfall.

**Grenzen**
- Das Modul hat keine eigene Scope-Logik, keine Queue, keinen Scheduler, keinen Cache und keine Residency- oder Reconcile-Logik. Es spawnt in `SingleFlight` nichts. Die Start-Funktion des Aufrufers startet den Ladevorgang.
- Es schreibt nie in den Index.
- Kein `unsafe`.
- Abhängigkeiten: `std`, `tracing`, `scopes`, `scope_resolution`, `index_contract`, `reconcile`, `task_model`.

## 2. Kontext

- Der Manager sammelt Load-Requests (Index-Key + `Scope`) im `Collector`.
  - `drain` liefert die Requests.
  - `load_aggregated` löst die Scopes je Index über `ScopeResolverRegistry::resolve` auf und bildet die Union (`EntrySet::union`).
  - Danach lädt es genau einmal pro Index mit `scope_resolution::load_candidates`.
- `SingleFlight` vereint gleichzeitige Ladevorgänge gleichen Keys. Ein Flight ist intern ein `Task<(), V, E>` aus dem Task-Modell. Joiner warten über `Task::wait`.
- `load_aggregated` ist blockierend und nutzt `std::thread::scope`. Der Aufrufer ruft es aus einem eigenen Worker auf, den die Start-Funktion von `SingleFlight` anstößt.

## 3. Interface

```rust
use std::{collections::HashMap, fmt::Debug, hash::Hash, sync::Arc};
use index_contract::{EntryId, Index};
use reconcile::LoadedEntry;
use scope_resolution::{CellSelector, ResolveError, ScopeResolverRegistry};
use scopes::Scope;
use task_model::{Task, TaskError};

// ---------- Aggregation ----------
#[derive(Clone, Debug, PartialEq)]
pub struct LoadRequest<I> { pub index: I, pub scope: Scope }

/// Thread-sicher (&self). Send + Sync, wenn I: Send.
pub struct Collector<I> { /* private */ }
impl<I> Collector<I> {
    pub fn new() -> Self;
    pub fn push(&self, req: LoadRequest<I>);
    /// Anzahl gesammelter Requests (nicht der Indizes).
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// Entnimmt atomar alle Requests in Einfügereihenfolge; danach ist der Collector leer.
    pub fn drain(&self) -> Vec<LoadRequest<I>>;
}
impl<I> Default for Collector<I> {}   // == new()

/// Geladene Entries eines Index, aufsteigend nach ID; `None` = im Index fehlend.
pub type Loaded = Vec<(EntryId, Option<LoadedEntry>)>;

#[derive(Debug, Clone)]
pub struct IndexLoad<I> { pub index: I, pub result: Result<Loaded, ResolveError> }

/// Blockiert, bis alle Indizes fertig sind. Ergebnis: ein Eintrag je distinktem Index,
/// in Reihenfolge des ersten Auftretens in `requests`. Leere Eingabe -> leerer Vec.
pub fn load_aggregated<I>(
    requests: Vec<LoadRequest<I>>,
    registry: &ScopeResolverRegistry,
    lookup: &(dyn Fn(&I) -> Option<Arc<dyn Index>> + Sync),
    cell_selector: Option<&dyn CellSelector>,
) -> Vec<IndexLoad<I>>
where I: Clone + Eq + Hash + Send + Sync;

// ---------- Single-Flight ----------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightRole { Starter, Joiner }

/// Handle auf einen laufenden oder beendeten Flight. Clone = gleicher Flight.
pub struct Flight<V, E> { /* role + Task<(), V, E> */ }
impl<V, E> Clone for Flight<V, E> {}
impl<V, E> Flight<V, E> {
    pub fn role(&self) -> FlightRole;
    /// Task-Modell-Handle (für result()/notifier()).
    pub fn task(&self) -> &Task<(), V, E>;
    /// Wie Task::wait: Ok(&V) | Err(Failed(&E)) | Err(Cancelled).
    pub async fn wait(&self) -> Result<&V, TaskError<&E>>;
}

/// Wird dem Starter übergeben; Send + 'static, wenn K,V,E es sind.
pub struct FlightCompleter<K, V, E> { /* private */ }
impl<K, V, E> FlightCompleter<K, V, E> {
    /// Entfernt den eigenen Map-Eintrag, danach Ergebnis setzen und Wartende wecken.
    pub fn finish(self, result: Result<V, E>);
}
// Drop ohne finish: eigener Eintrag wird entfernt, der Task cancelt (Wartende: Err(Cancelled)).

/// Cheap Clone (Arc). Send + Sync, wenn K: Send, V,E: Send + Sync.
pub struct SingleFlight<K, V, E> { /* private */ }
impl<K, V, E> Clone for SingleFlight<K, V, E> {}
impl<K: Eq + Hash + Clone + Debug, V, E> SingleFlight<K, V, E> {
    pub fn new() -> Self;
    /// Existiert ein laufender Flight für `key`: Joiner (start wird NICHT aufgerufen).
    /// Sonst: neuen Flight registrieren, `start(completer)` genau einmal aufrufen, Role Starter.
    /// `start` MUSS non-blocking sein (z. B. Worker/Thread anstoßen) und darf `finish` auch später aufrufen.
    pub fn join_or_start<S>(&self, key: K, start: S) -> Flight<V, E>
    where S: FnOnce(FlightCompleter<K, V, E>);
    /// Anzahl laufender Flights.
    pub fn in_flight(&self) -> usize;
    pub fn is_in_flight(&self, key: &K) -> bool;
    pub fn keys_in_flight(&self) -> Vec<K>;
}
impl<K: Eq + Hash + Clone + Debug, V, E> Default for SingleFlight<K, V, E> {}
```

## 4. Behavior und Invarianten

**Collector**
- `len()` zählt jeden `push` seit dem letzten `drain`, Duplikate eingeschlossen. `is_empty()` ⇔ `len() == 0`.
- `drain` liefert die Requests in `push`-Reihenfolge und leert den Collector atomar. Nebenläufige `push`-Aufrufe gehen weder verloren noch werden sie doppelt geliefert. Ein zweites `drain` ergibt einen leeren Vec.

**load_aggregated**
1. Requests werden nach `index` gruppiert (`Eq + Hash`). Die Reihenfolge der Ergebnisse folgt dem ersten Auftreten.
2. Pro Index gilt:
   - Der `lookup` wird genau einmal aufgerufen. Bei `None` ist das Ergebnis `Err(ResolveError::IndexUnavailable)`.
   - Jeder Scope der Gruppe wird mit `registry.resolve(scope, &ResolveContext{ index: Some(..), cell_selector })` aufgelöst, in Request-Reihenfolge.
   - Die Ergebnisse werden per `EntrySet::union` vereinigt.
   - Schlägt eine Auflösung fehl, ist das Ergebnis dieses Index der erste Fehler unverändert. Es wird nichts geladen.
   - Sonst läuft genau ein `load_candidates(index, &union, ..)` pro Index. Jede ID der Union erscheint genau einmal im `Loaded`, aufsteigend. Eine leere Union ergibt `Ok(vec![])` ohne `load_candidates`-Aufruf.
3. Die Indizes laufen nebenläufig, je einer in einem eigenen Thread (`std::thread::scope`). Ein Fehler in einem Index beeinflusst die anderen nicht.
4. Es gibt keinen Fallback auf alle Entries. Ein Panic in einem Thread wird propagiert.
5. Pro Index wird `tracing::debug!` mit der Anzahl der gebündelten Requests und der Größe der Union geloggt.

**SingleFlight**
- Pro Key existiert höchstens ein Flight. `join_or_start` prüft und registriert unter einem kurzen Mutex. `start` läuft außerhalb des Locks, nach der Registrierung. Genau ein nebenläufiger Aufrufer wird `Starter`, alle anderen `Joiner`.
- Der Starter ruft `start` genau einmal auf. Joiner rufen `start` nie auf. Der Aufruf von `join_or_start` blockiert nie auf den Ladevorgang selbst.
- Alle Handles eines Flights sehen dasselbe Ergebnis. Das gilt auch im Fehlerfall: `Err(Failed(&E))` wird geteilt, `E` wird nicht geklont.
- Jeder Flight hat eine eindeutige, monoton vergebene interne ID.
  - `finish` entfernt den Map-Eintrag nur, wenn der Eintrag zum Key diese ID trägt. Ein neuerer Flight desselben Keys bleibt unberührt.
  - Danach führt es `claim()` und `complete(result)` auf dem Task aus. Der Eintrag ist also vor dem Wecken der Wartenden entfernt.
- Ein neuer Aufrufer nach `finish` startet einen neuen Flight. Damit wird ein Fehlschlag vom nächsten neuen Aufrufer wiederholt. Ein Ergebnis wird nach Abschluss nicht gecacht.
- Wird der `FlightCompleter` ohne `finish` gedroppt, auch bei Panic in `start`, gilt:
  - Der eigene Eintrag wird entfernt (nur bei passender ID).
  - Der Task wird gecancelt, Wartende erhalten `Err(TaskError::Cancelled)`.
  - Der Panic in `start` selbst wird an den Aufrufer propagiert.
- Abfragen (`in_flight`, `is_in_flight`, `keys_in_flight`) sind Momentaufnahmen. Sie zählen nur Flights, die noch nicht beendet oder entfernt sind.
- Beim Beitritt eines Joiners loggt `tracing::debug!` das Ereignis `single_flight joiner` mit `key = {:?}`. Beim Start wird nichts als Joiner geloggt.
- Nebenläufigkeit: alle Methoden sind `&self` und thread-sicher. Das Lock wird nie über `start`, `finish` oder `wait` gehalten. Das Droppen eines `wait`-Futures ist sicher und beeinflusst den Flight nicht.
- Es gibt keine eigenen Fehlertypen in `SingleFlight`. Der Fehler `E` kommt vom Aufrufer. `Cancelled` zeigt einen abgebrochenen Starter an.

## 5. Relied-on elements

**scopes**
- `Scope` (`Clone + Debug + PartialEq`, `non_exhaustive`).
- `EntrySet::union(&self, &EntrySet) -> EntrySet`, `is_empty`, `iter`. Immutable, `Send + Sync`.

**scope_resolution**
- `ScopeResolverRegistry::resolve(&self, &Scope, &ResolveContext<'_>) -> Result<EntrySet, ResolveError>`.
  - Sie ist `&self` und `Send + Sync`.
  - Ohne `ctx.index` kommt `IndexUnavailable`.
  - Es gibt kein Ausweichen auf alle Entries.
- `ResolveContext<'a> { index: Option<&'a dyn Index>, cell_selector: Option<&'a dyn CellSelector> }`.
- `ResolveError` ist `Debug + Clone + PartialEq`, mit der Variante `IndexUnavailable`.
- `trait CellSelector: Send + Sync`.
- `pub fn load_candidates<F>(index: &dyn Index, candidates: &EntrySet, f: F) where F: FnMut(index_contract::EntryId, Option<LoadedEntry>)`.
  - Sie lädt genau die Mitglieder der Menge, aufsteigend.
  - Jede ID erhält genau einen Aufruf von `f`.
  - `None` heißt: Entry fehlt im Index.
  - Eine leere Menge macht keinen Index-Zugriff.

**index_contract**
- `trait Index: Send + Sync`. `EntryId(pub u64)`.

**reconcile**
- `LoadedEntry` ist `Clone + Send + Sync`.

**task_model**
- `Task::<D,T,E>::new(data, TaskParams::default())`.
- Methoden: `claim() -> Result<D, ClaimError>`, `complete(Result<T,E>) -> Result<(), CompleteError>`, `cancel() -> bool`, `async wait(&self) -> Result<&T, TaskError<&E>>`, `result()`, `notifier()`.
- `TaskError<E> { Cancelled, Failed(E) }`.
- `Task` ist `Clone` (gleicher Task) und `Send + Sync`, wenn `D: Send, T,E: Send + Sync`.
- Eine zweite Completion führt im Debug-Build zu `debug_assert!`. `finish` darf sie daher nie zweimal auslösen (`finish` konsumiert `self`).
- Ein `cancel` ist nur im Zustand `Enqueued` erfolgreich. Der Flight-Task bleibt bis `finish` im Zustand `Enqueued`.