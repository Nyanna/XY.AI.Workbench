use std::iter::FusedIterator;
use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue_state::{
    EntryState, QueueState,
};

pub trait QueueNode: Send + Sync + 'static {
    type Payload;
    type Output;
    type Error;
    type Meta;

    fn is_sentinel(&self) -> bool;
    fn next(&self) -> Option<Arc<Self>>;
    fn entry(&self) -> &Arc<EntryState<Self::Payload, Self::Output, Self::Error>>;
    fn meta(&self) -> &Self::Meta;
}

pub struct QueueItem<N: QueueNode>(Arc<N>);

impl<N: QueueNode> QueueItem<N> {
    pub fn entry(&self) -> &Arc<EntryState<N::Payload, N::Output, N::Error>> {
        self.0.entry()
    }

    pub fn meta(&self) -> &N::Meta {
        self.0.meta()
    }

    pub fn state(&self) -> QueueState {
        self.0.entry().state()
    }

    pub fn node(&self) -> &Arc<N> {
        &self.0
    }
}

impl<N: QueueNode> Clone for QueueItem<N> {
    fn clone(&self) -> Self {
        QueueItem(Arc::clone(&self.0))
    }
}

pub struct AllIter<N: QueueNode> {
    cursor: Option<Arc<N>>,
}

pub struct ActiveIter<N: QueueNode> {
    cursor: Option<Arc<N>>,
}

impl<N: QueueNode> Iterator for AllIter<N> {
    type Item = QueueItem<N>;

    fn next(&mut self) -> Option<QueueItem<N>> {
        advance(&mut self.cursor, false)
    }
}

impl<N: QueueNode> Iterator for ActiveIter<N> {
    type Item = QueueItem<N>;

    fn next(&mut self) -> Option<QueueItem<N>> {
        advance(&mut self.cursor, true)
    }
}

impl<N: QueueNode> FusedIterator for AllIter<N> {}
impl<N: QueueNode> FusedIterator for ActiveIter<N> {}

// `cursor` is the last visited node; None means the iteration has ended.
fn advance<N: QueueNode>(cursor: &mut Option<Arc<N>>, active_only: bool) -> Option<QueueItem<N>> {
    let mut current = cursor.take()?;
    loop {
        let next = current.next()?;
        if next.is_sentinel() {
            current = next;
            continue;
        }
        if active_only
            && matches!(
                next.entry().state(),
                QueueState::Completed | QueueState::Cancelled
            )
        {
            current = next;
            continue;
        }
        *cursor = Some(Arc::clone(&next));
        return Some(QueueItem(next));
    }
}

pub fn iter_all<N: QueueNode>(head: &Arc<N>) -> AllIter<N> {
    AllIter {
        cursor: Some(Arc::clone(head)),
    }
}

pub fn iter_active<N: QueueNode>(head: &Arc<N>) -> ActiveIter<N> {
    ActiveIter {
        cursor: Some(Arc::clone(head)),
    }
}

#[cfg(test)]
#[path = "test/queue_iter.rs"]
mod tests;
