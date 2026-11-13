# Spezifikation: Encoder-Schnittstelle (Build-Order 85)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul `encoder_interface`.
- Es prüft Input- und Output-Tensor (`TensorRef`) gegen die erwartete Shape und den erwarteten Datentyp.
- Es führt das Encoding asynchron auf einer übergebenen Lane aus: Hidden-States, optionale Projection, Pooling mit Normalisierung, Schreiben in die Output-Region.
- Es liefert ein Completion-Event mit hinterlegtem Verarbeitungsfehler und verteilt das Ergebnis über je einen One-Shot-Channel pro Task.

**Grenzen:**
- Das Modul enthält kein Batching, kein Tokenizing, keine eigene Modell-, Projection- oder Pooling-Logik und keine Scheduling-Policy. Es reiht keine Batches ein und ordnet keine Positionen zu.
- Es nutzt kein `RegionHandle`. Der Aufrufer löst Handles vorher zu `Region` und damit zu `TensorRef` auf.
- Es nutzt nur `std` und die Nachbarmodule. Es ändert `Cargo.toml` nicht. Für die Warnung wird `log::warn!` verwendet. Fehlt `log`, wird `eprintln!` mit demselben Text verwendet.
- Es gibt keinen Retry, keinen Fallback und keine Ordnungsgarantie zwischen zwei Aufrufen auf derselben Lane. Der Aufrufer ordnet über die Events.

## 2. Kontext

- Die **Batch-Bildung** liefert `Batch::to_bytes()`: u32 LE, Array `2 × rows × seq_len`, erst IDs, dann Mask. Der Aufrufer schreibt diese Bytes in die Input-Region.
- **Transformer-Hauptmodell:** `TransformerModel::encode` ist synchron und liefert `rows*seq*dim` f32.
- **Projection-Layer:** `Projection::apply` wirkt auf `Vec<Vec<f32>>`. Pooling und Normalisierung (`PoolNorm`) laufen in f32.
- **Speicherzugriff:** `TensorRef` ist die einzige Tensor-Darstellung an dieser Schnittstelle. Die Bytes werden nur über `Region::read` und `Region::write` bewegt.
- **Device und Lane und Event:** `Lane` ist der Ausführungskontext. `Event::cpu()` liefert den Completion-Marker, den der Worker signalisiert.
- **Task-Modell:** `Task` hat ein write-once Ergebnis. Der Aufrufer (Pipeline) verbindet die One-Shot-Receiver mit den Tasks seines Batches.

## 3. Interface

```rust
use std::sync::{Arc, Mutex, mpsc};
use crate::layout::DType;
use crate::memory_access::TensorRef;
use crate::device::{Event, EventError, Lane};
use crate::model_descriptor::Normalization;
use crate::pooling_norm::PoolingKind;
use crate::projection_layer::Projection;
use crate::transformer_model::TransformerModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncoderError {
    /// Anzeige: "inkompatibel: {what}: erwartet {expected}, tatsächlich {actual}"
    Incompatible { what: &'static str, expected: String, actual: String },
    /// Enqueue/Event-Erzeugung oder Warten auf das Event fehlgeschlagen. Anzeige: "Device-Error: {0}"
    Device(String),
    /// Ressourcenmangel beim Enqueue (z. B. Worker nicht startbar). Anzeige: "Memory-Error: {0}"
    Memory(String),
    /// Fehler während der Ausführung. Anzeige: "Verarbeitungsfehler: {0}"
    Processing(String),
}
impl std::fmt::Display for EncoderError {}
impl std::error::Error for EncoderError {}

/// Ergebnis je Task: Ok(()) = Ergebnis steht in der Output-Region.
pub type TaskOutcome = Result<(), EncoderError>;
pub type TaskSender = mpsc::SyncSender<TaskOutcome>;
pub type TaskReceiver = mpsc::Receiver<TaskOutcome>;
/// One-Shot-Channel (sync_channel mit Kapazität 1).
pub fn oneshot() -> (TaskSender, TaskReceiver);

/// Completion-Event plus hinterlegter Verarbeitungsfehler. Clone teilt Event und Fehlerslot.
#[derive(Clone)]
pub struct CompletionEvent { /* Event, Arc<Mutex<Option<EncoderError>>> */ }
impl CompletionEvent {
    pub fn event(&self) -> &Event;
    pub fn is_complete(&self) -> Result<bool, EncoderError>;   // nicht blockierend
    /// Blockiert bis zum Abschluss. Liefert einen hinterlegten Verarbeitungsfehler genau einmal.
    pub fn wait(&self) -> Result<(), EncoderError>;
}

pub struct Encoder { /* Arc<TransformerModel>, Option<Arc<Projection>>, PoolNorm, DType */ }

impl Encoder {
    /// storage_dtype muss F32, F16 oder BF16 sein.
    /// Ist eine Projection gesetzt, gilt: projection.input_dim() == model.dimension().
    /// Sonst Err(Incompatible).
    pub fn new(
        model: Arc<TransformerModel>,
        projection: Option<Arc<Projection>>,
        pooling: PoolingKind,
        normalization: Normalization,
        storage_dtype: DType,
    ) -> Result<Encoder, EncoderError>;

    /// projection.output_dim(), sonst model.dimension().
    pub fn output_dim(&self) -> usize;
    pub fn storage_dtype(&self) -> DType;

    /// Prüft synchron, startet die Ausführung und kehrt nach dem Enqueue zurück.
    /// input erwartet: Shape [2, rows, seq_len], U32, contiguous.
    /// output erwartet:
    ///   Mean-Pooling: [rows, output_dim()]
    ///   kein Pooling: [rows, seq_len, output_dim()]
    ///   Datentyp jeweils storage_dtype(), contiguous.
    /// `senders`: je Task ein Sender. Er erhält genau ein Ergebnis.
    pub fn encode_async(
        &self,
        lane: &Lane,
        input: &TensorRef,
        output: &TensorRef,
        rows: usize,
        seq_len: usize,
        senders: Vec<TaskSender>,
    ) -> Result<CompletionEvent, EncoderError>;
}
```

`Encoder`, `CompletionEvent` und `EncoderError` sind `Send + Sync + 'static`. Das wird per Compile-Time-Assertion im Test geprüft.

## 4. Behavior und Invarianten

**Abfragen**
- `output_dim()` liefert die Projection-Ausgabedimension, wenn eine Projection gesetzt ist, sonst `model.dimension()`.
- `storage_dtype()` liefert den bei `new` übergebenen Typ.
- `new` verhält sich so:
  - Ein nicht-float Datentyp ergibt `Incompatible`.
  - Eine Projection mit falscher `input_dim` ergibt `Incompatible`.
  - Der `PoolNorm`-Fehler (`dim == 0`) ergibt `Incompatible`.
  - `PoolNorm` wird mit `output_dim()` gebaut.

**Vorabprüfung (synchron, vor jedem Enqueue)**
- Die Prüfung läuft in dieser Reihenfolge: Input-Shape, Input-Datentyp, Input-Contiguity, Output-Shape, Output-Datentyp, Output-Contiguity. Die erste Verletzung gewinnt.
- Ein Fehler ist `Incompatible { what: "input"|"output", expected, actual }`.
  - Bei Shape-Fehlern stehen die Shapes in `expected` und `actual`, z. B. `[2, 4, 32]`.
  - Bei Datentyp-Fehlern stehen die Datentypen darin, z. B. `U32`.
  - Bei Nicht-Contiguity nennt `actual` die tatsächlichen Strides.
- Bei einem Fehler wird nichts gelesen, geschrieben, gestartet oder gesendet. Die Sender werden verworfen.
- Der Input enthält genau `2 × rows × seq_len` u32.
- `rows == 0` oder `seq_len == 0` ist zulässig. Die Ausführung läuft normal (leeres Encoding).

**Enqueue und Event**
- Der Aufruf kehrt zurück, sobald der Worker gestartet ist. Er wartet nicht auf das Ergebnis.
- Das Event ist `Event::cpu()`. Der Worker signalisiert es als Letztes, nachdem Fehlerslot und alle Channel-Sendungen abgeschlossen sind.
- Der Worker ist ein eigener Thread (`std::thread::Builder::spawn`). `lane` ist der Ausführungskontext des Aufrufs. Das Modul nutzt von ihr nur `&Lane`: weder `params_mut` noch `ensure_workspace`, und es zeichnet kein Event über den Stream auf.
- Schlägt das Starten des Workers fehl, folgt `Err(Memory(..))`. Schlägt die Event-Erzeugung fehl, folgt `Err(Device(..))`. In beiden Fällen wird nichts ausgeführt, und die Sender werden verworfen.
- Der Worker hält Klone der Arcs, `TensorRef`s und Sender. Der Aufrufer muss die Referenzen nicht über den Aufruf hinaus halten.

**Ablauf im Worker** (Fehler brechen ab und gehen in die Fehlerbehandlung)
1. Die Input-Bytes werden ab `input.offset_bytes()` mit `Region::read` gelesen (`byte_len`). Die Bytes werden als u32 LE interpretiert. Die erste Hälfte sind die IDs (`rows*seq_len`), die zweite Hälfte ist die Mask.
2. `model.encode(ids, mask, rows, seq_len)` liefert `rows*seq_len*model.dimension()` f32.
3. Mit Projection wird jeder Token-Vektor (auch Padding) projiziert. Die Reihenfolge `[row][token]` bleibt erhalten.
4. `PoolNorm::apply(hidden, mask_u8, rows, seq_len)` läuft genau einmal, mit `mask_u8 = (mask != 0) as u8`. Es liefert die Mean-Vektoren `rows*dim` oder bei `None` alle Token-Vektoren.
5. Die f32-Werte werden in den Storage-Datentyp gerundet und als Little-Endian-Bytes gelayoutet.
   - F32 bleibt unverändert.
   - F16 und BF16 runden auf nächsten Wert, bei Gleichstand auf gerade (round-to-nearest-even).
   - NaN und ±Inf bleiben erhalten.
   - Es wird kein `half`-Crate verwendet.
6. Die Bytes werden mit **einem** `Region::write` ab `output.offset_bytes()` geschrieben. Es wird nicht teilweise geschrieben: Vor dem Schreiben sind alle Rechenschritte erfolgreich.

**Fehler während der Ausführung**
- Jeder Fehler (`AccessError`, `TransformerError`, `ProjectionError`, `PoolNormError`, Längenfehler, Panic) wird zu `EncoderError::Processing(<Text>)`.
  - Ein Panic im Worker wird per `catch_unwind` abgefangen.
  - Der Worker signalisiert das Event in jedem Fall.
- Der Fehler wird mit `Warnung "Embedding-Batch fehlgeschlagen: {fehler}"` geloggt, im Fehlerslot des `CompletionEvent` hinterlegt und an alle Sender geschickt.
- Die Lane und das Event gelten als erfolgreich abgeschlossen: `Event::wait` liefert `Ok`, und `is_complete` meldet `Ok(true)`.

**Verteilung**
- Jeder Sender erhält genau eine Nachricht: `Ok(())` bei Erfolg, sonst `Err(Processing(..))` (je Sender ein Klon).
- Ist ein Receiver bereits verworfen, wird der Sendefehler ignoriert. Das stört weder andere Sender noch das Event.
- Die Sendung geschieht vor dem Signalisieren des Events. Wer das Event erfolgreich abwartet, findet die Ergebnisse bereits im Channel.
- `senders` darf leer sein.

**Warten (`CompletionEvent`)**
- `wait()` ruft `Event::wait`. Ein `EventError` ergibt `Err(Device(<EventError-Anzeige>))`. Das gilt auch für Fehler der Execution-Umgebung. Dann wird der Fehlerslot nicht verbraucht.
- Nach erfolgreichem Event-Wait wird der Slot entnommen (`take`). Ein hinterlegter Fehler wird **genau einmal** als `Err(Processing(..))` geliefert. Jeder weitere `wait()`-Aufruf auf diesem Event oder einem Klon liefert `Ok(())`.
- `is_complete()` blockiert nie und verbraucht den Fehler nicht. Ein `EventError` wird als `Device` gemeldet.
- Mehrere Threads dürfen gleichzeitig warten. Der Fehler wird genau einem Aufruf geliefert.

**Nebenläufigkeit**
- Mehrere `encode_async`-Aufrufe sind gleichzeitig möglich, auch auf derselben Lane und mit demselben `Encoder`. Das Modell ist nur lesend. Das Modul serialisiert dabei keine Zugriffe auf Output-Regionen. Überlappende Output-Bereiche zu vermeiden ist Sache des Aufrufers.
- Poison-Locks werden toleriert (innerer Zustand wird weiterverwendet). Es gibt keine Panics im Aufrufer-Thread bei beliebigen Eingaben.

## 5. Relied-on elements

- `memory_access`:
  - `TensorRef::{region() -> &Region, layout() -> &Layout, offset_bytes() -> usize, dtype() -> DType}`.
  - `Region::{read(&self, offset, &mut [u8]) -> Result<(), AccessError>, write(&self, offset, &[u8]) -> Result<(), AccessError>}`, `Send + Sync`, `AccessError: Display`.
  - `TensorRef` ist beim Konstruieren bereits gegen die Region validiert.
- `layout`:
  - `DType { F32, F16, BF16, U8, U32, I64 }` mit `to_safetensors()` für Anzeige und `is_float()`.
  - `Layout::{shape(), strides(), dtype(), is_contiguous(), byte_len()}`.
- `transformer_model`:
  - `TransformerModel::{dimension() -> usize, max_seq_len() -> usize, encode(&self, ids: &[u32], mask: &[u32], rows, seq_len) -> Result<Vec<f32>, TransformerError>}`.
  - Die Anordnung ist `[rows][seq][dim]`. `TransformerModel` ist `Send + Sync`, und `encode` blockiert bis zum Ergebnis.
- `projection_layer`: `Projection::{input_dim(), output_dim(), apply(&self, &[Vec<f32>]) -> Result<Vec<Vec<f32>>, ProjectionError>}`. Die Ausgabevektoren haben die Länge `output_dim`, `Projection` ist `Send + Sync`.
- `pooling_norm`:
  - `PoolNorm::new(PoolingKind, Normalization, dim) -> Result<_, PoolNormError>` (Copy).
  - `apply(&self, hidden: &[f32], mask: &[u8], batch, seq) -> Result<PoolNormOutput, PoolNormError>`.
  - `PoolNormOutput.vectors`: Mean `batch*dim`, None `batch*seq*dim`.
  - `PoolingKind { Mean, None }`.
  - `Normalization { L2, None }` (aus `model_descriptor`).
  - Der Aufrufer ruft es genau einmal am Encoder-Ende auf.
- `batch_formation` (Serialisierungsformat): u32 LE, `2 × rows × seq_len`, erst alle IDs, dann die ganze Mask. Mask: 1 = Token, 0 = Padding.
- `device`:
  - `Event::cpu() -> (Event, EventSignal)`, `EventSignal::signal()` (idempotent).
  - `Event::{wait() -> Result<(), EventError>, is_complete() -> Result<bool, EventError>}`.
  - `EventError: Display`, `Event: Clone + Send + Sync`.
  - `Lane: Send + Sync`.
- `task_model`: `Task` hat ein write-once Ergebnis. Die Verbindung der Receiver mit Tasks gehört dem Aufrufer, dieses Modul kennt `Task` nicht direkt.