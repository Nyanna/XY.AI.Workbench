use crate::resident::model::Prefixes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeMode {
    Query,
    Passage,
}

/// Late-interaction conventions. Marker text and lengths differ per checkpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColbertScheme {
    pub query_marker: String,
    pub doc_marker: String,
    pub query_maxlen: usize,
    pub doc_maxlen: usize,
    /// Queries are padded with this id up to `query_maxlen`; the padding attends.
    pub mask_token_id: u32,
}

/// How a mode alters the text before and the ids after tokenization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModeScheme {
    /// Plain-text prefixes in front of the input (e5).
    Prefix(Prefixes),
    Colbert(ColbertScheme),
}

impl ModeScheme {
    pub(super) fn prepare(&self, mode: EncodeMode, text: &str) -> String {
        let lead = match (self, mode) {
            (Self::Prefix(p), EncodeMode::Query) => &p.query,
            (Self::Prefix(p), EncodeMode::Passage) => &p.passage,
            (Self::Colbert(c), EncodeMode::Query) => &c.query_marker,
            (Self::Colbert(c), EncodeMode::Passage) => &c.doc_marker,
        };
        let mut out = String::with_capacity(lead.len() + text.len());
        out.push_str(lead);
        out.push_str(text);
        out
    }

    pub(super) fn finish(&self, mode: EncodeMode, mut ids: Vec<u32>) -> Vec<u32> {
        if let Self::Colbert(c) = self {
            match mode {
                EncodeMode::Query => {
                    ids.truncate(c.query_maxlen);
                    ids.resize(c.query_maxlen, c.mask_token_id);
                }
                EncodeMode::Passage => ids.truncate(c.doc_maxlen),
            }
        }
        ids
    }
}
