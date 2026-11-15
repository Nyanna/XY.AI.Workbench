//! One step of an operation chain: operation, input, write-once output, task and continuation.

use std::sync::{Arc, OnceLock};

use log::debug;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::HasResourceRequirements;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::Task;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::CompleteError;

/// Implemented by operations; provides the operation kind used for logging.
pub trait OperationKind {
    fn operation_kind(&self) -> &'static str;
}

/// Creates zero or more follow-up nodes from the output and the task context.
pub type ContinuationHandler<O, I, T, D, E> =
    Arc<dyn Fn(&T, &Task<D, T, E>) -> Vec<OperationNode<O, I, T, D, E>> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvanceError {
    OutputNotSet,
    Complete(CompleteError),
}

/// O = operation, I = input, T = output (= task result type), D = task data, E = task error.
pub struct OperationNode<O, I, T, D, E> {
    operation: O,
    input: I,
    output: OnceLock<T>,
    materialize: bool,
    task: Task<D, T, E>,
    continuation: Option<ContinuationHandler<O, I, T, D, E>>,
}

impl<O, I, T, D, E> OperationNode<O, I, T, D, E>
where
    O: HasResourceRequirements + OperationKind,
{
    pub fn new(
        task_id: u64,
        operation: O,
        input: I,
        materialize: bool,
        task: Task<D, T, E>,
    ) -> Self {
        debug!(
            "operation node created: task_id={} operation_kind={} resource_kind={:?} materialize={}",
            task_id,
            operation.operation_kind(),
            operation.resource_requirements().kind(),
            materialize
        );
        Self {
            operation,
            input,
            output: OnceLock::new(),
            materialize,
            task,
            continuation: None,
        }
    }
}

impl<O, I, T, D, E> OperationNode<O, I, T, D, E> {
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

    /// Attaches a handler, replacing any previous one.
    pub fn set_continuation(&mut self, handler: ContinuationHandler<O, I, T, D, E>) {
        self.continuation = Some(handler);
    }

    pub fn with_continuation(mut self, handler: ContinuationHandler<O, I, T, D, E>) -> Self {
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
    /// Returns the follow-up nodes, or completes the task with the output when the chain ends.
    pub fn advance(&self) -> Result<Vec<Self>, AdvanceError> {
        let output = self.output.get().ok_or(AdvanceError::OutputNotSet)?;
        if let Some(handler) = &self.continuation {
            let next = handler(output, &self.task);
            if !next.is_empty() {
                return Ok(next);
            }
        }
        self.task
            .complete(Ok(output.clone()))
            .map_err(AdvanceError::Complete)?;
        Ok(Vec::new())
    }
}

#[cfg(test)]
#[path = "test/operation_node.rs"]
mod tests;
