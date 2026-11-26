//! BERT hidden-state encoder over memory-mapped safetensors weights.
use std::fmt;
use std::path::Path;
use candle_core::{DType as LibDType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, HiddenAct};
use serde_json::{Map, Value};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::to_lib_dtype;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::weight_source::WeightFile;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::{
    DeviceClass, ModelDescriptor,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceHandle;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformerError {
    Incompatible { reason: String },
    WeightNameMismatch { name: String },
    Load { reason: String },
    EncodingFailed { reason: String },
}
impl fmt::Display for TransformerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformerError::Incompatible { reason } => {
                write!(f, "inkompatible Modellkonfiguration: {reason}")
            }
            TransformerError::WeightNameMismatch { name } => {
                write!(f, "Gewichtsname passt nicht: {name}")
            }
            TransformerError::Load { reason } => write!(f, "Load-Error: {reason}"),
            TransformerError::EncodingFailed { reason } => {
                write!(f, "Encoding fehlgeschlagen: {reason}")
            }
        }
    }
}
impl std::error::Error for TransformerError {}
fn incompatible4(reason: impl Into<String>) -> TransformerError {
    TransformerError::Incompatible {
        reason: reason.into(),
    }
}
fn load_error3(reason: impl Into<String>) -> TransformerError {
    TransformerError::Load {
        reason: reason.into(),
    }
}
fn encoding_failed(reason: impl Into<String>) -> TransformerError {
    TransformerError::EncodingFailed {
        reason: reason.into(),
    }
}
/// Loaded BERT model. Immutable after `load`.
pub struct TransformerModel {
    model: BertModel,
    device: Device,
    config: Config,
    handle: DeviceHandle,
    dtype: DType,
    hidden: usize,
    max_seq_len: usize,
}
struct ParsedConfig {
    config: Config,
    model_type: String,
    position_embedding_type: String,
}
struct ConfigFields<'a> {
    obj: &'a Map<String, Value>,
    path: String,
}
impl ConfigFields<'_> {
    fn problem(&self, key: &str, why: &str) -> TransformerError {
        load_error3(format!("Config {}: Feld \"{key}\" {why}", self.path))
    }
    fn present(&self, key: &str) -> Option<&Value> {
        self.obj.get(key).filter(|v| !v.is_null())
    }
    fn unsigned(&self, key: &str) -> Result<Option<usize>, TransformerError> {
        match self.present(key) {
            None => Ok(None),
            Some(v) => {
                v.as_u64()
                    .and_then(|n| usize::try_from(n).ok())
                    .map(Some)
                    .ok_or_else(|| {
                        self.problem(key, "ist keine nicht-negative Ganzzahl")
                    })
            }
        }
    }
    fn required_positive(&self, key: &str) -> Result<usize, TransformerError> {
        match self.unsigned(key)? {
            None => Err(self.problem(key, "fehlt")),
            Some(0) => Err(self.problem(key, "muss größer als 0 sein")),
            Some(n) => Ok(n),
        }
    }
    fn optional_unsigned(
        &self,
        key: &str,
        default: usize,
    ) -> Result<usize, TransformerError> {
        Ok(self.unsigned(key)?.unwrap_or(default))
    }
    fn optional_float(&self, key: &str, default: f64) -> Result<f64, TransformerError> {
        match self.present(key) {
            None => Ok(default),
            Some(v) => {
                v.as_f64()
                    .filter(|x| x.is_finite())
                    .ok_or_else(|| self.problem(key, "ist keine endliche Zahl"))
            }
        }
    }
    fn optional_str(&self, key: &str) -> Result<Option<&str>, TransformerError> {
        match self.present(key) {
            None => Ok(None),
            Some(v) => {
                v.as_str()
                    .map(Some)
                    .ok_or_else(|| self.problem(key, "ist keine Zeichenkette"))
            }
        }
    }
}
fn parse_hidden_act(name: &str) -> Option<HiddenAct> {
    match name {
        "gelu" => Some(HiddenAct::Gelu),
        "gelu_new" | "gelu_pytorch_tanh" | "gelu_fast" => {
            Some(HiddenAct::GeluApproximate)
        }
        "relu" => Some(HiddenAct::Relu),
        _ => None,
    }
}
fn read_config(path: &Path) -> Result<ParsedConfig, TransformerError> {
    let shown = path.display().to_string();
    let text = std::fs::read_to_string(path)
        .map_err(|e| load_error3(format!("Config {shown} nicht lesbar: {e}")))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| load_error3(
            format!("Config {shown} ist kein gültiges JSON: {e}"),
        ))?;
    let obj = value
        .as_object()
        .ok_or_else(|| load_error3(format!("Config {shown}: JSON-Objekt erwartet")))?;
    let fields = ConfigFields { obj, path: shown };
    let model_type = match fields.present("model_type") {
        None => return Err(fields.problem("model_type", "fehlt")),
        Some(v) => {
            v.as_str()
                .ok_or_else(|| fields.problem("model_type", "ist keine Zeichenkette"))?
                .to_string()
        }
    };
    let hidden_size = fields.required_positive("hidden_size")?;
    let num_hidden_layers = fields.required_positive("num_hidden_layers")?;
    let num_attention_heads = fields.required_positive("num_attention_heads")?;
    let intermediate_size = fields.required_positive("intermediate_size")?;
    let vocab_size = fields.required_positive("vocab_size")?;
    let max_position_embeddings = fields.required_positive("max_position_embeddings")?;
    let defaults = Config::default();
    let hidden_act = match fields.optional_str("hidden_act")? {
        None => defaults.hidden_act,
        Some(name) => {
            parse_hidden_act(name)
                .ok_or_else(|| {
                    fields.problem("hidden_act", &format!("unbekannt: \"{name}\""))
                })?
        }
    };
    let type_vocab_size = fields
        .optional_unsigned("type_vocab_size", defaults.type_vocab_size)?;
    if type_vocab_size == 0 {
        return Err(fields.problem("type_vocab_size", "muss größer als 0 sein"));
    }
    let position_embedding_type = fields
        .optional_str("position_embedding_type")?
        .unwrap_or("absolute")
        .to_string();
    let config = Config {
        vocab_size,
        hidden_size,
        num_hidden_layers,
        num_attention_heads,
        intermediate_size,
        hidden_act,
        hidden_dropout_prob: fields
            .optional_float("hidden_dropout_prob", defaults.hidden_dropout_prob)?,
        max_position_embeddings,
        type_vocab_size,
        initializer_range: fields
            .optional_float("initializer_range", defaults.initializer_range)?,
        layer_norm_eps: fields
            .optional_float("layer_norm_eps", defaults.layer_norm_eps)?,
        pad_token_id: fields.optional_unsigned("pad_token_id", defaults.pad_token_id)?,
        model_type: Some(model_type.clone()),
        ..defaults
    };
    Ok(ParsedConfig {
        config,
        model_type,
        position_embedding_type,
    })
}
fn open_gpu() -> Result<(Device, DeviceHandle), TransformerError> {
    match std::panic::catch_unwind(|| Device::new_cuda(0)) {
        Ok(Ok(device)) => Ok((device, DeviceHandle::Gpu(0))),
        Ok(Err(e)) => Err(load_error3(format!("GPU-Device 0 nicht verfügbar: {e}"))),
        Err(_) => {
            Err(
                load_error3(
                    "GPU-Device 0 nicht verfügbar: Treiber-Initialisierung fehlgeschlagen",
                ),
            )
        }
    }
}
fn expected_tensor_names(prefix: &str, layers: usize) -> Vec<String> {
    let mut names = Vec::with_capacity(5 + layers * 16);
    for part in ["word_embeddings", "position_embeddings", "token_type_embeddings"] {
        names.push(format!("{prefix}embeddings.{part}.weight"));
    }
    for kind in ["weight", "bias"] {
        names.push(format!("{prefix}embeddings.LayerNorm.{kind}"));
    }
    const LAYER_PARTS: [&str; 8] = [
        "attention.self.query",
        "attention.self.key",
        "attention.self.value",
        "attention.output.dense",
        "attention.output.LayerNorm",
        "intermediate.dense",
        "output.dense",
        "output.LayerNorm",
    ];
    for i in 0..layers {
        for part in LAYER_PARTS {
            for kind in ["weight", "bias"] {
                names.push(format!("{prefix}encoder.layer.{i}.{part}.{kind}"));
            }
        }
    }
    names
}
fn missing_tensor(e: &candle_core::Error) -> Option<String> {
    use candle_core::Error as E;
    match e {
        E::CannotFindTensor { path } => Some(path.clone()),
        E::Context { inner, .. }
        | E::WithPath { inner, .. }
        | E::WithBacktrace { inner, .. } => missing_tensor(inner),
        _ => None,
    }
}
fn map_load_error(e: candle_core::Error) -> TransformerError {
    match missing_tensor(&e) {
        Some(name) => {
            TransformerError::WeightNameMismatch {
                name,
            }
        }
        None => load_error3(e.to_string()),
    }
}
impl TransformerModel {
    pub fn load(
        desc: &ModelDescriptor,
        class: DeviceClass,
    ) -> Result<TransformerModel, TransformerError> {
        let ParsedConfig { config, model_type, position_embedding_type } = read_config(
            &desc.config_path,
        )?;
        if model_type != "bert" {
            return Err(
                incompatible4(
                    format!("modeltyp muss \"bert\" sein (gefunden: \"{model_type}\")"),
                ),
            );
        }
        if config.hidden_size != desc.dimension {
            return Err(
                incompatible4(
                    format!(
                        "hidden_size {} passt nicht zur Dimension {} des Modellbeschreibers",
                        config.hidden_size, desc.dimension
                    ),
                ),
            );
        }
        if desc.max_seq_len > config.max_position_embeddings {
            return Err(
                incompatible4(
                    format!(
                        "max_seq_len {} überschreitet max_position_embeddings {}", desc
                        .max_seq_len, config.max_position_embeddings
                    ),
                ),
            );
        }
        if desc.architecture != "bert" {
            return Err(
                incompatible4(
                    format!(
                        "architektur muss \"bert\" sein (gefunden: \"{}\")", desc
                        .architecture
                    ),
                ),
            );
        }
        if config.hidden_size % config.num_attention_heads != 0 {
            return Err(
                incompatible4(
                    format!(
                        "hidden_size {} ist nicht durch num_attention_heads {} teilbar",
                        config.hidden_size, config.num_attention_heads
                    ),
                ),
            );
        }
        if position_embedding_type != "absolute" {
            return Err(
                incompatible4(
                    format!(
                        "position_embedding_type muss \"absolute\" sein (gefunden: \"{position_embedding_type}\")"
                    ),
                ),
            );
        }
        let (device, handle) = match class {
            DeviceClass::Cpu => (Device::Cpu, DeviceHandle::Cpu),
            DeviceClass::Gpu => open_gpu()?,
        };
        let dtype = desc.compute_dtype(class);
        let lib_dtype = to_lib_dtype(dtype);
        if !matches!(lib_dtype, LibDType::F32 | LibDType::F16 | LibDType::BF16) {
            return Err(
                incompatible4(
                    format!(
                        "Datentyp {dtype:?} ist kein Compute-Datentyp (erlaubt: F32, F16, BF16)"
                    ),
                ),
            );
        }
        if desc.weight_paths.is_empty() {
            return Err(load_error3("keine Gewichtsdateien angegeben"));
        }
        let mut files = Vec::with_capacity(desc.weight_paths.len());
        for path in &desc.weight_paths {
            let file = WeightFile::open(path)
                .map_err(|e| load_error3(
                    format!("Gewichtsdatei {}: {e}", path.display()),
                ))?;
            files.push(file);
        }
        let has = |name: &str| files.iter().any(|f| f.info(name).is_some());
        let prefix = if !has("embeddings.word_embeddings.weight")
            && has("bert.embeddings.word_embeddings.weight")
        {
            "bert."
        } else {
            ""
        };
        for name in expected_tensor_names(prefix, config.num_hidden_layers) {
            if !has(&name) {
                return Err(TransformerError::WeightNameMismatch {
                    name,
                });
            }
        }
        drop(files);
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&desc.weight_paths, lib_dtype, &device)
        }
            .map_err(map_load_error)?;
        let vb = if prefix.is_empty() { vb } else { vb.pp("bert") };
        let model = BertModel::load(vb, &config).map_err(map_load_error)?;
        Ok(TransformerModel {
            model,
            device,
            hidden: config.hidden_size,
            max_seq_len: desc.max_seq_len,
            config,
            handle,
            dtype,
        })
    }
    pub fn dimension(&self) -> usize {
        self.hidden
    }
    pub fn max_seq_len(&self) -> usize {
        self.max_seq_len
    }
    pub fn device(&self) -> DeviceHandle {
        self.handle
    }
    pub fn compute_dtype(&self) -> DType {
        self.dtype
    }
    pub fn encode(
        &self,
        ids: &[u32],
        mask: &[u32],
        rows: usize,
        seq_len: usize,
    ) -> Result<Vec<f32>, TransformerError> {
        let cells = rows
            .checked_mul(seq_len)
            .ok_or_else(|| encoding_failed(format!("Überlauf: {rows} * {seq_len}")))?;
        if ids.len() != cells {
            return Err(
                encoding_failed(
                    format!(
                        "ids hat {} Werte, erwartet {rows} * {seq_len} = {cells}", ids
                        .len()
                    ),
                ),
            );
        }
        if mask.len() != cells {
            return Err(
                encoding_failed(
                    format!(
                        "mask hat {} Werte, erwartet {rows} * {seq_len} = {cells}", mask
                        .len()
                    ),
                ),
            );
        }
        if seq_len > self.max_seq_len {
            return Err(
                encoding_failed(
                    format!(
                        "seq_len {seq_len} überschreitet max_seq_len {}", self
                        .max_seq_len
                    ),
                ),
            );
        }
        let vocab = self.config.vocab_size as u64;
        if let Some(bad) = ids.iter().find(|&&id| u64::from(id) >= vocab) {
            return Err(
                encoding_failed(
                    format!("Token-ID {bad} liegt außerhalb von vocab_size {vocab}"),
                ),
            );
        }
        let total = cells
            .checked_mul(self.hidden)
            .ok_or_else(|| encoding_failed(
                format!("Überlauf: {cells} * {}", self.hidden),
            ))?;
        if cells == 0 {
            return Ok(Vec::new());
        }
        let forward = || -> candle_core::Result<Vec<f32>> {
            let shape = (rows, seq_len);
            let input_ids = Tensor::from_vec(ids.to_vec(), shape, &self.device)?;
            let token_types = input_ids.zeros_like()?;
            let valid: Vec<u32> = mask.iter().map(|&m| u32::from(m != 0)).collect();
            let attention = Tensor::from_vec(valid, shape, &self.device)?;
            let hidden = self.model.forward(&input_ids, &token_types, Some(&attention))?;
            hidden.to_dtype(LibDType::F32)?.flatten_all()?.to_vec1::<f32>()
        };
        let out = forward().map_err(|e| encoding_failed(e.to_string()))?;
        if out.len() != total {
            return Err(
                encoding_failed(format!("Ausgabelänge {} statt {total}", out.len())),
            );
        }
        if out.iter().any(|v| !v.is_finite()) {
            return Err(encoding_failed("Ausgabe enthält nicht-endliche Werte"));
        }
        Ok(out)
    }
}
#[cfg(test)]
#[path = "test/transformer_model.rs"]
mod tests;
