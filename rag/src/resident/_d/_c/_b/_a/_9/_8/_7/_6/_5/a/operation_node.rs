//! One step of an operation chain: operation, input, write-once output, task and continuation.
use std::fmt;
use std::sync::{Arc, OnceLock};
use log::debug;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::expander::PlannedOperation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    HasResourceRequirements, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::operation::OperationKind;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::b::task_context::TaskId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::b::task_model::{
    CompleteError, Task,
};
/// Implemented by operations; provides the operation kind.
pub trait HasOperationKind {
    fn operation_kind(&self) -> OperationKind;
}
impl HasResourceRequirements for PlannedOperation {
    fn resource_requirements(&self) -> &ResourceRequirements {
        self.resource()
    }
}
impl HasOperationKind for PlannedOperation {
    fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
/// Creates zero or more follow-up nodes from the output and the task context.
pub type ContinuationHandler<O, I, T, D, E> = Arc<
    dyn Fn(&T, &Task<D, T, E>) -> Vec<OperationNode<O, I, T, D, E>> + Send + Sync,
>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvanceError {
    OutputNotSet,
    Complete(CompleteError),
}
/// Node type of the reference path.
pub type PlannedNode<I, T, D, E> = OperationNode<PlannedOperation, I, T, D, E>;
/// Cloneable read-only view on the output cell of one node. Clones refer to the same cell.
pub struct OutputHandle<T> {
    cell: Arc<OnceLock<T>>,
}
impl<T> Clone for OutputHandle<T> {
    fn clone(&self) -> Self {
        Self {
            cell: Arc::clone(&self.cell),
        }
    }
}
impl<T> fmt::Debug for OutputHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutputHandle").field("set", &self.is_set()).finish()
    }
}
impl<T> OutputHandle<T> {
    pub fn get(&self) -> Option<&T> {
        self.cell.get()
    }
    pub fn is_set(&self) -> bool {
        self.cell.get().is_some()
    }
}
/// O = operation, I = input, T = output (= task result type), D = task data, E = task error.
pub struct OperationNode<O, I, T, D, E> {
    task_id: TaskId,
    operation: O,
    input: I,
    output: Arc<OnceLock<T>>,
    materialize: bool,
    task: Task<D, T, E>,
    continuation: Option<ContinuationHandler<O, I, T, D, E>>,
    ends_chain: bool,
}
impl<O, I, T, D, E> OperationNode<O, I, T, D, E>
where
    O: HasResourceRequirements + HasOperationKind,
{
    /// Logs one debug record with task ID, operation kind, resource kind and materialization flag.
    pub fn new(
        task_id: TaskId,
        operation: O,
        input: I,
        materialize: bool,
        task: Task<D, T, E>,
    ) -> Self {
        debug!(
            "operation node created: task_id={} operation_kind={:?} resource_kind={:?} materialize={}",
            task_id, operation.operation_kind(), operation.resource_requirements()
            .kind(), materialize
        );
        Self {
            task_id,
            operation,
            input,
            output: Arc::new(OnceLock::new()),
            materialize,
            task,
            continuation: None,
            ends_chain: true,
        }
    }
}
impl<O, I, T, D, E> OperationNode<O, I, T, D, E> {
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }
    pub fn operation(&self) -> &O {
        &self.operation
    }
    pub fn input(&self) -> &I {
        &self.input
    }
    pub fn materialize(&self) -> bool {
        self.materialize
    }
    pub fn task(&self) -> &Task<D, T, E> {
        &self.task
    }
    pub fn output(&self) -> Option<&T> {
        self.output.get()
    }
    pub fn output_handle(&self) -> OutputHandle<T> {
        OutputHandle {
            cell: Arc::clone(&self.output),
        }
    }
    /// Whether this node completes the task when it produces no follow-up nodes.
    pub fn set_ends_chain(&mut self, ends_chain: bool) {
        self.ends_chain = ends_chain;
    }
    pub fn with_ends_chain(mut self, ends_chain: bool) -> Self {
        self.ends_chain = ends_chain;
        self
    }
    pub fn ends_chain(&self) -> bool {
        self.ends_chain
    }
    /// Attaches a handler, replacing any previous one.
    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>) {
        self.continuation = Some(handler);
    }
    pub fn with_continuation(
        mut self,
        handler: ContinuationHandler<O, I, T, D, E>,
    ) -> Self {
        self.set_continuation(handler);
        self
    }
    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }
    /// Sets the output once; a later call returns the rejected value unchanged.
    pub fn set_output(&self, value: T) -> Result<(), T> {
        self.output.set(value)
    }
}
impl<O, I, T: Clone, D, E> OperationNode<O, I, T, D, E> {
    /// Returns the follow-up nodes. Without follow-ups the task is completed with the output if
    /// this node ends the chain; otherwise the task is left untouched.
    pub fn advance(&self) -> Result<Vec<Self>, AdvanceError> {
        let output = self.output.get().ok_or(AdvanceError::OutputNotSet)?;
        if let Some(handler) = &self.continuation {
            let next = handler(output, &self.task);
            if !next.is_empty() {
                return Ok(next);
            }
        }
        if self.ends_chain {
            self.task.complete(Ok(output.clone())).map_err(AdvanceError::Complete)?;
        }
        Ok(Vec::new())
    }
}
#[cfg(test)]
#[path = "test/operation_node.rs"]
mod tests;
