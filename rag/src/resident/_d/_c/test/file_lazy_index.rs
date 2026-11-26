use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::_b::_a::_9::_8::a::index_format::{
    TokenizerFingerprint, TOKENIZER_FP_ALGORITHM,
};
use super::super::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use super::super::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::ModelDescriptor;
use super::super::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn header(dim: usize) -> IndexHeader {
    let mut h = IndexHeader::from_descriptor(
        &ModelDescriptor::e5_small_repo_default(DType::F16),
        "rev-1",
        TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 0x2a),
        "prep",
    )
    .unwrap();
    h.dimension = dim;
    h
}

fn missing_path() -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("file_lazy_index_missing_{}_{n}.parquet", std::process::id()))
}

#[test]
fn first_get_opens_and_later_gets_return_the_same_instance() {
    let lazy = FileLazyIndex::new(missing_path(), header(8), OpenOptions::default(), IndexSlot::default());
    assert!(lazy.slot().get().is_none());
    let a = lazy.get().unwrap();
    let b = lazy.get().unwrap();
    assert!(Arc::ptr_eq(&a, &b));
    assert!(Arc::ptr_eq(lazy.slot().get().unwrap(), &a));
}

#[test]
fn missing_file_gives_an_empty_index() {
    let lazy = FileLazyIndex::new(missing_path(), header(8), OpenOptions::default(), IndexSlot::default());
    let index = lazy.get().unwrap();
    assert!(index.read(EntryId(1)).vector.is_empty());
}

#[test]
fn a_filled_slot_wins_over_opening() {
    let slot = IndexSlot::default();
    let preset = FileLazyIndex::new(missing_path(), header(8), OpenOptions::default(), IndexSlot::default())
        .get()
        .unwrap();
    slot.set(Arc::clone(&preset)).ok().unwrap();
    let lazy = FileLazyIndex::new(missing_path(), header(8), OpenOptions::default(), slot);
    assert!(Arc::ptr_eq(&lazy.get().unwrap(), &preset));
}

#[test]
fn an_unreadable_file_is_an_error_and_leaves_the_slot_empty() {
    let path = missing_path();
    std::fs::write(&path, b"not a parquet file").unwrap();
    let lazy = FileLazyIndex::new(path.clone(), header(8), OpenOptions::default(), IndexSlot::default());
    assert!(lazy.get().is_err());
    assert!(lazy.slot().get().is_none());
    let _ = std::fs::remove_file(path);
}

#[test]
fn the_type_is_a_lazy_index_object() {
    let lazy: Arc<dyn LazyIndex> =
        Arc::new(FileLazyIndex::new(missing_path(), header(8), OpenOptions::default(), IndexSlot::default()));
    assert!(lazy.get().is_ok());
}
