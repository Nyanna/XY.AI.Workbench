use super::*;
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("rustir-files-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
fn touch(path: &Path) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "").unwrap();
}
#[test]
fn finds_rust_files_recursively_in_sorted_order() {
    let dir = scratch("sorted");
    touch(&dir.join("b.rs"));
    touch(&dir.join("a/mod.rs"));
    touch(&dir.join("notes.txt"));
    let files = source_files(&dir).unwrap();
    assert_eq!(files, vec![dir.join("a/mod.rs"), dir.join("b.rs")]);
}
#[test]
fn skips_test_target_and_hidden_directories() {
    let dir = scratch("skipped");
    touch(&dir.join("keep.rs"));
    touch(&dir.join("test/x.rs"));
    touch(&dir.join("tests/x.rs"));
    touch(&dir.join("target/x.rs"));
    touch(&dir.join(".git/x.rs"));
    assert_eq!(source_files(& dir).unwrap(), vec![dir.join("keep.rs")]);
}
#[test]
fn skips_nested_packages() {
    let dir = scratch("nested");
    touch(&dir.join("keep.rs"));
    touch(&dir.join("inner/Cargo.toml"));
    touch(&dir.join("inner/src/lib.rs"));
    touch(&dir.join("plain/x.rs"));
    assert_eq!(
        source_files(& dir).unwrap(), vec![dir.join("keep.rs"), dir.join("plain/x.rs")]
    );
}
#[test]
fn the_root_package_itself_is_not_skipped() {
    let dir = scratch("root-manifest");
    touch(&dir.join("Cargo.toml"));
    touch(&dir.join("keep.rs"));
    assert_eq!(source_files(& dir).unwrap(), vec![dir.join("keep.rs")]);
}
#[test]
fn missing_directory_is_an_io_error() {
    let dir = scratch("missing").join("absent");
    assert!(matches!(source_files(& dir), Err(crate ::error::Error::Io(_))));
}
