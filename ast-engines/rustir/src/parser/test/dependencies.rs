use super::*;
fn names_of(source: &str) -> Vec<String> {
    let file = syn::parse_file(source).unwrap();
    item_names(&file.items[0]).into_iter().collect()
}
#[test]
fn collects_field_types() {
    let names = names_of("struct S { a: Alpha, b: Vec<Beta> }");
    assert!(names.contains(& "Alpha".to_string()));
    assert!(names.contains(& "Beta".to_string()));
}
#[test]
fn collects_calls_and_signature_types() {
    let names = names_of("fn f(x: In) -> Out { helper(x) }");
    for expected in ["In", "Out", "helper"] {
        assert!(names.contains(& expected.to_string()), "missing {expected}");
    }
}
#[test]
fn module_segments_are_not_dependencies() {
    let names = names_of("fn f() { crate::m::helper(); }");
    assert!(! names.contains(& "m".to_string()));
    assert!(! names.contains(& "crate".to_string()));
    assert!(names.contains(& "helper".to_string()));
}
#[test]
fn type_segment_counts_but_associated_item_name_does_not_replace_it() {
    let names = names_of("fn f() { m::Kind::build(); }");
    assert!(names.contains(& "Kind".to_string()));
    assert!(! names.contains(& "m".to_string()));
}
#[test]
fn collects_identifiers_inside_macro_tokens() {
    let names = names_of("fn f() { println!(\"{}\", compute(Value)); }");
    assert!(names.contains(& "compute".to_string()));
    assert!(names.contains(& "Value".to_string()));
}
#[test]
fn collects_inline_format_arguments() {
    let names = names_of(
        "fn f() { println!(\"{LIMIT} and {other:?} but {{ESCAPED}} {0}\"); }",
    );
    assert!(names.contains(& "LIMIT".to_string()));
    assert!(names.contains(& "other".to_string()));
    assert!(! names.contains(& "ESCAPED".to_string()));
}
#[test]
fn collects_trait_and_target_of_impl() {
    let names = names_of("impl Contract for Target {}");
    assert!(names.contains(& "Contract".to_string()));
    assert!(names.contains(& "Target".to_string()));
}
#[test]
fn does_not_collect_the_defined_name_or_local_bindings() {
    let names = names_of("fn defined() { let local = 1; }");
    assert!(! names.contains(& "local".to_string()));
    assert!(! names.contains(& "defined".to_string()));
}
