# Models

Each subdirectory is one model checkout (`config.json`, `model.safetensors`, tokenizer files).
Weights are not tracked by git; only this file is.

## Adding a model

1. Place the checkout in `model/<name>/`.
2. Register a `ModelSpec` constructor (dimension, pooling, prefixes, weight paths) in
   `src/resident/model/mod.rs` and map `<name>` to it in `spec_for` in
   `src/resident/model/candle_adapter.rs` (tests).
3. If the model uses query/passage prefixes, add them to `PREFIXES` in
   `scripts/gen_model_reference.py`.
4. Generate the reference embeddings:

   ```
   python3 scripts/gen_model_reference.py model/<name>
   ```

   This writes `tests/data/reference/<name>.json` (PyTorch f32). Commit it.
5. Run `cargo test --features candle installed_models`.

The reference test iterates over every installed model. It fails if no model is installed, if an
installed model has no reference file, or if no `ModelSpec` is registered for it.

## Regenerating

Regenerate the reference of a model whenever its weights, tokenizer, pooling, normalisation,
prefixes or the shared texts in `scripts/reference_texts.json` change. References are test data
only and are never read at runtime.

Requirements: `torch` and `transformers` (run without `python3 -I`). Pooling (mean/cls) and
normalisation are read from the model's sentence-transformers metadata; other pooling modes
require extending the script.
