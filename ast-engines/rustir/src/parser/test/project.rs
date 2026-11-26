use super::*;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rustir-parser-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn missing_manifest_is_not_found() {
    let dir = scratch("missing");
    let result = parse_project(&dir);
    assert_eq!(result, Err(Error::NotFound(dir.join("Cargo.toml"))));
}

#[test]
fn existing_manifest_yields_project() {
    let dir = scratch("present");
    fs::write(dir.join("Cargo.toml"), "").unwrap();
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.root, dir);
    assert!(project.modules.is_empty());
}
