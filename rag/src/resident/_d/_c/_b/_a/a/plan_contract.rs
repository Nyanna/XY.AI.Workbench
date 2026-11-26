use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::Task;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueueKind {
    Cpu,
    Gpu,
    Shared,
}

pub struct PlannedOp<O, V> {
    pub kind: QueueKind,
    pub meta: O,
    pub values: V,
}

pub trait TaskPlanner<D, T, E, O, V>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn expand(&self, task: &Task<D, T, E>) -> Result<Vec<PlannedOp<O, V>>, Self::Error>;
}
