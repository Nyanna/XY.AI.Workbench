use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostResourceError {
    /// Pool limit would be exceeded.
    PoolExhausted { requested: usize, available: usize },
    /// Requested size is 0 or larger than the pool's max_buffer_size.
    InvalidSize(usize),
    /// Registry key exists with a different value type.
    TypeMismatch { key: String },
    /// Registry key already present.
    AlreadyExists { key: String },
}

impl fmt::Display for HostResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PoolExhausted {
                requested,
                available,
            } => write!(
                f,
                "buffer pool exhausted: requested {requested} bytes, {available} available"
            ),
            Self::InvalidSize(n) => write!(f, "invalid buffer size: {n}"),
            Self::TypeMismatch { key } => {
                write!(f, "registry key '{key}' holds a value of a different type")
            }
            Self::AlreadyExists { key } => write!(f, "registry key '{key}' already exists"),
        }
    }
}

impl std::error::Error for HostResourceError {}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolConfig {
    /// Max total bytes held by live (checked-out) buffers. Idle pooled buffers do not count.
    pub max_total_bytes: usize,
    /// Max size of a single buffer.
    pub max_buffer_size: usize,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_total_bytes: 1 << 30,
            max_buffer_size: 256 << 20,
        }
    }
}

#[derive(Default)]
struct PoolState {
    in_use: usize,
    idle: Vec<Vec<u8>>,
    idle_bytes: usize,
}

struct PoolShared {
    config: PoolConfig,
    state: Mutex<PoolState>,
}

/// Exclusive handle to a zero-initialised byte buffer. Returned to the pool on drop.
pub struct PooledBuffer {
    data: Vec<u8>,
    len: usize,
    pool: Arc<PoolShared>,
}

impl PooledBuffer {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Convert into an immutable, cheaply clonable shared view.
    pub fn freeze(self) -> SharedBuffer {
        SharedBuffer {
            inner: Arc::new(self),
        }
    }
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        let data = std::mem::take(&mut self.data);
        let mut st = lock(&self.pool.state);
        st.in_use = st.in_use.saturating_sub(self.len);
        let cap = data.capacity();
        if cap > 0 && st.idle_bytes.saturating_add(cap) <= self.pool.config.max_total_bytes {
            st.idle_bytes += cap;
            st.idle.push(data);
        }
    }
}

/// Immutable shared view; clone is O(1).
#[derive(Clone)]
pub struct SharedBuffer {
    inner: Arc<PooledBuffer>,
}

impl SharedBuffer {
    pub fn len(&self) -> usize {
        self.inner.len
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

pub struct BufferPool {
    shared: Arc<PoolShared>,
}

impl BufferPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            shared: Arc::new(PoolShared {
                config,
                state: Mutex::new(PoolState::default()),
            }),
        }
    }

    pub fn acquire(&self, len: usize) -> Result<PooledBuffer, HostResourceError> {
        let cfg = self.shared.config;
        if len == 0 || len > cfg.max_buffer_size {
            return Err(HostResourceError::InvalidSize(len));
        }
        let mut recycled = {
            let mut st = lock(&self.shared.state);
            let available = cfg.max_total_bytes.saturating_sub(st.in_use);
            if len > available {
                return Err(HostResourceError::PoolExhausted {
                    requested: len,
                    available,
                });
            }
            st.in_use += len;
            let best = st
                .idle
                .iter()
                .enumerate()
                .filter(|(_, v)| v.capacity() >= len)
                .min_by_key(|(_, v)| v.capacity())
                .map(|(i, _)| i);
            best.map(|i| {
                let v = st.idle.swap_remove(i);
                st.idle_bytes -= v.capacity();
                v
            })
        }
        .unwrap_or_default();
        recycled.clear();
        recycled.resize(len, 0);
        Ok(PooledBuffer {
            data: recycled,
            len,
            pool: Arc::clone(&self.shared),
        })
    }

    pub fn config(&self) -> PoolConfig {
        self.shared.config
    }

    /// Bytes currently held by live PooledBuffer/SharedBuffer handles.
    pub fn bytes_in_use(&self) -> usize {
        lock(&self.shared.state).in_use
    }
}

type AnyArc = Arc<dyn Any + Send + Sync>;

struct Cell {
    value: OnceLock<AnyArc>,
}

pub struct ResourceRegistry {
    map: Mutex<HashMap<String, Arc<Cell>>>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    fn cell(&self, key: &str) -> Arc<Cell> {
        let mut map = lock(&self.map);
        if let Some(c) = map.get(key) {
            return Arc::clone(c);
        }
        let c = Arc::new(Cell {
            value: OnceLock::new(),
        });
        map.insert(key.to_owned(), Arc::clone(&c));
        c
    }

    fn downcast<T: Any + Send + Sync>(key: &str, v: &AnyArc) -> Result<Arc<T>, HostResourceError> {
        Arc::clone(v)
            .downcast::<T>()
            .map_err(|_| HostResourceError::TypeMismatch {
                key: key.to_owned(),
            })
    }

    /// Fails with AlreadyExists if the key is present (any type).
    pub fn insert_new<T: Any + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> Result<Arc<T>, HostResourceError> {
        let cell = self.cell(key);
        let arc = Arc::new(value);
        let any: AnyArc = arc.clone();
        cell.value
            .set(any)
            .map(|_| arc)
            .map_err(|_| HostResourceError::AlreadyExists {
                key: key.to_owned(),
            })
    }

    /// Returns the existing value, or creates it with `init` exactly once per key.
    /// `init` must not call back into the same registry.
    pub fn get_or_insert_with<T: Any + Send + Sync, F: FnOnce() -> T>(
        &self,
        key: &str,
        init: F,
    ) -> Result<Arc<T>, HostResourceError> {
        let cell = self.cell(key);
        let v = cell.value.get_or_init(|| Arc::new(init()) as AnyArc);
        Self::downcast::<T>(key, v)
    }

    /// None if absent or of a different type.
    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>> {
        let cell = lock(&self.map).get(key).cloned()?;
        let v = cell.value.get()?;
        Self::downcast::<T>(key, v).ok()
    }

    /// Removes the key; existing Arc clones stay valid.
    pub fn remove(&self, key: &str) -> bool {
        let cell = lock(&self.map).remove(key);
        cell.is_some_and(|c| c.value.get().is_some())
    }

    pub fn len(&self) -> usize {
        lock(&self.map)
            .values()
            .filter(|c| c.value.get().is_some())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Top-level container shared by both sub-engines.
pub struct HostResources {
    pub buffers: BufferPool,
    pub registry: ResourceRegistry,
}

impl HostResources {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            buffers: BufferPool::new(config),
            registry: ResourceRegistry::new(),
        }
    }

    pub fn shared(config: PoolConfig) -> Arc<HostResources> {
        Arc::new(Self::new(config))
    }
}

impl Default for HostResources {
    fn default() -> Self {
        Self::new(PoolConfig::default())
    }
}

#[cfg(test)]
#[path = "test/host_resources.rs"]
mod tests;
