# Kontextdatei — Gruppe A (Fundament & Kontext)

Verifizierungsdatum: 2026-10-10. Zielhardware: GTX 1660 (sm_75, 6 GB). Modell: `intfloat/multilingual-e5-small`.

Legende: **[V]** = am 2026-10-10 live verifiziert (Quelle genannt). **[W]** = aus Fachwissen, nicht live verifiziert (Unsicherheit). **[?]** = offen.

---

## Versionsübersicht (crates.io, Stand 2026-10-10)

| Crate | Stabil | Datum (updated_at) | Quelle |
|---|---|---|---|
| candle-core / candle-transformers | 0.11.0 | 2026-06-26 | crates.io API [V] |
| cudarc | 0.19.9 (max_stable) | 2026-08-11 | crates.io API [V] |
| tokenizers | 0.23.2 (stabil); 1.0.0-rc.2 ist Pre-Release | 2026-09-03 / 2026-09-21 | crates.io API [V] |
| safetensors | 0.8.0 | 2026-06-09 | crates.io API [V] |
| parquet (arrow-rs) | 60.0.0 | 2026-09-15 | crates.io Suche [V] |
| tokio | 1.53.2 | 2026-10-03 | crates.io Suche [V] |
| crossbeam | 0.8.5 | 2026-09-05 | crates.io Suche [V] |
| crossbeam-queue | 0.3.14 | 2026-09-05 | crates.io Suche [V] |
| parking_lot | 0.12.5 | 2025-10-03 | crates.io Suche [V] |
| arc-swap | 1.9.2 | 2026-06-28 | crates.io Suche [V] |
| thiserror | 2.0.20 | 2026-08-08 | crates.io Suche [V] |
| tracing | 0.1.44 | 2025-12-18 | crates.io Suche [V] |
| tracing-subscriber | 0.3.23 | 2026-03-13 | crates.io Suche [V] |
| half | 2.7.1 | 2025-10-14 | crates.io Suche [V] |
| mockall | 0.15.0 | 2026-06-28 | crates.io Suche [V] |

Hinweis: lib.rs-Seiten lieferten veraltete Cache-Stände (z. B. tokio 1.7.1, candle-transformers 0.8.4) und wurden verworfen.

---

## Paket A2 — Projekt- & Modulstruktur / Feature-Flags

### 1. Workspace/Crate mit `cuda`-Feature, Kernlogik ohne GPU
**Antwort [W]:**
- Kernlogik als GPU-freie Crate/Modulbaum; GPU-Backend hinter Trait (`trait Embedder`/`trait Kernel`).
- `candle-core` als optionale Abhängigkeit: `[dependencies] candle-core = { version = "0.11", optional = true }`, `[features] cuda = ["dep:candle-core", "candle-core/cuda"]`. Mit `dep:`-Syntax entsteht kein impliziter Feature-Name.
- Real-Backend: `#[cfg(feature = "cuda")] mod cuda_backend;`. Ohne Feature: `CpuBackend`/`MockBackend` als Default.
- Workspace-Variante: Crate `core` (rein, Traits, Logik), Crate `backend-cuda` (hängt von `core` + candle ab), Binary-Crate verdrahtet. CI baut `--no-default-features` ohne CUDA-Toolkit.
- Für Tests ohne GPU: `#[cfg(any(test, feature = "mock"))]`.
Unsicherheit: allgemeine Idiome, nicht gegen aktuelle Cargo-Docs geprüft.

### 2. Funktionsweise `cuda`-Feature in candle-core 0.11.0
**Antwort [V]:** Feature-Definition aus crates.io (0.11.0): `cuda = ["cudarc", "dep:candle-kernels", "candle-ug?/cuda"]`; `cudnn = ["cuda", "cudarc/cudnn"]`, `nccl = ["cuda", "cudarc/nccl"]`, `default = []`. `cudarc` ist optionale Abhängigkeit (`^0.19.8`), `candle-kernels ^0.11.0` ebenfalls optional.
- `candle-kernels` kompiliert beim Build CUDA-Kernel per `build.rs` (benötigt nvcc) — nur mit aktivem `cuda`.
- Ohne `cuda`-Feature: weder cudarc noch candle-kernels noch nvcc nötig → No-GPU-Build möglich.
- Isolation: `cuda` in eigener Crate/eigenem Feature kapseln, nie in `default`.
Quelle: https://crates.io/api/v1/crates/candle-core (Version 0.11.0), Candle-Repo `candle-core/Cargo.toml` (main).

### 3. Schichtmuster (Fassade / Orchestrator / pure Funktionen / Adapter)
**Antwort [W]:**
- Pure Funktionsschicht ohne I/O und ohne async → separate Crate oder Modul, vollständig unit-testbar.
- Ressourcen-Adapter (GPU, Dateien, Parquet) hinter Traits; Implementierungen per Feature.
- Orchestrator (tokio, Worker-Pool) kennt nur Traits.
- Fassade: eine `lib.rs`-API; interne Module `pub(crate)`, gezielte `pub use`-Re-Exports.
- Faustregel: Crate-Grenze, wenn eigene Feature-/Dependency-Menge (CUDA) oder Wiederverwendung; sonst Modul.
Unsicherheit: Best-Practice-Wissen, keine Primärquelle abgerufen.

---

## Paket A3 — Stack- & Versionskompatibilität

### 4. `cudarc`-Version von candle + Versionsgleichheit
**Antwort [V]:**
- candle-core 0.11.0 verlangt `cudarc ^0.19.8` (optional). Features laut crates.io: `std, cublas, cublaslt, curand, driver, nvrtc, f16, f8, cuda-version-from-build-system, dynamic-linking`.
- Neueste stabile cudarc auf crates.io: 0.19.9 (2026-08-11). candle-**main** nennt im Workspace `cudarc 0.19.10` — **[?]** ob 0.19.10 bereits veröffentlicht ist; crates.io-Abfrage zeigte max 0.19.9.
- Aufgelöste Version lokal: `cargo tree -i cudarc` / `Cargo.lock`.
**Empfehlung [W]:** Direkte cudarc-Nutzung vermeiden; wenn nötig `candle_core::cuda_backend::cudarc` re-exportiert nutzen (falls vorhanden — **[?]** nicht geprüft) oder `cudarc = "=0.19.8"`-Pinning plus identische Features; `cargo tree -d` prüft Duplikate. Zwei cudarc-Versionen im Baum → inkompatible Typen (`CudaDevice` etc.).
Quellen: crates.io API candle-core/0.11.0/dependencies; crates.io API cudarc; raw.githubusercontent.com/huggingface/candle/main/Cargo.toml.

### 5. CUDA-Toolkit/Treiber und GTX 1660 (sm_75)
**Antwort (teilweise [V]):**
- cudarc 0.19.x bietet Feature-Flags für `cuda-11040` bis `cuda-12090` und `cuda-13000`…`cuda-13030` [V]; candle nutzt `cuda-version-from-build-system` (Version vom installierten Toolkit) + `dynamic-linking` [V].
- candle-kernels `build.rs` (main) [V]: kein Mindest-Compute-Cap erzwungen; Cap per `cudaforge::detect_compute_cap()`, Fallback 80. Unter 80 wird `-DNO_BF16_KERNEL` gesetzt → **bf16-Kernel entfallen auf sm_75, f16/f32 bleiben**.
- Turing (sm_75) unterstützt f16 nativ, `dp4a` und INT8-Tensor-Cores; `ld.acquire` ab sm_70 [W — NVIDIA-Doku nicht neu abgerufen].
- **Risiko [W/?]:** CUDA 13 hat Unterstützung für Maxwell/Pascal/Volta entfernt; Turing bleibt laut Berichten unterstützt (Treiber der 580er-Reihe beendet Maxwell/Pascal/Volta, nicht Turing). Sicherste Wahl: **CUDA 12.x** (z. B. 12.8/12.9). Bei Build `CUDA_COMPUTE_CAP=75` explizit setzen.
Quellen: crates.io cudarc; candle `candle-kernels/build.rs`; Suchergebnis Guru3D/PyTorch-Forum (Maxwell/Pascal/Volta nach R580 nicht mehr unterstützt, Details nicht gelesen).

### 6. `tokenizers`
**Antwort [V]:** Stabil 0.23.2 (2026-09-03); 1.0.0-rc.2 (2026-09-21) nur Release Candidate (Edition 2024) — für Produktion bei 0.23.2 bleiben. candle selbst nutzt `tokenizers 0.23.1` (`default-features = false`).
API (docs.rs 0.23.2):
- `Tokenizer::from_file<P: AsRef<Path>>(file) -> Result<Self>` (lädt `tokenizer.json`)
- `encode(&self, input, add_special_tokens: bool) -> Result<Encoding>`
- `encode_batch(&self, inputs: Vec<E>, add_special_tokens: bool) -> Result<Vec<Encoding>>`
- `with_padding(&mut self, Option<PaddingParams>)`, `with_truncation(&mut self, Option<TruncationParams>) -> Result<&mut Self>`
- `Encoding::get_ids(&self) -> &[u32]`, `get_attention_mask(&self) -> &[u32]`, `get_type_ids(&self) -> &[u32]`
Hinweis: Mit `default-features = false` entfallen `onig` und `progressbar`; XLM-R-`tokenizer.json` (Unigram/Metaspace) sollte ohne onig funktionieren **[?]** — per Test verifizieren.
Quellen: https://docs.rs/tokenizers/0.23.2/…/Tokenizer.html, …/Encoding.html; crates.io tokenizers.

### 7. `parquet`/arrow: mmap, `FixedSizeList<f16,384>`, f16
**Antwort (teilweise [V]):**
- parquet 60.0.0 (2026-09-15) [V].
- mmap: kein eingebauter mmap-Reader gefunden **[?]**. Mechanismus [W]: `ChunkReader` ist für `bytes::Bytes` implementiert; gemappte Datei (z. B. `memmap2`) in `Bytes` überführen (Kopie oder `Bytes::from_owner`/Wrapper) und `ParquetRecordBatchReaderBuilder::try_new(bytes)` nutzen. Echtes Zero-Copy hängt von Kompression/Encoding ab (komprimierte Pages werden dekodiert/kopiert).
- f16: Arrow hat `Float16Type`/`Float16Array` (basiert auf `half::f16`) [W]; Parquet-Logical-Type Float16 war Issue apache/arrow-rs#4986 (Umsetzungsversion **[?]** nicht ermittelt; Writer-Funktion `get_float_16_array_slice` existiert laut docs-Suchtreffer [V-Treffer]). → Vollständigen f16-Support in 60.0.0 vor Einsatz per Roundtrip-Test prüfen.
- `FixedSizeList<Float16, 384>`: `DataType::FixedSizeList(Arc<Field::new("item", Float16, false)>, 384)`; Parquet speichert als `LIST`/repeated; Lesen gibt `FixedSizeListArray` zurück [W].
Quellen: crates.io parquet; GitHub arrow-rs #4986 (nur als Suchtreffer); arrow.apache.org/rust ChunkReader/arrow_writer Doku (Suchtreffer, nicht gelesen).

### 8. tokio
**Antwort [V]:** 1.53.2 (2026-10-03). Relevante Features [W]: `rt-multi-thread`, `sync` (watch, Notify, OnceCell), `macros`, `time`. `spawn_blocking` nutzt Blocking-Pool (Standard 512 Threads, via `Builder::max_blocking_threads`). Für GPU-Aufrufe dedizierten Thread/`spawn_blocking` statt async-Task nutzen. `tokio::sync::OnceCell` im Feature `sync`.
Quelle: crates.io Suche tokio.

### 9. crossbeam vs. parking_lot
**Antwort (Versionen [V], Bewertung [W]):** crossbeam 0.8.5, crossbeam-queue 0.3.14, parking_lot 0.12.5.
- `crossbeam-queue`: `ArrayQueue` (lock-free, beschränkter MPMC-Ring-Buffer) und `SegQueue` (unbounded) — direkt als Ring-Buffer geeignet.
- `parking_lot`: schnellere Mutex/RwLock/Condvar, **keine** lock-free Strukturen; passend für kurze kritische Abschnitte, nicht für CAS-Listen.
- Einschränkung ArrayQueue: kein beliebiges Unlink/Durchlaufen; für verkettete Liste mit Unlink → Epoch-Reclamation (`crossbeam-epoch`) oder arc-swap.

### 10. arc-swap
**Antwort [V]:** 1.9.2 (2026-06-28). `ArcSwapOption<T>` = `ArcSwapAny<Option<Arc<T>>>`.
- `load() -> Guard`, `store(val)`, `swap(new) -> old`, `compare_and_swap(current, new) -> Guard (vorheriger Wert)`, `rcu(f)` (Closure kann mehrfach laufen).
- `compare_and_swap`-`current` darf `&Arc`, `Guard` o. Ä. sein, **kein** owned `Arc`; Erfolg = Rückgabewert pointer-gleich zu `current`.
- [W] Append: neuen Knoten per CAS an `next` des Tails hängen; Unlink: `next` des Vorgängers per CAS auf `node.next` setzen. Speicherfreigabe: Arc-Refcount; Knoten werden freigegeben, sobald letzte Referenz/Guard fällt — keine Epochen nötig, aber Kosten pro Load und ABA ist durch Arc-Identität entschärft. Lange Ketten: rekursives Drop kann Stack überlaufen → iteratives Drop implementieren.
Quelle: docs.rs arc-swap 1.9.2 ArcSwapAny.

### 11. safetensors
**Antwort [V]:** 0.8.0 (2026-06-09); candle-core 0.11.0 hängt hart davon ab (`^0.8.0`, nicht optional) und nutzt `memmap2 ^0.9.3`. [W] mmap-Laden über `VarBuilder::from_mmaped_safetensors(&[path], dtype, &device)` (unsafe); candle liest Tensoren aus dem Mapping, Kopie erfolgt beim Transfer aufs GPU-Device. Zero-Copy gilt nur auf CPU.

### 12. `candle-transformers::models::bert::BertModel`
**Antwort [V]** (Quelle: candle `bert.rs`, main):
- Ja: `BertModel::load(vb: VarBuilder, config: &Config)`. Prefix-Logik: ist Tensor `bert` vorhanden → `vb.pp("bert")`; sonst Root; bei Fehler Retry mit `"{model_type}.embeddings"`/`.encoder`.
- Tensornamen: `embeddings.{word_embeddings, position_embeddings, token_type_embeddings, LayerNorm}`; `encoder.layer.{i}.attention.self.{query,key,value}`, `…attention.output.{dense,LayerNorm}`, `…intermediate.dense`, `…output.{dense,LayerNorm}`.
- Config-Felder inkl. `hidden_act` (`gelu`/`gelu_approximate`/`relu`), `model_type: Option<String>`, `position_embedding_type`.
- multilingual-e5-small `config.json` [V]: `model_type: "bert"`, `hidden_size 384`, `12 Layer`, `12 Heads`, `intermediate_size 1536`, `hidden_act gelu`, `layer_norm_eps 1e-12`, `vocab_size 250037`, `max_position_embeddings 512`, `type_vocab_size 2`, `tokenizer_class XLMRobertaTokenizer` → passt zu candle-`BertModel`.
- Abweichung `xlm_roberta` [W]: Präfix `roberta.` statt `bert.`, Positions-IDs ab `padding_idx+1`, kein/anderes token_type-Handling — für dieses Modell nicht relevant, da BERT-Architektur.
- Pooling: Mean-Pooling mit Attention-Maske + L2-Normalisierung selbst implementieren (Sentence-Transformers-Config `1_Pooling`). E5 erwartet Präfixe `query: ` / `passage: ` [W].

### 13. HF-Dateien `intfloat/multilingual-e5-small`
**Antwort [V]** (huggingface.co Tree, 2026-10-10): `model.safetensors` (471 MB) **vorhanden**, `pytorch_model.bin` (471 MB), `tokenizer.json` (17,1 MB), `sentencepiece.bpe.model`, `config.json` (655 B), `tokenizer_config.json`, `special_tokens_map.json`, `modules.json`, `sentence_bert_config.json`, Ordner `1_Pooling`, `onnx`, `openvino`. Keine Konvertierung nötig. Hinweis: 471 MB bei f32; f16-Konvertierung halbiert VRAM-Bedarf (passt in 6 GB).

### 14. thiserror / tracing
**Antwort [V]:** thiserror 2.0.20 (2.x seit 2024-11), tracing 0.1.44, tracing-subscriber 0.3.23. Migration 1→2 [W]: u. a. Format-Syntax-Änderungen (`{r#type}`-Raw-Identifier, Verbot von Trait-Bounds-Ambiguitäten bei `#[error("{0}")]` mit direct-Display-Feldern); Details im Changelog **[?]** nicht abgerufen. tracing hat keine neue Major-Version; 0.2 nicht veröffentlicht.

### 15. Bekannte Konflikte/Risiken
**Antwort (gemischt):**
- [V] Doppelte cudarc-Versionen vermeiden (candle `^0.19.8`; main bereits 0.19.10).
- [V] sm_75: bf16-Kernel in candle werden unter Cap 80 ausgeschaltet → nur f16/f32 planen.
- [W] CUDA 13 vs. Turing: auf CUDA 12.x bleiben; Treiber ≥ passende Mindestversion des Toolkits.
- [V] tokenizers: 1.0.0-rc.2 (Edition 2024) nicht mit candle-Stack mischen; candle nutzt 0.23.1 → Cargo löst 0.23.x; bei Version 1.0 Duplikate möglich.
- [V/W] arrow/parquet 60.x: schnelle Major-Versionen (alle ~1–3 Monate); `arrow`-Crates exakt gleiche Major wie `parquet` wählen; f16-Parquet-Support per Test absichern.
- [V] `hf-hub 1.0.0` in candle-Workspace — falls hf-hub direkt genutzt wird, gleiche Major wählen.

---

## Paket A4 — Querschnittskonventionen (alle [W], keine Primärquellen abgerufen)

### 16. Fehler-Taxonomie mit thiserror
- Ein Fehler-Enum pro Schicht (`AdapterError`, `OrchestratorError`, …), obere Schichten wrappen untere mit `#[from]`/`#[source]`; `#[non_exhaustive]` für öffentliche Enums.
- Fehler müssen `Send + Sync + 'static` sein, um über `async`-Grenzen/`JoinHandle`/Channels zu reisen. Kontext (Task-ID) als Felder statt String.
- Worker-Pool: Ergebnis als `Result<T, E>` per Channel zurück, nie `panic`; Panics mit `JoinError` abfangen. `anyhow` nur in `main`/Binary.
- Kritische GPU-Fehler (OOM) von wiederholbaren Fehlern per Variante trennen (`is_retryable()`).

### 17. tracing-Spans
- Ein Span pro Task (Feld `task_id`), Child-Spans pro Verarbeitungszyklus/Batch; Ressource (Device) als Feld, nicht als Span.
- Async: `#[tracing::instrument(skip_all, fields(task_id = %id))]` bzw. `.instrument(span)` auf Futures; **kein** `span.enter()` über `.await`.
- Threads/`spawn_blocking`: Span klonen und im Closure `span.in_scope(...)`/`let _g = span.enter()` setzen; Kontext propagiert nicht automatisch.
- Subscriber: `tracing-subscriber` mit `EnvFilter`; hohe Frequenz (Kernel-Level) auf `trace!`.

### 18. Traits + Mocking
- Kernel-/Mess-Schnittstellen als schmale Traits (`trait Gpu { fn embed(&self, batch:&Batch)->Result<Vec<f16>,E>; }`); Generics (statische Dispatch) im Hot-Path, `dyn` nur an Rändern.
- `mockall` 0.15.0 [V Version] geeignet für Orchestrator-Tests (`#[automock]`); für deterministische Fakes (z. B. fester Embedding-Output, simulierter Fehler) handgeschriebene `FakeGpu` oft einfacher, besonders bei async/Send-Anforderungen.
- Auswahl per Cargo-Feature: `#[cfg(feature = "cuda")] type DefaultBackend = CudaBackend; #[cfg(not(feature = "cuda"))] type DefaultBackend = CpuBackend;`.
- GPU-Integrationstests mit `#[ignore]` oder eigenem Feature, damit CI ohne GPU grün bleibt.

---

## Offene Punkte (priorisiert)
1. cudarc: 0.19.9 vs. 0.19.10 und Re-Export aus candle prüfen (`cargo tree -i cudarc`).
2. Parquet-f16-Roundtrip (`FixedSizeList<Float16,384>`) und mmap-Strategie in parquet 60.0.0 testen.
3. CUDA-Toolkit-Version festlegen (12.x empfohlen) und `CUDA_COMPUTE_CAP=75` im Build setzen.
4. `tokenizers` mit `default-features = false` gegen das XLM-R-`tokenizer.json` testen (Regex-Backend).
5. thiserror-2.0-Migrationsnotizen aus dem Changelog nachziehen.