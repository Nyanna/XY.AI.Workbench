use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::RegionBackend;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static_assertions::assert_impl_all!(WeightFile: Send, Sync);
static_assertions::assert_impl_all!(WeightInfo: Send, Sync);
static_assertions::assert_impl_all!(LoadedWeights: Send, Sync);
static_assertions::assert_impl_all!(WeightError: Send, Sync);
static_assertions::assert_eq_size!(u8, u8);

#[derive(Debug)]
struct MemBackend(Mutex<Vec<u8>>);

impl RegionBackend for MemBackend {
    fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        dst.copy_from_slice(&self.0.lock().unwrap()[offset..offset + dst.len()]);
        Ok(())
    }
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        self.0.lock().unwrap()[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
}

fn region(len: usize) -> Region {
    Region::new(Arc::new(MemBackend(Mutex::new(vec![0xEE; len]))))
}

fn weights_arena(capacity: u64) -> Arena {
    Arena { kind: ArenaKind::Weights, capacity }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_path() -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("weight_source_{}_{}.safetensors", std::process::id(), n))
}

fn write_file(header: &str, data: &[u8]) -> PathBuf {
    let p = temp_path();
    let mut bytes = (header.len() as u64).to_le_bytes().to_vec();
    bytes.extend_from_slice(header.as_bytes());
    bytes.extend_from_slice(data);
    std::fs::write(&p, bytes).unwrap();
    p
}

fn sample() -> (PathBuf, Vec<u8>) {
    let header = r#"{"__metadata__":{"a":"b"},"a":{"dtype":"U8","shape":[3],"data_offsets":[0,3]},"z":{"dtype":"U8","shape":[0],"data_offsets":[3,3]},"b":{"dtype":"F32","shape":[2],"data_offsets":[3,11]}}"#;
    let data: Vec<u8> = (1..=11).collect();
    (write_file(header, &data), data)
}

fn open_err(header: &str, data: &[u8]) -> WeightError {
    let p = write_file(header, data);
    let r = WeightFile::open(&p);
    std::fs::remove_file(&p).ok();
    r.unwrap_err()
}

#[test]
fn open_and_raw_bytes() {
    let (p, data) = sample();
    let w = WeightFile::open(&p).unwrap();
    assert_eq!(w.tensors().len(), 3);
    assert_eq!(w.raw_bytes("a").unwrap(), &data[0..3]);
    assert_eq!(w.raw_bytes("b").unwrap(), &data[3..11]);
    assert_eq!(w.raw_bytes("a").unwrap().as_ptr(), w.raw_bytes("a").unwrap().as_ptr());
    assert!(matches!(w.raw_bytes("q"), Err(WeightError::NotFound { .. })));
    assert_eq!(w.info("b").unwrap().shape, vec![2]);
    std::fs::remove_file(&p).ok();
}

#[test]
fn empty_header_is_valid() {
    let p = write_file("{}", &[]);
    let w = WeightFile::open(&p).unwrap();
    assert!(w.tensors().is_empty());
    std::fs::remove_file(&p).ok();
}

#[test]
fn invalid_formats() {
    let u8t = |b: u64, e: u64, s: u64| {
        format!(r#"{{"a":{{"dtype":"U8","shape":[{s}],"data_offsets":[{b},{e}]}}}}"#)
    };
    let cases: Vec<(String, Vec<u8>)> = vec![
        ("not json".into(), vec![]),
        (u8t(2, 1, 1), vec![0; 2]),
        (u8t(0, 5, 5), vec![0; 2]),
        (u8t(0, 2, 3), vec![0; 2]),
        (u8t(0, 2, 2), vec![0; 3]),
        (u8t(1, 3, 2), vec![0; 3]),
        (
            r#"{"a":{"dtype":"U8","shape":[2],"data_offsets":[0,2]},"b":{"dtype":"U8","shape":[2],"data_offsets":[1,3]}}"#.into(),
            vec![0; 3],
        ),
        (
            r#"{"a":{"dtype":"U8","shape":[1],"data_offsets":[0,1]},"a":{"dtype":"U8","shape":[1],"data_offsets":[1,2]}}"#.into(),
            vec![0; 2],
        ),
    ];
    for (h, d) in cases {
        assert!(matches!(open_err(&h, &d), WeightError::InvalidFormat { .. }), "{h}");
    }
}

#[test]
fn short_and_huge_header() {
    let p = temp_path();
    std::fs::write(&p, [1u8, 2, 3]).unwrap();
    assert!(matches!(WeightFile::open(&p), Err(WeightError::InvalidFormat { .. })));
    std::fs::write(&p, u64::MAX.to_le_bytes()).unwrap();
    assert!(matches!(WeightFile::open(&p), Err(WeightError::InvalidFormat { .. })));
    std::fs::remove_file(&p).ok();
    assert!(matches!(WeightFile::open(&p), Err(WeightError::Io { .. })));
}

#[test]
fn unknown_dtype() {
    let e = open_err(r#"{"a":{"dtype":"F64","shape":[1],"data_offsets":[0,8]}}"#, &[0; 8]);
    match e {
        WeightError::Dtype(LayoutError::UnsupportedDType { dtype, source }) => {
            assert_eq!(dtype, "F64");
            assert!(source.ends_with(":a"));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn copy_aligned_and_verbatim() {
    let (p, _) = sample();
    let w = WeightFile::open(&p).unwrap();
    let r = region(2048);
    let calls = Mutex::new(Vec::new());
    let sel = |n: &str| {
        calls.lock().unwrap().push(n.to_owned());
        true
    };
    let lw = w.copy_to_arena(&weights_arena(2048), &r, 10, &sel).unwrap();
    assert_eq!(calls.lock().unwrap().len(), 3);
    assert_eq!(lw.tensors[0].0, "a");
    assert_eq!(lw.get("a").unwrap().offset_bytes(), 256);
    assert_eq!(lw.get("z").unwrap().offset_bytes(), 512);
    assert_eq!(lw.get("b").unwrap().offset_bytes(), 512);
    assert_eq!(lw.end_offset, 520);
    for n in ["a", "b"] {
        let raw = w.raw_bytes(n).unwrap();
        let mut dst = vec![0u8; raw.len()];
        r.read(lw.get(n).unwrap().offset_bytes(), &mut dst).unwrap();
        assert_eq!(dst, raw);
    }
    let mut pad = [0u8; 1];
    r.read(300, &mut pad).unwrap();
    assert_eq!(pad[0], 0xEE);
    std::fs::remove_file(&p).ok();
}

#[test]
fn copy_errors_and_empty_selection() {
    let (p, _) = sample();
    let w = WeightFile::open(&p).unwrap();
    let r = region(1024);
    let all = |_: &str| true;
    let bad = Arena { kind: ArenaKind::Staging, capacity: 1024 };
    assert!(matches!(
        w.copy_to_arena(&bad, &r, 0, &all),
        Err(WeightError::ArenaKindMismatch { .. })
    ));
    assert!(matches!(
        w.copy_to_arena(&weights_arena(1024), &r, 600, &all),
        Err(WeightError::CapacityExceeded { .. })
    ));
    assert!(matches!(
        w.copy_to_arena(&weights_arena(1024), &r, u64::MAX - 1, &all),
        Err(WeightError::CapacityExceeded { .. })
    ));
    let mut first = [0u8; 1];
    r.read(0, &mut first).unwrap();
    assert_eq!(first[0], 0xEE);
    let none = |_: &str| false;
    let lw = w.copy_to_arena(&weights_arena(1024), &r, 7, &none).unwrap();
    assert!(lw.tensors.is_empty());
    assert_eq!(lw.end_offset, 7);
    std::fs::remove_file(&p).ok();
}

#[test]
fn display_mentions_path_and_name() {
    let e = WeightError::InvalidFormat { path: PathBuf::from("/x/y.safetensors"), reason: "boom".into() };
    let s = e.to_string();
    assert!(s.contains("/x/y.safetensors") && s.contains("boom"));
    assert!(WeightError::NotFound { name: "w1".into() }.to_string().contains("w1"));
}
