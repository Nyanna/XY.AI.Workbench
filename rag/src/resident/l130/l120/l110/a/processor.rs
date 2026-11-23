//! Owns the task queue and the three operations queues and routes planned operations into them.
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::a::plan_contract::{QueueKind, TaskPlanner};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ResourceKind;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::{
    Task, TaskParams,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::{
    Queue, Ticket,
};
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
/// Task-queue metadata: the task params and the resource kind of the task's first operation, if known.
/// The Processor does not interpret either.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskMeta {
    pub params: TaskParams,
    pub first_kind: Option<ResourceKind>,
}
pub type TaskQueue<D, T, E> = Queue<TaskMeta, Task<D, T, E>, (), ()>;
pub type OpsQueue<O, V, R, OE> = Queue<O, V, R, OE>;
struct Queues<D, T, E, O, V, R, OE> {
    task: TaskQueue<D, T, E>,
    cpu: Arc<OpsQueue<O, V, R, OE>>,
    gpu: Arc<OpsQueue<O, V, R, OE>>,
    shared: Arc<OpsQueue<O, V, R, OE>>,
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
    PL: TaskPlanner<D, T, E, O, V>,
{
    pub fn new(planner: PL) -> Arc<Self> {
        Arc::new(Self {
            planner,
            queues: Queues {
                task: Queue::new(),
                cpu: Arc::new(Queue::new()),
                gpu: Arc::new(Queue::new()),
                shared: Arc::new(Queue::new()),
            },
        })
    }
    pub fn submit(
        &self,
        data: D,
        params: TaskParams,
        first_kind: Option<ResourceKind>,
    ) -> Task<D, T, E> {
        let task = Task::new(data, params.clone());
        self.queues.task.push(TaskMeta { params, first_kind }, task.clone());
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
        self.ops_queue_arc(kind)
    }
    /// Shareable handle to the same queue `ops_queue` returns.
    pub fn ops_queue_arc(&self, kind: QueueKind) -> &Arc<OpsQueue<O, V, R, OE>> {
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
