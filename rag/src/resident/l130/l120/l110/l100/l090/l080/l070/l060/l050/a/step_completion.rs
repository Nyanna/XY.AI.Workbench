//! Completion of one executed step: output hand-over, release of the next steps, counters and errors.

use std::sync::Arc;

use log::debug;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::operation_node::{
    AdvanceError, OperationNode,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::b::task_context::TaskContext;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::Queue;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::QueueState;

/// A planned successor: its queue placement (opaque here) and its node.
pub struct PlannedStep<M, O, I, T, D, E> {
    pub placement: M,
    pub node: OperationNode<O, I, T, D, E>,
}

/// Pushes the payload of a follow-up step. `placement` is `None` for nodes that a continuation
/// handler created at run time; the implementation determines their placement itself.
pub trait FollowUpSink<M, P>: Send + Sync {
    fn submit(&self, placement: Option<M>, payload: P);
}

/// One queue entry: a node, the context of its task and the planned rest of its chain.
pub struct StepPayload<M, O, I, T, D, E> {
    node: OperationNode<O, I, T, D, E>,
    context: Arc<TaskContext<E>>,
    rest: Vec<PlannedStep<M, O, I, T, D, E>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepError {
    OutputAlreadySet,
    Advance(AdvanceError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepReport {
    /// Number of payloads submitted.
    pub follow_ups: usize,
    /// Result of the last `complete_operation()` of the call.
    pub task_finished: bool,
    /// The task was already cancelled: the output was not stored and `advance` was not called.
    pub discarded: bool,
}

impl<M, O, I, T, D, E> StepPayload<M, O, I, T, D, E> {
    /// A non-empty `rest` sets `ends_chain` of the node to false; otherwise the flag is kept.
    pub fn new(
        mut node: OperationNode<O, I, T, D, E>,
        context: Arc<TaskContext<E>>,
        rest: Vec<PlannedStep<M, O, I, T, D, E>>,
    ) -> Self {
        if !rest.is_empty() {
            node.set_ends_chain(false);
        }
        Self {
            node,
            context,
            rest,
        }
    }

    pub fn node(&self) -> &OperationNode<O, I, T, D, E> {
        &self.node
    }

    pub fn context(&self) -> &Arc<TaskContext<E>> {
        &self.context
    }

    /// `rest()[0]` is the next step.
    pub fn rest(&self) -> &[PlannedStep<M, O, I, T, D, E>] {
        &self.rest
    }

    /// Replaces the planned rest and returns the old one; `ends_chain` becomes `rest.is_empty()`.
    /// Neither rest is registered or submitted.
    pub fn replace_rest(
        &mut self,
        rest: Vec<PlannedStep<M, O, I, T, D, E>>,
    ) -> Vec<PlannedStep<M, O, I, T, D, E>> {
        self.node.set_ends_chain(rest.is_empty());
        std::mem::replace(&mut self.rest, rest)
    }
}

impl<M, O, I, T: Clone, D, E> StepPayload<M, O, I, T, D, E> {
    /// Stores the result of a successfully executed step and continues the chain.
    pub fn finish<S>(&mut self, output: T, sink: &S) -> Result<StepReport, StepError>
    where
        S: FollowUpSink<M, StepPayload<M, O, I, T, D, E>> + ?Sized,
    {
        if self.context.is_cancelled() {
            self.rest.clear();
            let task_finished = self.context.complete_operation();
            debug!(
                "task {}: step result discarded, task cancelled",
                self.context.id()
            );
            return Ok(StepReport {
                follow_ups: 0,
                task_finished,
                discarded: true,
            });
        }
        if self.node.set_output(output).is_err() {
            return Err(StepError::OutputAlreadySet);
        }
        let extra = self.node.advance().map_err(StepError::Advance)?;

        let mut rest = std::mem::take(&mut self.rest).into_iter();
        let next = rest.next();
        let has_next = next.is_some();
        let follow_ups = extra.len() + usize::from(has_next);
        self.context.register_follow_ups(follow_ups);

        if let Some(next) = next {
            let remaining: Vec<_> = rest.collect();
            sink.submit(
                Some(next.placement),
                StepPayload::new(next.node, Arc::clone(&self.context), remaining),
            );
        }
        let extra_count = extra.len();
        for (index, mut node) in extra.into_iter().enumerate() {
            node.set_ends_chain(!has_next && index + 1 == extra_count);
            sink.submit(
                None,
                StepPayload::new(node, Arc::clone(&self.context), Vec::new()),
            );
        }

        let task_finished = self.context.complete_operation();
        Ok(StepReport {
            follow_ups,
            task_finished,
            discarded: false,
        })
    }
}

impl<M, O, I, T, D, E: Clone> StepPayload<M, O, I, T, D, E> {
    /// Handles a failed step. `withdraw` removes entries of the task that were already pushed and
    /// returns how many it removed (see `withdraw_pending`).
    pub fn fail<W: FnOnce() -> usize>(&mut self, error: E, withdraw: W) -> StepReport {
        if self.context.fail(error.clone()) && self.node.task().state() == QueueState::Claimed {
            if let Err(e) = self.node.task().complete(Err(error)) {
                debug!(
                    "task {}: failing the task was not possible: {:?}",
                    self.context.id(),
                    e
                );
            }
        }
        self.rest.clear();
        for _ in 0..withdraw() {
            self.context.complete_operation();
        }
        let task_finished = self.context.complete_operation();
        StepReport {
            follow_ups: 0,
            task_finished,
            discarded: false,
        }
    }
}

/// Claims and drops (-> Abandoned) every Pending item accepted by `matches`. Returns the count.
pub fn withdraw_pending<M, P, R, E>(
    queue: &Queue<M, P, R, E>,
    mut matches: impl FnMut(&M) -> bool,
) -> usize
where
    M: Send + Sync + 'static,
    P: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    let mut removed = 0;
    while let Some(claim) = queue.take_where(&mut matches) {
        drop(claim);
        removed += 1;
    }
    removed
}

/// `withdraw_pending`, plus one `complete_operation()` on `context` per removed item.
pub fn withdraw_planned<M, P, R, E, X>(
    queue: &Queue<M, P, R, E>,
    matches: impl FnMut(&M) -> bool,
    context: &TaskContext<X>,
) -> usize
where
    M: Send + Sync + 'static,
    P: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    let removed = withdraw_pending(queue, matches);
    for _ in 0..removed {
        context.complete_operation();
    }
    removed
}

#[cfg(test)]
#[path = "test/step_completion.rs"]
mod tests;
