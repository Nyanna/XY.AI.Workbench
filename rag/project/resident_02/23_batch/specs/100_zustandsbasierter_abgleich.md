# Spezifikation: Zustandsbasierter Abgleich (Build-Order 100)

Crate-Modul `reconcile`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Je Subscriber einen eigenen `Reconciler` bereitstellen. Er wartet auf das Index-Signal, quittiert es und vergleicht die residenten Entries mit dem Index.
- Das Ergebnis nennt entfernte Entries und Entries, die neu geladen werden müssen.
- Einen Entry stamp-konsistent laden (`load_entry`).
- Ein `ResidencyDirectory` führen, das je residentem Entry Slot und bekannten Stamp hält.

**Grenzen**
- Das Modul enthält keine GPU-, CPU- oder Persistenz-Logik und keinen Upload. Es enthält weder Slot-Zuweisung noch Eviction.
- Der Reconciler verändert das Directory nie. Der Manager pflegt es nach Reload, Entfernen und eigenem Upsert.
- Es gibt keine Update-Queue-Typen. Entfernungen werden nur als Liste und über einen Callback geliefert.
- Das Modul schreibt nie in den Index.
- Es gibt kein `unsafe` und keine Abhängigkeiten außer `std`, `index_contract`, `change_stamp` und `kernel_layout::Slot`.

## 2. Context

- Jeder Subscriber (GPU-Manager, Persistenz-Timer, CPU-Manager) besitzt einen eigenen `Reconciler` und ein eigenes `ResidencyDirectory`. Die Sichten sind unabhängig.
- Das Signal (`StampObserver`) ist ein Level und weckt nur. Verglichen werden ausschließlich residente Entries. Nicht geladene Entries brauchen keine Behandlung, weil das Laden den aktuellen Stand liest. Der State skaliert mit dem geladenen Teil.
- Der Index gewinnt immer. Ein abweichender Stamp bedeutet Reload. Das gilt auch für Neuanlage nach Löschen, weil Stamps nie wiederverwendet werden.
- Zwei Manager mit demselben Entry konvergieren beide auf den Index-Stand. Der jeweils andere sieht einen abweichenden Stamp und lädt nach.
- Stamps werden nur auf Gleichheit verglichen (`ChangeStamp` hat kein `Ord`). Die Umwandlung erfolgt über `ChangeStamp::from_raw(stamp.0)`.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::{Index, EntryId, Stamp, Payload};
use change_stamp::ChangeStamp;
use kernel_layout::Slot;

// ---------- Residency-Verzeichnis ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotRef { pub segment: SegmentId, pub slot: Slot }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resident { pub slot: SlotRef, pub stamp: ChangeStamp } // stamp = bekannter Stamp

/// Send + Sync, alle Methoden &self (intern synchronisiert).
#[derive(Debug, Default)]
pub struct ResidencyDirectory { /* Lock<BTreeMap<EntryId, Resident>> */ }
impl ResidencyDirectory {
    pub fn new() -> Self;
    /// Fügt ein oder ersetzt; gibt den Vorgänger zurück.
    pub fn insert(&self, id: EntryId, slot: SlotRef, known: Stamp) -> Option<Resident>;
    pub fn remove(&self, id: EntryId) -> Option<Resident>;
    pub fn get(&self, id: EntryId) -> Option<Resident>;
    pub fn stamp(&self, id: EntryId) -> Option<ChangeStamp>;
    /// Echo-Suppression: trägt den Rückgabe-Stamp des eigenen Upserts als bekannt ein.
    /// Slot bleibt unverändert. false, wenn `id` nicht resident ist (dann keine Änderung).
    pub fn set_known_stamp(&self, id: EntryId, known: Stamp) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// Snapshot, aufsteigend nach EntryId.
    pub fn iter(&self) -> Vec<(EntryId, Resident)>;
}

// ---------- Laden ----------
pub const MAX_LOAD_RETRIES: u32 = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedEntry { pub stamp: Stamp, pub payload: Payload, pub retries: u32 }

/// Reihenfolge: stamp → read → stamp. None, wenn der Entry fehlt.
pub fn load_entry(index: &dyn Index, id: EntryId) -> Option<LoadedEntry>;

// ---------- Abgleich ----------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciliation {
    pub removed: Vec<EntryId>,   // aufsteigend
    pub reload: Vec<EntryId>,    // aufsteigend
    pub current: usize,          // Anzahl aktueller Entries
    pub watermark: Stamp,        // Signal-Stamp, der quittiert wurde
    pub clean: bool,             // removed.is_empty() && reload.is_empty()
}
impl Reconciliation {
    /// Ruft `f` je entferntem Entry in aufsteigender Reihenfolge auf (zum Einstellen von Delete-Updates).
    pub fn for_each_delete<F: FnMut(EntryId)>(&self, f: F);
}

/// Send + Sync. Eigene Sicht je Subscriber.
pub struct Reconciler { /* Weak<dyn Index>, StampObserver, seen: Stamp */ }
impl Reconciler {
    /// Abonniert den Index (`subscribe()`); `seen` = aktueller Max-Stamp zum Zeitpunkt des Abos.
    /// Hält den Index nur schwach (`Arc::downgrade`).
    pub fn new(index: &Arc<dyn Index>) -> Reconciler;
    /// Blockiert, bis ein ungesehenes Signal ansteht (true) oder der Index nicht mehr existiert (false).
    /// Quittiert NICHT.
    pub fn wait(&mut self) -> bool;
    /// Polling: steht ein ungesehenes Signal an (observer.current() != seen, d. h. > seen)? Blockiert nie.
    pub fn pending(&self) -> bool;
    /// Quittiert das Signal (Ack) und vergleicht danach. None, wenn der Index nicht mehr existiert.
    pub fn reconcile(&mut self, dir: &ResidencyDirectory) -> Option<Reconciliation>;
}
```

## 4. Behavior and invariants

**Signal / wait / pending**
- `wait` prüft zuerst, ob der Index noch existiert (`Weak::upgrade`). Falls nicht, gibt es `false` zurück.
- Ist `observer.current() > seen`, gibt `wait` sofort `true` zurück.
- Sonst wartet `wait` mit einem Clone des Observers in Scheiben von 100 ms (`wait_newer(Some(100ms))`). Nach jeder Scheibe prüft es erneut die Existenz des Index. Dadurch endet das Warten auch dann, wenn nie wieder publiziert wird.
- `wait` verändert `seen` nicht, es quittiert also nicht. Mehrfaches `wait` ohne `reconcile` kehrt sofort mit `true` zurück.
- `pending` ist `observer.current() > seen`. Es blockiert nicht und braucht den Index nicht.
- Koaleszierte Signale sind erlaubt. Ein Signal ist ein Level, kein Zähler.

**reconcile**
1. Existenz prüfen. Ist der Index weg, kommt `None` zurück.
2. Ack zuerst, vor jeder Index-Abfrage zum Vergleich. Dazu wird `observer.wait_newer(Some(Duration::ZERO))` aufgerufen.
   - `Some(w)`: `seen = watermark = w`.
   - `None`: `watermark = seen`.
3. Danach Vergleich. Ein Signal, das während oder nach dem Ack eintrifft, ist > `seen`. `pending()` und `wait()` melden es, und es löst den nächsten Abgleich aus. Es geht nie verloren.
4. `reconcile` ist auch ohne anstehendes Signal zulässig und vergleicht dann trotzdem.

**Vergleich**
- Für jeden Eintrag von `dir.iter()` (aufsteigend) gilt:
  - `index.stamp(id) == None` → `removed`.
  - Stamp ungleich dem bekannten (`ChangeStamp::from_raw(s.0) != resident.stamp`) → `reload`. Das gilt auch, wenn der Index-Stamp numerisch kleiner ist.
  - Stamp gleich → aktuell, `current += 1`. Das schließt Echos eigener Writes ein, deren Stamp per `insert` oder `set_known_stamp` eingetragen wurde.
- Der Index-Zugriff nutzt entweder einen `stamps()`-Snapshot (Binärsuche oder Merge) oder `stamp(id)` je Entry. Beide liefern dasselbe Ergebnis. Es werden nie Vektordaten gelesen.
- `removed.len() + reload.len() + current == dir.len()` zum Zeitpunkt des `iter()`-Snapshots.
- `removed` und `reload` sind disjunkt und aufsteigend sortiert.
- `clean == removed.is_empty() && reload.is_empty()`.
- Nicht residente Entries werden nie betrachtet. Das Directory wird nicht verändert.
- Ein Entry, der gelöscht und mit gleicher ID neu angelegt wurde, hat einen anderen Stamp und erscheint in `reload`, nicht in `removed`. Wurde er zwischen zwei Abgleichen nur gelöscht, erscheint er in `removed`.
- `for_each_delete` liefert genau die IDs aus `removed`.
- Ein leeres Directory ergibt eine leere, `clean`e Reconciliation mit `current == 0`.

**load_entry**
- Ablauf je Versuch:
  1. `s1 = index.stamp(id)`. Bei `None` kommt sofort `None` zurück, auch in einem Retry.
  2. `payload = index.read(id)`.
  3. `s2 = index.stamp(id)`.
- Gilt `s2 == Some(s1)`, kommt `LoadedEntry { stamp: s1, payload, retries }` zurück. `retries` ist die Zahl der bisherigen Wiederholungen.
- Sonst (Stamp bewegt, auch `s2 == None`) folgt eine Wiederholung. Es gibt höchstens `MAX_LOAD_RETRIES = 8` Wiederholungen, also höchstens 9 Versuche.
- Nach der letzten Wiederholung ohne Stabilität kommt `LoadedEntry { stamp: erster s1 des allerersten Versuchs, payload: zuletzt gelesen, retries: 8 }` zurück.
- Der zurückgegebene Stamp ist nie neuer als die Daten. Ein veralteter Stamp führt höchstens zu einem überzähligen Reload, was harmlos ist.
- Die umgekehrte Reihenfolge (Daten, dann Stamp) ist verboten, weil sie stale Daten als aktuell ausweisen würde.
- `load_entry` ist lock-frei und kann blockieren, weil `read` blockieren darf. Es hält keine eigenen Locks.

**Echo-Suppression**
- Der Manager trägt den Stamp aus dem Rückgabewert des eigenen `write` per `insert` oder `set_known_stamp` ein. Ein Stamp aus einem späteren `stamp()` oder `read()` darf nie als Echo eingetragen werden.
- Schreibt ein anderer Writer nach dem eigenen Write, weicht der Index-Stamp ab. Das ergibt `reload`.
- Fehlt der Rückgabewert, wird nichts eingetragen. Das führt höchstens zu einem redundanten Upload.

**ResidencyDirectory**
- Es hält höchstens einen Eintrag je `EntryId`. `insert` ersetzt und gibt den Vorgänger zurück (`None`, wenn neu). `remove` gibt den entfernten Eintrag zurück.
- `len`, `is_empty` und `iter` sind konsistent zu einem Zeitpunkt. `iter` ist ein Snapshot, spätere Änderungen verändern ihn nicht.
- Ein vergifteter Lock wird übernommen (`into_inner`). Das Modul paniciert nie bei gültigen Eingaben.
- `ResidencyDirectory`, `Reconciler`, `Reconciliation` und `LoadedEntry` sind `Send + Sync`. Das wird per statischem Test geprüft.

## 5. Relied-on elements

**index_contract**
- `EntryId(pub u64)` ist `Copy + Ord + Hash`. `Stamp(pub u64)` ist `Copy + Eq + Ord`. `Payload { vector: SharedBytes, meta: SharedBytes }` ist `Clone`.
- `Index: Send + Sync` mit:
  - `stamp(&self, EntryId) -> Option<Stamp>`
  - `stamps(&self) -> Vec<(EntryId, Stamp)>`: Snapshot, aufsteigend nach `EntryId`, ohne Duplikate, ohne Vektordaten.
  - `read(&self, EntryId) -> Payload`: darf blockieren, bei fehlendem Entry leer.
  - `subscribe(&self) -> StampObserver`
- Reihenfolge im Index: Daten sichtbar, dann Stamp, dann Signal. Ein sichtbarer Stamp garantiert, dass `read` Daten dieses oder eines späteren Writes liefert.
- Stamps sind streng monoton und werden nie wiederverwendet. `write` gibt den Stamp genau dieses Writes zurück.
- `StampObserver` ist `Clone + Send + Sync` mit:
  - `current(&self) -> Stamp`
  - `wait_newer(&mut self, timeout: Option<Duration>) -> Option<Stamp>`: wartet auf Signal > gesehen, setzt "gesehen" auf den Rückgabewert, `None` bei Timeout.
- Das Signal ist ein monotoner Level. Ein neuer Observer startet mit dem aktuellen Max-Stamp.

**change_stamp**
- `ChangeStamp` ist `Clone, Copy, PartialEq, Eq, Hash, Debug`, aber nicht `Ord`.
- `ChangeStamp::from_raw(u64)` und `as_raw(&self)`.

**kernel_layout**
- `Slot(pub u32)` ist `Copy + Eq + Hash + Debug`.
- `Slot` und `SegmentId` bilden zusammen die Position im Residency-Verzeichnis. Segmentgröße und Slot-Layout (`slot_layout`) werden hier nicht benötigt.

**consistency**
- Die Identität `(EntryId, ChangeStamp)` und "der Index gewinnt" gelten unverändert. Dieses Modul nutzt `consistency` nicht direkt.