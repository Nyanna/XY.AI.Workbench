use super::*;
use std::sync::Mutex;

struct Mock {
    len: u64,
    calls: Mutex<Vec<(Advice, ByteRange)>>,
    fail: bool,
}

impl Mock {
    fn new(len: u64, fail: bool) -> Self {
        Self {
            len,
            calls: Mutex::new(Vec::new()),
            fail,
        }
    }
}

impl AdviceTarget for Mock {
    fn len(&self) -> u64 {
        self.len
    }
    fn advise(&self, advice: Advice, range: ByteRange) -> Result<(), AdviceError> {
        self.calls.lock().unwrap().push((advice, range));
        if self.fail {
            Err(AdviceError::Os(io::Error::from_raw_os_error(22)))
        } else {
            Ok(())
        }
    }
}

fn r(start: u64, end: u64) -> ByteRange {
    ByteRange { start, end }
}

#[test]
fn check_range_cases() {
    assert!(check_range(r(0, 10), 10).is_ok());
    assert!(check_range(r(5, 5), 10).is_ok());
    assert!(matches!(
        check_range(r(6, 5), 10),
        Err(AdviceError::StartAfterEnd { start: 6, end: 5 })
    ));
    assert!(matches!(
        check_range(r(0, 11), 10),
        Err(AdviceError::EndBeyondLen { end: 11, len: 10 })
    ));
    assert!(matches!(
        check_range(r(20, 15), 10),
        Err(AdviceError::StartAfterEnd { .. })
    ));
}

#[test]
fn advise_forwards_once_and_skips_empty() {
    let m = Mock::new(100, false);
    advise(&m, Advice::Random, r(10, 20)).unwrap();
    advise(&m, Advice::Random, r(30, 30)).unwrap();
    assert_eq!(*m.calls.lock().unwrap(), vec![(Advice::Random, r(10, 20))]);
    assert!(advise(&m, Advice::Cold, r(0, 101)).is_err());
    assert_eq!(m.calls.lock().unwrap().len(), 1);
}

#[test]
fn advise_passes_error_through() {
    let m = Mock::new(100, true);
    assert!(matches!(
        advise(&m, Advice::Cold, r(0, 10)),
        Err(AdviceError::Os(_))
    ));
}

#[test]
fn prefetch_is_will_need() {
    let m = Mock::new(100, false);
    prefetch(&m, r(1, 2)).unwrap();
    assert_eq!(*m.calls.lock().unwrap(), vec![(Advice::WillNeed, r(1, 2))]);
}

fn anon_map(len: usize) -> Arc<memmap2::Mmap> {
    let mut m = memmap2::MmapMut::map_anon(len).unwrap();
    m[5] = 7;
    Arc::new(m.make_read_only().unwrap())
}

#[test]
fn mmap_advisor_all_advice() {
    let map = anon_map(1 << 16);
    let a = MmapAdvisor::new(map.clone());
    assert_eq!(a.len(), 1 << 16);
    for adv in [
        Advice::WillNeed,
        Advice::Sequential,
        Advice::Random,
        Advice::Cold,
    ] {
        advise(&a, adv, r(5, 9000)).unwrap();
        advise(&a, adv, r(1 << 15, 1 << 16)).unwrap();
    }
    assert_eq!(map[5], 7);
    assert!(matches!(
        advise(&a, Advice::Cold, r(0, (1 << 16) + 1)),
        Err(AdviceError::EndBeyondLen { .. })
    ));
}

#[tokio::test]
async fn read_blocking_runs_off_thread_and_returns_value() {
    let caller = std::thread::current().id();
    let (id, v) = read_blocking(|| (std::thread::current().id(), 42)).await.unwrap();
    assert_ne!(id, caller);
    assert_eq!(v, 42);
}

#[tokio::test]
async fn read_blocking_panic_is_error() {
    let res = read_blocking(|| -> u32 { panic!("boom") }).await;
    assert!(matches!(res, Err(BlockingError::Panicked)));
}

#[tokio::test]
async fn read_with_advice_default_and_order() {
    let m = Arc::new(Mock::new(100, false));
    let src: Arc<dyn AdviceTarget> = m.clone();
    let m2 = m.clone();
    let seen = read_with_advice(Some(src), r(0, 10), None, move || {
        m2.calls.lock().unwrap().len()
    })
    .await
    .unwrap();
    assert_eq!(seen, 1);
    assert_eq!(m.calls.lock().unwrap()[0].0, Advice::WillNeed);
}

#[tokio::test]
async fn read_with_advice_runs_despite_failures_and_none() {
    let m = Arc::new(Mock::new(100, true));
    let src: Arc<dyn AdviceTarget> = m.clone();
    assert_eq!(
        read_with_advice(Some(src.clone()), r(0, 10), Some(Advice::Cold), || 1)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        read_with_advice(Some(src.clone()), r(0, 500), None, || 2)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        read_with_advice(Some(src), r(4, 4), None, || 3).await.unwrap(),
        3
    );
    assert_eq!(m.calls.lock().unwrap().len(), 1);
    assert_eq!(read_with_advice(None, r(0, 1), None, || 4).await.unwrap(), 4);
}
