use std::fmt;
use super::deps::*;
#[derive(Debug)]
pub enum GpuError {
    InvalidConfig(String),
    NoDevice,
    Backend(String),
    Capability(DetectError),
    Subengine(SubengineError),
    Memory(AllocatorError),
    Budget(BudgetError),
    Lifecycle(LifecycleError),
    IndexMemory(GpuManagerError),
    IndexOp(IndexOpError),
    Table(TableSwapError),
    Weights(WeightError),
    LaneSet(LaneSetError),
    UnknownModel(ModelId),
    UnknownIndex(IndexId),
    AlreadyAttached(IndexId),
    AlreadyRegistered(ModelId),
}
impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuError::InvalidConfig(m) => write!(f, "ungültige GPU-Konfiguration: {m}"),
            GpuError::NoDevice => write!(f, "keine nutzbare GPU"),
            GpuError::Backend(m) => write!(f, "GPU-Backend: {m}"),
            GpuError::Capability(e) => write!(f, "Capability-Erkennung: {e}"),
            GpuError::Subengine(e) => write!(f, "{e}"),
            GpuError::Memory(e) => write!(f, "VRAM-Speicher: {e}"),
            GpuError::Budget(e) => write!(f, "VRAM-Budget: {e}"),
            GpuError::Lifecycle(e) => write!(f, "Ressourcen-Lebenszyklus: {e:?}"),
            GpuError::IndexMemory(e) => write!(f, "{e}"),
            GpuError::IndexOp(e) => write!(f, "{e}"),
            GpuError::Table(e) => write!(f, "{e}"),
            GpuError::Weights(e) => write!(f, "{e}"),
            GpuError::LaneSet(e) => write!(f, "{e}"),
            GpuError::UnknownModel(m) => write!(f, "Modell {m:?} nicht registriert"),
            GpuError::UnknownIndex(i) => write!(f, "Index {i:?} nicht angebunden"),
            GpuError::AlreadyAttached(i) => write!(f, "Index {i:?} bereits angebunden"),
            GpuError::AlreadyRegistered(m) => {
                write!(f, "Modell {m:?} bereits registriert")
            }
        }
    }
}
impl std::error::Error for GpuError {}
static_assertions::assert_impl_all!(GpuError : Send, Sync);
#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
