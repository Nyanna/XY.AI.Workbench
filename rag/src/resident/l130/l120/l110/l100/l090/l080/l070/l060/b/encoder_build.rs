use std::fmt;
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::encoder_interface::projection_layer::{
    Projection, ProjectionError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::encoder_interface::transformer_model::{
    TransformerError, TransformerModel,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::encoder_interface::{
    Encoder, EncoderError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::weight_source::WeightFile;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    DeviceClass, ModelConfigError, ModelDescriptor, Normalization, Pooling,
    ProjectionLayer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
pub const DEFAULT_NORMALIZATION: Normalization = Normalization::L2;
pub fn resolve_normalization(n: Option<Normalization>) -> Normalization {
    n.unwrap_or(DEFAULT_NORMALIZATION)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncoderBuildError {
    Incompatible { reason: String },
    Load { reason: String },
    WeightNameMismatch { name: String },
}
impl fmt::Display for EncoderBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncoderBuildError::Incompatible { reason } => {
                write!(f, "inkompatibel: {reason}")
            }
            EncoderBuildError::Load { reason } => write!(f, "Load-Error: {reason}"),
            EncoderBuildError::WeightNameMismatch { name } => {
                write!(f, "Gewichtsname passt nicht: {name}")
            }
        }
    }
}
impl std::error::Error for EncoderBuildError {}
fn incompatible(reason: impl Into<String>) -> EncoderBuildError {
    EncoderBuildError::Incompatible {
        reason: reason.into(),
    }
}
fn load_error(reason: impl Into<String>) -> EncoderBuildError {
    EncoderBuildError::Load {
        reason: reason.into(),
    }
}
/// Assembled encoder plus tracing metadata. Immutable.
pub struct BuiltEncoder {
    encoder: Encoder,
    model: Arc<TransformerModel>,
    projection: Option<Arc<Projection>>,
    device_class: DeviceClass,
    compute_dtype: DType,
    multi_vector: bool,
}
impl BuiltEncoder {
    pub fn encoder(&self) -> &Encoder {
        &self.encoder
    }
    pub fn into_encoder(self) -> Encoder {
        self.encoder
    }
    pub fn model(&self) -> &Arc<TransformerModel> {
        &self.model
    }
    pub fn projection(&self) -> Option<&Arc<Projection>> {
        self.projection.as_ref()
    }
    pub fn device_class(&self) -> DeviceClass {
        self.device_class
    }
    pub fn compute_dtype(&self) -> DType {
        self.compute_dtype
    }
    pub fn storage_dtype(&self) -> DType {
        self.encoder.storage_dtype()
    }
    pub fn is_multi_vector(&self) -> bool {
        self.multi_vector
    }
}
fn map_model_error(e: TransformerError) -> EncoderBuildError {
    match e {
        TransformerError::Incompatible { reason } => {
            EncoderBuildError::Incompatible {
                reason,
            }
        }
        TransformerError::WeightNameMismatch { name } => {
            EncoderBuildError::WeightNameMismatch {
                name,
            }
        }
        TransformerError::Load { reason }
        | TransformerError::EncodingFailed { reason } => {
            EncoderBuildError::Load { reason }
        }
    }
}
fn map_projection_error(e: ProjectionError) -> EncoderBuildError {
    match e {
        ProjectionError::WeightNameMismatch { name } => {
            EncoderBuildError::WeightNameMismatch {
                name,
            }
        }
        ProjectionError::IncompatibleShape { tensor, actual, expected } => {
            incompatible(
                format!("Tensor {tensor} hat Shape {actual:?}, erwartet {expected:?}"),
            )
        }
        ProjectionError::IncompatibleDType { tensor, dtype } => {
            incompatible(format!("Tensor {tensor} hat nicht unterstützten Typ {dtype}"))
        }
        ProjectionError::InputDimMismatch { expected, actual } => {
            incompatible(
                format!("Projection-Eingabedimension {actual} statt {expected}"),
            )
        }
        ProjectionError::Load { path: Some(path), reason } => {
            load_error(format!("{reason} ({})", path.display()))
        }
        ProjectionError::Load { path: None, reason } => load_error(reason),
    }
}
fn load_projection(
    desc: &ModelDescriptor,
    layer: &ProjectionLayer,
    model_dim: usize,
) -> Result<Projection, EncoderBuildError> {
    let path = desc
        .weight_paths
        .first()
        .ok_or_else(|| load_error("keine Gewichtsdateien"))?;
    let file = WeightFile::open(path).map_err(|e| load_error(e.to_string()))?;
    let info = file
        .info(&layer.tensor_name)
        .ok_or_else(|| {
            EncoderBuildError::WeightNameMismatch {
                name: layer.tensor_name.clone(),
            }
        })?;
    let (output, input) = match info.shape.as_slice() {
        [output, input] => (*output, *input),
        other => {
            return Err(
                incompatible(
                    format!(
                        "Projection-Gewicht {} hat Rank {}, erwartet 2 ([output, input])",
                        layer.tensor_name, other.len()
                    ),
                ),
            );
        }
    };
    if input != model_dim {
        return Err(
            incompatible(
                format!(
                    "Projection-Eingabe {input} inkompatibel zur Hauptmodell-Ausgabe {model_dim}"
                ),
            ),
        );
    }
    if let Some(mv) = &desc.multi_vector {
        if output != mv.dimension {
            return Err(
                incompatible(
                    format!(
                        "Projection-Ausgabe {output} passt nicht zur Multi-Vector-Dimension {}",
                        mv.dimension
                    ),
                ),
            );
        }
    }
    Projection::from_weight_file(&file, layer, input, output)
        .map_err(map_projection_error)
}
pub fn build_encoder(
    desc: &ModelDescriptor,
    class: DeviceClass,
) -> Result<BuiltEncoder, EncoderBuildError> {
    desc.validate()
        .map_err(|e| match e {
            ModelConfigError::Incompatible { reason } => {
                EncoderBuildError::Incompatible {
                    reason,
                }
            }
        })?;
    let multi_vector = desc.multi_vector.is_some();
    let pooling = if multi_vector {
        Pooling::PerToken
    } else {
        match desc.pooling {
            Pooling::Mean | Pooling::PerToken => desc.pooling,
            Pooling::Cls => {
                return Err(incompatible("Pooling Cls wird nicht unterstützt"));
            }
        }
    };
    if multi_vector && desc.projection_layers.is_empty() {
        return Err(incompatible("Projection-Head fehlt"));
    }
    if desc.projection_layers.len() > 1 {
        return Err(
            incompatible(
                format!(
                    "{} Projection-Layers angegeben, der Encoder unterstützt genau eine",
                    desc.projection_layers.len()
                ),
            ),
        );
    }
    if !desc.projection_layers.is_empty() && desc.weight_paths.is_empty() {
        return Err(load_error("keine Gewichtsdateien"));
    }
    let model = Arc::new(TransformerModel::load(desc, class).map_err(map_model_error)?);
    let projection = match desc.projection_layers.first() {
        Some(layer) => Some(Arc::new(load_projection(desc, layer, model.dimension())?)),
        None => None,
    };
    let normalization = resolve_normalization(Some(desc.normalization));
    let encoder = Encoder::new(
            Arc::clone(&model),
            projection.clone(),
            pooling,
            normalization,
            desc.storage_dtype,
        )
        .map_err(|e| match e {
            EncoderError::Incompatible { .. } => incompatible(e.to_string()),
            other => load_error(other.to_string()),
        })?;
    Ok(BuiltEncoder {
        encoder,
        model,
        projection,
        device_class: class,
        compute_dtype: desc.compute_dtype(class),
        multi_vector,
    })
}
#[cfg(test)]
#[path = "test/encoder_build.rs"]
mod tests;
