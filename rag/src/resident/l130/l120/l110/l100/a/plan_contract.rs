use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::Task;

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
