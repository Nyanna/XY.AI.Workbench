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
#[test]
fn parse_display() {
    let error = Error::Parse {
        path: PathBuf::from("/x.rs"),
        message: "bad".into(),
    };
    assert_eq!(error.to_string(), "parse error in /x.rs: bad");
}
#[test]
fn conflicts_display_lists_every_name_and_file() {
    let error = Error::NameConflicts(
        vec![
            NameConflict { name : "A".into(), files : vec![PathBuf::from("/a.rs"),
            PathBuf::from("/b.rs")], }, NameConflict { name : "B".into(), files :
            vec![PathBuf::from("/c.rs"), PathBuf::from("/d.rs")], },
        ],
    );
    assert_eq!(
        error.to_string(),
        "2 name conflict(s):\n  A\n    /a.rs\n    /b.rs\n  B\n    /c.rs\n    /d.rs"
    );
}
