use super::*;
use crate::ir::{Object, ObjectTable};
use std::path::PathBuf;

fn declared(name: &str, module: &str, kind: ObjectKind, public: bool) -> Declared {
    Declared {
        name: name.into(),
        module: module.into(),
        kind,
        public,
    }
}

fn layout_of(names: &[(&str, ObjectKind)]) -> Layout {
    let mut table = ObjectTable::new();
    for (name, kind) in names {
        table.insert(Object {
            name: name.to_string(),
            kind: *kind,
            body: String::new(),
            dependencies: Vec::new(),
        });
    }
    Layout::new(&table)
}

#[test]
fn package_name_prefers_the_manifest() {
    let mut project = Project::new(PathBuf::from("/work/Some Dir"));
    assert_eq!(package_name(&project), "some_dir");
    project.manifest.package = Some("demo-app".into());
    assert_eq!(package_name(&project), "demo_app");
}

#[test]
fn package_name_is_never_empty_or_numeric() {
    assert_eq!(package_name(&Project::new(PathBuf::from("/"))), "generated");
    assert_eq!(package_name(&Project::new(PathBuf::from("/1st"))), "p_1st");
}

#[test]
fn lib_declares_macros_first_and_reexports_public_objects() {
    let layout = layout_of(&[("Alpha", ObjectKind::Struct), ("run", ObjectKind::Function), ("shout", ObjectKind::Macro)]);
    let text = lib_source(
        &[
            declared("Alpha", "alpha", ObjectKind::Struct, true),
            declared("run", "run", ObjectKind::Function, false),
            declared("shout", "shout", ObjectKind::Macro, true),
        ],
        &layout,
    );
    assert_eq!(
        text,
        "#![allow(unused_imports)]\n\n#[macro_use]\nmod shout;\npub mod alpha;\nmod run;\n\npub use alpha::Alpha;\n"
    );
}

#[test]
fn reexports_skip_main_and_type_names_that_equal_a_module() {
    let layout = layout_of(&[("foo", ObjectKind::TypeAlias), ("Foo", ObjectKind::Struct), ("main", ObjectKind::Function)]);
    let text = lib_source(
        &[
            declared("Foo", "foo", ObjectKind::Struct, true),
            declared("foo", "foo_type", ObjectKind::TypeAlias, true),
            declared("main", "main_", ObjectKind::Function, true),
        ],
        &layout,
    );
    assert!(text.contains("pub use foo::Foo;"));
    assert!(!text.contains("pub use foo_type::foo;"));
    assert!(!text.contains("pub use main_::main;"));
}

#[test]
fn main_source_imports_the_entry_point() {
    assert_eq!(main_source("demo", "main_"), "use demo::main_::main;\n");
}

#[test]
fn cargo_source_carries_edition_and_dependencies() {
    let mut project = Project::new(PathBuf::from("/p"));
    project.manifest.edition = Some("2018".into());
    project.manifest.dependencies = "[dependencies]\nsyn = \"2\"\n".into();
    let text = cargo_source(&project, "demo", false);
    assert!(text.starts_with(GENERATED_MARK));
    assert!(text.contains("name = \"demo\"") && text.contains("edition = \"2018\""));
    assert!(text.ends_with("\n[dependencies]\nsyn = \"2\"\n"));
    assert!(!text.contains("[[bin]]"));
}

#[test]
fn cargo_source_declares_the_binary_when_requested() {
    let text = cargo_source(&Project::new(PathBuf::from("/p")), "demo", true);
    assert!(text.contains("[[bin]]\nname = \"demo\"\npath = \"src/main.rs\""));
    assert!(text.contains("edition = \"2021\""));
    assert!(!text.contains("[dependencies]"));
}
