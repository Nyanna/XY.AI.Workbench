use super::*;
use crate::ir::Object;

fn table(entries: &[(&str, ObjectKind)]) -> ObjectTable {
    let mut table = ObjectTable::new();
    for (name, kind) in entries {
        table.insert(Object {
            name: name.to_string(),
            kind: *kind,
            body: String::new(),
            dependencies: Vec::new(),
        });
    }
    table
}

#[test]
fn snake_case_handles_words_acronyms_and_constants() {
    assert_eq!(snake_case("ObjectTable"), "object_table");
    assert_eq!(snake_case("HTTPServer"), "http_server");
    assert_eq!(snake_case("CONFLICTS_FILE"), "conflicts_file");
    assert_eq!(snake_case("parse_args"), "parse_args");
    assert_eq!(snake_case("Utf8Text"), "utf8_text");
}

#[test]
fn names_starting_with_a_digit_get_a_prefix() {
    assert_eq!(snake_case("9lives"), "_9lives");
}

#[test]
fn every_object_gets_a_module() {
    let layout = Layout::new(&table(&[("Error", ObjectKind::Enum), ("run", ObjectKind::Function)]));
    assert_eq!(layout.module("Error"), Some("error"));
    assert_eq!(layout.module("run"), Some("run"));
    assert_eq!(layout.module("absent"), None);
}

#[test]
fn colliding_module_names_are_made_unique_by_kind() {
    let layout = Layout::new(&table(&[
        ("FOO", ObjectKind::Const),
        ("Foo", ObjectKind::Struct),
        ("foo", ObjectKind::Function),
    ]));
    assert_eq!(layout.module("FOO"), Some("foo"));
    assert_eq!(layout.module("Foo"), Some("foo_struct"));
    assert_eq!(layout.module("foo"), Some("foo_fn"));
}

#[test]
fn reserved_names_are_not_used_as_modules() {
    let layout = Layout::new(&table(&[
        ("main", ObjectKind::Function),
        ("type", ObjectKind::Function),
        ("Std", ObjectKind::Struct),
    ]));
    assert_eq!(layout.module("main"), Some("main_"));
    assert_eq!(layout.module("type"), Some("type_"));
    assert_eq!(layout.module("Std"), Some("std_"));
}

#[test]
fn macros_are_not_rewritable() {
    let layout = Layout::new(&table(&[("m", ObjectKind::Macro), ("f", ObjectKind::Function)]));
    assert!(layout.is_macro("m"));
    assert_eq!(layout.rewritable("m"), None);
    assert_eq!(layout.rewritable("f"), Some("f"));
}

#[test]
fn module_names_are_queryable() {
    let layout = Layout::new(&table(&[("Foo", ObjectKind::Struct)]));
    assert!(layout.is_module_name("foo"));
    assert!(!layout.is_module_name("Foo"));
}
