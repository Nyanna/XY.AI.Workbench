# model

Model resources of the compute side: description, registry, encoder contract and the loaded encoder as an evictable resource.

## Concept

- **Spec owns nothing.** A model spec is plain data: ids, dimension, pooling, normalisation, query/passage prefixes, storage dtype, compute dtype per device class, maximum sequence length, optional projection head, file paths. Validation rejects configurations that would silently degrade embeddings (for example a prefix without trailing space). A preset exists for `intfloat/multilingual-e5-small`.
- **Registry.** Architecture id → factory. A new model is a registry entry plus a spec.
- **Backbone vs. pipeline.** A backbone only turns tokens into hidden states. The shared pipeline around it is backbone-agnostic: tokens → hidden states → pooling → optional projection → normalisation → write into the destination slot (L2-normalised, converted to the slot dtype). Candle types appear only in `candle_adapter.rs` (feature `candle`).
- **Pooling.** Masked mean gives one vector per sequence (dense); per-token keeps all vectors (late interaction). Host reference versions exist for comparison.
- **Batching.** Token budget instead of item count, length buckets, padding to a common length inside a batch.
- **Loaded encoder** = architecture + weights on one device, a lifecycle resource that can be leased and evicted. Output is written straight into a region; the source tensor must stay alive until the write event.

## Files

| File | Role |
|---|---|
| `mod.rs` | Spec, paths, prefixes, dtypes, tensor reference, region I/O contract, encoder contract, registry, loaded-encoder resource |
| `encoder.rs` | Backbone contract and the backbone-agnostic encode pipeline |
| `candle_adapter.rs` | The only Candle/BERT code; factory for the registry (feature `candle`) |
| `pooling.rs` | Host reference: masked mean pooling, L2 normalisation |
| `head.rs` | Optional linear projection of token embeddings |
| `slot.rs` | Writes encoder output into a slot with normalisation and dtype conversion |
| `batch.rs` | Token-budget batching with length buckets |
| `error.rs` | Model errors |

Tests: `test/` per file with fake backbones.
