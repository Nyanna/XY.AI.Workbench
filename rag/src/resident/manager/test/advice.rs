use super::*;
use memmap2::MmapMut;

fn anon(len: usize) -> MmapAdvice {
    MmapAdvice::new(MmapMut::map_anon(len).unwrap().make_read_only().unwrap())
}

#[test]
fn all_hints_accepted_on_valid_range() {
    let a = anon(3 * 4096);
    for hint in [Hint::WillNeed, Hint::Sequential, Hint::Random, Hint::Cold] {
        a.advise(100..9000, hint).unwrap();
    }
}

#[test]
fn out_of_bounds_and_empty_ranges() {
    let a = anon(4096);
    assert!(a.advise(0..4097, Hint::WillNeed).is_err());
    a.advise(10..10, Hint::Cold).unwrap();
}
