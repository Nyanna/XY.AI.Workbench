# Spezifikation: Erkennung von Dateiänderungen (Modul `change_detect`, Build-Order 123)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Das Modul entscheidet, ob eine Quelldatei seit ihrer Aufzeichnung unverändert ist. Es nutzt dafür die Fallback-Kette Change-Stamp, dann Größe und Zeiten, dann Content-Hash. Es erkennt racy Aufzeichnungen und fordert dann eine Content-Prüfung an. Es stellt den schnellen 64-Bit-Content-Hash bereit.

**Grenzen:**
- Erlaubt sind nur `std` und `change_stamp`. Aus `index_format` werden `IndexHeader`, `HashAlgorithm`, `HASH_ALGORITHM` und `HASH_VERSION` genutzt. Neue Crates, `unsafe`, globaler Zustand und Threads sind verboten.
- Das Modul liest keine Datei-Metadaten vom Dateisystem. Der Aufrufer liefert sie als Werte.
- Das Modul persistiert nicht, schreibt keine Index-Header und vergibt keine Stamps.
- Das Modul loggt nicht selbst. Das Diagnose-Log steckt als Text im Ergebnis, und der Aufrufer schreibt es.
- Ob eine Datei neu indexiert wird, entscheidet der Aufrufer. Das Modul liefert nur das Ergebnis.

## 2. Kontext

- Der Index führt je Datei eine Aufzeichnung (`FileRecord`) mit Größe, mtime, optional ctime, Referenzzeitpunkt der Iteration, optional `ChangeStamp` und optional Content-Hash. Der Aufrufer speichert und lädt sie.
- Der Index-Header (`index_format`) trägt Hash-Algorithmus und Hash-Version (`fast_64_bit`, Version 1). Ein Hash gilt nur, wenn beide zum Header passen.
- `ChangeStamp` ist opak und wird nur auf Gleichheit verglichen. Es gibt kein `Ord`.

## 3. Interface

```rust
use std::io::Read;
use std::time::{Duration, SystemTime};
use crate::change_stamp::ChangeStamp;
use crate::index_format::{IndexHeader, HashAlgorithm, HASH_ALGORITHM, HASH_VERSION};

/// Racy-Fenster: 2 Sekunden.
pub const RACY_WINDOW: Duration = Duration::from_secs(2);

/// Metadaten-Schnappschuss einer Datei (aktuell gelesen oder aufgezeichnet).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMeta {
    pub size: u64,
    pub mtime: SystemTime,
    pub ctime: Option<SystemTime>,
}

/// Aufzeichnung einer Datei im Index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileRecord {
    pub meta: FileMeta,
    /// Referenzzeitpunkt der Iteration, in der aufgezeichnet wurde.
    pub reference: SystemTime,
    pub stamp: Option<ChangeStamp>,
    pub content_hash: Option<u64>,
}

impl FileRecord {
    /// Racy-Prüfung der Aufzeichnung (siehe Verhalten).
    pub fn is_racy(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckReason {
    StampDiffers,
    SizeDiffers,
    MtimeDiffers,
    CtimeDiffers,
    Racy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeCheck {
    Unchanged,
    /// Content-Prüfung nötig. `diagnostic` ist ein deutscher Log-Text, nie leer.
    ContentCheckRequired { reason: CheckReason, diagnostic: String },
}

/// Fallback-Kette ohne Content-Hash. Reine Funktion.
/// `current_stamp`: aktueller Stamp der Datei, falls der Aufrufer einen kennt.
pub fn check_file(
    record: &FileRecord,
    current: &FileMeta,
    current_stamp: Option<ChangeStamp>,
) -> ChangeCheck;

/// Hash-Ergebnis der Content-Prüfung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentCheck { Unchanged, Changed, NoComparableHash }

/// Vergleicht den neu berechneten Hash mit dem aufgezeichneten.
/// `NoComparableHash`, wenn `record.content_hash` fehlt oder `header` nicht zu
/// HASH_ALGORITHM/HASH_VERSION passt. Dann gilt die Datei für den Aufrufer als geändert.
pub fn compare_content(record: &FileRecord, header: &IndexHeader, new_hash: u64) -> ContentCheck;

/// True, wenn Header-Algorithmus == HASH_ALGORITHM und Header-Version == HASH_VERSION.
pub fn hash_matches_header(header: &IndexHeader) -> bool;

/// Schneller, nicht kryptografischer 64-Bit-Hash (Algorithmus `Fast64`, Version `HASH_VERSION`).
pub fn content_hash(bytes: &[u8]) -> u64;
/// Streaming-Variante, ergibt für dieselben Bytes denselben Wert wie `content_hash`.
pub fn content_hash_reader<R: Read>(r: R) -> std::io::Result<u64>;
```

Alle Typen sind `Send + Sync + 'static`. Alle Funktionen sind frei von Zustand.

## 4. Behavior und Invarianten

**Racy (`is_racy`):**
- Sei `t` die spätere der Zeiten `meta.mtime` und `meta.ctime` (falls vorhanden).
- Racy gilt genau dann, wenn `t >= reference - 2 s`. Das umfasst alle `t` bis zu 2 s vor dem Referenzzeitpunkt, den Referenzzeitpunkt selbst und jedes spätere `t`.
- Die Zeitrechnung läuft mit `checked_sub`. Ein Unterlauf zählt als nicht racy. Es gibt keine Panics.

**`check_file`, Auswertung in dieser Reihenfolge, der erste Treffer gewinnt:**
1. Sind `record.stamp` und `current_stamp` beide `Some` und ungleich, ist das Ergebnis `ContentCheckRequired(StampDiffers)`. Ist einer von beiden `None` oder sind sie gleich, geht es mit Schritt 2 weiter. Stamps werden nur auf Gleichheit verglichen.
2. Ist `size` ungleich, ist das Ergebnis `SizeDiffers`.
3. Ist `mtime` ungleich (exakt, ohne Toleranz), ist das Ergebnis `MtimeDiffers`.
4. Ist `ctime` ungleich, ist das Ergebnis `CtimeDiffers`. Verglichen wird als `Option`: `None == None` gilt als gleich, `Some` gegen `None` als ungleich.
5. Ist `record.is_racy()`, ist das Ergebnis `Racy`.
6. Sonst ist das Ergebnis `Unchanged`.

- `Unchanged` entsteht nur, wenn Größe, mtime und ctime übereinstimmen und die Aufzeichnung nicht racy ist. Ein gleicher Stamp allein genügt nie.
- Es gibt nur diese zwei Ergebnisse. Eine Abweichung ist nicht „geändert“, sondern „Content-Prüfung nötig“. Erst der Hash entscheidet.
- `diagnostic` nennt den Grund und die verglichenen Werte. Bei `Racy` nennt es mtime/ctime, den Referenzzeitpunkt und das Fenster. Bei Stamps nennt es die Rohwerte (`as_raw`).

**Content-Hash:**
- `content_hash` ist ein deterministischer 64-Bit-FNV-1a-Hash (offset `0xcbf29ce484222325`, prime `0x100000001b3`) über die Rohbytes. Dieser Algorithmus gilt als `Fast64` Version 1 und darf sich ohne Änderung von `HASH_VERSION` nicht ändern.
- Ein leerer Input ergibt den Offset-Wert.
- `content_hash_reader` liest in Blöcken, ohne die ganze Datei im Speicher zu halten. Es wiederholt bei `ErrorKind::Interrupted` und gibt andere I/O-Fehler unverändert zurück.
- `compare_content`:
  - `NoComparableHash`, wenn `!hash_matches_header(header)` oder `record.content_hash` `None` ist.
  - Sonst `Unchanged`, wenn `new_hash` dem aufgezeichneten Hash entspricht, andernfalls `Changed`.

**Concurrency:** Alle Funktionen sind rein und beliebig parallel aufrufbar. Das Modul panict nie. Es kennt keine eigenen Fehlertypen außer `std::io::Error` aus dem Reader.

## 5. Relied-on elements

```rust
// change_stamp (verbatim)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChangeStamp(/* u64 */);   // KEIN PartialOrd/Ord; nur Gleichheit erlaubt
impl ChangeStamp { pub fn as_raw(&self) -> u64; pub fn from_raw(raw: u64) -> ChangeStamp; }

// index_format (verbatim, genutzt)
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Fast64;   // Text "fast_64_bit"
pub const HASH_VERSION: u64 = 1;
pub enum HashAlgorithm { Fast64, Fast128, Cryptographic }          // Debug, Clone, Copy, PartialEq, Eq, Hash
pub struct IndexHeader { pub hash_algorithm: HashAlgorithm, pub hash_version: u64, /* weitere ungenutzt */ }
```

- `index_format` berechnet den Content-Hash nicht. Das tut dieses Modul. Eine Änderung von Algorithmus oder Version bricht die Bindung des Index.
- Die Spalte `checksum` (`UInt64`) im Index ist ein möglicher Ablageort für Hashes. Das Persistieren übernimmt der Aufrufer.