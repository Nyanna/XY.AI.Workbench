# index

The index contract and its implementations. An index is the RAM-resident source of truth for vectors; it reports changes and delivers structure, nothing more.

## Concept

- **Independent of the engine.** An index knows no engine, profile, kernel or geometry. The contract-level header holds model id and dimension (plus the stamp high-water mark and the hash algorithm); the Parquet file header additionally binds model revision, pooling, normalisation, prefixes, storage dtype, maximum sequence length and tokenizer fingerprint, and opening fails on any mismatch. Persistence format and timing belong to the index object, not to the engine.
- **Own model per index.** Whole files, sliding windows, AST nodes, chunks or cascading structures are index decisions. An entry is a vector range of variable length (dense = 1 vector, multi-vector = many); entries may overlap. The Parquet index stores exactly one vector per entry.
- **Stamps.** Each entry carries the value of a global monotonic counter at its last write. Stamps are unique across removal and re-creation of an id and are compared with `!=` only. File-backed indexes without a counter fall back to size+mtime, then to a content hash, treating timestamps within the clock resolution (2 s window) as unsafe; the chain exists as primitives in `stamp.rs`, the shipped indexes draw their stamps from the counter. Stamps live in their own column so a scan never pages in vectors.
- **Change signalling.** A level signal without content (a watch value carrying a watermark). Lost or merged signals are harmless because consumers reconcile by comparing stamps. Binding write order: data, then stamp, then signal.
- **Structure only.** The index can list children of a node, members of a cell and summary entries (for example a centroid table), and declares which access methods it supports. It does not interpret distance, radius, top-k or prefix match and does not resolve scopes; the manager does. Both shipped indexes declare only `Range`: they return no children, treat cell 0 as the list of all entries, offer no summary, and `prefetch` is a no-op.
- **Writers.** Applying a write is serialised and returns the stamp of the write; last writer wins.
- **Lazy and possibly blocking.** Reads may hit page faults or disk; callers use a blocking pool. The Parquet index decodes the whole file on the first vector or payload read and verifies all checksums while opening (default); reads return shared copies of the vector bytes.
- **Source back-reference.** Each entry can point to its source (target file, character offsets, checksum), so an outer layer can fetch the original text, for example for a cross-encoder.

## Files

| File | Role |
|---|---|
| `mod.rs` | Ids, stamp, access methods, scope, entry sets and data, header, the index contract |
| `memory.rs` | Reference in-memory index; builds without `cuda`/`candle` |
| `parquet_io.rs` | Parquet-backed index: stamps scanned without decoding vectors, lazy vector/payload decoding, writes held in memory until `flush` replaces the file atomically |
| `schema.rs` | Arrow schema, footer header keys, schema versioning and compatibility check (model, dimension, pooling, prefixes, dtype, tokenizer fingerprint) |
| `stamp.rs` | Stamp counter and the file-stamp fallback chain |
| `payload.rs` | Source back-reference and checksum |

Changing the model means a new schema version and re-embedding.

Tests: `test/` per file.

## Deviations

- ParquetIndex supports exactly one f16 vector per entry; `apply` panics on any other length. Design: entries of variable length (multi-vector).
- ParquetIndex decodes the whole file into RAM on first `read`/`payload`; `verify` (default true) decodes everything at open. Design: lazy, mmap indexes larger than RAM.
- ParquetIndex::read copies the entry bytes (`Arc::from(slice)`). Design: reference, no copy.
- ParquetIndex does not implement `extent`/`backing` (defaults `None`). The manager's bulk upload and page-cache hints (`GatherTransfer`, `CpuIndexManager::read_entries`) are inactive for it.
- ParquetIndex header binds model revision, pooling, normalize, prefixes, dtype, max_seq_len and tokenizer sha. Design: only model ID and dimension.
- Both indexes declare only `Range`. `children` is empty, `members` treats cell 0 as "all", and `summary` is `None`. No shipped index delivers structure for Centroid, Prefix or Subtree.
- `Index::prefetch` is a no-op in both and is called nowhere in manager. Prefetch hints go through `AdviceSink` instead.
- The stamp fallback chain (`FileStamp`, `Verdict`, `content_hash`) is used by no Index implementation. `RACY_WINDOW_NS` and the hash choice carry a TODO.
