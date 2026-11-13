# Spezifikation: Index-Öffnung und Lazy Loading (Build-Order 47)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul `index_open`. Es öffnet eine Index-Datei beim ersten Zugriff und liest Header, Schema, IDs und Stamps ohne Vektor-Dekodierung ein. Optional verifiziert es dabei die ganze Datei. Vektoren und Nutzdaten dekodiert es beim ersten Gebrauch, je Batch genau einmal.

**Nicht Teil des Moduls:**
- Das Datei- und Spaltenformat. Header-Parsing, Spaltendekodierung und Checksum-Berechnung liefert ein injizierter `FormatDecoder` (§3). Das Modul erfindet kein Format.
- Schreiben, Löschen, Publication und Signal.
- Die Implementierung von `Index` (Allgemeiner Index-Vertrag). Das Modul liefert den geladenen, lesbaren Zustand. Der konkrete Index delegiert an ihn.
- Alle anderen Fachmodule.

**Crates:** `memmap2`, `tracing`, `thiserror`. Interne Module: `page_advice`, `change_stamp` (`StampCounter`) und `index_contract` (nur `EntryId`, `Stamp`, `SharedBytes`, `Payload`).

## 2. Kontext

- Ein Index lebt in einer Datei. Konkrete Indizes (Read-Pfad, Retrieval, Cache) rufen `stamp`, `stamps`, `read` oft auf. `stamp` und `stamps` dürfen keine Vektordaten berühren. Deshalb lädt dieses Modul ID- und Stamp-Spalte getrennt und früh, Vektoren und Nutzdaten spät.
- Der Stamp-Zähler (`StampCounter`) wird beim Öffnen mit `resume` fortgesetzt. Der Write-Pfad besitzt ihn danach.
- Der Allgemeine Index-Vertrag verlangt: `read` bei fehlendem Entry liefert leere `Payload`, `read` darf blockieren, `stamp` und `stamps` sind aufsteigend nach `EntryId` sortiert und duplikatfrei. Dieses Modul garantiert Letzteres, indem es doppelte IDs beim Öffnen ablehnt.
- Async-Aufrufer führen die blockierenden Methoden dieses Moduls über `page_advice::read_blocking` aus.

## 3. Interface

```rust
use std::{path::{Path, PathBuf}, sync::Arc};
use index_contract::{EntryId, Stamp, SharedBytes, Payload};
use change_stamp::StampCounter;

pub const BATCH_ROWS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OpenError {
    /// "Index nicht verfügbar": Datei-/Lesefehler, Format-/Checksum-Fehler, doppelte IDs.
    #[error("Index nicht verfügbar: {path}: {cause}")]
    Unavailable { path: PathBuf, cause: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind { Mmap, FileRead }

#[derive(Debug, Clone)]
pub struct OpenOptions {
    pub use_mmap: bool,  // Default true; bei mmap-Fehler Fallback auf FileRead
    pub verify: bool,    // Default true
}
impl Default for OpenOptions { /* use_mmap: true, verify: true */ }

/// Vom Format-Modul gelieferte Dateiinfo (ohne Vektordaten).
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub schema_version: u32,
    pub model_id: String,
    pub model_revision: String,
    pub row_groups: usize,
    pub rows: u64,
    pub stored_max_stamp: Option<u64>, // im Header gespeicherter Max-Stamp
}

/// Dekodierte Zeilen eines Batches, Zeilenindex relativ zum Batch-Anfang.
pub struct DecodedBatch {
    pub vectors: Vec<SharedBytes>, // je Zeile FP16, Little-Endian (leer = Zeile ohne Vektor)
    pub metas: Vec<SharedBytes>,   // je Zeile Nutzdaten
}

/// Injiziert, Send + Sync. Fehler sind Klartext.
pub trait FormatDecoder: Send + Sync {
    fn read_info(&self, bytes: &[u8]) -> Result<FileInfo, String>;
    /// Alle (ID, Stamp) in Dateireihenfolge, nur ID-/Stamp-Spalten gelesen.
    fn read_ids_stamps(&self, bytes: &[u8], info: &FileInfo) -> Result<Vec<(u64, u64)>, String>;
    /// Dekodiert die Zeilen [start, end) vollständig (Vektor + Nutzdaten).
    fn decode_rows(&self, bytes: &[u8], info: &FileInfo, start: u64, end: u64) -> Result<DecodedBatch, String>;
    /// Dekodiert alles und kontrolliert alle Checksums; Err = erster Fehler.
    fn verify(&self, bytes: &[u8], info: &FileInfo) -> Result<(), String>;
}

/// Lazy geöffneter Index. Send + Sync. Alle Methoden &self.
pub struct LazyIndexFile { /* path, decoder, options, OnceLock<Result<Loaded, OpenError>> */ }

impl LazyIndexFile {
    /// Öffnet NICHT. Kein IO.
    pub fn new(path: impl Into<PathBuf>, decoder: Arc<dyn FormatDecoder>, options: OpenOptions) -> Self;
    pub fn path(&self) -> &Path;

    /// Lädt beim ersten Aufruf (genau einmal, auch bei Nebenläufigkeit); liefert danach das gemerkte Ergebnis.
    pub fn ensure_open(&self) -> Result<(), OpenError>;
    pub fn info(&self) -> Result<&FileInfo, OpenError>;
    pub fn access_kind(&self) -> Result<AccessKind, OpenError>;
    /// Stamp-Zähler, fortgesetzt via StampCounter::resume. Gehört dem Aufrufer-Index; Arc-geteilt.
    pub fn stamp_counter(&self) -> Result<Arc<StampCounter>, OpenError>;

    /// Ohne Vektor-Dekodierung. Aufsteigend nach EntryId.
    pub fn stamps(&self) -> Result<Vec<(EntryId, Stamp)>, OpenError>;
    pub fn stamp(&self, id: EntryId) -> Result<Option<Stamp>, OpenError>;

    /// Vektor (FP16 LE). Unbekannte ID: leer. Dekodier-Fehler: leer + error!-Log. Blockiert ggf.
    pub fn read_vector(&self, id: EntryId) -> SharedBytes;
    /// Nutzdaten. Unbekannte ID oder Dekodier-Fehler: None (letzterer mit error!-Log).
    pub fn read_meta(&self, id: EntryId) -> Option<SharedBytes>;
    /// Vektor + Nutzdaten als Payload; Teile wie oben (leer, wenn nicht vorhanden).
    pub fn read(&self, id: EntryId) -> Payload;
}
```

Zusicherung: `LazyIndexFile: Send + Sync` (Test per static assertion).

## 4. Verhalten und Invarianten

**Öffnen**
- `new` führt kein IO aus. Der erste Aufruf einer anderen Methode löst `ensure_open` aus. Parallele erste Zugriffe öffnen genau einmal, die anderen warten und erhalten dasselbe Ergebnis.
- Das Öffnungsergebnis, auch ein Fehler, wird gemerkt. Jeder spätere Zugriff liefert denselben `OpenError`. Es gibt keinen Retry.
- Zugriffsart:
  - Bei `use_mmap` wird gemappt (`memmap2::Mmap`, read-only). Danach wird auf der ganzen Länge `Advice::Random` über `page_advice::advise` gesetzt (`MmapAdvisor`). Ein Fehler dabei wird nur mit `warn!` geloggt.
  - Schlägt das Mappen fehl oder ist `use_mmap == false`, wird die ganze Datei per File-Read in den Speicher gelesen (`AccessKind::FileRead`). Ein Fallback wird geloggt.
- Ablauf: `read_info` → `read_ids_stamps` → ID-Prüfung → `StampCounter::resume` → optional `verify`. Erst danach gilt der Index als offen.
- ID-Prüfung:
  - Doppelte IDs ergeben `OpenError::Unavailable`.
  - Die Einträge werden nach `EntryId` sortiert gehalten, und die Zeilennummer der Datei bleibt erhalten.
- Stamp-Zähler: `StampCounter::resume(info.stored_max_stamp, Some(höchster Stamp der Spalte))`. Bei 0 Zeilen ist der zweite Wert `None`. Stamp-Rohwerte werden 1:1 als `Stamp(raw)` übernommen.
- Verifikation (`verify == true`, Default):
  - `decoder.verify` dekodiert alle Daten und kontrolliert die Checksums.
  - Ein Fehler bricht das Öffnen mit `Unavailable` ab.
  - Bei `verify == false` wird das Ergebnis „übersprungen“ geloggt.
- Fehlerabbildung: Jeder Datei-, Lese-, Mapping-, Format- oder Checksum-Fehler wird zu `OpenError::Unavailable { path, cause }`. `path` ist der Pfad, `cause` die Ursache im Klartext. Es gibt kein Panic.
- Log (`tracing::info!`, einmal bei Erfolg): Pfad, Schema-Version, Model-ID, Model-Revision, Zugriffsart, Anzahl Row-Groups und Checksum-Ergebnis (`ok` oder `übersprungen`). Fehlschläge werden mit `error!` und Pfad und Ursache geloggt.

**Stamps und IDs**
- `stamps()` und `stamp()` lösen nie Vektor-Dekodierung aus. Sie arbeiten nur auf der geladenen ID-/Stamp-Tabelle.
- `stamps()` ist ein Snapshot, aufsteigend nach `EntryId`, ohne Duplikate.
- Bei Öffnungsfehler liefern sie den gemerkten `OpenError`.

**Lazy Loading**
- Zeile `r` (Dateireihenfolge, 0-basiert) gehört zu Batch `r / 4096`. Ein Batch umfasst die Zeilen `[k*4096, min((k+1)*4096, rows))`.
- Der erste Zugriff auf einen Vektor oder Nutzdaten einer Zeile dekodiert ihren ganzen Batch über `decoder.decode_rows`, vollständig (Vektor und Nutzdaten zusammen).
- Jeder Batch wird höchstens einmal dekodiert, auch bei gleichzeitigen Zugriffen mehrerer Threads (einmalige Initialisierung je Batch). Andere Threads warten auf das Ergebnis. Verschiedene Batches dekodieren unabhängig und parallel.
- Das dekodierte Ergebnis wird dauerhaft gehalten. Vektoren liegen als FP16-Werte in Little-Endian vor. Das Modul konvertiert nichts.
- Rückgaben sind `SharedBytes` (Arc-geteilt, unveränderlich). Der Inhalt bleibt auch nach späteren Zugriffen identisch.
- Schlägt die Dekodierung eines Batches fehl, wird der Fehler für diesen Batch gemerkt und bei jedem Zugriff darauf erneut gemeldet. Es gibt keinen Retry, andere Batches bleiben nutzbar.
- Fehler bei `read_vector`: leeres `SharedBytes` plus `error!`-Log (Pfad, Batch, Ursache).
- Fehler bei `read_meta`: `None` plus `error!`-Log.
- Fehler bei `read`: Der Batch wird einmal angefragt. Beide Teile sind leer (`Payload::default()`), das Log erfolgt einmal pro Aufruf.
- Unbekannte ID: `read_vector` liefert leer, `read_meta` liefert `None`, `read` liefert `Payload::default()`. Es gibt kein Log und keine Dekodierung.
- Ist der Index nicht verfügbar, verhalten sich `read_vector`, `read_meta` und `read` wie bei einem Dekodier-Fehler (leer bzw. `None`, `error!`-Log). Sie geben den `OpenError` nicht zurück.

**Nebenläufigkeit**
- Alle Methoden sind `&self`, von beliebig vielen Threads aufrufbar, ohne externes Locking.
- Nach dem Öffnen ist die ID-/Stamp-Tabelle unveränderlich und lock-frei lesbar.
- Lesemethoden dürfen blockieren (IO, Page-Faults). Im Async-Kontext gehören sie in `page_advice::read_blocking`.

## 5. Relied-on elements

Aus `change_stamp`:
```rust
StampCounter::resume(stored_max: Option<u64>, highest_present: Option<u64>) -> StampCounter
// nächstes next() liefert max(stored, highest) + 1; base == 0 verhält sich wie new()
```
`StampCounter: Send + Sync`.

Aus `index_contract`:
```rust
pub struct EntryId(pub u64);   // Ord, Hash
pub struct Stamp(pub u64);     // Ord; Stamp(0) wird nie an Writes vergeben
pub struct SharedBytes(pub Arc<[u8]>);  // Default = leer, Clone billig
pub struct Payload { pub vector: SharedBytes, pub meta: SharedBytes } // Default = beide leer
```
Vertrag: `stamp` und `stamps` berühren keine Vektordaten. `stamps` ist nach `EntryId` sortiert und duplikatfrei.

Aus `page_advice`:
```rust
pub enum Advice { WillNeed, Sequential, Random, Cold }
pub struct ByteRange { pub start: u64, pub end: u64 }   // [start, end)
pub struct MmapAdvisor; impl MmapAdvisor { pub fn new(map: Arc<memmap2::Mmap>) -> Self; }
pub fn advise(target: &dyn AdviceTarget, advice: Advice, range: ByteRange) -> Result<(), AdviceError>;
pub async fn read_blocking<T, F>(f: F) -> Result<T, BlockingError> where F: FnOnce() -> T + Send + 'static, T: Send + 'static;
```
Hinweise sind unverbindlich. Ein leerer Bereich ist ein No-op.