use super::*;
use crate::emit::edits::apply_edits;
use crate::ir::{Object, ObjectKind, ObjectTable};
fn rewrite(source: &str) -> String {
    let mut table = ObjectTable::new();
    for (name, kind) in [
        ("Object", ObjectKind::Struct),
        ("run", ObjectKind::Function),
        ("report", ObjectKind::Function),
        ("Result", ObjectKind::TypeAlias),
        ("shout", ObjectKind::Macro),
    ] {
        table
            .insert(Object {
                name: name.into(),
                kind,
                body: String::new(),
                dependencies: Vec::new(),
            });
    }
    let qualifiers: BTreeSet<String> = ["ir", "cli", "report", "demo"]
        .iter()
        .map(|q| q.to_string())
        .collect();
    let file = syn::parse_file(source).unwrap();
    let edits = path_edits(
        &file,
        &Source::new(source),
        &Layout::new(&table),
        &qualifiers,
    );
    apply_edits(source, edits)
}
#[test]
fn crate_paths_point_at_the_flat_module() {
    assert_eq!(
        rewrite("fn f() -> crate::ir::Object { todo!() }"),
        "fn f() -> crate::object::Object { todo!() }"
    );
}
#[test]
fn trailing_segments_are_kept() {
    assert_eq!(
        rewrite("fn f() { crate::ir::Object::new(); }"),
        "fn f() { crate::object::Object::new(); }"
    );
}
#[test]
fn module_qualifiers_and_the_crate_name_are_rewritten() {
    assert_eq!(
        rewrite("fn f() { cli::run(); demo::run(); }"),
        "fn f() { crate::run::run(); crate::run::run(); }"
    );
}
#[test]
fn the_deepest_object_wins_when_module_and_object_share_a_name() {
    assert_eq!(
        rewrite("fn f() { crate::report::report(); }"),
        "fn f() { crate::report::report(); }"
    );
    assert_eq!(
        rewrite("fn f() { cli::report::report(1); }"),
        "fn f() { crate::report::report(1); }"
    );
}
#[test]
fn foreign_paths_are_left_alone() {
    let source = "fn f() -> std::fmt::Result { fmt::Result::Ok(()) }";
    assert_eq!(rewrite(source), source);
}
#[test]
fn paths_without_a_known_object_are_left_alone() {
    let source = "fn f() { crate::ir::unknown(); }";
    assert_eq!(rewrite(source), source);
}
#[test]
fn macro_paths_are_left_alone() {
    let source = "fn f() { crate::shout!(); }";
    assert_eq!(rewrite(source), source);
}
#[test]
fn nested_generic_arguments_are_rewritten_too() {
    assert_eq!(
        rewrite("fn f() -> Vec<crate::ir::Object> { todo!() }"),
        "fn f() -> Vec<crate::object::Object> { todo!() }"
    );
}
#[test]
fn paths_inside_macro_arguments_are_rewritten() {
    assert_eq!(
        rewrite("fn f() { println!(\"{}\", cli::run(crate::ir::Object::new())); }"),
        "fn f() { println!(\"{}\", crate::run::run(crate::object::Object::new())); }"
    );
}
#[test]
fn macro_tokens_without_known_objects_are_left_alone() {
    let source = "fn f() { println!(\"{}\", std::fmt::Result::Ok(())); }";
    assert_eq!(rewrite(source), source);
}
