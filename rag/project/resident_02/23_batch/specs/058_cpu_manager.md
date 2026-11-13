# Spezifikation: Index-Residency (Build-Order 58)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Das Crate-Modul `index_residency` liefert `IndexResidency`. Es hat zwei Aufgaben:
- Es sichert die Residency dateigestützter Index-Daten im Page-Cache und markiert Bereiche nach Leerlauf als „cold“.
- Es liest einzelne und mehrere Entries mit Zugriffsmuster im Blocking-Pool.

**Grenzen:**
- `IndexResidency` ist eine Komponente der CPU-Subengine, weder der Fassade noch des Index. Es enthält keine Suche, keinen Index, keine Persistenz und keine Schreibpfade.
- Das Modul speichert keine Referenz auf die CPU-Subengine, um Zyklen zu vermeiden. Die Subengine konstruiert es und übergibt den lazy Index.
- Es startet keinen eigenen Timer und keinen Hintergrund-Thread. Die Leerlauf-Markierung läuft nur, wenn der Eigentümer `mark_idle_cold` aufruft.
- Die Hinweisquelle (`MemoryAdvisor`) wird von außen injiziert. Das Modul ruft selbst kein `madvise` auf und erfindet keine Mmap-Zugriffe.
- Es gibt keine weiteren Verhaltensweisen als die hier beschriebenen.

## 2. Kontext

- Das RAM ist die Primärinstanz. Lesezugriffe gehen direkt an den `Index` (`index_contract::Index`, `Arc<dyn Index>`). Der Index ist typischerweise ein `FileIndex` (Parquet, mmap) und kann größer als das RAM sein. Die Page-Cache-Residency wird deshalb über Hinweise gesteuert.
- Der Index wird lazy aktiviert. Der Manager hält einen geteilten Slot, der erst später gefüllt wird. Bis dahin ist der Index „nicht angehängt“.
- `Index::read` darf blockieren (IO). Deshalb laufen alle Lesezugriffe über den Blocking-Pool (`tokio::task::spawn_blocking`).
- Ein Hinweisbereich ist ein `index_contract::ByteRange` (`offset`/`len` im Speicher des Index). Der Index liefert ihn über die optionale Zugriffsmethode `Range` (`byte_ranges`). `FileIndex` deklariert keine Zugriffsmethoden. Dann entfallen die Hinweise, und das ist kein Fehler.

## 3. Interface

```rust
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use index_contract::{ByteRange, EntryId, Index};

/// Lazy aktivierter Index: Slot wird später (einmalig) vom Eigentümer gefüllt.
pub type IndexSlot = Arc<OnceLock<Arc<dyn Index>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Advice { WillNeed, Sequential, Random, Cold }

/// Hinweisquelle (z. B. mmap-Region des Index). Setzt den Hinweis für [offset, offset+len).
pub trait MemoryAdvisor: Send + Sync {
    fn advise(&self, offset: u64, len: u64, advice: Advice) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferError(pub String);
impl std::fmt::Display for TransferError { /* schreibt self.0 */ }
impl std::error::Error for TransferError {}

/// Kopie der Entry-Bytes (nicht geteilt). Fehlender Entry: beide leer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OwnedPayload { pub vector: Vec<u8>, pub meta: Vec<u8> }

pub struct IndexResidency { /* privat */ }   // Send + Sync

impl IndexResidency {
    /// `advisor = None`: Hinweise sind No-ops. `idle_after`: Leerlaufschwelle für Cold-Markierung.
    pub fn new(index: IndexSlot, advisor: Option<Arc<dyn MemoryAdvisor>>, idle_after: Duration) -> IndexResidency;

    /// Direkter, synchroner Durchgriff auf den Index (kein Blocking-Pool, keine Hinweise).
    pub fn read_direct(&self, id: EntryId) -> Option<index_contract::Payload>;   // None: Index nicht angehängt

    /// Sichert Residency: Hinweis WillNeed auf `range`, Bereich wird als „zuletzt genutzt jetzt“ verfolgt.
    pub fn secure_residency(&self, range: ByteRange);

    /// Markiert alle verfolgten Bereiche mit `now - letzte Nutzung >= idle_after` als Cold
    /// und stellt die Verfolgung dieser Bereiche ein. Rückgabe: Anzahl markierter Bereiche.
    pub fn mark_idle_cold(&self, now: Instant) -> usize;

    /// Einzelnes Lesen. `hint`: Hinweisbereich des Entries, falls bekannt.
    pub async fn read_entry(&self, id: EntryId, hint: Option<ByteRange>) -> Result<OwnedPayload, TransferError>;

    /// Mehrfaches Lesen mit Zugriffsmuster. Jeder Eintrag trägt seinen Hinweisbereich (oder None).
    pub async fn read_entries(&self, entries: &[(EntryId, Option<ByteRange>)]) -> Result<Vec<OwnedPayload>, TransferError>;

    /// Variante: Hinweisbereiche werden vom Index erfragt (`byte_ranges`).
    pub async fn read_entries_auto(&self, ids: &[EntryId]) -> Result<Vec<OwnedPayload>, TransferError>;
}
```

## 4. Behavior und Invarianten

**Allgemein**
- Alle Methoden sind von beliebig vielen Tasks/Threads gleichzeitig aufrufbar, ohne externes Locking.
- Hinweisfehler (`Err` der `MemoryAdvisor`) werden nur mit Warn-Log gemeldet. Sie lassen nie ein Lesen scheitern und nie `mark_idle_cold` abbrechen.
- Ohne Advisor (`None`) passiert bei Hinweisen und Cold-Markierung nichts. Das ist kein Fehler.
- Der Index wird erst bei der Operation aus dem Slot gelesen (`OnceLock::get`). Ein später gefüllter Slot ist ab dann wirksam.
- `read_direct` ruft `Index::read` auf dem aufrufenden Thread auf. Es liefert `None`, wenn der Index nicht angehängt ist.

**Residency und Cold**
- `secure_residency(r)` ruft `advise(r.offset, r.len, WillNeed)` auf und setzt die Nutzungszeit von `r` auf `Instant::now()`. Der Schlüssel ist `(offset, len)`.
- Jeder in Lesezugriffen mit `WillNeed`, `Sequential` oder `Random` hinterlegte Hinweisbereich wird ebenfalls so verfolgt und seine Nutzungszeit aktualisiert.
- `mark_idle_cold(now)` ruft für jeden verfolgten Bereich mit `now.saturating_duration_since(letzte Nutzung) >= idle_after` `advise(.., Cold)` auf und entfernt ihn aus der Verfolgung.
- Auch bei Ablehnung durch den Advisor wird der Bereich entfernt. Die Ablehnung wird nur geloggt.
- Ohne Advisor liefert `mark_idle_cold` 0 und ändert nichts.
- Bereiche mit `len == 0` werden weder gehintet noch verfolgt.

**read_entry**
1. Ist `hint` gesetzt, wird `WillNeed` darauf gesetzt (siehe oben).
2. `Index::read(id)` läuft in `spawn_blocking`.
3. Das Ergebnis wird in `OwnedPayload` kopiert (Vektor und Meta). Ein fehlender Entry ergibt zwei leere Vektoren und ist kein Fehler.

- Ist der Index nicht angehängt, ist das Ergebnis `Err(TransferError("Index nicht angehängt"))`.
- Wird die Blocking-Task abgebrochen oder ist sie panicked (`JoinError`), ist das Ergebnis `Err(TransferError("Lese-Task abgebrochen"))`. Der Text ist exakt so.

**read_entries**
- Bei leerer Eingabe ist das Ergebnis `Ok(vec![])`. Das gilt vor der Prüfung des Index und ohne Hinweise.
- Bei nicht-leerer Eingabe und nicht angehängtem Index ist das Ergebnis `Err(TransferError("Index nicht angehängt"))`.
- Das Ergebnis hat genau `entries.len()` Elemente in Eingabereihenfolge, auch bei Duplikaten (jedes Duplikat wird geliefert). Fehlende Entries ergeben ein leeres `OwnedPayload` an ihrer Stelle.
- Zugriffsmuster, bestimmt aus den Einträgen mit `Some(range)` und `len > 0`:
  - `span` = `[min offset, max (offset+len))`.
  - `covered` = Länge der Vereinigung aller Bereiche (Überlappungen und Duplikate zählen einmal).
  - **dicht**, wenn `covered * 2 >= span.len` ist. Dann gibt es genau einen Hinweis `Sequential` auf den gesamten `span`.
  - **verstreut**, sonst. Dann gibt es je distinktem Bereich einen Hinweis `Random`.
  - Ohne Bereiche gibt es keinen Hinweis.
- Alle Hinweise werden vor dem Lesen gesetzt. Die Bereiche werden verfolgt, bei dichtem Muster der `span` als ein Bereich.
- Das Lesen erfolgt in einer einzigen `spawn_blocking`-Task, nacheinander in Eingabereihenfolge. Ein Abbruch ergibt `Err(TransferError("Lese-Task abgebrochen"))`.

**read_entries_auto**
- Bei leerer Eingabe ist das Ergebnis `Ok(vec![])`. Ist der Index nicht angehängt, gilt dieselbe Fehlerregel wie bei `read_entries`.
- In der Blocking-Task wird `Index::byte_ranges(min_id, max_id)` (inklusiv, über die Eingabe-IDs) aufgerufen, falls `AccessMethod::Range` in `access_methods()` steht.
- Ein Entry `id` gehört zu einem `ByteRange` `r`, wenn `r.first.0 <= id.0 < r.first.0 + r.count` gilt (saturierend). Sein Hinweisbereich ist dann `(r.offset, r.len)`.
- Entries ohne zusammenhängende Speicherung (kein `ByteRange` enthält sie) werden ohne Hinweis gelesen.
- Deklariert der Index `Range` nicht oder liefert `byte_ranges` einen Fehler, wird alles ohne Hinweis gelesen. Ein Warn-Log ist erlaubt, ein Fehlschlag nicht.
- Anschließend gelten Muster, Hinweise, Reihenfolge und Fehler wie bei `read_entries`. Hinweise werden vor den Lesezugriffen gesetzt, aber innerhalb derselben Blocking-Task.

## 5. Relied-on elements

Aus `index_contract` (unverändert):
```rust
pub struct EntryId(pub u64);                       // Ord, Copy
pub struct SharedBytes(pub Arc<[u8]>);             // Deref<Target=[u8]>
pub struct Payload { pub vector: SharedBytes, pub meta: SharedBytes }   // Default = beide leer
pub enum AccessMethod { Centroid, Prefix, Subtree, Range }
pub struct ByteRange { pub first: EntryId, pub count: u64, pub offset: u64, pub len: u64 }
pub enum IndexError { InvalidVectorLength{..}, Unsupported(AccessMethod), NotFound, Backend(String) }
pub trait Index: Send + Sync {
    fn access_methods(&self) -> &[AccessMethod];
    fn read(&self, id: EntryId) -> Payload;        // darf blockieren; fehlend: Payload::default()
    fn byte_ranges(&self, first: EntryId, last: EntryId) -> Result<Vec<ByteRange>, IndexError>; // Range, inklusiv, aufsteigend nach first; Default Err(Unsupported(Range))
    /* weitere Methoden werden nicht genutzt */
}
```
- `ByteRange` beschreibt zusammenhängend gespeicherte Entries: `len` ist die Summe der Bytes, `count` die Anzahl, `offset` ist relativ zum Speicher des Index.
- `Index` ist object-safe und von mehreren Threads ohne Locking nutzbar.
- `FileIndex` (`file_index`) implementiert `Index` mit `access_methods() == &[]`. Dort liefert `byte_ranges` `Unsupported(Range)`, und alle Lesezugriffe laufen ohne Hinweise. `read` liefert bei IO- oder Dekodierfehlern `Payload::default()`.
- Der Index ist ein reiner Index-Vertrag. Weder `FileIndex` noch der Vertrag bieten eine eigene mmap- oder `madvise`-Schnittstelle. Deshalb stammt die Hinweisquelle von außen (`MemoryAdvisor`).