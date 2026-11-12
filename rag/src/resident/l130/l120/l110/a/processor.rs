//! Owns and wires the task queue, the three operations queues, the worker pool and the notifications.
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};
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
fn lock<X>(m: &Mutex<X>) -> MutexGuard<'_, X> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
fn deadline_of(timeout: Option<Duration>) -> Option<Instant> {
    timeout.and_then(|d| Instant::now().checked_add(d))
}
#[derive(Default)]
struct PoolState {
    next_id: u64,
    registered: HashSet<u64>,
    parked: HashSet<u64>,
}
struct NotifyCore {
    epoch: AtomicU64,
    state: Mutex<PoolState>,
    cv: Condvar,
}
struct IdleCore {
    epoch: Mutex<u64>,
    cv: Condvar,
}
impl IdleCore {
    fn bump(&self) {
        let mut e = lock(&self.epoch);
        *e = e.wrapping_add(1);
        drop(e);
        self.cv.notify_all();
    }
}
/// Wake-up signal for parked workers.
pub struct Notification {
    core: Arc<NotifyCore>,
}
impl Notification {
    pub fn epoch(&self) -> u64 {
        self.core.epoch.load(Ordering::SeqCst)
    }
    pub fn notify(&self) {
        self.core.epoch.fetch_add(1, Ordering::SeqCst);
        drop(lock(&self.core.state));
        self.core.cv.notify_all();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkerId(pub u64);
/// Parking lot of the workers.
pub struct WorkerPool {
    core: Arc<NotifyCore>,
    idle: Arc<IdleCore>,
}
impl WorkerPool {
    pub fn register(&self) -> WorkerId {
        let mut st = lock(&self.core.state);
        let id = st.next_id;
        st.next_id += 1;
        st.registered.insert(id);
        WorkerId(id)
    }
    pub fn deregister(&self, id: WorkerId) {
        let mut st = lock(&self.core.state);
        if !st.registered.remove(&id.0) {
            return;
        }
        st.parked.remove(&id.0);
        self.check_idle(&st);
    }
    pub fn park(&self, id: WorkerId, seen: u64, timeout: Option<Duration>) -> u64 {
        let deadline = deadline_of(timeout);
        let mut st = lock(&self.core.state);
        if !st.registered.contains(&id.0) || st.parked.contains(&id.0) {
            return self.core.epoch.load(Ordering::SeqCst);
        }
        if self.core.epoch.load(Ordering::SeqCst) != seen {
            return self.core.epoch.load(Ordering::SeqCst);
        }
        st.parked.insert(id.0);
        self.check_idle(&st);
        let result = loop {
            let e = self.core.epoch.load(Ordering::SeqCst);
            if e != seen || !st.parked.contains(&id.0) {
                break e;
            }
            match deadline {
                None => {
                    st = self.core.cv.wait(st).unwrap_or_else(|e| e.into_inner());
                }
                Some(dl) => {
                    let now = Instant::now();
                    if now >= dl {
                        break e;
                    }
                    st = self
                        .core
                        .cv
                        .wait_timeout(st, dl - now)
                        .unwrap_or_else(|e| e.into_inner())
                        .0;
                }
            }
        };
        st.parked.remove(&id.0);
        result
    }
    pub fn registered(&self) -> usize {
        lock(&self.core.state).registered.len()
    }
    pub fn parked(&self) -> usize {
        lock(&self.core.state).parked.len()
    }
    fn check_idle(&self, st: &PoolState) {
        if !st.registered.is_empty() && st.parked.len() == st.registered.len() {
            self.idle.bump();
        }
    }
}
/// Idle events: raised when the system becomes idle.
pub struct IdleNotifications {
    core: Arc<IdleCore>,
}
impl IdleNotifications {
    pub fn epoch(&self) -> u64 {
        *lock(&self.core.epoch)
    }
    pub fn wait_change(&self, seen: u64, timeout: Option<Duration>) -> u64 {
        let deadline = deadline_of(timeout);
        let mut e = lock(&self.core.epoch);
        loop {
            if *e != seen {
                return *e;
            }
            match deadline {
                None => {
                    e = self.core.cv.wait(e).unwrap_or_else(|x| x.into_inner());
                }
                Some(dl) => {
                    let now = Instant::now();
                    if now >= dl {
                        return *e;
                    }
                    e = self
                        .core
                        .cv
                        .wait_timeout(e, dl - now)
                        .unwrap_or_else(|x| x.into_inner())
                        .0;
                }
            }
        }
    }
}
struct Queues<D, T, E, O, V, R, OE> {
    task: TaskQueue<D, T, E>,
    cpu: OpsQueue<O, V, R, OE>,
    gpu: OpsQueue<O, V, R, OE>,
    shared: OpsQueue<O, V, R, OE>,
}
pub struct Processor<D, T, E, O, V, R, OE, PL> {
    planner: PL,
    queues: Queues<D, T, E, O, V, R, OE>,
    pool: WorkerPool,
    notification: Notification,
    idle: IdleNotifications,
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
        let core = Arc::new(NotifyCore {
            epoch: AtomicU64::new(0),
            state: Mutex::new(PoolState::default()),
            cv: Condvar::new(),
        });
        let idle_core = Arc::new(IdleCore {
            epoch: Mutex::new(0),
            cv: Condvar::new(),
        });
        Arc::new(Self {
            planner,
            queues: Queues {
                task: Queue::new(),
                cpu: Queue::new(),
                gpu: Queue::new(),
                shared: Queue::new(),
            },
            pool: WorkerPool {
                core: core.clone(),
                idle: idle_core.clone(),
            },
            notification: Notification { core },
            idle: IdleNotifications {
                core: idle_core,
            },
        })
    }
    pub fn submit(&self, data: D, params: TaskParams) -> Task<D, T, E> {
        let task = Task::new(data, params.clone());
        self.queues.task.push(params, task.clone());
        self.notification.notify();
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
        if !tickets.is_empty() {
            self.notification.notify();
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
    pub fn pool(&self) -> &WorkerPool {
        &self.pool
    }
    pub fn notification(&self) -> &Notification {
        &self.notification
    }
    pub fn idle(&self) -> &IdleNotifications {
        &self.idle
    }
}
#[cfg(test)]
#[path = "test/processor.rs"]
mod tests;
