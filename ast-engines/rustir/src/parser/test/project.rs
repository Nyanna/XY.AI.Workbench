use super::*;
use crate::error::NameConflict;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rustir-parser-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, relative: &str, text: &str) {
    let path = dir.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

#[test]
fn missing_directory_is_not_found() {
    let dir = scratch("missing").join("absent");
    assert_eq!(parse_project(&dir), Err(Error::NotFound(dir.clone())));
}

#[test]
fn empty_directory_yields_empty_project() {
    let dir = scratch("empty");
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.root, dir);
    assert!(project.objects.is_empty());
}

#[test]
fn objects_are_keyed_by_name_regardless_of_module() {
    let dir = scratch("names");
    write(&dir, "a.rs", "pub struct Foo { b: Bar }");
    write(&dir, "sub/b.rs", "pub struct Bar; pub fn make() -> Foo { todo!() }");
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.objects.len(), 3);
    assert_eq!(project.objects.get("Foo").unwrap().dependencies, vec!["Bar".to_string()]);
    assert_eq!(project.objects.get("make").unwrap().dependencies, vec!["Foo".to_string()]);
    assert!(project.objects.get("Bar").unwrap().dependencies.is_empty());
}

#[test]
fn unknown_names_are_not_dependencies() {
    let dir = scratch("unknown");
    write(&dir, "a.rs", "pub struct Foo { v: Vec<String> }");
    let project = parse_project(&dir).unwrap();
    assert!(project.objects.get("Foo").unwrap().dependencies.is_empty());
}

#[test]
fn impl_block_joins_the_body_of_its_target_type() {
    let dir = scratch("impls");
    write(&dir, "a.rs", "pub struct Foo; pub trait Contract {} pub struct Out;");
    write(&dir, "b.rs", "impl Contract for Foo { fn get(&self) -> Out { todo!() } }");
    let project = parse_project(&dir).unwrap();
    let foo = project.objects.get("Foo").unwrap();
    assert_eq!(foo.dependencies, vec!["Contract".to_string(), "Out".to_string()]);
    assert!(foo.body.starts_with("pub struct Foo;"));
    assert!(foo.body.contains("impl Contract for Foo"));
    assert_eq!(project.objects.get("Contract").unwrap().body, "pub trait Contract {}");
}

#[test]
fn impl_for_foreign_type_belongs_to_the_local_trait() {
    let dir = scratch("trait-owner");
    write(&dir, "a.rs", "pub trait Loader {}");
    write(&dir, "b.rs", "impl Loader for Vec<u8> {}");
    let project = parse_project(&dir).unwrap();
    assert!(project.objects.get("Loader").unwrap().body.contains("impl Loader for Vec<u8>"));
}

#[test]
fn impl_with_only_a_local_type_in_the_signature_belongs_to_that_type() {
    let dir = scratch("signature-owner");
    write(&dir, "a.rs", "pub struct Config;");
    write(&dir, "b.rs", "impl From<Config> for String { fn from(_: Config) -> String { todo!() } }");
    let project = parse_project(&dir).unwrap();
    assert!(project.objects.get("Config").unwrap().body.contains("impl From<Config> for String"));
}

#[test]
fn impl_without_any_local_name_is_dropped() {
    let dir = scratch("dropped");
    write(&dir, "a.rs", "pub struct Foo;");
    write(&dir, "b.rs", "impl Default for Vec<u8> {}");
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.objects.get("Foo").unwrap().body, "pub struct Foo;");
}

#[test]
fn all_conflicts_are_reported_together() {
    let dir = scratch("conflicts");
    write(&dir, "a.rs", "pub struct Dup; pub fn twice() {}");
    write(&dir, "b.rs", "pub enum Dup {} pub fn twice() {}");
    write(&dir, "c.rs", "pub struct Unique;");
    let expected = vec![
        NameConflict {
            name: "Dup".into(),
            files: vec![dir.join("a.rs"), dir.join("b.rs")],
        },
        NameConflict {
            name: "twice".into(),
            files: vec![dir.join("a.rs"), dir.join("b.rs")],
        },
    ];
    assert_eq!(parse_project(&dir), Err(Error::NameConflicts(expected)));
}

#[test]
fn test_directories_and_test_modules_do_not_conflict() {
    let dir = scratch("tests");
    write(&dir, "a.rs", "pub fn shared() {}\n#[cfg(test)]\nmod tests { pub fn helper() {} }");
    write(&dir, "test/a.rs", "pub fn shared() {}");
    write(&dir, "b.rs", "#[cfg(test)]\nmod tests { pub fn helper() {} }");
    assert_eq!(parse_project(&dir).unwrap().objects.len(), 1);
}

#[test]
fn invalid_syntax_is_a_parse_error() {
    let dir = scratch("syntax");
    write(&dir, "broken.rs", "fn {");
    assert!(matches!(parse_project(&dir), Err(Error::Parse { .. })));
}
