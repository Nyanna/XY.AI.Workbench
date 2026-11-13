# Spezifikation: Write- und Treffer-Queues (Build-Order 95)

Crate-Modul `hit_queues`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Das Modul führt zwei getrennte, beschränkte SPSC-Queues:
  - Die Write-Queue trägt Embedding-Writes (`EmbeddingWrite`).
  - Die Treffer-Queue trägt Kernel-Treffer (`KernelHit`).
- Jede Queue hat genau einen Producer und genau einen Consumer.
- Der Consumer entnimmt in einem Aufruf zuerst alle Writes und danach die Treffer. So wird kein Treffer ohne zugehörigen, bereits entnommenen Write beobachtet.
- Enqueue und Tiefe sind abfragbar und werden geloggt.

**Grenzen**
- Nicht Teil des Moduls:
  - Interpretation der Writes und Treffer. Das Modul wendet Writes nicht auf `AddressMap` oder `MappingStore` an und löst Treffer nicht auf.
  - Das Erzeugen von Writes und Treffern.
  - Top-k, Kernel, Eviction und Threads.
- Keine Persistenz und keine Blockierung. Es gibt kein Warten, kein Parken und keine Callbacks.
- Es gibt keine Multi-Producer-Variante.
- Abhängigkeiten:
  - `kernel_layout` (`KernelHit`, `Slot`)
  - `address_map` (`SegmentId`)
  - `index_contract` (`EntryId`)
  - das Crate `log` (Fassade)
- Die Typen der Abhängigkeiten werden nicht neu definiert.
- `unsafe` ist nur intern im Ring-Puffer erlaubt. Die öffentliche API ist safe.

## 2. Context

- Der Manager indexiert, während Suchen laufen.
- Ein neuer Embedding-Write legt fest, welcher `EntryId` in welchem Segment und Slot der `AddressMap` liegt. Das ist die Adresszuordnung.
- Der Kernel liefert Treffer als `KernelHit { addr, score }`. Die Auflösung eines Treffers auf einen Entry braucht die Adresszuordnung des zugehörigen Writes.
- Deshalb gilt Reihenfolge-Disziplin:
  - Der Write-Producer registriert den Write zuerst, der Treffer-Producer meldet erst danach.
  - Der Consumer liest die Writes vor den Treffern.
- Writes und Treffer können von verschiedenen Threads kommen. Je Queue gibt es genau einen Producer-Thread.

## 3. Interface

```rust
use index_contract::EntryId;
use kernel_layout::{KernelHit, Slot};
use address_map::SegmentId;

/// Adresszuordnung eines Embeddings: Entry liegt in (segment, slot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmbeddingWrite { pub entry: EntryId, pub segment: SegmentId, pub slot: Slot }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError { Full }   // Queue voll; Element wurde nicht aufgenommen

/// Fehler von push: gibt das Element zurück.
#[derive(Debug, PartialEq)]
pub struct PushError<T> { pub item: T, pub error: QueueError }
impl<T> std::fmt::Display for PushError<T> {}          // "queue full"
impl<T: std::fmt::Debug> std::error::Error for PushError<T> {}

/// Ergebnis eines Consumer-Durchlaufs.
#[derive(Debug, Default, PartialEq)]
pub struct Drained { pub writes: Vec<EmbeddingWrite>, pub hits: Vec<KernelHit> }

/// Erzeugt das Paar. Capacity je Queue (Elemente). capacity == 0 wird zu 1.
pub fn channel(write_capacity: usize, hit_capacity: usize)
    -> (WriteProducer, HitProducer, QueueConsumer);

/// Einziger Producer der Write-Queue. Nicht Clone. Send, nicht Sync-nötig.
pub struct WriteProducer { /* ... */ }
impl WriteProducer {
    pub fn push(&mut self, w: EmbeddingWrite) -> Result<(), PushError<EmbeddingWrite>>;
    /// Aktuelle Tiefe (Elemente in der Queue). &self.
    pub fn depth(&self) -> usize;
    pub fn capacity(&self) -> usize;
}

/// Einziger Producer der Treffer-Queue. Nicht Clone. Send.
pub struct HitProducer { /* ... */ }
impl HitProducer {
    pub fn push(&mut self, h: KernelHit) -> Result<(), PushError<KernelHit>>;
    pub fn depth(&self) -> usize;
    pub fn capacity(&self) -> usize;
}

/// Einziger Consumer beider Queues. Nicht Clone. Send.
pub struct QueueConsumer { /* ... */ }
impl QueueConsumer {
    /// Entnimmt zuerst alle Writes, dann die Treffer (siehe Abschnitt 4).
    pub fn drain(&mut self) -> Drained;
    pub fn write_depth(&self) -> usize;
    pub fn hit_depth(&self) -> usize;
}
```

`WriteProducer`, `HitProducer` und `QueueConsumer` sind `Send`. Das wird per statischem Test geprüft. Sie sind nicht `Clone`, und `push` und `drain` nehmen `&mut self`. Der Compiler erzwingt damit SPSC.

## 4. Behavior and invariants

**Queues**
- Jede Queue ist FIFO, beschränkt und lock-frei bzw. wait-free. `push` und `drain` blockieren nie.
- `push` auf eine volle Queue gibt `Err(PushError { item, error: QueueError::Full })` zurück. Das Element geht nicht verloren, und die Queue bleibt unverändert. Es wird nie überschrieben und nie verworfen.
- Elemente kommen unverändert und in Push-Reihenfolge beim Consumer an. Es gibt keine Verluste und keine Duplikate.
- Wird ein Endpunkt gedroppt, bleibt der andere nutzbar. Noch enthaltene Elemente bleiben per `drain` entnehmbar.

**Reihenfolge Write → Treffer**
- Vertrag für Aufrufer: Der Write wird per `WriteProducer::push` erfolgreich registriert, bevor der zugehörige Treffer per `HitProducer::push` gemeldet wird. Das Modul erzwingt das nicht, prüft keine Zuordnung und löst keine Treffer auf.
- `drain` arbeitet in genau dieser Reihenfolge:
  1. Es liest `n = hit_depth()` (Acquire-Snapshot der Treffer-Queue).
  2. Es entnimmt alle Writes, die danach in der Write-Queue sichtbar sind (bis leer).
  3. Es entnimmt genau `n` Treffer.
- Garantie: Ist ein Treffer in `Drained.hits` enthalten, dann war der zuvor registrierte Write schon in diesem oder einem früheren `drain` in `writes`.
- Treffer, die nach Schritt 1 eintreffen, bleiben für den nächsten `drain` in der Queue.
- Writes dürfen in `writes` ohne Treffer auftauchen. Treffer ohne Write sind nur möglich, wenn der Aufrufer den Vertrag verletzt hat.
- `drain` auf leere Queues gibt `Drained::default()` zurück. Es gibt keinen Fehler und kein Warten.
- Indexieren während der Suche: Neue Writes dürfen jederzeit pushen. Sie werden spätestens im `drain` entnommen, der die Treffer entnimmt, die nach ihnen registriert wurden.

**Tiefe und Logging**
- `depth()`, `write_depth()` und `hit_depth()` liefern die Anzahl der Elemente in der Queue. Mit gleichzeitigem Betrieb ist der Wert eine momentane Näherung, die nie größer als `capacity()` ist. Im Single-Thread-Betrieb ist er exakt.
- Nach erfolgreichem `push` ist `depth()` im Single-Thread-Betrieb um 1 größer. Nach `drain` ist `write_depth() == 0`. `hit_depth()` ist 0, wenn nach dem Snapshot keine Treffer hinzukamen.
- Geloggt wird über die `log`-Fassade, Target `"hit_queues"`:
  - Erfolgreicher `push`: `debug!` mit Queue (`"write"` oder `"hit"`) und Tiefe nach dem Push.
  - `push` auf eine volle Queue: `warn!` mit Queue und Kapazität.
  - `drain`: `debug!` mit der Anzahl der entnommenen Writes und Treffer sowie den Resttiefen.
- Logging ändert weder Verhalten noch Rückgabewerte.

**Fehler und Randfälle**
- Einziger Fehler: `QueueError::Full`.
- `channel` mit Kapazität 0 nutzt 1. `capacity()` gibt die tatsächliche Kapazität zurück.
- Keine Panics bei gültiger Nutzung.

## 5. Relied-on elements

- `index_contract::EntryId(pub u64)`: `Copy`, `Eq`, `Send + Sync`.
- `kernel_layout::KernelHit { pub addr: Addr, pub score: f32 }`: `Debug, Clone, Copy, PartialEq`, `Send + Sync`.
- `kernel_layout::Slot(pub u32)`: `Copy`, `Eq`.
- `address_map::SegmentId(pub u32)`: `Copy`, `Eq`, `Send + Sync`.
- Die Auflösung `Slot` → `EntryId` oder `Addr` → `Entry` liegt bei Aufrufern über `SlotMap::resolve` bzw. `AddressMap::get`/`lookup`. Dieses Modul ruft sie nicht auf.