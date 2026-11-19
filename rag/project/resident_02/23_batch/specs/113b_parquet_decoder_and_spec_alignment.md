# Spec: Parquet Decoder and Spec Alignment (modules `meta_codec` and `parquet_decoder`, build order 113b)

## 1. Task, restrictions and boundaries

**Task.** Two small modules complete the file-backed index chain:
- `meta_codec` defines the byte encoding of the entry payload (`meta` bytes of `index_contract::Payload`) from the three source columns `target_file`, `char_start` and `char_end`.
- `parquet_decoder` implements `index_open::FormatDecoder` for the Parquet index file defined by `index_format`. It is the single place that parses the footer header, checks the column schema, checks the model binding, and decodes ids, stamps, vectors and payloads.

The spec also contains the exact amendments to the other specs (§6) so that the chain 047, 113, 113b, 121, 122, 129 and the consumers 058 and 137 is consistent, and it withdraws the superseded specs 048 and 049.

**Not part of the modules.**
- Writing files. That belongs to `index_save` (129).
- Opening, mapping, lazy batching, stamp counter and duplicate-id rejection. That belongs to `index_open` (047). The decoder neither sorts nor deduplicates ids.
- The header, the column schema and the binding rules themselves. They belong to `index_format` (113). The decoder only calls them.
- The checksum algorithm. It belongs to `entry_payload` (121) and is injected as a function (`ChecksumFn`), because `entry_payload` is built after this layer and cannot be imported here.
- Entry management, overlay and dirty state (122), and the adapter that turns an entry store into a snapshot source (129). Their additions are specified in 6.5 and 6.6.
- Any migration of files in the withdrawn format. There is none (see 4.9).

**Restrictions.**
- No `unsafe`. No new crates. Allowed: `parquet`, `arrow`, `half`, `thiserror`, `std`, and `Bytes` as already re-exported through `tokio_util::bytes` in the existing code.
- Internal modules: `index_format`, `index_open` (only `FileInfo`, `DecodedBatch`, `FormatDecoder`) and `index_contract` (only `SharedBytes`).
- The decoder never copies the whole file. It copies at most the column chunks of the row groups it needs, plus the footer (see 4.1).
- All functions are free of global state. Comments in the code are minimal and carry no plan or spec numbers.
- Placement: `l080/a/meta_codec.rs` and `l080/a/parquet_decoder.rs`, registered in `l080/a/mod.rs`. Tests are in `l080/a/test/meta_codec.rs` and `l080/a/test/parquet_decoder.rs`. Each source file ends with:
  ```rust
  #[cfg(test)]
  #[path = "test/<name>.rs"]
  mod tests;
  ```

## 2. Context

- `index_open::LazyIndexFile` gets its file knowledge only from an injected `FormatDecoder` and hands it the complete file bytes (mmap or read). It calls `read_info`, then `read_ids_stamps`, then optionally `verify`, and later `decode_rows` per batch of 4096 rows.
- `index_format` defines `IndexHeader`, `from_kv`, `check_binding` and `check_column_schema`. No module calls them yet. The decoder is their caller. It holds the expected header (derived from the current model) and therefore performs the full binding check (model id, dimension and all pipeline features) when the file is opened.
- `index_open::FileInfo` carries only `model_id` and `model_revision`. The full header stays inside the decoder.
- The old Parquet format (`id`, `stamp`, `vector` as Binary, `meta` as Binary, footer keys `index.*`) is withdrawn. A file in that format fails at `read_info` with a header error (4.9).
- `Payload.meta` is a byte string. The index chain needs it to carry the source reference of an entry, because the file stores the source in three columns and the checksum covers it. This spec fixes the encoding so that writer (129), reader (this module), entry store (122) and checksum provider agree.

## 3. Interface

```rust
// ---------- meta_codec ----------
// no internal dependency

/// Field-for-field equal to entry_payload::SourceRef. It exists because that module is built later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MetaFields {
    pub target_file: String,
    pub char_start: u64,
    pub char_end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MetaError {
    #[error("meta truncated: need {needed} bytes, have {have}")]
    Truncated { needed: u64, have: usize },
    #[error("meta has {extra} trailing bytes")]
    Trailing { extra: usize },
    #[error("meta target_file is not valid UTF-8")]
    Utf8,
}

/// Canonical encoding. Empty vector for the default fields.
pub fn encode_meta(fields: &MetaFields) -> Vec<u8>;
pub fn encode_meta_parts(target_file: &str, char_start: u64, char_end: u64) -> Vec<u8>;
pub fn decode_meta(bytes: &[u8]) -> Result<MetaFields, MetaError>;

// ---------- parquet_decoder ----------
use std::sync::Arc;
use index_format::{IndexHeader, FormatError};
use index_open::{FileInfo, DecodedBatch, FormatDecoder};

pub const READ_BATCH_ROWS: usize = 4096;

/// Same signature as entry_payload::compute_checksum_parts(vector, target_file, char_start, char_end).
pub type ChecksumFn = Arc<dyn Fn(&[u8], &str, u64, u64) -> u64 + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    /// Not a readable Parquet file (magic, footer, metadata, truncation).
    #[error("unreadable file: {0}")]
    Footer(String),
    /// Header, column schema or model binding. Display is the FormatError text unchanged.
    #[error("{0}")]
    Format(FormatError),
    /// Inconsistent data (row counts, nulls, ranges, vector length, ordering, stamps).
    #[error("invalid data: {0}")]
    Data(String),
    #[error("checksum mismatch at row {row} (entry {entry_id}): stored {stored:016x}, computed {computed:016x}")]
    Checksum { row: u64, entry_id: u64, stored: u64, computed: u64 },
}

/// Send + Sync. Immutable after construction. All methods take &self.
pub struct ParquetDecoder { /* expected: IndexHeader, checksum: ChecksumFn */ }

impl ParquetDecoder {
    /// `expected` is the header derived from the current model; its max_stamp is ignored.
    pub fn new(expected: IndexHeader, checksum: ChecksumFn) -> ParquetDecoder;
    pub fn expected(&self) -> &IndexHeader;

    /// Footer header and column schema, without the binding check. For tools and diagnostics.
    pub fn read_header(&self, bytes: &[u8]) -> Result<IndexHeader, DecodeError>;

    /// (entry_id, checksum) of all rows in file order. Reads only these two columns.
    /// No binding check and no checksum verification.
    pub fn read_stored_checksums(&self, bytes: &[u8]) -> Result<Vec<(u64, u64)>, DecodeError>;
}

impl FormatDecoder for ParquetDecoder {
    fn read_info(&self, bytes: &[u8]) -> Result<FileInfo, String>;
    fn read_ids_stamps(&self, bytes: &[u8], info: &FileInfo) -> Result<Vec<(u64, u64)>, String>;
    fn decode_rows(&self, bytes: &[u8], info: &FileInfo, start: u64, end: u64) -> Result<DecodedBatch, String>;
    fn verify(&self, bytes: &[u8], info: &FileInfo) -> Result<(), String>;
}
```

All error strings of the trait methods are the `Display` text of a `DecodeError`.

## 4. Behavior and invariants

### 4.1 Reading the bytes
- `bytes` is the complete file. The decoder reads the footer and metadata through a borrowing adapter over the slice (`ChunkReader` + `Length`). The adapter copies only the requested ranges.
- For data reads the decoder determines, from the metadata, the byte ranges of the needed column chunks (`ColumnChunkMetaData::byte_range`, dictionary pages included) of the needed row groups. It copies exactly these ranges into an owned, `'static` range source and hands that to the Parquet reader. A request outside the copied ranges is a `Data` error and never a read of other bytes.
- A call never copies a column chunk it does not need. `read_ids_stamps` copies no vector chunk. `decode_rows` copies the chunks of `vector`, `target_file`, `char_start` and `char_end` of the overlapping row groups only.
- If the used crate version requires a different mechanism, the invariants above still hold: no `unsafe`, no whole-file copy, bounded copies by column chunk.
- Bytes that are too short, have no Parquet magic or a damaged footer give `Footer`. No input makes any method panic.
- Any compression codec supported by the enabled `parquet` features is accepted. Page index and bloom filters are not used.

### 4.2 `read_header`
1. Load the metadata. Failure gives `Footer`.
2. Collect the footer key-value pairs that have a value. Call `IndexHeader::from_kv`. An error gives `Format` with all collected mismatches. Extra keys (for example `ARROW:schema`) are ignored by `from_kv`.
3. Take the Arrow schema of the file and call `index_format::check_column_schema(&schema, header.dimension)`. An error gives `Format`.
4. Return the header, including the file's `max_stamp`.

The order is fixed: footer, header, schema. The first failing step returns its error. Within a step all mismatches are reported together.

### 4.3 `read_info`
1. `read_header` as in 4.2.
2. `header.check_binding(self.expected())`. An error gives `Format`. The binding rules are those of `index_format`: model id and dimension are the hard binding, the other pipeline features must be equal, `max_stamp` and `compute_dtype` never fail.
3. `FileInfo`:
   - `schema_version = u32::try_from(header.schema_version)`. A value above `u32::MAX` gives `Data`.
   - `model_id`, `model_revision` from the header.
   - `row_groups` = number of row groups.
   - `rows` = sum of the row-group row counts. A negative count, or a sum different from the file-level row count, gives `Data`.
   - `stored_max_stamp = Some(header.max_stamp)`. The value 0 means empty and behaves like `None` in `StampCounter::resume`.

A file written for another model, another revision, another tokenizer fingerprint, other prefixes or another preprocessing id is rejected here. `LazyIndexFile` stores the failure and does not retry.

### 4.4 `read_ids_stamps`
- Reads the root columns `entry_id` and `change_stamp` of all row groups. Returns `(entry_id, change_stamp)` in file order, not sorted.
- The count must equal `info.rows`. A null value or a missing column gives `Data`.
- No check of stamp or id values is made here. That is the job of `verify`.

### 4.5 `decode_rows(start, end)`
- `start <= end <= info.rows` is required, else `Data`. An empty range returns an empty batch without reading.
- Reads the row groups that overlap `[start, end)`, the columns `vector`, `target_file`, `char_start`, `char_end`, in batches of `READ_BATCH_ROWS`, and slices to the range.
- `vectors[i]`: the FP16 elements of the fixed-size list as little-endian bytes. Length is exactly `expected().vector_bytes()` (dimension × 2). The bits are copied unchanged (no conversion, no normalisation, NaN bits preserved).
- `metas[i]`: `encode_meta_parts(target_file, char_start, char_end)`.
- Both vectors have exactly `end - start` elements. A null value, a wrong list size or a length mismatch gives `Data`.
- Checksums are not verified here. Verification is the explicit `verify` call, or the entry store's own check (122).
- Memory is bounded by the row groups touched by the range.

### 4.6 `verify`
Reads all row groups in file order, one at a time (no whole-file decode held in memory), and returns the first error:
1. Row count equals `info.rows`, else `Data`.
2. No null in any column, else `Data`.
3. Vector length is `vector_bytes()`, else `Data`.
4. `entry_id` is strictly ascending over the whole file, else `Data` (the writer guarantees sorted, unique ids).
5. `1 <= change_stamp <= header.max_stamp`, else `Data`. `Stamp(0)` is never assigned, and `max_stamp` covers every stamp ever assigned, deletes included.
6. `checksum(vector, target_file, char_start, char_end) == stored checksum`, else `Checksum { row, entry_id, stored, computed }`.

### 4.7 `read_stored_checksums`
Runs 4.2 (header and schema, no binding), then reads `entry_id` and `checksum` of all rows. Row count is not compared to anything else. Nulls give `Data`.

### 4.8 `meta_codec`
- Canonical encoding of `(target_file, char_start, char_end)`:
  - The default value (`""`, 0, 0) encodes to the **empty** byte string. This is the payload of new entries (122: "for new entries it is empty").
  - Every other value encodes to: `u64` little-endian byte length of `target_file`, the UTF-8 bytes of `target_file`, `char_start` as `u64` little-endian, `char_end` as `u64` little-endian. Length is `24 + len(target_file)`.
  - The field order and widths equal the checksum input of `entry_payload` after the vector. The file name is not trimmed or normalised. `char_end < char_start` is valid.
- `decode_meta`:
  - Empty input gives the default.
  - Otherwise the layout above is required. A declared file-name length larger than the remaining bytes (compared without overflow) or fewer than 16 bytes after the name gives `Truncated`. Extra bytes give `Trailing`. A name that is not UTF-8 gives `Utf8`.
  - The 24-byte encoding of the default value is accepted and decodes to the default. `encode_meta` never produces it.
- `decode_meta(encode_meta(f)) == Ok(f)` for every `MetaFields`. `encode_meta(decode_meta(b)?)` equals `b` for every `b` that `encode_meta` can produce.
- Pure functions, any thread, no panic on any input.

### 4.9 Files in the withdrawn format
A file whose footer has the keys `index.model_id`, `index.dimension`, `index.dtype`, `index.max_stamp` and the columns `id`, `stamp`, `vector` (Binary), `meta` fails in `read_info` with `Format(SchemaIncompatible)` that lists every header key as `<fehlt>`. The decoder has no read path for it and no conversion. No production writer of this format exists, so no migration is needed.

### 4.10 Concurrency and determinism
- `ParquetDecoder` is `Send + Sync`. Calls are independent and may run in parallel, also on the same bytes.
- Equal bytes and equal expected header always give equal results.
- The decoder logs nothing. Logging is done by `index_open`.

### 4.11 Required tests
Tests build files with a test-local writer (Arrow `ArrowWriter`, `header.to_kv()` as footer, `column_schema(dim)`), because the production writer is built later.
- `meta_codec`: default encodes to empty; round trip with empty name, non-empty name, Unicode, embedded NUL, `char_end < char_start`; truncated at every length; trailing bytes; invalid UTF-8; huge declared length without overflow; 24-byte zero form decodes to default.
- Header: round trip with all 16 keys; missing key reports `<fehlt>`; extra key ignored; wrong dimension in the schema list size fails `Format`; wrong column name, type or nullability fails `Format`.
- Binding: each bound key differs in turn and yields exactly that mismatch; `max_stamp` and `compute_dtype` differences pass; several differences are reported together.
- `read_info`: counts, `stored_max_stamp = Some(max)`, zero rows, several row groups, `schema_version` conversion.
- `read_ids_stamps` on a file with unsorted ids returns file order. A spy `ChunkReader` or byte counter proves that the vector chunk is not copied.
- `decode_rows`: range inside one row group, across groups, at both ends, empty range, `end > rows` error; bit-exact FP16 including NaN, -0.0, subnormals; `metas` equal `encode_meta_parts`.
- `verify`: valid file passes; a flipped vector byte, a changed `char_end`, a changed stored checksum each give `Checksum` with the right row and entry; unsorted or duplicate ids, stamp 0, stamp above `max_stamp`, wrong row count each give `Data`.
- Robustness: every prefix of a valid file (all lengths up to a bounded sample) and files with flipped footer bytes never panic and give `Footer`, `Format` or `Data`.
- Integration with `LazyIndexFile`: open a decoded file with `verify` true and false; wrong model fails open with the binding text; `read`, `read_vector`, `read_meta` return the expected bytes; stamp counter resumes above `max(max_stamp, highest stamp)`.
- Legacy file (4.9) is rejected with all keys `<fehlt>`.
- `ParquetDecoder: Send + Sync` by static assertion.

## 5. Relied-on elements

```rust
// index_format (verbatim, used)
impl IndexHeader {
    pub fn from_kv(pairs: &[(String, String)]) -> Result<IndexHeader, FormatError>;
    pub fn check_binding(&self, expected: &IndexHeader) -> Result<(), FormatError>;
    pub fn vector_bytes(&self) -> usize;               // dimension * 2
    // fields used: schema_version: u64, max_stamp: u64, model_id, model_revision, dimension: usize
}
pub fn check_column_schema(found: &Schema, dimension: usize) -> Result<(), FormatError>;
pub const COL_ENTRY_ID: &str = "entry_id";   // also COL_CHANGE_STAMP, COL_VECTOR, COL_TARGET_FILE,
                                             // COL_CHAR_START, COL_CHAR_END, COL_CHECKSUM
// Columns, all non-null: entry_id UInt64, change_stamp UInt64, vector FixedSizeList(Float16, dim),
// target_file Utf8, char_start UInt64, char_end UInt64, checksum UInt64.

// index_open (verbatim, used)
pub struct FileInfo { pub schema_version: u32, pub model_id: String, pub model_revision: String,
                      pub row_groups: usize, pub rows: u64, pub stored_max_stamp: Option<u64> }
pub struct DecodedBatch { pub vectors: Vec<SharedBytes>, pub metas: Vec<SharedBytes> }
pub trait FormatDecoder: Send + Sync { /* read_info, read_ids_stamps, decode_rows, verify as in §3 */ }

// index_contract
pub struct SharedBytes(pub Arc<[u8]>);

// change_stamp
StampCounter::resume(stored_max: Option<u64>, highest_present: Option<u64>)  // None behaves like Some(0)
```

- Parquet and Arrow: the existing `parquet` and `arrow` dependencies. `ArrowReaderMetadata::load`, `ParquetRecordBatchReaderBuilder` with projection by root column and row-group selection, and `ChunkReader` / `Length` are the used APIs. `half::f16` serves the byte conversion.
- `entry_payload::compute_checksum_parts` is the function that is passed as `ChecksumFn` by the assembling code. This module never imports `entry_payload`.

## 6. Amendments to other specs

Each item replaces or adds exactly the text named. Items marked *consequence* follow from the F16-only vector format that 113 and 122 already fix.

### 6.1 Withdrawn specs
- **048 (file-backed index, module `file_index`): withdrawn.** Its tasks are taken over: opening, lazy loading, stamps, ids: 047; header and schema: 113; reading and binding: 113b; payload and checksum: 121; in-memory changes, signal, `Index` implementation: 122; atomic saving: 129. The module `file_index.rs`, its test file and its `mod` line are removed at the cutover (6.10).
- **049 (index, module `index`): withdrawn.** No spec and no code uses it. It defines a tree-based index with its own `EntryId`/`Stamp` aliases and `IndexError`, which collide with `index_contract`. Its `IndexHeader` (with `producing_profile`, without prefixes and sequence length) and its statement that prefixes and `max_seq_len` do not invalidate an index are void. 113 is the only header and binding definition. `index.rs` and `test/index.rs` and the `mod` line are removed at the cutover.

### 6.2 047 (index open)
- §1, "Not part of the module", first bullet, append: "The production decoder is `parquet_decoder::ParquetDecoder`."
- §4 "Open", append: "The decoder performs the model binding in `read_info`. A binding or format error is an `OpenError::Unavailable` with the decoder's text and is remembered. A `LazyIndexFile` is created only for a path that exists. For a missing path the caller builds an index without file (122)."
- No change to the interface or to `index_open.rs`.

### 6.3 113 (index format)
- §2 append: "`from_kv`, `check_binding` and `check_column_schema` are called by `parquet_decoder`. The writer (129) calls `to_kv` and `column_schema`."
- No change to interface or behavior. `Pooling::Cls` stays rejected, in line with the pooling, model registry and encoder specs.

### 6.4 121 (payload and checksum)
- §3 append to `SourceRef`:
  ```rust
  impl From<meta_codec::MetaFields> for SourceRef {}
  impl From<&SourceRef> for meta_codec::MetaFields {}
  impl SourceRef {
      /// meta_codec::encode_meta of this source.
      pub fn to_meta(&self) -> Vec<u8>;
      pub fn from_meta(bytes: &[u8]) -> Result<SourceRef, meta_codec::MetaError>;
  }
  ```
- §1 internal modules: add `meta_codec`. §4: `to_meta` and `from_meta` are exactly `encode_meta` and `decode_meta` on the three fields. `SourceRef::default()` has empty meta.

### 6.5 122 (entry store)
- §1 internal modules: add `meta_codec`, `parquet_decoder`, `entry_payload`.
- `ChecksumProvider::checksum` becomes fallible:
  ```rust
  fn checksum(&self, id: EntryId, vector: &[u8], meta: &[u8]) -> Result<u64, String>;
  ```
- `EntryError` gets `#[error("invalid payload: {0}")] Meta(String)`.
- `upsert`: the first step is `checksum.checksum(id, vector, meta)`. An `Err(cause)` gives `EntryError::Meta(cause)` and changes nothing (no stamp, no signal, not dirty).
- `verify` and the read-time check: an `Err(cause)` from `checksum` is reported as `VerifyError::Entry { entry, cause }` and as a decode failure respectively.
- New production implementation in the same module:
  ```rust
  pub struct PayloadChecksums { /* Arc<ParquetDecoder> */ }
  impl PayloadChecksums { pub fn new(decoder: Arc<ParquetDecoder>) -> Self; }
  impl ChecksumProvider for PayloadChecksums { /* see below */ }
  ```
  - `checksum(id, vector, meta)`: `meta_codec::decode_meta(meta)`, then `entry_payload::compute_checksum_parts(vector, &f.target_file, f.char_start, f.char_end)`. A `MetaError` gives `Err(text)`.
  - `stored_checksums(path)`: reads the file bytes and returns `ParquetDecoder::read_stored_checksums`, mapped to `(EntryId, u64)`; errors as text.
- The decoder passed to `PayloadChecksums` and to `LazyIndexFile` is the same instance, built with `ChecksumFn = Arc::new(entry_payload::compute_checksum_parts)`.
- The meta bytes of `Payload.meta`, `EntryStore::meta` and `upsert` are always in the `meta_codec` format. Empty meta means the default source.
- **Export for saving** (decision D-1, §7). `EntryStore` does not import `index_save` (it is built later). It offers a neutral export, and `index_save` adapts it (6.6):
  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
  #[error("export failed: {0}")]
  pub struct ExportError(pub String);

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct ExportedEntry {
      pub id: EntryId,
      pub stamp: Stamp,
      pub vector: SharedBytes,   // dimension * 2 bytes, FP16 LE
      pub meta: SharedBytes,     // meta_codec format
      pub checksum: u64,         // stored checksum: computed at upsert, or read from the file
  }

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct Exported {
      pub seq: u64,              // change sequence this export reflects
      pub max_stamp: u64,        // counter max_raw, None -> 0
      pub entries: Vec<ExportedEntry>,   // ascending by id, unique, no tombstones
  }

  impl EntryStore {
      /// Number of upserts and removes since construction. Starts at 0.
      pub fn change_seq(&self) -> u64;
      /// Ok(None) if not dirty. Blocking.
      pub fn export(&self) -> Result<Option<Exported>, ExportError>;
      /// Sets saved_seq = max(saved_seq, seq). Never lowers it.
      pub fn mark_saved_upto(&self, seq: u64);
  }
  ```
  - Counters: `change_seq` increases by 1 on every successful `upsert` and every `remove` that found an entry. `saved_seq` starts at 0. `is_dirty()` is `change_seq > saved_seq`. This replaces the old definition "a change happened since opening".
  - `export`: under the overlay lock, copy the overlay and read `change_seq` (this is `seq`). After releasing the lock, read every file row that the overlay neither replaces nor removes through `LazyIndexFile::{stamps, read_vector, read_meta}`, and their stored checksums through `ChecksumProvider::stored_checksums(path)`. Then merge by id. An overlay upsert replaces the file row, a tombstone removes it. The file part is immutable, so the result is consistent for `seq`.
  - Stored checksums are exported as stored. They are not recomputed, so a damaged file row stays detectable after saving.
  - Errors give `Err(ExportError)` and change nothing: the file is unavailable (`OpenError`), a file row has an empty vector or no meta (decode failure), `stored_checksums` fails, or an id has no stored checksum.
  - After a successful save the store keeps serving from its old mapping and the overlay (the rename leaves the old mapping valid). The overlay is not pruned. The caller may build a new `EntryStore` over the new file to release the memory. Correctness does not depend on it.
  - Concurrency: `export` holds the overlay lock only for the copy. Writers are never blocked for the file reads.

### 6.6 129 (index save)
- §2 append the reader rules the saver must satisfy: entries ascending and unique by id (already), `1 <= stamp.as_raw() <= snapshot.max_stamp` for every entry, vectors of exactly `vector_bytes()`, no nulls.
- `SaveConfig.row_group_rows` replaces `OpenOptions.rows_per_group` of the withdrawn spec 048. A default of 8192 is set by the caller.
- The saver writes `target_file`, `char_start`, `char_end`, `checksum` from `EntryData` directly, not from `meta` bytes.
- §1 internal modules: add `entry_store` (for the adapter below only).
- New adapter in `index_save`, which makes `EntryStore` the snapshot source (decision D-1):
  ```rust
  pub struct StoreSource { /* Arc<EntryStore> */ }
  impl StoreSource { pub fn new(store: Arc<EntryStore>) -> Self; }
  impl SnapshotSource for StoreSource { /* below */ }
  ```
  - `snapshot()`: `store.export()`. `Ok(None)` gives `None`. `Err(e)` is logged with `error!` and gives `None` (the store stays dirty, the saver reports `Unchanged`). `Ok(Some(x))` gives `Snapshot { token: SnapshotToken(x.seq), max_stamp: x.max_stamp, entries }`, where every entry becomes `SnapshotEntry { id, stamp: ChangeStamp::from_raw(stamp.0), data: EntryData::from_stored(vector, SourceRef::from_meta(&meta)?, checksum) }`. A `MetaError` is treated like `Err(e)`. `from_stored` keeps the stored checksum and does not recompute it.
  - `mark_saved(token)`: `store.mark_saved_upto(token.0)`.
- The caller of `IndexSaver::save` is not defined by any spec of this chain (decision D-3, §7).

### 6.7 058 (index residency)
- Replace every statement about `FileIndex (file_index)` by the same statement about `EntryStore (entry_store)`: it implements `Index` with `access_methods() == &[]`; `byte_ranges` gives `Unsupported(Range)`; reads run without hints; `read` gives `Payload::default()` on IO or decode errors.
- Replace "The index is typically a `FileIndex` (Parquet, mmap)" by "The index is typically an `EntryStore` over a `LazyIndexFile` (Parquet, mmap)".
- Test fixture: `EntryStore::new(header, None, checksum)` with `upsert(id, vector, meta)`, vector length `dimension × 2`, meta empty or from `encode_meta`. No file is needed.

### 6.8 137 (CPU sub-engine)
- §3 imports: replace `use file_index::{FileIndex, IndexHeader, OpenOptions};` by
  ```rust
  use index_format::IndexHeader;
  use index_open::{LazyIndexFile, OpenOptions};
  use parquet_decoder::ParquetDecoder;
  use entry_store::{EntryStore, PayloadChecksums};
  ```
- `IndexSpec.header` is the expected header (`IndexHeader::from_descriptor(..)`). `IndexSpec.open` is `index_open::OpenOptions` (`use_mmap`, `verify`). A field `row_group_rows` is not added; the CPU sub-engine does not save.
- §4.5, `FileLazyIndex::get` replaces "calls `FileIndex::open(..)`":
  1. If `spec.path` does not exist: `EntryStore::new(header, None, checksums)`.
  2. Otherwise build a **new** `LazyIndexFile::new(path, decoder, open)` on every attempt, call `ensure_open()`, and on `Err(e)` return `Err(e.to_string())` and drop it. A failure is therefore not cached, although `LazyIndexFile` remembers its own failure.
  3. On success build `EntryStore::new(header, Some(Arc::new(file)), checksums)`.
  4. The decoder is `ParquetDecoder::new(header.clone(), Arc::new(entry_payload::compute_checksum_parts))`, `checksums` is `PayloadChecksums::new(decoder)`.
  5. The rest (storing in the `IndexSlot` with `OnceLock::set`, same instance for `Manager`, `CpuManager::index` and `IndexResidency`) is unchanged.
- `CpuError::IndexOpen(String)` carries the `Display` of `OpenError` (which contains the decoder's header or binding text).
- §5: replace the `index_contract / file_index` bullet by `index_contract / entry_store / index_open / parquet_decoder`, with `EntryStore`, `LazyIndexFile::{new, ensure_open}` and `FileLazyIndex` building them.
- *Consequence of F16-only vectors* (index vectors are `dimension × 2` bytes, and `Manager` passes them unchanged, so `ManagerConfig.slot_bytes == header.vector_bytes()`):
  - `HostSegments::with_f32` is replaced by `fn with_f16(&self, segment: &IndexSegment, byte_offset: u64, count: usize, f: &mut dyn FnMut(&[u8])) -> Result<(), String>`, zero-copy, little-endian FP16 bytes for `count` elements.
  - §4.6 validation step 3 becomes `query.len() * 2 == slot_bytes`, else `QueryDimMismatch`.
  - `CpuContext::score` converts the rows block-wise (at most 1024 rows per block) into a scratch `Vec<f32>` with `half::f16::to_f32` and calls the unchanged `CpuScoreKernel::score(query, rows_f32, dim, out)`. Accumulation stays f32. The default compute dtype stays `DType::F32`; `has_scoring_kernel(Cpu, F16)` is unaffected because it refers to the compute dtype.
  - `CpuSlotTransfer::write_slot` is unchanged.

### 6.9 Specs that need no change
033 (index contract), 123 (change detection, uses only header constants), 124, 125, 135, 136, 138, 140. They use `index_contract::Index` and `Arc<dyn Index>` only.

### 6.10 Cutover order
1. Build `meta_codec` and `parquet_decoder` (this spec). The old modules stay untouched. Both `IndexHeader` types coexist in different modules.
2. Build `entry_payload` (121), `entry_store` (122) and `index_save` (129) with the amendments above.
3. Port the `index_residency` test to `EntryStore` (6.7).
4. Delete `file_index.rs`, `test/file_index.rs`, `index.rs`, `test/index.rs` and their `mod` lines in `l030/a/mod.rs`. Run the complete test suite.
5. Build `cpu_subengine` (137) against the amended interface.

## 7. Decisions taken in this spec

These were open questions during the analysis. They are decided here so that the spec is complete. A different decision changes exactly the items named.

- **D-1: snapshot source.** `EntryStore` provides `change_seq`, `export` and `mark_saved_upto` (6.5). `index_save` provides the adapter `StoreSource` (6.6). Reason: 129 requires a snapshot source, 122 excludes saving and dirty reset, and `EntryStore` cannot import `index_save` (built later). Changing this affects 122 and 129 only.
- **D-2: vectors are FP16 only.** Header `storage_dtype` is `F16`, the column is `FixedSizeList<Float16>`, `EntryStore::dtype()` is `F16` (113 and 122 already fix this). Consequence for the CPU sub-engine is in 6.8. F32 in the index is not supported. Changing this affects 113, 122, 129, 137 and 058 together.
- **D-3: save trigger.** No module in this chain calls `IndexSaver::save`, and `cpu_subengine` never writes the file. A later spec for the indexing path must define the caller. It builds `IndexSaver::new(config, Arc::new(StoreSource::new(store)))` and calls `save` through `page_advice::read_blocking`. This does not block any item of this spec.
- **D-4: row groups.** `SaveConfig.row_group_rows` defaults to 8192 (set by the caller). `decode_rows` reads whole row groups, so one call needs memory for at most the row groups that overlap 4096 rows.
- **D-5: index `index.rs` and `file_index.rs`.** Both are withdrawn (6.1). `producing_profile` is dropped. Prefix and `max_seq_len` changes invalidate an index (113 is the only binding definition).
- **D-6: meta format.** The empty byte string is the default source, everything else is length-prefixed (4.8). Writers other than `index_save` never write `meta` bytes into the file, only the three columns.

## 8. Analysis record (why this spec exists)

This section is the evidence for the amendments. It was verified by reading code and specs unless marked *not verified*.

### 8.1 Three `IndexHeader` types existed

| Type | Fields | Binding / notes |
|---|---|---|
| `l030/a/index.rs` `index::IndexHeader` (spec 049) | `model_id`, `dimension`, `storage_dtype`, `pooling` (Mean or Cls), `normalization`, `producing_profile: Option<String>` | `is_compatible` compares 5 fields, ignores `producing_profile`. Stored in its own binary format, not Parquet. Spec 049 states that prefixes, `max_seq_len` and compute dtypes do not invalidate the index. |
| `l030/a/file_index.rs` `file_index::IndexHeader` (spec 048) | `model_id`, `dimension`, `dtype` | Parquet footer keys `index.model_id`, `index.dimension`, `index.dtype`. `check_header` stops at the first error (`HeaderMismatch`, `<missing>`). |
| `l080/a/index_format.rs` `index_format::IndexHeader` (spec 113) | 16 fields | `check_binding` collects all mismatches. Model id and dimension are the hard binding, the other pipeline features must be equal. `max_stamp` and `compute_dtype` are ignored. Pooling has no `Cls`. |

### 8.2 Two Parquet formats existed

| Item | Old (`file_index`, spec 048) | New (`index_format`, spec 113) |
|---|---|---|
| Row key | `id` UInt64 | `entry_id` UInt64 |
| Stamp | `stamp` UInt64, >= 1 | `change_stamp` UInt64 |
| Vector | `vector` Binary, any `DType` | `vector` FixedSizeList<Float16, dim>, F16 only |
| Payload | `meta` Binary, opaque | `target_file` Utf8, `char_start` UInt64, `char_end` UInt64 |
| Integrity | none per row | `checksum` UInt64 |
| Footer keys | `index.model_id`, `index.dimension`, `index.dtype`, `index.max_stamp` (missing = `Corrupt`) | 16 keys without prefix: `schema_version`, `max_stamp` (0 = empty), `hash_algorithm`, `hash_version`, `model_id`, `model_revision`, `dimension`, `pooling`, `normalization`, `query_prefix`, `passage_prefix`, `storage_dtype`, `compute_dtype`, `max_seq_len`, `tokenizer_fingerprint`, `preprocessing_id` (missing = `<fehlt>` mismatch) |
| Errors | `FileIndexError::{Io, HeaderMismatch, Corrupt}` | `FormatError`, `OpenError::Unavailable` |

Opening a file of one format with the other format's code fails: the old `open` rejects a new file at `check_header` (`index.model_id` missing, and later `schema mismatch`). The new `read_info` rejects an old file with 16 `<fehlt>` mismatches (4.9).

### 8.3 The old monolith was split into specs
- 048 (`file_index`) was one module: file format, open, lazy read, in-memory changes, stamps, signal, atomic save. The later specs distribute these tasks: open and lazy loading 047, format 113, payload and checksum 121, entry management and `Index` implementation 122, change detection 123, atomic save 129. Specs 122 and 129 already import `index_format::IndexHeader` and `index_open::LazyIndexFile`. Spec 137 (CPU sub-engine, later than all of them) still used `file_index::{FileIndex, IndexHeader, OpenOptions}`. That conflict is resolved in 6.8.
- 049 (`index`) is a tree index (`ParentRef`, `Op`, `ChangeSet`, `changes_since`) with its own `EntryId`/`Stamp` aliases (`u64`) and `IndexError`. No spec lists it as relied-on, and no code imports it (only its own test). The consumers named in its context (expander, kernel mirror, sub-engines) use `index_contract::Index`.

### 8.4 Gaps that this spec closes
- **Decoder gap.** Spec 047 requires an injected `FormatDecoder`. Spec 113 defines only header and schema. No spec and no code provided the production decoder. The only implementation was a `Mock` in `test/index_open.rs`. Closed by `parquet_decoder`.
- **Binding gap.** No module called `IndexHeader::check_binding`, `from_kv` or `check_column_schema`. `LazyIndexFile` has no expected-header parameter, and `FileInfo` carries only `model_id` and `model_revision`. Closed by the decoder holding the expected header (4.3).
- **Meta gap.** Spec 122 passes `meta` bytes to `ChecksumProvider::checksum(id, vector, meta)`, while the checksum of 121 is defined over `(vector, SourceRef)`. No spec defined how `meta` bytes map to the three source columns. Closed by `meta_codec`.
- **Snapshot gap.** Closed by D-1.
- **F16 vs F32 gap.** 137 scored F32 slots (`query.len() * 4 == slot_bytes`) while the index holds F16. Closed in 6.8.

### 8.5 Checks that came out fine (no change needed)
- `StampCounter::resume(stored, highest)` computes `stored.unwrap_or(0).max(highest.unwrap_or(0))`. `None` and `Some(0)` behave the same, so `stored_max_stamp = Some(header.max_stamp)` loses nothing. `LazyIndexFile::open` passes `None` for `highest` when there are 0 rows.
- `Pooling::Cls` is rejected by `index_format::from_descriptor` and also by the pooling spec (`UnsupportedPooling`), the model registry (no `PoolingKind` counterpart) and the encoder construction (`Incompatible`). Rejecting it in the header is consistent with the whole pipeline. It is not a conflict.
- The only code consumer of `file_index` outside its own module is `l040/test/index_residency.rs`. It builds a `FileIndex` with `dtype: F32`, `dimension: 2` and a path that does not exist (no file IO). No other module under `src`, no integration test under `tests/`, and nothing in `benches/` or `src/bin/` references `file_index`, `index_open`, `LazyIndexFile`, `index_format` or `l030::a::index`. `examples/` does not exist.
- Parquet and Arrow versions in `Cargo.toml`: `parquet = "60.0.0"`, `arrow = "60.0.0"`. `arrow_schema` is not a direct dependency, so `arrow::datatypes` is used. `half = "2.7.1"`, `memmap2`, `thiserror = "2.0.20"`, `tracing`, `static_assertions` and `tokio-util` (its `bytes` re-export is used by `file_index.rs`) are available. New crates are forbidden.

### 8.6 Not verified
- The `'static` requirement of `ParquetRecordBatchReaderBuilder` in parquet 60 (4.1 assumes it and designs around it). The invariants of 4.1 are the requirement, the mechanism is a proposal. If the crate allows a borrowing reader, the simpler mechanism may be used.
- Whether a Parquet round trip of `FixedSizeList<Float16>` keeps the item field as non-nullable and named `item` in parquet 60. `check_column_schema` ignores the child name but compares its nullability and type. The tests in 4.11 (header and schema) decide this. If the round trip yields a nullable item, the fix belongs to the writer (`ArrowWriter` with the embedded Arrow schema), not to the decoder.
- Specs 122 §4 (read-time checksum check in detail), 129 §4 (written file content, in detail), 136 and 138 were read only where they mention the index types.
- `cargo test` was green with 1940 tests before this spec. The `runTests` command does not support a test filter.

## 9. Work packages and acceptance

Order (each step ends with a complete green `cargo test`, GPU build as default):

1. **WP-1: `meta_codec`.** `l080/a/meta_codec.rs`, `l080/a/test/meta_codec.rs`, registered in `l080/a/mod.rs`. Tests per 4.11.
2. **WP-2: `parquet_decoder`.** `l080/a/parquet_decoder.rs`, `l080/a/test/parquet_decoder.rs`, registered in `l080/a/mod.rs`. Uses `index_format` (same layer) and `index_open` (`l070/.../l030/a`). The file `index_open.rs` is not changed. Tests per 4.11, including the integration test with `LazyIndexFile`.
3. **WP-3: spec amendments.** Apply section 6 to the specs 047, 113, 121, 122, 129, 058 and 137 (text changes) and mark 048 and 049 as withdrawn. After that these specs are the authority again, and this section 6 is history.
4. **WP-4 (later builds, not part of 113b):** `entry_payload` (121), `entry_store` (122, with export and `PayloadChecksums`), `index_save` (129, with `StoreSource`), `change_detect` (123), `cpu_subengine` (137). Each uses the amended text.
5. **WP-5: cutover** (6.10): port `index_residency` test, delete `file_index.rs`, `index.rs`, their tests and `mod` lines.

Acceptance for WP-1 and WP-2: all tests of 4.11 pass; no `unsafe`; no new crate; no panic on truncated or garbled input; `cargo test` is green overall; `file_index` and `index` still compile and pass until WP-5.

## 10. Project rules that apply

- Project documents and code comments are English only. Names of modules, files, types, structs and functions are English. Comments are minimal and never refer to plan elements or plan numbers.
- A module's tests live in `<level>/test/<name>.rs`. The source file ends with `#[cfg(test)] #[path = "test/<name>.rs"] mod tests;`.
- Hierarchy: the deeper the directory, the earlier it is built. A module may import only from deeper (earlier) levels. `index_format`, `meta_codec` and `parquet_decoder` (level `l080/a`) may import `l070/l060/l050/l040/l030/...` (`index_open`, `index_contract`, `change_stamp`), but `l030` cannot import them. `entry_payload`, `entry_store`, `index_save` are at level `l090/a`, one level above, and may import `l080/a`.
- Paths: base `/home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/`. `index_format` is `a/index_format.rs`. The `l030` modules are `l070/l060/l050/l040/l030/a/{index,file_index,index_open}.rs` with tests in `.../a/test/`. `index_contract` is `.../l030/l020/a/index_contract.rs`, `change_stamp` is `.../l030/l020/l010/change_stamp.rs`, `layout` is `.../l030/l020/l010/layout.rs`, `model_descriptor` is `.../l030/l020/a/model_descriptor.rs`. The consumer test is `l070/l060/l050/l040/test/index_residency.rs`. Specs are in `/home/user/xyan/xy.ai.workbench/rag/project/resident_02/23_batch/specs/`.
- Check the real implementation of every referenced element, and use existing elements instead of re-implementing them. Example: use `index_format::{check_column_schema, IndexHeader}` and `index_open` types as they exist.
- Tools: `cargo test` through the `runTests` command; the manifest is shown by `read.Cargo.toml`. Environment: CUDA 13.3, GPU sm_75 (GTX 1660 SUPER). The default build includes the GPU; there are no feature switches for it.
- Model for reference data: `/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small`. Crate root: `/home/user/xyan/xy.ai.workbench/rag/src/lib.rs` (contains `pub mod resident;`).
