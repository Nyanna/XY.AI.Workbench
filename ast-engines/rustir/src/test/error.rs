use super::*;

#[test]
fn usage_display() {
    assert_eq!(Error::Usage("x".into()).to_string(), "usage error: x");
}

#[test]
fn not_found_display() {
    let error = Error::NotFound(PathBuf::from("/a/b"));
    assert_eq!(error.to_string(), "not found: /a/b");
}

#[test]
fn io_error_converts() {
    let source = std::io::Error::new(std::io::ErrorKind::Other, "boom");
    assert_eq!(Error::from(source), Error::Io("boom".into()));
}
