use std::borrow::Cow;

/// Optional text rewrite applied before prefixing and tokenization. Normalization inside the
/// tokenizer (`tokenizer.json`) is never duplicated here.
pub trait TextNormalizer: Send + Sync {
    fn normalize<'a>(&self, text: &'a str) -> Cow<'a, str>;
}

/// Leaves the text untouched.
#[derive(Clone, Copy, Debug, Default)]
pub struct Identity;

impl TextNormalizer for Identity {
    fn normalize<'a>(&self, text: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(text)
    }
}
