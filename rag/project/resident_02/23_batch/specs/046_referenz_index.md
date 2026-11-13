# Spezifikation: Referenz-Index (Build-Order 46)

## 1. Task, restrictions and boundaries

**Task:** Das Crate-Modul `reference_index` liefert `ReferenceIndex`. Das ist eine konkrete Implementierung des Traits `index_contract::Index`, die vollständig im RAM liegt.

**Restrictions:**
- Keine GPU-Beschleunigung, keine Machine-Learning-Bibliotheken, kein `unsafe`.
- Erlaubte Abhängigkeiten: `std`, `index_contract`, `layout` (nur `DType`), `change_stamp` (`StampCounter`) und `scopes` (`EntrySet`).
- Keine Persistenz, kein IO, keine Suche und keine Vektorkonvertierung. Der Index speichert Bytes unverändert.
- Es werden keine Zugriffsmethoden deklariert. `access_methods()` ist leer.
- Alle optionalen Methoden (`summary`, `children`, `root`, `cell_entries`, `byte_ranges`) bleiben beim Default aus dem Vertrag und liefern `Err(Unsupported(..))`. Sie werden nicht überschrieben.

## 2. Context

- Der Vertrag `Index` (Build-Order 33) wird hier unverändert erfüllt. Verbraucher wie Retrieval, Sync und Cache nutzen den Index über `Arc<dyn Index>` aus mehreren Threads.
- Stamps kommen aus `StampCounter` (Build-Order 15). Der Index wandelt sie per `Stamp(change_stamp.as_raw())` in den Vertragstyp um.
- `scopes::EntrySet` (Build-Order 16) dient nur als Rückgabetyp der Zusatzmethode `entry_set`.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::*;
use layout::DType;

pub struct ReferenceIndex { /* privat */ }

impl ReferenceIndex {
    /// Leerer Index mit festem dtype. Max-Stamp des Signals = Stamp(0).
    pub fn new(dtype: DType) -> Self;
    /// Bequemlichkeit: Arc<ReferenceIndex>.
    pub fn new_shared(dtype: DType) -> Arc<ReferenceIndex>;
    /// Snapshot aller vorhandenen EntryIds als EntrySet (IDs als u64 aus EntryId.0).
    /// Berührt keine Vektordaten.
    pub fn entry_set(&self) -> scopes::EntrySet;
    /// Anzahl vorhandener Entries.
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl Index for ReferenceIndex { /* alle Pflichtmethoden gemäß Vertrag; optionale Methoden nicht überschrieben */ }
impl Default for ReferenceIndex {} // nicht vorhanden: new braucht dtype
```
`ReferenceIndex: Send + Sync`. Ein statischer Test prüft das.

## 4. Behavior and invariants

**Zustand**
- Die Entries liegen in einer geordneten Map `EntryId → (Stamp, Payload)` hinter einem `RwLock`. Stamp und Payload eines Entries stehen im selben Map-Eintrag. Sie werden nur gemeinsam unter dem Write-Lock verändert.
- Ein `Mutex` serialisiert alle Writer (`write`, `delete`).
- Pro Index gibt es genau einen `StampCounter::new()` und eine `StampSignal::new(Stamp(0))`.
- Der Index nutzt keine Hintergrund-Threads.

**Statische Eigenschaften**
- `dtype()` liefert den Wert aus `new`. `access_methods()` liefert `&[]`.
- `subscribe()` gibt `signal.observer()` zurück. Ein neuer Observer startet mit dem aktuellen Max-Stamp.

**write(id, payload)**
1. Prüfung: `vector.len() % dtype.size_in_bytes() != 0` ergibt `Err(InvalidVectorLength { len, elem_size })`. Zustand, Zähler und Signal bleiben unverändert. Ein leerer Vektor ist erlaubt. `meta` ist beliebig.
2. Im exklusiven Writer-Abschnitt:
   - `counter.next()` vergeben.
   - `Err(Exhausted)` wird zu `Err(IndexError::Backend(<Display-Text>))`. Es ändert sich nichts.
   - Vektor und Meta einmalig in `Arc<[u8]>` kopieren.
   - Unter dem Map-Write-Lock den Eintrag (Stamp, Payload) ersetzen oder einfügen.
   - Das Map-Lock freigeben, dann `signal.publish(stamp)`. Die Reihenfolge Daten, Stamp, Signal ist verbindlich.
3. Rückgabe: genau der Stamp dieses Writes. Er ist `> Stamp(0)`.

**delete(id)**
- Fehlt der Entry, ist das ein No-op mit `Ok(None)`. Es wird kein Stamp vergeben und das Signal bleibt unverändert. Die Prüfung erfolgt im Writer-Abschnitt, damit sie nicht mit einem parallelen Write racet.
- Sonst im Writer-Abschnitt: Stamp vergeben (Fehler wie bei `write`), Eintrag entfernen, publizieren. Rückgabe `Ok(Some(delete_stamp))`.

**Stamps**
- Stamps werden im Writer-Abschnitt vergeben. Die Reihenfolge der Stamps entspricht daher der Reihenfolge der Sichtbarkeit. Sie sind streng monoton und werden nie wiederverwendet.
- Der Endzustand eines Entries entspricht dem Write mit dem höchsten Stamp (Last-Writer-Wins).
- `signal.current()` ist immer der höchste publizierte Stamp.

**Lesen**
- `read` nimmt kurz das Read-Lock und klont die beiden `Arc`-Payload-Handles (billig). Fehlt der Entry, liefert es `Payload::default()`. Kein Mischzustand, und ein gelesener `Payload` ändert sich nie.
- `stamp(id)` und `stamps()` lesen nur Stamps. `stamps()` ist ein Snapshot, aufsteigend nach `EntryId` sortiert (Map-Reihenfolge) und ohne Duplikate.
- Ist ein Stamp sichtbar, liefert `read` Daten dieses oder eines späteren Writes.

**write_many**
- Das ist der Default aus dem Vertrag: sequentielle Einzel-`write`-Aufrufe, Abbruch beim ersten Fehler. Er ist nicht atomar. Bereits geschriebene Entries bleiben bestehen.

**entry_set / len**
- Beide sind konsistente Snapshots unter dem Read-Lock.
- `entry_set` enthält genau die IDs, die `stamps()` im selben Zustand liefern würde.

**Nebenläufigkeit**
- Alle Methoden nehmen `&self` und sind von beliebig vielen Threads aufrufbar, ohne externes Locking.
- Lesende Aufrufe blockieren nur kurz während der Map-Mutation. Der Writer-Mutex wird nie gehalten, während ein Observer wartet. Verklemmungen sind ausgeschlossen. Die Lock-Reihenfolge ist immer Writer-Mutex, dann Map-Lock, und `publish` läuft nach Freigabe des Map-Locks.
- Ein Observer hindert Writes nicht.
- Lock-Poisoning wird toleriert (`into_inner`). Die Methoden panicen nicht wegen Poisoning.

**Tests (Mindestumfang)**
- Write, Read und Replace.
- Delete vorhanden und nicht vorhanden. Beim nicht vorhandenen Entry bleibt der Max-Stamp gleich.
- `InvalidVectorLength` ohne Signaländerung.
- Streng monotone Stamps bei parallelen Writern aus mehreren Threads.
- Sortierung von `stamps()`.
- `wait_newer` wird nach einem Write geweckt.
- Die optionalen Methoden liefern `Unsupported(<Methode>)`.
- `Send + Sync`.

## 5. Relied-on elements

Aus `index_contract` (Build-Order 33), unverändert:
- Typen `EntryId(pub u64)`, `Stamp(pub u64)`, `SharedBytes(pub Arc<[u8]>)`, `PayloadRef<'a> { vector, meta }`, `Payload { vector, meta }` (mit `Default`), `AccessMethod`, `IndexError { InvalidVectorLength { len, elem_size }, Unsupported(AccessMethod), NotFound, Backend(String) }`, `StampSignal`, `StampObserver`.
- Trait `Index: Send + Sync` mit:
  - Pflichtmethoden: `dtype`, `access_methods`, `stamp`, `stamps`, `read`, `write`, `delete`, `subscribe`.
  - `write_many` mit Default-Implementierung.
  - Optionalen Methoden mit Default `Err(Unsupported(..))`.
- `StampSignal::new(Stamp) -> Arc<StampSignal>`, `publish(&self, Stamp)` (setzt max), `current()`, `observer(self: &Arc<Self>) -> StampObserver`.

Aus `change_stamp` (Build-Order 15):
- `StampCounter::new()`.
- `next(&self) -> Result<ChangeStamp, StampError>`. Fehler: `StampError::Exhausted`.
- `ChangeStamp::as_raw(&self) -> u64`. Vergebene Rohwerte sind ≥ 1.
- `StampError` implementiert `Display`.

Aus `scopes` (Build-Order 16):
- `EntrySet::from_ids<I: IntoIterator<Item = u64>>(ids) -> EntrySet`. `EntryId` ist dort der Alias `u64`, nicht der Newtype aus `index_contract`.

Aus `layout` (Build-Order 11):
- `DType` (`Copy`) mit `const fn size_in_bytes(self) -> usize`.