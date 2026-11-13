# Spezifikation: Nutzdaten und Checksum (Modul `entry_payload`, Build-Order 121)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:**
- Je Entry den Verweis auf den Quelltext führen: Zieldatei, Zeichenanfang, Zeichenende.
- Die 64-Bit-Checksum über Vektorbytes und Verweis berechnen und verifizieren.

**Grenzen:**
- Reine Wertetypen und reine Funktionen. Kein I/O, kein globaler Zustand, keine Threads, kein `unsafe`, keine neuen Crates (nur `std` und `index_contract`).
- Keine Parquet-Logik: Der Aufrufer (Parquet-Schreiber/-Leser) überträgt die Felder in die Spalten und zurück.
- Keine Stamp-Vergabe, keine Vektor-Konvertierung und keine Prüfung der Vektorlänge (das macht der Index).
- Es gibt keinen Fehlertyp für Dateiinhalt: Zieldatei und Zeichenbereich werden nicht gegen das Dateisystem geprüft. `char_start <= char_end` wird nicht erzwungen.
- Die Checksum ist ein Integritätscode, keine kryptografische Signatur.

## 2. Kontext
- Der Index-Vertrag (`index_contract`) speichert pro Entry `Payload { vector: SharedBytes, meta: SharedBytes }`. Die Vektorbytes liegen im festen `DType` des Index (im Parquet-Format FP16).
- Das Parquet-Schema (`index_format`) hat pro Entry die Spalten `target_file` (Utf8), `char_start` (UInt64), `char_end` (UInt64) und `checksum` (UInt64), alle nicht nullable.
- Dieses Modul liefert die Werte für diese Spalten:
  - Beim Speichern berechnet der Parquet-Schreiber die Checksum neu und schreibt sie in `checksum`.
  - Bei der Verifikation berechnet der Leser sie aus den gelesenen Spalten neu und vergleicht sie mit dem gespeicherten Wert.
- Der Header-Wert `hash_algorithm = Fast64` mit `hash_version = 1` (`index_format::HASH_ALGORITHM`, `HASH_VERSION`) gilt auch für diese Checksum. Eine Änderung des Algorithmus oder der Checksum-Bytefolge erfordert eine neue `HASH_VERSION`.

## 3. Interface
```rust
use index_contract::SharedBytes;

/// Verweis auf den Quelltext eines Entries.
/// Default: leere Datei, 0, 0.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SourceRef {
    pub target_file: String,   // Spalte `target_file`
    pub char_start: u64,       // Spalte `char_start`
    pub char_end: u64,         // Spalte `char_end`
}
impl SourceRef {
    pub fn new(target_file: impl Into<String>, char_start: u64, char_end: u64) -> SourceRef;
}

/// Berechnet die 64-Bit-Checksum (Algorithmus siehe §4).
pub fn compute_checksum(vector: &[u8], source: &SourceRef) -> u64;

/// Gleiche Berechnung ohne SourceRef-Struktur (für Spalten-Leser).
pub fn compute_checksum_parts(vector: &[u8], target_file: &str, char_start: u64, char_end: u64) -> u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumMismatch { pub stored: u64, pub computed: u64 }
impl std::fmt::Display for ChecksumMismatch {}   // deutscher Text, nennt beide Werte (hex, 16 Stellen)
impl std::error::Error for ChecksumMismatch {}

/// Prüft einen gespeicherten Wert gegen die Neuberechnung.
pub fn verify_checksum(vector: &[u8], source: &SourceRef, stored: u64) -> Result<(), ChecksumMismatch>;

/// Entry-Nutzdaten mit Verweis und konsistenter Checksum.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntryData {
    pub vector: SharedBytes,
    pub source: SourceRef,
    checksum: u64,                      // privat; immer zu (vector, source) passend, außer bei from_stored
}
impl EntryData {
    /// Berechnet die Checksum.
    pub fn new(vector: SharedBytes, source: SourceRef) -> EntryData;
    /// Übernimmt einen gelesenen Wert ungeprüft (für den Leser; danach `verify`).
    pub fn from_stored(vector: SharedBytes, source: SourceRef, stored_checksum: u64) -> EntryData;
    pub fn checksum(&self) -> u64;
    /// Berechnet neu und vergleicht mit dem gespeicherten Wert.
    pub fn verify(&self) -> Result<(), ChecksumMismatch>;
    /// Berechnet die Checksum neu und speichert sie (nach Änderung der öffentlichen Felder; beim Speichern).
    pub fn recompute(&mut self) -> u64;
}
```
Alle Typen sind `Send + Sync + 'static`. Alle Funktionen sind zustandslos und paniert nie.

## 4. Behavior und Invarianten
**Default:** `SourceRef::default()` ist `{ target_file: "", char_start: 0, char_end: 0 }`. `EntryData::default()` hat leeren Vektor und Default-SourceRef. Seine Checksum ist `compute_checksum(&[], &SourceRef::default())`, nicht 0.

**Algorithmus (Fast64, Version 1, bitgenau verbindlich).** FNV-1a 64:
- Startwert `0xcbf29ce484222325`.
- Pro Byte gilt `h ^= byte; h = h.wrapping_mul(0x100000001b3)`.

Die Eingabebytes in dieser Reihenfolge:
1. Vektorlänge in Bytes als `u64` little-endian (8 Bytes).
2. Vektorbytes.
3. Länge der Zieldatei in UTF-8-Bytes als `u64` little-endian.
4. UTF-8-Bytes der Zieldatei.
5. `char_start` als `u64` little-endian.
6. `char_end` als `u64` little-endian.

Das Ergebnis ist der Endwert `h`.
- Der Dateiname geht unverändert ein: kein Normalisieren und kein Trimmen.
- Die vorangestellten Längen schließen mehrdeutige Konkatenationen aus. Beispiel: Vektor `[1,2]` mit Datei `"a"` unterscheidet sich von Vektor `[1]` mit Datei `"\x02a"`.
- Testvektor: `compute_checksum(&[], &SourceRef::default())` ist FNV-1a über 16 Nullbytes (Längen 0 und 0) gefolgt von 16 Nullbytes (Start und Ende), also über 32 Nullbytes.
- Zusätzlich gelten:
  - Jede Änderung eines einzelnen Bytes in einem Eingabefeld ergibt eine andere Checksum. Der Grund: XOR und Multiplikation mit der ungeraden Primzahl sind bijektiv auf dem Zustand.
  - Alle anderen Änderungen (mehrere Bytes) werden mit Hash-Wahrscheinlichkeit erkannt.
  - Eine Änderung von `char_start` oder `char_end` um ein Bit ist erkennbar.
- Die Funktion ist deterministisch, plattformunabhängig (explizit little-endian) und unabhängig vom Thread.
- `compute_checksum(v, s) == compute_checksum_parts(v, &s.target_file, s.char_start, s.char_end)`.

**Verifikation:**
- `verify_checksum` berechnet immer neu und vergleicht mit `stored`. Bei Ungleichheit kommt `Err(ChecksumMismatch { stored, computed })`, sonst `Ok(())`.
- `EntryData::new` setzt `checksum == compute_checksum(&vector, &source)`, also `verify()` ist `Ok`.
- `EntryData::from_stored` prüft nicht. `verify()` meldet danach ein abweichendes `stored`.
- Das Ändern der öffentlichen Felder `vector` oder `source` macht `verify()` fehlschlagend, bis `recompute()` aufgerufen wird. `recompute()` setzt die Checksum neu und gibt sie zurück.
- Der Aufrufer muss beim Speichern `recompute()` bzw. `compute_checksum` aufrufen und darf einen früher gelesenen Wert nicht übernehmen. Bei der Verifikation muss er neu berechnen und vergleichen.

**Edge Cases:**
- Leerer Vektor, leerer Dateiname und `char_end < char_start` sind gültig und werden nicht abgelehnt.
- Dateinamen mit beliebigem Unicode oder Nullbytes gehen unverändert ein.
- Vektorlänge und Dateinamenlänge werden als `u64` kodiert (`usize as u64`).

**Nebenläufigkeit:** Alle Funktionen sind rein und beliebig parallel aufrufbar. `EntryData` ist ein Wertetyp ohne innere Veränderlichkeit. `recompute` braucht `&mut self`.

## 5. Relied-on elements
```rust
// index_contract (Build-Order 33, verbatim)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SharedBytes(pub Arc<[u8]>);          // impl Deref<Target=[u8]>; Clone billig; leer = len 0
pub struct Payload { pub vector: SharedBytes, pub meta: SharedBytes }
```
`EntryData.vector` verwendet `SharedBytes` und verlässt sich auf `Deref<Target=[u8]>` und `Default` (leer).

**Aus `index_format` (Build-Order 113), unverändert:**
- Spalten (alle `nullable = false`): `target_file: Utf8`, `char_start: UInt64`, `char_end: UInt64`, `checksum: UInt64`. Namen: `COL_TARGET_FILE`, `COL_CHAR_START`, `COL_CHAR_END`, `COL_CHECKSUM`.
- `HASH_ALGORITHM = HashAlgorithm::Fast64` und `HASH_VERSION = 1`. Dieses Modul implementiert diesen Content-Hash für die Entry-Checksum. `index_format` berechnet ihn nicht selbst.
- `FNV-1a-64` (offset `0xcbf29ce484222325`, prime `0x100000001b3`) entspricht der Konstante, die dort für den Tokenizer-Fingerprint genutzt wird.