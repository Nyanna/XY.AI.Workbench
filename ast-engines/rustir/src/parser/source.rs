use proc_macro2::LineColumn;
use syn::spanned::Spanned;
use syn::Item;

pub fn item_text(source: &str, item: &Item) -> String {
    let span = item.span();
    slice_text(source, span.start(), span.end())
}

fn slice_text(source: &str, start: LineColumn, end: LineColumn) -> String {
    let from = byte_offset(source, start);
    let to = byte_offset(source, end).max(from);
    source[from..to].to_string()
}

fn byte_offset(source: &str, position: LineColumn) -> usize {
    let mut line_start = 0;
    for _ in 1..position.line {
        match source[line_start..].find('\n') {
            Some(index) => line_start += index + 1,
            None => return source.len(),
        }
    }
    let rest = &source[line_start..];
    let line_end = rest.find('\n').unwrap_or(rest.len());
    rest[..line_end]
        .char_indices()
        .nth(position.column)
        .map_or(line_start + line_end, |(index, _)| line_start + index)
}

#[cfg(test)]
#[path = "test/source.rs"]
mod tests;
