# Spezifikation: Expander (Build-Order 65)

## 1. Aufgabe, Einschränkungen, Grenzen

Rust-Modul `expander`. Es zerlegt einen Task als reine Funktion genau einmal in eine geordnete Kette von `PlannedOperation`. Es übersetzt Task-Parameter, Pipeline-Konfiguration und Subengine-Capabilities in die Konfiguration jeder Operation.

Es unterstützt zwei Task-Arten:
- Embedding
- Similarity, mit Vektor-Scorer oder Cross-Encoder

Einschränkungen:
- Es ist zustandslos. Es hat keine Queues, Worker und Notification und startet keine Subengines.
- Es führt keine Operation aus und baut keine `Operation<R>` (keine Regionen, keine `TaskId`, keine Priority). Der Planner baut daraus die `Operation<R>`.
- Das Ergebnis hängt nicht von Nutzdaten ab. Texte, Query-Vektoren und Payload werden nicht gelesen.
- Es führt keine I/O aus, hat keinen globalen Zustand und keinen `unsafe`-Code. Einzige Nebenwirkung ist der Debug-Log.
- Zusätzliche Crate-Abhängigkeit: `log`.

## 2. Kontext

- Der Planner ruft `expand` pro Task auf und erhält die Kette. Die Reihenfolge der Kette ist der implizite Graph: Ausgabe *n* speist Operation *n+1*.
- Die Sub-Engines wählen ihren Ressourceneintrag über `PlannedOperation::resource` (`ResourceKind`, Modell, Index, Environment).
- Worker filtern über `PlannedOperation::descriptor`. Der Descriptor ist als Schlüssel nutzbar.
- Die Pipeline-Konfiguration pro Modell stammt aus dem `ModelDescriptor`. `PipelineConfig::from_descriptor` bildet sie ab.
- Die Idle-Notification meldet, welche Gerätearten gerade frei sind.

## 3. Interface

```rust
use std::collections::HashMap;
use capability_model::CapabilitySet;
use device::DeviceKind;
use layout::DType;
use model_descriptor::{ModelDescriptor, Normalization, Pooling};
use operation::{OperationDescriptor, OperationKind};
use resource_requirements::{IndexId, ModelId, ResourceRequirements};
use embedding_task::{EmbeddingMode, EmbeddingTask};
use similarity_task::{SimilarityParams, Scoring};
use scopes::Scope;

/// Eingabe: Task-Art plus Parameter. Nutzdaten (Texte, Queries, Payload) werden nicht gelesen.
#[non_exhaustive]
pub enum TaskSpec {
    /// `task.target` wird ignoriert. Das Schreiben in den Index richtet sich nur nach `target_index`.
    Embedding { task: EmbeddingTask, target_index: Option<IndexId> },
    Similarity(SimilaritySpec),
    /// Fremde Task-Art, nur ihr Name.
    Other { kind: String },
}

#[derive(Clone, Debug)]
pub struct SimilaritySpec {
    pub params: SimilarityParams,   // index, top_k, scope, ...
    pub model: ModelId,             // Modell des Scorers; Schlüssel in die Pipeline-Konfigurationen
    pub scorer: ScorerChoice,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScorerChoice {
    Vector(Scoring),
    /// Name des Cross-Encoder-Scorers.
    CrossEncoder(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoolingMethod { Mean, Cls, None }

/// Pipeline-Konfiguration eines Modells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineConfig {
    pub query_prefix: String,
    pub passage_prefix: String,
    pub pooling: PoolingMethod,
    pub normalize: bool,
    pub max_seq_len: usize,
    pub dtype: DType,
    pub dimension: usize,           // Zieldimension
}
impl PipelineConfig {
    /// Pooling::Mean->Mean, Cls->Cls (die Methode `None` ist nur direkt setzbar).
    /// normalize = (Normalization::L2), dtype = storage_dtype, dimension = d.dimension.
    pub fn from_descriptor(d: &ModelDescriptor) -> Self;
}

/// Schlüssel: Modell-ID als String.
pub type PipelineConfigs = HashMap<String, PipelineConfig>;

/// Welche Gerätearten gerade frei sind. Default = leer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdleNotification { pub idle_devices: Vec<DeviceKind> }

/// Eine geplante Operation samt Konfiguration.
#[derive(Clone, Debug)]
pub struct PlannedOperation {
    pub kind: OperationKind,
    pub descriptor: OperationDescriptor,
    pub resource: ResourceRequirements,   // resource_requirements::ResourceRequirements
    pub config: OperationConfig,
}

/// Nicht belegte Felder sind None.
#[derive(Clone, Debug, Default)]
pub struct OperationConfig {
    pub index: Option<IndexId>,
    pub tokenizer: Option<ModelId>,
    pub model: Option<ModelId>,
    pub scorer: Option<String>,
    pub pooling: Option<PoolingMethod>,
    pub prefix: Option<String>,
    pub target_dimension: Option<usize>,
    pub top_k: Option<u32>,
    pub max_seq_len: Option<usize>,
    pub dtype: Option<DType>,
    pub scope: Option<Scope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigError { ZeroTopK }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpandError {
    ForeignTaskKind { expected: String, got: String },
    MissingPipelineConfig { model: String },
    Config(ConfigError),
    ScoringUnavailable { scorer: String },
}
// Display, std::error::Error. Texte siehe Abschnitt 4.

pub fn expand(
    task: &TaskSpec,
    pipelines: &PipelineConfigs,
    capabilities: &CapabilitySet,           // Capabilities der Subengines
    idle: Option<&IdleNotification>,        // None == IdleNotification::default()
) -> Result<Vec<PlannedOperation>, ExpandError>;
```

Alle Typen sind `Send + Sync + 'static`. Dafür gibt es eine Compile-Time-Assertion.

## 4. Verhalten und Invarianten

**Allgemein**
- `expand` ist rein: Gleiche Eingabe ergibt eine gleiche Kette (`kind`, `descriptor`, `resource` und `config` je Position).
- Bei Erfolg loggt `expand` auf Debug-Level die Task-Art (`"embedding"`, `"similarity"` oder `"similarity/cross-encoder"`) und die Anzahl der Operationen. Bei Fehlern wird nichts geloggt.
- Prüfreihenfolge:
  1. Task-Art.
  2. Pipeline-Konfiguration des Modells.
  3. Top-k.
  4. Cross-Encoder-Verfügbarkeit.
- Bei einem Fehler wird keine Teilkette geliefert.

**Fremde Art**
- `TaskSpec::Other{kind}` ergibt `ForeignTaskKind{expected: "Embedding/Similarity", got: kind}`.
- Display: `"fremde Task-Art: erwartet Embedding/Similarity, erhalten {got}"`. Die Meldung nennt beide Arten.

**Pipeline-Konfiguration**
- Das Modell ist `task.model.0` (Embedding) bzw. `spec.model.as_str()` (Similarity). Fehlt es in `pipelines`, ergibt das `MissingPipelineConfig{model}`.
- Display: `"keine Pipeline-Konfiguration für Modell {model}"`.
- Die Embedding-`ModelId` (`embedding_task::ModelId(String)`) wird in `resource_requirements::ModelId::new(..)` umgewandelt.

**Mapping auf `resource` und `descriptor`**
- `resource` ist `ResourceRequirements::new(kind, env)`. Es enthält `with_model(model)`. `with_index(..)` kommt nur dort dazu, wo die Tabellen unten einen Index nennen. Es gibt keine Capabilities und keine Affinity.
- Der `descriptor` ist `OperationDescriptor::new(kind, OperationParams::new(req))`.
  - `req` ist `operation::ResourceRequirements{resource, model, mode}`.
  - `resource` ist `"tokenizer"`, `"encoder"`, `"index"`, `"scorer"` oder `"cross_encoder"` (entsprechend dem `ResourceKind`).
  - `model` ist die Modell-ID.
  - `mode` ist `Some("query")` bzw. `Some("passage")` bei jeder Embedding-Operation, sonst `None`.
  - Danach folgen `with_dimension(d)`, wo `target_dimension` gesetzt ist, und `with_top_k(k as usize)`, wo `top_k` gesetzt ist.
- Ein Index wird umgewandelt als `IndexId::new(params.index.0.to_string())` (Similarity-`IndexId(u64)`). Die Index-ID der Embedding-Variante wird unverändert übernommen.

**Environment der Embedding-Berechnung**
- Die Environment wird aus `idle` bestimmt:
  - Enthält `idle_devices` nur `Gpu` (eventuell mehrfach), ist sie `GpuOnly`.
  - Enthält sie nur `Cpu`, ist sie `CpuOnly`.
  - Ist sie leer oder enthält beide Arten, ist sie `Any`.
- Sie gilt für Embedding-Berechnung, Pooling und L2-Normalisierung. Das sind Operationen derselben Encoder-Ressource.
- Alle anderen Operationen haben die in den Tabellen genannte feste Environment.

**Embedding**

Kette in dieser Reihenfolge:

| # | Kind | ResourceKind / Env | Config |
|---|------|--------------------|--------|
| 1 | Tokenization | Tokenizer / CpuOnly | `tokenizer`=Modell, `prefix` (Query→`query_prefix`, Passage→`passage_prefix`), `max_seq_len` |
| 2 | Embedding | Encoder / aus `idle` | `model`, `dtype`, `target_dimension`=`dimension` |
| 3 | Pooling | Encoder / aus `idle` | `pooling` |
| 4 | L2Normalization | Encoder / aus `idle` | `model` |
| 5 | IndexWrite | Index / Any | `index`=Ziel-Index (auch in `resource`) |

- Pooling (3) entfällt bei `PoolingMethod::None`. Die Token-Vektoren bleiben dann erhalten.
- L2Normalization (4) entfällt bei `normalize == false`.
- IndexWrite (5) entfällt bei `target_index == None`.
- Länge: 2 bis 5. Die Reihenfolge der verbleibenden Operationen bleibt erhalten.

**Similarity mit Vektor-Scorer (`ScorerChoice::Vector`)**

Genau zwei Operationen:

| # | Kind | ResourceKind / Env | Config |
|---|------|--------------------|--------|
| 1 | Similarity | Scorer / Any | `model`, `scorer`=Debug-Name des `Scoring`, `index`, `scope`=`params.scope.clone()`, `target_dimension`=`dimension`, `dtype` |
| 2 | TopK | Scorer / Any | `model`, `index`, `top_k` |

- `top_k = min(params.top_k, u32::MAX as usize) as u32`.
- Beide Operationen fordern die Scorer-Ressource des Modells an (`with_model`) und referenzieren den Index (`with_index`).
- `params.top_k == 0` ergibt `Err(Config(ConfigError::ZeroTopK))`. Display: `"Konfigurationsfehler: top_k muss größer als 0 sein"`.

**Similarity mit Cross-Encoder (`ScorerChoice::CrossEncoder(name)`)**

Der Scope stammt aus einem früheren Ergebnis. Er ist `params.scope`, und der Expander verändert ihn nicht. Der Top-k-Check gilt auch hier.

Der Cross-Encoder ist verfügbar, wenn `capabilities.contains(&Name::new("cross_encoder"), &Name::new(name))`. Die Achse heißt `"cross_encoder"`, der Wert ist der Scorer-Name. Sonst gilt `Err(ScoringUnavailable{scorer: name})`. Display: `"Scoring-Verfahren nicht verfügbar: {scorer}"`.

Genau drei Operationen:

| # | Kind | ResourceKind / Env | Config |
|---|------|--------------------|--------|
| 1 | Tokenization (Paare aus Query und Text der Payload) | Tokenizer / CpuOnly | `tokenizer`=Modell, `max_seq_len`, `scope`, `index` |
| 2 | Inference | CrossEncoder / Any | `model`, `scorer`=name, `dtype` |
| 3 | Similarity (Scores) | CrossEncoder / Any | `model`, `scorer`=name, `index`, `scope`, `top_k` (wie oben) |

- `mode` ist bei Similarity immer `None`.

**Nebenläufigkeit**
- `expand` hat keinen geteilten veränderlichen Zustand. Beliebig viele gleichzeitige Aufrufe auf geteilten Referenzen sind sicher. `expand` blockiert nicht.

**Fehler**
- Die Fehler sind `ForeignTaskKind`, `MissingPipelineConfig`, `Config(ZeroTopK)` und `ScoringUnavailable`. Kein Fall führt zu einem Panic.

## 5. Relied-on elements

- `operation`:
  - `OperationKind` (`#[non_exhaustive]`) mit den Varianten `Tokenization`, `Embedding`, `Pooling`, `L2Normalization`, `Similarity`, `TopK`, `IndexWrite`, `Inference`.
  - `OperationParams::new(operation::ResourceRequirements{resource, model, mode})`, mit `with_top_k(usize)` und `with_dimension(usize)` (jeweils `self` konsumierend).
  - `OperationDescriptor::new(kind, params)`.
  - `operation::ResourceRequirements` hat die Felder `resource`, `model` und `mode`, alle `Option<String>` (`Default`).
- `resource_requirements`:
  - `ResourceRequirements::new(ResourceKind, ExecutionEnvironment)` mit `with_model(ModelId)` und `with_index(IndexId)`.
  - `ResourceKind::{Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder}`.
  - `ExecutionEnvironment::{CpuOnly, GpuOnly, Any}`.
  - `ModelId::new` und `IndexId::new` (`impl Into<String>`), `as_str`.
- `capability_model`:
  - `CapabilitySet::contains(&Axis, &Value) -> bool`.
  - `Name::new(impl AsRef<str>)`.
- `device`: `DeviceKind::{Cpu, Gpu}` (`#[non_exhaustive]`, `Copy + Eq`). Eine unbekannte Art in `idle_devices` zählt weder als `Cpu` noch als `Gpu`, löst aber auch kein `Any` aus.
- `model_descriptor`:
  - `ModelDescriptor` mit den Feldern `dimension`, `pooling` (`Pooling::{Mean, Cls}`), `normalization` (`Normalization::{L2, None}`), `query_prefix`, `passage_prefix`, `max_seq_len`, `storage_dtype`.
  - Der Descriptor wird nur gelesen.
- `layout::DType` (`Copy`).
- `embedding_task`:
  - `EmbeddingTask { input, model: ModelId(pub String), mode: EmbeddingMode::{Query, Passage}, precision, target, extensions }`.
  - Nur `model` und `mode` werden gelesen.
- `similarity_task`:
  - `SimilarityParams { index: IndexId(pub u64), top_k: usize, scope: Scope, scoring: Scoring, .. }`.
  - `Scoring` ist `Debug + Copy`.
- `scopes::Scope` ist `Clone + Debug`.