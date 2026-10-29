# tokenize

The one shared CPU tokenizer. Token ids reach the device only as `u32` encoder input.

## Concept

- **Single instance, CPU only.** It is a CPU resource (keyed like other lifecycle resources) built from a model spec. There is deliberately no device-side tokenizer and no id-to-address gather.
- **Mode scheme.** Text is encoded as query or passage. A scheme decides what that means: textual prefixes for dense models, or marker tokens with fixed-length mask padding (queries) and truncation to the document maximum (documents) for ColBERT-style models.
- **Truncation yes, padding no.** Sequences are truncated to the maximum length (closing special token kept). Padding to a common length belongs to the batching in `model`; the tokenizer can hand its sequences to that batching.
- **Normalisation is pluggable.** A text-normaliser hook (default: identity) runs before tokenisation, always on the CPU.

## Files

| File | Role |
|---|---|
| `mod.rs` | The shared tokenizer: loading from a model spec, encoding, batch production |
| `modes.rs` | Encode mode and mode schemes (prefix, ColBERT-style) |
| `normalize.rs` | Normaliser hook and identity default |
| `error.rs` | Tokenizer errors |

Tests: `test/mod.rs`.
