use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::a::index_format::HashAlgorithm;
use std::io::{self, Cursor};
fn at(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}
fn meta(size: u64, mtime: u64, ctime: Option<u64>) -> FileMeta {
    FileMeta {
        size,
        mtime: at(mtime),
        ctime: ctime.map(at),
    }
}
fn record(m: FileMeta) -> FileRecord {
    FileRecord {
        meta: m,
        reference: at(10_000),
        stamp: None,
        content_hash: None,
    }
}
fn header(algorithm: HashAlgorithm, version: u64) -> IndexHeader {
    let base = sample_header();
    IndexHeader {
        hash_algorithm: algorithm,
        hash_version: version,
        ..base
    }
}
fn sample_header() -> IndexHeader {
    use crate::resident::l130::l120::l110::l100::l090::l080::a::index_format::*;
    let pairs: Vec<(String, String)> = [
        (KEY_SCHEMA_VERSION, "1"),
        (KEY_MAX_STAMP, "0"),
        (KEY_HASH_ALGORITHM, "fast_64_bit"),
        (KEY_HASH_VERSION, "1"),
        (KEY_MODEL_ID, "m"),
        (KEY_MODEL_REVISION, "r"),
        (KEY_DIMENSION, "384"),
        (KEY_POOLING, "masked_mean"),
        (KEY_NORMALIZATION, "l2"),
        (KEY_QUERY_PREFIX, "query: "),
        (KEY_PASSAGE_PREFIX, "passage: "),
        (KEY_STORAGE_DTYPE, "F16"),
        (KEY_COMPUTE_DTYPE, "cpu=F32,gpu=F16"),
        (KEY_MAX_SEQ_LEN, "512"),
        (KEY_TOKENIZER_FINGERPRINT, "fnv1a64:000000000000002a"),
        (KEY_PREPROCESSING_ID, "p"),
    ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    IndexHeader::from_kv(&pairs).expect("valid sample header")
}
fn reason_of(c: ChangeCheck) -> CheckReason {
    match c {
        ChangeCheck::ContentCheckRequired { reason, diagnostic } => {
            assert!(! diagnostic.is_empty());
            reason
        }
        ChangeCheck::Unchanged => panic!("expected content check"),
    }
}
#[test]
fn types_are_send_sync() {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<FileMeta>();
    assert_traits::<FileRecord>();
    assert_traits::<CheckReason>();
    assert_traits::<ChangeCheck>();
    assert_traits::<ContentCheck>();
}
#[test]
fn racy_window_is_two_seconds() {
    assert_eq!(RACY_WINDOW, Duration::from_secs(2));
}
#[test]
fn racy_boundaries() {
    let mut r = record(meta(1, 0, None));
    r.reference = at(100);
    for (mtime, expected) in [
        (97, false),
        (98, true),
        (99, true),
        (100, true),
        (101, true),
    ] {
        r.meta.mtime = at(mtime);
        assert_eq!(r.is_racy(), expected, "mtime {mtime}");
    }
}
#[test]
fn racy_uses_later_of_mtime_and_ctime() {
    let mut r = record(meta(1, 10, Some(99)));
    r.reference = at(100);
    assert!(r.is_racy());
    r.meta.ctime = Some(at(10));
    assert!(! r.is_racy());
    r.meta.mtime = at(99);
    assert!(r.is_racy());
}
#[test]
fn racy_underflow_is_not_racy() {
    let (mut lo, mut hi) = (0u64, i64::MAX as u64);
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if UNIX_EPOCH.checked_sub(Duration::from_secs(mid)).is_some() {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let earliest = UNIX_EPOCH - Duration::from_secs(lo);
    let mut r = record(FileMeta {
        size: 1,
        mtime: earliest,
        ctime: None,
    });
    r.reference = earliest;
    assert!(r.reference.checked_sub(RACY_WINDOW).is_none());
    assert!(! r.is_racy());
}
#[test]
fn unchanged_requires_equal_meta_and_not_racy() {
    let m = meta(5, 100, Some(100));
    let r = record(m);
    assert_eq!(check_file(& r, & m, None), ChangeCheck::Unchanged);
}
#[test]
fn equal_stamp_alone_is_not_enough() {
    let m = meta(5, 100, Some(100));
    let mut r = record(m);
    r.stamp = Some(ChangeStamp::from_raw(7));
    r.reference = at(101);
    let c = check_file(&r, &m, Some(ChangeStamp::from_raw(7)));
    assert_eq!(reason_of(c), CheckReason::Racy);
}
#[test]
fn stamp_difference_wins_over_everything() {
    let m = meta(5, 100, Some(100));
    let mut r = record(m);
    r.stamp = Some(ChangeStamp::from_raw(1));
    let other = meta(6, 101, None);
    match check_file(&r, &other, Some(ChangeStamp::from_raw(2))) {
        ChangeCheck::ContentCheckRequired { reason, diagnostic } => {
            assert_eq!(reason, CheckReason::StampDiffers);
            assert!(diagnostic.contains('1') && diagnostic.contains('2'));
        }
        ChangeCheck::Unchanged => panic!("expected content check"),
    }
}
#[test]
fn missing_stamp_on_either_side_falls_through() {
    let m = meta(5, 100, Some(100));
    let mut r = record(m);
    r.stamp = Some(ChangeStamp::from_raw(1));
    assert_eq!(check_file(& r, & m, None), ChangeCheck::Unchanged);
    r.stamp = None;
    assert_eq!(
        check_file(& r, & m, Some(ChangeStamp::from_raw(2))), ChangeCheck::Unchanged
    );
}
#[test]
fn size_then_mtime_then_ctime_order() {
    let r = record(meta(5, 100, Some(100)));
    assert_eq!(
        reason_of(check_file(& r, & meta(6, 101, Some(101)), None)),
        CheckReason::SizeDiffers
    );
    assert_eq!(
        reason_of(check_file(& r, & meta(5, 101, Some(101)), None)),
        CheckReason::MtimeDiffers
    );
    assert_eq!(
        reason_of(check_file(& r, & meta(5, 100, Some(101)), None)),
        CheckReason::CtimeDiffers
    );
}
#[test]
fn mtime_comparison_is_exact() {
    let r = record(meta(5, 100, None));
    let mut cur = meta(5, 100, None);
    cur.mtime += Duration::from_nanos(1);
    assert_eq!(reason_of(check_file(& r, & cur, None)), CheckReason::MtimeDiffers);
}
#[test]
fn ctime_option_comparison() {
    let none = record(meta(5, 100, None));
    assert_eq!(check_file(& none, & meta(5, 100, None), None), ChangeCheck::Unchanged);
    assert_eq!(
        reason_of(check_file(& none, & meta(5, 100, Some(100)), None)),
        CheckReason::CtimeDiffers
    );
    let some = record(meta(5, 100, Some(100)));
    assert_eq!(
        reason_of(check_file(& some, & meta(5, 100, None), None)),
        CheckReason::CtimeDiffers
    );
}
#[test]
fn racy_diagnostic_names_values() {
    let m = meta(5, 100, None);
    let mut r = record(m);
    r.reference = at(101);
    match check_file(&r, &m, None) {
        ChangeCheck::ContentCheckRequired { reason, diagnostic } => {
            assert_eq!(reason, CheckReason::Racy);
            assert!(diagnostic.contains("mtime=100."));
            assert!(diagnostic.contains("ctime=keine"));
            assert!(diagnostic.contains("Referenzzeitpunkt=101."));
            assert!(diagnostic.contains("Fenster=2s"));
        }
        ChangeCheck::Unchanged => panic!("expected racy"),
    }
}
#[test]
fn pre_epoch_times_do_not_panic() {
    let before = UNIX_EPOCH - Duration::from_secs(5);
    let m = FileMeta {
        size: 1,
        mtime: before,
        ctime: Some(before),
    };
    let mut r = record(m);
    r.reference = before;
    let cur = FileMeta { size: 2, ..m };
    assert_eq!(reason_of(check_file(& r, & cur, None)), CheckReason::SizeDiffers);
    let cur = FileMeta { mtime: UNIX_EPOCH, ..m };
    assert_eq!(reason_of(check_file(& r, & cur, None)), CheckReason::MtimeDiffers);
}
#[test]
fn content_hash_known_vectors() {
    assert_eq!(content_hash(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(content_hash(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(content_hash(b"foobar"), 0x8594_4171_f739_67e8);
}
#[test]
fn reader_matches_slice_hash_across_blocks() {
    let data: Vec<u8> = (0..(READ_BLOCK_SIZE * 2 + 123))
        .map(|i| (i % 251) as u8)
        .collect();
    assert_eq!(content_hash_reader(Cursor::new(& data)).unwrap(), content_hash(& data));
    assert_eq!(
        content_hash_reader(Cursor::new(Vec::< u8 >::new())).unwrap(), content_hash(b"")
    );
}
struct Flaky {
    data: Vec<u8>,
    pos: usize,
    interrupt_next: bool,
}
impl Read for Flaky {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.interrupt_next {
            self.interrupt_next = false;
            return Err(io::Error::from(ErrorKind::Interrupted));
        }
        self.interrupt_next = true;
        let n = out.len().min(3).min(self.data.len() - self.pos);
        out[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}
#[test]
fn reader_retries_on_interrupted() {
    let data = b"hello interrupted world".to_vec();
    let r = Flaky {
        data: data.clone(),
        pos: 0,
        interrupt_next: true,
    };
    assert_eq!(content_hash_reader(r).unwrap(), content_hash(& data));
}
struct Failing;
impl Read for Failing {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(ErrorKind::PermissionDenied, "denied"))
    }
}
#[test]
fn reader_propagates_other_errors() {
    let err = content_hash_reader(Failing).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::PermissionDenied);
}
#[test]
fn header_matching() {
    assert!(hash_matches_header(& header(HASH_ALGORITHM, HASH_VERSION)));
    assert!(! hash_matches_header(& header(HashAlgorithm::Fast128, HASH_VERSION)));
    assert!(! hash_matches_header(& header(HASH_ALGORITHM, HASH_VERSION + 1)));
}
#[test]
fn compare_content_results() {
    let good = header(HASH_ALGORITHM, HASH_VERSION);
    let mut r = record(meta(1, 1, None));
    assert_eq!(compare_content(& r, & good, 5), ContentCheck::NoComparableHash);
    r.content_hash = Some(5);
    assert_eq!(compare_content(& r, & good, 5), ContentCheck::Unchanged);
    assert_eq!(compare_content(& r, & good, 6), ContentCheck::Changed);
    let wrong_algo = header(HashAlgorithm::Cryptographic, HASH_VERSION);
    assert_eq!(compare_content(& r, & wrong_algo, 5), ContentCheck::NoComparableHash);
    let wrong_version = header(HASH_ALGORITHM, HASH_VERSION + 1);
    assert_eq!(compare_content(& r, & wrong_version, 5), ContentCheck::NoComparableHash);
}
