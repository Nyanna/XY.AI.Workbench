use super::*;
use std::sync::Arc;
use std::thread;

#[test]
fn lock_returns_the_guard_of_a_healthy_mutex() {
    let m = Mutex::new(5);
    *lock(&m) += 1;
    assert_eq!(*lock(&m), 6);
}

#[test]
fn lock_recovers_a_poisoned_mutex() {
    let m = Arc::new(Mutex::new(1));
    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || {
        let _guard = m2.lock().unwrap();
        panic!("poison");
    })
    .join();
    assert!(m.is_poisoned());
    assert_eq!(*lock(&m), 1);
}

#[test]
fn lock_accepts_unsized_contents() {
    let m: Box<Mutex<[u8]>> = Box::new(Mutex::new([1u8, 2, 3]));
    assert_eq!(lock(&m).len(), 3);
}

#[test]
fn payload_text_covers_str_and_string() {
    let a: Box<dyn Any + Send> = Box::new("static text");
    let b: Box<dyn Any + Send> = Box::new(String::from("owned text"));
    let c: Box<dyn Any + Send> = Box::new(7u32);
    assert_eq!(panic_payload_text(a.as_ref()), Some("static text"));
    assert_eq!(panic_payload_text(b.as_ref()), Some("owned text"));
    assert_eq!(panic_payload_text(c.as_ref()), None);
}
