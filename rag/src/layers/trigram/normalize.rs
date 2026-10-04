//! Text normalisation (spec part A, section 4).
//!
//! Produces a line-faithful, normalised copy of a text: Unicode NFKD with
//! combining marks dropped, every non-ASCII-alphanumeric reduced to a space
//! (letters of other languages transliterated where NFKD yields ASCII, plus
//! an explicit `ß -> ss`), CamelCase split then lowercased, and whitespace
//! runs collapsed to a single space. Newlines are preserved so line N of the
//! normalised text maps to line N of the original (decisions 9.3, 9.4, 9.7).
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;
/// Error returned for inputs that must not be indexed (decision 9.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizeError {
    /// NUL byte or invalid UTF-8 - treated as a binary file.
    Binary,
}
/// Normalises already-decoded text. Callers detect binary content (NUL byte
/// or invalid UTF-8) before decoding; see [`normalize_bytes`].
pub fn normalize(input: &str) -> String {
    let mut reduced: Vec<char> = Vec::with_capacity(input.len());
    for ch in input.nfkd() {
        if is_combining_mark(ch) {
            continue;
        }
        match ch {
            '\n' => reduced.push('\n'),
            c if c.is_ascii_alphanumeric() => reduced.push(c),
            'ß' => {
                reduced.push('s');
                reduced.push('s');
            }
            _ => reduced.push(' '),
        }
    }
    let split = split_camel_case(&reduced);
    let mut out = String::with_capacity(split.len());
    let mut first_line = true;
    for line in split.split('\n') {
        if !first_line {
            out.push('\n');
        }
        first_line = false;
        let mut first_tok = true;
        for tok in line.split(' ').filter(|t| !t.is_empty()) {
            if !first_tok {
                out.push(' ');
            }
            first_tok = false;
            out.push_str(tok);
        }
    }
    out
}
/// Decodes raw file bytes and normalises them, rejecting binary content.
pub fn normalize_bytes(bytes: &[u8]) -> Result<String, NormalizeError> {
    if bytes.contains(&0) {
        return Err(NormalizeError::Binary);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| NormalizeError::Binary)?;
    Ok(normalize(text))
}
/// Splits CamelCase boundaries and lowercases the result (decision 9.4):
/// a space is inserted at `[a-z0-9][A-Z]` and at `[A-Z][A-Z][a-z]`.
fn split_camel_case(chars: &[char]) -> String {
    let mut out = String::with_capacity(chars.len() + chars.len() / 4);
    for i in 0..chars.len() {
        let cur = chars[i];
        if i > 0 {
            let prev = chars[i - 1];
            let next = chars.get(i + 1).copied();
            let lower_or_digit = prev.is_ascii_lowercase() || prev.is_ascii_digit();
            let r1 = lower_or_digit && cur.is_ascii_uppercase();
            let r2 = prev.is_ascii_uppercase() && cur.is_ascii_uppercase()
                && next.is_some_and(|n| n.is_ascii_lowercase());
            if r1 || r2 {
                out.push(' ');
            }
        }
        out.extend(cur.to_lowercase());
    }
    out
}
#[cfg(test)]
mod tests {
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
}
