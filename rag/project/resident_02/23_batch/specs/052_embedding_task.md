# Spezifikation: Embedding-Task (Build-Order 52)

## 1. Task, restrictions and boundaries
Das Crate-Modul `embedding_task` beschreibt einen Embedding-Auftrag als Datentyp und führt ihn über ein injiziertes Backend aus. Es liefert `TaskResult::Embeddings` oder, bei gesetztem Ziel-Index, `TaskResult::EntryRefs`.

- Das Modul wandelt Texte in Vektoren um, indem es ein `EmbeddingBackend` aufruft. Es enthält selbst keine Modellinferenz, kein Scheduling, keine Queue, kein Batching über Tasks hinweg und keine Serialisierung.
- Es gibt keine Determinismusgarantie. Dieselbe Eingabe darf bei zwei Läufen unterschiedliche Vektoren liefern. Tests dürfen keine Bit-Gleichheit zwischen Läufen erwarten.
- Es vergibt keine Entry-IDs und keine Stamps. Das Ziel vergibt sie.
- Es wird kein Cache, keine Normalisierung und keine Nachbearbeitung der Vektoren vorgenommen.
- Abhängigkeiten sind `std`, `result_kinds`, `extensions` (Erweiterbare Zusatzparameter) und `index_contract::EntryId`. Keine `unsafe`-Blöcke.

## 2. Context
- Der Task ist eine Task-Art unter mehreren. Die Parameter reisen als Wertobjekt durch Queue und Planung. Darum muss `EmbeddingTask` `Clone + Send + Sync + 'static` sein.
- Das Ergebnis ist ein `result_kinds::TaskResult`. Ohne Ziel-Index ist es `Embeddings(Vec<Vec<f32>>)`, mit Ziel-Index ist es `EntryRefs(Vec<EntryId>)`. Im zweiten Fall liegen die Daten im Index.
- Zusatzparameter sind ein `Extensions`. Der Kern interpretiert sie nicht. Dieses Modul gibt sie unverändert an das Backend und das Ziel weiter.
- Der Index-Vertrag ist noch nicht verfügbar. Dieses Modul definiert deshalb den schmalen Trait `EmbeddingTarget`, den ein Index-Adapter implementiert. Der Adapter ruft dahinter `index_contract::Index::write` auf.

## 3. Interface
```rust
use std::sync::Arc;
use extensions::Extensions;
use index_contract::EntryId;
use result_kinds::TaskResult;

/// Modus des Embeddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddingMode { Query, Passage }

/// Numerische Precision der gelieferten Vektoren (Werte bleiben im Ergebnis f32).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Precision { F32, F16, Bf16, Int8 }

/// Modellbezeichner (opak, vom Backend aufgelöst).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);

/// Eingabe: ein Text oder eine Liste von Texten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingInput { One(String), Many(Vec<String>) }
impl EmbeddingInput {
    pub fn texts(&self) -> &[String];   // One -> Slice der Länge 1
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
impl From<String> for EmbeddingInput {}
impl From<&str> for EmbeddingInput {}
impl From<Vec<String>> for EmbeddingInput {}

/// Metadaten, die mit jedem Vektor ins Ziel geschrieben werden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryMetadata {
    pub text: String,
    pub model: ModelId,
    pub mode: EmbeddingMode,
    pub precision: Precision,
}

/// Ziel-Index (vom Index-Adapter implementiert).
pub trait EmbeddingTarget: Send + Sync {
    /// Schreibt Vektoren samt Metadaten. `vectors.len() == metadata.len()`.
    /// Liefert genau eine EntryId je Vektor, in gleicher Reihenfolge.
    fn write(&self, vectors: Vec<Vec<f32>>, metadata: Vec<EntryMetadata>, extensions: &Extensions)
        -> Result<Vec<EntryId>, TargetError>;
}

/// Backend, das Texte in Vektoren umwandelt.
pub trait EmbeddingBackend: Send + Sync {
    /// Liefert genau einen Vektor je Text in Eingabereihenfolge.
    fn embed(&self, model: &ModelId, mode: EmbeddingMode, precision: Precision,
             texts: &[String], extensions: &Extensions)
        -> Result<Vec<Vec<f32>>, BackendError>;
}

#[derive(Debug, Clone, PartialEq, Eq)] pub struct BackendError(pub String);
#[derive(Debug, Clone, PartialEq, Eq)] pub struct TargetError(pub String);

/// Beschreibung eines Embedding-Auftrags.
#[derive(Clone)]
pub struct EmbeddingTask {
    pub input: EmbeddingInput,
    pub model: ModelId,
    pub mode: EmbeddingMode,
    pub precision: Precision,
    pub target: Option<Arc<dyn EmbeddingTarget>>,
    pub extensions: Extensions,
}
impl EmbeddingTask {
    pub fn new(input: impl Into<EmbeddingInput>, model: ModelId,
               mode: EmbeddingMode, precision: Precision) -> Self; // target None, extensions leer
    pub fn with_target(self, target: Arc<dyn EmbeddingTarget>) -> Self;
    pub fn with_extensions(self, extensions: Extensions) -> Self;
    /// Führt den Auftrag aus.
    pub fn run(&self, backend: &dyn EmbeddingBackend) -> Result<TaskResult, EmbeddingError>;
}
impl std::fmt::Debug for EmbeddingTask {} // manuell; zeigt target nur als is_some

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingError {
    Backend(BackendError),
    Target(TargetError),
    /// Backend lieferte eine andere Anzahl Vektoren als Texte, oder das Ziel eine andere Anzahl IDs als Vektoren.
    CountMismatch { expected: usize, got: usize },
}
impl std::fmt::Display for EmbeddingError {}
impl std::error::Error for EmbeddingError {}
```
`EmbeddingTask`, `EmbeddingError` und alle Typen sind `Send + Sync + 'static`. Dafür gibt es eine Compile-Time-Assertion.

## 4. Behavior and invariants
**Ergebnis**
- Ohne `target` ist das Ergebnis `TaskResult::Embeddings(v)`. Dabei gilt `v.len() == input.len()`. `v[i]` gehört zu `texts()[i]`.
- Mit `target` ist das Ergebnis `TaskResult::EntryRefs(ids)`. Dabei gilt `ids.len() == input.len()`. `ids[i]` gehört zu `texts()[i]`. Der Aufrufer erhält die Vektoren dann nicht zurück.
- `EmbeddingInput::One(s)` verhält sich wie `Many(vec![s])`. Das Ergebnis ist eine Liste mit einem Vektor bzw. einer ID, kein bloßer Vektor.

**Ablauf von `run`**
1. Ist die Eingabe leer (`Many(vec![])`), liefert `run` ohne Backend- und Target-Aufruf ein leeres `Embeddings` (ohne Ziel) bzw. ein leeres `EntryRefs` (mit Ziel). Es ist kein Fehler.
2. Sonst ruft `run` genau einmal `backend.embed(model, mode, precision, texts, &extensions)` mit allen Texten auf.
3. Stimmt die Anzahl der Vektoren nicht mit der Anzahl der Texte überein, liefert `run` `CountMismatch { expected: texts, got: vectors }`.
4. Mit Ziel ruft `run` genau einmal `target.write(vectors, metadata, &extensions)` auf. `metadata[i]` enthält `text = texts()[i]` sowie `model`, `mode` und `precision` des Tasks. Die Anzahl der IDs wird geprüft, bei Abweichung `CountMismatch { expected: vectors, got: ids }`.
5. Die Vektoren werden unverändert weitergereicht. Es erfolgt keine Normalisierung, Rundung oder Umsortierung. Die Reihenfolge bleibt erhalten, auch bei Duplikaten. Gleiche Texte erzeugen getrennte Einträge.

**Fehler**
- Ein Backend-Fehler liefert `EmbeddingError::Backend` und ruft das Ziel nicht auf.
- Ein Ziel-Fehler liefert `EmbeddingError::Target`. Es gibt kein Teilergebnis. Ein Rollback bereits geschriebener Entries ist Sache des Ziels.
- Es gibt keine Panics durch `run`, auch nicht bei leeren Texten oder leerer Liste. Ein leerer String ist eine gültige Eingabe und wird ans Backend gereicht.
- Eine Validierung von Modellname, Precision oder Vektordimension findet nicht statt. Das ist Sache des Backends.

**Determinismus und Nebenläufigkeit**
- Es gibt keine Determinismusgarantie (siehe §1). Das Backend darf Aufträge frei batchen, umordnen oder parallel ausführen. Es muss nur die Ordnung innerhalb des Auftrags wahren.
- `run(&self)` ist ohne Interior Mutability. Derselbe Task darf von mehreren Threads gleichzeitig ausgeführt werden, wenn das Backend `Sync` ist. Jeder Lauf ist unabhängig.
- `clone()` kopiert die Parameter. `target` und `extensions` werden geteilt (`Arc`). Der Task selbst wird beim Ausführen nicht verändert.
- `extensions` werden unverändert und ungelesen an Backend und Ziel gegeben.

## 5. Relied-on elements
Aus `result_kinds` (unverändert genutzt):
```rust
#[non_exhaustive] pub enum TaskResult {
    Embeddings(Vec<Vec<f32>>), EntryRefs(Vec<EntryId>), /* weitere Arten */ }
```
Das Modul erzeugt nur die Varianten `Embeddings` und `EntryRefs`.

Aus `index_contract` (Build-Order 33):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(pub u64);
```
Aus `extensions` (Build-Order 1):
```rust
#[derive(Clone, Default)] pub struct Extensions { /* Arc-geteilte Werte */ }
impl Extensions { pub fn new() -> Self; /* insert/get/remove/len/is_empty */ }
```
`Extensions` ist `Send + Sync`. Der Klon teilt die Werte. Der Kern interpretiert sie nicht.