use super::*;
use std::sync::Arc;

#[test]
fn initial_state() {
    let b = SeqBuf::new(4);
    let (v, r) = b.read();
    assert_eq!(v, vec![0; 4]);
    assert_eq!(r, 0);
    assert_eq!(b.sequence(), 0);
}

#[test]
fn write_then_read() {
    let b = SeqBuf::from_words(&[1, 2, 3]);
    b.write(&[4, 5, 6]);
    assert_eq!(b.read().0, vec![4, 5, 6]);
    assert_eq!(b.sequence(), 2);
}

#[test]
fn zero_length() {
    let b = SeqBuf::new(0);
    b.write(&[]);
    assert_eq!(b.sequence(), 2);
    assert!(b.read().0.is_empty());
}

#[test]
#[should_panic]
fn write_length_mismatch_panics() {
    let b = SeqBuf::new(2);
    b.write(&[1]);
}

#[test]
#[should_panic]
fn read_length_mismatch_panics() {
    let b = SeqBuf::new(2);
    let mut out = [0usize; 3];
    b.read_into(&mut out);
}

#[test]
fn concurrent_writers_and_readers() {
    let b = Arc::new(SeqBuf::new(8));
    let mut hs = Vec::new();
    for t in 1..=4usize {
        let b = b.clone();
        hs.push(std::thread::spawn(move || {
            for i in 0..2000usize {
                let v = t * 1_000_000 + i;
                b.write(&[v; 8]);
            }
        }));
    }
    for _ in 0..2 {
        let b = b.clone();
        hs.push(std::thread::spawn(move || {
            for _ in 0..2000 {
                let (v, _) = b.read();
                assert!(v.iter().all(|&x| x == v[0]));
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(b.sequence(), 2 * 4 * 2000);
}

#[test]
fn raw_parts_view() {
    let b = SeqBuf::from_words(&[7, 8]);
    let r = b.region();
    let c = r;
    c.write(&[9, 10]);
    assert_eq!(r.read().0, vec![9, 10]);
    assert_eq!(r.len(), 2);
}
