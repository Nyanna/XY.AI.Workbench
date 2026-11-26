use proc_macro2::{LineColumn, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

pub struct Source<'a> {
    text: &'a str,
    starts: Vec<usize>,
}

impl<'a> Source<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut starts = vec![0];
        starts.extend(text.match_indices('\n').map(|(index, _)| index + 1));
        Self { text, starts }
    }

    pub fn offset(&self, position: LineColumn) -> usize {
        let Some(&start) = self.starts.get(position.line.saturating_sub(1)) else {
            return self.text.len();
        };
        let rest = &self.text[start..];
        let length = rest.find('\n').unwrap_or(rest.len());
        rest[..length]
            .char_indices()
            .nth(position.column)
            .map_or(start + length, |(index, _)| start + index)
    }

    pub fn start_of(&self, span: Span) -> usize {
        self.offset(span.start())
    }

    pub fn end_of(&self, span: Span) -> usize {
        self.offset(span.end())
    }
}

pub fn apply_edits(text: &str, mut edits: Vec<Edit>) -> String {
    edits.sort_by(|a, b| b.start.cmp(&a.start));
    let mut out = text.to_string();
    for edit in edits {
        out.replace_range(edit.start..edit.end, &edit.text);
    }
    out
}

#[cfg(test)]
#[path = "test/edits.rs"]
mod tests;
