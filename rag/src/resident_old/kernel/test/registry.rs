use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counting(AtomicUsize);

impl KernelLoader<u32> for Counting {
    fn load(&self, key: &KernelKey) -> Result<u32, KernelError> {
        if key.name == "missing" {
            return Err(KernelError::FuncNotFound { name: key.name.into() });
        }
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(key.variant)
    }
}

fn registry() -> (Arc<Counting>, KernelRegistry<u32>) {
    let loader = Arc::new(Counting(AtomicUsize::new(0)));
    (loader.clone(), KernelRegistry::new(loader))
}

#[test]
fn loads_once_per_key() {
    let (loader, reg) = registry();
    let k = KernelKey::new("cosine", DType::F16, 75, 384);
    assert_eq!(reg.get(&k).unwrap(), 384);
    assert_eq!(reg.get(&k).unwrap(), 384);
    assert_eq!(loader.0.load(Ordering::SeqCst), 1);
    reg.get(&KernelKey::new("cosine", DType::F32, 75, 384)).unwrap();
    assert_eq!(reg.len(), 2);
}

#[test]
fn failed_load_is_not_cached() {
    let (_, reg) = registry();
    let k = KernelKey::new("missing", DType::F32, 75, 0);
    assert!(matches!(reg.get(&k), Err(KernelError::FuncNotFound { .. })));
    assert!(reg.is_empty());
}

#[test]
fn evict_forces_reload() {
    let (loader, reg) = registry();
    let k = KernelKey::new("cosine", DType::F16, 75, 8);
    reg.get(&k).unwrap();
    assert_eq!(reg.evict_all(), 1);
    reg.get(&k).unwrap();
    assert_eq!(loader.0.load(Ordering::SeqCst), 2);
}

#[test]
fn concurrent_gets_load_once() {
    let (loader, reg) = registry();
    let reg = Arc::new(reg);
    let k = KernelKey::new("cosine", DType::F16, 75, 1);
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let reg = reg.clone();
            std::thread::spawn(move || reg.get(&k).unwrap())
        })
        .collect();
    handles.into_iter().for_each(|h| assert_eq!(h.join().unwrap(), 1));
    assert_eq!(loader.0.load(Ordering::SeqCst), 1);
}
