use super::*;

fn edit(start: usize, end: usize, text: &str) -> Edit {
    Edit {
        start,
        end,
        text: text.into(),
    }
}

#[test]
fn offsets_follow_lines_and_columns() {
    let source = Source::new("ab\ncd\n");
    assert_eq!(source.offset(LineColumn { line: 1, column: 1 }), 1);
    assert_eq!(source.offset(LineColumn { line: 2, column: 0 }), 3);
    assert_eq!(source.offset(LineColumn { line: 2, column: 2 }), 5);
}

#[test]
fn columns_count_characters_not_bytes() {
    let source = Source::new("\u{e9}x;");
    assert_eq!(source.offset(LineColumn { line: 1, column: 1 }), 2);
}

#[test]
fn positions_past_the_end_are_clamped() {
    let source = Source::new("ab");
    assert_eq!(source.offset(LineColumn { line: 9, column: 0 }), 2);
    assert_eq!(source.offset(LineColumn { line: 1, column: 9 }), 2);
}

#[test]
fn edits_apply_independently_of_their_order() {
    let edits = vec![edit(0, 0, "X"), edit(2, 3, "Y")];
    assert_eq!(apply_edits("abc", edits), "XabY");
}

#[test]
fn no_edits_keep_the_text() {
    assert_eq!(apply_edits("abc", Vec::new()), "abc");
}
