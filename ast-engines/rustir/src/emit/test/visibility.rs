use super::*;
use crate::emit::edits::apply_edits;
fn expose(source: &str, exposure: Exposure) -> String {
    let file = syn::parse_file(source).unwrap();
    apply_edits(source, visibility_edits(&file, &Source::new(source), exposure))
}
fn crate_wide(source: &str) -> String {
    expose(source, Exposure::Crate)
}
#[test]
fn private_items_become_crate_visible() {
    assert_eq!(crate_wide("fn f() {}"), "pub(crate) fn f() {}");
    assert_eq!(crate_wide("enum E { A }"), "pub(crate) enum E { A }");
    assert_eq!(crate_wide("type T = u8;"), "pub(crate) type T = u8;");
    assert_eq!(crate_wide("const C: u8 = 1;"), "pub(crate) const C: u8 = 1;");
    assert_eq!(crate_wide("static S: u8 = 1;"), "pub(crate) static S: u8 = 1;");
}
#[test]
fn modifiers_stay_behind_the_visibility() {
    assert_eq!(crate_wide("async fn f() {}"), "pub(crate) async fn f() {}");
    assert_eq!(crate_wide("unsafe trait T {}"), "pub(crate) unsafe trait T {}");
    assert_eq!(
        crate_wide("union U { a: u8 }"), "pub(crate) union U { pub(crate) a: u8 }"
    );
}
#[test]
fn attributes_stay_in_front_of_the_visibility() {
    assert_eq!(
        crate_wide("#[derive(Debug)]\nstruct S { #[x] a: u8 }"),
        "#[derive(Debug)]\npub(crate) struct S { #[x] pub(crate) a: u8 }"
    );
}
#[test]
fn public_items_are_kept() {
    assert_eq!(crate_wide("pub fn f() {}"), "pub fn f() {}");
    assert_eq!(crate_wide("pub(crate) fn f() {}"), "pub(crate) fn f() {}");
}
#[test]
fn restricted_visibility_is_widened_to_the_crate() {
    assert_eq!(crate_wide("pub(super) fn f() {}"), "pub(crate) fn f() {}");
    assert_eq!(crate_wide("pub(in crate::a) fn f() {}"), "pub(crate) fn f() {}");
    assert_eq!(crate_wide("pub(self) fn f() {}"), "pub(crate) fn f() {}");
}
#[test]
fn tuple_fields_are_widened() {
    assert_eq!(
        crate_wide("struct S(u8, pub u16);"),
        "pub(crate) struct S(pub(crate) u8, pub u16);"
    );
}
#[test]
fn inherent_members_are_widened_but_trait_impls_are_not() {
    let source = "struct S; impl S { fn a() {} pub fn b() {} const C: u8 = 1; }\nimpl Tr for S { fn a() {} }";
    assert_eq!(
        crate_wide(source),
        "pub(crate) struct S; impl S { pub(crate) fn a() {} pub fn b() {} pub(crate) const C: u8 = 1; }\nimpl Tr for S { fn a() {} }"
    );
}
#[test]
fn public_exposure_replaces_crate_visibility() {
    assert_eq!(expose("fn main() {}", Exposure::Public), "pub fn main() {}");
    assert_eq!(expose("pub(crate) fn main() {}", Exposure::Public), "pub fn main() {}");
}
#[test]
fn keep_changes_nothing() {
    assert_eq!(expose("struct S { a: u8 }", Exposure::Keep), "struct S { a: u8 }");
}
#[test]
fn only_the_first_item_is_exposed_but_every_impl_is_widened() {
    let source = "struct S; impl S { fn a() {} }\nimpl S { fn b() {} }";
    assert_eq!(
        crate_wide(source),
        "pub(crate) struct S; impl S { pub(crate) fn a() {} }\nimpl S { pub(crate) fn b() {} }"
    );
}
#[test]
fn macros_have_no_visibility() {
    assert_eq!(crate_wide("macro_rules! m { () => {} }"), "macro_rules! m { () => {} }");
}
#[test]
fn publicity_is_read_from_the_first_item() {
    let parse = |text: &str| syn::parse_file(text).unwrap();
    assert!(is_public(& parse("pub struct S;")));
    assert!(! is_public(& parse("struct S;")));
    assert!(! is_public(& parse("pub(crate) struct S;")));
    assert!(! is_public(& parse("macro_rules! m { () => {} }")));
}
