use super::*;
use crate::error::NameConflict;
use std::path::PathBuf;
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("rustir-parser-{name}-{}", std::process::id()));
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
    assert_eq!(parse_project(& dir), Err(Error::NotFound(dir.clone())));
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
    assert_eq!(
        project.objects.get("Foo").unwrap().dependencies, vec!["Bar".to_string()]
    );
    assert_eq!(
        project.objects.get("make").unwrap().dependencies, vec!["Foo".to_string()]
    );
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
    assert!(
        project.objects.get("Loader").unwrap().body.contains("impl Loader for Vec<u8>")
    );
}
#[test]
fn impl_with_only_a_local_type_in_the_signature_belongs_to_that_type() {
    let dir = scratch("signature-owner");
    write(&dir, "a.rs", "pub struct Config;");
    write(
        &dir,
        "b.rs",
        "impl From<Config> for String { fn from(_: Config) -> String { todo!() } }",
    );
    let project = parse_project(&dir).unwrap();
    assert!(
        project.objects.get("Config").unwrap().body
        .contains("impl From<Config> for String")
    );
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
        NameConflict { name : "Dup".into(), files : vec![dir.join("a.rs"), dir
        .join("b.rs")], }, NameConflict { name : "twice".into(), files : vec![dir
        .join("a.rs"), dir.join("b.rs")], },
    ];
    assert_eq!(parse_project(& dir), Err(Error::NameConflicts(expected)));
}
#[test]
fn test_directories_and_test_modules_do_not_conflict() {
    let dir = scratch("tests");
    write(
        &dir,
        "a.rs",
        "pub fn shared() {}\n#[cfg(test)]\nmod tests { pub fn helper() {} }",
    );
    write(&dir, "test/a.rs", "pub fn shared() {}");
    write(&dir, "b.rs", "#[cfg(test)]\nmod tests { pub fn helper() {} }");
    assert_eq!(parse_project(& dir).unwrap().objects.len(), 1);
}
#[test]
fn invalid_syntax_is_a_parse_error() {
    let dir = scratch("syntax");
    write(&dir, "broken.rs", "fn {");
    assert!(matches!(parse_project(& dir), Err(Error::Parse { .. })));
}
#[test]
fn external_imports_follow_the_objects_that_use_them() {
    let dir = scratch("imports");
    write(
        &dir,
        "a.rs",
        "use std::collections::BTreeMap;\nuse std::fmt;\npub struct Foo { m: BTreeMap<u8, u8> }",
    );
    let project = parse_project(&dir).unwrap();
    let expected = vec![
        "std::collections::BTreeMap".to_string(), "std::fmt".to_string()
    ];
    assert_eq!(project.imports["Foo"], expected);
}
#[test]
fn internal_uses_are_not_external_imports() {
    let dir = scratch("internal");
    write(&dir, "lib.rs", "mod a; mod b;");
    write(
        &dir,
        "a.rs",
        "use crate::b::Bar;\nuse b::Bar as Other;\npub fn f() -> Bar { todo!() }",
    );
    write(&dir, "b.rs", "pub struct Bar;");
    let project = parse_project(&dir).unwrap();
    assert!(project.imports.is_empty());
    assert!(project.qualifiers.contains("a") && project.qualifiers.contains("b"));
}
#[test]
fn impl_blocks_contribute_the_imports_of_their_file() {
    let dir = scratch("impl-imports");
    write(&dir, "a.rs", "pub struct Foo;");
    write(
        &dir,
        "b.rs",
        "use std::fmt;\nimpl fmt::Display for Foo { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { todo!() } }",
    );
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.imports["Foo"], vec!["std::fmt".to_string()]);
}
#[test]
fn manifest_provides_package_and_dependencies() {
    let dir = scratch("manifest");
    write(
        &dir,
        "Cargo.toml",
        "[package]\nname = \"demo-app\"\n\n[dependencies]\nsyn = \"2\"\n",
    );
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.manifest.package.as_deref(), Some("demo-app"));
    assert!(project.manifest.dependencies.contains("syn = \"2\""));
    assert!(project.qualifiers.contains("demo_app"));
}
#[test]
fn module_imports_stay_with_the_objects_of_that_module() {
    let dir = scratch("module-imports");
    write(
        &dir,
        "a.rs",
        "use std::fmt;\nmod m { use std::io::Write; pub struct A(Box<dyn Write>); }\npub struct B(Box<dyn fmt::Debug>);",
    );
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.imports["A"], vec!["std::io::Write".to_string()]);
    assert_eq!(project.imports["B"], vec!["std::fmt".to_string()]);
}
#[test]
fn gates_are_recorded_per_object() {
    let dir = scratch("gates");
    write(&dir, "a.rs", "#[cfg(unix)] mod m { pub fn f() {} }\npub fn g() {}");
    let project = parse_project(&dir).unwrap();
    assert_eq!(project.gates["f"], "unix");
    assert!(! project.gates.contains_key("g"));
}
