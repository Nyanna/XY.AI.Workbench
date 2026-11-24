use std::fmt;
use crate::resident::l130::l120::l110::l100::l090::l080;
use l080::a::kernel_scoring::Metric;
use l080::l070::l060::l050::l040::b::embedding_task::Precision;
use l080::l070::l060::l050::l040::b::similarity_task::Scoring;
use l080::l070::l060::l050::l040::l030::l020::a::supported_precisions::has_scoring_kernel;
use l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
use l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreTransform {
    Identity,
    NegRoot,
}
impl ScoreTransform {
    pub fn apply(self, score: f32) -> f32 {
        match self {
            ScoreTransform::Identity => score,
            ScoreTransform::NegRoot => {
                let d = -score;
                -(if d < 0.0 { 0.0 } else { d }).sqrt()
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoringPlan {
    pub metric: Metric,
    pub transform: ScoreTransform,
    pub dtype: DType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrecisionError(pub &'static str);
impl fmt::Display for PrecisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "precision {} has no scoring kernel", self.0)
    }
}
impl std::error::Error for PrecisionError {}
/// Cosine maps to `Dot` because encoder output is L2-normalized.
pub fn metric_for(scoring: Scoring) -> (Metric, ScoreTransform) {
    match scoring {
        Scoring::Cosine | Scoring::DotProduct => (Metric::Dot, ScoreTransform::Identity),
        Scoring::NegEuclidean => (Metric::NegL2Sq, ScoreTransform::NegRoot),
    }
}
pub fn dtype_for(precision: Precision) -> Result<DType, PrecisionError> {
    let (dtype, name) = match precision {
        Precision::F32 => (DType::F32, "F32"),
        Precision::F16 => (DType::F16, "F16"),
        Precision::Bf16 => (DType::BF16, "Bf16"),
        Precision::Int8 => return Err(PrecisionError("Int8")),
    };
    if [DeviceKind::Cpu, DeviceKind::Gpu]
        .into_iter()
        .any(|kind| has_scoring_kernel(kind, dtype))
    {
        Ok(dtype)
    } else {
        Err(PrecisionError(name))
    }
}
pub fn plan(
    scoring: Scoring,
    precision: Precision,
) -> Result<ScoringPlan, PrecisionError> {
    let dtype = dtype_for(precision)?;
    let (metric, transform) = metric_for(scoring);
    Ok(ScoringPlan {
        metric,
        transform,
        dtype,
    })
}
#[cfg(test)]
#[path = "test/scoring_plan.rs"]
mod tests;
