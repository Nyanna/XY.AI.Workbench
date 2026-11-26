use super::*;

fn collect(source: &str) -> Collected {
    let file = syn::parse_file(source).unwrap();
    let mut out = Collected::default();
    collect_items(Path::new("x.rs"), source, &file.items, &mut out);
    out
}

fn names(collected: &Collected) -> Vec<&str> {
    collected.objects.iter().map(|l| l.object.name.as_str()).collect()
}

#[test]
fn collects_all_addressable_kinds() {
    let collected = collect(
        "struct S; enum E {} union U { a: u8 } trait T {} type A = u8; fn f() {} \
         const C: u8 = 1; static ST: u8 = 1; macro_rules! m { () => {} }",
    );
    let kinds: Vec<ObjectKind> = collected.objects.iter().map(|l| l.object.kind).collect();
    assert_eq!(
        kinds,
        vec![
            ObjectKind::Struct,
            ObjectKind::Enum,
            ObjectKind::Union,
            ObjectKind::Trait,
            ObjectKind::TypeAlias,
            ObjectKind::Function,
            ObjectKind::Const,
            ObjectKind::Static,
            ObjectKind::Macro,
        ]
    );
}

#[test]
fn body_is_the_original_item_text() {
    let collected = collect("/// doc\npub struct S {\n    a: u8,\n}\n");
    assert_eq!(
        collected.objects[0].object.body,
        "/// doc\npub struct S {\n    a: u8,\n}"
    );
}

#[test]
fn location_is_recorded_outside_the_object() {
    let collected = collect("struct S;");
    assert_eq!(collected.objects[0].file, PathBuf::from("x.rs"));
}

#[test]
fn skips_test_only_items_and_modules() {
    let collected = collect(
        "fn kept() {} #[cfg(test)] fn hidden() {} #[test] fn case() {} \
         #[cfg(test)] mod tests { pub fn inner() {} }",
    );
    assert_eq!(names(&collected), vec!["kept"]);
}

#[test]
fn keeps_items_under_cfg_not_test() {
    let collected = collect("#[cfg(not(test))] fn kept() {}");
    assert_eq!(names(&collected), vec!["kept"]);
}

#[test]
fn descends_into_inline_modules_but_not_function_bodies() {
    let collected = collect("mod m { pub struct Inner; } fn f() { struct Local; }");
    assert_eq!(names(&collected), vec!["Inner", "f"]);
}

#[test]
fn ignores_underscore_constants_and_macro_invocations() {
    let collected = collect("const _: () = (); invoke! { x }");
    assert!(collected.objects.is_empty());
}

#[test]
fn dependencies_exclude_own_name() {
    let collected = collect("struct Node { next: Option<Box<Node>>, data: Data }");
    let deps = &collected.objects[0].object.dependencies;
    assert!(deps.contains(&"Data".to_string()));
    assert!(!deps.contains(&"Node".to_string()));
}

#[test]
fn inherent_impl_records_target_without_trait() {
    let collected = collect("impl Target { fn make() -> Output { todo!() } }");
    assert!(collected.objects.is_empty());
    let block = &collected.impls[0];
    assert_eq!(block.self_name.as_deref(), Some("Target"));
    assert_eq!(block.trait_name, None);
    assert!(block.names.contains(&"Output".to_string()));
    assert!(block.body.starts_with("impl Target"));
}

#[test]
fn trait_impl_records_trait_and_target() {
    let collected = collect("impl Contract for &Target {}");
    let block = &collected.impls[0];
    assert_eq!(block.self_name.as_deref(), Some("Target"));
    assert_eq!(block.trait_name.as_deref(), Some("Contract"));
}

#[test]
fn generic_parameter_is_not_an_impl_target() {
    let collected = collect("impl<T: Loader> Describe for T {}");
    let block = &collected.impls[0];
    assert_eq!(block.self_name, None);
    assert_eq!(block.trait_name.as_deref(), Some("Describe"));
}
