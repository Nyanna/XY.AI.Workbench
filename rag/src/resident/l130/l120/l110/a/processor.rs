//! Owns the task queue and the three operations queues and routes planned operations into them.
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::{
    Task, TaskParams,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::{
    Queue, Ticket,
};
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
pub trait Planner<D, T, E, O, V>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    fn expand(&self, task: &Task<D, T, E>) -> Result<Vec<PlannedOp<O, V>>, Self::Error>;
}
#[derive(Debug)]
pub enum ExpandError<PE> {
    Planner(PE),
}
impl<PE: std::fmt::Display> std::fmt::Display for ExpandError<PE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpandError::Planner(e) => write!(f, "planner error: {e}"),
        }
    }
}
impl<PE: std::error::Error + 'static> std::error::Error for ExpandError<PE> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExpandError::Planner(e) => Some(e),
        }
    }
}
pub type TaskQueue<D, T, E> = Queue<TaskParams, Task<D, T, E>, (), ()>;
pub type OpsQueue<O, V, R, OE> = Queue<O, V, R, OE>;
struct Queues<D, T, E, O, V, R, OE> {
    task: TaskQueue<D, T, E>,
    cpu: OpsQueue<O, V, R, OE>,
    gpu: OpsQueue<O, V, R, OE>,
    shared: OpsQueue<O, V, R, OE>,
}
pub struct Processor<D, T, E, O, V, R, OE, PL> {
    planner: PL,
    queues: Queues<D, T, E, O, V, R, OE>,
}
impl<D, T, E, O, V, R, OE, PL> Processor<D, T, E, O, V, R, OE, PL>
where
    D: Send + 'static,
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    O: Send + Sync + 'static,
    V: Send + 'static,
    R: Send + 'static,
    OE: Send + 'static,
    PL: Planner<D, T, E, O, V>,
{
    pub fn new(planner: PL) -> Arc<Self> {
        Arc::new(Self {
            planner,
            queues: Queues {
                task: Queue::new(),
                cpu: Queue::new(),
                gpu: Queue::new(),
                shared: Queue::new(),
            },
        })
    }
    pub fn submit(&self, data: D, params: TaskParams) -> Task<D, T, E> {
        let task = Task::new(data, params.clone());
        self.queues.task.push(params, task.clone());
        task
    }
    pub fn expand(
        &self,
        task: &Task<D, T, E>,
    ) -> Result<Vec<(QueueKind, Ticket<O, V, R, OE>)>, ExpandError<PL::Error>> {
        let ops = self.planner.expand(task).map_err(ExpandError::Planner)?;
        let mut tickets = Vec::with_capacity(ops.len());
        for op in ops {
            let ticket = self.ops_queue(op.kind).push(op.meta, op.values);
            tickets.push((op.kind, ticket));
        }
        Ok(tickets)
    }
    pub fn task_queue(&self) -> &TaskQueue<D, T, E> {
        &self.queues.task
    }
    pub fn ops_queue(&self, kind: QueueKind) -> &OpsQueue<O, V, R, OE> {
        match kind {
            QueueKind::Cpu => &self.queues.cpu,
            QueueKind::Gpu => &self.queues.gpu,
            QueueKind::Shared => &self.queues.shared,
        }
    }
}
#[cfg(test)]
#[path = "test/processor.rs"]
mod tests;
