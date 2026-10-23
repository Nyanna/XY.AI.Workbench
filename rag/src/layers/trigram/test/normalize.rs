use super::*;
#[test]
fn camel_case_and_lowercase() {
    assert_eq!(normalize("getUserName"), "get user name");
    assert_eq!(normalize("HTTPServer"), "http server");
    assert_eq!(normalize("foo2Bar"), "foo2 bar");
}
#[test]
fn umlauts_and_sharp_s() {
    assert_eq!(normalize("äöü"), "aou");
    assert_eq!(normalize("Straße"), "strasse");
}
#[test]
fn syntax_becomes_space_and_collapses() {
    assert_eq!(normalize("foo_bar.baz()"), "foo bar baz");
    assert_eq!(normalize("a   b\t\tc"), "a b c");
}
#[test]
fn newlines_and_blank_lines_preserved() {
    let out = normalize("Foo\n   \nBar");
    assert_eq!(out, "foo\n\nbar");
    assert_eq!(out.split('\n').count(), 3);
}
#[test]
fn line_count_is_stable() {
    let input = "line1\nline2\nline3\n";
    let out = normalize(input);
    assert_eq!(input.split('\n').count(), out.split('\n').count());
}
#[test]
fn binary_is_rejected() {
    assert_eq!(normalize_bytes(b"ab\0cd"), Err(NormalizeError::Binary));
    assert!(normalize_bytes(b"hello").is_ok());
}
