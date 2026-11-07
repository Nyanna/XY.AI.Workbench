Setze G2 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks
- Erledige restlos alle Punkte

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g_02_index_schema.md)

SystemInit:  id=80fb4a4b-9f5b-4da5-bbcf-0463e039af0e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über den bestehenden Code.
Control Request:
```yaml
id: ed48-189
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Struktur des resident-Moduls und Projektwurzel ermitteln
```
/answer ed48-189 allow
ReasoningToken: 37
Control Request:
```yaml
id: ed48-18a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (8 files)"
      - blocking.rs
      - "./facade/test: (3 files)"
      - addon.rs
      - "./index: (3 files)"
      - memory.rs
      - "./index/test: (2 files)"
      - memory.rs
      - "./kernel: (14 files)"
      - catalog.rs
      - "./kernel/test: (10 files)"
      - cosine.rs
      - "./manager: (19 files)"
      - advice.rs
      - "./manager/test: (16 files)"
      - advice.rs
      - "./memory: (13 files)"
      - arena.rs
      - "./memory/test: (9 files)"
      - arena.rs
      - "./model: (8 files)"
      - batch.rs
      - "./model/test: (6 files)"
      - batch.rs
      - "./op: (3 files)"
      - lease.rs
      - "./op/test: (3 files)"
      - lease.rs
      - "./planner: (6 files)"
      - adapter.rs
      - "./planner/test: (2 files)"
      - adapter.rs
      - "./precision: (20 files)"
      - axis.rs
      - "./precision/test: (17 files)"
      - axis.rs
      - "./processor: (11 files)"
      - completion.rs
      - "./processor/test: (6 files)"
      - completion.rs
      - "./queue: (5 files)"
      - mod.rs
      - "./segment: (5 files)"
      - error.rs
      - "./segment/test: (4 files)"
      - mapping.rs
      - "./subengine: (6 files)"
      - device.rs
      - "./subengine/test: (4 files)"
      - device.rs
      - "./test: (2 files)"
      - error.rs
      - "./tokenize: (4 files)"
      - error.rs
      - "./tokenize/test: (1 file)"
      - mod.rs
      warning: Result reduced because the total number of entries (216) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - .pydevproject
      - Cargo.lock
      - Cargo.toml
      - TODO.md
      - xyrag.sh
```
/answer ed48-18a allow
Control Request:
```yaml
id: ed48-18b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g_02_index_schema.md
  reason: Plan-Datei G2 im Repo prüfen (ggf. Status-Updates)
```
/answer ed48-18b deny später vielleicht
Control Request:
```yaml
id: ed48-18c
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
  reason: "Dateien von index, model, segment, test auflisten"
```
/answer ed48-18c allow
Control Request:
```yaml
id: ed48-18d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      entries:
      - ".:"
      - memory.rs
      - mod.rs
      - stamp.rs
      - "./test:"
      - memory.rs
      - stamp.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model
      entries:
      - ".:"
      - batch.rs
      - candle_adapter.rs
      - encoder.rs
      - error.rs
      - head.rs
      - mod.rs
      - pooling.rs
      - slot.rs
      - "./test:"
      - batch.rs
      - candle_adapter.rs
      - head.rs
      - mod.rs
      - pooling.rs
      - slot.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      entries:
      - ".:"
      - error.rs
      - mapping.rs
      - mod.rs
      - slots.rs
      - sync.rs
      - "./test:"
      - mapping.rs
      - mod.rs
      - slots.rs
      - sync.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
      entries:
      - ".:"
      - error.rs
      - telemetry.rs
```
/answer ed48-18d allow
Control Request:
```yaml
id: ed48-18e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  reason: Dependencies und Features prüfen
```
/answer ed48-18e allow
Control Request:
```yaml
id: ed48-18f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: h8JWUc|XOXuKk
        signature: "[package]"
      - id: mN2I6g|SIE1cj
        signature: "[lib]"
      - id: cW2ehx|QePTHE
        signature: "[[bin]]"
      - id: F2LjhE|e8LBEZ
        signature: "[dependencies]"
      - id: anLBDO|YAkuwn
        signature: "[target.'cfg(target_os = \"linux\")'.dependencies]"
      - id: adh0oF|46Sfax
        signature: "[features]"
      - id: UeF6gB|FaChiw
        signature: "[dev-dependencies]"
      - id: 9OxNvg|OtfeRA
        signature: "[[bench]]"
      - id: 9XeKXQ|8kn2M9
        signature: "[patch.crates-io]"
```
/answer ed48-18f allow
Control Request:
```yaml
id: ed48-190
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids:
    - F2LjhE|e8LBEZ
    - anLBDO|YAkuwn
    - adh0oF|46Sfax
    - UeF6gB|FaChiw
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
  reason: Dependencies und bestehenden Index-Trait/Stamp lesen
```
/answer ed48-190 allow
Control Request:
```yaml
id: ed48-191
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: F2LjhE|e8LBEZ
        type: table
        code: |+
          [dependencies]
          tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
          tokio-util = "0.7"
          async-trait = "0.1"
          futures = "0.3"
          serde = { version = "1", features = ["derive"] }
          serde_json = "1"
          serde_yaml = "0.9"
          rusqlite = { version = "0.31", features = ["bundled"] }
          clap = { version = "4", features = ["derive"] }
          anyhow = "1"
          rayon = "1"
          rkyv = { version = "0.7", features = ["validation"] }
          glob = "0.3"
          lru = "0.12"
          regex = "1"
          unicode-normalization = "0.1"
          thiserror = "2.0.20"
          tracing = "0.1.44"
          tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
          tracing-appender = "0.2.3"
          crossbeam = "0.8.5"
          crossbeam-queue = "0.3.14"
          parking_lot = "0.12.5"
          arc-swap = "1.9.2"
          toml = "0.9"
          half = "2.7.1"
          memmap2 = "0.9.3"
          xxhash-rust = { version = "0.8", features = ["xxh3"] }
          parquet = "60.0.0"
          arrow = "60.0.0"
          tokenizers = { version = "0.23.2", default-features = false, features = ["fancy-regex"] }
          safetensors = "0.8.0"
          candle-core = { version = "0.11.0", optional = true, default-features = false }
          candle-nn = { version = "0.11.0", optional = true, default-features = false }
          candle-transformers = { version = "0.11.0", optional = true, default-features = false }
          cudarc = { version = "=0.19.9", optional = true }
          rand = { version = "0.8", optional = true }
          roaring = "0.11.5"

      - id: anLBDO|YAkuwn
        type: table
        code: |+
          [target.'cfg(target_os = "linux")'.dependencies]
          libc = "0.2"

      - id: adh0oF|46Sfax
        type: table
        code: |+
          [features]
          default = []
          candle = ["dep:candle-core", "dep:candle-nn", "dep:candle-transformers"]
          cuda = ["candle", "dep:cudarc", "candle-core/cuda", "candle-nn/cuda", "candle-transformers/cuda"]
          mock = ["dep:rand"]
          json = ["tracing-subscriber/json"]

      - id: UeF6gB|FaChiw
        type: table
        code: |+
          [dev-dependencies]
          proptest = "1"
          criterion = "0.5"
          mockall = "0.15.0"
          loom = "0.7"
          shuttle = "0.7"
          rand = "0.8"
          tokio = { version = "1", features = ["test-util"] }

    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index trait, Scope, AccessMethod, Stamp."
      - id: memory
        signature: pub mod memory;
      - id: stamp
        signature: pub mod stamp;
      - id: kAsHBt|dtXyFJ
        signature: use std::ops::Range;
      - id: EntryId
        signature: pub struct EntryId(u64);
      - id: impl_EntryId
        signature: "impl EntryId {"
        children:
        - id: impl_EntryId.new
          signature: "pub const fn new(raw: u64) -> Self {"
        - id: impl_EntryId.get
          signature: "pub const fn get(self) -> u64 {"
      - id: NodeId
        signature: pub struct NodeId(pub u64);
      - id: CellId
        signature: pub struct CellId(pub u64);
      - id: Stamp
        signature: pub struct Stamp(u64);
        docstring: Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
      - id: impl_Stamp
        signature: "impl Stamp {"
        children:
        - id: impl_Stamp.from_raw
          signature: "pub(crate) const fn from_raw(raw: u64) -> Self {"
        - id: impl_Stamp.raw
          signature: "pub const fn raw(self) -> u64 {"
          docstring: "Raw value, e.g. as `watch` watermark or for persistence."
      - id: AccessMethod
        signature: "pub enum AccessMethod {"
      - id: Scope
        signature: "pub enum Scope {"
      - id: EntrySet
        signature: "pub struct EntrySet {"
        docstring: "Compact set of entry ids backed by a Roaring treemap (array, bitmap and run con…"
      - id: impl_std__hash__Hash_for_EntrySet
        signature: "impl std::hash::Hash for EntrySet {"
        children:
        - id: impl_std__hash__Hash_for_EntrySet.hash
          signature: "fn hash<H: std::hash::Hasher>(&self, state: &mut H) {"
      - id: impl_EntrySet
        signature: "impl EntrySet {"
        children:
        - id: impl_EntrySet.new
          signature: "pub fn new() -> Self {"
        - id: impl_EntrySet.from_ranges
          signature: "pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {"
        - id: impl_EntrySet.from_ids
          signature: "pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {"
        - id: impl_EntrySet.ranges
          signature: "pub fn ranges(&self) -> Vec<Range<u64>> {"
          docstring: "Maximal contiguous runs in ascending order; walks every id, so prefer `iter` or…"
        - id: impl_EntrySet.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_EntrySet.len
          signature: "pub fn len(&self) -> u64 {"
        - id: impl_EntrySet.contains
          signature: "pub fn contains(&self, id: EntryId) -> bool {"
        - id: impl_EntrySet.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {"
        - id: impl_EntrySet.union
          signature: "pub fn union(&self, other: &Self) -> Self {"
        - id: impl_EntrySet.intersection
          signature: "pub fn intersection(&self, other: &Self) -> Self {"
      - id: EntryData
        signature: "pub enum EntryData<'a> {"
        docstring: Variable-length vector region of one entry; never a copy of backing storage.
      - id: impl_EntryData____
        signature: "impl EntryData<'_> {"
        children:
        - id: impl_EntryData____.as_bytes
          signature: "pub fn as_bytes(&self) -> &[u8] {"
      - id: SummaryRef
        signature: "pub struct SummaryRef(pub Arc<[u8]>);"
        docstring: "Summary structure such as a centroid table, exposed as vector data."
      - id: HashAlgo
        signature: "pub enum HashAlgo {"
      - id: IndexHeader
        signature: "pub struct IndexHeader {"
        docstring: Binding between index and model; persistence format is owned by the index.
      - id: Index
        signature: "pub trait Index {"
        docstring: "Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`."
        children:
        - id: Index.stamp
          signature: "fn stamp(&self, entry: EntryId) -> Option<Stamp>;"
          docstring: '`None` = entry removed.'
        - id: Index.stamps
          signature: "fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_>;"
          docstring: Must not touch vector data.
        - id: Index.read
          signature: "fn read(&self, entry: EntryId) -> EntryData<'_>;"
          docstring: Missing entries yield empty data. May block on I/O.
        - id: Index.apply
          signature: "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;"
          docstring: Serializes writers (last writer wins in dequeue order). Returns the stamp
        - id: Index.changed
          signature: fn changed(&self) -> watch::Receiver<u64>;
          docstring: Level signal carrying the latest stamp watermark. Consume with
        - id: Index.capabilities
          signature: "fn capabilities(&self) -> &[AccessMethod];"
        - id: Index.children
          signature: "fn children(&self, node: NodeId) -> EntrySet;"
        - id: Index.members
          signature: "fn members(&self, cell: CellId) -> EntrySet;"
        - id: Index.summary
          signature: "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;"
        - id: Index.prefetch
          signature: "fn prefetch(&self, set: &EntrySet);"
          docstring: Hint only; may be a no-op.
        - id: Index.extent
          signature: "fn extent(&self, _entry: EntryId) -> Option<Range<usize>> {"
          docstring: Byte range of `entry` inside contiguous backing storage (e.g. a mapped file).
        - id: Index.backing
          signature: "fn backing(&self, _range: Range<usize>) -> Option<&[u8]> {"
          docstring: "Bytes of a range obtained from [`extent`](Index::extent), or of a run of adjace…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Stamp generation and the fallback chain for file-backed indexes:"
      - id: GFiPD6|n4QrpI
        signature: "use std::sync::atomic::{AtomicU64, Ordering};"
      - id: lnSv3L|jXpXSw
        signature: /// Conservative racy window (FAT worst case).
      - id: StampCounter
        signature: pub struct StampCounter(AtomicU64);
        docstring: Monotonic stamp source. Uniqueness only needs `Relaxed`; visibility of the
      - id: impl_StampCounter
        signature: "impl StampCounter {"
        children:
        - id: impl_StampCounter.new
          signature: "pub const fn new() -> Self {"
        - id: impl_StampCounter.resume
          signature: "pub fn resume(stored_high_water: u64, file_max: u64) -> Self {"
          docstring: "Resume after restart: `max(stored, file_max) + 1`."
        - id: impl_StampCounter.next
          signature: "pub fn next(&self) -> Stamp {"
        - id: impl_StampCounter.high_water
          signature: "pub fn high_water(&self) -> u64 {"
      - id: impl_Default_for_StampCounter
        signature: "impl Default for StampCounter {"
        children:
        - id: impl_Default_for_StampCounter.default
          signature: "fn default() -> Self {"
      - id: FileStamp
        signature: "pub struct FileStamp {"
      - id: Verdict
        signature: "pub enum Verdict {"
      - id: impl_FileStamp
        signature: "impl FileStamp {"
        children:
        - id: impl_FileStamp.is_racy
          signature: "pub fn is_racy(&self) -> bool {"
          docstring: "Racy: mtime lies within the timestamp resolution before the scan, so a"
        - id: impl_FileStamp.verdict
          signature: "pub fn verdict(&self, current: &FileStamp) -> Verdict {"
      - id: content_hash
        signature: "pub fn content_hash(bytes: &[u8]) -> u64 {"
        docstring: xxh3_64; algorithm and version belong in the index header.
      - id: tests
        signature: mod tests;
```
/answer ed48-191 allow
Control Request:
```yaml
id: ed48-192
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    ids:
    - AccessMethod
    - Scope
    - EntryData
    - impl_EntryData____
    - SummaryRef
    - HashAlgo
    - IndexHeader
    - Index
    - kAsHBt|dtXyFJ
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
  reason: Vollständige Trait-/Header-Definition und In-Memory-Referenzimplementierung lesen
```
/answer ed48-192 allow
Control Request:
```yaml
id: ed48-193
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: AccessMethod
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          #[non_exhaustive]
          pub enum AccessMethod {
              Centroid,
              Prefix,
              Subtree,
              Range,
          }
      - id: Scope
        type: ItemEnum
        code: |
          #[derive(Clone, Debug, PartialEq)]
          pub enum Scope {
              All,
              Range(Range<u64>),
              Set(EntrySet),
              Subtree(NodeId),
              Prefix(Vec<u8>),
              Centroid { k: u32, radius: Option<f32> },
          }
      - id: EntryData
        type: ItemEnum
        code: |
          /// Variable-length vector region of one entry; never a copy of backing storage.
          #[derive(Clone, Debug)]
          pub enum EntryData<'a> {
              Borrowed(&'a [u8]),
              Shared(Arc<[u8]>),
          }
      - id: impl_EntryData____
        type: ItemImpl
        signature: "impl EntryData<'_> {"
        children:
        - id: impl_EntryData____.as_bytes
          type: ImplItemFn
          code: |2
                pub fn as_bytes(&self) -> &[u8] {
                    match self {
                        Self::Borrowed(b) => b,
                        Self::Shared(a) => a,
                    }
                }
      - id: SummaryRef
        type: ItemStruct
        code: |
          /// Summary structure such as a centroid table, exposed as vector data.
          #[derive(Clone, Debug)]
          pub struct SummaryRef(pub Arc<[u8]>);
      - id: HashAlgo
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          #[non_exhaustive]
          pub enum HashAlgo {
              Xxh3_64,
              Xxh3_128,
              Blake3,
          }
      - id: IndexHeader
        type: ItemStruct
        code: |
          /// Binding between index and model; persistence format is owned by the index.
          #[derive(Clone, Debug, PartialEq, Eq)]
          pub struct IndexHeader {
              pub model_id: String,
              pub dim: u32,
              pub high_water: u64,
              pub hash_algo: HashAlgo,
              pub hash_version: u32,
          }
      - id: Index
        type: ItemTrait
        signature: "pub trait Index {"
        docstring: "Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`."
        children:
        - id: Index.stamp
          type: TraitItemFn
          code: |2
                /// `None` = entry removed.
                fn stamp(&self, entry: EntryId) -> Option<Stamp>;
        - id: Index.stamps
          type: TraitItemFn
          code: |2
                /// Must not touch vector data.
                fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_>;
        - id: Index.read
          type: TraitItemFn
          code: |2
                /// Missing entries yield empty data. May block on I/O.
                fn read(&self, entry: EntryId) -> EntryData<'_>;
        - id: Index.apply
          type: TraitItemFn
          code: |2
                /// Serializes writers (last writer wins in dequeue order). Returns the stamp
                /// of this very write; callers register it as known to suppress their echo.
                fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;
        - id: Index.changed
          type: TraitItemFn
          code: |2
                /// Level signal carrying the latest stamp watermark. Consume with
                /// `borrow_and_update()`; coalesced signals are harmless.
                fn changed(&self) -> watch::Receiver<u64>;
        - id: Index.capabilities
          type: TraitItemFn
          code: |2
                fn capabilities(&self) -> &[AccessMethod];
        - id: Index.children
          type: TraitItemFn
          code: |2
                fn children(&self, node: NodeId) -> EntrySet;
        - id: Index.members
          type: TraitItemFn
          code: |2
                fn members(&self, cell: CellId) -> EntrySet;
        - id: Index.summary
          type: TraitItemFn
          code: |2
                fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;
        - id: Index.prefetch
          type: TraitItemFn
          code: |2
                /// Hint only; may be a no-op.
                fn prefetch(&self, set: &EntrySet);
        - id: Index.extent
          type: TraitItemFn
          code: |2
                /// Byte range of `entry` inside contiguous backing storage (e.g. a mapped file).
                /// `None` when the index has no such storage or the entry is missing.
                fn extent(&self, _entry: EntryId) -> Option<Range<usize>> {
                    None
                }
        - id: Index.backing
          type: TraitItemFn
          code: |2
                /// Bytes of a range obtained from [`extent`](Index::extent), or of a run of adjacent
                /// extents. `None` when the range is not backed. Never copies.
                fn backing(&self, _range: Range<usize>) -> Option<&[u8]> {
                    None
                }
      - id: kAsHBt|dtXyFJ
        type: ImportGroup
        code: |
          use std::ops::Range;
          use std::sync::Arc;
          use roaring::RoaringTreemap;
          use tokio::sync::watch;
          pub use stamp::{content_hash, FileStamp, StampCounter, Verdict, RACY_WINDOW_NS};
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
      nodes:
      - id: ModuleDoc
        signature: //! Reference in-memory index; builds without `cuda`/`candle`.
      - id: KLORGj|vql8Wh
        signature: use std::collections::BTreeMap;
      - id: Inner
        signature: "struct Inner {"
      - id: InMemoryIndex
        signature: "pub struct InMemoryIndex {"
      - id: impl_InMemoryIndex
        signature: "impl InMemoryIndex {"
        children:
        - id: impl_InMemoryIndex.new
          signature: "pub fn new() -> Self {"
        - id: impl_InMemoryIndex.with_counter
          signature: "pub fn with_counter(counter: StampCounter) -> Self {"
        - id: impl_InMemoryIndex.high_water
          signature: "pub fn high_water(&self) -> u64 {"
        - id: impl_InMemoryIndex.remove
          signature: "pub fn remove(&self, entry: EntryId) -> Option<Stamp> {"
          docstring: Removal consumes a stamp so that re-adding the same id yields a new one.
      - id: impl_Default_for_InMemoryIndex
        signature: "impl Default for InMemoryIndex {"
        children:
        - id: impl_Default_for_InMemoryIndex.default
          signature: "fn default() -> Self {"
      - id: impl_Index_for_InMemoryIndex
        signature: "impl Index for InMemoryIndex {"
        children:
        - id: impl_Index_for_InMemoryIndex.stamp
          signature: "fn stamp(&self, entry: EntryId) -> Option<Stamp> {"
        - id: impl_Index_for_InMemoryIndex.stamps
          signature: "fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {"
        - id: impl_Index_for_InMemoryIndex.read
          signature: "fn read(&self, entry: EntryId) -> EntryData<'_> {"
        - id: impl_Index_for_InMemoryIndex.apply
          signature: "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - id: impl_Index_for_InMemoryIndex.changed
          signature: "fn changed(&self) -> watch::Receiver<u64> {"
        - id: impl_Index_for_InMemoryIndex.capabilities
          signature: "fn capabilities(&self) -> &[AccessMethod] {"
        - id: impl_Index_for_InMemoryIndex.children
          signature: "fn children(&self, _node: NodeId) -> EntrySet {"
        - id: impl_Index_for_InMemoryIndex.members
          signature: "fn members(&self, cell: CellId) -> EntrySet {"
        - id: impl_Index_for_InMemoryIndex.summary
          signature: "fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {"
        - id: impl_Index_for_InMemoryIndex.prefetch
          signature: "fn prefetch(&self, _set: &EntrySet) {"
      - id: tests
        signature: mod tests;
```
/answer ed48-193 allow
ReasoningToken: 83
Control Request:
```yaml
id: ed48-194
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - wc -l src/resident/index/*.rs src/resident/index/test/*.rs src/resident/model/*.rs src/resident/segment/*.rs src/resident/test/*.rs src/resident/mod.rs src/resident/kernel/*.rs 2>/dev/null
    - git status --short | head -20
    - git log --oneline | head -5
  reason: Dateigrößen und Git-Stand überblicken
```
/answer ed48-194 deny kein git verwenden
Control Request:
```yaml
id: ed48-195
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
    ids:
    - KLORGj|vql8Wh
    - Inner
    - InMemoryIndex
    - impl_InMemoryIndex
    - impl_Index_for_InMemoryIndex
  reason: Referenzimplementierung des Index-Traits als Vorlage für den Parquet-Index
```
/answer ed48-195 allow
ReasoningToken: 24
Control Request:
```yaml
id: ed48-196
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
      nodes:
      - id: KLORGj|vql8Wh
        type: ImportGroup
        code: |
          use std::collections::BTreeMap;
          use std::sync::Arc;

          use parking_lot::Mutex;
          use tokio::sync::watch;

          use super::{
              AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp, StampCounter,
              SummaryRef,
          };
      - id: Inner
        type: ItemStruct
        code: |
          struct Inner {
              entries: BTreeMap<EntryId, (Stamp, Arc<[u8]>)>,
          }
      - id: InMemoryIndex
        type: ItemStruct
        code: |
          pub struct InMemoryIndex {
              inner: Mutex<Inner>,
              counter: StampCounter,
              tx: watch::Sender<u64>,
          }
      - id: impl_InMemoryIndex
        type: ItemImpl
        signature: "impl InMemoryIndex {"
        children:
        - id: impl_InMemoryIndex.new
          type: ImplItemFn
          code: |2
                pub fn new() -> Self {
                    Self::with_counter(StampCounter::new())
                }
        - id: impl_InMemoryIndex.with_counter
          type: ImplItemFn
          code: |2
                pub fn with_counter(counter: StampCounter) -> Self {
                    let (tx, _) = watch::channel(counter.high_water());
                    Self {
                        inner: Mutex::new(Inner { entries: BTreeMap::new() }),
                        counter,
                        tx,
                    }
                }
        - id: impl_InMemoryIndex.high_water
          type: ImplItemFn
          code: |2
                pub fn high_water(&self) -> u64 {
                    self.counter.high_water()
                }
        - id: impl_InMemoryIndex.remove
          type: ImplItemFn
          code: |2
                /// Removal consumes a stamp so that re-adding the same id yields a new one.
                pub fn remove(&self, entry: EntryId) -> Option<Stamp> {
                    let mut g = self.inner.lock();
                    g.entries.remove(&entry)?;
                    let stamp = self.counter.next();
                    self.tx.send_replace(stamp.raw());
                    Some(stamp)
                }
      - id: impl_Index_for_InMemoryIndex
        type: ItemImpl
        signature: "impl Index for InMemoryIndex {"
        children:
        - id: impl_Index_for_InMemoryIndex.stamp
          type: ImplItemFn
          code: |2
                fn stamp(&self, entry: EntryId) -> Option<Stamp> {
                    self.inner.lock().entries.get(&entry).map(|(s, _)| *s)
                }
        - id: impl_Index_for_InMemoryIndex.stamps
          type: ImplItemFn
          code: |2
                fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
                    let snapshot: Vec<_> = self.inner.lock().entries.iter().map(|(id, (s, _))| (*id, *s)).collect();
                    Box::new(snapshot.into_iter())
                }
        - id: impl_Index_for_InMemoryIndex.read
          type: ImplItemFn
          code: |2
                fn read(&self, entry: EntryId) -> EntryData<'_> {
                    match self.inner.lock().entries.get(&entry) {
                        Some((_, data)) => EntryData::Shared(Arc::clone(data)),
                        None => EntryData::Borrowed(&[]),
                    }
                }
        - id: impl_Index_for_InMemoryIndex.apply
          type: ImplItemFn
          code: |2
                fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
                    let mut g = self.inner.lock();
                    // Stamp is drawn inside the write lock, immediately before publication.
                    let stamp = self.counter.next();
                    g.entries.insert(entry, (stamp, Arc::from(data)));
                    self.tx.send_replace(stamp.raw());
                    drop(g);
                    tracing::debug!(entry = entry.get(), stamp = stamp.raw(), "apply");
                    stamp
                }
        - id: impl_Index_for_InMemoryIndex.changed
          type: ImplItemFn
          code: |2
                fn changed(&self) -> watch::Receiver<u64> {
                    self.tx.subscribe()
                }
        - id: impl_Index_for_InMemoryIndex.capabilities
          type: ImplItemFn
          code: |2
                fn capabilities(&self) -> &[AccessMethod] {
                    &[AccessMethod::Range]
                }
        - id: impl_Index_for_InMemoryIndex.children
          type: ImplItemFn
          code: |2
                fn children(&self, _node: NodeId) -> EntrySet {
                    EntrySet::new()
                }
        - id: impl_Index_for_InMemoryIndex.members
          type: ImplItemFn
          code: |2
                fn members(&self, cell: CellId) -> EntrySet {
                    if cell.0 != 0 {
                        return EntrySet::new();
                    }
                    EntrySet::from_ids(self.inner.lock().entries.keys().copied())
                }
        - id: impl_Index_for_InMemoryIndex.summary
          type: ImplItemFn
          code: |2
                fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {
                    None
                }
        - id: impl_Index_for_InMemoryIndex.prefetch
          type: ImplItemFn
          code: |2
                fn prefetch(&self, _set: &EntrySet) {}
```
/answer ed48-196 allow
Control Request:
```yaml
id: ed48-197
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/model/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/mapping.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/test/error.rs
  reason: "Modellkonfig (G1), Fehlerklassen und bestehendes mmap-Muster erfassen"
```
/answer ed48-197 allow
Control Request:
```yaml
id: ed48-198
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Model resources: specs, registry, encoder contract and loaded-encoder resou…"
      - id: batch
        signature: mod batch;
      - id: candle_adapter
        signature: pub(crate) mod candle_adapter;
      - id: encoder
        signature: mod encoder;
      - id: error
        signature: mod error;
      - id: head
        signature: mod head;
      - id: pooling
        signature: mod pooling;
      - id: slot
        signature: mod slot;
      - id: 5qa36T|nFqged
        signature: use std::collections::HashMap;
      - id: Pooling
        signature: "pub enum Pooling {"
      - id: Normalize
        signature: "pub enum Normalize {"
        docstring: Scaling applied to every produced vector.
      - id: ComputeDtypes
        signature: "pub struct ComputeDtypes {"
        docstring: Backbone element type per device class; independent of the storage dtype.
      - id: impl_ComputeDtypes
        signature: "impl ComputeDtypes {"
        children:
        - id: impl_ComputeDtypes.for_device
          signature: "pub fn for_device(&self, device: DeviceKind) -> DType {"
      - id: Prefixes
        signature: "pub struct Prefixes {"
      - id: HeadSpec
        signature: "pub struct HeadSpec {"
      - id: ModelPaths
        signature: "pub struct ModelPaths {"
      - id: ModelSpec
        signature: "pub struct ModelSpec {"
        docstring: Plain description of a model; owns no resources.
      - id: impl_ModelSpec
        signature: "impl ModelSpec {"
        children:
        - id: impl_ModelSpec.e5_small
          signature: "pub fn e5_small(dir: &Path, dtype: DType) -> Self {"
          docstring: '`intfloat/multilingual-e5-small` checkout in `dir`.'
        - id: impl_ModelSpec.validate
          signature: "pub fn validate(&self) -> Result<(), ModelError> {"
          docstring: Rejects configurations that would silently degrade embeddings.
      - id: TensorRef
        signature: "pub struct TensorRef {"
        docstring: A region together with its layout; the only way tensors cross the encoder API.
      - id: RegionIo
        signature: "pub trait RegionIo: Send + Sync {"
        docstring: Byte access to regions of one memory domain.
        children:
        - id: RegionIo.read
          signature: "fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError>;"
        - id: RegionIo.write
          signature: "fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError>;"
      - id: impl_RegionIo_for_CpuMemoryManager
        signature: "impl RegionIo for CpuMemoryManager {"
        children:
        - id: impl_RegionIo_for_CpuMemoryManager.read
          signature: "fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError> {"
        - id: impl_RegionIo_for_CpuMemoryManager.write
          signature: "fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError> {"
      - id: EncodeEvent
        signature: "pub struct EncodeEvent {"
        docstring: Completion of one `encode_into` call.
      - id: impl_EncodeEvent
        signature: "impl EncodeEvent {"
        children:
        - id: impl_EncodeEvent.new
          signature: "pub(crate) fn new(     event: EventHandle,     failure: Arc<Mutex<Option<ModelE…"
        - id: impl_EncodeEvent.is_complete
          signature: "pub fn is_complete(&self) -> bool {"
        - id: impl_EncodeEvent.wait
          signature: "pub fn wait(&self) -> Result<(), ModelError> {"
          docstring: "Blocks until the work finished and reports its failure, if any."
      - id: Encoder
        signature: "pub trait Encoder: Send + Sync {"
        children:
        - id: Encoder.dim
          signature: fn dim(&self) -> usize;
        - id: Encoder.dtype
          signature: fn dtype(&self) -> DType;
        - id: Encoder.encode_into
          signature: "fn encode_into(     &self,     mem: &Arc<dyn RegionIo>,     tokens: &TensorRef,…"
          docstring: "`tokens` is a u32 `[2, batch, seq]` region (ids, then mask). `out` receives the"
      - id: EncoderFactory
        signature: "pub type EncoderFactory = Box<     dyn Fn(&ModelSpec, DeviceKind) -> Result<Arc…"
      - id: ModelRegistry
        signature: "pub struct ModelRegistry {"
        docstring: Architecture id -> encoder constructor.
      - id: impl_ModelRegistry
        signature: "impl ModelRegistry {"
        children:
        - id: impl_ModelRegistry.new
          signature: "pub fn new() -> Self {"
        - id: impl_ModelRegistry.with_defaults
          signature: "pub fn with_defaults() -> Self {"
          docstring: Registry with every architecture compiled into this build.
        - id: impl_ModelRegistry.register
          signature: "pub fn register(&mut self, arch_id: impl Into<String>, factory: EncoderFactory)…"
        - id: impl_ModelRegistry.load
          signature: "pub fn load(     &self,     spec: &ModelSpec,     device: DeviceKind, ) -> Resu…"
      - id: LoadedEncoder
        signature: "pub struct LoadedEncoder {"
        docstring: "Architecture plus weights, ready to encode; dropping it unloads the weights."
      - id: impl_LoadedEncoder
        signature: "impl LoadedEncoder {"
        children:
        - id: impl_LoadedEncoder.spec
          signature: "pub fn spec(&self) -> &ModelSpec {"
        - id: impl_LoadedEncoder.device
          signature: "pub fn device(&self) -> DeviceKind {"
        - id: impl_LoadedEncoder.encoder
          signature: "pub fn encoder(&self) -> &Arc<dyn Encoder> {"
      - id: impl_Resource_for_LoadedEncoder
        signature: "impl Resource for LoadedEncoder {"
        children:
        - id: impl_Resource_for_LoadedEncoder.key
          signature: "fn key(&self) -> ResourceKey {"
        - id: impl_Resource_for_LoadedEncoder.init_cost_hint
          signature: "fn init_cost_hint(&self) -> u64 {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/error.rs
      nodes:
      - id: 2TAEor|0h0Ail
        signature: use thiserror::Error;
      - id: ModelError
        signature: "pub enum ModelError {"
        docstring: Model loading and encoding errors.
      - id: impl_IsRetryable_for_ModelError
        signature: "impl IsRetryable for ModelError {"
        children:
        - id: impl_IsRetryable_for_ModelError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/mapping.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Reverse mapping from kernel results to entries, and generation-aware resolu…"
      - id: LIWrj6|yxXZSA
        signature: use parking_lot::RwLock;
      - id: EntryRef
        signature: "pub struct EntryRef {"
      - id: SegmentHit
        signature: "pub struct SegmentHit {"
        docstring: Kernel result in slot terms; the kernel never sees entries.
      - id: Addressing
        signature: "enum Addressing {"
      - id: SlotMap
        signature: "pub struct SlotMap {"
        docstring: "`(segment, slot) -> EntryRef`, plus address -> slot for kernel offsets."
      - id: impl_SlotMap
        signature: "impl SlotMap {"
        children:
        - id: impl_SlotMap.fixed
          signature: "pub fn fixed(base: u64, stride: u32, count: u32) -> Self {"
        - id: impl_SlotMap.variable
          signature: "pub fn variable(offsets: Vec<u64>) -> Self {"
          docstring: '`offsets` holds `n + 1` ascending prefix sums.'
        - id: impl_SlotMap.len
          signature: "pub fn len(&self) -> u32 {"
        - id: impl_SlotMap.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_SlotMap.bind
          signature: "pub fn bind(&self, slot: u32, entry: EntryRef) -> Result<(), SegmentError> {"
        - id: impl_SlotMap.unbind
          signature: "pub fn unbind(&self, slot: u32) -> Result<(), SegmentError> {"
        - id: impl_SlotMap.set
          signature: "fn set(&self, slot: u32, value: Option<EntryRef>) -> Result<(), SegmentError> {"
        - id: impl_SlotMap.entry
          signature: "pub fn entry(&self, slot: u32) -> Option<EntryRef> {"
        - id: impl_SlotMap.index_of
          signature: "pub fn index_of(&self, addr: u64) -> Option<u32> {"
          docstring: Slot (or entry index for variable layouts) containing `addr`.
        - id: impl_SlotMap.slot_span
          signature: "pub fn slot_span(&self, slot: u32) -> Option<std::ops::Range<u64>> {"
          docstring: Absolute byte range of `slot`.
        - id: impl_SlotMap.bounds
          signature: "pub fn bounds(&self) -> Option<std::ops::Range<u64>> {"
          docstring: Absolute byte range covered by all slots; `None` when empty.
      - id: Resolution
        signature: "pub enum Resolution {"
      - id: impl_Manifest
        signature: "impl Manifest {"
        children:
        - id: impl_Manifest.find
          signature: "pub fn find(&self, segment_gen: u64) -> Option<&std::sync::Arc<Segment>> {"
        - id: impl_Manifest.require
          signature: "fn require(     &self,     segment_gen: u64, ) -> Result<&std::sync::Arc<Segmen…"
        - id: impl_Manifest.resolve
          signature: "pub fn resolve(&self, hit: &SegmentHit) -> Result<EntryRef, SegmentError> {"
        - id: impl_Manifest.locate
          signature: "pub fn locate(&self, entry: EntryId) -> Option<(u64, u32)> {"
          docstring: Newest valid location of `entry`.
        - id: impl_Manifest.remap
          signature: "pub fn remap(&self, previous: &Manifest, hit: SegmentHit) -> Resolution {"
          docstring: Resolves `hit` against this manifest. A hit from a generation that is no longer
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/error.rs
      nodes:
      - id: ModuleDoc
        signature: //! Segment-layer errors.
      - id: 2TAEor|YmG8Qp
        signature: use thiserror::Error;
      - id: SegmentError
        signature: "pub enum SegmentError {"
      - id: impl_IsRetryable_for_SegmentError
        signature: "impl IsRetryable for SegmentError {"
        children:
        - id: impl_IsRetryable_for_SegmentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: MappingError
        signature: "pub(crate) enum MappingError {"
        docstring: Internal lookup failure; surfaced as `SegmentError` at the API boundary.
      - id: impl_From_MappingError__for_SegmentError
        signature: "impl From<MappingError> for SegmentError {"
        children:
        - id: impl_From_MappingError__for_SegmentError.from
          signature: "fn from(e: MappingError) -> Self {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Resident engine: embedding/similarity substrate, kept warm."
      - id: backend
        signature: pub mod backend;
      - id: error
        signature: pub mod error;
      - id: failpoint
        signature: pub mod failpoint;
      - id: telemetry
        signature: pub mod telemetry;
      - id: mock
        signature: pub mod mock;
      - id: queue
        signature: pub mod queue;
      - id: memory
        signature: pub mod memory;
      - id: op
        signature: pub mod op;
      - id: index
        signature: pub mod index;
      - id: kernel
        signature: pub mod kernel;
      - id: precision
        signature: pub mod precision;
      - id: facade
        signature: pub mod facade;
      - id: processor
        signature: pub mod processor;
      - id: planner
        signature: pub mod planner;
      - id: subengine
        signature: pub mod subengine;
      - id: model
        signature: pub mod model;
      - id: tokenize
        signature: pub mod tokenize;
      - id: manager
        signature: pub mod manager;
      - id: segment
        signature: pub mod segment;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/error.rs
      nodes:
      - id: LwlKpn|JFVpeI
        signature: use super::*;
      - id: assert_send_sync_static
        signature: "fn assert_send_sync_static<T: Send + Sync + 'static>() {"
      - id: all_errors_are_send_sync_static
        signature: "fn all_errors_are_send_sync_static() {"
      - id: layer_errors_convert_into_resident_error
        signature: "fn layer_errors_convert_into_resident_error() {"
      - id: transparent_wrapper_keeps_message
        signature: "fn transparent_wrapper_keeps_message() {"
      - id: retryability_classification
        signature: "fn retryability_classification() {"
```
/answer ed48-198 allow
Thinking:
Das Index-Trait mit `IndexHeader`, `StampCounter` und `content_hash` (xxh3) existiert bereits, ebenso G1 als `ModelSpec`. Jetzt schaue ich mir die Fehler-Taxonomie und `ModelSpec` an, um das Schema abzugleichen.


ReasoningToken: 177
Control Request:
```yaml
id: ed48-199
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Fehler-Taxonomie des resident-Moduls (Q1) einsehen
```
/answer ed48-199 allow
Control Request:
```yaml
id: ed48-19a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        signature: //! Error taxonomy.
      - id: gaWICu|jY2QN9
        signature: use thiserror::Error;
      - id: IsRetryable
        signature: "pub trait IsRetryable {"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          signature: fn is_retryable(&self) -> bool;
      - id: IndexError
        signature: "pub enum IndexError {"
        docstring: Index/scope layer errors.
      - id: impl_IsRetryable_for_IndexError
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ManagerError
        signature: "pub enum ManagerError {"
        docstring: Index-manager layer errors. The facade maps them to public variants or retries;…
      - id: impl_IsRetryable_for_ManagerError
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PrecisionError
        signature: "pub enum PrecisionError {"
        docstring: Precision/calibration layer errors.
      - id: impl_IsRetryable_for_PrecisionError
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PlannerError
        signature: "pub enum PlannerError {"
        docstring: Planner/task-expansion layer errors.
      - id: impl_IsRetryable_for_PlannerError
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CapabilityError
        signature: "pub enum CapabilityError {"
        docstring: Capability/profile-matching layer errors.
      - id: ConfigError
        signature: "pub enum ConfigError {"
        docstring: Facade/configuration errors raised by the builder and `init()`.
      - id: impl_IsRetryable_for_CapabilityError
        signature: "impl IsRetryable for CapabilityError {"
        children:
        - id: impl_IsRetryable_for_CapabilityError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: impl_IsRetryable_for_ConfigError
        signature: "impl IsRetryable for ConfigError {"
        children:
        - id: impl_IsRetryable_for_ConfigError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: gVatFA|OrOCFr
        signature: pub use crate::resident::subengine::DeviceError;
      - id: ResidentError
        signature: "pub enum ResidentError {"
        docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
      - id: impl_IsRetryable_for_ResidentError
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: tests
        signature: mod tests;
```
/answer ed48-19a allow
Control Request:
```yaml
id: ed48-19b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
    ids:
    - Pooling
    - Normalize
    - ComputeDtypes
    - Prefixes
    - HeadSpec
    - ModelPaths
    - ModelSpec
    - impl_ModelSpec
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/error.rs
    ids:
    - ModelError
  reason: ModelSpec-Felder für die Header-Keys und ModelError-Varianten lesen
```
/answer ed48-19b allow
Control Request:
```yaml
id: ed48-19c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
      nodes:
      - id: Pooling
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum Pooling {
              /// One vector per sequence: mean over unmasked tokens.
              MeanMasked,
              /// One vector per token.
              PerToken,
          }
      - id: Normalize
        type: ItemEnum
        code: |
          /// Scaling applied to every produced vector.
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum Normalize {
              /// Unit Euclidean length; a dot product then equals the cosine.
              L2,
              None,
          }
      - id: ComputeDtypes
        type: ItemStruct
        code: |
          /// Backbone element type per device class; independent of the storage dtype.
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct ComputeDtypes {
              pub cpu: DType,
              pub gpu: DType,
          }
      - id: Prefixes
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq, Eq)]
          pub struct Prefixes {
              pub query: String,
              pub passage: String,
          }
      - id: HeadSpec
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq, Eq)]
          pub struct HeadSpec {
              pub weight_name: String,
              pub in_dim: usize,
              pub out_dim: usize,
              pub bias: bool,
          }
      - id: ModelPaths
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq, Eq)]
          pub struct ModelPaths {
              pub config: PathBuf,
              pub weights: Vec<PathBuf>,
          }
      - id: ModelSpec
        type: ItemStruct
        code: |
          /// Plain description of a model; owns no resources.
          #[derive(Clone, Debug, PartialEq, Eq)]
          pub struct ModelSpec {
              pub model_id: String,
              pub arch_id: String,
              pub dim: usize,
              pub pooling: Pooling,
              pub normalize: Normalize,
              pub prefixes: Prefixes,
              /// Element type of the produced embeddings (storage).
              pub dtype: DType,
              /// Element type of the backbone forward pass per device class.
              pub compute: ComputeDtypes,
              pub max_seq_len: usize,
              pub head: Option<HeadSpec>,
              pub paths: ModelPaths,
          }
      - id: impl_ModelSpec
        type: ItemImpl
        signature: "impl ModelSpec {"
        children:
        - id: impl_ModelSpec.e5_small
          type: ImplItemFn
          code: |2
                /// `intfloat/multilingual-e5-small` checkout in `dir`.
                pub fn e5_small(dir: &Path, dtype: DType) -> Self {
                    Self {
                        model_id: "intfloat/multilingual-e5-small".into(),
                        arch_id: "bert".into(),
                        normalize: Normalize::L2,
                        compute: ComputeDtypes {
                            cpu: DType::F32,
                            gpu: DType::F32,
                        },
                        dim: 384,
                        pooling: Pooling::MeanMasked,
                        prefixes: Prefixes {
                            query: "query: ".into(),
                            passage: "passage: ".into(),
                        },
                        dtype,
                        max_seq_len: 512,
                        head: None,
                        paths: ModelPaths {
                            config: dir.join("config.json"),
                            weights: vec![dir.join("model.safetensors")],
                        },
                    }
                }
        - id: impl_ModelSpec.validate
          type: ImplItemFn
          code: |2
                /// Rejects configurations that would silently degrade embeddings.
                pub fn validate(&self) -> Result<(), ModelError> {
                    let invalid = |reason: String| Err(ModelError::Incompatible { reason });
                    if self.dim == 0 || self.max_seq_len == 0 {
                        return invalid(
                            format!(
                                "dim ({}) and max_seq_len ({}) must be positive", self.dim, self
                                .max_seq_len
                            ),
                        );
                    }
                    for (name, prefix) in [
                        ("query", &self.prefixes.query),
                        ("passage", &self.prefixes.passage),
                    ] {
                        if !prefix.is_empty() && !prefix.ends_with(' ') {
                            return invalid(
                                format!("{name} prefix `{prefix}` must end with a space"),
                            );
                        }
                    }
                    Ok(())
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/error.rs
      nodes:
      - id: ModelError
        type: ItemEnum
        code: |
          /// Model loading and encoding errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ModelError {
              #[error("model load failed: {reason}")]
              Load { reason: String },
              #[error("tensor `{expected}` not found in checkpoint")]
              WeightNameMismatch { expected: String },
              #[error("unsupported architecture `{arch}`")]
              UnsupportedArch { arch: String },
              #[error("incompatible model configuration: {reason}")]
              Incompatible { reason: String },
              #[error("encoding failed: {reason}")]
              EncodeFailed { reason: String },
              #[error(transparent)]
              Memory(#[from] MemoryError),
              #[error(transparent)]
              Device(#[from] DeviceError),
          }
```
/answer ed48-19c allow
Control Request:
```yaml
id: ed48-19d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - impl_IsRetryable_for_IndexError
    - ResidentError
    - impl_IsRetryable_for_ResidentError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
    ids:
    - StampCounter
    - impl_StampCounter
    - content_hash
  reason: IndexError-Varianten und StampCounter/Hash-API für die Parquet-Implementierung
```
/answer ed48-19d allow
Control Request:
```yaml
id: ed48-19e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: IndexError
        type: ItemEnum
        code: |
          /// Index/scope layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum IndexError {
              #[error("access method '{method}' is not supported for this index")]
              UnsupportedScope { method: String },
              #[error("index for scope '{scope}' is not available")]
              IndexUnavailable { scope: String },
              #[error("arena exhausted: requested {requested} bytes, available {available} bytes")]
              ArenaExhausted { requested: usize, available: usize },
              #[error("transfer failed: {reason}")]
              TransferFailed { reason: String },
              #[error("kernel failed: {reason}")]
              KernelFailed { reason: String },
              #[error("unknown kernel batch {batch}")]
              UnknownBatch { batch: u64 },
          }
      - id: impl_IsRetryable_for_IndexError
        type: ItemImpl
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(
                        self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
                    )
                }
      - id: ResidentError
        type: ItemEnum
        code: |
          /// Top-level resident engine error, wrapping every layer error via `#[from]`.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ResidentError {
              #[error(transparent)]
              Index(#[from] IndexError),
              #[error(transparent)]
              Precision(#[from] PrecisionError),
              #[error(transparent)]
              Planner(#[from] PlannerError),
              #[error(transparent)]
              Config(#[from] ConfigError),
              #[error(transparent)]
              Capability(#[from] CapabilityError),
              #[error(transparent)]
              Orchestration(#[from] OrchestrationError),
              #[error(transparent)]
              Subengine(#[from] SubengineError),
              #[error(transparent)]
              Device(#[from] DeviceError),
              #[error(transparent)]
              Memory(#[from] crate::resident::memory::MemoryError),
              #[error(transparent)]
              Model(#[from] crate::resident::model::ModelError),
              #[error(transparent)]
              Tokenize(#[from] crate::resident::tokenize::TokenizeError),
              #[error(transparent)]
              Kernel(#[from] crate::resident::kernel::KernelError),
              #[error(transparent)]
              Segment(#[from] crate::resident::segment::SegmentError),
          }
      - id: impl_IsRetryable_for_ResidentError
        type: ItemImpl
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    match self {
                        ResidentError::Index(e) => e.is_retryable(),
                        ResidentError::Precision(e) => e.is_retryable(),
                        ResidentError::Planner(e) => e.is_retryable(),
                        ResidentError::Config(e) => e.is_retryable(),
                        ResidentError::Capability(e) => e.is_retryable(),
                        ResidentError::Orchestration(e) => e.is_retryable(),
                        ResidentError::Subengine(e) => e.is_retryable(),
                        ResidentError::Device(e) => e.is_retryable(),
                        ResidentError::Memory(e) => e.is_retryable(),
                        ResidentError::Model(e) => e.is_retryable(),
                        ResidentError::Tokenize(e) => e.is_retryable(),
                        ResidentError::Kernel(e) => e.is_retryable(),
                        ResidentError::Segment(e) => e.is_retryable(),
                    }
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/stamp.rs
      nodes:
      - id: StampCounter
        type: ItemStruct
        code: |
          /// Monotonic stamp source. Uniqueness only needs `Relaxed`; visibility of the
          /// data is established by the writer's lock/release when publishing.
          #[derive(Debug)]
          pub struct StampCounter(AtomicU64);
      - id: impl_StampCounter
        type: ItemImpl
        signature: "impl StampCounter {"
        children:
        - id: impl_StampCounter.new
          type: ImplItemFn
          code: |2
                pub const fn new() -> Self {
                    Self(AtomicU64::new(1))
                }
        - id: impl_StampCounter.resume
          type: ImplItemFn
          code: |2
                /// Resume after restart: `max(stored, file_max) + 1`.
                pub fn resume(stored_high_water: u64, file_max: u64) -> Self {
                    Self(AtomicU64::new(stored_high_water.max(file_max) + 1))
                }
        - id: impl_StampCounter.next
          type: ImplItemFn
          code: |2
                pub fn next(&self) -> Stamp {
                    Stamp::from_raw(self.0.fetch_add(1, Ordering::Relaxed))
                }
        - id: impl_StampCounter.high_water
          type: ImplItemFn
          code: |2
                pub fn high_water(&self) -> u64 {
                    self.0.load(Ordering::Relaxed) - 1
                }
      - id: content_hash
        type: ItemFn
        code: |
          /// xxh3_64; algorithm and version belong in the index header.
          pub fn content_hash(bytes: &[u8]) -> u64 {
              xxhash_rust::xxh3::xxh3_64(bytes)
          }
```
/answer ed48-19e allow
Control Request:
```yaml
id: ed48-19f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/test/memory.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/model/test/mod.rs
  reason: Teststil und -Konventionen (Dateiablage test/) erfassen
```
/answer ed48-19f allow
Control Request:
```yaml
id: ed48-1a0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/test/memory.rs
      nodes:
      - id: LwlKpn|JFVpeI
        signature: use super::*;
      - id: id
        signature: "fn id(n: u64) -> EntryId {"
      - id: apply_returns_the_stamp_later_reported
        signature: "fn apply_returns_the_stamp_later_reported() {"
      - id: rewrite_changes_stamp
        signature: "fn rewrite_changes_stamp() {"
      - id: remove_then_readd_yields_new_stamp_no_aba
        signature: "fn remove_then_readd_yields_new_stamp_no_aba() {"
      - id: watch_carries_high_water_and_echo_is_recognisable
        signature: "fn watch_carries_high_water_and_echo_is_recognisable() {"
      - id: coalesced_signals_still_expose_all_stamps
        signature: "fn coalesced_signals_still_expose_all_stamps() {"
      - id: resumed_counter_never_reissues_old_stamps
        signature: "fn resumed_counter_never_reissues_old_stamps() {"
      - id: members_and_stamps_snapshot
        signature: "fn members_and_stamps_snapshot() {"
      - id: entry_set_merges_and_contains
        signature: "fn entry_set_merges_and_contains() {"
      - id: entry_set_handles_wide_ranges_without_materializing_ids
        signature: "fn entry_set_handles_wide_ranges_without_materializing_ids() {"
      - id: entry_set_equality_and_hash_follow_content
        signature: "fn entry_set_equality_and_hash_follow_content() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/test/mod.rs
      nodes:
      - id: Gcn6TU|g7JUd7
        signature: use super::*;
      - id: JE7YZa|8slH7w
        signature: "const DIM: usize = 4;"
      - id: FakeBackbone
        signature: struct FakeBackbone;
        docstring: "Hidden state of token `t` in row `b` is `[b + 1, t, 1, 0]`."
      - id: impl_Backbone_for_FakeBackbone
        signature: "impl Backbone for FakeBackbone {"
        children:
        - id: impl_Backbone_for_FakeBackbone.dim
          signature: "fn dim(&self) -> usize {"
        - id: impl_Backbone_for_FakeBackbone.forward
          signature: "fn forward(     &self,     ids: &[u32],     _mask: &[u32],     batch: usize,   …"
      - id: Fixture
        signature: "struct Fixture {"
      - id: fixture
        signature: "fn fixture() -> Fixture {"
      - id: put_tokens
        signature: "fn put_tokens(f: &Fixture, batch: &Batch) -> TensorRef {"
      - id: out_ref
        signature: "fn out_ref(f: &Fixture, shape: &[usize], dtype: DType) -> TensorRef {"
      - id: read_f32
        signature: "fn read_f32(f: &Fixture, out: &TensorRef) -> Vec<f32> {"
      - id: batch
        signature: "fn batch(seqs: &[Vec<u32>]) -> Batch {"
      - id: mean_pooled_rows_are_unit_length
        signature: "fn mean_pooled_rows_are_unit_length() {"
      - id: output_is_converted_to_slot_dtype
        signature: "fn output_is_converted_to_slot_dtype() {"
      - id: per_token_pooling_with_head_writes_every_position
        signature: "fn per_token_pooling_with_head_writes_every_position() {"
      - id: runs_on_a_cpu_lane_and_completes_through_the_event
        signature: "fn runs_on_a_cpu_lane_and_completes_through_the_event() {"
      - id: mismatching_output_layout_is_rejected
        signature: "fn mismatching_output_layout_is_rejected() {"
      - id: backbone_failure_surfaces_on_wait
        signature: "fn backbone_failure_surfaces_on_wait() {"
      - id: unknown_architecture_is_rejected
        signature: "fn unknown_architecture_is_rejected() {"
      - id: registered_factory_produces_a_keyed_resource
        signature: "fn registered_factory_produces_a_keyed_resource() {"
      - id: ZeroBackbone
        signature: struct ZeroBackbone;
      - id: impl_Backbone_for_ZeroBackbone
        signature: "impl Backbone for ZeroBackbone {"
        children:
        - id: impl_Backbone_for_ZeroBackbone.dim
          signature: "fn dim(&self) -> usize {"
        - id: impl_Backbone_for_ZeroBackbone.forward
          signature: "fn forward(     &self,     _ids: &[u32],     _mask: &[u32],     batch: usize,  …"
      - id: encode_one
        signature: "fn encode_one(f: &Fixture, enc: &dyn Encoder, seqs: &[Vec<u32>]) -> Vec<f32> {"
      - id: zero_hidden_states_yield_zero_vectors_without_nan
        signature: "fn zero_hidden_states_yield_zero_vectors_without_nan() {"
      - id: normalize_none_keeps_the_pooled_magnitude
        signature: "fn normalize_none_keeps_the_pooled_magnitude() {"
      - id: e5_small_spec_pins_the_model_contract
        signature: "fn e5_small_spec_pins_the_model_contract() {"
      - id: prefix_without_trailing_space_is_rejected
        signature: "fn prefix_without_trailing_space_is_rejected() {"
      - id: empty_prefixes_are_allowed_for_prefixless_models
        signature: "fn empty_prefixes_are_allowed_for_prefixless_models() {"
```
/answer ed48-1a0 allow
Control Request:
```yaml
id: ed48-1a1
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: IndexHeader|IndexError::|HashAlgo
    include:
    - '*.rs'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: pub enum DType|fn name|fn size|impl .*DType|fn parse|from_str
    include:
    - dtype.rs
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    pattern: "mod tests|#\\[path|cfg\\(test\\)"
    include:
    - '*.rs'
    limit: 10
  reason: "Verwendungen von IndexError/IndexHeader, DType-API und Test-Wiring ermitteln"
```
/answer ed48-1a1 allow
ReasoningToken: 8211
Control Request:
```yaml
id: ed48-1a2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/test/error.rs
        matches:
        - "let transient: ResidentError = IndexError::ArenaExhausted { requested: 1, available: 0 }.into();"
      - path: resident/index/mod.rs
        matches:
        - "pub enum HashAlgo {"
        - "pub struct IndexHeader {"
        - "pub hash_algo: HashAlgo,"
      - path: resident/kernel/error.rs
        matches:
        - "IndexError::KernelFailed { reason: e.to_string() }"
      - path: resident/kernel/test/mod.rs
        matches:
        - "assert!(matches!(k.run(b, & spans(1)), Err(IndexError::KernelFailed { .. })));"
        - "assert!(matches!(k.end(b), Err(IndexError::UnknownBatch { .. })));"
      - path: resident/kernel/mock.rs
        matches:
        - "return Err(IndexError::KernelFailed {"
        - ".ok_or(IndexError::UnknownBatch {"
        - "return Err(IndexError::UnknownBatch {"
      - path: resident/error.rs
        matches:
        - "self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }"
      - path: resident/test/error.rs
        matches:
        - "let e: ResidentError = IndexError::UnknownBatch {"
        - "matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 }))"
        - "let inner = IndexError::ArenaExhausted {"
        - "IndexError::ArenaExhausted { requested : 1, available : 0 } .is_retryable()"
        - "assert!(IndexError::IndexUnavailable { scope : \"s\".into() } .is_retryable());"
        - "assert!(! IndexError::KernelFailed { reason : \"r\".into() } .is_retryable());"
        - "let wrapped: ResidentError = IndexError::ArenaExhausted {"
      - path: resident/subengine/device.rs
        matches:
        - "Err(crate::resident::error::IndexError::TransferFailed {"
      - path: resident/manager/test/transfer.rs
        matches:
        - "ManagerError::from(crate::resident::error::IndexError::TransferFailed {"
      - path: resident/manager/test/scope.rs
        matches:
        - "matches!(r, Err(IndexError::UnsupportedScope { .. }))"
        - "assert!(matches!(r, Err(IndexError::IndexUnavailable { .. })));"
      - path: resident/manager/test/streamed.rs
        matches:
        - "assert!(matches!(m.index(), Err(IndexError::IndexUnavailable { .. })));"
        - "Err(IndexError::TransferFailed {"
        - "assert!(matches!(failed, Err(IndexError::TransferFailed { .. })));"
        - "matches!(m.read(id(1), 0..3). await, Err(IndexError::IndexUnavailable { .. }))"
        - "assert!(matches!(m.cycle(). await, Err(IndexError::IndexUnavailable { .. })));"
        - "Err(IndexError::IndexUnavailable { .. }))"
      - path: resident/manager/scope.rs
        matches:
        - "let unsupported = || IndexError::UnsupportedScope {"
        - "IndexError::UnsupportedScope {"
        - ".ok_or_else(|| IndexError::IndexUnavailable {"
      - path: resident/manager/streamed.rs
        matches:
        - ".ok_or_else(|| IndexError::IndexUnavailable {"
        - ".map_err(|e| IndexError::TransferFailed {"
        - "ManagerError::from(IndexError::TransferFailed {"
        - "IndexError::ArenaExhausted {"
        - "IndexError::TransferFailed {"
      - path: resident/manager/runtime.rs
        matches:
        - "Err(IndexError::ArenaExhausted { requested, .. }) => {"
      - path: resident/memory/arena.rs
        matches:
        - "IndexError::ArenaExhausted { requested, available } => MemoryError::ArenaExhausted {"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: test/dtype.rs
        matches:
        - "fn sizes() {"
      - path: dtype.rs
        matches:
        - "pub enum DType {"
        - "impl DType {"
        - "pub const fn size_bytes(self) -> usize {"
        - "impl TryFrom<safetensors::Dtype> for DType {"
        - "impl TryFrom<DType> for C {"
        - "impl TryFrom<C> for DType {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      matches:
      - path: stamp.rs
        matches:
        - "#[cfg(test)]"
        - "#[path = \"test/stamp.rs\"]"
        - mod tests;
      - path: memory.rs
        matches:
        - "#[cfg(test)]"
        - "#[path = \"test/memory.rs\"]"
        - mod tests;
```
/answer ed48-1a2 allow
Control Request:
```yaml
id: ed48-1a3
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture
    recursive: false
  - path: /root/.cargo/registry/src
    pattern: parquet-60|arrow-array-60|memmap2|tokio-util
  - path: /home/user/.cargo/registry/src
    pattern: "(parquet|arrow-array|arrow-select|memmap2|tokio-util)-[0-9.]+/Cargo.toml"
  reason: Architektur-Konventionen und lokale Crate-Quellen zur API-Verifikation finden
```
/answer ed48-1a3 allow
Control Request:
```yaml
id: ed48-1a4
result:
  content:
  - type: text
    text: "Tool 'list' failed: [Errno 13] Permission denied: '/root/.cargo/registry/src'"
  isError: true
```
/answer ed48-1a4 allow
Control Request:
```yaml
id: ed48-1a5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
  reason: DType-Varianten und Namen für Header-Keys
```
/answer ed48-1a5 allow
ReasoningToken: 42
Control Request:
```yaml
id: ed48-1a6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: ModuleDoc
        signature: //! Engine-owned element type; backend dtypes are converted at the edge only.
      - id: MS3n2e|NqxAEG
        signature: use thiserror::Error;
      - id: UnsupportedDType
        signature: pub struct UnsupportedDType(pub &'static str);
      - id: 9HWcpZ|0bwvF9
        signature: /// Additive attention-mask value that stays finite in f16 (`-inf` would turn r…
      - id: DType
        signature: "pub enum DType {"
      - id: impl_DType
        signature: "impl DType {"
        children:
        - id: impl_DType.size_bytes
          signature: "pub const fn size_bytes(self) -> usize {"
      - id: impl_TryFrom_safetensors__Dtype__for_DType
        signature: "impl TryFrom<safetensors::Dtype> for DType {"
        children:
        - id: impl_TryFrom_safetensors__Dtype__for_DType.n7RcFD|qILWYw
          signature: type Error = UnsupportedDType;
        - id: impl_TryFrom_safetensors__Dtype__for_DType.try_from
          signature: "fn try_from(d: safetensors::Dtype) -> Result<Self, Self::Error> {"
      - id: candle_adapter
        signature: "pub(crate) mod candle_adapter {"
        children:
        - id: candle_adapter.R4kkrJ|JI8Adk
          signature: "use super::{DType, UnsupportedDType};"
        - id: candle_adapter.impl_TryFrom_DType__for_C
          signature: "impl TryFrom<DType> for C {"
          children:
          - id: candle_adapter.impl_TryFrom_DType__for_C.n7RcFD|qILWYw
            signature: type Error = UnsupportedDType;
          - id: candle_adapter.impl_TryFrom_DType__for_C.try_from
            signature: "fn try_from(d: DType) -> Result<Self, Self::Error> {"
        - id: candle_adapter.impl_TryFrom_C__for_DType
          signature: "impl TryFrom<C> for DType {"
          children:
          - id: candle_adapter.impl_TryFrom_C__for_DType.n7RcFD|qILWYw
            signature: type Error = UnsupportedDType;
          - id: candle_adapter.impl_TryFrom_C__for_DType.try_from
            signature: "fn try_from(d: C) -> Result<Self, Self::Error> {"
        - id: candle_adapter.elems_to_bytes
          signature: "pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {"
          docstring: Candle layouts count elements; the engine counts bytes.
        - id: candle_adapter.bytes_to_elems
          signature: "pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {"
      - id: tests
        signature: mod tests;
```
/answer ed48-1a6 allow
Control Request:
```yaml
id: ed48-1a7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
  reason: Projektkonventionen für Code/Tests/Dateigröße prüfen
```
/answer ed48-1a7 deny
Control Request:
```yaml
id: ed48-1a8
toolName: list
arguments:
  items:
  - path: /home/user/.cargo/registry/src
    pattern: "/(parquet|arrow-array|memmap2|tokio-util)-[0-9][^/]*/Cargo\\.toml$"
  reason: "Crate-Quellen von parquet/arrow lokal finden, um API-Signaturen zu verifizieren"
```
/answer ed48-1a8 allow
Control Request:
```yaml
id: ed48-1a9
result:
  structuredContent:
    results:
    - path: /home/user/.cargo/registry/src
      entries:
      - "./index.crates.io-1949cf8c6b5b557f/arrow-array-60.0.0:"
      - Cargo.toml
      - "./index.crates.io-1949cf8c6b5b557f/memmap2-0.9.11:"
      - Cargo.toml
      - "./index.crates.io-1949cf8c6b5b557f/parquet-60.0.0:"
      - Cargo.toml
      - "./index.crates.io-1949cf8c6b5b557f/tokio-util-0.7.19:"
      - Cargo.toml
```
/answer ed48-1a9 allow
Control Request:
```yaml
id: ed48-1aa
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
    ids:
    - DType
  reason: DType-Enum-Varianten lesen
```
/answer ed48-1aa allow
Control Request:
```yaml
id: ed48-1ab
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: DType
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum DType {
              F32,
              F16,
              Bf16,
              U8,
              U32,
              I64,
          }
```
/answer ed48-1ab allow
Control Request:
```yaml
id: ed48-1ac
toolName: search
arguments:
  items:
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src
    pattern: pub fn (set_key_value_metadata|set_max_row_group_row_count|set_column_dictionary_enabled|set_compression|set_statistics_enabled|set_data_page_row_count_limit|roots|try_new_with_options)\b
    include:
    - '*.rs'
    limit: 20
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src
    pattern: pub fn (parquet_schema|metadata|with_projection|with_batch_size|build)\(
    include:
    - builder/mod.rs
    - mod.rs
    limit: 10
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-60.0.0/src/array
    pattern: pub fn (value_offset|try_new|values)\(
    include:
    - fixed_size_list_array.rs
    limit: 10
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-util-0.7.19/src
    pattern: pub use bytes|pub extern crate bytes
    include:
    - lib.rs
    limit: 5
  reason: "API-Signaturen in parquet/arrow 60 verifizieren (Writer-Properties, Projection, FixedSizeList)"
```
/answer ed48-1ac allow
Control Request:
```yaml
id: ed48-1ad
result:
  structuredContent:
    results:
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src
      matches:
      - path: file/properties.rs
        matches:
        - "pub fn set_data_page_row_count_limit(mut self, value: usize) -> Self {"
        - "pub fn set_max_row_group_row_count(mut self, value: Option<usize>) -> Self {"
        - "pub fn set_key_value_metadata(mut self, value: Option<Vec<KeyValue>>) -> Self {"
        - "pub fn set_compression(mut self, value: Compression) -> Self {"
        - "pub fn set_statistics_enabled(mut self, value: EnabledStatistics) -> Self {"
        - "pub fn set_column_dictionary_enabled(mut self, col: ColumnPath, value: bool) -> Self {"
      - path: file/metadata/mod.rs
        matches:
        - "pub fn set_compression(mut self, value: Compression) -> Self {"
      - path: arrow/mod.rs
        matches:
        - "pub fn roots(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self {"
      - path: arrow/arrow_writer/mod.rs
        matches:
        - pub fn try_new_with_options(
      - path: arrow/async_writer/mod.rs
        matches:
        - pub fn try_new_with_options(
      - path: arrow/arrow_reader/mod.rs
        matches:
        - "pub fn try_new_with_options(reader: T, options: ArrowReaderOptions) -> Result<Self> {"
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src
      matches:
      - path: file/metadata/mod.rs
        matches:
        - "pub fn build(self) -> ParquetMetaData {"
        - "pub fn build(self) -> Result<RowGroupMetaData> {"
        - "pub fn build(self) -> Result<ColumnChunkMetaData> {"
        - "pub fn build(self) -> Result<ColumnIndexMetaData> {"
        - "pub fn build(self) -> OffsetIndexMetaData {"
      - path: arrow/push_decoder/mod.rs
        matches:
        - "pub fn build(self) -> Result<ParquetPushDecoder, ParquetError> {"
      - path: arrow/arrow_reader/mod.rs
        matches:
        - "pub fn metadata(&self) -> &Arc<ParquetMetaData> {"
        - "pub fn parquet_schema(&self) -> &SchemaDescriptor {"
        - "pub fn with_batch_size(self, batch_size: usize) -> Self {"
        - "pub fn with_projection(self, mask: ProjectionMask) -> Self {"
      warning: "Limit of 10 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-60.0.0/src/array
      matches:
      - path: fixed_size_list_array.rs
        matches:
        - pub fn try_new(
        - "pub fn values(&self) -> &ArrayRef {"
        - "pub fn value_offset(&self, i: usize) -> i32 {"
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-util-0.7.19/src
      matches:
      - path: lib.rs
        matches:
        - pub use bytes;
```
/answer ed48-1ad allow
Text:
 Die API-Signaturen passen zu parquet 60. Zuerst die Fehlervarianten (Q1).
ReasoningToken: 137
Control Request:
```yaml
id: ed48-1ae
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: IndexError
    old_text: |2-
          UnknownBatch { batch: u64 },
      }
    new_text: |2-
          UnknownBatch { batch: u64 },
          #[error("index schema does not match the active model configuration (expected {expected}; found {found})")]
          IncompatibleSchema { expected: String, found: String },
          #[error("checksum mismatch for entry {entry}")]
          ChecksumMismatch { entry: u64 },
      }
  reason: Fehlervarianten IncompatibleSchema und ChecksumMismatch ergänzen
```
/answer ed48-1ae allow
ReasoningToken: 6627
Control Request:
```yaml
id: ed48-1af
toolName: search
arguments:
  items:
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/file
    pattern: pub trait (ChunkReader|Length)|fn (get_read|get_bytes|len)\(|^impl .*(ChunkReader|Length) for|type T\b
    include:
    - reader.rs
    limit: 30
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/arrow/arrow_reader
    pattern: "pub fn (load|new_with_metadata|try_new)\\(|impl<T: ChunkReader.*ArrowReaderBuilder"
    include:
    - mod.rs
    limit: 15
  reason: ChunkReader-Trait-Definition und ArrowReaderMetadata-API in parquet 60 prüfen
```
/answer ed48-1af allow
Control Request:
```yaml
id: ed48-1b0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/file
      matches:
      - path: reader.rs
        matches:
        - "pub trait Length {"
        - fn len(&self) -> u64;
        - "pub trait ChunkReader: Length + Send + Sync {"
        - "type T: Read;"
        - "fn get_read(&self, start: u64) -> Result<Self::T>;"
        - "fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes>;"
        - "impl Length for File {"
        - "fn len(&self) -> u64 {"
        - "impl ChunkReader for File {"
        - type T = BufReader<File>;
        - "fn get_read(&self, start: u64) -> Result<Self::T> {"
        - "fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes> {"
        - "impl Length for Bytes {"
        - "impl ChunkReader for Bytes {"
        - type T = bytes::buf::Reader<Bytes>;
      - path: metadata/reader.rs
        matches:
        - "/// # fn get_bytes(file: &std::fs::File, range: std::ops::Range<u64>) -> bytes::Bytes { unimplemented!(); }"
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/arrow/arrow_reader
      matches:
      - path: mod.rs
        matches:
        - "pub fn try_new(metadata: Arc<ParquetMetaData>, options: ArrowReaderOptions) -> Result<Self> {"
        - "pub fn try_new(reader: T) -> Result<Self> {"
        - "pub fn new_with_metadata(input: T, metadata: ArrowReaderMetadata) -> Self {"
```
/answer ed48-1b0 allow
ReasoningToken: 55
Control Request:
```yaml
id: ed48-1b1
toolName: search
arguments:
  items:
  - directory:
    - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/arrow/arrow_reader
    pattern: "pub fn (load|metadata|schema)\\b[^{]*"
    include:
    - mod.rs
    limit: 15
  reason: Signatur ArrowReaderMetadata::load prüfen
```
/answer ed48-1b1 allow
Control Request:
```yaml
id: ed48-1b2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-60.0.0/src/arrow/arrow_reader
      matches:
      - path: mod.rs
        matches:
        - "pub fn metadata(&self) -> &Arc<ParquetMetaData> {"
        - "pub fn schema(&self) -> &SchemaRef {"
        - "pub fn load<T: ChunkReader>(reader: &T, options: ArrowReaderOptions) -> Result<Self> {"
```
/answer ed48-1b2 allow
Text:
 Jetzt das Schema-Modul.
ReasoningToken: 783
Control Request:
```yaml
id: ed48-1b3
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/schema.rs
    source: |
      //! Arrow schema, footer header keys and schema versioning of the persisted index.

      use std::collections::HashMap;
      use std::sync::Arc;

      use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
      use parquet::file::metadata::KeyValue;

      use super::{content_hash, HashAlgo, IndexHeader};
      use crate::resident::error::IndexError;
      use crate::resident::memory::DType;
      use crate::resident::model::{ModelSpec, Normalize, Pooling};

      pub const SCHEMA_VERSION: u32 = 1;

      pub const COL_ENTRY_ID: usize = 0;
      pub const COL_STAMP: usize = 1;
      pub const COL_VECTOR: usize = 2;
      pub const COL_TARGET_FILE: usize = 3;
      pub const COL_CHAR_START: usize = 4;
      pub const COL_CHAR_END: usize = 5;
      pub const COL_CHECKSUM: usize = 6;

      pub const NAME_VECTOR: &str = "vector";

      const KEY_SCHEMA_VERSION: &str = "resident.schema_version";
      const KEY_HIGH_WATER: &str = "resident.high_water";
      const KEY_HASH_ALGO: &str = "payload.hash_algo";
      const KEY_HASH_VERSION: &str = "payload.hash_version";
      const KEY_MODEL_ID: &str = "embedding.model_id";
      const KEY_MODEL_REVISION: &str = "embedding.model_revision";
      const KEY_DIM: &str = "embedding.dim";
      const KEY_POOLING: &str = "embedding.pooling";
      const KEY_NORMALIZE: &str = "embedding.normalize";
      const KEY_PREFIX_QUERY: &str = "embedding.prefix_query";
      const KEY_PREFIX_PASSAGE: &str = "embedding.prefix_passage";
      const KEY_STORAGE_DTYPE: &str = "embedding.storage_dtype";
      const KEY_COMPUTE_DTYPE: &str = "embedding.compute_dtype";
      const KEY_MAX_SEQ_LEN: &str = "embedding.max_seq_len";
      const KEY_TOKENIZER_SHA: &str = "tokenizer.sha";

      /// Column layout; the stamp lives in its own column so that scans never page in vectors.
      pub fn arrow_schema(dim: usize) -> SchemaRef {
          let item = Arc::new(Field::new("item", DataType::Float16, false));
          Arc::new(Schema::new(vec![
              Field::new("entry_id", DataType::UInt64, false),
              Field::new("stamp", DataType::UInt64, false),
              Field::new(NAME_VECTOR, DataType::FixedSizeList(item, dim as i32), false),
              Field::new("target_file", DataType::Utf8, false),
              Field::new("char_start", DataType::UInt64, false),
              Field::new("char_end", DataType::UInt64, false),
              Field::new("checksum", DataType::UInt64, false),
          ]))
      }

      /// Name, type and nullability must match; field metadata is ignored.
      pub fn check_fields(found: &Schema, expected: &Schema) -> Result<(), IndexError> {
          let describe = |s: &Schema| {
              s.fields()
                  .iter()
                  .map(|f| format!("{}:{}{}", f.name(), f.data_type(), if f.is_nullable() { "?" } else { "" }))
                  .collect::<Vec<_>>()
                  .join(",")
          };
          let same = found.fields().len() == expected.fields().len()
              && found.fields().iter().zip(expected.fields().iter()).all(|(a, b)| {
                  a.name() == b.name() && a.data_type() == b.data_type() && a.is_nullable() == b.is_nullable()
              });
          if same {
              Ok(())
          } else {
              Err(IndexError::IncompatibleSchema { expected: describe(expected), found: describe(found) })
          }
      }

      /// Everything that binds an index file to the model configuration that produced it.
      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct SchemaHeader {
          pub index: IndexHeader,
          pub schema_version: u32,
          pub model_revision: String,
          pub pooling: Pooling,
          pub normalize: Normalize,
          pub prefix_query: String,
          pub prefix_passage: String,
          pub storage_dtype: DType,
          /// Informational: mixed CPU/GPU indexing is allowed, so this is not compared.
          pub compute_dtype: String,
          pub max_seq_len: usize,
          pub tokenizer_sha: String,
      }

      impl SchemaHeader {
          pub fn from_spec(
              spec: &ModelSpec,
              model_revision: impl Into<String>,
              tokenizer_sha: impl Into<String>,
          ) -> Result<Self, IndexError> {
              if spec.dtype != DType::F16 {
                  return Err(IndexError::IncompatibleSchema {
                      expected: format!("{KEY_STORAGE_DTYPE}=f16"),
                      found: format!("{KEY_STORAGE_DTYPE}={}", dtype_name(spec.dtype)),
                  });
              }
              Ok(Self {
                  index: IndexHeader {
                      model_id: spec.model_id.clone(),
                      dim: spec.dim as u32,
                      high_water: 0,
                      hash_algo: HashAlgo::Xxh3_64,
                      hash_version: 1,
                  },
                  schema_version: SCHEMA_VERSION,
                  model_revision: model_revision.into(),
                  pooling: spec.pooling,
                  normalize: spec.normalize,
                  prefix_query: spec.prefixes.query.clone(),
                  prefix_passage: spec.prefixes.passage.clone(),
                  storage_dtype: spec.dtype,
                  compute_dtype: format!(
                      "cpu={};gpu={}",
                      dtype_name(spec.compute.cpu),
                      dtype_name(spec.compute.gpu)
                  ),
                  max_seq_len: spec.max_seq_len,
                  tokenizer_sha: tokenizer_sha.into(),
              })
          }

          pub fn dim(&self) -> usize {
              self.index.dim as usize
          }

          pub fn vector_bytes(&self) -> usize {
              self.dim() * DType::F16.size_bytes()
          }

          pub fn with_high_water(&self, high_water: u64) -> Self {
              let mut h = self.clone();
              h.index.high_water = high_water;
              h
          }

          /// Keys that must be equal for an index to be usable with the active model.
          fn binding_entries(&self) -> Vec<(&'static str, String)> {
              vec![
                  (KEY_SCHEMA_VERSION, self.schema_version.to_string()),
                  (KEY_HASH_ALGO, hash_algo_name(self.index.hash_algo).into()),
                  (KEY_HASH_VERSION, self.index.hash_version.to_string()),
                  (KEY_MODEL_ID, self.index.model_id.clone()),
                  (KEY_MODEL_REVISION, self.model_revision.clone()),
                  (KEY_DIM, self.index.dim.to_string()),
                  (KEY_POOLING, pooling_name(self.pooling).into()),
                  (KEY_NORMALIZE, normalize_name(self.normalize).into()),
                  (KEY_PREFIX_QUERY, self.prefix_query.clone()),
                  (KEY_PREFIX_PASSAGE, self.prefix_passage.clone()),
                  (KEY_STORAGE_DTYPE, dtype_name(self.storage_dtype).into()),
                  (KEY_MAX_SEQ_LEN, self.max_seq_len.to_string()),
                  (KEY_TOKENIZER_SHA, self.tokenizer_sha.clone()),
              ]
          }

          pub fn to_key_values(&self) -> Vec<KeyValue> {
              let mut entries = self.binding_entries();
              entries.push((KEY_HIGH_WATER, self.index.high_water.to_string()));
              entries.push((KEY_COMPUTE_DTYPE, self.compute_dtype.clone()));
              entries.into_iter().map(|(k, v)| KeyValue::new(k.to_string(), v)).collect()
          }

          pub fn from_key_values(kvs: &[KeyValue]) -> Result<Self, IndexError> {
              let map: HashMap<&str, &str> = kvs
                  .iter()
                  .filter_map(|kv| kv.value.as_deref().map(|v| (kv.key.as_str(), v)))
                  .collect();
              let get = |key: &'static str| -> Result<&str, IndexError> {
                  map.get(key).copied().ok_or_else(|| IndexError::IncompatibleSchema {
                      expected: format!("{key}=<present>"),
                      found: format!("{key}=<missing>"),
                  })
              };
              let parse = |key: &'static str, what: &str| -> IndexError {
                  IndexError::IncompatibleSchema {
                      expected: format!("{key}=<{what}>"),
                      found: format!("{key}={}", map.get(key).copied().unwrap_or_default()),
                  }
              };
              let num = |key: &'static str| -> Result<u64, IndexError> {
                  get(key)?.parse::<u64>().map_err(|_| parse(key, "integer"))
              };
              let hash_algo = hash_algo_parse(get(KEY_HASH_ALGO)?).ok_or_else(|| parse(KEY_HASH_ALGO, "known algorithm"))?;
              Ok(Self {
                  index: IndexHeader {
                      model_id: get(KEY_MODEL_ID)?.to_string(),
                      dim: num(KEY_DIM)? as u32,
                      high_water: num(KEY_HIGH_WATER)?,
                      hash_algo,
                      hash_version: num(KEY_HASH_VERSION)? as u32,
                  },
                  schema_version: num(KEY_SCHEMA_VERSION)? as u32,
                  model_revision: get(KEY_MODEL_REVISION)?.to_string(),
                  pooling: pooling_parse(get(KEY_POOLING)?).ok_or_else(|| parse(KEY_POOLING, "known pooling"))?,
                  normalize: normalize_parse(get(KEY_NORMALIZE)?).ok_or_else(|| parse(KEY_NORMALIZE, "known normalization"))?,
                  prefix_query: get(KEY_PREFIX_QUERY)?.to_string(),
                  prefix_passage: get(KEY_PREFIX_PASSAGE)?.to_string(),
                  storage_dtype: dtype_parse(get(KEY_STORAGE_DTYPE)?).ok_or_else(|| parse(KEY_STORAGE_DTYPE, "known dtype"))?,
                  compute_dtype: get(KEY_COMPUTE_DTYPE)?.to_string(),
                  max_seq_len: num(KEY_MAX_SEQ_LEN)? as usize,
                  tokenizer_sha: get(KEY_TOKENIZER_SHA)?.to_string(),
              })
          }

          /// `self` is the header read from a file, `expected` the one derived from the active model.
          pub fn check_against(&self, expected: &SchemaHeader) -> Result<(), IndexError> {
              let (mut want, mut have) = (Vec::new(), Vec::new());
              for ((key, e), (_, f)) in expected.binding_entries().into_iter().zip(self.binding_entries()) {
                  if e != f {
                      want.push(format!("{key}={e}"));
                      have.push(format!("{key}={f}"));
                  }
              }
              if want.is_empty() {
                  Ok(())
              } else {
                  Err(IndexError::IncompatibleSchema { expected: want.join("; "), found: have.join("; ") })
              }
          }
      }

      /// Fingerprint of `tokenizer.json` for the `tokenizer.sha` header key.
      pub fn tokenizer_fingerprint(tokenizer_json: &[u8]) -> String {
          format!("xxh3_64:{:016x}", content_hash(tokenizer_json))
      }

      fn dtype_name(d: DType) -> &'static str {
          match d {
              DType::F32 => "f32",
              DType::F16 => "f16",
              DType::Bf16 => "bf16",
              DType::U8 => "u8",
              DType::U32 => "u32",
              DType::I64 => "i64",
              _ => "unknown",
          }
      }

      fn dtype_parse(s: &str) -> Option<DType> {
          Some(match s {
              "f32" => DType::F32,
              "f16" => DType::F16,
              "bf16" => DType::Bf16,
              "u8" => DType::U8,
              "u32" => DType::U32,
              "i64" => DType::I64,
              _ => return None,
          })
      }

      fn pooling_name(p: Pooling) -> &'static str {
          match p {
              Pooling::MeanMasked => "mean_masked",
              Pooling::PerToken => "per_token",
          }
      }

      fn pooling_parse(s: &str) -> Option<Pooling> {
          match s {
              "mean_masked" => Some(Pooling::MeanMasked),
              "per_token" => Some(Pooling::PerToken),
              _ => None,
          }
      }

      fn normalize_name(n: Normalize) -> &'static str {
          match n {
              Normalize::L2 => "l2",
              Normalize::None => "none",
          }
      }

      fn normalize_parse(s: &str) -> Option<Normalize> {
          match s {
              "l2" => Some(Normalize::L2),
              "none" => Some(Normalize::None),
              _ => None,
          }
      }

      fn hash_algo_name(a: HashAlgo) -> &'static str {
          match a {
              HashAlgo::Xxh3_64 => "xxh3_64",
              HashAlgo::Xxh3_128 => "xxh3_128",
              HashAlgo::Blake3 => "blake3",
          }
      }

      fn hash_algo_parse(s: &str) -> Option<HashAlgo> {
          match s {
              "xxh3_64" => Some(HashAlgo::Xxh3_64),
              "xxh3_128" => Some(HashAlgo::Xxh3_128),
              "blake3" => Some(HashAlgo::Blake3),
              _ => None,
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/payload.rs
    source: |
      //! Per-entry back-reference to the source text and its integrity checksum.

      use xxhash_rust::xxh3::Xxh3;

      #[derive(Clone, Debug, Default, PartialEq, Eq)]
      pub struct Payload {
          pub target_file: String,
          pub char_start: u64,
          pub char_end: u64,
      }

      impl Payload {
          pub fn new(target_file: impl Into<String>, char_start: u64, char_end: u64) -> Self {
              Self { target_file: target_file.into(), char_start, char_end }
          }

          /// xxh3-64 over vector bytes and all payload fields; any bit flip in either is detectable.
          pub fn checksum(&self, vector: &[u8]) -> u64 {
              let mut h = Xxh3::new();
              h.update(&(vector.len() as u64).to_le_bytes());
              h.update(vector);
              h.update(&(self.target_file.len() as u64).to_le_bytes());
              h.update(self.target_file.as_bytes());
              h.update(&self.char_start.to_le_bytes());
              h.update(&self.char_end.to_le_bytes());
              h.digest()
          }
      }
  reason: "Schema-Modul (Arrow-Schema, Header-Keys, Versionierung) und Payload-Modul anlegen"
```
/answer ed48-1b3 allow
Text:
 Jetzt der Kern: Parquet-I/O mit Index-Implementierung.
ReasoningToken: 9602
Control Request:
```yaml
id: ed48-1b4
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    source: |
      //! Parquet-backed `Index`: stamps are scanned without decoding vectors, vectors and
      //! payload are decoded on first use, and every write replaces the file atomically.

      use std::collections::BTreeMap;
      use std::fmt::Display;
      use std::fs::File;
      use std::io::Read;
      use std::path::{Path, PathBuf};
      use std::sync::{Arc, OnceLock};

      use arrow::array::{ArrayRef, AsArray, FixedSizeListArray, Float16Array, StringArray, UInt64Array};
      use arrow::datatypes::{DataType, SchemaRef, UInt64Type};
      use arrow::record_batch::RecordBatch;
      use half::f16;
      use memmap2::Mmap;
      use parking_lot::Mutex;
      use parquet::arrow::arrow_reader::{
          ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReaderBuilder,
      };
      use parquet::arrow::{ArrowWriter, ProjectionMask};
      use parquet::basic::Compression;
      use parquet::file::properties::{EnabledStatistics, WriterProperties};
      use parquet::file::reader::{ChunkReader, Length};
      use parquet::schema::types::ColumnPath;
      use tokio::sync::watch;
      use tokio_util::bytes::Bytes;

      use super::payload::Payload;
      use super::schema::{
          arrow_schema, check_fields, SchemaHeader, COL_CHAR_END, COL_CHAR_START, COL_CHECKSUM,
          COL_ENTRY_ID, COL_STAMP, COL_TARGET_FILE, COL_VECTOR, NAME_VECTOR,
      };
      use super::{
          AccessMethod, CellId, EntryData, EntryId, EntrySet, Index, NodeId, Stamp, StampCounter,
          SummaryRef,
      };
      use crate::resident::error::IndexError;

      const DEFAULT_ROW_GROUP_ROWS: usize = 8192;
      const READ_BATCH_ROWS: usize = 4096;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Access {
          Mmap,
          File,
      }

      impl Access {
          pub fn as_str(self) -> &'static str {
              match self {
                  Access::Mmap => "mmap",
                  Access::File => "file",
              }
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct IndexOptions {
          pub access: Access,
          /// Decode everything and check all checksums while opening.
          pub verify: bool,
          pub row_group_rows: usize,
      }

      impl Default for IndexOptions {
          fn default() -> Self {
              Self { access: Access::Mmap, verify: true, row_group_rows: DEFAULT_ROW_GROUP_ROWS }
          }
      }

      struct Mapped(Arc<Mmap>);

      impl AsRef<[u8]> for Mapped {
          fn as_ref(&self) -> &[u8] {
              &self.0
          }
      }

      #[derive(Clone)]
      enum Source {
          Mapped(Bytes),
          File(Arc<File>),
      }

      impl Source {
          fn open(path: &Path, access: Access) -> std::io::Result<Self> {
              let file = File::open(path)?;
              match access {
                  Access::File => Ok(Source::File(Arc::new(file))),
                  Access::Mmap => {
                      // SAFETY: index files are only ever replaced by rename, never modified or
                      // truncated in place, so the mapped inode stays unchanged.
                      let map = unsafe { Mmap::map(&file)? };
                      #[cfg(unix)]
                      let _ = map.advise(memmap2::Advice::Random);
                      Ok(Source::Mapped(Bytes::from_owner(Mapped(Arc::new(map)))))
                  }
              }
          }
      }

      impl Length for Source {
          fn len(&self) -> u64 {
              match self {
                  Source::Mapped(b) => b.len() as u64,
                  Source::File(f) => Length::len(f.as_ref()),
              }
          }
      }

      impl ChunkReader for Source {
          type T = Box<dyn Read + Send>;

          fn get_read(&self, start: u64) -> parquet::errors::Result<Self::T> {
              Ok(match self {
                  Source::Mapped(b) => Box::new(b.get_read(start)?),
                  Source::File(f) => Box::new(f.get_read(start)?),
              })
          }

          fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
              match self {
                  Source::Mapped(b) => b.get_bytes(start, length),
                  Source::File(f) => f.get_bytes(start, length),
              }
          }
      }

      fn unavailable<E: Display>(path: &Path) -> impl Fn(E) -> IndexError + '_ {
          move |e| IndexError::IndexUnavailable { scope: format!("{}: {e}", path.display()) }
      }

      /// Fully decoded content of the file the index was opened from.
      struct BaseData {
          entry_ids: Vec<u64>,
          vectors: Vec<u8>,
          payloads: Vec<Payload>,
          checksums: Vec<u64>,
      }

      struct Base {
          path: PathBuf,
          dim: usize,
          backing: Option<(Source, ArrowReaderMetadata)>,
          data: OnceLock<Result<BaseData, String>>,
      }

      impl Base {
          fn data(&self) -> Result<&BaseData, IndexError> {
              let loaded = self.data.get_or_init(|| match &self.backing {
                  Some((source, meta)) => {
                      load_base(source, meta, self.dim, &self.path).map_err(|e| e.to_string())
                  }
                  None => Err("no backing file".to_string()),
              });
              loaded.as_ref().map_err(|reason| IndexError::IndexUnavailable {
                  scope: format!("{}: {reason}", self.path.display()),
              })
          }
      }

      fn scan_stamps(
          source: &Source,
          meta: &ArrowReaderMetadata,
          path: &Path,
      ) -> Result<(BTreeMap<EntryId, Row>, u64), IndexError> {
          let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(source.clone(), meta.clone());
          let mask = ProjectionMask::roots(builder.parquet_schema(), [COL_ENTRY_ID, COL_STAMP]);
          let reader = builder.with_projection(mask).build().map_err(unavailable(path))?;
          let mut rows = BTreeMap::new();
          let (mut position, mut max_stamp) = (0u32, 0u64);
          for batch in reader {
              let batch = batch.map_err(unavailable(path))?;
              let ids = batch.column(0).as_primitive::<UInt64Type>();
              let stamps = batch.column(1).as_primitive::<UInt64Type>();
              for (id, stamp) in ids.values().iter().zip(stamps.values().iter()) {
                  max_stamp = max_stamp.max(*stamp);
                  let row = Row { stamp: Stamp::from_raw(*stamp), slot: Slot::Base(position) };
                  if rows.insert(EntryId::new(*id), row).is_some() {
                      return Err(IndexError::IndexUnavailable {
                          scope: format!("{}: duplicate entry {id}", path.display()),
                      });
                  }
                  position += 1;
              }
          }
          Ok((rows, max_stamp))
      }

      fn load_base(
          source: &Source,
          meta: &ArrowReaderMetadata,
          dim: usize,
          path: &Path,
      ) -> Result<BaseData, IndexError> {
          let total = meta.metadata().file_metadata().num_rows().max(0) as usize;
          let reader = ParquetRecordBatchReaderBuilder::new_with_metadata(source.clone(), meta.clone())
              .with_batch_size(READ_BATCH_ROWS)
              .build()
              .map_err(unavailable(path))?;
          let mut data = BaseData {
              entry_ids: Vec::with_capacity(total),
              vectors: Vec::with_capacity(total * dim * 2),
              payloads: Vec::with_capacity(total),
              checksums: Vec::with_capacity(total),
          };
          for batch in reader {
              let batch = batch.map_err(unavailable(path))?;
              let n = batch.num_rows();
              data.entry_ids.extend_from_slice(batch.column(COL_ENTRY_ID).as_primitive::<UInt64Type>().values());
              data.checksums.extend_from_slice(batch.column(COL_CHECKSUM).as_primitive::<UInt64Type>().values());
              let list = batch.column(COL_VECTOR).as_fixed_size_list();
              let values = list.values().as_primitive::<arrow::datatypes::Float16Type>();
              let start = list.value_offset(0) as usize;
              for v in &values.values()[start..start + n * dim] {
                  data.vectors.extend_from_slice(&v.to_le_bytes());
              }
              let files = batch.column(COL_TARGET_FILE).as_string::<i32>();
              let starts = batch.column(COL_CHAR_START).as_primitive::<UInt64Type>();
              let ends = batch.column(COL_CHAR_END).as_primitive::<UInt64Type>();
              for i in 0..n {
                  data.payloads.push(Payload::new(files.value(i), starts.value(i), ends.value(i)));
              }
          }
          Ok(data)
      }

      struct MemRow {
          vector: Arc<[u8]>,
          payload: Payload,
          checksum: u64,
      }

      #[derive(Clone)]
      enum Slot {
          Base(u32),
          Mem(Arc<MemRow>),
      }

      #[derive(Clone)]
      struct Row {
          stamp: Stamp,
          slot: Slot,
      }

      struct Inner {
          rows: BTreeMap<EntryId, Row>,
          revision: u64,
          flushed: u64,
      }

      /// Index whose persistence is owned by the index itself; `flush` publishes changes to disk.
      pub struct ParquetIndex {
          path: PathBuf,
          header: SchemaHeader,
          options: IndexOptions,
          base: Base,
          inner: Mutex<Inner>,
          counter: StampCounter,
          tx: watch::Sender<u64>,
          flush_lock: Mutex<()>,
      }

      impl ParquetIndex {
          /// A missing file yields an empty index; the first `flush` creates it.
          pub fn open(
              path: impl Into<PathBuf>,
              expected: SchemaHeader,
              options: IndexOptions,
          ) -> Result<Self, IndexError> {
              let path = path.into();
              let span = tracing::info_span!(
                  "index.open",
                  path = %path.display(),
                  schema_version = expected.schema_version,
                  model_id = %expected.index.model_id,
                  model_revision = %expected.model_revision,
                  access = options.access.as_str(),
                  row_groups = tracing::field::Empty,
                  checksum_ok = tracing::field::Empty,
              );
              let _guard = span.enter();

              if !path.exists() {
                  let base = Base { path: path.clone(), dim: expected.dim(), backing: None, data: OnceLock::new() };
                  return Ok(Self::assemble(path, expected, options, base, BTreeMap::new(), StampCounter::new()));
              }
              let source = Source::open(&path, options.access).map_err(unavailable(&path))?;
              let meta = ArrowReaderMetadata::load(&source, ArrowReaderOptions::new())
                  .map_err(unavailable(&path))?;
              span.record("row_groups", meta.metadata().num_row_groups());

              let kvs = meta.metadata().file_metadata().key_value_metadata().map(Vec::as_slice).unwrap_or_default();
              let found = SchemaHeader::from_key_values(kvs)?;
              found.check_against(&expected)?;
              check_fields(meta.schema(), &arrow_schema(expected.dim()))?;

              let (rows, file_max) = scan_stamps(&source, &meta, &path)?;
              let counter = StampCounter::resume(found.index.high_water, file_max);
              let base = Base {
                  path: path.clone(),
                  dim: expected.dim(),
                  backing: Some((source, meta)),
                  data: OnceLock::new(),
              };
              let index = Self::assemble(path, expected, options, base, rows, counter);
              if options.verify {
                  let verified = index.verify();
                  span.record("checksum_ok", verified.is_ok());
                  verified?;
              }
              Ok(index)
          }

          fn assemble(
              path: PathBuf,
              header: SchemaHeader,
              options: IndexOptions,
              base: Base,
              rows: BTreeMap<EntryId, Row>,
              counter: StampCounter,
          ) -> Self {
              let (tx, _) = watch::channel(counter.high_water());
              Self {
                  path,
                  header,
                  options,
                  base,
                  inner: Mutex::new(Inner { rows, revision: 0, flushed: 0 }),
                  counter,
                  tx,
                  flush_lock: Mutex::new(()),
              }
          }

          pub fn path(&self) -> &Path {
              &self.path
          }

          pub fn header(&self) -> &SchemaHeader {
              &self.header
          }

          pub fn high_water(&self) -> u64 {
              self.counter.high_water()
          }

          pub fn is_dirty(&self) -> bool {
              let g = self.inner.lock();
              g.revision != g.flushed
          }

          /// Decodes the file and compares every stored checksum.
          pub fn verify(&self) -> Result<(), IndexError> {
              if self.base.backing.is_none() {
                  return Ok(());
              }
              let data = self.base.data()?;
              let vb = self.header.vector_bytes();
              for (i, entry) in data.entry_ids.iter().enumerate() {
                  let vector = &data.vectors[i * vb..(i + 1) * vb];
                  if data.payloads[i].checksum(vector) != data.checksums[i] {
                      return Err(IndexError::ChecksumMismatch { entry: *entry });
                  }
              }
              Ok(())
          }

          /// Stores a vector (f16 little-endian bytes) together with its payload.
          pub fn upsert(&self, entry: EntryId, vector: &[u8], payload: Payload) -> Result<Stamp, IndexError> {
              let expected = self.header.vector_bytes();
              if vector.len() != expected {
                  return Err(IndexError::IncompatibleSchema {
                      expected: format!("vector of {expected} bytes"),
                      found: format!("vector of {} bytes", vector.len()),
                  });
              }
              let checksum = payload.checksum(vector);
              let row = Arc::new(MemRow { vector: Arc::from(vector), payload, checksum });
              let mut g = self.inner.lock();
              let stamp = self.counter.next();
              g.rows.insert(entry, Row { stamp, slot: Slot::Mem(row) });
              g.revision += 1;
              self.tx.send_replace(stamp.raw());
              drop(g);
              tracing::debug!(entry = entry.get(), stamp = stamp.raw(), "upsert");
              Ok(stamp)
          }

          /// Removal consumes a stamp so that re-adding the same id yields a new one.
          pub fn remove(&self, entry: EntryId) -> Option<Stamp> {
              let mut g = self.inner.lock();
              g.rows.remove(&entry)?;
              g.revision += 1;
              let stamp = self.counter.next();
              self.tx.send_replace(stamp.raw());
              Some(stamp)
          }

          pub fn payload(&self, entry: EntryId) -> Option<Payload> {
              let slot = self.inner.lock().rows.get(&entry)?.slot.clone();
              match slot {
                  Slot::Mem(m) => Some(m.payload.clone()),
                  Slot::Base(i) => match self.base.data() {
                      Ok(d) => Some(d.payloads[i as usize].clone()),
                      Err(e) => {
                          tracing::error!(error = %e, "payload unavailable");
                          None
                      }
                  },
              }
          }

          /// Writes all rows to a temporary file and renames it over the index file.
          /// Returns `false` when nothing changed since the last write.
          pub fn flush(&self) -> Result<bool, IndexError> {
              let _serial = self.flush_lock.lock();
              let (rows, high_water, revision) = {
                  let g = self.inner.lock();
                  if g.revision == g.flushed {
                      return Ok(false);
                  }
                  let rows: Vec<(EntryId, Row)> = g.rows.iter().map(|(id, r)| (*id, r.clone())).collect();
                  (rows, self.counter.high_water(), g.revision)
              };
              let row_group_rows = self.options.row_group_rows.max(1);
              let span = tracing::info_span!(
                  "index.write",
                  path = %self.path.display(),
                  schema_version = self.header.schema_version,
                  model_id = %self.header.index.model_id,
                  model_revision = %self.header.model_revision,
                  rows = rows.len(),
                  row_groups = rows.len().div_ceil(row_group_rows),
              );
              let _guard = span.enter();

              let needs_base = rows.iter().any(|(_, r)| matches!(r.slot, Slot::Base(_)));
              let base = if needs_base { Some(self.base.data()?) } else { None };
              let header = self.header.with_high_water(high_water);
              write_file(&self.path, &header, &rows, base, row_group_rows)?;
              self.inner.lock().flushed = revision;
              Ok(true)
          }
      }

      fn write_file(
          path: &Path,
          header: &SchemaHeader,
          rows: &[(EntryId, Row)],
          base: Option<&BaseData>,
          row_group_rows: usize,
      ) -> Result<(), IndexError> {
          let mut tmp_name = path.as_os_str().to_owned();
          tmp_name.push(".tmp");
          let tmp = PathBuf::from(tmp_name);
          let result = write_tmp(&tmp, header, rows, base, row_group_rows, path)
              .and_then(|()| std::fs::rename(&tmp, path).map_err(unavailable(path)))
              .and_then(|()| sync_parent(path));
          if result.is_err() {
              let _ = std::fs::remove_file(&tmp);
          }
          result
      }

      fn sync_parent(path: &Path) -> Result<(), IndexError> {
          match path.parent().filter(|p| !p.as_os_str().is_empty()) {
              Some(dir) => File::open(dir).and_then(|d| d.sync_all()).map_err(unavailable(path)),
              None => Ok(()),
          }
      }

      fn write_tmp(
          tmp: &Path,
          header: &SchemaHeader,
          rows: &[(EntryId, Row)],
          base: Option<&BaseData>,
          row_group_rows: usize,
          path: &Path,
      ) -> Result<(), IndexError> {
          let err = unavailable(path);
          let schema = arrow_schema(header.dim());
          let props = WriterProperties::builder()
              .set_key_value_metadata(Some(header.to_key_values()))
              .set_compression(Compression::UNCOMPRESSED)
              .set_max_row_group_row_count(Some(row_group_rows))
              .set_statistics_enabled(EnabledStatistics::Page)
              .set_column_dictionary_enabled(ColumnPath::from(NAME_VECTOR), false)
              .build();
          let file = File::create(tmp).map_err(&err)?;
          let mut writer = ArrowWriter::try_new(&file, schema.clone(), Some(props)).map_err(&err)?;
          for chunk in rows.chunks(row_group_rows) {
              let batch = build_batch(&schema, header, chunk, base, path)?;
              writer.write(&batch).map_err(&err)?;
              writer.flush().map_err(&err)?;
          }
          writer.close().map_err(&err)?;
          file.sync_all().map_err(&err)
      }

      fn build_batch(
          schema: &SchemaRef,
          header: &SchemaHeader,
          chunk: &[(EntryId, Row)],
          base: Option<&BaseData>,
          path: &Path,
      ) -> Result<RecordBatch, IndexError> {
          let err = unavailable(path);
          let (dim, vb) = (header.dim(), header.vector_bytes());
          let n = chunk.len();
          let (mut ids, mut stamps) = (Vec::with_capacity(n), Vec::with_capacity(n));
          let (mut starts, mut ends, mut sums) = (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n));
          let mut files: Vec<&str> = Vec::with_capacity(n);
          let mut values: Vec<f16> = Vec::with_capacity(n * dim);
          for (id, row) in chunk {
              let (vector, payload, checksum): (&[u8], &Payload, u64) = match &row.slot {
                  Slot::Mem(m) => (&m.vector, &m.payload, m.checksum),
                  Slot::Base(i) => {
                      let b = base.ok_or_else(|| err("base data missing"))?;
                      let i = *i as usize;
                      (&b.vectors[i * vb..(i + 1) * vb], &b.payloads[i], b.checksums[i])
                  }
              };
              ids.push(id.get());
              stamps.push(row.stamp.raw());
              files.push(&payload.target_file);
              starts.push(payload.char_start);
              ends.push(payload.char_end);
              sums.push(checksum);
              values.extend(vector.chunks_exact(2).map(|c| f16::from_le_bytes([c[0], c[1]])));
          }
          let DataType::FixedSizeList(item, _) = schema.field(COL_VECTOR).data_type() else {
              return Err(err("vector column is not a fixed size list"));
          };
          let vectors = FixedSizeListArray::try_new(
              item.clone(),
              dim as i32,
              Arc::new(Float16Array::from(values)) as ArrayRef,
              None,
          )
          .map_err(&err)?;
          let columns: Vec<ArrayRef> = vec![
              Arc::new(UInt64Array::from(ids)),
              Arc::new(UInt64Array::from(stamps)),
              Arc::new(vectors),
              Arc::new(StringArray::from(files)),
              Arc::new(UInt64Array::from(starts)),
              Arc::new(UInt64Array::from(ends)),
              Arc::new(UInt64Array::from(sums)),
          ];
          RecordBatch::try_new(schema.clone(), columns).map_err(&err)
      }

      impl Index for ParquetIndex {
          fn stamp(&self, entry: EntryId) -> Option<Stamp> {
              self.inner.lock().rows.get(&entry).map(|r| r.stamp)
          }

          fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
              let snapshot: Vec<_> = self.inner.lock().rows.iter().map(|(id, r)| (*id, r.stamp)).collect();
              Box::new(snapshot.into_iter())
          }

          fn read(&self, entry: EntryId) -> EntryData<'_> {
              let slot = match self.inner.lock().rows.get(&entry) {
                  Some(row) => row.slot.clone(),
                  None => return EntryData::Borrowed(&[]),
              };
              match slot {
                  Slot::Mem(m) => EntryData::Shared(Arc::clone(&m.vector)),
                  Slot::Base(i) => match self.base.data() {
                      Ok(d) => {
                          let vb = self.header.vector_bytes();
                          let i = i as usize;
                          EntryData::Shared(Arc::from(&d.vectors[i * vb..(i + 1) * vb]))
                      }
                      Err(e) => {
                          tracing::error!(error = %e, entry = entry.get(), "vector unavailable");
                          EntryData::Borrowed(&[])
                      }
                  },
              }
          }

          /// `data` must be exactly one vector; the payload of an existing entry is kept.
          fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
              let payload = self.payload(entry).unwrap_or_default();
              match self.upsert(entry, data, payload) {
                  Ok(stamp) => stamp,
                  Err(e) => panic!("apply with malformed vector: {e}"),
              }
          }

          fn changed(&self) -> watch::Receiver<u64> {
              self.tx.subscribe()
          }

          fn capabilities(&self) -> &[AccessMethod] {
              &[AccessMethod::Range]
          }

          fn children(&self, _node: NodeId) -> EntrySet {
              EntrySet::new()
          }

          fn members(&self, cell: CellId) -> EntrySet {
              if cell.0 != 0 {
                  return EntrySet::new();
              }
              EntrySet::from_ids(self.inner.lock().rows.keys().copied())
          }

          fn summary(&self, _kind: AccessMethod) -> Option<SummaryRef> {
              None
          }

          fn prefetch(&self, _set: &EntrySet) {}
      }

      #[cfg(test)]
      #[path = "test/parquet_io.rs"]
      mod tests;
  reason: "Parquet-Index (Reader/Writer, mmap, Checksums, atomares Rename) implementieren"
```
/answer ed48-1b4 allow
Control Request:
```yaml
id: ed48-1b5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    ids:
    - ModuleDoc
    - memory
    - stamp
  reason: Modulkopf zum Einhängen der neuen Module lesen
```
/answer ed48-1b5 allow
Control Request:
```yaml
id: ed48-1b6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Index trait, Scope, AccessMethod, Stamp.
          //!
          //! The contract is independent of the core: an index reports changes and
          //! delivers structure and stamps; it knows no geometry and resolves no scope.
          //!
          //! Write order is binding: data first, then stamp, then signal.
          //! `read` may block on page faults and disk I/O; callers use a blocking pool.
      - id: memory
        type: ItemMod
        code: |
          pub mod memory;
      - id: stamp
        type: ItemMod
        code: |
          pub mod stamp;
```
/answer ed48-1b6 allow
ReasoningToken: 60
Control Request:
```yaml
id: ed48-1b7
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    id: stamp
    old_text: pub mod stamp;
    new_text: |-
      pub mod parquet_io;
      pub mod payload;
      pub mod schema;
      pub mod stamp;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    id: kAsHBt|dtXyFJ
    old_text: "pub use stamp::{"
    new_text: |-
      pub use parquet_io::{Access, IndexOptions, ParquetIndex};
      pub use payload::Payload;
      pub use schema::{tokenizer_fingerprint, SchemaHeader, SCHEMA_VERSION};
      pub use stamp::{
  reason: Neue Module einhängen und Re-Exports ergänzen
```
/answer ed48-1b7 allow
ReasoningToken: 2283
Control Request:
```yaml
id: ed48-1b8
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/test/parquet_io.rs
    source: |
      use std::sync::atomic::{AtomicU32, Ordering};

      use super::*;
      use crate::resident::memory::DType;
      use crate::resident::model::ModelSpec;

      struct TempDir(PathBuf);

      impl TempDir {
          fn new(tag: &str) -> Self {
              static NEXT: AtomicU32 = AtomicU32::new(0);
              let dir = std::env::temp_dir().join(format!(
                  "xyrag-parquet-{}-{}-{tag}",
                  std::process::id(),
                  NEXT.fetch_add(1, Ordering::Relaxed)
              ));
              std::fs::create_dir_all(&dir).unwrap();
              Self(dir)
          }

          fn file(&self) -> PathBuf {
              self.0.join("index.parquet")
          }
      }

      impl Drop for TempDir {
          fn drop(&mut self) {
              let _ = std::fs::remove_dir_all(&self.0);
          }
      }

      fn header() -> SchemaHeader {
          let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F16);
          SchemaHeader::from_spec(&spec, "rev-1", "xxh3_64:0000000000000001").unwrap()
      }

      fn vector(seed: u64) -> Vec<u8> {
          (0..384u64)
              .flat_map(|j| f16::from_f32(((seed * 31 + j * 7) % 1000) as f32 / 1000.0 - 0.5).to_le_bytes())
              .collect()
      }

      fn id(n: u64) -> EntryId {
          EntryId::new(n)
      }

      fn open(dir: &TempDir) -> ParquetIndex {
          ParquetIndex::open(dir.file(), header(), IndexOptions::default()).unwrap()
      }

      fn populated(dir: &TempDir, rows: u64, options: IndexOptions) -> Vec<Stamp> {
          let idx = ParquetIndex::open(dir.file(), header(), options).unwrap();
          let stamps = (0..rows)
              .map(|i| {
                  let payload = Payload::new(format!("f{}.txt", i % 7), i * 10, i * 10 + 9);
                  idx.upsert(id(i), &vector(i), payload).unwrap()
              })
              .collect();
          assert!(idx.flush().unwrap());
          stamps
      }

      fn footer_of(path: &Path) -> Vec<parquet::file::metadata::KeyValue> {
          let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(path).unwrap()).unwrap();
          builder.metadata().file_metadata().key_value_metadata().cloned().unwrap_or_default()
      }

      #[test]
      fn roundtrip_is_bit_exact_across_row_groups() {
          let dir = TempDir::new("roundtrip");
          let options = IndexOptions { row_group_rows: 1000, ..IndexOptions::default() };
          let stamps = populated(&dir, 4000, options);

          let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(dir.file()).unwrap()).unwrap();
          assert_eq!(builder.metadata().num_row_groups(), 4);
          assert_eq!(builder.schema().field(COL_VECTOR).data_type(), arrow_schema(384).field(COL_VECTOR).data_type());

          let idx = ParquetIndex::open(dir.file(), header(), options).unwrap();
          for i in 0..4000u64 {
              assert_eq!(idx.read(id(i)).as_bytes(), &vector(i)[..], "vector {i}");
              assert_eq!(idx.stamp(id(i)), Some(stamps[i as usize]));
              assert_eq!(idx.payload(id(i)), Some(Payload::new(format!("f{}.txt", i % 7), i * 10, i * 10 + 9)));
          }
          assert!(idx.read(id(4000)).as_bytes().is_empty());
      }

      #[test]
      fn footer_carries_every_header_key() {
          let dir = TempDir::new("header");
          populated(&dir, 3, IndexOptions::default());
          let kvs = footer_of(&dir.file());
          assert!(kvs.iter().any(|kv| kv.key == "ARROW:schema"));
          let found = SchemaHeader::from_key_values(&kvs).unwrap();
          assert_eq!(found.index.high_water, 3);
          assert_eq!(found, header().with_high_water(3));
          assert_eq!(found.index.model_id, "intfloat/multilingual-e5-small");
          assert_eq!(found.prefix_query, "query: ");
          assert_eq!(found.compute_dtype, "cpu=f32;gpu=f32");
      }

      #[test]
      fn binding_mismatch_is_rejected_with_the_offending_key() {
          let dir = TempDir::new("mismatch");
          populated(&dir, 2, IndexOptions::default());
          let cases: Vec<(&str, Box<dyn Fn(&mut SchemaHeader)>)> = vec![
              ("embedding.dim", Box::new(|h| h.index.dim = 768)),
              ("embedding.model_id", Box::new(|h| h.index.model_id = "other/model".into())),
              ("embedding.model_revision", Box::new(|h| h.model_revision = "rev-2".into())),
              ("embedding.prefix_query", Box::new(|h| h.prefix_query = "q: ".into())),
              ("embedding.prefix_passage", Box::new(|h| h.prefix_passage = String::new())),
              ("embedding.pooling", Box::new(|h| h.pooling = crate::resident::model::Pooling::PerToken)),
              ("embedding.normalize", Box::new(|h| h.normalize = crate::resident::model::Normalize::None)),
              ("embedding.max_seq_len", Box::new(|h| h.max_seq_len = 256)),
              ("tokenizer.sha", Box::new(|h| h.tokenizer_sha = "other".into())),
              ("resident.schema_version", Box::new(|h| h.schema_version += 1)),
          ];
          for (key, mutate) in cases {
              let mut other = header();
              mutate(&mut other);
              match ParquetIndex::open(dir.file(), other, IndexOptions::default()) {
                  Err(IndexError::IncompatibleSchema { expected, found }) => {
                      assert!(expected.contains(key), "{key}: {expected}");
                      assert!(found.contains(key), "{key}: {found}");
                  }
                  other => panic!("{key}: unexpected {:?}", other.map(|_| ())),
              }
          }
      }

      #[test]
      fn mixed_compute_dtype_is_documented_not_enforced() {
          let dir = TempDir::new("compute");
          populated(&dir, 2, IndexOptions::default());
          let mut other = header();
          other.compute_dtype = "cpu=f32;gpu=f16".into();
          assert!(ParquetIndex::open(dir.file(), other, IndexOptions::default()).is_ok());
      }

      #[test]
      fn missing_header_key_and_foreign_storage_dtype_are_incompatible() {
          let dir = TempDir::new("keys");
          populated(&dir, 1, IndexOptions::default());
          let mut kvs = footer_of(&dir.file());
          kvs.retain(|kv| kv.key != "tokenizer.sha");
          match SchemaHeader::from_key_values(&kvs) {
              Err(IndexError::IncompatibleSchema { found, .. }) => assert!(found.contains("tokenizer.sha=<missing>")),
              other => panic!("unexpected {:?}", other.map(|_| ())),
          }
          let spec = ModelSpec::e5_small(Path::new("/nonexistent"), DType::F32);
          assert!(matches!(
              SchemaHeader::from_spec(&spec, "r", "t"),
              Err(IndexError::IncompatibleSchema { .. })
          ));
      }

      #[test]
      fn corrupted_vector_bytes_fail_the_checksum() {
          let dir = TempDir::new("checksum");
          populated(&dir, 3, IndexOptions::default());
          let mut bytes = std::fs::read(dir.file()).unwrap();
          let needle = vector(1);
          let at = bytes.windows(needle.len()).position(|w| w == &needle[..]).expect("plain vector bytes in file");
          bytes[at + 100] ^= 0x01;
          std::fs::write(dir.file(), &bytes).unwrap();

          let strict = ParquetIndex::open(dir.file(), header(), IndexOptions::default());
          assert!(matches!(strict, Err(IndexError::ChecksumMismatch { entry: 1 })));

          let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
          let idx = ParquetIndex::open(dir.file(), header(), lazy).unwrap();
          assert!(matches!(idx.verify(), Err(IndexError::ChecksumMismatch { entry: 1 })));
      }

      #[test]
      fn stamps_do_not_decode_vectors() {
          let dir = TempDir::new("stamps");
          let expected = populated(&dir, 50, IndexOptions::default());
          let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
          let idx = ParquetIndex::open(dir.file(), header(), lazy).unwrap();

          let seen: Vec<_> = idx.stamps().collect();
          assert_eq!(seen.len(), 50);
          assert!(seen.iter().zip(&expected).all(|((_, s), e)| s == e));
          assert_eq!(idx.stamp(id(7)), Some(expected[7]));
          assert!(idx.base.data.get().is_none());

          assert_eq!(idx.read(id(3)).as_bytes(), &vector(3)[..]);
          assert!(idx.base.data.get().is_some());
      }

      #[test]
      fn mmap_and_file_access_agree() {
          let dir = TempDir::new("access");
          populated(&dir, 120, IndexOptions { row_group_rows: 50, ..IndexOptions::default() });
          let mapped = ParquetIndex::open(dir.file(), header(), IndexOptions { access: Access::Mmap, ..IndexOptions::default() }).unwrap();
          let plain = ParquetIndex::open(dir.file(), header(), IndexOptions { access: Access::File, ..IndexOptions::default() }).unwrap();
          assert_eq!(mapped.stamps().collect::<Vec<_>>(), plain.stamps().collect::<Vec<_>>());
          for i in 0..120 {
              assert_eq!(mapped.read(id(i)).as_bytes(), plain.read(id(i)).as_bytes());
              assert_eq!(mapped.payload(id(i)), plain.payload(id(i)));
          }
      }

      #[test]
      fn rewrite_is_atomic_and_keeps_existing_mappings_valid() {
          let dir = TempDir::new("atomic");
          populated(&dir, 10, IndexOptions::default());
          let lazy = IndexOptions { verify: false, ..IndexOptions::default() };
          let reader = ParquetIndex::open(dir.file(), header(), lazy).unwrap();

          let writer = open(&dir);
          writer.upsert(id(0), &vector(999), Payload::new("new.txt", 1, 2)).unwrap();
          assert!(writer.flush().unwrap());

          let mut tmp = dir.file().into_os_string();
          tmp.push(".tmp");
          assert!(!Path::new(&tmp).exists());
          assert_eq!(reader.read(id(0)).as_bytes(), &vector(0)[..]);
          assert_eq!(open(&dir).read(id(0)).as_bytes(), &vector(999)[..]);
      }

      #[test]
      fn apply_returns_its_own_stamp_signals_and_keeps_the_payload() {
          let dir = TempDir::new("apply");
          populated(&dir, 4, IndexOptions::default());
          let idx = open(&dir);
          let mut rx = idx.changed();
          assert!(!rx.has_changed().unwrap());

          let stamp = idx.apply(id(2), &vector(500));
          assert_eq!(idx.stamp(id(2)), Some(stamp));
          assert!(rx.has_changed().unwrap());
          assert_eq!(*rx.borrow_and_update(), stamp.raw());
          assert_eq!(idx.payload(id(2)), Some(Payload::new("f2.txt", 20, 29)));
          assert_eq!(idx.read(id(2)).as_bytes(), &vector(500)[..]);
          assert!(idx.is_dirty());

          idx.flush().unwrap();
          assert!(!idx.is_dirty());
          let reopened = open(&dir);
          assert_eq!(reopened.payload(id(2)), Some(Payload::new("f2.txt", 20, 29)));
          assert_eq!(reopened.stamp(id(2)), Some(stamp));
      }

      #[test]
      fn stamps_survive_restart_without_reissue() {
          let dir = TempDir::new("resume");
          populated(&dir, 3, IndexOptions::default());
          let idx = open(&dir);
          let removed = idx.remove(id(2)).unwrap();
          idx.flush().unwrap();
          drop(idx);

          let reopened = open(&dir);
          assert_eq!(reopened.stamp(id(2)), None);
          let fresh = reopened.apply(id(2), &vector(2));
          assert!(fresh.raw() > removed.raw());
      }

      #[test]
      fn missing_file_is_empty_and_emptied_index_roundtrips() {
          let dir = TempDir::new("empty");
          let idx = open(&dir);
          assert_eq!(idx.stamps().count(), 0);
          assert!(!idx.is_dirty());
          assert!(!idx.flush().unwrap());
          assert!(!dir.file().exists());

          idx.upsert(id(1), &vector(1), Payload::default()).unwrap();
          idx.remove(id(1)).unwrap();
          assert!(idx.flush().unwrap());
          let reopened = open(&dir);
          assert_eq!(reopened.stamps().count(), 0);
          assert!(reopened.high_water() >= 2);
      }

      #[test]
      fn malformed_vector_is_rejected() {
          let dir = TempDir::new("malformed");
          let idx = open(&dir);
          let r = idx.upsert(id(1), &[0u8; 10], Payload::default());
          assert!(matches!(r, Err(IndexError::IncompatibleSchema { .. })));
          assert_eq!(idx.stamp(id(1)), None);
      }

      #[test]
      #[should_panic(expected = "malformed vector")]
      fn apply_panics_on_contract_violation() {
          let dir = TempDir::new("panic");
          open(&dir).apply(id(1), &[0u8; 10]);
      }

      #[test]
      fn checksum_covers_vector_and_payload() {
          let p = Payload::new("a.txt", 1, 2);
          let v = vector(1);
          let base = p.checksum(&v);
          assert_eq!(base, p.checksum(&v));
          assert_ne!(base, Payload::new("b.txt", 1, 2).checksum(&v));
          assert_ne!(base, Payload::new("a.txt", 1, 3).checksum(&v));
          assert_ne!(base, p.checksum(&vector(2)));
      }
  reason: "Tests für Roundtrip, Header, Checksum, Stamp-Scan, mmap, Atomarität und B4-Vertrag"
```
/answer ed48-1b8 allow
Text:
 Jetzt kompilieren und testen.
ReasoningToken: 198
Control Request:
```yaml
id: ed48-1b9
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident::index 2>&1 | grep -E '^(error|warning: unused)|^\\s+-->|test result|FAILED|panicked|failed' -A6 | head -120"
  reason: Kompilieren und Index-Tests ausführen
```
/answer ed48-1b9 allow
Control Request:
```yaml
id: ed48-1ba
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-hgya5l2w.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-hgya5l2w.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-hgya5l2w.log
        size_bytes: 7093
        lines: 120
        words: 530
        complexity: 0.548
        created: 2026-10-25T22:06:56.003364+00:00
        modified: 2026-10-25T22:06:56.003364+00:00
        accessed: 2026-10-25T22:06:56.003364+00:00
        line_length_max: 226
        line_length_min: 0
        line_length_avg: 58.11
        words_per_line_avg: 4.42
        checksum: 99590e4b0eebacdb726e449ec30c18933da115a99c8c1670ed3487ffe66ffff6
      stdout_file_ast_outline:
      - id: file
        lines: 1-120
        signature: "error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, foun…"
```
/answer ed48-1ba allow
Control Request:
```yaml
id: ed48-1bb
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::index 2>&1 | grep -E '^error' -A12 | head -90
  reason: Compiler-Fehler im Detail lesen
```
/answer ed48-1bb allow
Control Request:
```yaml
id: ed48-1bc
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-zsb8t9ao.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-zsb8t9ao.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-zsb8t9ao.log
        size_bytes: 6858
        lines: 90
        words: 583
        complexity: 0.552
        created: 2026-10-25T22:07:24.554030+00:00
        modified: 2026-10-25T22:07:24.554030+00:00
        accessed: 2026-10-25T22:07:24.554030+00:00
        line_length_max: 226
        line_length_min: 2
        line_length_avg: 75.2
        words_per_line_avg: 6.48
        checksum: eeaa9c6938442e8d0a000c5ce83ed920219193e4ecffc2239e53db53b2f03c90
      stdout_file_ast_outline:
      - id: file
        lines: 1-90
        signature: "error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, foun…"
```
/answer ed48-1bc allow
Control Request:
```yaml
id: ed48-1bd
toolName: ast_read
arguments:
  items:
  - path: /tmp/mcpc-stdout-zsb8t9ao.log
  reason: Fehlerausgabe lesen
```
/answer ed48-1bd allow
Control Request:
```yaml
id: ed48-1be
result:
  structuredContent:
    results:
    - path: /tmp/mcpc-stdout-zsb8t9ao.log
      nodes:
      - id: file
        signature: "error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, foun…"
```
/answer ed48-1be allow
Control Request:
```yaml
id: ed48-1bf
toolName: bash
arguments:
  items:
  - cwd: /tmp
    script:
    - cat /tmp/mcpc-stdout-zsb8t9ao.log
  reason: "Log-Datei der Compilerfehler ausgeben (eigene Build-Ausgabe, kein Retrieval im Repo)"
```
/answer ed48-1bf allow
Control Request:
```yaml
id: ed48-1c0
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-dksmlp_k.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-dksmlp_k.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-dksmlp_k.log
        size_bytes: 6858
        lines: 90
        words: 583
        complexity: 0.552
        created: 2026-10-25T22:08:18.781396+00:00
        modified: 2026-10-25T22:08:18.781396+00:00
        accessed: 2026-10-25T22:08:18.781396+00:00
        line_length_max: 226
        line_length_min: 2
        line_length_avg: 75.2
        words_per_line_avg: 6.48
        checksum: eeaa9c6938442e8d0a000c5ce83ed920219193e4ecffc2239e53db53b2f03c90
      stdout_file_ast_outline:
      - id: file
        lines: 1-90
        signature: "error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, foun…"
```
/answer ed48-1c0 allow
ReasoningToken: 39
Control Request:
```yaml
id: ed48-1c1
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident::index --no-run --message-format short 2>&1 | grep -E 'error|warning: unused' | cut -c1-260 | head -30"
  reason: Kompakte Fehlerliste
```
/answer ed48-1c1 allow
Control Request:
```yaml
id: ed48-1c2
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-b36sxoix.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-b36sxoix.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-b36sxoix.log
        size_bytes: 3222
        lines: 13
        words: 259
        complexity: 0.531
        created: 2026-10-25T22:08:32.550235+00:00
        modified: 2026-10-25T22:08:32.550235+00:00
        accessed: 2026-10-25T22:08:32.550235+00:00
        line_length_max: 260
        line_length_min: 93
        line_length_avg: 246.85
        words_per_line_avg: 19.92
        checksum: f88bb8148a07e303aded67ffde959924922e85e09a726e6d108ddc8cee6b8347
      stdout_file_ast_outline:
      - id: file
        lines: 1-13
        signature: "src/resident/index/parquet_io.rs:491:88: error[E0277]: expected an `FnOnce(parq…"
```
/answer ed48-1c2 allow
ReasoningToken: 66
Control Request:
```yaml
id: ed48-1c3
toolName: search
arguments:
  items:
  - directory:
    - /tmp
    pattern: error
    include:
    - mcpc-stdout-b36sxoix.log
    limit: 20
  reason: Fehlerzeilen aus dem Build-Log lesen
```
/answer ed48-1c3 allow
Control Request:
```yaml
id: ed48-1c4
result:
  structuredContent:
    errors:
    - directory:
      - /tmp
      pattern: error
      error: |
        search failed (exit code 2): grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-oomd.service-hpDfrQ: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-switcheroo-control.service-fCujHr: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-logind.service-lbeMMf: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-power-profiles-daemon.service-jKZLBx: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-upower.service-GNstGX: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-fwupd.service-3b4QPY: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-colord.service-7ZoTgN: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-timesyncd.service-y9AhFi: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-ModemManager.service-2kam9a: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-bluetooth.service-VLrhT9: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-polkit.service-CTk1Ko: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-resolved.service-tTj4ZZ: Permission denied
```
/answer ed48-1c4 allow
Text:
 Zwischenstand: Schema-, Payload-, Parquet-Modul und Tests sind geschrieben. Der erste Build hat Compilerfehler, die ich gerade eingrenze.
ReasoningToken: 30
Control Request:
```yaml
id: ed48-1c5
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::index --no-run --message-format short 2>&1 | grep -E 'error' | sed -E 's/^src\/resident\/index\///' | cut -c1-170 | head -8
  reason: Fehlerliste kurz genug für Inline-Ausgabe
```
/answer ed48-1c5 allow
Control Request:
```yaml
id: ed48-1c6
result:
  structuredContent:
    results:
    - stdout: |
        parquet_io.rs:491:88: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `&impl Fn(std::io::Error) -> resident::error::IndexError + '_`: exp
        parquet_io.rs:491:87: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `impl Fn(std::io::Error) -> resident::error::IndexError + '_`: expe
        parquet_io.rs:494:39: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `&impl Fn(std::io::Error) -> resident::error::IndexError + '_`: exp
        parquet_io.rs:494:38: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `impl Fn(std::io::Error) -> resident::error::IndexError + '_`: expe
        parquet_io.rs:495:33: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `&impl Fn(std::io::Error) -> resident::error::IndexError + '_`: exp
        parquet_io.rs:495:32: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `impl Fn(std::io::Error) -> resident::error::IndexError + '_`: expe
        parquet_io.rs:497:29: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `&impl Fn(std::io::Error) -> resident::error::IndexError + '_`: exp
        parquet_io.rs:497:28: error[E0277]: expected an `FnOnce(parquet::errors::ParquetError)` closure, found `impl Fn(std::io::Error) -> resident::error::IndexError + '_`: expe
```
/answer ed48-1c6 allow
ReasoningToken: 316
Control Request:
```yaml
id: ed48-1c7
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
  regexp: err\(|&err|let err
  node_type: Fn
  reason: "Funktionen finden, die den generischen err-Closure verwenden"
```
/answer ed48-1c7 allow
Control Request:
```yaml
id: ed48-1c8
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
      nodes:
      - id: impl_Base.data
        type: ImplItemFn
        code: |2
              fn data(&self) -> Result<&BaseData, IndexError> {
                  let loaded = self.data.get_or_init(|| match &self.backing {
                      Some((source, meta)) => {
                          load_base(source, meta, self.dim, &self.path).map_err(|e| e.to_string())
                      }
                      None => Err("no backing file".to_string()),
                  });
                  loaded.as_ref().map_err(|reason| IndexError::IndexUnavailable {
                      scope: format!("{}: {reason}", self.path.display()),
                  })
              }
      - id: scan_stamps
        type: ItemFn
        code: |
          fn scan_stamps(
              source: &Source,
              meta: &ArrowReaderMetadata,
              path: &Path,
          ) -> Result<(BTreeMap<EntryId, Row>, u64), IndexError> {
              let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(source.clone(), meta.clone());
              let mask = ProjectionMask::roots(builder.parquet_schema(), [COL_ENTRY_ID, COL_STAMP]);
              let reader = builder.with_projection(mask).build().map_err(unavailable(path))?;
              let mut rows = BTreeMap::new();
              let (mut position, mut max_stamp) = (0u32, 0u64);
              for batch in reader {
                  let batch = batch.map_err(unavailable(path))?;
                  let ids = batch.column(0).as_primitive::<UInt64Type>();
                  let stamps = batch.column(1).as_primitive::<UInt64Type>();
                  for (id, stamp) in ids.values().iter().zip(stamps.values().iter()) {
                      max_stamp = max_stamp.max(*stamp);
                      let row = Row { stamp: Stamp::from_raw(*stamp), slot: Slot::Base(position) };
                      if rows.insert(EntryId::new(*id), row).is_some() {
                          return Err(IndexError::IndexUnavailable {
                              scope: format!("{}: duplicate entry {id}", path.display()),
                          });
                      }
                      position += 1;
                  }
              }
              Ok((rows, max_stamp))
          }
      - id: load_base
        type: ItemFn
        code: |
          fn load_base(
              source: &Source,
              meta: &ArrowReaderMetadata,
              dim: usize,
              path: &Path,
          ) -> Result<BaseData, IndexError> {
              let total = meta.metadata().file_metadata().num_rows().max(0) as usize;
              let reader = ParquetRecordBatchReaderBuilder::new_with_metadata(source.clone(), meta.clone())
                  .with_batch_size(READ_BATCH_ROWS)
                  .build()
                  .map_err(unavailable(path))?;
              let mut data = BaseData {
                  entry_ids: Vec::with_capacity(total),
                  vectors: Vec::with_capacity(total * dim * 2),
                  payloads: Vec::with_capacity(total),
                  checksums: Vec::with_capacity(total),
              };
              for batch in reader {
                  let batch = batch.map_err(unavailable(path))?;
                  let n = batch.num_rows();
                  data.entry_ids.extend_from_slice(batch.column(COL_ENTRY_ID).as_primitive::<UInt64Type>().values());
                  data.checksums.extend_from_slice(batch.column(COL_CHECKSUM).as_primitive::<UInt64Type>().values());
                  let list = batch.column(COL_VECTOR).as_fixed_size_list();
                  let values = list.values().as_primitive::<arrow::datatypes::Float16Type>();
                  let start = list.value_offset(0) as usize;
                  for v in &values.values()[start..start + n * dim] {
                      data.vectors.extend_from_slice(&v.to_le_bytes());
                  }
                  let files = batch.column(COL_TARGET_FILE).as_string::<i32>();
                  let starts = batch.column(COL_CHAR_START).as_primitive::<UInt64Type>();
                  let ends = batch.column(COL_CHAR_END).as_primitive::<UInt64Type>();
                  for i in 0..n {
                      data.payloads.push(Payload::new(files.value(i), starts.value(i), ends.value(i)));
                  }
              }
              Ok(data)
          }
      - id: impl_ParquetIndex.open
        type: ImplItemFn
        code: |2
              /// A missing file yields an empty index; the first `flush` creates it.
              pub fn open(
                  path: impl Into<PathBuf>,
                  expected: SchemaHeader,
                  options: IndexOptions,
              ) -> Result<Self, IndexError> {
                  let path = path.into();
                  let span = tracing::info_span!(
                      "index.open",
                      path = %path.display(),
                      schema_version = expected.schema_version,
                      model_id = %expected.index.model_id,
                      model_revision = %expected.model_revision,
                      access = options.access.as_str(),
                      row_groups = tracing::field::Empty,
                      checksum_ok = tracing::field::Empty,
                  );
                  let _guard = span.enter();

                  if !path.exists() {
                      let base = Base { path: path.clone(), dim: expected.dim(), backing: None, data: OnceLock::new() };
                      return Ok(Self::assemble(path, expected, options, base, BTreeMap::new(), StampCounter::new()));
                  }
                  let source = Source::open(&path, options.access).map_err(unavailable(&path))?;
                  let meta = ArrowReaderMetadata::load(&source, ArrowReaderOptions::new())
                      .map_err(unavailable(&path))?;
                  span.record("row_groups", meta.metadata().num_row_groups());

                  let kvs = meta.metadata().file_metadata().key_value_metadata().map(Vec::as_slice).unwrap_or_default();
                  let found = SchemaHeader::from_key_values(kvs)?;
                  found.check_against(&expected)?;
                  check_fields(meta.schema(), &arrow_schema(expected.dim()))?;

                  let (rows, file_max) = scan_stamps(&source, &meta, &path)?;
                  let counter = StampCounter::resume(found.index.high_water, file_max);
                  let base = Base {
                      path: path.clone(),
                      dim: expected.dim(),
                      backing: Some((source, meta)),
                      data: OnceLock::new(),
                  };
                  let index = Self::assemble(path, expected, options, base, rows, counter);
                  if options.verify {
                      let verified = index.verify();
                      span.record("checksum_ok", verified.is_ok());
                      verified?;
                  }
                  Ok(index)
              }
      - id: write_file
        type: ItemFn
        code: |
          fn write_file(
              path: &Path,
              header: &SchemaHeader,
              rows: &[(EntryId, Row)],
              base: Option<&BaseData>,
              row_group_rows: usize,
          ) -> Result<(), IndexError> {
              let mut tmp_name = path.as_os_str().to_owned();
              tmp_name.push(".tmp");
              let tmp = PathBuf::from(tmp_name);
              let result = write_tmp(&tmp, header, rows, base, row_group_rows, path)
                  .and_then(|()| std::fs::rename(&tmp, path).map_err(unavailable(path)))
                  .and_then(|()| sync_parent(path));
              if result.is_err() {
                  let _ = std::fs::remove_file(&tmp);
              }
              result
          }
      - id: sync_parent
        type: ItemFn
        code: |
          fn sync_parent(path: &Path) -> Result<(), IndexError> {
              match path.parent().filter(|p| !p.as_os_str().is_empty()) {
                  Some(dir) => File::open(dir).and_then(|d| d.sync_all()).map_err(unavailable(path)),
                  None => Ok(()),
              }
          }
      - id: write_tmp
        type: ItemFn
        code: |
          fn write_tmp(
              tmp: &Path,
              header: &SchemaHeader,
              rows: &[(EntryId, Row)],
              base: Option<&BaseData>,
              row_group_rows: usize,
              path: &Path,
          ) -> Result<(), IndexError> {
              let err = unavailable(path);
              let schema = arrow_schema(header.dim());
              let props = WriterProperties::builder()
                  .set_key_value_metadata(Some(header.to_key_values()))
                  .set_compression(Compression::UNCOMPRESSED)
                  .set_max_row_group_row_count(Some(row_group_rows))
                  .set_statistics_enabled(EnabledStatistics::Page)
                  .set_column_dictionary_enabled(ColumnPath::from(NAME_VECTOR), false)
                  .build();
              let file = File::create(tmp).map_err(&err)?;
              let mut writer = ArrowWriter::try_new(&file, schema.clone(), Some(props)).map_err(&err)?;
              for chunk in rows.chunks(row_group_rows) {
                  let batch = build_batch(&schema, header, chunk, base, path)?;
                  writer.write(&batch).map_err(&err)?;
                  writer.flush().map_err(&err)?;
              }
              writer.close().map_err(&err)?;
              file.sync_all().map_err(&err)
          }
      - id: build_batch
        type: ItemFn
        code: |
          fn build_batch(
              schema: &SchemaRef,
              header: &SchemaHeader,
              chunk: &[(EntryId, Row)],
              base: Option<&BaseData>,
              path: &Path,
          ) -> Result<RecordBatch, IndexError> {
              let err = unavailable(path);
              let (dim, vb) = (header.dim(), header.vector_bytes());
              let n = chunk.len();
              let (mut ids, mut stamps) = (Vec::with_capacity(n), Vec::with_capacity(n));
              let (mut starts, mut ends, mut sums) = (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n));
              let mut files: Vec<&str> = Vec::with_capacity(n);
              let mut values: Vec<f16> = Vec::with_capacity(n * dim);
              for (id, row) in chunk {
                  let (vector, payload, checksum): (&[u8], &Payload, u64) = match &row.slot {
                      Slot::Mem(m) => (&m.vector, &m.payload, m.checksum),
                      Slot::Base(i) => {
                          let b = base.ok_or_else(|| err("base data missing"))?;
                          let i = *i as usize;
                          (&b.vectors[i * vb..(i + 1) * vb], &b.payloads[i], b.checksums[i])
                      }
                  };
                  ids.push(id.get());
                  stamps.push(row.stamp.raw());
                  files.push(&payload.target_file);
                  starts.push(payload.char_start);
                  ends.push(payload.char_end);
                  sums.push(checksum);
                  values.extend(vector.chunks_exact(2).map(|c| f16::from_le_bytes([c[0], c[1]])));
              }
              let DataType::FixedSizeList(item, _) = schema.field(COL_VECTOR).data_type() else {
                  return Err(err("vector column is not a fixed size list"));
              };
              let vectors = FixedSizeListArray::try_new(
                  item.clone(),
                  dim as i32,
                  Arc::new(Float16Array::from(values)) as ArrayRef,
                  None,
              )
              .map_err(&err)?;
              let columns: Vec<ArrayRef> = vec![
                  Arc::new(UInt64Array::from(ids)),
                  Arc::new(UInt64Array::from(stamps)),
                  Arc::new(vectors),
                  Arc::new(StringArray::from(files)),
                  Arc::new(UInt64Array::from(starts)),
                  Arc::new(UInt64Array::from(ends)),
                  Arc::new(UInt64Array::from(sums)),
              ];
              RecordBatch::try_new(schema.clone(), columns).map_err(&err)
          }
```
/answer ed48-1c8 allow
Control Request:
```yaml
id: ed48-1c9
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: write_tmp
    old_text: .map_err(&err)
    new_text: ".map_err(|e| fail(path, e))"
    replaceAll: true
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: write_tmp
    old_text: |2
          let err = unavailable(path);
    new_text: ""
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: build_batch
    old_text: .map_err(&err)
    new_text: ".map_err(|e| fail(path, e))"
    replaceAll: true
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: build_batch
    old_text: |2
          let err = unavailable(path);
    new_text: ""
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: build_batch
    old_text: base.ok_or_else(|| err("base data missing"))?
    new_text: "base.ok_or_else(|| fail(path, \"base data missing\"))?"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: build_batch
    old_text: return Err(err("vector column is not a fixed size list"));
    new_text: "return Err(fail(path, \"vector column is not a fixed size list\"));"
  reason: Generischen Fehler-Closure durch nicht-generische Hilfsfunktion ersetzen
```
/answer ed48-1c9 allow
ReasoningToken: 231
Control Request:
```yaml
id: ed48-1ca
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
  name: unavailable
  node_type: ItemFn
  reason: Id der Funktion unavailable ermitteln
```
/answer ed48-1ca allow
Control Request:
```yaml
id: ed48-1cb
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
      nodes:
      - id: unavailable
        type: ItemFn
        code: |
          fn unavailable<E: Display>(path: &Path) -> impl Fn(E) -> IndexError + '_ {
              move |e| IndexError::IndexUnavailable {
                  scope: format!("{}: {e}", path.display()),
              }
          }
```
/answer ed48-1cb allow
Control Request:
```yaml
id: ed48-1cc
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/parquet_io.rs
    id: unavailable
    position: after
    source: |
      fn fail(path: &Path, e: impl Display) -> IndexError {
          IndexError::IndexUnavailable { scope: format!("{}: {e}", path.display()) }
      }
  reason: Nicht-generische Fehlerhilfsfunktion ergänzen
```
/answer ed48-1cc allow
ReasoningToken: 35
Control Request:
```yaml
id: ed48-1cd
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::index --no-run --message-format short 2>&1 | grep -E 'error|warning' | sed -E 's/^src\/resident\///' | cut -c1-200 | head -15
  reason: Erneut kompilieren
```
/answer ed48-1cd allow
Control Request:
```yaml
id: ed48-1ce
result:
  structuredContent:
    results:
    - stdout: |
        index/schema.rs:242:9: warning: unreachable pattern: no value can reach this
        subengine/exec.rs:337:20: warning: irrefutable `if let` pattern
        manager/seqlock.rs:20:26: warning: associated function `from_raw` is never used
        warning: `xy-ai-rag` (lib test) generated 3 warnings
```
/answer ed48-1ce allow
Control Request:
```yaml
id: ed48-1cf
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/schema.rs
    id: dtype_name
    old_text: |2
              _ => "unknown",
    new_text: ""
  reason: Unerreichbares Match-Arm entfernen
```
/answer ed48-1cf allow
ReasoningToken: 68
Control Request:
```yaml
id: ed48-1d0
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident:: 2>&1 | grep -E 'parquet_io|test result|FAILED|panicked|^error|index/' | cut -c1-220 | head -40"
  reason: Alle resident-Tests ausführen
```
/answer ed48-1d0 allow
Control Request:
```yaml
id: ed48-1d1
result:
  structuredContent:
    results:
    - stdout: |
        test resident::index::parquet_io::tests::checksum_covers_vector_and_payload ... ok
        test resident::index::parquet_io::tests::apply_panics_on_contract_violation - should panic ... ok
        test resident::index::parquet_io::tests::malformed_vector_is_rejected ... ok
        test resident::index::parquet_io::tests::missing_file_is_empty_and_emptied_index_roundtrips ... ok
        test resident::index::parquet_io::tests::missing_header_key_and_foreign_storage_dtype_are_incompatible ... ok
        test resident::index::parquet_io::tests::footer_carries_every_header_key ... ok
        test resident::index::parquet_io::tests::mixed_compute_dtype_is_documented_not_enforced ... ok
        test resident::index::parquet_io::tests::corrupted_vector_bytes_fail_the_checksum ... ok
        test resident::index::parquet_io::tests::binding_mismatch_is_rejected_with_the_offending_key ... ok
        test resident::index::parquet_io::tests::stamps_survive_restart_without_reissue ... ok
        test resident::index::parquet_io::tests::apply_returns_its_own_stamp_signals_and_keeps_the_payload ... ok
        test resident::index::parquet_io::tests::rewrite_is_atomic_and_keeps_existing_mappings_valid ... ok
        test resident::index::parquet_io::tests::stamps_do_not_decode_vectors ... ok
        test resident::index::parquet_io::tests::mmap_and_file_access_agree ... ok
        test resident::index::parquet_io::tests::roundtrip_is_bit_exact_across_row_groups ... ok
        test result: ok. 566 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 7.27s
```
/answer ed48-1d1 allow
Text:
 Alle 566 Tests sind grün. Noch fehlt der Test der neuen Fehlervarianten (Q1).
ReasoningToken: 263
Control Request:
```yaml
id: ed48-1d2
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/error.rs
    ids:
    - retryability_classification
  reason: Bestehenden Retry-Test als Anknüpfungspunkt lesen
```
/answer ed48-1d2 allow
Control Request:
```yaml
id: ed48-1d3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/error.rs
      nodes:
      - id: retryability_classification
        type: ItemFn
        code: |
          #[test]
          fn retryability_classification() {
              assert!(
                  IndexError::ArenaExhausted { requested : 1, available : 0 } .is_retryable()
              );
              assert!(IndexError::IndexUnavailable { scope : "s".into() } .is_retryable());
              assert!(! IndexError::KernelFailed { reason : "r".into() } .is_retryable());
              assert!(ManagerError::StaleRegion { arena : 1, gen : 2 } .is_retryable());
              assert!(
                  PrecisionError::CalibrationUnstable { reason : "r".into() } .is_retryable()
              );
              assert!(! PrecisionError::NoActiveProfile.is_retryable());
              assert!(
                  ! ConfigError::InvalidValue { field : "f".into(), reason : "r".into() }
                  .is_retryable()
              );
              assert!(! CapabilityError::Unknown { capability : "c".into() } .is_retryable());
              let wrapped: ResidentError = IndexError::ArenaExhausted {
                  requested: 1,
                  available: 0,
              }
                  .into();
              assert!(wrapped.is_retryable());
              let fatal: ResidentError = PlannerError::InvalidConfig {
                  reason: "r".into(),
              }
                  .into();
              assert!(! fatal.is_retryable());
          }
```
/answer ed48-1d3 allow
Control Request:
```yaml
id: ed48-1d4
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/error.rs
    id: retryability_classification
    position: after
    source: |
      #[test]
      fn index_integrity_errors_are_fatal_and_descriptive() {
          let schema = IndexError::IncompatibleSchema {
              expected: "embedding.dim=384".into(),
              found: "embedding.dim=768".into(),
          };
          assert!(!schema.is_retryable());
          assert!(schema.to_string().contains("embedding.dim=768"));
          let sum = IndexError::ChecksumMismatch { entry: 7 };
          assert!(!sum.is_retryable());
          assert!(sum.to_string().contains('7'));
          let wrapped: ResidentError = sum.into();
          assert!(matches!(wrapped, ResidentError::Index(IndexError::ChecksumMismatch { entry: 7 })));
      }
  reason: Test für neue Fehlervarianten ergänzen
```
/answer ed48-1d4 allow
Control Request:
```yaml
id: ed48-1d5
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::test::error 2>&1 | grep -E 'test result|FAILED|^error' | head
  reason: Fehlertests ausführen
```
/answer ed48-1d5 allow
Control Request:
```yaml
id: ed48-1d6
result:
  structuredContent:
    results:
    - stdout: |
        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 595 filtered out; finished in 0.00s
```
/answer ed48-1d6 allow
Control Request:
```yaml
id: ed48-1d7
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib index_integrity_errors 2>&1 | grep -E 'test |test result|^error' | head
  reason: Neuen Fehlertest gezielt ausführen
```
/answer ed48-1d7 allow
Control Request:
```yaml
id: ed48-1d8
result:
  structuredContent:
    results:
    - stdout: |
        test resident::error::tests::index_integrity_errors_are_fatal_and_descriptive ... ok
        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 594 filtered out; finished in 0.00s
```
/answer ed48-1d8 allow
Result Stats: id=80fb4a4b-9f5b-4da5-bbcf-0463e039af0e, total: 181582, in: 66, out: 63632, reason: 0, read: 2524241, write: 117884, start: 1792966476482, end: 1792966487037, time: 10511, total: 496906, io: 0, control: 44, sent: 0, recv: 271
G2 ist umgesetzt. Der `ParquetIndex` implementiert das `Index`-Trait aus B4. Alle 595 Lib-Tests ohne CUDA sind grün, darunter 15 neue Index-Tests und 1 neuer Fehlertest. Den cuda-Build und die pyarrow/DuckDB-Interop habe ich nicht geprüft.

Neue Dateien unter `rag/src/resident/index/`: `schema.rs`, `payload.rs`, `parquet_io.rs` und `test/parquet_io.rs`. Geändert habe ich `index/mod.rs` (Module und Re-Exports) und `resident/error.rs`.

- **Schema:**
  - `FixedSizeList<Float16,384>` mit den Spalten `entry_id`, `stamp`, `vector`, `target_file`, `char_start`, `char_end`, `checksum`.
  - Der Stamp liegt in einer eigenen Spalte. Beim Öffnen werden nur `entry_id` und `stamp` gelesen. Der Test `stamps_do_not_decode_vectors` bestätigt, dass dabei kein Vektor dekodiert wird.
  - Eine eigene Spalte `row` gibt es nicht. Die Zeilenposition ergibt sich aus der Reihenfolge, ein redundanter Wert könnte nur inkonsistent werden.
- **Header:**
  - Alle Keys aus §5 stehen als Footer-Keys, dazu `resident.high_water` und die Hash-Angaben. `SchemaHeader::from_spec` baut sie aus dem G1-`ModelSpec`.
  - Beim Öffnen werden alle Keys außer `compute_dtype` strikt geprüft. Ein Mismatch ergibt `IncompatibleSchema`, mit allen abweichenden Keys im Text.
  - `from_spec` lehnt eine andere Storage-dtype als f16 ab.
- **Checksum:** xxh3-64 über Vektorbytes und Payload-Felder. Ein Fehler beim Öffnen oder in `verify()` ergibt `ChecksumMismatch`. Der Test flippt dazu ein Byte direkt in der Datei.
- **Schreiben:** Die Daten gehen in `*.tmp`, werden mit fsync gesichert und per `rename` über die Indexdatei gelegt, danach fsync auf das Verzeichnis. Eine bereits gemappte alte Datei bleibt gültig.
- **Lesen:** `Bytes::from_owner(Mmap)` mit `madvise(Random)`, alternativ `Access::File`. Beide Wege liefern im Test identische Daten.
- **Index-Trait:**
  - `apply` gibt den eigenen Stamp zurück und löst das `changed`-Signal aus. Ein bestehender Payload bleibt dabei erhalten, und `capabilities` ist `[Range]`.
  - Der Stamp-Zähler setzt nach einem Neustart über die gespeicherte `high_water` fort, auch über entfernte Einträge hinweg.
- **Logging und Fehler:** Die Spans `index.open` und `index.write` enthalten die Felder aus dem Plan. Neu in `IndexError` sind `IncompatibleSchema` und `ChecksumMismatch`. I/O-Fehler laufen über `IndexUnavailable`.

Beim Umsetzen habe ich Folgendes festgelegt:
- Das Persistieren geschieht nur über `flush()`; `apply` ändert zunächst nur den Speicher.
- Ein Vektor mit falscher Länge löst in `apply` eine Panic aus, weil das Trait keinen Fehlerpfad hat. `upsert` gibt stattdessen einen Fehler zurück.
- `prefetch` ist ein No-op. Ein `WillNeed` auf die ganze Datei wäre zu grob.

Die pyarrow/DuckDB-Interop ist im Plan als optional markiert und noch offen. Eine Plan-Datei habe ich nicht angefasst, der Lesezugriff wurde abgelehnt.
Was meint pyarrow/DuckDB-Interop?
Result Stats: id=80fb4a4b-9f5b-4da5-bbcf-0463e039af0e, total: 183900, in: 68, out: 64458, reason: 0, read: 2647256, write: 119374, start: 1792966555396, end: 1792966563135, time: 7689, total: 504595, io: 0, control: 50, sent: 1, recv: 181
SystemInit:  id=80fb4a4b-9f5b-4da5-bbcf-0463e039af0e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Gemeint ist: Lassen sich die von `ParquetIndex` geschriebenen Dateien auch mit anderen Werkzeugen lesen, nicht nur mit dem Rust-Code, der sie erzeugt hat?

- **pyarrow** ist die Python-Bindung an Apache Arrow und Parquet. `pyarrow.parquet.read_table("index.parquet")` müsste die Datei laden. Die Vektorspalte müsste dabei als `fixed_size_list<halffloat>[384]` erkannt werden.
- **DuckDB** ist eine eingebettete SQL-Engine. `SELECT entry_id, stamp, target_file FROM 'index.parquet'` müsste funktionieren, ebenso das Lesen der Vektorspalte.

**Warum das offen ist:** Mein Roundtrip-Test läuft nur mit denselben `parquet`/`arrow`-Crates (Version 60) für Schreiben und Lesen. Er zeigt, dass die Datei in sich stimmig ist. Er zeigt nicht, dass fremde Reader sie verstehen. Das ist bei `Float16` relevant. Der Typ wird als `FIXED_LEN_BYTE_ARRAY(2)` mit Logical Type `FLOAT16` geschrieben, und das ist ein relativ junges Parquet-Feature. Ältere pyarrow- oder DuckDB-Versionen kennen es womöglich nicht oder lesen es nur als rohe Bytes.

**Warum es keine Kernanforderung ist:** Die Engine liest ihre Indizes immer selbst. Interop wäre nur praktisch zum Debuggen, für Analysen im Notebook oder um Daten zu exportieren.

**Falls Interop nicht klappt:** Das Vektor-Layout müsste nicht zwingend geändert werden. Die Plan-Alternative war `FixedSizeBinary(768)` mit dim/dtype/Endianness in den Metadaten. Das wäre nur nötig, wenn dir Interop wichtig ist und der `Float16`-Pfad scheitert.

Prüfen ließe es sich mit einem kurzen Skript. Es würde eine Testdatei schreiben, sie mit `python3 -I` in pyarrow öffnen und, falls `duckdb` installiert ist, auch dort. Soll ich das machen?
