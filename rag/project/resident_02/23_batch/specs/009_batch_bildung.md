# Spezifikation: Batch-Bildung (Build-Order 9)

## 1. Task, restrictions and boundaries

**Aufgabe**
- Das Modul bildet aus Tokenfolgen (`&[Vec<u32>]`) gepaddete Batches mit Attention-Mask.
- Es serialisiert Batches als u32 Little-Endian.
- Es leitet die Batch-Größe aus Token-Budget, Length-Bucket und Deadline je Klasse ab.

**Grenzen**
- Keine Abhängigkeiten zu anderen Modulen. Es gibt kein Tokenizing, keine Inferenz und kein I/O.
- Padding geschieht ausschließlich hier.
- Alle Funktionen sind rein und zustandslos.
- Erfinde keine weiteren Verhaltensweisen. Nur `std` ist erlaubt.

## 2. Context

- Upstream liefert Token-ID-Folgen (u32), vermutlich aus einem Tokenizer. Downstream läuft die Inferenz.
- Die Inferenz erwartet pro Batch ein Array `2 × Zeilen × Sequenzlänge`. Es enthält erst die IDs, dann die Mask.
- Die Inferenz ordnet Ergebnisse über die mitgeführten ursprünglichen Positionen wieder der Eingabe zu.
- Die Obergrenze der Batch-Dauer begrenzt die Query-Latenz.

## 3. Interface

```rust
use std::time::Duration;

pub const BUCKETS: [usize; 5] = [32, 64, 128, 256, 512];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    /// token_budget == 0 oder max_seq_len == 0
    InvalidConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchConfig {
    pub token_budget: usize,   // max. Zeilen * Sequenzlänge je Batch, >= 1
    pub max_seq_len: usize,    // >= 1
    pub pad_token_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    // Felder privat; Zugriff über Methoden
}

impl Batch {
    pub fn rows(&self) -> usize;
    pub fn seq_len(&self) -> usize;
    /// Zeilenweise, row-major, Länge rows*seq_len
    pub fn input_ids(&self) -> &[u32];
    /// Gleiche Form; 1 = echtes Token, 0 = Padding
    pub fn attention_mask(&self) -> &[u32];
    /// Ursprüngliche Eingabepositionen je Zeile, Länge rows
    pub fn positions(&self) -> &[u32];
    /// Länge 2*rows*seq_len*4 Bytes: erst alle IDs, dann die ganze Mask, je u32 LE
    pub fn to_bytes(&self) -> Vec<u8>;
    /// positions als u32 LE, Länge rows*4
    pub fn positions_to_bytes(&self) -> Vec<u8>;
}

pub fn build_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Result<Vec<Batch>, BatchError>;

/// Padded length einer Folge (nach Trunkierung auf max_seq_len).
pub fn padded_len(len: usize, max_seq_len: usize) -> usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchClass { Query, Bulk }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassDeadline {
    /// Obergrenze der Batch-Dauer; None = keine Begrenzung
    pub max_batch_duration: Option<Duration>,
    /// Geschätzte Kosten je (gepaddetem) Token, > 0
    pub cost_per_token: Duration,
}

/// Zeilenzahl pro Batch für einen Bucket (Sequenzlänge `bucket_len`).
pub fn batch_size_for(token_budget: usize, bucket_len: usize, deadline: &ClassDeadline) -> usize;
```

## 4. Behavior and invariants

**`padded_len(len, max)`**
- Es trunkiert zuerst: `t = min(len, max)`.
- Dann gilt: kleinster Wert aus `BUCKETS` mit `>= t`. Gibt es keinen (`t > 512`), ist das Ergebnis `max`.
- Das Ergebnis wird auf `min(…, max)` begrenzt und auf mindestens 1 angehoben.
- Beispiele: `padded_len(0, 100) == 32`, `padded_len(40, 50) == 50`, `padded_len(600, 1000) == 600`, `padded_len(5, 16) == 16`.

**`build_batches`**
- Bei ungültiger Konfiguration kommt `Err(InvalidConfig)` zurück. Leere Eingabe ergibt `Ok(vec![])`.
- Die Sortierung ist stabil und aufsteigend nach ursprünglicher (untrunkierter) Länge. Gleiche Länge behält die Eingabereihenfolge.
- Gierige Gruppierung in dieser Reihenfolge:
  - Für jede Folge gilt `p = padded_len(len)`.
  - Der aktuelle Batch ist nicht leer und `(rows+1) * p > token_budget`. Dann wird der Batch abgeschlossen und ein neuer begonnen.
  - Sonst kommt die Zeile in den aktuellen Batch.
  - Eine Folge, die allein `p > token_budget` hat, bildet einen eigenen Batch, weil ein leerer Batch immer eine Zeile aufnimmt. Die nachfolgende Folge beginnt dann ebenfalls einen neuen Batch, weil die Prüfung für sie fehlschlägt.
- `seq_len` eines Batches ist die gepaddete Länge der letzten (längsten) Zeile. Alle Zeilen haben genau diese Länge.
- Jede Zeile enthält die auf `max_seq_len` trunkierten Token, danach `pad_token_id`.
- Die Mask hat 1 für echte Token und 0 für Padding. Eine leere Folge ergibt eine Zeile nur aus Padding mit Mask 0.
- Jede Eingabefolge steht in genau einem Batch. `positions` enthält ihren Index im Eingabe-Slice als u32. Die Zeilen eines Batches stehen in sortierter Reihenfolge.
- Batches stehen in aufsteigender Sortierreihenfolge.
- Es gilt immer `seq_len <= max_seq_len` und `seq_len >= 1`.
- Echte Token zählen nie mehr als `seq_len`, und `rows * seq_len <= token_budget`, außer bei Einzelzeilen-Batches.

**Serialisierung**
- `to_bytes` entspricht einem Array `2 × rows × seq_len`: erst `input_ids`, dann `attention_mask`. Jeder Wert ist ein `u32::to_le_bytes`.
- `positions_to_bytes` gibt die Positionen in Zeilenreihenfolge als u32 LE aus.

**`batch_size_for`**
- `by_budget = max(1, token_budget / bucket_len)`. Bei `bucket_len == 0` wird 1 verwendet.
- Ist `max_batch_duration = Some(d)`, gilt `by_deadline = max(1, d.as_nanos() / (bucket_len * cost_per_token.as_nanos()))`. Bei `None` gibt es keine Deadline-Begrenzung.
- Das Ergebnis ist `min(by_budget, by_deadline)` und immer `>= 1`.
- Die Deadline begrenzt damit die Batch-Dauer. Die Funktion ist je Klasse mit ihrer eigenen `ClassDeadline` aufrufbar.
- `build_batches` verwendet `batch_size_for` nicht. Es ist eine eigenständige, aufrufbare Ableitung.

**Nebenläufigkeit und Fehler**
- Alle Typen sind `Send + Sync`. Funktionen sind reentrant und haben keinen globalen Zustand.
- Kein Panic bei beliebiger Eingabe. `BatchError::InvalidConfig` ist der einzige Fehler.

## 5. Relied-on elements

Keine, das Modul hat keine Abhängigkeiten.