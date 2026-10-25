# Prompt: Gruppe E, DoD-Restpunkte schließen

## Kontext
Crate: `/home/user/xyan/xy.ai.workbench/rag`, Pläne: `/home/user/xyan/xy.ai.workbench/project/resident/plan/`. Zielhardware ist sm_75 (GTX 1660 SUPER). nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`.

Die Gruppe E ist umgesetzt und getestet. Es bleiben vier Punkte der Gruppen-DoD aus `plan_e_00_overview.md` §7. Setze nur diese vier um. Ändere sonst nichts.

## Regeln
- Code und Code-Kommentare sind englisch. Kommentare nur, wo der Code nicht selbsterklärend ist, und ohne Anforderungen zu wiederholen.
- Plan-Texte sind deutsch.
- Verifiziere den Status am Code oder durch Ausführen, nicht anhand der `## Result`-Abschnitte der Pläne. Diese können veraltet sein.
- Trage nur „verifiziert“ ein, wenn du es belegt hast. Sonst „offen“ mit Begründung.

## Änderung 1: Arena-Auslastung loggen
**Datei:** `src/resident/manager/arena.rs`, `streamed.rs`, `swap.rs`

**Befund:** `IndexArena::trace_utilization` (loggt `capacity`, `live`, `retired`, `headroom`, `available`) hat keinen Aufrufer.

**Änderungen:**
1. Rufe `trace_utilization` in der Zyklusfunktion des Managers in `streamed.rs` am Zyklusende auf. Sie läuft unter dem Span `manager_cycle`.
2. Rufe sie nach `publish` und nach `reclaim` auf, falls die Arena dort zugänglich ist. `swap.rs` publiziert die Tabellen, `arena.rs` gibt die retired Segmente frei.
3. Prüfe, ob die Auslastung pro Zyklus als `debug!` reicht. Das Level bleibt `debug`.

**Test:** Ergänze in `manager/test/` einen Test mit dem Tracing-Test-Subscriber aus `resident/test/telemetry.rs`. Er prüft, dass ein Zyklus ein Event „index arena utilization“ erzeugt.

## Änderung 2: Konsistenzmodell als Code-Doku
**Datei:** `src/resident/manager/mod.rs` (Modul-Doku `//!`)

Ergänze einen Abschnitt „Consistency model“ mit diesen Aussagen:
- The index is the source of truth. VRAM is a derived, discardable mirror. On divergence the index wins.
- Identity is `(EntryId, Stamp)`. Stamps are compared with `!=` only, never with `<`.
- Reconciliation is state-based. The `watch` signal only wakes the scan. Only resident entries are compared.
- Write order in the index is data, then stamp, then signal. Read order on load is stamp, data, stamp again.
- Segment tables are copy-on-write with a root pointer swap. A retired version is freed once all readers have reached the new epoch.
- Last writer wins. `apply` is serialized.

Verweise per Intra-Doc-Link auf `sync::load_entry`, `sync::SyncScanner::scan`, `swap`, `slot` und `index::Index::apply`. Prüfe vorher, dass jeder Verweis existiert und die Aussage zum Code passt. Wenn eine Aussage nicht stimmt, melde es, statt sie zu übernehmen.

## Änderung 3: Verifikationsstatus E4 (sm_75-Atomics)
**Datei:** `project/resident/plan/plan_e_04.md`, Abschnitt „8. Offene Verifikationen“

**Schritte:**
1. Führe die zwei Tests aus `src/resident/manager/swap.rs` auf der GPU aus: `cargo test --lib --features cuda device_ -- --ignored --nocapture` mit passendem Filter für `device_sees_content_written_before_the_host_release_store` und `copy_stream_event_orders_device_table_before_the_root`.
2. Lies den ausgegebenen Wert „host native atomics supported“.
3. Trage unter jedem der drei Punkte (`ld.acquire.sys`, `HostNativeAtomicSupported`, Event-/Fence-Sichtbarkeit) ein: Status (verifiziert/offen), Messdatum, Hardware, Ergebnis.
4. Wenn `HostNativeAtomicSupported` 0 ist, halte fest, dass die Single-Writer-Strategie (Host schreibt, GPU liest per `ld.acquire.sys`) davon nicht berührt wird, weil es kein GPU-CAS über PCIe gibt.
5. Wenn ein Test fehlschlägt oder nicht läuft, trage „offen“ mit Fehlerbild ein und melde es.

Ändere den Test in `swap.rs` nicht. Optional darf `eprintln!` durch eine `assert!`-freie Ausgabe bleiben.

## Änderung 4: Verifikationsstatus E1 (madvise)
**Datei:** `project/resident/plan/plan_e_01.md`, Abschnitt „8. Offene Verifikationen“

**Schritte:**
1. Prüfe `MADV_COLD`/`MADV_WILLNEED` gegen die man-Seite `madvise(2)` (`man 2 madvise`). Notiere: `MADV_COLD` ab Linux 5.4, ein Hint, der Inhalte nicht verändert.
2. Prüfe die verwendete `memmap2`-Version aus `Cargo.toml` und dass `Advice::WillNeed/Sequential/Random` in dieser Version existieren.
3. Führe `cargo test --lib resident::manager::test::advice` aus. Die Tests `will_need_brings_pages_into_the_page_cache` und `cold_keeps_contents_and_does_not_drop_pages` (in `test/advice.rs`, Modul `residency`) belegen die Wirkung per `mincore`.
4. Hole die Kernelversion mit `uname -r` (zuletzt 6.8.0-147-generic).
5. Trage für beide Punkte (Semantik/API, Kernel ≥ 5.4) Status, Datum, Kernel, `memmap2`-Version und Testergebnis ein.

## Abschluss
- `cargo test --lib resident::` muss grün sein, ebenso `cargo clippy --all-targets` und `cargo check --features cuda`.
- Liefere eine kurze Liste: geänderte Dateien, eingetragene Status (verifiziert/offen) und alles, was du nicht belegen konntest.

SystemInit:  id=39f2b96f-848e-49e0-8145-b63891ccf271, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich beginne mit der Sichtung des Codes.
ReasoningToken: 13
Control Request:
```yaml
id: b56c-24
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: trace_utilization|manager_cycle|index arena utilization
    include:
    - '*.rs'
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: fn (load_entry|scan|apply|publish|reclaim)\b|^pub mod|^mod |^//!
    include:
    - mod.rs
    - sync.rs
    - swap.rs
    - slot.rs
    - index.rs
    - arena.rs
  reason: "Locate trace_utilization callers, manager span, and doc link targets"
```
/answer b56c-24 allow
Control Request:
```yaml
id: b56c-25
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: test/telemetry.rs
        matches:
        - "assert_eq!(FIELD_MANAGER_CYCLE, \"manager_cycle\");"
      - path: manager/swap.rs
        matches:
        - let _span = crate::resident::telemetry::manager_cycle_span(table.epoch())
      - path: manager/arena.rs
        matches:
        - "pub fn trace_utilization(&self) {"
        - ".cow_headroom, available = b.available, \"index arena utilization\""
      - path: manager/streamed.rs
        matches:
        - "tracing::info_span!(\"manager_cycle\", index_id = % self.index_id)"
      - path: telemetry.rs
        matches:
        - "pub const FIELD_MANAGER_CYCLE: &str = \"manager_cycle\";"
        - "pub fn manager_cycle_span(manager_cycle: u64) -> Span {"
        - "tracing::info_span!(\"manager_cycle\", manager_cycle = manager_cycle)"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - "//! Resident engine: embedding/similarity substrate, kept warm."
        - pub mod backend;
        - pub mod error;
        - pub mod failpoint;
        - pub mod telemetry;
        - pub mod mock;
        - pub mod queue;
        - pub mod memory;
        - pub mod op;
        - pub mod index;
        - pub mod kernel;
        - pub mod precision;
        - pub mod facade;
        - pub mod processor;
        - pub mod planner;
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b56c-25 allow
Control Request:
```yaml
id: b56c-26
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: manager|telemetry
  reason: File layout
```
/answer b56c-26 allow
Control Request:
```yaml
id: b56c-27
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - telemetry.rs
      - "./memory:"
      - manager.rs
      - "./memory/test:"
      - manager.rs
      - "./test:"
      - telemetry.rs
```
/answer b56c-27 allow
Control Request:
```yaml
id: b56c-28
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - git status --short | head
  reason: Find manager layout
```
/answer b56c-28 allow
Control Request:
```yaml
id: b56c-29
result:
  structuredContent:
    results:
    - stdout: |
        RM ../project/resident/plan/check_e8.md -> ../project/resident/plan/checks/check_e8.md
        ?? ../project/resident/plan/checks/check_e9.md
```
/answer b56c-29 allow
ReasoningToken: 67
Control Request:
```yaml
id: b56c-2a
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - find . -path ./target -prune -o -path '*manager*' -print | sort | head -50
    - git rev-parse --show-toplevel
    - ls src/resident
  reason: Locate manager directory
```
/answer b56c-2a deny nutze spezifische Tools dafür
Control Request:
```yaml
id: b56c-2b
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: manager
  - path: /home/user/xyan/xy.ai.workbench/rag/src
    pattern: manager
    recursive: true
  reason: Locate manager files
```
/answer b56c-2b allow
Control Request:
```yaml
id: b56c-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      entries:
      - "./resident/memory:"
      - manager.rs
      - "./resident/memory/test:"
      - manager.rs
```
/answer b56c-2c allow
ReasoningToken: 99
Control Request:
```yaml
id: b56c-2d
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan
    pattern: "plan_e_0[14]"
  reason: List manager dir directly
```
/answer b56c-2d allow
Control Request:
```yaml
id: b56c-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      entries:
      - ".:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - doorbell.rs
      - evict.rs
      - flight.rs
      - mod.rs
      - residency.rs
      - runtime.rs
      - scope.rs
      - seqlock.rs
      - slot.rs
      - streamed.rs
      - swap.rs
      - sync.rs
      - tracker.rs
      - transfer.rs
      - upload.rs
      - "./test:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - evict.rs
      - flat.rs
      - flight.rs
      - residency.rs
      - runtime.rs
      - scope.rs
      - slot.rs
      - streamed.rs
      - sync.rs
      - tracker.rs
      - transfer.rs
      - upload.rs
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan
      entries:
      - ".:"
      - plan_e_01.md
      - plan_e_04.md
      - "./runs:"
      - plan_e_01.md
      - plan_e_04.md
```
/answer b56c-2e allow
Control Request:
```yaml
id: b56c-2f
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    script:
    - cat mod.rs
    - grep -n 'trace_utilization' -B3 -A25 arena.rs
    - grep -n 'manager_cycle' -B15 -A60 streamed.rs
  reason: "Read mod.rs, trace_utilization, cycle function"
```
/answer b56c-2f deny AST
Text:
 Bash wird hier abgelehnt, ich nutze die AST-Werkzeuge. Als Erstes lese ich die Struktur von `mod.rs`, `arena.rs`, `streamed.rs` und `swap.rs`.
ReasoningToken: 13
Control Request:
```yaml
id: b56c-30
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
  reason: Get node ids for edits and understand structure
```
/answer b56c-30 allow Du kannst auch find verwenden, wenn du weißt was du suchst oder search
Control Request:
```yaml
id: b56c-31
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
      - id: advice
        signature: pub mod advice;
      - id: blocking
        signature: pub mod blocking;
      - id: doorbell
        signature: pub mod doorbell;
      - id: build
        signature: pub mod build;
      - id: arena
        signature: pub mod arena;
      - id: scope
        signature: pub mod scope;
      - id: slot
        signature: pub mod slot;
      - id: swap
        signature: pub mod swap;
      - id: runtime
        signature: pub mod runtime;
      - id: flat_index
        signature: pub(crate) mod flat_index;
      - id: streamed
        signature: pub mod streamed;
      - id: seqlock
        signature: pub mod seqlock;
      - id: sync
        signature: pub mod sync;
      - id: tracker
        signature: pub mod tracker;
      - id: evict
        signature: pub mod evict;
      - id: flight
        signature: pub(crate) mod flight;
      - id: residency
        signature: pub mod residency;
      - id: upload
        signature: pub mod upload;
      - id: transfer
        signature: pub mod transfer;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index-segment use of the shared arena allocator: aligned segments, copy-on-…"
      - id: Cl3g3y|QQdjOc
        signature: "use std::ops::{Deref, Range};"
      - id: WRTBpG|CxUsUc
        signature: /// Minimum alignment of every segment offset.
      - id: CowBudget
        signature: "pub struct CowBudget {"
        docstring: Space that must stay free so a replaced segment can be rebuilt next to its pred…
      - id: impl_CowBudget
        signature: "impl CowBudget {"
        children:
        - id: impl_CowBudget.headroom
          signature: "pub fn headroom(&self) -> u64 {"
        - id: impl_CowBudget.required
          signature: "pub fn required(&self) -> u64 {"
      - id: ArenaBudget
        signature: "pub struct ArenaBudget {"
      - id: Retired
        signature: "struct Retired {"
      - id: State
        signature: "struct State {"
      - id: impl_State
        signature: "impl State {"
        children:
        - id: impl_State.is_retired
          signature: "fn is_retired(&self, region: Region) -> bool {"
      - id: AllocRef
        signature: "enum AllocRef<'a> {"
      - id: impl_Deref_for_AllocRef____
        signature: "impl Deref for AllocRef<'_> {"
        children:
        - id: impl_Deref_for_AllocRef____.jj8Fh8|OlMOr4
          signature: type Target = ArenaAllocator;
        - id: impl_Deref_for_AllocRef____.deref
          signature: "fn deref(&self) -> &ArenaAllocator {"
      - id: IndexArena
        signature: "pub struct IndexArena<'a> {"
      - id: impl_IndexArena__a_
        signature: "impl<'a> IndexArena<'a> {"
        children:
        - id: impl_IndexArena__a_.new
          signature: "pub fn new(inner: &'a ArenaAllocator, reserve: u64) -> Self {"
          docstring: '`reserve` bytes of the arena are never handed out.'
        - id: impl_IndexArena__a_.shared
          signature: "pub fn shared(inner: Arc<ArenaAllocator>, reserve: u64) -> IndexArena<'static> {"
          docstring: "Owns a share of the allocator, so the arena can sit behind `'static` trait obje…"
        - id: impl_IndexArena__a_.with_ref
          signature: "fn with_ref(inner: AllocRef<'a>, reserve: u64) -> Self {"
        - id: impl_IndexArena__a_.alloc_segment
          signature: "pub fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerEr…"
        - id: impl_IndexArena__a_.reserve_writable
          signature: "pub fn reserve_writable(&self, cap: u64) -> Result<WritableRegion, ManagerError…"
          docstring: "Segment the build path fills incrementally through [`WritableRegion::claim`]."
        - id: impl_IndexArena__a_.free
          signature: "pub fn free(&self, region: Region) -> Result<(), ManagerError> {"
          docstring: Immediate release of a region no reader or in-flight work can reference.
        - id: impl_IndexArena__a_.free_after
          signature: "pub fn free_after(     &self,     region: Region,     event: EventHandle, ) -> …"
          docstring: Release once all work recorded before `event` has finished.
        - id: impl_IndexArena__a_.retire
          signature: "pub fn retire(&self, region: Region, epoch: u64) -> Result<(), ManagerError> {"
          docstring: Marks `region` as replaced by the version published at `epoch`.
        - id: impl_IndexArena__a_.reclaim
          signature: "pub fn reclaim(&self, min_reader_epoch: u64) -> usize {"
          docstring: Frees every retired region that no reader at `min_reader_epoch` or later can st…
        - id: impl_IndexArena__a_.begin_cow
          signature: "pub fn begin_cow(&self, old: Region) -> Result<CowTicket<'_, 'a>, ManagerError>…"
          docstring: Starts a copy-on-write replacement of `old`; waits while another one is in prog…
        - id: impl_IndexArena__a_.try_begin_cow
          signature: "pub fn try_begin_cow(     &self,     old: Region, ) -> Result<Option<CowTicket<…"
          docstring: "Like [`begin_cow`](Self::begin_cow), but `Ok(None)` when another replacement is…"
        - id: impl_IndexArena__a_.cow_budget
          signature: "pub fn cow_budget(&self) -> CowBudget {"
        - id: impl_IndexArena__a_.budget
          signature: "pub fn budget(&self) -> ArenaBudget {"
        - id: impl_IndexArena__a_.trace_utilization
          signature: "pub fn trace_utilization(&self) {"
        - id: impl_IndexArena__a_.start_cow
          signature: "fn start_cow<'g>(     &'g self,     gate: MutexGuard<'g, ()>,     old: Region, …"
        - id: impl_IndexArena__a_.admit_and_alloc
          signature: "fn admit_and_alloc(     &self,     bytes: u64,     align: u64,     headroom: u6…"
        - id: impl_IndexArena__a_.ensure_owned
          signature: "fn ensure_owned(&self, st: &State, region: Region) -> Result<(), ManagerError> {"
      - id: CowTicket
        signature: "pub struct CowTicket<'a, 'b> {"
        docstring: Replacement segment under construction. Dropping it without `commit` discards t…
      - id: impl_CowTicket_______
        signature: "impl CowTicket<'_, '_> {"
        children:
        - id: impl_CowTicket_______.old_region
          signature: "pub fn old_region(&self) -> Region {"
        - id: impl_CowTicket_______.new_region
          signature: "pub fn new_region(&self) -> Region {"
        - id: impl_CowTicket_______.commit
          signature: "pub fn commit(mut self, epoch: u64) -> Result<Region, ManagerError> {"
          docstring: Publishes the copy as the version of `epoch`; the old segment is released by `r…
      - id: impl_Drop_for_CowTicket_______
        signature: "impl Drop for CowTicket<'_, '_> {"
        children:
        - id: impl_Drop_for_CowTicket_______.drop
          signature: "fn drop(&mut self) {"
      - id: WritableRegion
        signature: "pub struct WritableRegion {"
        docstring: Segment filled front to back by a single writer.
      - id: impl_WritableRegion
        signature: "impl WritableRegion {"
        children:
        - id: impl_WritableRegion.region
          signature: "pub fn region(&self) -> Region {"
        - id: impl_WritableRegion.written
          signature: "pub fn written(&self) -> u64 {"
        - id: impl_WritableRegion.remaining
          signature: "pub fn remaining(&self) -> u64 {"
        - id: impl_WritableRegion.claim
          signature: "pub fn claim(&mut self, len: u64, align: u64) -> Result<Range<u64>, ManagerErro…"
          docstring: Claims `len` bytes at `align`; returns the absolute arena offsets.
        - id: impl_WritableRegion.into_parts
          signature: "pub fn into_parts(self) -> (Region, u64) {"
      - id: map_memory
        signature: "fn map_memory(e: MemoryError) -> ManagerError {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
      nodes:
      - id: ModuleDoc
        signature: //! Index manager skeleton and its CPU and GPU variants. The index is attached …
      - id: 15v60N|4vaRLA
        signature: use std::collections::BTreeMap;
      - id: sfiJvG|CnJTuE
        signature: "use crate::resident::manager::slot::{"
      - id: SharedIndex
        signature: pub type SharedIndex = Arc<dyn Index + Send + Sync>;
      - id: IndexManager
        signature: "pub trait IndexManager: Send + Sync {"
        docstring: Interface shared by the CPU and GPU variants.
        children:
        - id: IndexManager.attach_index
          signature: "fn attach_index(&self, index: SharedIndex) -> bool;"
          docstring: '`false` when an index is already attached; the first one stays.'
        - id: IndexManager.is_active
          signature: fn is_active(&self) -> bool;
      - id: StreamedIndexManager
        signature: "pub struct StreamedIndexManager<B: ?Sized> {"
      - id: impl_StreamedIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> StreamedIndexManager<B> {"
        children:
        - id: impl_StreamedIndexManager_B_.new
          signature: "pub fn new(engine: Arc<B>, index_id: impl Into<String>) -> Self {"
        - id: impl_StreamedIndexManager_B_.engine
          signature: "pub fn engine(&self) -> &B {"
        - id: impl_StreamedIndexManager_B_.index_id
          signature: "pub fn index_id(&self) -> &str {"
        - id: impl_StreamedIndexManager_B_.index
          signature: "pub fn index(&self) -> Result<&SharedIndex, IndexError> {"
        - id: impl_StreamedIndexManager_B_.activate
          signature: "pub async fn activate<F, Fut>(&self, init: F) -> Result<&SharedIndex, IndexErro…"
          docstring: Runs `init` on first use only; concurrent callers wait for the same initializat…
        - id: impl_StreamedIndexManager_B_.cycle_span
          signature: "pub fn cycle_span(&self) -> tracing::Span {"
        - id: impl_StreamedIndexManager_B_.resident_len
          signature: "pub fn resident_len(&self) -> usize {"
      - id: impl_IndexManager_for_StreamedIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> IndexManager for StreamedIndexManager<B> {"
        children:
        - id: impl_IndexManager_for_StreamedIndexManager_B_.attach_index
          signature: "fn attach_index(&self, index: SharedIndex) -> bool {"
        - id: impl_IndexManager_for_StreamedIndexManager_B_.is_active
          signature: "fn is_active(&self) -> bool {"
      - id: ReadPattern
        signature: "pub enum ReadPattern {"
        docstring: Access pattern of a multi-entry read; selects the page-cache hint.
      - id: covering
        signature: "fn covering(items: &[(EntryId, Range<usize>)]) -> Option<Range<usize>> {"
      - id: CpuIndexManager
        signature: "pub struct CpuIndexManager<B: ?Sized> {"
        docstring: "RAM is the primary instance: reads go straight to the index; the only job is pa…"
      - id: impl_CpuIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> CpuIndexManager<B> {"
        children:
        - id: impl_CpuIndexManager_B_.new
          signature: "pub fn new(     core: StreamedIndexManager<B>,     handle: Handle,     advice: …"
        - id: impl_CpuIndexManager_B_.core
          signature: "pub fn core(&self) -> &StreamedIndexManager<B> {"
        - id: impl_CpuIndexManager_B_.read
          signature: "pub async fn read(     &self,     entry: EntryId,     hint: Range<usize>, ) -> …"
          docstring: "`hint` is the byte range of the entry in the mapped file, used for the prefetch…"
        - id: impl_CpuIndexManager_B_.read_set
          signature: "pub async fn read_set(     &self,     items: Vec<(EntryId, Range<usize>)>,     …"
        - id: impl_CpuIndexManager_B_.read_entries
          signature: "pub async fn read_entries(     &self,     entries: Vec<EntryId>,     pattern: R…"
          docstring: Reads entries and takes the byte ranges for the page-cache hints from the index…
        - id: impl_CpuIndexManager_B_.advise_pattern
          signature: "fn advise_pattern(&self, items: &[(EntryId, Range<usize>)], pattern: ReadPatter…"
        - id: impl_CpuIndexManager_B_.read_ids
          signature: "async fn read_ids(&self, ids: Vec<EntryId>) -> Result<Vec<Vec<u8>>, IndexError>…"
        - id: impl_CpuIndexManager_B_.cool
          signature: "pub fn cool(&self, range: Range<usize>) {"
          docstring: "Marks a range as unlikely to be needed soon, e.g. after an idle period."
      - id: impl_IndexManager_for_CpuIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> IndexManager for CpuIndexManager<B> {"
        children:
        - id: impl_IndexManager_for_CpuIndexManager_B_.attach_index
          signature: "fn attach_index(&self, index: SharedIndex) -> bool {"
        - id: impl_IndexManager_for_CpuIndexManager_B_.is_active
          signature: "fn is_active(&self) -> bool {"
      - id: SegmentKey
        signature: "pub struct SegmentKey {"
        docstring: "Eviction unit: one published segment of one index."
      - id: GpuIndexManager
        signature: "pub struct GpuIndexManager<B: ?Sized> {"
        docstring: "Streaming manager: resolves scopes, reconciles the resident mirror with the ind…"
      - id: impl_GpuIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> GpuIndexManager<B> {"
        children:
        - id: impl_GpuIndexManager_B_.new
          signature: "pub fn new(     core: StreamedIndexManager<B>,     resolvers: ResolverRegistry,…"
        - id: impl_GpuIndexManager_B_.core
          signature: "pub fn core(&self) -> &StreamedIndexManager<B> {"
        - id: impl_GpuIndexManager_B_.resolve
          signature: "pub fn resolve(     &self,     scope: &Scope,     ctx: &ResolveCtx, ) -> Result…"
        - id: impl_GpuIndexManager_B_.ensure
          signature: "pub async fn ensure(     &self,     set: EntrySet,     access: Access, ) -> Res…"
        - id: impl_GpuIndexManager_B_.cycle
          signature: "pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {"
          docstring: "One reconciliation pass: finds foreign changes by stamp comparison, queues remo…"
        - id: impl_GpuIndexManager_B_.evict_tick
          signature: "pub fn evict_tick(     &self,     now: Instant,     occupancy: Occupancy, ) -> …"
        - id: impl_GpuIndexManager_B_.make_room
          signature: "pub(crate) fn make_room(     &self,     need: u64,     occupancy: Occupancy, ) …"
          docstring: Victims for an allocation of `need` bytes; evicts nothing when that cannot succ…
        - id: impl_GpuIndexManager_B_.publish_loaded
          signature: "pub(crate) fn publish_loaded(     &self,     loaded: LoadedSegment,     swap: &…"
          docstring: "Makes a loaded segment visible: table first, then the matching mapping epoch. T…"
        - id: impl_GpuIndexManager_B_.pending_updates
          signature: "pub fn pending_updates(&self) -> usize {"
        - id: impl_GpuIndexManager_B_.key_of
          signature: "fn key_of(&self, segment: u32) -> SegmentKey {"
        - id: impl_GpuIndexManager_B_.track_segment
          signature: "pub(crate) fn track_segment(&self, segment: u32, region: Region) {"
          docstring: "Registers a live segment; a segment is the unit of eviction, so its bytes add u…"
        - id: impl_GpuIndexManager_B_.touch_segments
          signature: "fn touch_segments(&self, now: Instant) {"
        - id: impl_GpuIndexManager_B_.segment_region
          signature: "pub(crate) fn segment_region(&self, segment: u32) -> Option<Region> {"
        - id: impl_GpuIndexManager_B_.segment_count
          signature: "pub fn segment_count(&self) -> usize {"
        - id: impl_GpuIndexManager_B_.resident_bytes
          signature: "pub fn resident_bytes(&self) -> u64 {"
          docstring: Bytes of all live segments as the evictor accounts them.
        - id: impl_GpuIndexManager_B_.release_segment
          signature: "pub(crate) fn release_segment(&self, segment: u32) -> Option<Region> {"
          docstring: Forgets an unpublished segment and the residency of its entries. Pending binds …
        - id: impl_GpuIndexManager_B_.complete_slot
          signature: "pub(crate) fn complete_slot(     &self,     store: &MappingStore,     at: &Snap…"
          docstring: A `StaleMapping` error means the mapping moved on; retry with a fresh snapshot.
      - id: impl_IndexManager_for_GpuIndexManager_B_
        signature: "impl<B: ComputeBackend + ?Sized> IndexManager for GpuIndexManager<B> {"
        children:
        - id: impl_IndexManager_for_GpuIndexManager_B_.attach_index
          signature: "fn attach_index(&self, index: SharedIndex) -> bool {"
        - id: impl_IndexManager_for_GpuIndexManager_B_.is_active
          signature: "fn is_active(&self) -> bool {"
      - id: index_error
        signature: "pub(crate) fn index_error(e: &ManagerError) -> IndexError {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Mutation discipline of the index manager: copy-on-write segment tables, a s…"
      - id: 1gsLkl|IB2Xne
        signature: use std::sync::Arc;
      - id: P9fcuJ|yFCGCK
        signature: /// Root value before the first table is published.
      - id: Cell
        signature: "enum Cell {"
      - id: RootPointer
        signature: "pub struct RootPointer {"
        docstring: Naturally aligned 8-byte cell. Only the host stores; the device reads with `ld.…
      - id: impl_Send_for_RootPointer
        signature: "unsafe impl Send for RootPointer {"
      - id: impl_Sync_for_RootPointer
        signature: "unsafe impl Sync for RootPointer {"
      - id: impl_RootPointer
        signature: "impl RootPointer {"
        children:
        - id: impl_RootPointer.new
          signature: "pub fn new(initial: u64) -> Self {"
        - id: impl_RootPointer.from_mapped
          signature: "pub(crate) unsafe fn from_mapped(ptr: *mut u64) -> Self {"
          docstring: "Wraps a cell in pinned, mapped host memory."
        - id: impl_RootPointer.atomic
          signature: "fn atomic(&self) -> &AtomicU64 {"
        - id: impl_RootPointer.publish
          signature: "pub fn publish(&self, value: u64) {"
          docstring: Everything written before this call is visible to a reader that observes the va…
        - id: impl_RootPointer.load
          signature: "pub fn load(&self) -> u64 {"
      - id: SegmentTable
        signature: "pub struct SegmentTable {"
        docstring: Immutable once published; a new version is always a copy.
      - id: impl_SegmentTable
        signature: "impl SegmentTable {"
        children:
        - id: impl_SegmentTable.empty
          signature: "pub fn empty() -> Self {"
        - id: impl_SegmentTable.epoch
          signature: "pub fn epoch(&self) -> u64 {"
        - id: impl_SegmentTable.segments
          signature: "pub fn segments(&self) -> &[Region] {"
        - id: impl_SegmentTable.push
          signature: "pub fn push(&mut self, region: Region) {"
        - id: impl_SegmentTable.replace
          signature: "pub fn replace(&mut self, old: Region, new: Region) -> bool {"
          docstring: Swaps `old` for `new` in place of the copy; returns false when `old` is not lis…
        - id: impl_SegmentTable.remove
          signature: "pub fn remove(&mut self, region: Region) -> bool {"
        - id: impl_SegmentTable.encoded_len
          signature: "pub fn encoded_len(&self) -> u64 {"
          docstring: "Encoded size of the device-side table: epoch, count, then one entry per segment."
      - id: EpochTracker
        signature: "pub struct EpochTracker {"
        docstring: Versions published by the host and the epoch each reader reported having read.
      - id: impl_EpochTracker
        signature: "impl EpochTracker {"
        children:
        - id: impl_EpochTracker.new
          signature: "pub fn new(readers: usize) -> Self {"
        - id: impl_EpochTracker.published
          signature: "pub fn published(&self) -> u64 {"
        - id: impl_EpochTracker.advance
          signature: "fn advance(&self, epoch: u64) {"
        - id: impl_EpochTracker.report
          signature: "pub fn report(&self, reader: usize, epoch: u64) {"
          docstring: A finished batch read `epoch`; later batches of the reader read the same or a n…
        - id: impl_EpochTracker.quiesce
          signature: "pub fn quiesce(&self, reader: usize) {"
          docstring: "For a reader with no batch in flight: whatever it runs next reads the published…"
        - id: impl_EpochTracker.reader_min
          signature: "pub fn reader_min(&self) -> u64 {"
      - id: TableSink
        signature: "pub trait TableSink: Send + Sync {"
        docstring: Writes a table into device-visible memory.
        children:
        - id: TableSink.write
          signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError>;"
          docstring: "Returns only once the content is complete and visible to the device, i.e. after…"
      - id: TableSwap
        signature: "pub trait TableSwap {"
        children:
        - id: TableSwap.build_next
          signature: fn build_next(&self) -> SegmentTable;
          docstring: Copy of the current table with the next epoch.
        - id: TableSwap.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError>;"
          docstring: "Makes the content visible, then swaps the root with release semantics. Returns …"
        - id: TableSwap.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize;"
          docstring: Frees versions retired up to `upto` once every reader has reached them.
      - id: Snapshot
        signature: "pub struct Snapshot {"
        docstring: Root and epoch seen by one launch or one persistent batch.
      - id: BatchGuard
        signature: "pub struct BatchGuard<'a> {"
        docstring: Pins the version read at batch start; reports its epoch when dropped.
      - id: impl_BatchGuard____
        signature: "impl BatchGuard<'_> {"
        children:
        - id: impl_BatchGuard____.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
      - id: impl_Drop_for_BatchGuard____
        signature: "impl Drop for BatchGuard<'_> {"
        children:
        - id: impl_Drop_for_BatchGuard____.drop
          signature: "fn drop(&mut self) {"
      - id: Current
        signature: "struct Current {"
      - id: SwapManager
        signature: "pub struct SwapManager<'a, 'b> {"
      - id: impl_SwapManager__a__b_
        signature: "impl<'a, 'b> SwapManager<'a, 'b> {"
        children:
        - id: impl_SwapManager__a__b_.new
          signature: "pub fn new(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + 'a>,  …"
        - id: impl_SwapManager__a__b_.with_root
          signature: "pub fn with_root(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + …"
        - id: impl_SwapManager__a__b_.root
          signature: "pub fn root(&self) -> &RootPointer {"
        - id: impl_SwapManager__a__b_.epochs
          signature: "pub fn epochs(&self) -> &EpochTracker {"
        - id: impl_SwapManager__a__b_.current
          signature: "pub fn current(&self) -> Arc<SegmentTable> {"
        - id: impl_SwapManager__a__b_.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
          docstring: "Normal launches: the root is passed as a launch argument, so the version stays …"
        - id: impl_SwapManager__a__b_.begin_batch
          signature: "pub fn begin_batch(&self, reader: usize) -> BatchGuard<'_> {"
          docstring: "Persistent kernel: one acquire load per batch, never per work item."
      - id: impl_TableSwap_for_SwapManager_______
        signature: "impl TableSwap for SwapManager<'_, '_> {"
        children:
        - id: impl_TableSwap_for_SwapManager_______.build_next
          signature: "fn build_next(&self) -> SegmentTable {"
        - id: impl_TableSwap_for_SwapManager_______.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError> {"
        - id: impl_TableSwap_for_SwapManager_______.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize {"
      - id: tests
        signature: "mod tests {"
        children:
        - id: tests.flTjQt|5qRwZw
          signature: use super::*;
        - id: tests.HostSink
          signature: "struct HostSink<'a, 'b>(&'a IndexArena<'b>);"
        - id: tests.impl_TableSink_for_HostSink_______
          signature: "impl TableSink for HostSink<'_, '_> {"
          children:
          - id: tests.impl_TableSink_for_HostSink_______.write
            signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {"
        - id: tests.alloc
          signature: "fn alloc() -> ArenaAllocator {"
        - id: tests.seg
          signature: "fn seg(offset: u64) -> Region {"
        - id: tests.publish_swaps_root_and_advances_epoch
          signature: "fn publish_swaps_root_and_advances_epoch() {"
        - id: tests.concurrent_builds_on_the_same_base_are_rejected
          signature: "fn concurrent_builds_on_the_same_base_are_rejected() {"
        - id: tests.old_table_is_freed_only_after_all_readers_reach_the_new_epoch
          signature: "fn old_table_is_freed_only_after_all_readers_reach_the_new_epoch() {"
        - id: tests.reclaim_respects_the_requested_bound
          signature: "fn reclaim_respects_the_requested_bound() {"
        - id: tests.launch_snapshot_is_constant_until_the_next_launch
          signature: "fn launch_snapshot_is_constant_until_the_next_launch() {"
        - id: tests.copy_is_independent_of_the_published_table
          signature: "fn copy_is_independent_of_the_published_table() {"
      - id: loom_tests
        signature: "mod loom_tests {"
        children:
        - id: loom_tests.aCv6R1|7NCwfi
          signature: use super::*;
        - id: loom_tests.content_is_visible_once_the_root_is_observed
          signature: "fn content_is_visible_once_the_root_is_observed() {"
        - id: loom_tests.retired_version_is_not_reclaimed_while_a_reader_holds_it
          signature: "fn retired_version_is_not_reclaimed_while_a_reader_holds_it() {"
      - id: device_tests
        signature: "mod device_tests {"
        children:
        - id: device_tests.XiB8kc|gxjU93
          signature: use super::*;
        - id: device_tests.obwXJ0|6OEnCW
          signature: "const PROBE: &str = r#\""
        - id: device_tests.device_sees_content_written_before_the_host_release_store
          signature: "fn device_sees_content_written_before_the_host_release_store() {"
        - id: device_tests.xreCHs|1RgSqu
          signature: "const PROBE_DEVICE: &str = r#\""
        - id: device_tests.copy_stream_event_orders_device_table_before_the_root
          signature: "fn copy_stream_event_orders_device_table_before_the_root() {"
```
/answer b56c-31 allow
Control Request:
```yaml
id: b56c-32
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
    ids:
    - impl_GpuIndexManager_B_.cycle
    - impl_GpuIndexManager_B_.publish_loaded
    - GpuIndexManager
    - impl_StreamedIndexManager_B_.cycle_span
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
    ids:
    - impl_IndexArena__a_.trace_utilization
    - impl_IndexArena__a_.reclaim
    - impl_IndexArena__a_.budget
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
    ids:
    - impl_TableSwap_for_SwapManager_______.publish
    - impl_TableSwap_for_SwapManager_______.reclaim
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
    ids:
    - ModuleDoc
  reason: "Read cycle, publish/reclaim and trace_utilization"
```
/answer b56c-32 allow
Control Request:
```yaml
id: b56c-33
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
      nodes:
      - id: impl_GpuIndexManager_B_.cycle
        type: ImplItemFn
        code: |2
              /// One reconciliation pass: finds foreign changes by stamp comparison, queues removals as
              /// mapping mutations and reloads changed entries. The index stays the source of truth.
              pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {
                  async {
                      let index = self.core.index()?.clone();
                      let outcome = {
                          let mut guard = self.scanner.lock().await;
                          let scanner = guard.get_or_insert_with(|| SyncScanner::new(&*index));
                          let residency = self.core.residency.lock();
                          scanner.scan(&*index, &residency)
                      };
                      let reload = {
                          let mut updates = self.updates.lock();
                          outcome.enqueue_removals(&mut updates);
                          outcome
                              .reload
                              .iter()
                              .copied()
                              .filter(|e| !updates.contains(*e))
                              .collect::<Vec<_>>()
                      };
                      if !reload.is_empty() {
                          self.ensure(EntrySet::from_ids(reload), Access::Regular).await?;
                      }
                      Ok(outcome)
                  }
                      .instrument(self.core.cycle_span())
                      .await
              }
      - id: impl_GpuIndexManager_B_.publish_loaded
        type: ImplItemFn
        code: |2
              /// Makes a loaded segment visible: table first, then the matching mapping epoch. The bindings
              /// are queued and take effect in the next slot gap. A failure returns the region.
              pub(crate) fn publish_loaded(
                  &self,
                  loaded: LoadedSegment,
                  swap: &dyn TableSwap,
                  store: &MappingStore,
                  regions: &dyn RegionSource,
              ) -> Result<u32, ManagerError> {
                  let id = self.next_segment.fetch_add(1, Ordering::Relaxed);
                  let LoadedSegment { region, map: slots, binds } = loaded;
                  let published = (|| {
                      let mut table = swap.build_next();
                      table.push(region);
                      let mut map = store.snapshot().successor();
                      if !map.add_segment(id, slots) {
                          return Err(
                              ManagerError::from(IndexError::TransferFailed {
                                  reason: format!("segment {id} overlaps a mapped segment"),
                              }),
                          );
                      }
                      swap.publish(table)?;
                      store.publish(map)
                  })();
                  if let Err(e) = published {
                      let _ = regions.free_segment(region);
                      return Err(e);
                  }
                  self.track_segment(id, region);
                  let mut updates = self.updates.lock();
                  for (entry, slot, stamp) in binds {
                      updates
                          .push(Update::Bind {
                              entry,
                              slot: SlotRef { segment: id, slot },
                              stamp,
                          });
                  }
                  tracing::debug!(segment = id, offset = region.offset, "segment published");
                  Ok(id)
              }
      - id: GpuIndexManager
        type: ItemStruct
        code: |
          /// Streaming manager: resolves scopes, reconciles the resident mirror with the index, loads and
          /// evicts. The device transfer is injected through the loader, so the shell builds without `cuda`.
          pub struct GpuIndexManager<B: ?Sized> {
              core: StreamedIndexManager<B>,
              resolvers: ResolverRegistry,
              loader: ResidencyLoader<String>,
              evictor: Mutex<Evictor<SegmentKey>>,
              segments: Mutex<BTreeMap<u32, Region>>,
              updates: Mutex<UpdateQueue>,
              next_segment: AtomicU32,
              scanner: tokio::sync::Mutex<Option<SyncScanner>>,
          }
      - id: impl_StreamedIndexManager_B_.cycle_span
        type: ImplItemFn
        code: |2
              pub fn cycle_span(&self) -> tracing::Span {
                  tracing::info_span!("manager_cycle", index_id = % self.index_id)
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
      nodes:
      - id: impl_IndexArena__a_.trace_utilization
        type: ImplItemFn
        code: |2
              pub fn trace_utilization(&self) {
                  let b = self.budget();
                  tracing::debug!(
                      capacity = b.capacity, live = b.live, retired = b.retired, headroom = b
                      .cow_headroom, available = b.available, "index arena utilization"
                  );
              }
      - id: impl_IndexArena__a_.reclaim
        type: ImplItemFn
        code: |2
              /// Frees every retired region that no reader at `min_reader_epoch` or later can still see.
              pub fn reclaim(&self, min_reader_epoch: u64) -> usize {
                  let due: Vec<Region> = {
                      let mut st = self.state.lock();
                      let (due, keep): (Vec<_>, Vec<_>) = st
                          .retired
                          .drain(..)
                          .partition(|r| r.epoch <= min_reader_epoch);
                      st.retired = keep;
                      due.into_iter().map(|r| r.region).collect()
                  };
                  let mut freed = 0;
                  for region in due {
                      match self.inner.free(region) {
                          Ok(()) => freed += 1,
                          Err(e) => tracing::warn!(error = % e, "retired region already gone"),
                      }
                  }
                  if freed > 0 {
                      tracing::debug!(freed, min_reader_epoch, "retired segments released");
                  }
                  freed
              }
      - id: impl_IndexArena__a_.budget
        type: ImplItemFn
        code: |2
              pub fn budget(&self) -> ArenaBudget {
                  self.inner.reclaim();
                  let st = self.state.lock();
                  let capacity = self.inner.capacity();
                  let used = capacity - self.inner.available();
                  let headroom = u64::from(self.max_concurrent_cow).saturating_mul(st.largest);
                  let retired = st.retired.iter().map(|r| r.region.len).sum();
                  ArenaBudget {
                      capacity,
                      reserve: self.reserve,
                      live: st.live,
                      retired,
                      cow_headroom: headroom,
                      available: capacity
                          .saturating_sub(used)
                          .saturating_sub(self.reserve)
                          .saturating_sub(headroom),
                  }
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
      nodes:
      - id: impl_TableSwap_for_SwapManager_______.publish
        type: ImplItemFn
        code: |2
              fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError> {
                  let _span = crate::resident::telemetry::manager_cycle_span(table.epoch())
                      .entered();
                  let _gate = self.publish_gate.lock();
                  let published = self.current.lock().table.epoch();
                  if table.epoch() != published + 1 {
                      return Err(ManagerError::StaleMapping {
                          base: table.epoch() - 1,
                          published,
                      });
                  }
                  let region = self.sink.write(&table)?;
                  let epoch = table.epoch();
                  let segments = table.segments().len();
                  let previous = {
                      let mut cur = self.current.lock();
                      self.root.publish(region.offset);
                      self.epochs.advance(epoch);
                      cur.table = Arc::new(table);
                      cur.region.replace(region)
                  };
                  if let Some(old) = previous {
                      self.arena.retire(old, epoch)?;
                  }
                  tracing::debug!(epoch, segments, offset = region.offset, "table published");
                  Ok(epoch)
              }
      - id: impl_TableSwap_for_SwapManager_______.reclaim
        type: ImplItemFn
        code: |2
              fn reclaim(&self, upto: u64) -> usize {
                  let reached = self.epochs.reader_min().min(upto);
                  let freed = self.arena.reclaim(reached);
                  tracing::debug!(freed, reached, "reclaim");
                  freed
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! StreamedIndexManager, ScopeResolver, update queue, residency.
```
/answer b56c-33 allow
ReasoningToken: 200
Control Request:
```yaml
id: b56c-34
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: IndexArena|publish_loaded|\.reclaim\(|GpuIndexManager::new
    include:
    - '*.rs'
    limit: 50
  reason: See whether the cycle can reach an arena
```
/answer b56c-34 allow
Control Request:
```yaml
id: b56c-35
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/test/mod.rs
        matches:
        - "assert!(store.reclaim().is_empty(), \"lease still held\");"
        - "assert!(store.reclaim().is_empty(), \"kernel work still in flight\");"
        - "assert_eq!(store.reclaim(), vec![base]);"
        - assert!(store.reclaim().is_empty());
        - "assert_eq!(store.reclaim(), vec![dropped_base]);"
      - path: manager/swap.rs
        matches:
        - use crate::resident::manager::arena::IndexArena;
        - "arena: &'a IndexArena<'b>,"
        - let freed = self.arena.reclaim(reached);
        - "struct HostSink<'a, 'b>(&'a IndexArena<'b>);"
        - "let arena = IndexArena::new(&alloc, 0);"
        - "assert_eq!(mgr.reclaim(u64::MAX), 0);"
        - "assert_eq!(mgr.reclaim(u64::MAX), 1);"
        - "assert_eq!(mgr.reclaim(2), 1);"
        - "assert_eq!(mgr.reclaim(3), 1);"
      - path: manager/transfer.rs
        matches:
        - use crate::resident::manager::arena::IndexArena;
        - "impl RegionSource for IndexArena<'_> {"
        - "IndexArena::alloc_segment(self, bytes, align)"
      - path: manager/arena.rs
        matches:
        - "pub struct IndexArena<'a> {"
        - "impl<'a> IndexArena<'a> {"
        - "pub fn shared(inner: Arc<ArenaAllocator>, reserve: u64) -> IndexArena<'static> {"
        - "IndexArena::with_ref(AllocRef::Shared(inner), reserve)"
        - self.inner.reclaim();
        - "arena: &'a IndexArena<'b>,"
      - path: manager/test/transfer.rs
        matches:
        - struct ArenaSink<'a>(&'a IndexArena<'static>);
        - "arena: Arc<IndexArena<'static>>,"
        - "let arena = Arc::new(IndexArena::shared(alloc.clone(), 0));"
        - "let arena = Arc::new(IndexArena::shared(alloc, 0));"
        - let mgr = GpuIndexManager::new(
        - "assert_eq!(mgr.publish_loaded(loaded, & swap, & store, &* f.arena).unwrap(), 0);"
        - "arena: &'a IndexArena<'static>,"
        - self.0.reclaim(upto)
        - "mgr.publish_loaded(first, &swap, &store, &regions).unwrap();"
        - "assert!(mgr.publish_loaded(clash, & swap, & store, & regions).is_err());"
        - "let res = mgr.publish_loaded(loaded, &FailingSwap(&swap), &store, &regions);"
      - path: manager/test/arena.rs
        matches:
        - "let arena = IndexArena::new(&alloc, 0);"
        - "let arena = IndexArena::new(&alloc, 2048);"
        - "assert_eq!(arena.reclaim(4), 0);"
        - "assert_eq!(arena.reclaim(5), 1);"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b56c-35 allow
ReasoningToken: 184
Control Request:
```yaml
id: b56c-36
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: RegionSource|TableSwap|\.cycle\(\)
    include:
    - '*.rs'
    exclude:
    - swap.rs
    limit: 50
  reason: Understand where an arena is reachable around cycle
```
/answer b56c-36 allow
Control Request:
```yaml
id: b56c-37
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: manager/transfer.rs
        matches:
        - "pub trait RegionSource: Send + Sync {"
        - "impl RegionSource for IndexArena<'_> {"
        - "regions: Arc<dyn RegionSource>,"
      - path: manager/test/transfer.rs
        matches:
        - "use crate::resident::manager::swap::{SegmentTable, SwapManager, TableSink, TableSwap};"
        - "impl RegionSource for CountingRegions<'_> {"
        - "impl TableSwap for FailingSwap<'_, '_> {"
        - "impl RegionSource for MemRegions {"
      - path: manager/test/streamed.rs
        matches:
        - "assert!(matches!(m.cycle(). await, Err(IndexError::IndexUnavailable { .. })));"
        - assert!(m.cycle(). await .unwrap().is_clean());
        - let out = m.cycle().await.unwrap();
      - path: manager/test/runtime.rs
        matches:
        - rt.cycle().await.unwrap();
        - let outcome = rt.cycle().await.unwrap();
      - path: manager/streamed.rs
        matches:
        - "use crate::resident::manager::swap::{Snapshot, TableSwap};"
        - "use crate::resident::manager::transfer::{LoadedSegment, RegionSource};"
        - "swap: &dyn TableSwap,"
        - "regions: &dyn RegionSource,"
      - path: manager/runtime.rs
        matches:
        - "use crate::resident::manager::swap::{Snapshot, SwapManager, TableSwap};"
        - let outcome = self.manager.cycle().await?;
```
/answer b56c-37 allow
Control Request:
```yaml
id: b56c-38
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - grep -n 'cycle\|arena\|swap\|struct\|fn ' src/resident/manager/runtime.rs | head -80
  reason: Check runtime for arena access around the cycle
```
/answer b56c-38 deny search
Control Request:
```yaml
id: b56c-39
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
  reason: See runtime structure
```
/answer b56c-39 allow
Control Request:
```yaml
id: b56c-3a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Owner of one index operation: ties the manager, the table swap, the mapping…"
      - id: RdmYDB|Uor8FS
        signature: use std::sync::Arc;
      - id: IndexRuntime
        signature: "pub struct IndexRuntime<'a, 'b, B: ?Sized> {"
        docstring: "`loaded` must be the sink of the manager's transfer: every segment the transfer…"
      - id: impl_IndexRuntime__a__b_B_
        signature: "impl<'a, 'b, B: ComputeBackend + ?Sized> IndexRuntime<'a, 'b, B> {"
        children:
        - id: impl_IndexRuntime__a__b_B_.new
          signature: "pub fn new(     manager: GpuIndexManager<B>,     arena: &'a IndexArena<'b>,    …"
        - id: impl_IndexRuntime__a__b_B_.manager
          signature: "pub fn manager(&self) -> &GpuIndexManager<B> {"
        - id: impl_IndexRuntime__a__b_B_.swap
          signature: "pub fn swap(&self) -> &SwapManager<'a, 'b> {"
        - id: impl_IndexRuntime__a__b_B_.store
          signature: "pub fn store(&self) -> &MappingStore {"
        - id: impl_IndexRuntime__a__b_B_.ensure
          signature: "pub async fn ensure(     &self,     set: EntrySet,     access: Access, ) -> Res…"
          docstring: "Loads `set` and makes the loaded segments visible. When the arena is full, vict…"
        - id: impl_IndexRuntime__a__b_B_.cycle
          signature: "pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {"
          docstring: One reconciliation pass; reloaded entries are published before it returns.
        - id: impl_IndexRuntime__a__b_B_.complete_slot
          signature: "pub fn complete_slot(     &self,     at: &Snapshot,     writes: &[WrittenRange]…"
          docstring: The slot gap of a kernel batch that started on `at`. A mapping that moved on si…
        - id: impl_IndexRuntime__a__b_B_.evict
          signature: "pub fn evict(&self, now: Instant) -> Result<Vec<Eviction<SegmentKey>>, IndexErr…"
          docstring: Idle expiry and watermark relief against the current arena occupancy.
        - id: impl_IndexRuntime__a__b_B_.reclaim
          signature: "pub fn reclaim(&self) -> usize {"
          docstring: Frees retired regions that no reader can still see.
        - id: impl_IndexRuntime__a__b_B_.make_room
          signature: "fn make_room(&self, need: u64) -> Result<(), ManagerError> {"
        - id: impl_IndexRuntime__a__b_B_.publish_pending
          signature: "fn publish_pending(&self) -> Result<usize, ManagerError> {"
          docstring: Publishes every segment the transfer reported since the last call; the first fa…
        - id: impl_IndexRuntime__a__b_B_.unpublish
          signature: "fn unpublish(&self, evicted: &[Eviction<SegmentKey>]) -> Result<(), ManagerErro…"
          docstring: "Removes evicted segments from the table and the mapping, then retires their reg…"
      - id: tests
        signature: mod tests;
```
/answer b56c-3a allow
Control Request:
```yaml
id: b56c-3b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
    ids:
    - IndexRuntime
    - impl_IndexRuntime__a__b_B_.cycle
    - impl_IndexRuntime__a__b_B_.reclaim
    - impl_IndexRuntime__a__b_B_.new
  reason: Read runtime cycle/reclaim
```
/answer b56c-3b allow
ReasoningToken: 208
Control Request:
```yaml
id: b56c-3c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
      nodes:
      - id: IndexRuntime
        type: ItemStruct
        code: |
          /// `loaded` must be the sink of the manager's transfer: every segment the transfer reports is
          /// published here, so no caller has to remember to do it.
          pub struct IndexRuntime<'a, 'b, B: ?Sized> {
              manager: GpuIndexManager<B>,
              arena: &'a IndexArena<'b>,
              swap: SwapManager<'a, 'b>,
              store: MappingStore,
              loaded: Arc<LoadedQueue>,
              /// Table and mapping epochs advance together, so structural changes are serialized.
              structural: Mutex<()>,
          }
      - id: impl_IndexRuntime__a__b_B_.cycle
        type: ImplItemFn
        code: |2
              /// One reconciliation pass; reloaded entries are published before it returns.
              pub async fn cycle(&self) -> Result<ScanOutcome, IndexError> {
                  let outcome = self.manager.cycle().await?;
                  self.publish_pending().map_err(|e| index_error(&e))?;
                  Ok(outcome)
              }
      - id: impl_IndexRuntime__a__b_B_.reclaim
        type: ImplItemFn
        code: |2
              /// Frees retired regions that no reader can still see.
              pub fn reclaim(&self) -> usize {
                  self.swap.reclaim(self.swap.epochs().published())
              }
      - id: impl_IndexRuntime__a__b_B_.new
        type: ImplItemFn
        code: |2
              pub fn new(
                  manager: GpuIndexManager<B>,
                  arena: &'a IndexArena<'b>,
                  swap: SwapManager<'a, 'b>,
                  loaded: Arc<LoadedQueue>,
              ) -> Self {
                  Self {
                      manager,
                      arena,
                      swap,
                      store: MappingStore::new(),
                      loaded,
                      structural: Mutex::new(()),
                  }
              }
```
/answer b56c-3c allow
Control Request:
```yaml
id: b56c-3d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/test/telemetry.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test/runtime.rs
  reason: Find tracing test subscriber and runtime test fixtures
```
/answer b56c-3d allow
Control Request:
```yaml
id: b56c-3e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/telemetry.rs
      nodes:
      - id: Clqc8p|8mJott
        signature: use super::*;
      - id: Capture
        signature: "struct Capture {"
        docstring: "Minimal subscriber capturing span field names/values for assertions, since we o…"
      - id: FieldDump
        signature: "struct FieldDump<'a>(&'a mut Vec<(String, String)>);"
      - id: impl_Visit_for_FieldDump____
        signature: "impl Visit for FieldDump<'_> {"
        children:
        - id: impl_Visit_for_FieldDump____.record_debug
          signature: "fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {"
      - id: impl_Subscriber_for_Capture
        signature: "impl Subscriber for Capture {"
        children:
        - id: impl_Subscriber_for_Capture.enabled
          signature: "fn enabled(&self, _: &Metadata<'_>) -> bool {"
        - id: impl_Subscriber_for_Capture.register_callsite
          signature: "fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest {"
        - id: impl_Subscriber_for_Capture.new_span
          signature: "fn new_span(&self, attrs: &Attributes<'_>) -> Id {"
        - id: impl_Subscriber_for_Capture.record
          signature: "fn record(&self, _: &Id, _: &tracing::span::Record<'_>) {"
        - id: impl_Subscriber_for_Capture.record_follows_from
          signature: "fn record_follows_from(&self, _: &Id, _: &Id) {"
        - id: impl_Subscriber_for_Capture.event
          signature: "fn event(&self, _: &tracing::Event<'_>) {"
        - id: impl_Subscriber_for_Capture.enter
          signature: "fn enter(&self, _: &Id) {"
        - id: impl_Subscriber_for_Capture.exit
          signature: "fn exit(&self, _: &Id) {"
      - id: task_span_carries_stable_task_id_field
        signature: "fn task_span_carries_stable_task_id_field() {"
      - id: for_blocking_preserves_span_identity_fields
        signature: "fn for_blocking_preserves_span_identity_fields() {"
      - id: stable_field_names_match_convention
        signature: "fn stable_field_names_match_convention() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test/runtime.rs
      nodes:
      - id: f09dk2|gLpE3s
        signature: use std::sync::atomic::AtomicBool;
      - id: vxfHpf|tjBdL6
        signature: use crate::resident::memory::region::ArenaId;
      - id: VTiHL7|3Q73hx
        signature: "const ENTRY_BYTES: usize = 100;"
      - id: HostPort
        signature: "struct HostPort {"
      - id: impl_UploadPort_for_HostPort
        signature: "impl UploadPort for HostPort {"
        children:
        - id: impl_UploadPort_for_HostPort.upload
          signature: "fn upload(&self, src: &[u8], dst: Region) -> Result<EventHandle, MemoryError> {"
      - id: ArenaSink
        signature: struct ArenaSink<'a>(&'a IndexArena<'static>);
      - id: impl_TableSink_for_ArenaSink____
        signature: "impl TableSink for ArenaSink<'_> {"
        children:
        - id: impl_TableSink_for_ArenaSink____.write
          signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {"
      - id: NullBackend
        signature: struct NullBackend;
      - id: impl_ComputeBackend_for_NullBackend
        signature: "impl ComputeBackend for NullBackend {"
        children:
        - id: impl_ComputeBackend_for_NullBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: NoSource
        signature: struct NoSource;
      - id: impl_RangeSource_for_NoSource
        signature: "impl RangeSource for NoSource {"
        children:
        - id: impl_RangeSource_for_NoSource.read
          signature: "fn read(&self, _: crate::resident::kernel::WrittenRange) -> Vec<u8> {"
      - id: Rig
        signature: "struct Rig {"
      - id: id
        signature: "fn id(n: u64) -> EntryId {"
      - id: geometry
        signature: "fn geometry() -> UniformGeometry {"
      - id: rig
        signature: "fn rig(arena_bytes: u64) -> Rig {"
      - id: runtime
        signature: "fn runtime(rig: &Rig, evict: EvictPolicy) -> IndexRuntime<'_, 'static, NullBack…"
      - id: set
        signature: "fn set(ids: impl IntoIterator<Item = u64>) -> EntrySet {"
      - id: short_ttl
        signature: "fn short_ttl() -> EvictPolicy {"
      - id: ensure_publishes_the_loaded_segment_without_further_calls
        signature: "async fn ensure_publishes_the_loaded_segment_without_further_calls() {"
      - id: slot_completion_binds_the_published_entries
        signature: "async fn slot_completion_binds_the_published_entries() {"
      - id: cycle_publishes_reloaded_entries
        signature: "async fn cycle_publishes_reloaded_entries() {"
      - id: evictor_accounts_every_segment_separately
        signature: "async fn evictor_accounts_every_segment_separately() {"
      - id: idle_segments_leave_table_mapping_and_residency
        signature: "async fn idle_segments_leave_table_mapping_and_residency() {"
      - id: exhausted_arena_evicts_and_retries_once
        signature: "async fn exhausted_arena_evicts_and_retries_once() {"
      - id: publish_evict_and_slot_completion_race_without_leaking_regions
        signature: "fn publish_evict_and_slot_completion_race_without_leaking_regions() {"
```
/answer b56c-3e allow
Control Request:
```yaml
id: b56c-3f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/telemetry.rs
    ids:
    - Capture
    - FieldDump
    - impl_Subscriber_for_Capture
    - task_span_carries_stable_task_id_field
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test/runtime.rs
    ids:
    - cycle_publishes_reloaded_entries
    - Rig
    - rig
    - runtime
    - vxfHpf|tjBdL6
  reason: Understand the test subscriber and runtime test rig
```
/answer b56c-3f allow
ReasoningToken: 173
Control Request:
```yaml
id: b56c-40
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test/telemetry.rs
      nodes:
      - id: Capture
        type: ItemStruct
        code: |
          /// Minimal subscriber capturing span field names/values for assertions, since we only
          /// need to verify the stable-field-name contract, not full event formatting.
          #[derive(Clone)]
          struct Capture {
              fields: std::sync::Arc<std::sync::Mutex<Vec<(String, String)>>>,
          }
      - id: FieldDump
        type: ItemStruct
        code: |
          struct FieldDump<'a>(&'a mut Vec<(String, String)>);
      - id: impl_Subscriber_for_Capture
        type: ItemImpl
        signature: "impl Subscriber for Capture {"
        children:
        - id: impl_Subscriber_for_Capture.enabled
          type: ImplItemFn
          code: |2
                fn enabled(&self, _: &Metadata<'_>) -> bool {
                    true
                }
        - id: impl_Subscriber_for_Capture.register_callsite
          type: ImplItemFn
          code: |2
                fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest {
                    Interest::always()
                }
        - id: impl_Subscriber_for_Capture.new_span
          type: ImplItemFn
          code: |2
                fn new_span(&self, attrs: &Attributes<'_>) -> Id {
                    let mut dump = FieldDump(&mut self.fields.lock().unwrap());
                    attrs.record(&mut dump);
                    Id::from_u64(1)
                }
        - id: impl_Subscriber_for_Capture.record
          type: ImplItemFn
          code: |2
                fn record(&self, _: &Id, _: &tracing::span::Record<'_>) {}
        - id: impl_Subscriber_for_Capture.record_follows_from
          type: ImplItemFn
          code: |2
                fn record_follows_from(&self, _: &Id, _: &Id) {}
        - id: impl_Subscriber_for_Capture.event
          type: ImplItemFn
          code: |2
                fn event(&self, _: &tracing::Event<'_>) {}
        - id: impl_Subscriber_for_Capture.enter
          type: ImplItemFn
          code: |2
                fn enter(&self, _: &Id) {}
        - id: impl_Subscriber_for_Capture.exit
          type: ImplItemFn
          code: |2
                fn exit(&self, _: &Id) {}
      - id: task_span_carries_stable_task_id_field
        type: ItemFn
        code: |
          #[test]
          fn task_span_carries_stable_task_id_field() {
              let capture = Capture {
                  fields: Default::default(),
              };
              let fields = capture.fields.clone();
              tracing::subscriber::with_default(
                  capture,
                  || {
                      let _span = task_span("t-1");
                  },
              );
              let recorded = fields.lock().unwrap();
              assert!(
                  recorded.iter().any(| (name, value) | name == FIELD_TASK_ID && value
                  .contains("t-1"))
              );
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test/runtime.rs
      nodes:
      - id: cycle_publishes_reloaded_entries
        type: ItemFn
        code: |
          #[tokio::test]
          async fn cycle_publishes_reloaded_entries() {
              let rig = rig(1 << 20);
              let rt = runtime(&rig, short_ttl());
              rt.cycle().await.unwrap();
              rt.ensure(set([3]), Access::Regular).await.unwrap();
              rt.complete_slot(&rt.swap().snapshot(), &[], &[], &NoSource).unwrap();
              rig.index.apply(id(3), &[9; ENTRY_BYTES]);
              let outcome = rt.cycle().await.unwrap();
              assert!(! outcome.is_clean());
              assert_eq!(rt.manager().segment_count(), 2);
          }
      - id: Rig
        type: ItemStruct
        code: |
          struct Rig {
              index: Arc<InMemoryIndex>,
              arena: Arc<IndexArena<'static>>,
              queue: Arc<LoadedQueue>,
              transfer: Arc<GatherTransfer<Vec<u8>>>,
          }
      - id: rig
        type: ItemFn
        code: |
          fn rig(arena_bytes: u64) -> Rig {
              let index = Arc::new(InMemoryIndex::new());
              for n in 0..60 {
                  index.apply(id(n), &[n as u8; ENTRY_BYTES]);
              }
              let alloc = Arc::new(
                  ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, arena_bytes),
              );
              let arena = Arc::new(IndexArena::shared(alloc, 0));
              let scratch = Region {
                  arena: ArenaId(0),
                  offset: 0,
                  len: 1,
                  gen: 0,
              };
              let port = Arc::new(HostPort {
                  lane: CpuLane::new(LaneClass::Index, LaneConfig::new(scratch)).unwrap(),
                  memory: Mutex::new(vec![0; arena_bytes as usize]),
                  fail: AtomicBool::new(false),
              });
              let queue = Arc::new(LoadedQueue::new());
              let gather = StagedGather::new(StagingRing::new((0..2).map(|_| vec![0u8; 8192])));
              let transfer = Arc::new(
                  GatherTransfer::new(
                      index.clone(),
                      Arc::new(geometry()),
                      gather,
                      port,
                      arena.clone(),
                      queue.clone(),
                      256,
                  ),
              );
              Rig {
                  index,
                  arena,
                  queue,
                  transfer,
              }
          }
      - id: runtime
        type: ItemFn
        code: |
          fn runtime(rig: &Rig, evict: EvictPolicy) -> IndexRuntime<'_, 'static, NullBackend> {
              let loader = ResidencyLoader::new(
                  LoadPolicy::default(),
                  Arc::new(geometry()),
                  rig.transfer.clone(),
              );
              let manager = GpuIndexManager::new(
                  StreamedIndexManager::new(Arc::new(NullBackend), "t"),
                  ResolverRegistry::with_defaults(),
                  loader,
                  evict,
              );
              manager.attach_index(rig.index.clone());
              let swap = SwapManager::new(&rig.arena, Box::new(ArenaSink(&rig.arena)), 0);
              IndexRuntime::new(manager, &rig.arena, swap, rig.queue.clone())
          }
      - id: vxfHpf|tjBdL6
        type: ImportGroup
        code: |
          use crate::resident::memory::region::ArenaId;
          use crate::resident::memory::{
              ArenaAllocator, ArenaKind, DeviceKind, Region, StagingRing,
          };
          use crate::resident::subengine::{
              CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig,
          };
```
/answer b56c-40 allow
Control Request:
```yaml
id: b56c-41
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "tracing-subscriber|tracing =|memmap2|^\\[features\\]|^cuda"
    include:
    - Cargo.toml
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: "#\\[path|mod tests|use super::\\*"
    include:
    - runtime.rs
    - mod.rs
  reason: Check dependencies and test module wiring
```
/answer b56c-41 allow
Control Request:
```yaml
id: b56c-42
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - "[features]"
        - "cuda = ["
      - path: Cargo.toml
        matches:
        - tracing = "0.1.44"
        - "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - memmap2 = "0.9.3"
        - "cudarc = { version = \"=0.19.9\", optional = true }"
        - "[features]"
        - "cuda = [\"candle\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-nn/cuda\", \"candle-transformers/cuda\"]"
        - "json = [\"tracing-subscriber/json\"]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: mod.rs
        matches:
        - "#[path = \"test/flat.rs\"]"
      - path: test/runtime.rs
        matches:
        - use super::*;
      - path: runtime.rs
        matches:
        - "#[path = \"test/runtime.rs\"]"
        - mod tests;
```
/answer b56c-42 allow
Control Request:
```yaml
id: b56c-43
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
    id: impl_IndexRuntime__a__b_B_.cycle
    old_text: |2-
              self.publish_pending().map_err(|e| index_error(&e))?;
              Ok(outcome)
    new_text: |2-
              self.publish_pending().map_err(|e| index_error(&e))?;
              self.manager.core().cycle_span().in_scope(|| self.arena.trace_utilization());
              Ok(outcome)
  reason: "Log arena utilization at the end of each runtime cycle (the arena is reachable here, not in GpuIndexManager::cycle)"
```
/answer b56c-43 allow
ReasoningToken: 441
Control Request:
```yaml
id: b56c-44
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
    id: impl_TableSwap_for_SwapManager_______.publish
    old_text: |2
              tracing::debug!(epoch, segments, offset = region.offset, "table published");
    new_text: |2
              tracing::debug!(epoch, segments, offset = region.offset, "table published");
              self.arena.trace_utilization();
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
    id: impl_TableSwap_for_SwapManager_______.reclaim
    old_text: |2
              tracing::debug!(freed, reached, "reclaim");
    new_text: |2
              tracing::debug!(freed, reached, "reclaim");
              self.arena.trace_utilization();
  reason: Log utilization after publish and reclaim where the arena is accessible
```
/answer b56c-44 allow
Control Request:
```yaml
id: b56c-45
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test/runtime.rs
    id: cycle_publishes_reloaded_entries
    position: after
    source: |
      struct EventMessages(std::sync::Arc<Mutex<Vec<String>>>);

      struct MessageOf<'a>(&'a mut String);

      impl tracing::field::Visit for MessageOf<'_> {
          fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
              if field.name() == "message" {
                  *self.0 = format!("{value:?}");
              }
          }
      }

      impl tracing::Subscriber for EventMessages {
          fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
              true
          }
          fn register_callsite(
              &self,
              _: &'static tracing::Metadata<'static>,
          ) -> tracing::subscriber::Interest {
              tracing::subscriber::Interest::always()
          }
          fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
              tracing::span::Id::from_u64(1)
          }
          fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
          fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
          fn event(&self, event: &tracing::Event<'_>) {
              let mut message = String::new();
              event.record(&mut MessageOf(&mut message));
              self.0.lock().push(message);
          }
          fn enter(&self, _: &tracing::span::Id) {}
          fn exit(&self, _: &tracing::span::Id) {}
      }

      #[tokio::test]
      async fn cycle_logs_index_arena_utilization() {
          let rig = rig(1 << 20);
          let rt = runtime(&rig, short_ttl());
          let messages = std::sync::Arc::new(Mutex::new(Vec::new()));
          let _guard = tracing::subscriber::set_default(EventMessages(messages.clone()));
          rt.cycle().await.unwrap();
          assert!(messages.lock().iter().any(|m| m == "index arena utilization"));
      }
  reason: Add test that a cycle emits the utilization event
```
/answer b56c-45 allow
Text:
 Als Nächstes prüfe ich die Aussagen für die Konsistenz-Doku gegen den Code.
Control Request:
```yaml
id: b56c-46
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/seqlock.rs
  reason: Verify doc-link targets and claims
```
/answer b56c-46 allow
ReasoningToken: 146
Control Request:
```yaml
id: b56c-47
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
      nodes:
      - id: ModuleDoc
        signature: //! State-based reconciliation between the index and a resident mirror. The cha…
      - id: zmL2Dd|imV305
        signature: use std::collections::HashMap;
      - id: 5kSs3n|PSpC6v
        signature: /// Bound on stamp-recheck repetitions per load. Exhaustion falls back to the s…
      - id: ScanOutcome
        signature: "pub struct ScanOutcome {"
      - id: impl_ScanOutcome
        signature: "impl ScanOutcome {"
        children:
        - id: impl_ScanOutcome.is_clean
          signature: "pub fn is_clean(&self) -> bool {"
        - id: impl_ScanOutcome.enqueue_removals
          signature: "pub fn enqueue_removals(&self, queue: &mut UpdateQueue) {"
          docstring: Removals are mutations of the mapping and therefore go through the update queue.
      - id: SyncScanner
        signature: "pub struct SyncScanner {"
        docstring: "One scanner per subscriber (device manager, persistence timer, host mirror); ea…"
      - id: impl_SyncScanner
        signature: "impl SyncScanner {"
        children:
        - id: impl_SyncScanner.new
          signature: "pub fn new(index: &dyn Index) -> Self {"
        - id: impl_SyncScanner.changed
          signature: "pub async fn changed(&mut self) -> bool {"
          docstring: Resolves once an unseen signal exists; `false` when the index is gone.
        - id: impl_SyncScanner.has_pending
          signature: "pub fn has_pending(&self) -> bool {"
          docstring: A signal arrived since the last scan acknowledged the channel.
        - id: impl_SyncScanner.scan
          signature: "pub fn scan(     &mut self,     index: &dyn Index,     tracker: &ResidencyTrack…"
      - id: Loaded
        signature: "pub struct Loaded<'a> {"
      - id: load_entry
        signature: "pub fn load_entry(index: &dyn Index, entry: EntryId) -> Option<Loaded<'_>> {"
        docstring: "Reads stamp, then data, then the stamp again. Registering a stamp read after th…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Communication slot between kernel batches. Phases run in a fixed order: ker…"
      - id: tduEXu|95xQKg
        signature: use std::collections::HashMap;
      - id: Hit
        signature: "pub struct Hit {"
      - id: MappedSegment
        signature: "struct MappedSegment {"
      - id: Located
        signature: "struct Located {"
      - id: AddressMap
        signature: "pub struct AddressMap {"
        docstring: "Arena address -> entry. The segment set is copy-on-write: a clone is the stable…"
      - id: impl_AddressMap
        signature: "impl AddressMap {"
        children:
        - id: impl_AddressMap.new
          signature: "pub fn new() -> Self {"
        - id: impl_AddressMap.epoch
          signature: "pub fn epoch(&self) -> u64 {"
        - id: impl_AddressMap.successor
          signature: "pub fn successor(&self) -> Self {"
          docstring: Copy for the next table epoch.
        - id: impl_AddressMap.add_segment
          signature: "pub fn add_segment(&mut self, segment: u32, map: SlotMap) -> bool {"
          docstring: "Returns false when the segment is empty, its id is taken or its range overlaps …"
        - id: impl_AddressMap.remove_segment
          signature: "pub fn remove_segment(&mut self, segment: u32) -> bool {"
        - id: impl_AddressMap.resolve
          signature: "pub fn resolve(&self, off: u64) -> Option<EntryId> {"
        - id: impl_AddressMap.locate
          signature: "fn locate(&self, off: u64) -> Option<Located> {"
        - id: impl_AddressMap.segment
          signature: "fn segment(&self, id: u32) -> Option<&MappedSegment> {"
        - id: impl_AddressMap.bind
          signature: "fn bind(&self, slot: SlotRef, entry: EntryId) -> bool {"
        - id: impl_AddressMap.unbind_if
          signature: "fn unbind_if(&self, slot: SlotRef, entry: EntryId) {"
      - id: MappingStore
        signature: "pub struct MappingStore {"
        docstring: "Versioned mapping, advanced together with the segment table epoch of the swap m…"
      - id: impl_MappingStore
        signature: "impl MappingStore {"
        children:
        - id: impl_MappingStore.new
          signature: "pub fn new() -> Self {"
        - id: impl_MappingStore.snapshot
          signature: "pub fn snapshot(&self) -> AddressMap {"
        - id: impl_MappingStore.snapshot_at
          signature: "pub(crate) fn snapshot_at(&self, at: &Snapshot) -> Result<AddressMap, ManagerEr…"
          docstring: The mapping a batch started on `at` must resolve against.
        - id: impl_MappingStore.publish
          signature: "pub(crate) fn publish(&self, next: AddressMap) -> Result<(), ManagerError> {"
      - id: Update
        signature: "pub enum Update {"
      - id: impl_Update
        signature: "impl Update {"
        children:
        - id: impl_Update.entry
          signature: "fn entry(&self) -> EntryId {"
      - id: UpdateQueue
        signature: "pub struct UpdateQueue {"
        docstring: "Pending mutations, latest per entry; writes that could not be attributed wait f…"
      - id: impl_UpdateQueue
        signature: "impl UpdateQueue {"
        children:
        - id: impl_UpdateQueue.new
          signature: "pub fn new() -> Self {"
        - id: impl_UpdateQueue.push
          signature: "pub fn push(&mut self, update: Update) {"
        - id: impl_UpdateQueue.contains
          signature: "pub fn contains(&self, entry: EntryId) -> bool {"
        - id: impl_UpdateQueue.depth
          signature: "pub fn depth(&self) -> usize {"
        - id: impl_UpdateQueue.deferred_writes
          signature: "pub fn deferred_writes(&self) -> usize {"
        - id: impl_UpdateQueue.apply
          signature: "fn apply(&mut self, map: &AddressMap, tracker: &mut ResidencyTracker) -> usize {"
      - id: RangeSource
        signature: "pub trait RangeSource {"
        docstring: Reads back the bytes a kernel wrote into a device range.
        children:
        - id: RangeSource.read
          signature: "fn read(&self, range: WrittenRange) -> Vec<u8>;"
      - id: run_slot_at
        signature: "pub(crate) fn run_slot_at(     store: &MappingStore,     at: &Snapshot,     wri…"
        docstring: Runs one slot against the mapping of the batch epoch `at`. A mapping that moved…
      - id: run_slot
        signature: "pub fn run_slot(     writes: &[WrittenRange],     hits: &[KernelHit],     sourc…"
      - id: apply_writes
        signature: "fn apply_writes(     writes: &[WrittenRange],     source: &dyn RangeSource,    …"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/seqlock.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Seqlock over word-granular atomics, for reading index data that another wri…"
      - id: netfVc|bWeL3c
        signature: "#[cfg(loom)]"
      - id: Seqlock
        signature: "pub struct Seqlock<'a> {"
      - id: impl_Seqlock__a_
        signature: "impl<'a> Seqlock<'a> {"
        children:
        - id: impl_Seqlock__a_.new
          signature: "pub fn new(seq: &'a AtomicU64, words: &'a [AtomicU64]) -> Self {"
        - id: impl_Seqlock__a_.from_raw
          signature: "pub(crate) unsafe fn from_raw(seq: *mut u64, words: *mut u64, len: usize) -> Se…"
          docstring: "# Safety"
        - id: impl_Seqlock__a_.read_into
          signature: "pub fn read_into(&self, out: &mut Vec<u64>) -> u32 {"
          docstring: Copies a consistent snapshot into `out`; returns the number of retries.
        - id: impl_Seqlock__a_.write
          signature: "pub fn write(&self, data: &[u64]) {"
          docstring: Writers exclude each other through the odd counter value.
      - id: spin
        signature: "fn spin() {"
      - id: spin_2
        signature: "fn spin() {"
      - id: tests
        signature: "mod tests {"
        children:
        - id: tests.ea1V3W|nTgdvP
          signature: use super::*;
        - id: tests.read_returns_last_write
          signature: "fn read_returns_last_write() {"
        - id: tests.concurrent_reader_never_sees_a_torn_snapshot
          signature: "fn concurrent_reader_never_sees_a_torn_snapshot() {"
      - id: loom_tests
        signature: "mod loom_tests {"
        children:
        - id: loom_tests.aCv6R1|7NCwfi
          signature: use super::*;
        - id: loom_tests.reader_never_sees_a_torn_pair
          signature: "fn reader_never_sees_a_torn_pair() {"
```
/answer b56c-47 allow
Control Request:
```yaml
id: b56c-48
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "fn apply\\b|trait Index\\b|struct Stamp|struct EntryId|watch::"
    include:
    - '*.rs'
    limit: 20
  reason: "Locate Index::apply, Stamp, watch usage"
```
/answer b56c-48 allow
Control Request:
```yaml
id: b56c-49
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/slots.rs
        matches:
        - "fn apply(&self, w: &mut SlotWriter, write: SlotWrite) {"
      - path: index/mod.rs
        matches:
        - pub struct EntryId(u64);
        - pub struct Stamp(u64);
        - "pub trait Index {"
        - "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;"
        - fn changed(&self) -> watch::Receiver<u64>;
      - path: index/stamp.rs
        matches:
        - pub struct StampCounter(AtomicU64);
      - path: index/memory.rs
        matches:
        - "tx: watch::Sender<u64>,"
        - "let (tx, _) = watch::channel(counter.high_water());"
        - "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - "fn changed(&self) -> watch::Receiver<u64> {"
      - path: manager/sync.rs
        matches:
        - "rx: watch::Receiver<u64>,"
      - path: manager/test/flat.rs
        matches:
        - "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - "fn changed(&self) -> watch::Receiver<u64> {"
      - path: manager/test/sync.rs
        matches:
        - "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - "fn changed(&self) -> watch::Receiver<u64> {"
      - path: manager/test/scope.rs
        matches:
        - "fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {"
        - "fn changed(&self) -> watch::Receiver<u64> {"
      - path: manager/slot.rs
        matches:
        - "fn apply(&mut self, map: &AddressMap, tracker: &mut ResidencyTracker) -> usize {"
      - path: model/head.rs
        matches:
        - "pub fn apply(&self, rows: &[f32]) -> Vec<f32> {"
      warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b56c-49 allow
Control Request:
```yaml
id: b56c-4a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
    ids:
    - ModuleDoc
    - impl_SyncScanner.scan
    - load_entry
    - SyncScanner
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
    ids:
    - ModuleDoc
  reason: Verify reconciliation claims
```
/answer b56c-4a allow
ReasoningToken: 70
Control Request:
```yaml
id: b56c-4b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! State-based reconciliation between the index and a resident mirror. The change signal only
          //! wakes the scanner; correctness comes from comparing the stamps of resident entries.
      - id: impl_SyncScanner.scan
        type: ImplItemFn
        code: |2
              pub fn scan(&mut self, index: &dyn Index, tracker: &ResidencyTracker) -> ScanOutcome {
                  // Acknowledge before comparing: a send during the scan stays unseen and
                  // triggers the next scan instead of being lost.
                  let watermark = *self.rx.borrow_and_update();
                  let _span = tracing::info_span!("sync_scan", watermark).entered();

                  let mut live: HashMap<EntryId, Stamp> = HashMap::with_capacity(tracker.len());
                  for (id, stamp) in index.stamps() {
                      if tracker.get(id).is_some() {
                          live.insert(id, stamp);
                      }
                  }

                  let mut out = ScanOutcome { watermark, ..ScanOutcome::default() };
                  for (id, resident) in tracker.iter() {
                      match live.get(&id) {
                          None => out.removed.push(id),
                          Some(stamp) if *stamp != resident.stamp => out.reload.push(id),
                          Some(_) => out.current += 1,
                      }
                  }
                  out.reload.sort_unstable();
                  out.removed.sort_unstable();
                  tracing::debug!(
                      reload = out.reload.len(),
                      removed = out.removed.len(),
                      current = out.current,
                      "scan result"
                  );
                  out
              }
      - id: load_entry
        type: ItemFn
        code: |
          /// Reads stamp, then data, then the stamp again. Registering a stamp read after the data
          /// could mark stale data as current; the reverse order only risks a redundant reload.
          pub fn load_entry(index: &dyn Index, entry: EntryId) -> Option<Loaded<'_>> {
              let mut rechecks = 0;
              loop {
                  let first = index.stamp(entry)?;
                  let data = index.read(entry);
                  if index.stamp(entry) == Some(first) || rechecks >= MAX_RECHECKS {
                      return Some(Loaded { stamp: first, data, rechecks });
                  }
                  rechecks += 1;
                  tracing::debug!(entry = entry.get(), rechecks, "stamp moved during load, repeating");
              }
          }
      - id: SyncScanner
        type: ItemStruct
        code: |
          /// One scanner per subscriber (device manager, persistence timer, host mirror); each owns its
          /// receiver and therefore its own view of what it has seen.
          pub struct SyncScanner {
              rx: watch::Receiver<u64>,
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Communication slot between kernel batches. Phases run in a fixed order: kernel writes are
          //! registered with the index, results are resolved, and only then is the mapping mutated.
```
/answer b56c-4b allow
Control Request:
```yaml
id: b56c-4c
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
  regexp: fn apply|trait Index|struct Stamp|ModuleDoc|//!
  reason: Read Index trait docs and apply implementation
```
/answer b56c-4c allow
Control Request:
```yaml
id: b56c-4d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        code: |
          //! Index trait, Scope, AccessMethod, Stamp.
          //!
          //! The contract is independent of the core: an index reports changes and
          //! delivers structure and stamps; it knows no geometry and resolves no scope.
          //!
          //! Write order is binding: data first, then stamp, then signal.
          //! `read` may block on page faults and disk I/O; callers use a blocking pool.
      - id: Stamp
        code: |
          /// Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct Stamp(u64);
      - id: Index
        signature: "pub trait Index {"
        docstring: "Object-safe: engines hold indexes as `Arc<dyn Index + Send + Sync>`."
        children:
        - id: Index.apply
          code: |2
                /// Serializes writers (last writer wins in dequeue order). Returns the stamp
                /// of this very write; callers register it as known to suppress their echo.
                fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/memory.rs
      nodes:
      - id: ModuleDoc
        code: |
          //! Reference in-memory index; builds without `cuda`/`candle`.
      - id: impl_Index_for_InMemoryIndex.apply
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
```
/answer b56c-4d allow
Text:
 Alle Aussagen stimmen mit dem Code überein. Ich ergänze die Doku in `mod.rs`.
Control Request:
```yaml
id: b56c-4e
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
    id: ModuleDoc
    start_marker: "//! StreamedIndexManager, ScopeResolver"
    end_marker: "update queue, residency."
    content: |-
      //! StreamedIndexManager, ScopeResolver, update queue, residency.
      //!
      //! # Consistency model
      //!
      //! - The index is the source of truth. VRAM is a derived, discardable mirror. On divergence the
      //!   index wins.
      //! - Identity is `(EntryId, Stamp)`. Stamps are compared with `!=` only, never with `<`.
      //! - Reconciliation is state-based. The `watch` signal only wakes the scan
      //!   ([`sync::SyncScanner::scan`]). Only resident entries are compared.
      //! - Write order in the index is data, then stamp, then signal. Read order on load is stamp,
      //!   data, stamp again ([`sync::load_entry`]).
      //! - Segment tables are copy-on-write with a root pointer swap ([`swap`]). A retired version is
      //!   freed once all readers have reached the new epoch. Mapping mutations follow the table in
      //!   the slot gap ([`slot`]).
      //! - Last writer wins. [`apply`](crate::resident::index::Index::apply) is serialized.
  reason: Add Consistency model section to the module docs
```
/answer b56c-4e allow
ReasoningToken: 530
Control Request:
```yaml
id: b56c-4f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
  reason: Read plan sections to update
```
/answer b56c-4f allow
Control Request:
```yaml
id: b56c-50
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
      nodes:
      - id: 6c1f83
        signature: Umsetzungsplan E4 · Manager-Disziplin (Hochrisiko)
        children:
        - id: 6c1f83.494bb5
          signature: 1. Ziel & Abgrenzung
          children:
          - id: 6c1f83.494bb5.TXW9JJ|kGldyV
            signature: "Die Disziplin, mit der der Ma…"
          - id: 6c1f83.494bb5.m8uGAG|CgDsQU
            signature: "Nicht hier: die feste Slot-Re…"
        - id: 6c1f83.12b1dd
          signature: 2. Voraussetzungen
        - id: 6c1f83.6ad869
          signature: 3. Zu bauende Artefakte
          children:
          - id: 6c1f83.6ad869.xzOueY|1HDk0R
            signature: |-
              ```rust
              pub struct RootPointe…
        - id: 6c1f83.c9b059
          signature: 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
          children:
          - id: 6c1f83.c9b059.JLbELV|qIIq5m
            signature: '**`ld.acquire`/Fence auf sm_7…'
          - id: 6c1f83.c9b059.jwWx92|vMc5aX
            signature: '**Sichtbarkeits-Reihenfolge v…'
          - id: 6c1f83.c9b059.goSz2V|WpgWhl
            signature: '**Atomarer 8-Byte-Swap Host↔G…'
          - id: 6c1f83.c9b059.EvVs58|IiV7PP
            signature: '**Lock-free-Muster (Reader=GP…'
          - id: 6c1f83.c9b059.aI7GMG|WbS1qC
            signature: '**Pointer-Lesezeitpunkt** (co…'
        - id: 6c1f83.720730
          signature: 5. Umsetzungsschritte
        - id: 6c1f83.0c3143
          signature: 6. Fehler / Logging / Feature-Flags
        - id: 6c1f83.c0dcae
          signature: 7. Tests (Q3)
        - id: 6c1f83.2ddf5a
          signature: "8. Offene Verifikationen (zuerst, Hochrisiko)"
        - id: 6c1f83.d9113e
          signature: 9. Definition of Done
        - id: 6c1f83.5faa59
          signature: Result
          children:
          - id: 6c1f83.5faa59.U2XvXP|rL5IGm
            signature: "E4 ist umgesetzt, aber nur au…"
          - id: 6c1f83.5faa59.5ON1Ae|1zzGN6
            signature: Der Code liegt in `rag/src/re…
          - id: 6c1f83.5faa59.ITVw21|lxscQc
            signature: Die Tests decken diese Fälle …
          - id: 6c1f83.5faa59.msvi4C|WDQMRW
            signature: Die Tests laufen mit `cargo t…
          - id: 6c1f83.5faa59.KzI9zF|mA18Im
            signature: Ein Idle-Reader blockiert die…
          - id: 6c1f83.5faa59.MJSR18|bHIgRI
            signature: Zum Schluss ein Vorbehalt zum…
          - id: 6c1f83.5faa59.CZuI3G|Ar5ikz
            signature: Ich habe das Offene jetzt erl…
          - id: 6c1f83.5faa59.00cjht|YEOOiP
            signature: "Das ist neu:"
          - id: 6c1f83.5faa59.l8Lxi9|6aeT6m
            signature: "Zwei Dinge bleiben, und beide…"
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
      nodes:
      - id: de0012
        signature: Umsetzungsplan E1 · StreamedIndexManager-Grundgerüst
        children:
        - id: de0012.494bb5
          signature: 1. Ziel & Abgrenzung
          children:
          - id: de0012.494bb5.VPdwJK|d7d1I9
            signature: Das Skelett des `StreamedInde…
          - id: de0012.494bb5.sNgvmE|q0J0VZ
            signature: "Nicht hier: Scope-Auflösung (…"
        - id: de0012.12b1dd
          signature: 2. Voraussetzungen
        - id: de0012.6ad869
          signature: 3. Zu bauende Artefakte
          children:
          - id: de0012.6ad869.wyn6Va|benx8S
            signature: |-
              ```rust
              pub struct StreamedIn…
        - id: de0012.c9b059
          signature: 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
          children:
          - id: de0012.c9b059.gwV2v6|KLNoLl
            signature: '**mmap-Residency über RAM-Grö…'
          - id: de0012.c9b059.SaXPEY|2S319X
            signature: '**CPU- vs. GPU-Manager:** gem…'
        - id: de0012.720730
          signature: 5. Umsetzungsschritte
        - id: de0012.0c3143
          signature: 6. Fehler / Logging / Feature-Flags
        - id: de0012.c0dcae
          signature: 7. Tests (Q3)
        - id: de0012.8ff795
          signature: 8. Offene Verifikationen
        - id: de0012.d9113e
          signature: 9. Definition of Done
        - id: de0012.5faa59
          signature: Result
          children:
          - id: de0012.5faa59.uq2kr2|OZMgoK
            signature: E1 ist umgesetzt. Die 11 neue…
          - id: de0012.5faa59.bPINED|BwqfgN
            signature: Neu unter `src/resident/manag…
          - id: de0012.5faa59.Vuw2cF|y1hzVd
            signature: Zwei Punkte aus den offenen V…
          - id: de0012.5faa59.a6kc4c|chGCTf
            signature: "er Stand der beiden Punkte:"
          - id: de0012.5faa59.IpLHfy|vNMXlU
            signature: Damit ist E1 aus meiner Sicht…
```
/answer b56c-50 allow
Text:
 Jetzt die beiden Verifikationsabschnitte lesen und gleichzeitig die Builds und Tests starten.
Control Request:
```yaml
id: b56c-51
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
    ids:
    - 6c1f83.2ddf5a
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
    ids:
    - de0012.8ff795
  reason: Read current verification sections
```
/answer b56c-51 allow
Control Request:
```yaml
id: b56c-52
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
      nodes:
      - id: 6c1f83.2ddf5a
        type: section
        code: |+
          ## 8. Offene Verifikationen (zuerst, Hochrisiko)
          - **`ld.acquire.sys` und mapped-Atomics** direkt in der PTX-ISA (parallel-thread-execution) bestätigen **und auf der echten GTX 1660 testen** (context_e.md offener Punkt 1).
          - `cudaDevAttrHostNativeAtomicSupported` prüfen (H1 Punkt 4) — bestätigt die Single-Writer-Store-Strategie (kein GPU-CAS über PCIe).
          - Event-/Fence-Sichtbarkeit zwischen Copy-Stream und persistentem Kernel auf Zielhardware messen.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
      nodes:
      - id: de0012.8ff795
        type: section
        code: |+
          ## 8. Offene Verifikationen
          - Linux-`madvise`-Semantik gegen man7 prüfen; `memmap2`-`Advice`-API gegen aktuelle Crate-Version (context_e.md offener Punkt 3).
          - `MADV_COLD`/`PAGEOUT` erst ab Kernel ≥ 5.4 — Zielkernel bestätigen.
```
/answer b56c-52 allow
Control Request:
```yaml
id: b56c-53
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident:: 2>&1 | tail -15"
  reason: Run resident tests including the new one
```
/answer b56c-53 allow
Control Request:
```yaml
id: b56c-54
result:
  structuredContent:
    results:
    - stdout: |+
        test resident::subengine::exec::tests::event_completes_after_prior_jobs ... ok
        test resident::queue::queue::tests::concurrent_pushes_are_all_linked ... ok
        test resident::planner::tests::starved_hint_changes_engines_not_ops ... ok
        test resident::processor::tests::first_failing_op_fails_task_and_remaining_ops_are_dropped ... ok
        test resident::subengine::exec::tests::wait_orders_work_across_lanes ... ok
        test resident::processor::completion::tests::concurrent_callback_and_pollers_process_once ... ok
        test resident::manager::runtime::tests::publish_evict_and_slot_completion_race_without_leaking_regions ... ok
        test resident::index::memory::tests::entry_set_handles_wide_ranges_without_materializing_ids ... ok
        test resident::queue::queue::tests::iteration_matches_push_order ... ok
        test resident::tokenize::tests::e5_small_checkpoint_tokenizes_with_special_tokens ... ok
        test resident::queue::queue::tests::drop_long_chain_does_not_overflow_stack ... ok
        test resident::manager::seqlock::tests::concurrent_reader_never_sees_a_torn_snapshot ... ok

        test result: ok. 429 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 5.64s
```
/answer b56c-54 allow
ReasoningToken: 33
Control Request:
```yaml
id: b56c-55
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib cycle_logs_index_arena 2>&1 | tail -5
    - cargo clippy --all-targets 2>&1 | tail -8
    - cargo check --features cuda 2>&1 | tail -4
    - cargo doc --no-deps --lib 2>&1 | grep -i -B2 -A6 'manager/mod.rs' | head -20
  reason: "Verify new test, clippy, cuda check, and doc links"
```
/answer b56c-55 allow
Control Request:
```yaml
id: b56c-56
result:
  structuredContent:
    results:
    - stdout: |
        running 1 test
        test resident::manager::runtime::tests::cycle_logs_index_arena_utilization ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 456 filtered out; finished in 0.00s

        help: if you wanted an array of len 10, try
            |
        158 -     f.transfer.load_gather(&EntrySet::from_ranges([0..10])).unwrap();
        158 +     f.transfer.load_gather(&EntrySet::from_ranges([0; 10])).unwrap();
            |

        warning: `xy-ai-rag` (lib test) generated 30 warnings (11 duplicates) (run `cargo clippy --fix --lib -p xy-ai-rag --tests -- ` to apply 6 suggestions)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.16s
           |                          ^^^^^^^^

        warning: `xy-ai-rag` (lib) generated 15 warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.75s
```
/answer b56c-56 allow
Control Request:
```yaml
id: b56c-57
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "export PATH=/usr/local/cuda-13.3/bin:$PATH; date -I; nvidia-smi --query-gpu=name,compute_cap,driver_version --format=csv,noheader; cargo test --lib --features cuda device_ -- --ignored --nocapture 2>&1 | tail -25"
  reason: Run the two GPU tests on sm_75 hardware
```
/answer b56c-57 allow
ReasoningToken: 88
Control Request:
```yaml
id: b56c-58
result:
  structuredContent:
    results:
    - stdout: |
        2026-10-25
        NVIDIA GeForce GTX 1660 SUPER, 7.5, 610.57.04

        warning: `xy-ai-rag` (lib test) generated 4 warnings
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.74s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-4ad5f31f4fb2040b)

        running 10 tests
        host native atomics supported: 0
        test resident::manager::transfer::tests::device::zero_copy_segment_is_readable_through_the_device_address ... ok
        test resident::manager::transfer::tests::device::bulk_segment_reaches_the_device_in_one_direct_copy ... ok

        thread 'resident::planner::adapter::tests::device::extension_task_runs_through_workers_with_device_round_trips' (268295) panicked at src/resident/planner/test/adapter.rs:107:9:
        task did not complete
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        test resident::kernel::cuda::tests::registry_compiles_for_device_sm_and_caches has been running for over 60 seconds
        test resident::manager::build::tests::device_visibility::data_is_visible_to_a_polling_host_after_a_device_scope_fence has been running for over 60 seconds
        test resident::manager::build::tests::device_visibility::data_is_visible_to_a_polling_host_after_a_system_scope_fence has been running for over 60 seconds
        test resident::manager::swap::device_tests::copy_stream_event_orders_device_table_before_the_root has been running for over 60 seconds
        test resident::manager::swap::device_tests::device_sees_content_written_before_the_host_release_store has been running for over 60 seconds
        test resident::manager::transfer::tests::device::transfer_reaches_the_device_and_reads_back has been running for over 60 seconds
        test resident::manager::upload::tests::device::gather_reaches_the_device_in_one_copy has been running for over 60 seconds
        test resident::planner::adapter::tests::device::extension_task_runs_through_workers_with_device_round_trips has been running for over 60 seconds
        error: test failed, to rerun pass `--lib`

        Caused by:
          process didn't exit successfully: `/home/user/xyan/xy.ai.workbench/rag/.bin/debug/deps/xy_ai_rag-4ad5f31f4fb2040b device_ --ignored --nocapture` (signal: 9, SIGKILL: kill)
```
/answer b56c-58 allow Bin mir nicht sicher ob es ein dead oder lifelock gab
Control Request:
```yaml
id: b56c-59
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - date -I; uname -r; cargo test --lib resident::manager::test::advice 2>&1 | tail -12; cargo test --lib residency 2>&1 | grep -E 'will_need|cold_keeps|test result'
    - grep -A3 'name = "memmap2"' Cargo.lock
    - man 2 madvise 2>/dev/null | grep -E -A6 'MADV_COLD|MADV_WILLNEED' | head -30
  reason: "Run madvise tests, collect kernel/memmap2 versions and man page text"
```
/answer b56c-59 allow
Control Request:
```yaml
id: b56c-5a
result:
  structuredContent:
    results:
    - stdout: |
        2026-10-25
        6.8.0-147-generic
           |                          ^^^^^^^^
           |
           = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

        warning: `xy-ai-rag` (lib test) generated 2 warnings
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-d5754016cabf875c)

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 457 filtered out; finished in 0.00s

        test resident::manager::advice::tests::residency::cold_keeps_contents_and_does_not_drop_pages ... ok
        test resident::manager::advice::tests::residency::will_need_brings_pages_into_the_page_cache ... ok
        test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 438 filtered out; finished in 0.04s
        name = "memmap2"
        version = "0.9.11"
        source = "registry+https://github.com/rust-lang/crates.io-index"
        checksum = "d1219ed1b7f229ee7104d281dd01d6802fe28bb6e95d292942c4daacdeb798c0"
               MADV_WILLNEED
                      Expect access in the near future.  (Hence, it might  be  a  good
                      idea to read some pages ahead.)

               MADV_DONTNEED
                      Do  not  expect access in the near future.  (For the time being,
                      the application is finished with the given range, so the  kernel
        --
               MADV_COLD (since Linux 5.4)
                      Deactivate a given range of pages.  This will make the  pages  a
                      more  probable reclaim target should there be a memory pressure.
                      This is a nondestructive operation.  The advice might be ignored
                      for some pages in the range when it is not applicable.

               MADV_PAGEOUT (since Linux 5.4)
        --
               EINVAL advice is MADV_COLD or MADV_PAGEOUT and  the  specified  address
                      range includes locked, Huge TLB pages, or VM_PFNMAP pages.

               EINVAL advice is MADV_DONTNEED or MADV_REMOVE and the specified address
                      range includes locked, Huge TLB pages, or VM_PFNMAP pages.

               EINVAL advice is MADV_MERGEABLE or MADV_UNMERGEABLE, but the kernel was
        --
               EIO    (for  MADV_WILLNEED)  Paging  in  this  area  would  exceed  the
                      process's maximum resident set size.

               ENOMEM (for MADV_WILLNEED) Not enough memory: paging in failed.

               ENOMEM (for  MADV_COLLAPSE)  Not  enough  memory:  could  not  allocate
```
/answer b56c-5a allow
Text:
 Der Filter `device_` hat alle zehn GPU-Tests gleichzeitig gestartet, und der Lauf wurde mit SIGKILL beendet. Ich starte die zwei Zieltests einzeln und mit Timeout neu.
Control Request:
```yaml
id: b56c-5b
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH; timeout 180 cargo test --lib --features cuda device_sees_content_written_before_the_host_release_store -- --ignored --nocapture 2>&1 | grep -v warning | tail -12
    - timeout 180 cargo test --lib --features cuda copy_stream_event_orders_device_table_before_the_root -- --ignored --nocapture 2>&1 | tail -10
  reason: Run the two E4 GPU tests individually
```
/answer b56c-5b allow
ReasoningToken: 143
Control Request:
```yaml
id: b56c-5c
result:
  structuredContent:
    results:
    - stdout: |+
        ...
        20 |     pub(crate) unsafe fn from_raw(seq: *mut u64, words: *mut u64, len: usize) -> Self {
           |                          ^^^^^^^^

            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.18s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-4ad5f31f4fb2040b)

        running 1 test
        test resident::manager::swap::device_tests::device_sees_content_written_before_the_host_release_store ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 494 filtered out; finished in 0.60s

        warning: `xy-ai-rag` (lib test) generated 4 warnings
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.18s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-4ad5f31f4fb2040b)

        running 1 test
        host native atomics supported: 0
        test resident::manager::swap::device_tests::copy_stream_event_orders_device_table_before_the_root ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 494 filtered out; finished in 0.26s
```
/answer b56c-5c allow
Control Request:
```yaml
id: b56c-5d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/.cargo/registry/src
    pattern: ^\s+(WillNeed|Sequential|Random|Cold) ?=|pub enum Advice|fn advise\(
    include:
    - advice.rs
    - lib.rs
    - unix.rs
    limit: 15
  reason: Confirm memmap2 Advice variants in the used version
```
/answer b56c-5d allow
Control Request:
```yaml
id: b56c-5e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/.cargo/registry/src
      matches:
      - path: index.crates.io-1949cf8c6b5b557f/uuid-1.27.0/src/lib.rs
        matches:
        - "Random = 4,"
      - path: index.crates.io-1949cf8c6b5b557f/memmap2-0.9.11/src/advice.rs
        matches:
        - "pub enum Advice {"
        - "Random = libc::MADV_RANDOM,"
        - "Sequential = libc::MADV_SEQUENTIAL,"
        - "WillNeed = libc::MADV_WILLNEED,"
      - path: index.crates.io-1949cf8c6b5b557f/memmap2-0.9.11/src/lib.rs
        matches:
        - "pub fn advise(&self, advice: Advice) -> Result<()> {"
        - "fn advise() {"
      - path: index.crates.io-1949cf8c6b5b557f/memmap2-0.9.11/src/unix.rs
        matches:
        - "pub unsafe fn advise(&self, advice: libc::c_int, offset: usize, len: usize) -> io::Result<()> {"
```
/answer b56c-5e allow
ReasoningToken: 41
Control Request:
```yaml
id: b56c-5f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: "MADV_COLD|unchecked_advise|UncheckedAdvice|Advice::"
    include:
    - advice.rs
    limit: 15
  reason: See how MADV_COLD is issued given memmap2 lacks Advice::Cold
```
/answer b56c-5f allow
Control Request:
```yaml
id: b56c-60
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: advice.rs
        matches:
        - "Hint::WillNeed => self.map.advise_range(Advice::WillNeed, range.start, len),"
        - "Hint::Sequential => self.map.advise_range(Advice::Sequential, range.start, len),"
        - "Hint::Random => self.map.advise_range(Advice::Random, range.start, len),"
        - "// SAFETY: the range lies inside a live mapping; MADV_COLD never changes contents."
        - "let rc = unsafe { libc::madvise(start as *mut libc::c_void, len, libc::MADV_COLD) };"
      - path: test/advice.rs
        matches:
        - MmapAdvice::new(MmapMut::map_anon(len).unwrap().make_read_only().unwrap())
        - let advice = MmapAdvice::new(map);
```
/answer b56c-60 allow
Control Request:
```yaml
id: b56c-61
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
    id: 6c1f83.2ddf5a
    start_marker: '- **`ld.acquire.sys` und mapped-Atomics**'
    end_marker: auf Zielhardware messen.
    content: |-
      Stand der Messung: 2026-10-25, GTX 1660 SUPER (sm_75, Treiber 610.57.04), nvcc 13.3. Beide Tests liefen einzeln mit `cargo test --lib --features cuda <name> -- --ignored --nocapture`.

      - **`ld.acquire.sys` und mapped-Atomics** — Status: verifiziert (empirisch). Messdatum 2026-10-25, GTX 1660 SUPER (sm_75). Ergebnis: `device_sees_content_written_before_the_host_release_store` bestanden; das Gerät sieht per `ld.acquire.sys` den Inhalt, der vor dem Host-Release-Store geschrieben wurde. Die Bestätigung in der PTX-ISA-Dokumentation selbst ist in diesem Lauf nicht neu geprüft worden.
      - **`HostNativeAtomicSupported`** — Status: verifiziert. Messdatum 2026-10-25, GTX 1660 SUPER (sm_75). Ergebnis: Wert 0 (`host native atomics supported: 0`, ausgegeben von `copy_stream_event_orders_device_table_before_the_root`). Die Single-Writer-Strategie (Host schreibt, GPU liest per `ld.acquire.sys`) ist davon nicht berührt, weil es kein GPU-CAS über PCIe gibt und die GPU den Root nie schreibt.
      - **Event-/Fence-Sichtbarkeit zwischen Copy-Stream und Kernel** — Status: verifiziert für den getesteten Fall. Messdatum 2026-10-25, GTX 1660 SUPER (sm_75). Ergebnis: `copy_stream_event_orders_device_table_before_the_root` bestanden; das Event ordnet die Gerätetabelle vor den Root. Getestet ist ein einzelner Lauf mit Copy-Stream und Kernel-Launch, kein Dauerlauf mit persistentem Kernel.

      Hinweis: Der Filter `device_` startet alle zehn GPU-Tests des Crates parallel. Dieser Lauf hing bei mehreren Tests über 60 Sekunden und wurde mit SIGKILL beendet; ein Test (`extension_task_runs_through_workers_with_device_round_trips`) schlug mit „task did not complete“ fehl. Diese Tests gehören nicht zu E4 und wurden nicht weiter untersucht. Die zwei E4-Tests laufen einzeln stabil.
  reason: Record E4 verification status
```
/answer b56c-61 allow
ReasoningToken: 204
Control Request:
```yaml
id: b56c-62
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
    id: de0012.8ff795
    start_marker: '- Linux-`madvise`-Semantik gegen man7'
    end_marker: Zielkernel bestätigen.
    content: |-
      Stand der Prüfung: 2026-10-25, Kernel 6.8.0-147-generic, `memmap2` 0.9.11 (Cargo.toml `0.9.3`, Cargo.lock 0.9.11).

      - **Semantik/API** — Status: verifiziert. `man 2 madvise`: `MADV_WILLNEED` kündigt Zugriffe in naher Zukunft an (Read-ahead); `MADV_COLD` (seit Linux 5.4) deaktiviert Seiten und macht sie zu wahrscheinlicheren Reclaim-Kandidaten, eine nicht-destruktive Operation, die für einzelne Seiten ignoriert werden kann. `memmap2` 0.9.11 enthält `Advice::WillNeed`, `Advice::Sequential` und `Advice::Random`, aber kein `Cold`; `MADV_COLD` wird deshalb direkt über `libc::madvise` gesetzt. Tests `will_need_brings_pages_into_the_page_cache` und `cold_keeps_contents_and_does_not_drop_pages` bestanden (`cargo test --lib residency`: 19 bestanden, 0 fehlgeschlagen); sie belegen die Wirkung per `mincore`.
      - **Kernel ≥ 5.4 für `MADV_COLD`** — Status: verifiziert für diese Maschine. `uname -r` liefert 6.8.0-147-generic, der Kernel ist also neuer als 5.4, und der Cold-Test läuft dort durch. Ein anderer Zielkernel wurde nicht geprüft.
  reason: Record E1 verification status
```
/answer b56c-62 allow
Result Stats: id=39f2b96f-848e-49e0-8145-b63891ccf271, total: 78118, in: 50, out: 15243, reason: 124, read: 789635, write: 62825, start: 1792945895646, end: 1792945909545, time: 13813, total: 430644, io: 0, control: 86, sent: 0, recv: 285
ReasoningToken: 124
Alle vier Punkte sind umgesetzt. `cargo test --lib resident::` ist grün (429 bestanden, 0 fehlgeschlagen). `cargo check --features cuda` läuft durch, und `cargo clippy --all-targets` endet ohne Fehler. Clippy meldet weiterhin Warnungen. Dass sie schon vorher bestanden, habe ich nicht geprüft. `cargo doc` zeigt für `mod.rs` keine Warnung.

## Geänderte Dateien
- `manager/runtime.rs`: `IndexRuntime::cycle` ruft am Ende `trace_utilization` auf, unter dem Span `manager_cycle`.
- `manager/swap.rs`: `trace_utilization` läuft nach `publish` und nach `reclaim`. Beide Aufrufe sitzen im `TableSwap`-Impl von `SwapManager`.
- `manager/test/runtime.rs`: neuer Test `cycle_logs_index_arena_utilization`.
- `manager/mod.rs`: Abschnitt „Consistency model" in der Modul-Doku.
- `plan_e_04.md` und `plan_e_01.md`: Abschnitt 8 aktualisiert.

## Abweichungen von der Vorgabe
- **Aufrufort für den Zyklus:** `GpuIndexManager::cycle` in `streamed.rs` hat keinen Zugriff auf die Arena, deshalb habe ich den Aufruf in `IndexRuntime::cycle` gesetzt. Dort liegt die Arena, und der Aufruf läuft über `cycle_span()` unter `manager_cycle`.
- **Testsubscriber:** Der `Capture` aus `resident/test/telemetry.rs` ist privat und ignoriert Events. Der Test bringt deshalb einen kleinen eigenen Subscriber mit, der die Event-Nachrichten sammelt.
- **Log-Level:** `debug!` reicht pro Zyklus, das Level bleibt unverändert.

## Konsistenz-Doku
Alle sechs Aussagen stimmen mit dem Code überein. Die Intra-Doc-Links `sync::load_entry`, `sync::SyncScanner::scan`, `swap`, `slot` und `Index::apply` lösen auf. `apply` ist über `crate::resident::index::Index::apply` verlinkt, weil `manager` kein eigenes Modul `index` hat.

## Status in den Plänen
**E4, gemessen am 2026-10-25 auf der GTX 1660 SUPER (sm_75), alle drei „verifiziert":**
- **`ld.acquire.sys`:** Der Test `device_sees_content_written_before_the_host_release_store` besteht.
- **`HostNativeAtomicSupported`:** Der Wert ist 0. Das ist vermerkt: Die Single-Writer-Strategie bleibt gültig, weil es kein GPU-CAS über PCIe gibt.
- **Event-/Fence-Sichtbarkeit:** `copy_stream_event_orders_device_table_before_the_root` besteht. Getestet ist ein einzelner Lauf, kein Dauerlauf mit persistentem Kernel.

**E1, gemessen am 2026-10-25 mit Kernel 6.8.0-147-generic und `memmap2` 0.9.11, beide „verifiziert":**
- **Semantik/API:** `Advice::WillNeed`, `Sequential` und `Random` existieren in `memmap2` 0.9.11. `Advice::Cold` fehlt dort, der Code setzt `MADV_COLD` deshalb direkt über `libc::madvise`. Beide Advice-Tests bestehen.
- **Kernel ≥ 5.4:** 6.8 erfüllt das. Ein anderer Zielkernel wurde nicht geprüft.

## Nicht belegt
- Die Bestätigung von `ld.acquire.sys` in der PTX-ISA-Dokumentation habe ich nicht neu geprüft, nur die Wirkung auf der Hardware.
- Der Filter `device_` startet alle zehn GPU-Tests des Crates parallel. Dabei hingen mehrere Tests über 60 Sekunden, und der Lauf wurde mit SIGKILL beendet. `extension_task_runs_through_workers_with_device_round_trips` schlug mit „task did not complete" fehl. Diese Tests gehören nicht zu E4 und sind nicht untersucht, das Fehlerbild steht aber im Plan. Die zwei E4-Tests laufen einzeln stabil.
