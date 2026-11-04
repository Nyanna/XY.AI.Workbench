use super::*;
use std::sync::Arc;
const S0: StreamId = StreamId(0);
const S1: StreamId = StreamId(1);
fn cfg() -> Config {
    Config {
        persistent_bytes: 4096,
        step_bytes: 4096,
        dynamic_bytes: 8192,
        max_streams: 4,
        max_dynamic_blocks: 64,
        default_align: 256,
    }
}
fn mgr() -> MemoryManager {
    MemoryManager::new(cfg()).unwrap()
}
#[test]
fn invalid_config() {
    let mut c = cfg();
    c.max_streams = 0;
    assert_eq!(MemoryManager::new(c).err(), Some(MemError::InvalidConfig));
    let mut c = cfg();
    c.default_align = 3;
    assert_eq!(MemoryManager::new(c).err(), Some(MemError::InvalidConfig));
    let mut c = cfg();
    c.dynamic_bytes = 0;
    assert_eq!(MemoryManager::new(c).err(), Some(MemError::InvalidConfig));
}
#[test]
fn argument_errors() {
    let m = mgr();
    assert_eq!(m.alloc(Lifetime::Step, 0, None, S0), Err(MemError::ZeroSize));
    assert_eq!(m.alloc(Lifetime::Step, 1, Some(3), S0), Err(MemError::BadAlign));
    assert_eq!(
        m.alloc(Lifetime::Step, 1, None, StreamId(9)), Err(MemError::UnknownStream)
    );
    assert_eq!(
        m.alloc(Lifetime::Persistent, 5000, None, S0), Err(MemError::OutOfMemory {
        lifetime : Lifetime::Persistent, requested : 5000 })
    );
}
#[test]
fn persistent_disjoint_and_aligned() {
    let m = mgr();
    let a = m.alloc(Lifetime::Persistent, 10, None, S0).unwrap();
    let b = m.alloc(Lifetime::Persistent, 10, Some(16), S0).unwrap();
    assert_eq!(a.offset % 256, 0);
    assert!(b.offset >= a.offset + a.len);
    assert_eq!(m.free(a, S0, Fence(1)), Err(MemError::WrongLifetime));
}
#[test]
fn same_stream_reuse() {
    let m = mgr();
    let a = m.alloc(Lifetime::Dynamic, 100, None, S0).unwrap();
    m.free(a, S0, Fence(1)).unwrap();
    let b = m.alloc(Lifetime::Dynamic, 100, None, S0).unwrap();
    assert_eq!(a.offset, b.offset);
    assert_eq!(m.free(a, S0, Fence(2)), Ok(()));
    assert_eq!(m.free(a, S0, Fence(2)), Err(MemError::InvalidRegion));
}
#[test]
fn cross_stream_needs_complete() {
    let m = mgr();
    let a = m.alloc(Lifetime::Dynamic, 8192, None, S0).unwrap();
    m.free(a, S0, Fence(5)).unwrap();
    assert_eq!(m.alloc(Lifetime::Dynamic, 100, None, S1), Err(MemError::NotReady));
    m.complete(S0, Fence(4)).unwrap();
    assert_eq!(m.alloc(Lifetime::Dynamic, 100, None, S1), Err(MemError::NotReady));
    m.complete(S0, Fence(5)).unwrap();
    assert!(m.alloc(Lifetime::Dynamic, 100, None, S1).is_ok());
    assert!(
        matches!(m.alloc(Lifetime::Dynamic, 9000, None, S1), Err(MemError::OutOfMemory {
        .. }))
    );
}
#[test]
fn merge_rules() {
    let m = mgr();
    let a = m.alloc(Lifetime::Dynamic, 4096, None, S0).unwrap();
    let b = m.alloc(Lifetime::Dynamic, 4096, None, S1).unwrap();
    m.free(a, S0, Fence(1)).unwrap();
    m.free(b, S1, Fence(1)).unwrap();
    assert_eq!(m.alloc(Lifetime::Dynamic, 8192, None, S0), Err(MemError::NotReady));
    m.complete(S0, Fence(1)).unwrap();
    m.complete(S1, Fence(1)).unwrap();
    assert!(m.alloc(Lifetime::Dynamic, 8192, None, StreamId(2)).is_ok());
}
#[test]
fn block_table_full() {
    let mut c = cfg();
    c.max_dynamic_blocks = 2;
    let m = MemoryManager::new(c).unwrap();
    m.alloc(Lifetime::Dynamic, 256, None, S0).unwrap();
    assert_eq!(m.alloc(Lifetime::Dynamic, 256, None, S0), Err(MemError::BlockTableFull));
    assert_eq!(m.used(Lifetime::Dynamic), 256);
}
#[test]
fn step_reset_barrier() {
    let m = mgr();
    m.alloc(Lifetime::Step, 100, None, S0).unwrap();
    assert_eq!(m.used(Lifetime::Step), 100);
    m.reset_step(S0, Fence(3)).unwrap();
    assert_eq!(m.used(Lifetime::Step), 0);
    assert_eq!(m.alloc(Lifetime::Step, 100, None, S1), Err(MemError::NotReady));
    let r = m.alloc(Lifetime::Step, 100, None, S0).unwrap();
    assert_eq!(r.offset, 0);
    assert_eq!(m.free(r, S0, Fence(4)), Err(MemError::WrongLifetime));
    m.complete(S0, Fence(3)).unwrap();
    assert!(m.alloc(Lifetime::Step, 100, None, S1).is_ok());
}
#[test]
fn no_heap_growth() {
    let m = mgr();
    let cap = m.lock().blocks.capacity();
    let mut regs = Vec::new();
    for _ in 0..20 {
        if let Ok(r) = m.alloc(Lifetime::Dynamic, 300, Some(64), S0) {
            regs.push(r);
        }
    }
    for (i, r) in regs.into_iter().enumerate() {
        m.free(r, S0, Fence(i as u64)).unwrap();
    }
    assert_eq!(m.lock().blocks.capacity(), cap);
}
#[test]
fn concurrent_disjoint() {
    let m = Arc::new(
        MemoryManager::new(Config {
                dynamic_bytes: 1 << 20,
                max_dynamic_blocks: 1024,
                ..cfg()
            })
            .unwrap(),
    );
    let handles: Vec<_> = (0..8)
        .map(|t| {
            let m = m.clone();
            std::thread::spawn(move || {
                (0..50)
                    .map(|_| {
                        m.alloc(Lifetime::Dynamic, 500, None, StreamId(t % 4)).unwrap()
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut all: Vec<Region> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all.sort_by_key(|r| r.offset);
    for w in all.windows(2) {
        assert!(w[0].offset + w[0].len <= w[1].offset);
    }
}
