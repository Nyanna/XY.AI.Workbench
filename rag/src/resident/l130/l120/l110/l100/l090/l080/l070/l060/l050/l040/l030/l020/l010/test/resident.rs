use super::*;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::time::Instant;
struct Echo {
    dim: usize,
    warm_calls: AtomicUsize,
}
impl Echo {
    fn new() -> Arc<Self> {
        Arc::new(Echo {
            dim: 4,
            warm_calls: AtomicUsize::new(0),
        })
    }
}
impl EmbedBackend for Echo {
    fn dim(&self) -> usize {
        self.dim
    }
    fn warm_up(&self) -> Result<(), String> {
        self.warm_calls.fetch_add(1, SeqCst);
        Ok(())
    }
    fn embed(&self, texts: &[String]) -> Result<Vec<Vector>, String> {
        Ok(texts.iter().map(|t| vec![t.len() as f32; self.dim]).collect())
    }
}
struct Gate {
    open: AtomicBool,
    entered: AtomicUsize,
    max_seen: AtomicUsize,
}
impl Gate {
    fn new() -> Arc<Self> {
        Arc::new(Gate {
            open: AtomicBool::new(false),
            entered: AtomicUsize::new(0),
            max_seen: AtomicUsize::new(0),
        })
    }
}
impl EmbedBackend for Gate {
    fn dim(&self) -> usize {
        2
    }
    fn embed(&self, texts: &[String]) -> Result<Vec<Vector>, String> {
        self.entered.fetch_add(1, SeqCst);
        self.max_seen.fetch_max(texts.len(), SeqCst);
        while !self.open.load(SeqCst) {
            thread::sleep(Duration::from_millis(1));
        }
        Ok(texts.iter().map(|_| vec![0.0; 2]).collect())
    }
}
struct Broken {
    mode: u8,
    panicked: AtomicBool,
}
impl EmbedBackend for Broken {
    fn dim(&self) -> usize {
        3
    }
    fn warm_up(&self) -> Result<(), String> {
        if self.mode == 0 { Err("no device".into()) } else { Ok(()) }
    }
    fn embed(&self, texts: &[String]) -> Result<Vec<Vector>, String> {
        match self.mode {
            1 => {
                if !self.panicked.swap(true, SeqCst) {
                    panic!("boom");
                }
                Ok(texts.iter().map(|_| vec![0.0; 3]).collect())
            }
            2 => Ok(Vec::new()),
            _ => Ok(texts.iter().map(|_| vec![0.0; 1]).collect()),
        }
    }
}
struct Fixed {
    calls: AtomicUsize,
    id: u64,
}
impl SimilarityIndex for Fixed {
    fn search(&self, _q: &[f32], k: usize, _d: Target) -> Result<Vec<Hit>, String> {
        self.calls.fetch_add(1, SeqCst);
        Ok(
            (0..k as u64)
                .map(|i| Hit {
                    id: self.id + i,
                    score: 1.0 - i as f32,
                })
                .collect(),
        )
    }
}
fn cfg(cpu: usize, gpu: usize, cap: usize, batch: usize) -> Config {
    Config {
        cpu_workers: cpu,
        gpu_workers: gpu,
        queue_capacity: cap,
        max_batch: batch,
    }
}
fn spin_until(f: impl Fn() -> bool) {
    let end = Instant::now() + Duration::from_secs(5);
    while !f() {
        assert!(Instant::now() < end, "condition not reached");
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn lazy_start_and_single_warm_up() {
    let b = Echo::new();
    let r = ResidentBuilder::new(cfg(2, 0, 16, 4)).cpu_embedder(b.clone()).build();
    let s = r.stats();
    assert!(! s.cpu_started && ! s.gpu_started);
    assert_eq!(b.warm_calls.load(SeqCst), 0);
    let v = r.submit_embed("abc".into(), Target::Cpu).unwrap().wait().unwrap();
    assert_eq!(v, vec![3.0; 4]);
    r.submit_embed("x".into(), Target::Any).unwrap().wait().unwrap();
    assert_eq!(b.warm_calls.load(SeqCst), 1);
    assert!(r.stats().cpu_started && ! r.stats().gpu_started);
    assert_eq!(r.stats().completed, 2);
}
#[test]
fn absent_subengine_is_unavailable() {
    let r = ResidentBuilder::new(cfg(1, 1, 16, 4)).cpu_embedder(Echo::new()).build();
    assert_eq!(
        r.submit_embed("a".into(), Target::Gpu).err(),
        Some(ResidentError::BackendUnavailable)
    );
    assert!(r.submit_embed("a".into(), Target::Any).is_ok());
    let none = ResidentBuilder::new(cfg(1, 1, 16, 4)).build();
    assert_eq!(
        none.submit_embed("a".into(), Target::Any).err(),
        Some(ResidentError::BackendUnavailable)
    );
    assert_eq!(none.stats().rejected, 1);
}
#[test]
fn search_paths() {
    let idx = Arc::new(Fixed {
        calls: AtomicUsize::new(0),
        id: 10,
    });
    let r = ResidentBuilder::new(cfg(1, 0, 16, 4))
        .cpu_embedder(Echo::new())
        .index("main", idx.clone())
        .build();
    let hits = r
        .submit_search("main", vec![0.0], 2, Target::Cpu)
        .unwrap()
        .wait()
        .unwrap();
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].id, 10);
    let none = r
        .submit_search("main", vec![0.0], 0, Target::Cpu)
        .unwrap()
        .wait()
        .unwrap();
    assert!(none.is_empty());
    assert_eq!(idx.calls.load(SeqCst), 1);
    assert_eq!(
        r.submit_search("nope", vec![0.0], 1, Target::Cpu).unwrap().wait(),
        Err(ResidentError::Backend("unknown index".into()))
    );
    r.set_index(
        "nope",
        Arc::new(Fixed {
            calls: AtomicUsize::new(0),
            id: 99,
        }),
    );
    let h = r.submit_search("nope", vec![0.0], 1, Target::Cpu).unwrap().wait().unwrap();
    assert_eq!(h[0].id, 99);
}
#[test]
fn overload_and_batching() {
    let g = Gate::new();
    let r = ResidentBuilder::new(cfg(1, 0, 8, 8)).cpu_embedder(g.clone()).build();
    let first = r.submit_embed("0".into(), Target::Cpu).unwrap();
    spin_until(|| g.entered.load(SeqCst) == 1);
    let mut tickets = Vec::new();
    for i in 0..8 {
        tickets.push(r.submit_embed(i.to_string(), Target::Cpu).unwrap());
    }
    assert_eq!(
        r.submit_embed("x".into(), Target::Cpu).err(), Some(ResidentError::Overloaded)
    );
    assert_eq!(r.stats().rejected, 1);
    assert_eq!(r.stats().queued, 8);
    g.open.store(true, SeqCst);
    first.wait().unwrap();
    for t in tickets {
        t.wait().unwrap();
    }
    assert_eq!(g.max_seen.load(SeqCst), 8);
}
#[test]
fn wait_timeout_keeps_ticket() {
    let g = Gate::new();
    let r = ResidentBuilder::new(cfg(1, 0, 8, 1)).cpu_embedder(g.clone()).build();
    let mut t = r.submit_embed("a".into(), Target::Cpu).unwrap();
    assert_eq!(t.wait_timeout(Duration::from_millis(20)), Err(ResidentError::Timeout));
    assert!(t.try_take().is_none());
    g.open.store(true, SeqCst);
    assert!(t.wait_timeout(Duration::from_secs(5)).is_ok());
}
#[test]
fn shutdown_cancels_queued_and_finishes_running() {
    let g = Gate::new();
    let r = ResidentBuilder::new(cfg(1, 0, 8, 1)).cpu_embedder(g.clone()).build();
    let running = r.submit_embed("a".into(), Target::Cpu).unwrap();
    spin_until(|| g.entered.load(SeqCst) == 1);
    let queued = r.submit_embed("b".into(), Target::Cpu).unwrap();
    let g2 = g.clone();
    let opener = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        g2.open.store(true, SeqCst);
    });
    r.shutdown();
    r.shutdown();
    opener.join().unwrap();
    assert!(running.wait().is_ok());
    assert_eq!(queued.wait(), Err(ResidentError::Cancelled));
    assert_eq!(
        r.submit_embed("c".into(), Target::Cpu).err(), Some(ResidentError::ShutDown)
    );
}
#[test]
fn warm_up_failure_marks_engine_failed() {
    let b = Arc::new(Broken {
        mode: 0,
        panicked: AtomicBool::new(false),
    });
    let r = ResidentBuilder::new(cfg(2, 0, 8, 4)).cpu_embedder(b).build();
    let t = r.submit_embed("a".into(), Target::Cpu).unwrap();
    assert_eq!(t.wait(), Err(ResidentError::BackendUnavailable));
    assert_eq!(
        r.submit_embed("a".into(), Target::Any).err(),
        Some(ResidentError::BackendUnavailable)
    );
}
#[test]
fn backend_panic_and_bad_output() {
    let p = Arc::new(Broken {
        mode: 1,
        panicked: AtomicBool::new(false),
    });
    let r = ResidentBuilder::new(cfg(1, 0, 8, 1)).cpu_embedder(p).build();
    assert_eq!(
        r.submit_embed("a".into(), Target::Cpu).unwrap().wait(),
        Err(ResidentError::Backend("panic".into()))
    );
    assert!(r.submit_embed("a".into(), Target::Cpu).unwrap().wait().is_ok());
    for mode in [2u8, 3] {
        let b = Arc::new(Broken {
            mode,
            panicked: AtomicBool::new(false),
        });
        let r = ResidentBuilder::new(cfg(1, 0, 8, 1)).cpu_embedder(b).build();
        assert!(
            matches!(r.submit_embed("a".into(), Target::Cpu).unwrap().wait(),
            Err(ResidentError::Backend(_)))
        );
    }
}
#[test]
fn concurrent_submitters() {
    let r = Arc::new(
        ResidentBuilder::new(cfg(3, 0, 4096, 16)).cpu_embedder(Echo::new()).build(),
    );
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let r = r.clone();
            thread::spawn(move || {
                let ts: Vec<_> = (0..200)
                    .map(|i| r.submit_embed("z".repeat(i % 7), Target::Any).unwrap())
                    .collect();
                for t in ts {
                    t.wait().unwrap();
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(r.stats().completed, 800);
}
#[test]
fn json_handler() {
    let r = ResidentBuilder::new(cfg(1, 0, 8, 4))
        .cpu_embedder(Echo::new())
        .index(
            "m",
            Arc::new(Fixed {
                calls: AtomicUsize::new(0),
                id: 1,
            }),
        )
        .build();
    let out = handle_request(&r, r#"{"op":"embed","text":"ab","target":"cpu"}"#);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"].as_array().unwrap().len(), 4);
    let out = handle_request(
        &r,
        r#"{"op":"search","index":"m","query":[0.5],"k":1,"target":"cpu"}"#,
    );
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"] [0] ["id"], 1);
    assert_eq!(
        handle_request(& r, r#"{"op":"embed","text":"a","target":"gpu"}"#),
        r#"{"err":"BackendUnavailable"}"#
    );
    assert_eq!(handle_request(& r, "nonsense"), r#"{"err":"BadRequest"}"#);
}
