# index

The index contract and its implementations. An index is the RAM-resident source of truth for vectors; it reports changes and delivers structure, nothing more.

## Concept

- **Independent of the engine.** An index knows no engine, profile, kernel or geometry. The only hard binding to a model is model id and dimension in its header. Persistence format and timing belong to the index object, not to the engine.
- **Own model per index.** Whole files, sliding windows, AST nodes, chunks or cascading structures are index decisions. An entry is a vector range of variable length (dense = 1 vector, multi-vector = many); entries may overlap.
- **Stamps.** Each entry carries the value of a global monotonic counter at its last write. Stamps are unique across removal and re-creation of an id and are compared with `!=` only. File-backed indexes without a counter fall back to size+mtime, then to a content hash, treating timestamps within the clock resolution as unsafe. Stamps live in their own column so a scan never pages in vectors.
- **Change signalling.** A level signal without content (a watch value carrying a watermark). Lost or merged signals are harmless because consumers reconcile by comparing stamps. Binding write order: data, then stamp, then signal.
- **Structure only.** The index can list children of a node, members of a cell and summary entries (for example a centroid table), and declares which access methods it supports. It does not interpret distance, radius, top-k or prefix match and does not resolve scopes; the manager does.
- **Writers.** Applying a write is serialised and returns the stamp of the write; last writer wins.
- **Lazy and possibly blocking.** Reads may hit page faults or disk; callers use a blocking pool.
- **Source back-reference.** Each entry can point to its source (target file, character offsets, checksum), so an outer layer can fetch the original text, for example for a cross-encoder.

## Files

| File | Role |
|---|---|
| `mod.rs` | Ids, stamp, access methods, scope, entry sets and data, header, the index contract |
| `memory.rs` | Reference in-memory index; builds without `cuda`/`candle` |
| `parquet_io.rs` | Parquet-backed index: stamps scanned without decoding vectors, lazy vector/payload decoding, atomic file replacement on every write |
| `schema.rs` | Arrow schema, footer header keys, schema versioning and compatibility check (model, dimension, pooling, prefixes, dtype, tokenizer fingerprint) |
| `stamp.rs` | Stamp counter and the file-stamp fallback chain |
| `payload.rs` | Source back-reference and checksum |

Changing the model means a new schema version and re-embedding.

Tests: `test/` per file.
