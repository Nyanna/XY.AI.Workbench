use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn cfg(total: usize, single: usize) -> PoolConfig {
    PoolConfig {
        max_total_bytes: total,
        max_buffer_size: single,
    }
}

#[test]
fn auto_traits() {
    fn check<T: Send + Sync>() {}
    check::<HostResources>();
    check::<BufferPool>();
    check::<ResourceRegistry>();
    check::<PooledBuffer>();
    check::<SharedBuffer>();
}

#[test]
fn default_config() {
    let c = PoolConfig::default();
    assert_eq!(c.max_total_bytes, 1 << 30);
    assert_eq!(c.max_buffer_size, 256 << 20);
}

#[test]
fn acquire_validates_sizes() {
    let p = BufferPool::new(cfg(100, 50));
    assert_eq!(p.acquire(0).err(), Some(HostResourceError::InvalidSize(0)));
    assert_eq!(p.acquire(51).err(), Some(HostResourceError::InvalidSize(51)));
    let _a = p.acquire(50).unwrap();
    let _b = p.acquire(40).unwrap();
    assert_eq!(
        p.acquire(11).err(),
        Some(HostResourceError::PoolExhausted {
            requested: 11,
            available: 10
        })
    );
    assert_eq!(p.bytes_in_use(), 90);
}

#[test]
fn recycled_memory_is_zeroed() {
    let p = BufferPool::new(cfg(1024, 1024));
    let mut a = p.acquire(64).unwrap();
    a.as_mut_slice().fill(0xAB);
    drop(a);
    let b = p.acquire(32).unwrap();
    assert_eq!(b.len(), 32);
    assert!(b.as_slice().iter().all(|&x| x == 0));
    assert_eq!(p.bytes_in_use(), 32);
}

#[test]
fn freeze_keeps_accounting_until_last_clone() {
    let p = BufferPool::new(cfg(100, 100));
    let mut a = p.acquire(10).unwrap();
    a.as_mut_slice()[0] = 7;
    let s = a.freeze();
    assert_eq!(p.bytes_in_use(), 10);
    let s2 = s.clone();
    drop(s);
    assert_eq!(p.bytes_in_use(), 10);
    assert_eq!(s2.as_slice()[0], 7);
    drop(s2);
    assert_eq!(p.bytes_in_use(), 0);
}

#[test]
fn handle_outlives_pool() {
    let p = BufferPool::new(cfg(100, 100));
    let b = p.acquire(10).unwrap();
    drop(p);
    drop(b);
}

#[test]
fn concurrent_accounting() {
    let p = Arc::new(BufferPool::new(cfg(1000, 100)));
    let hs: Vec<_> = (0..8)
        .map(|_| {
            let p = Arc::clone(&p);
            std::thread::spawn(move || {
                for _ in 0..500 {
                    if let Ok(b) = p.acquire(100) {
                        assert!(p.bytes_in_use() <= 1000);
                        assert!(b.as_slice().iter().all(|&x| x == 0));
                    }
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(p.bytes_in_use(), 0);
}

#[test]
fn registry_basic() {
    let r = ResourceRegistry::new();
    let a = r.insert_new("", 5u32).unwrap();
    assert_eq!(*a, 5);
    assert_eq!(
        r.insert_new("", 6u32).err(),
        Some(HostResourceError::AlreadyExists { key: "".into() })
    );
    assert_eq!(*r.get::<u32>("").unwrap(), 5);
    assert!(r.get::<u64>("").is_none());
    assert!(r.get::<u32>("K").is_none());
    assert_eq!(
        r.get_or_insert_with("", || 1u64).err(),
        Some(HostResourceError::TypeMismatch { key: "".into() })
    );
    assert_eq!(r.len(), 1);
    assert!(r.remove(""));
    assert!(!r.remove(""));
    assert!(r.get::<u32>("").is_none());
    assert_eq!(*a, 5);
    assert_eq!(r.len(), 0);
}

#[test]
fn get_or_insert_runs_once_under_race() {
    let r = Arc::new(ResourceRegistry::new());
    let calls = Arc::new(AtomicUsize::new(0));
    let hs: Vec<_> = (0..8)
        .map(|_| {
            let (r, calls) = (Arc::clone(&r), Arc::clone(&calls));
            std::thread::spawn(move || {
                r.get_or_insert_with("k", || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    42u32
                })
                .unwrap()
            })
        })
        .collect();
    let vals: Vec<_> = hs.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(vals.iter().all(|v| Arc::ptr_eq(v, &vals[0])));
}

#[test]
fn panicking_init_stores_nothing() {
    let r = ResourceRegistry::new();
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = r.get_or_insert_with::<u32, _>("k", || panic!("boom"));
    }));
    assert!(res.is_err());
    assert!(r.get::<u32>("k").is_none());
    assert_eq!(r.len(), 0);
    assert_eq!(*r.get_or_insert_with("k", || 3u32).unwrap(), 3);
    assert_eq!(r.len(), 1);
}

#[test]
fn host_resources_shared() {
    let h = HostResources::shared(cfg(10, 10));
    let b = h.buffers.acquire(4).unwrap();
    assert_eq!(b.len(), 4);
    h.registry.insert_new("a", String::from("x")).unwrap();
    assert_eq!(h.registry.len(), 1);
    let _ = HostResources::default();
}
