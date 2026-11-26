use super::*;

fn texts(source: &str) -> Vec<String> {
    let file = syn::parse_file(source).unwrap();
    file.items.iter().map(|item| item_text(source, item)).collect()
}

#[test]
fn single_line_item_is_returned_verbatim() {
    assert_eq!(texts("struct A;\nfn f() {}\n"), vec!["struct A;", "fn f() {}"]);
}

#[test]
fn multi_line_item_keeps_formatting_and_comments() {
    let source = "fn f() {\n    // keep\n    let x = 1;\n}\n";
    assert_eq!(texts(source), vec!["fn f() {\n    // keep\n    let x = 1;\n}"]);
}

#[test]
fn attributes_and_doc_comments_belong_to_the_item() {
    let source = "/// doc\n#[derive(Debug)]\npub struct A {\n    pub a: u8,\n}\n";
    assert_eq!(
        texts(source),
        vec!["/// doc\n#[derive(Debug)]\npub struct A {\n    pub a: u8,\n}"]
    );
}

#[test]
fn non_ascii_text_is_sliced_on_character_columns() {
    let source = "const A: &str = \"äö\"; const B: u8 = 1;\n";
    assert_eq!(
        texts(source),
        vec!["const A: &str = \"äö\";", "const B: u8 = 1;"]
    );
}
