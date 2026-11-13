# Spezifikation: Modellregister (Modul `model_registry`, Build-Order 120)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe.**
- Das Modul führt einen festen Modellsatz. Er wird vor der Initialisierung der Engine konfiguriert.
- Es bildet Architektur-IDs auf Encoder-Factories ab und Scoring-Arten auf Scorer.
- Es lädt Encoder erst bei Bedarf, für eine Device-Klasse.
- Es stellt den geladenen Encoder als verwaltbare Ressource bereit. Sie hat einen Key und Initialisierungskosten 100.

**Grenzen.**
- Das Modul führt keine Pipeline aus, kein Batching, kein Tokenizing und kein Scoring. Es plant nicht. Es entscheidet nicht, wann geladen wird.
- Es führt keine Zustandsmaschine, keine TTL und keine Eviction. Das gehört zu `resource_lifecycle`. Das Modul liefert dafür nur Key, Kosten und Loader.
- Es lädt beim Registrieren nichts. Die Registrierung ist reiner Zustand.
- Es nutzt kein `unsafe`. Es ändert `Cargo.toml` nicht und fügt keine Dependencies hinzu. Fehler-Displays schreibt es von Hand, außer `thiserror` ist ohnehin im Projekt.
- Es gibt keinen Fallback auf eine andere Architektur und keinen Retry.
- Es gibt keine globalen Zustände (kein `static mut`, kein Singleton).

## 2. Kontext

- **Modellbeschreibung:** `ModelDescriptor` ist reine Daten. `architecture` ist die Architektur-ID, z. B. `"bert"`. `validate()` prüft den Descriptor.
- **Encoder-Schnittstelle:** `Encoder` wird mit `Encoder::new(model, projection, pooling, normalization, storage_dtype)` gebaut. Die Factory liefert einen fertigen `Encoder`.
- **Ressourcen-Lebenszyklus:** `ResourceRegistry<H>` verwaltet Ressourcen über `ResourceKey = String` und `Arc<dyn ResourceLoader<H>>`. Der Aufrufer ruft dort `register`. Das Laden läuft synchron in `LoadTicket::run` über `ResourceLoader::load`. Beim Evict oder Verwerfen des letzten `Arc<H>` wird `H` gedroppt. Das gibt die Gewichte frei.
- **Kernel und Scoring:** `Scorer` ist zustandslos und `Send + Sync`. `Metric { Dot, NegL2Sq }` ist die Scoring-Art. `Metric::kernel_name()` liefert `"score_dot"` bzw. `"score_negl2"`.
- **Device und Lane und Event / Zielumgebung:** Device-Klassen kommen aus `model_descriptor::DeviceClass { Cpu, Gpu }`. Das Backend (candle, CUDA) liegt hinter dem Feature-Flag `bert`. Die Zielumgebung bietet keinen BF16-Compute- und keinen Tensor-Core-Pfad. Das Modul beachtet das nur über den Descriptor.

## 3. Interface

```rust
use std::collections::HashMap;
use std::sync::Arc;
use crate::model_descriptor::{DeviceClass, ModelConfigError, ModelDescriptor};
use crate::encoder_interface::Encoder;
use crate::kernel_scoring::{Metric, Scorer};
use crate::resource_lifecycle::{ResourceKey, ResourceLoader};

/// Scoring-Art = Metrik.
pub type ScoringKind = Metric;

/// Initialisierungskosten der Encoder-Ressource.
pub const ENCODER_INIT_COST: u32 = 100;

/// Konstruktor-Funktion für eine Architektur. Baut aus Descriptor und Device-Klasse
/// einen Encoder (Architektur/Code + Gewichte/Daten auf dem Device). Blockiert bis fertig.
pub trait EncoderFactory: Send + Sync {
    fn build(&self, descriptor: &ModelDescriptor, class: DeviceClass) -> Result<Encoder, String>;
}
// Für jede Closure `Fn(&ModelDescriptor, DeviceClass) -> Result<Encoder, String> + Send + Sync`
// gilt eine Blanket-Impl von EncoderFactory.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Anzeige: "nicht unterstützte Architektur: {0}"
    UnsupportedArchitecture(String),
    /// Descriptor ungültig. Anzeige: Display des inneren Fehlers.
    Config(ModelConfigError),
    /// Factory schlug fehl. Anzeige: "Encoder-Laden fehlgeschlagen ({architecture}): {reason}"
    Load { architecture: String, reason: String },
    /// Anzeige: "Scorer nicht verfügbar: {scorer}"
    ScorerUnavailable { kind: ScoringKind, scorer: String },
}
impl std::fmt::Display for RegistryError {}
impl std::error::Error for RegistryError {}

/// Geladener Encoder: Beschreibung, Device-Klasse und Encoder, nur lesend zugänglich.
/// Das ist der Ressourcen-Handle `H` für `ResourceRegistry<LoadedEncoder>`.
pub struct LoadedEncoder { /* private: Arc<ModelDescriptor>, DeviceClass, Encoder */ }
impl LoadedEncoder {
    pub fn descriptor(&self) -> &ModelDescriptor;
    pub fn device_class(&self) -> DeviceClass;
    pub fn encoder(&self) -> &Encoder;
}
// LoadedEncoder: Send + Sync + 'static (Compile-Time-Assertion im Test).
// Kein Clone. Drop entlädt die Gewichte.

/// Verwaltbare Ressource: Key, Initialisierungskosten und Loader.
pub struct EncoderResource {
    pub key: ResourceKey,
    pub init_cost: u32,                                   // == ENCODER_INIT_COST
    pub loader: Arc<dyn ResourceLoader<LoadedEncoder>>,
}

/// Key aus Architektur-ID und Device-Klasse: "{architecture}:{cpu|gpu}", z. B. "bert:gpu".
pub fn encoder_key(architecture: &str, class: DeviceClass) -> ResourceKey;

pub struct ModelRegistry { /* HashMap<String, Arc<dyn EncoderFactory>>, HashMap<ScoringKind, Arc<Scorer>> */ }
// ModelRegistry: Send + Sync + 'static

impl ModelRegistry {
    /// Leeres Register.
    pub fn new() -> ModelRegistry;
    /// Register mit allen in den Build einkompilierten Architekturen.
    /// Enthält "bert" nur mit `#[cfg(feature = "bert")]`. Ohne Feature ist es leer.
    /// Enthält keine Scorer.
    pub fn with_builtin() -> ModelRegistry;

    /// Registriert oder ersetzt die Factory der Architektur-ID. Gibt die ersetzte zurück.
    pub fn register_architecture(&mut self, architecture: impl Into<String>, factory: Arc<dyn EncoderFactory>)
        -> Option<Arc<dyn EncoderFactory>>;
    /// Registriert oder ersetzt den Scorer der Scoring-Art. Gibt den ersetzten zurück.
    pub fn register_scorer(&mut self, kind: ScoringKind, scorer: Arc<Scorer>) -> Option<Arc<Scorer>>;

    pub fn supports_architecture(&self, architecture: &str) -> bool;
    /// Registrierte Architektur-IDs, aufsteigend sortiert.
    pub fn architectures(&self) -> Vec<String>;

    /// true, wenn für die Scoring-Art ein Scorer registriert ist.
    pub fn scorer_available(&self, kind: ScoringKind) -> bool;
    /// Ok(Scorer) oder Err(ScorerUnavailable { kind, scorer: kind.kernel_name().to_string() }).
    pub fn scorer(&self, kind: ScoringKind) -> Result<Arc<Scorer>, RegistryError>;

    /// Lädt synchron und sofort einen Encoder für die Device-Klasse.
    /// Reihenfolge: 1. descriptor.validate(); 2. Factory zu descriptor.architecture suchen;
    /// 3. factory.build(descriptor, class).
    pub fn load_encoder(&self, descriptor: &ModelDescriptor, class: DeviceClass)
        -> Result<LoadedEncoder, RegistryError>;

    /// Baut die verwaltbare Ressource. Lädt nichts und prüft nichts.
    /// Der Loader ruft beim Laden `load_encoder` mit Descriptor und Klasse auf.
    /// Er bildet Fehler mit `to_string()` auf `Err(String)` ab.
    pub fn encoder_resource(self: &Arc<Self>, descriptor: Arc<ModelDescriptor>, class: DeviceClass)
        -> EncoderResource;
}
impl Default for ModelRegistry {}   // == new()
```

## 4. Behavior und Invariants

**Festigkeit und Nebenläufigkeit.**
- Das Register wird mit `&mut self` vor der Engine-Initialisierung konfiguriert. Danach wird es in `Arc<ModelRegistry>` geteilt und nur noch lesend benutzt.
- Alle Leseoperationen sind thread-sicher. Es gibt keine innere Mutation und keine Locks.
- Mehrere Threads dürfen gleichzeitig `load_encoder` aufrufen. Das Modul serialisiert nichts. Die Einmaligkeit des Ladens garantiert `resource_lifecycle`.
- Ein neues Modell bedeutet einen neuen Eintrag über `register_architecture`. Es sind keine anderen Codeänderungen nötig.

**Registrierung.**
- `register_architecture` mit einer vorhandenen ID ersetzt die bisherige Factory und gibt die alte zurück (`Some`). Bei einer neuen ID gibt es `None` zurück.
- `register_scorer` verhält sich für dieselbe Scoring-Art genauso.
- Die ID-Suche ist exakt und case-sensitiv.
- Die Registrierung lädt nichts und ruft keine Factory auf.

**`with_builtin`.**
- Mit Feature `bert` enthält es die Architektur `"bert"`. Ohne Feature enthält es sie nicht, und `load_encoder` für `"bert"` liefert `UnsupportedArchitecture("bert")`.
- Der bert-Eintrag steckt in einer einzigen privaten `#[cfg(feature = "bert")]`-Funktion. Sie baut `TransformerModel`, optional `Projection` und `Encoder::new` aus dem Descriptor.
  - Sie liest `config_path`, `weight_paths`, `projection_layers`, `pooling`, `normalization`, `storage_dtype` und `compute_dtype(class)`.
  - Mapping von Pooling: `Pooling::Mean` wird zu `PoolingKind::Mean`. Für `Pooling::Cls` gibt es in `PoolingKind` kein Gegenstück, daher meldet die Factory `Err` mit Begründung.
  - Fehler der Modellbausteine werden zu `Err(text)`.
  - Die Loader-API von `transformer_model` steht nicht in den vorliegenden Spezifikationen. Der Implementierer nutzt sie, wie sie dort vorliegt.

**`load_encoder`.**
- Die Schritte laufen strikt in der Reihenfolge Validieren, Factory suchen, Konstruieren. Der erste Fehler gewinnt, und spätere Schritte laufen nicht.
- Ein ungültiger Descriptor ergibt `Config(e)`. Die Factory wird nicht aufgerufen.
- Eine unbekannte `architecture` ergibt `UnsupportedArchitecture(<ID>)`. Der Text enthält die ID. Das gilt auch für eine leere ID.
- Ein `Err(reason)` der Factory ergibt `Load { architecture, reason }`.
- Ein Panic der Factory wird nicht abgefangen. Der Aufrufer `LoadTicket::run` fängt ihn.
- Bei Erfolg enthält das `LoadedEncoder`:
  - einen Klon des Descriptors (`descriptor()` ist gleich dem übergebenen Wert),
  - die übergebene Device-Klasse (`device_class()`),
  - den von der Factory gebauten Encoder.
  - Das Modul verändert nichts davon. Es gibt nur lesenden Zugriff.
- Das Modul prüft nicht, ob `encoder.output_dim()` zu `descriptor.dimension` passt, und führt keine weiteren Prüfungen ein.

**Scorer.**
- `scorer_available(k)` ist genau dann `true`, wenn für `k` ein Scorer registriert ist.
- `scorer(k)` ohne Eintrag liefert `ScorerUnavailable { kind: k, scorer: k.kernel_name() }`. Für `Dot` ist das `"score_dot"`, für `NegL2Sq` `"score_negl2"`. Der Display-Text nennt diesen Namen.
- Mit Eintrag liefert `scorer(k)` einen Klon des `Arc`. Es lädt oder erzeugt nichts.

**Ressource.**
- `encoder_resource` liefert:
  - `key == encoder_key(&descriptor.architecture, class)`,
  - `init_cost == 100` (`ENCODER_INIT_COST`).
- `encoder_key` hat das Format `"{architecture}:{cpu|gpu}"`, mit `cpu` für `DeviceClass::Cpu` und `gpu` für `DeviceClass::Gpu`.
  - Gleiche Architektur und gleiche Klasse ergeben denselben Key.
  - Verschiedene Klassen oder Architekturen ergeben verschiedene Keys.
- `encoder_resource` ist infallibel und lädt nicht. Ein Fehler (z. B. unbekannte Architektur) erscheint erst beim Laden, als `Err(String)` mit dem `RegistryError`-Display.
- Der Loader hält einen `Arc<ModelRegistry>` und einen `Arc<ModelDescriptor>`. Er ist `Send + Sync` und mehrfach aufrufbar. Jeder Aufruf baut einen neuen Encoder.
- Freigeben: Die Ressource ist `ResourceRegistry<LoadedEncoder>`. Beim Entladen wird der `Arc<LoadedEncoder>` gedroppt. Wenn kein `Lease` mehr lebt, wird der Encoder und mit ihm das Gewicht freigegeben (Drop). Das Modul hält keine zusätzliche Referenz.
- Der Aufrufer registriert die Ressource selbst per `ResourceRegistry::register(key, class, loader)`. Dieses Modul ruft `ResourceRegistry` nicht auf. `MemoryClass` bestimmt der Aufrufer.

## 5. Relied-on elements

- `model_descriptor`:
  - `ModelDescriptor { architecture: String, dimension, pooling, normalization, storage_dtype, config_path, weight_paths, projection_layers, … }`, `Clone`, `Send + Sync`, alle Felder öffentlich.
  - `ModelDescriptor::validate(&self) -> Result<(), ModelConfigError>` und `compute_dtype(&self, DeviceClass) -> DType`.
  - `DeviceClass { Cpu, Gpu }` (`Copy, Eq, Hash`).
  - `ModelConfigError::Incompatible { reason }` (`Clone, PartialEq, Eq`) mit Display `"inkompatible Modellkonfiguration: {reason}"`.
  - `Pooling { Mean, Cls }` und `Normalization { L2, None }`.
- `encoder_interface`:
  - `Encoder::new(Arc<TransformerModel>, Option<Arc<Projection>>, PoolingKind, Normalization, DType) -> Result<Encoder, EncoderError>`.
  - `Encoder::{output_dim(), storage_dtype()}`. `Encoder` ist `Send + Sync + 'static`.
  - `EncoderError: Display`.
  - `PoolingKind { Mean, None }`.
- `resource_lifecycle`:
  - `type ResourceKey = String`.
  - `trait ResourceLoader<H>: Send + Sync { fn load(&self) -> Result<H, String>; }`.
  - Der Loader läuft synchron in `LoadTicket::run`, und ein Panic wird dort abgefangen.
  - Das Entladen droppt den `Arc<H>` der Registry. Leases bleiben gültig.
- `kernel_scoring`:
  - `Metric { Dot, NegL2Sq }` (`Copy, Eq, Hash`) und `Metric::kernel_name() -> &'static str`.
  - `Scorer: Send + Sync`, zustandslos, geteilt über `Arc<Scorer>`.
- Device und Zielumgebung: Es gibt keinen direkten Zugriff. Das Modul nutzt `DeviceClass` nur als Label und löst kein `DeviceHandle` auf. Es gibt keinen BF16-Compute- und keinen Tensor-Core-Pfad.