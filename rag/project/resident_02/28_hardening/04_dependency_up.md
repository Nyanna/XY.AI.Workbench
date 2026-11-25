Schreibe einen Hygiene/Arch Test der prüft das keine Dependencies von höher im Modulbaum liegenden Modulen eingebunden werden. Dependencies dürfen nicht nach oben zeigen. Das gilt für eintragabe verzeichnisse wie `/home/user/xyan/xy.ai.workbench/rag/src/resident`.

[include](/home/user/xyan/xy.ai.workbench/rag/project/resident_02/28_hardening/index.md)
SystemInit:  id=b1ed5547-549c-4d45-8cb3-07a6daf1a68c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die Struktur an.
Control Request:
```yaml
id: 906f-1-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag
    pattern: ^(?!.*(/target/|/model/|/\.git/)).*$
  reason: "Struktur des rag-Crates erfassen (Module, vorhandene Tests)"
```
/answer 906f-1-0 allow
ReasoningToken: 16
Control Request:
```yaml
id: 906f-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - "./src/resident: (1 file below)"
      - "./src/resident/l130: (1 file below)"
      - "./src/resident/l130/a: (4 files below)"
      - "./src/resident/l130/a/cuda_kernel: (4 files below)"
      - "./src/resident/l130/a/cuda_kernel/test: (3 files below)"
      - "./src/resident/l130/a/gpu_subengine: (10 files below)"
      - "./src/resident/l130/a/gpu_subengine/test: (8 files below)"
      - "./src/resident/l130/a/index_operations_coordinator: (3 files below)"
      - "./src/resident/l130/a/index_operations_coordinator/gpu_manager: (26 files below)"
      - "./src/resident/l130/a/index_operations_coordinator/load_strategy: (4 files below)"
      - "./src/resident/l130/a/index_operations_coordinator/test: (2 files below)"
      - "./src/resident/l130/a/test: (3 files below)"
      - "./src/resident/l130/l120: (3 files below)"
      - "./src/resident/l130/l120/l110: (1 file below)"
      - "./src/resident/l130/l120/l110/a: (15 files below)"
      - "./src/resident/l130/l120/l110/l100: (269 files below)"
      - "./src/resident/l130/l120/test: (2 files below)"
      - "./tests: (3 files below)"
      - "./tests/data/reference: (1 file below)"
      - "./third_party/candle-kernels: (6 files below)"
      - "./third_party/candle-kernels/src: (18 files below)"
      - "./third_party/candle-kernels/src/mmq_gguf: (15 files below)"
      - "./third_party/candle-kernels/src/moe: (5 files below)"
      - "./third_party/candle-transformers: (4 files below)"
      - "./third_party/candle-transformers/src: (6 files below)"
      - "./third_party/candle-transformers/src/generation: (1 file below)"
      - "./third_party/candle-transformers/src/models: (106 files below)"
      - "./third_party/candle-transformers/src/models/chinese_clip: (3 files below)"
      - "./third_party/candle-transformers/src/models/clip: (3 files below)"
      - "./third_party/candle-transformers/src/models/flux: (5 files below)"
      - "./third_party/candle-transformers/src/models/gemma4: (6 files below)"
      - "./third_party/candle-transformers/src/models/llava: (3 files below)"
      - "./third_party/candle-transformers/src/models/mimi: (6 files below)"
      - "./third_party/candle-transformers/src/models/mmdit: (5 files below)"
      - "./third_party/candle-transformers/src/models/nvembed_v2: (3 files below)"
      - "./third_party/candle-transformers/src/models/openclip: (2 files below)"
      - "./third_party/candle-transformers/src/models/paddleocr_vl: (4 files below)"
      - "./third_party/candle-transformers/src/models/pixtral: (3 files below)"
      - "./third_party/candle-transformers/src/models/qwen3_vl: (5 files below)"
      - "./third_party/candle-transformers/src/models/segment_anything: (7 files below)"
      - "./third_party/candle-transformers/src/models/smol: (4 files below)"
      - "./third_party/candle-transformers/src/models/stable_diffusion: (14 files below)"
      - "./third_party/candle-transformers/src/models/voxtral: (4 files below)"
      - "./third_party/candle-transformers/src/models/whisper: (4 files below)"
      - "./third_party/candle-transformers/src/models/wuerstchen: (7 files below)"
      - "./third_party/candle-transformers/src/models/z_image: (7 files below)"
      - "./third_party/candle-transformers/src/pipelines: (2 files below)"
      warnings:
      - Result reduced to directories up to depth 6 because the number of directories with matches (110) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 906f-2-1 allow
Control Request:
```yaml
id: 906f-3-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    id: read.Cargo.toml
  reason: "Cargo.toml lesen (Dependencies, Test-Konfiguration)"
```
/answer 906f-3-0 allow
Control Request:
```yaml
id: 906f-4-3
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        [package]
        name = "xy-ai-rag"
        version = "0.1.0"
        edition = "2021"
        description = "xy.ai.rag - Layered Anytime Retrieval Engine"

        [lib]
        name = "xy_ai_rag"
        path = "src/lib.rs"

        [[bin]]
        name = "xyrag"
        path = "src/bin/xyrag.rs"

        [dependencies]
        tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
        tokio-util = "0.7"
        log = "0.4"
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
        candle-core = { version = "0.11.0", default-features = false, features = ["cuda"] }
        candle-nn = { version = "0.11.0", default-features = false, features = ["cuda"] }
        candle-transformers = { version = "0.11.0", default-features = false, features = ["cuda"] }
        cudarc = "=0.19.9"
        roaring = "0.11.5"
        static_assertions = "1.1"

        [target.'cfg(target_os = "linux")'.dependencies]
        libc = "0.2"

        [dev-dependencies]
        proptest = "1"
        criterion = "0.5"
        mockall = "0.15.0"
        loom = "0.7"
        shuttle = "0.7"
        rand = "0.8"
        tokio = { version = "1", features = ["test-util"] }

        [features]
        gpu-tests = []

        [[bench]]
        name = "trie_bench"
        harness = false

        [patch.crates-io]
        candle-kernels = { path = "third_party/candle-kernels" }
        candle-transformers = { path = "third_party/candle-transformers" }
```
/answer 906f-4-3 allow
Control Request:
```yaml
id: 906f-5-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/tests
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "^[^/]+(/[^/]+){0,3}$"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120
    pattern: "^(l110/)?[^/]+$"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100
    recursive: false
  reason: Existierende Tests und Modulebenen im Detail ansehen
```
/answer 906f-5-0 allow
ReasoningToken: 86
Control Request:
```yaml
id: 906f-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/tests
      entries:
      - ".:"
      - api_hygiene.rs
      - deps_smoke.rs
      - trigram_e2e.rs
      - "./data/reference:"
      - multilingual-e5-small.json
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - mod.rs
      - "./l130:"
      - mod.rs
      - "./l130/a:"
      - cpu_subengine.rs
      - cuda_backend.rs
      - mod.rs
      - scoring_plan.rs
      - "./l130/l120:"
      - file_lazy_index.rs
      - manager.rs
      - mod.rs
      - "./l130/l120/l110/l100:"
      - mod.rs
      warnings:
      - "Items below this directory were merged into this result: each pattern was applied to its own directory and the matches were combined: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120 (pattern '^(l110/)?[^/]+$'), /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100."
```
/answer 906f-6-5 allow
Control Request:
```yaml
id: 906f-7-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "^\\s*(pub(\\([a-z]+\\))? )?mod \\w+"
    include:
    - mod.rs
    - lib.rs
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110
    pattern: "^\\s*(pub(\\([a-z]+\\))? )?mod \\w+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  outline:
  - /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
  reason: Moduldeklarationen und bestehende Hygiene-Tests lesen
```
/answer 906f-7-0 allow
Control Request:
```yaml
id: 906f-8-7
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src
        matches:
        - path: resident/mod.rs
          matches:
          - pub mod l130;
        - path: resident/l130/mod.rs
          matches:
          - pub mod a;
          - pub mod l120;
        - path: resident/l130/a/mod.rs
          matches:
          - pub mod cpu_subengine;
          - pub mod cuda_backend;
          - pub mod cuda_kernel;
          - pub mod gpu_subengine;
          - pub mod index_operations_coordinator;
          - pub mod scoring_plan;
        - path: resident/l130/a/gpu_subengine/mod.rs
          matches:
          - mod backend;
          - mod config;
          - mod deps;
          - mod device_manager;
          - mod error;
          - mod index_services;
          - mod policy;
          - mod subengine;
          - mod vram;
          - pub(crate) mod fixture;
        - path: resident/l130/a/index_operations_coordinator/mod.rs
          matches:
          - pub mod eviction;
          - pub mod gpu_manager;
          - pub mod index_ops;
          - pub mod load_strategy;
        - path: resident/l130/a/index_operations_coordinator/load_strategy/mod.rs
          matches:
          - pub mod profile_selection;
          - mod tests;
        - path: resident/l130/a/index_operations_coordinator/gpu_manager/mod.rs
          matches:
          - mod deps;
          - mod ensure;
          - mod lifecycle;
          - mod placement;
          - mod reconcile_pass;
          - mod services;
          - mod slot_services;
          - mod state;
          - mod types;
          - mod vram_sync;
          - mod write_back;
          - pub mod load_aggregation;
          - pub mod scope_resolution;
          - pub(crate) mod fixture;
          - mod tests;
        - path: resident/l130/a/cuda_kernel/mod.rs
          matches:
          - pub mod driver_launcher;
          - pub mod driver_loader;
          - pub mod nvcc_compiler;
        - path: resident/l130/l120/mod.rs
          matches:
          - pub mod file_lazy_index;
          - pub mod l110;
          - pub mod manager;
        - path: resident/l130/l120/l110/mod.rs
          matches:
          - pub mod a;
          - pub mod l100;
        - path: resident/l130/l120/l110/a/mod.rs
          matches:
          - pub mod backpressure;
          - pub mod processor;
        warnings:
        - "Limit of 50 matches reached (total 209); narrow pattern, directory or include/exclude filters."
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110
        matches:
        - path: mod.rs
          matches:
          - pub mod a;
          - pub mod l100;
        - path: a/mod.rs
          matches:
          - pub mod backpressure;
          - pub mod processor;
          - pub mod subengine_core;
          - pub mod worker_loop;
          - pub mod worker_pool;
          - pub mod subengine;
          - pub mod engine;
        - path: a/backpressure.rs
          matches:
          - mod tests;
        - path: a/worker_pool.rs
          matches:
          - mod tests;
        - path: a/engine.rs
          matches:
          - mod tests;
        - path: a/subengine_core.rs
          matches:
          - mod tests;
        - path: a/subengine.rs
          matches:
          - mod tests;
        - path: a/processor.rs
          matches:
          - mod tests;
        - path: a/worker_loop.rs
          matches:
          - mod tests;
        - path: l100/mod.rs
          matches:
          - pub mod a;
          - pub mod l090;
        - path: l100/a/mod.rs
          matches:
          - pub mod embedding_batch;
          - pub mod measurement;
          - pub mod plan_contract;
          - pub mod planner;
        - path: l100/a/embedding_batch/mod.rs
          matches:
          - pub mod write_pointer_region;
          - mod tests;
        - path: l100/a/embedding_batch/write_pointer_region.rs
          matches:
          - mod tests;
        - path: l100/a/planner.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/mod.rs
          matches:
          - pub mod bench_workload;
          - pub mod calibration_adapter;
          - pub mod cpu_measurer;
          - pub mod gpu_measurer;
          - pub mod hooks;
          - pub mod result;
          - mod tests;
        - path: l100/a/measurement/calibration_adapter.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/bench_workload.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/result.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/cpu_measurer.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/hooks.rs
          matches:
          - mod tests;
        - path: l100/a/measurement/gpu_measurer.rs
          matches:
          - mod tests;
        - path: l100/l090/mod.rs
          matches:
          - pub mod a;
          - pub mod l080;
        - path: l100/l090/a/gpu_kernel/mod.rs
          matches:
          - pub mod max_similarity_operation;
          - mod tests;
        - path: l100/l090/a/gpu_kernel/max_similarity_operation.rs
          matches:
          - mod tests;
        - path: l100/l090/a/gpu_kernel/test/gpu_kernel.rs
          matches:
          - "mod device {"
        - path: l100/l090/a/mod.rs
          matches:
          - pub mod gpu_kernel;
          - pub mod residency_control;
          - pub mod calibration_store;
          - pub mod entry_store;
          - pub mod gpu_completion;
        warnings:
        - "Limit of 50 matches reached (total 294); narrow pattern, directory or include/exclude filters."
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
        nodes:
        - id: ModuleDoc
          excert: //! Public signatures of the resident engine must not expose Candle tensors or …
        - id: 6rDhuO|zwcJ2N
          excert: use std::fs;
        - id: aCgTVc|c8OftX
          excert: "const ALLOWED: &[&str] = &["
        - id: sources
          excert: "fn sources(dir: &Path, out: &mut Vec<PathBuf>) {"
        - id: public_signatures
          excert: "fn public_signatures(text: &str) -> Vec<String> {"
        - id: has_word
          excert: "fn has_word(sig: &str, word: &str) -> bool {"
        - id: leaks
          excert: "fn leaks(sig: &str) -> bool {"
        - id: public_api_is_free_of_tensors_and_pointers
          excert: "fn public_api_is_free_of_tensors_and_pointers() {"
        - id: detector_flags_leaks
          excert: "fn detector_flags_leaks() {"
```
/answer 906f-8-7 allow
ReasoningToken: 1464
Control Request:
```yaml
id: 906f-9-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120
    pattern: (crate::|super::|self::)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/a
    pattern: (crate::resident|super::super|xy_ai_rag)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "#\\[path|#\\[cfg\\(test\\)\\]\\s*$"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 10
  reason: "Import-Stile (crate::, super::, self::, use-Gruppen, pfadattribute) im Quellbaum sichten"
```
/answer 906f-9-0 allow
Control Request:
```yaml
id: 906f-a-9
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120
        matches:
        - path: l110/a/backpressure.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::ResourceKey;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::load_meter::LoadReport;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ResourceKind;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::subengine_loop::{IdleNotice, IterationOutcome};"
        - path: l110/a/worker_pool.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::background_pool::PoolError;
        - path: l110/a/engine.rs
          matches:
          - use crate::resident::l130::l120::l110::a::backpressure::ExpansionControl;
          - use crate::resident::l130::l120::l110::l100::l090::a::gpu_completion::GpuCompletion;
          - "use crate::resident::l130::l120::l110::a::processor::{ExpandError, Processor};"
          - "use crate::resident::l130::l120::l110::l100::a::plan_contract::{"
          - use crate::resident::l130::l120::l110::a::subengine::SubengineHandle;
          - "use crate::resident::l130::l120::l110::a::worker_loop::{"
          - "use crate::resident::l130::l120::l110::a::worker_pool::{"
          - "use crate::resident::l130::l120::l110::l100::a::planner::{"
          - use crate::resident::l130::l120::l110::l100::l090::a::model_registry::ModelRegistry;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::notification_topology::{"
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::chain_handover::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::expander::PlannedOperation;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::step_completion::{"
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::b::task_context::TaskId;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::Index;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ModelDescriptor;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::b::task_model::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::CapabilitySet;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::Queue;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
        - path: l110/a/subengine_core.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::a::plan_contract::QueueKind;
          - use crate::resident::l130::l120::l110::l100::a::planner::Subengine;
          - use crate::resident::l130::l120::l110::l100::l090::a::model_registry::LoadedEncoder;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{"
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::RegionHandle;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{"
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{"
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
        - path: l110/a/subengine.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::a::gpu_completion::FinishedLane;
          - use crate::resident::l130::l120::l110::a::worker_loop::FeedStep;
          - use crate::resident::l130::l120::l110::l100::a::planner::Subengine;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
        - path: l110/a/test/backpressure.rs
          matches:
          - use super::*;
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceHandle;
        - path: l110/a/test/worker_pool.rs
          matches:
          - use super::*;
        - path: l110/a/test/engine.rs
          matches:
          - use super::*;
          - use crate::resident::l130::l120::l110::l100::l090::l080 as l080;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as l060;
          - use crate::resident::l130::l120::l110::l100::a::planner::PlanStep;
        warnings:
        - "Limit of 50 matches reached (total 855); narrow pattern, directory or include/exclude filters."
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/a
        matches:
        - path: cuda_backend.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090 as l090;
        - path: scoring_plan.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080;
        - path: cpu_subengine.rs
          matches:
          - use crate::resident::l130::l120::l110;
          - use crate::resident::l130::l120::file_lazy_index::FileLazyIndex;
          - use crate::resident::l130::l120::manager::LazyIndex;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
        - path: gpu_subengine/deps.rs
          matches:
          - use crate::resident::l130::l120::l110 as l110;
          - "pub(super) use super::super::index_operations_coordinator::eviction::{Eviction, TierBudget};"
          - "pub(super) use super::super::index_operations_coordinator::gpu_manager::scope_resolution::{"
          - "pub(super) use super::super::index_operations_coordinator::gpu_manager::{"
          - "pub(super) use super::super::index_operations_coordinator::index_ops::{"
        - path: gpu_subengine/test/index_services.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
          - "use super::super::fixture::{config, MockBackend, SEGMENTS_BASE};"
          - use super::super::vram::VramManager;
        - path: gpu_subengine/test/policy.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::ExecutionEnvironment;
          - use super::super::config::default_kernel_names;
          - use super::super::fixture::MockBackend;
          - type Axis = crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::Axis;
          - type Value = crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::Value;
        - path: gpu_subengine/test/device_manager.rs
          matches:
          - "use super::super::fixture::{config, shared, MockBackend};"
        - path: gpu_subengine/test/subengine.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090 as l090;
          - "use super::super::fixture::{config, device, shared, shared_with, subengine, MockBackend};"
          - "super::super::fixture::subengine_with(& backend, cfg).compute_dtype(), DType::F32"
          - "let sub = super::super::fixture::subengine_with(&backend, cfg);"
        - path: gpu_subengine/test/fixture.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090 as l090;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060 as l060;
        - path: gpu_subengine/test/vram.rs
          matches:
          - "use super::super::fixture::{config, MockBackend, MIB};"
        - path: test/cuda_backend.rs
          matches:
          - "use super::super::gpu_subengine::fixture::{config, shared, MIB};"
          - "use super::super::gpu_subengine::{"
          - use crate::resident::l130::l120::l110::l100::a::planner::Subengine;
        - path: index_operations_coordinator/load_strategy/mod.rs
          matches:
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::{"
        - path: index_operations_coordinator/load_strategy/profile_selection.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::pareto_profiles::PrecisionProfile;
        - path: index_operations_coordinator/load_strategy/test/load_strategy.rs
          matches:
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::{"
        - path: index_operations_coordinator/gpu_manager/load_aggregation.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::reconcile::LoadedEntry;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020 as l020;
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
        - path: index_operations_coordinator/gpu_manager/scope_resolution.rs
          matches:
          - "use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::reconcile::{"
          - use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020 as l020;
        - path: index_operations_coordinator/gpu_manager/deps.rs
          matches:
          - use crate::resident::l130::l120::l110::l100::l090 as l090;
        warnings:
        - "Limit of 40 matches reached (total 78); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src
        matches:
        - path: resident/l130/a/cuda_backend.rs
          matches:
          - "#[cfg(test)]"
          - "#[path = \"test/cuda_backend.rs\"]"
        - path: resident/l130/a/scoring_plan.rs
          matches:
          - "#[cfg(test)]"
          - "#[path = \"test/scoring_plan.rs\"]"
        - path: resident/l130/a/cpu_subengine.rs
          matches:
          - "#[cfg(test)]"
          - "#[path = \"test/cpu_subengine.rs\"]"
        - path: resident/l130/a/gpu_subengine/mod.rs
          matches:
          - "#[cfg(test)]"
          - "#[path = \"test/fixture.rs\"]"
        - path: resident/l130/a/gpu_subengine/index_services.rs
          matches:
          - "#[cfg(test)]"
          - "#[path = \"test/index_services.rs\"]"
        warnings:
        - "Limit of 10 matches reached (total 324); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 906f-a-9 allow
Control Request:
```yaml
id: 906f-b-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/bin
  reason: Top-level src Inhalt prüfen
```
/answer 906f-b-0 allow
ReasoningToken: 4147
Control Request:
```yaml
id: 906f-c-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      entries: []
    - path: /home/user/xyan/xy.ai.workbench/rag/src/bin
      entries: []
```
/answer 906f-c-b allow
Control Request:
```yaml
id: 906f-d-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
  reason: lib.rs und bestehenden Hygiene-Test als Stilvorlage lesen
```
/answer 906f-d-0 allow
Control Request:
```yaml
id: 906f-e-d
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
        nodes:
        - id: ModuleDoc
          excert: //! xy.ai.rag - Layered Anytime Retrieval Engine.
        - id: wlWS30|6Rdvlf
          excert: pub mod core;
        - id: xLpY3U|tx8ZzX
          excert: "pub const VERSION: &str = env!(\"CARGO_PKG_VERSION\");"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/rag/src/lib.rs instead."
      - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
        nodes:
        - id: ModuleDoc
          excert: //! Public signatures of the resident engine must not expose Candle tensors or …
        - id: 6rDhuO|zwcJ2N
          excert: use std::fs;
        - id: aCgTVc|c8OftX
          excert: "const ALLOWED: &[&str] = &["
        - id: sources
          excert: "fn sources(dir: &Path, out: &mut Vec<PathBuf>) {"
        - id: public_signatures
          excert: "fn public_signatures(text: &str) -> Vec<String> {"
        - id: has_word
          excert: "fn has_word(sig: &str, word: &str) -> bool {"
        - id: leaks
          excert: "fn leaks(sig: &str) -> bool {"
        - id: public_api_is_free_of_tensors_and_pointers
          excert: "fn public_api_is_free_of_tensors_and_pointers() {"
        - id: detector_flags_leaks
          excert: "fn detector_flags_leaks() {"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs instead."
```
/answer 906f-e-d allow
ReasoningToken: 89
Control Request:
```yaml
id: 906f-f-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
    regexp: .
  - paths:
    - /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
    node_type: function|const|use
    regexp: .
  reason: Vollständigen Inhalt von lib.rs und api_hygiene.rs lesen (lib.rs zeigt `pub mod core;` statt `pub mod resident;`)
```
/answer 906f-f-0 allow
Control Request:
```yaml
id: 906f-10-f
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
        nodes:
        - id: ModuleDoc
          code: |
            //! xy.ai.rag - Layered Anytime Retrieval Engine.
            //!
            //! Used both as a library (`xy_ai_rag::core::...`) and as an on-demand CLI
            //! utility (`xyrag`, see `src/bin/xyrag.rs`) - no daemon or server process.
        - id: wlWS30|6Rdvlf
          code: |
            pub mod core;
            pub mod layers;
            pub mod resident;
        - id: xLpY3U|tx8ZzX
          code: |
            pub const VERSION: &str = env!("CARGO_PKG_VERSION");
      - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
        nodes: []
```
/answer 906f-10-f allow
ReasoningToken: 107
Control Request:
```yaml
id: 906f-11-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
    ids:
    - ModuleDoc
    - aCgTVc|c8OftX
    - sources
    - public_api_is_free_of_tensors_and_pointers
    - detector_flags_leaks
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ^$
    include:
    - core.rs
    - layers.rs
    - mod.rs
    exclude:
    - resident/**
    limit: 5
  reason: "api_hygiene.rs im Detail lesen und prüfen, ob src/core und src/layers existieren"
```
/answer 906f-11-0 allow
Control Request:
```yaml
id: 906f-12-11
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/rag/src
        matches:
        - path: resident/l130/a/gpu_subengine/mod.rs
          matches:
          - ""
        - path: resident/l130/a/index_operations_coordinator/load_strategy/mod.rs
          matches:
          - ""
        warnings:
        - "Limit of 5 matches reached (total 76); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/rag/tests/api_hygiene.rs
        nodes:
        - id: ModuleDoc
          type: ModuleAttrs
          code: |
            //! Public signatures of the resident engine must not expose Candle tensors or raw pointers.
            //! The Candle adapter and the CUDA backends are the only places allowed to touch them.
        - id: aCgTVc|c8OftX
          type: StatementGroup
          code: |
            const ALLOWED: &[&str] = &[
                "model/candle_adapter.rs",
                "kernel/cuda.rs",
                "kernel/maxsim.rs",
                "l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/a/library_adapter.rs",
                "l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/consistent_snapshots.rs",
                "l130/l120/l110/l100/l090/l080/a/communication_slot/device_transfer/staging_ring.rs",
            ];
        - id: sources
          type: ItemFn
          code: |
            fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
                for entry in fs::read_dir(dir).unwrap() {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        sources(&path, out);
                    } else if path.extension().is_some_and(|e| e == "rs") {
                        out.push(path);
                    }
                }
            }
        - id: public_api_is_free_of_tensors_and_pointers
          type: ItemFn
          code: |
            #[test]
            fn public_api_is_free_of_tensors_and_pointers() {
                let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
                let mut files = Vec::new();
                sources(&root, &mut files);
                assert!(! files.is_empty());
                let mut violations = Vec::new();
                for file in files {
                    let rel = file.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
                    if ALLOWED.contains(&rel.as_str()) {
                        continue;
                    }
                    let text = fs::read_to_string(&file).unwrap();
                    let body = text.split("#[cfg(test)]").next().unwrap_or(&text);
                    for sig in public_signatures(body) {
                        if leaks(&sig) {
                            violations.push(format!("{rel}: {sig}"));
                        }
                    }
                }
                assert!(violations.is_empty(), "leaking signatures:\n{}", violations.join("\n"));
            }
        - id: detector_flags_leaks
          type: ItemFn
          code: |
            #[test]
            fn detector_flags_leaks() {
                let sigs = public_signatures(
                    "pub fn a(t: &Tensor) -> u8 {\n}\npub fn b(\n    p: *const u8,\n) {\n}\npub fn c() {}",
                );
                let flagged: Vec<_> = sigs.iter().map(|s| leaks(s)).collect();
                assert_eq!(flagged, vec![true, true, false]);
            }
```
/answer 906f-12-11 allow
Text:
 Ich lege den Test jetzt an. Er prüft den Modulbaum unter `src/resident` anhand der Ebenenverzeichnisse `lNNN`.
ReasoningToken: 15881
Control Request:
```yaml
id: 906f-1b-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/rag/tests/dependency_direction.rs
    overwrite: false
    source: |
      //! Dependencies inside the resident tree must never point upward.
      //!
      //! A module may only depend on modules of its own layer or of lower layers, and never on one of
      //! its enclosing modules. The layer of a module is the number of its nearest `lNNN` path segment.
      //! Test files and `#[cfg(test)]` items are not part of the check.

      use regex::Regex;
      use std::collections::BTreeSet;
      use std::fs;
      use std::path::{Path, PathBuf};

      type ModPath = Vec<String>;

      struct SourceFile {
          rel: String,
          module: ModPath,
          text: String,
      }

      fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
          let mut entries: Vec<PathBuf> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
          entries.sort();
          for path in entries {
              if path.is_dir() {
                  collect(&path, out);
              } else if path.extension().is_some_and(|e| e == "rs") {
                  out.push(path);
              }
          }
      }

      fn is_test_file(rel: &Path) -> bool {
          rel.components().any(|c| {
              let name = c.as_os_str().to_string_lossy();
              name == "test" || name == "tests" || name == "tests.rs"
          })
      }

      fn module_of(rel: &Path) -> ModPath {
          let parts: Vec<String> = rel.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
          let mut module = vec!["resident".to_string()];
          for (index, part) in parts.iter().enumerate() {
              if index + 1 == parts.len() {
                  let stem = part.strip_suffix(".rs").unwrap_or(part);
                  if stem != "mod" {
                      module.push(stem.to_string());
                  }
              } else {
                  module.push(part.clone());
              }
          }
          module
      }

      fn load_sources() -> (Vec<SourceFile>, BTreeSet<ModPath>) {
          let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
          let mut paths = Vec::new();
          collect(&root, &mut paths);
          let mut files = Vec::new();
          let mut modules = BTreeSet::new();
          for path in paths {
              let rel = path.strip_prefix(&root).unwrap().to_path_buf();
              if is_test_file(&rel) {
                  continue;
              }
              let module = module_of(&rel);
              modules.insert(module.clone());
              files.push(SourceFile {
                  rel: rel.to_string_lossy().replace('\\', "/"),
                  module,
                  text: fs::read_to_string(&path).unwrap(),
              });
          }
          (files, modules)
      }

      fn layer_number(segment: &str) -> Option<u32> {
          let digits = segment.strip_prefix('l')?;
          if digits.len() == 3 && digits.chars().all(|c| c.is_ascii_digit()) {
              digits.parse().ok()
          } else {
              None
          }
      }

      fn layer_of(module: &[String]) -> u32 {
          module.iter().rev().find_map(|s| layer_number(s)).unwrap_or(u32::MAX)
      }

      fn blank(out: &mut String, c: char) {
          if c == '\n' {
              out.push('\n');
          } else {
              for _ in 0..c.len_utf8() {
                  out.push(' ');
              }
          }
      }

      fn raw_string_end(chars: &[char], start: usize) -> Option<usize> {
          let mut j = start + 1;
          let mut hashes = 0;
          while chars.get(j) == Some(&'#') {
              hashes += 1;
              j += 1;
          }
          if chars.get(j) != Some(&'"') {
              return None;
          }
          j += 1;
          while j < chars.len() {
              if chars[j] == '"' && (0..hashes).all(|k| chars.get(j + 1 + k) == Some(&'#')) {
                  return Some(j + 1 + hashes);
              }
              j += 1;
          }
          Some(chars.len())
      }

      fn strip_comments_and_literals(text: &str) -> String {
          let chars: Vec<char> = text.chars().collect();
          let mut out = String::with_capacity(text.len());
          let mut i = 0;
          while i < chars.len() {
              let c = chars[i];
              let next = chars.get(i + 1).copied();
              let prev_is_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
              if c == '/' && next == Some('/') {
                  while i < chars.len() && chars[i] != '\n' {
                      blank(&mut out, chars[i]);
                      i += 1;
                  }
              } else if c == '/' && next == Some('*') {
                  let mut depth = 0usize;
                  while i < chars.len() {
                      if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                          depth += 1;
                          blank(&mut out, '/');
                          blank(&mut out, '*');
                          i += 2;
                      } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                          depth -= 1;
                          blank(&mut out, '*');
                          blank(&mut out, '/');
                          i += 2;
                          if depth == 0 {
                              break;
                          }
                      } else {
                          blank(&mut out, chars[i]);
                          i += 1;
                      }
                  }
              } else if c == 'r' && !prev_is_word && raw_string_end(&chars, i).is_some() {
                  let end = raw_string_end(&chars, i).unwrap();
                  while i < end {
                      blank(&mut out, chars[i]);
                      i += 1;
                  }
              } else if c == '"' {
                  blank(&mut out, c);
                  i += 1;
                  while i < chars.len() {
                      let d = chars[i];
                      blank(&mut out, d);
                      i += 1;
                      if d == '\\' && i < chars.len() {
                          blank(&mut out, chars[i]);
                          i += 1;
                      } else if d == '"' {
                          break;
                      }
                  }
              } else if c == '\'' {
                  let end = if next == Some('\\') {
                      (i + 3..chars.len()).find(|&j| chars[j] == '\'').map(|j| j + 1)
                  } else if chars.get(i + 2) == Some(&'\'') {
                      Some(i + 3)
                  } else {
                      None
                  };
                  match end {
                      Some(end) => {
                          while i < end {
                              blank(&mut out, chars[i]);
                              i += 1;
                          }
                      }
                      None => {
                          out.push(c);
                          i += 1;
                      }
                  }
              } else {
                  out.push(c);
                  i += 1;
              }
          }
          out
      }

      fn matching(b: &[u8], start: usize, open: u8, close: u8) -> usize {
          let mut depth = 0i32;
          let mut i = start;
          while i < b.len() {
              if b[i] == open {
                  depth += 1;
              } else if b[i] == close {
                  depth -= 1;
                  if depth == 0 {
                      return i + 1;
                  }
              }
              i += 1;
          }
          b.len()
      }

      fn item_end(text: &str, start: usize) -> usize {
          let b = text.as_bytes();
          let mut i = start;
          loop {
              while i < b.len() && b[i].is_ascii_whitespace() {
                  i += 1;
              }
              if b[i..].starts_with(b"#[") {
                  i = matching(b, i + 1, b'[', b']');
              } else {
                  break;
              }
          }
          let is_use = Regex::new(r"^(pub(\s*\([^)]*\))?\s+)?use\b").unwrap().is_match(&text[i..]);
          let mut depth = 0i32;
          while i < b.len() {
              match b[i] {
                  b'(' | b'[' => depth += 1,
                  b')' | b']' => depth -= 1,
                  b';' if depth <= 0 => return i + 1,
                  b'{' if depth <= 0 && !is_use => return matching(b, i, b'{', b'}'),
                  _ => {}
              }
              i += 1;
          }
          b.len()
      }

      fn blank_range(bytes: &mut [u8], range: std::ops::Range<usize>) {
          for byte in &mut bytes[range] {
              if *byte != b'\n' {
                  *byte = b' ';
              }
          }
      }

      fn drop_cfg_test_items(text: &str) -> String {
          let attribute = Regex::new(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]").unwrap();
          let mut bytes = text.as_bytes().to_vec();
          let mut from = 0;
          while let Some(found) = attribute.find_at(text, from) {
              let end = item_end(text, found.end());
              blank_range(&mut bytes, found.start()..end);
              from = end.max(found.end());
          }
          String::from_utf8(bytes).unwrap()
      }

      fn split_uses(text: &str) -> (Vec<(usize, String)>, String) {
          let keyword = Regex::new(r"\buse\b").unwrap();
          let mut rest = text.as_bytes().to_vec();
          let mut uses = Vec::new();
          let mut from = 0;
          while let Some(found) = keyword.find_at(text, from) {
              let Some(len) = text[found.end()..].find(';') else {
                  break;
              };
              let end = found.end() + len;
              uses.push((found.start(), text[found.end()..end].to_string()));
              blank_range(&mut rest, found.start()..end + 1);
              from = end + 1;
          }
          (uses, String::from_utf8(rest).unwrap())
      }

      fn tokenize(body: &str) -> Vec<String> {
          let chars: Vec<char> = body.chars().collect();
          let mut tokens = Vec::new();
          let mut i = 0;
          while i < chars.len() {
              let c = chars[i];
              if c.is_alphanumeric() || c == '_' {
                  let begin = i;
                  while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                      i += 1;
                  }
                  tokens.push(chars[begin..i].iter().collect());
                  continue;
              }
              if c == ':' && chars.get(i + 1) == Some(&':') {
                  tokens.push("::".to_string());
                  i += 2;
                  continue;
              }
              if matches!(c, '{' | '}' | ',' | '*') {
                  tokens.push(c.to_string());
              }
              i += 1;
          }
          tokens
      }

      fn expand(tokens: &[String], pos: &mut usize, prefix: &[String], out: &mut Vec<Vec<String>>) {
          let mut path = prefix.to_vec();
          loop {
              match tokens.get(*pos).map(String::as_str) {
                  Some("::") => *pos += 1,
                  Some("as") => *pos += 2,
                  Some("*") => {
                      *pos += 1;
                      break;
                  }
                  Some("{") => {
                      *pos += 1;
                      loop {
                          expand(tokens, pos, &path, out);
                          match tokens.get(*pos).map(String::as_str) {
                              Some(",") => *pos += 1,
                              Some("}") => {
                                  *pos += 1;
                                  break;
                              }
                              _ => break,
                          }
                      }
                      return;
                  }
                  Some(",") | Some("}") | None => break,
                  Some(ident) => {
                      path.push(ident.to_string());
                      *pos += 1;
                  }
              }
          }
          if path.len() > prefix.len() {
              if path.len() > 1 && path.last().map(String::as_str) == Some("self") {
                  path.pop();
              }
              out.push(path);
          }
      }

      fn expand_use(body: &str) -> Vec<Vec<String>> {
          let tokens = tokenize(body);
          let mut out = Vec::new();
          let mut pos = 0;
          expand(&tokens, &mut pos, &[], &mut out);
          out
      }

      fn resolve(path: &[String], source: &[String]) -> Option<ModPath> {
          let (head, tail) = path.split_first()?;
          let (mut base, mut tail): (ModPath, &[String]) = match head.as_str() {
              "crate" => (Vec::new(), tail),
              "self" => (source.to_vec(), tail),
              "super" => {
                  let mut base = source.to_vec();
                  base.pop()?;
                  (base, tail)
              }
              _ => return None,
          };
          while tail.first().map(String::as_str) == Some("super") {
              base.pop()?;
              tail = &tail[1..];
          }
          base.extend(tail.iter().cloned());
          (base.first().map(String::as_str) == Some("resident")).then_some(base)
      }

      fn target_module(full: &[String], modules: &BTreeSet<ModPath>) -> Option<ModPath> {
          (1..=full.len()).rev().map(|k| full[..k].to_vec()).find(|candidate| modules.contains(candidate))
      }

      fn violation(source: &[String], target: &[String]) -> Option<&'static str> {
          if target.len() < source.len() && source.starts_with(target) {
              return Some("enclosing module");
          }
          if layer_of(target) > layer_of(source) {
              return Some("higher layer");
          }
          None
      }

      fn line_at(text: &str, offset: usize) -> usize {
          text.as_bytes()[..offset].iter().filter(|b| **b == b'\n').count() + 1
      }

      fn analyze(text: &str, source: &[String], modules: &BTreeSet<ModPath>) -> Vec<(usize, ModPath, &'static str)> {
          let cleaned = drop_cfg_test_items(&strip_comments_and_literals(text));
          let (uses, rest) = split_uses(&cleaned);
          let mut paths: Vec<(usize, Vec<String>)> = Vec::new();
          for (offset, body) in uses {
              let line = line_at(&cleaned, offset);
              for path in expand_use(&body) {
                  paths.push((line, path));
              }
          }
          let inline = Regex::new(r"\b(?:crate|super|self)(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)+").unwrap();
          for found in inline.find_iter(&rest) {
              let path = found.as_str().split("::").map(|s| s.trim().to_string()).collect();
              paths.push((line_at(&rest, found.start()), path));
          }
          let mut found = Vec::new();
          for (line, path) in paths {
              let Some(full) = resolve(&path, source) else {
                  continue;
              };
              let Some(target) = target_module(&full, modules) else {
                  continue;
              };
              if let Some(reason) = violation(source, &target) {
                  found.push((line, target, reason));
              }
          }
          found
      }

      #[test]
      fn dependencies_never_point_upward() {
          let (files, modules) = load_sources();
          assert!(!files.is_empty());
          let mut violations = Vec::new();
          for file in &files {
              for (line, target, reason) in analyze(&file.text, &file.module, &modules) {
                  violations.push(format!("{}:{line}: depends on {} ({reason})", file.rel, target.join("::")));
              }
          }
          violations.sort();
          violations.dedup();
          assert!(violations.is_empty(), "upward dependencies:\n{}", violations.join("\n"));
      }

      #[test]
      fn layer_numbers_decrease_with_depth() {
          let (_, modules) = load_sources();
          let mut violations = Vec::new();
          for module in &modules {
              let numbers: Vec<u32> = module.iter().filter_map(|s| layer_number(s)).collect();
              if numbers.windows(2).any(|w| w[0] <= w[1]) {
                  violations.push(module.join("::"));
              }
          }
          assert!(violations.is_empty(), "layer numbers must strictly decrease:\n{}", violations.join("\n"));
      }

      fn path_of(text: &str) -> ModPath {
          text.split("::").map(str::to_string).collect()
      }

      fn sample_modules() -> BTreeSet<ModPath> {
          [
              "resident",
              "resident::l130",
              "resident::l130::a",
              "resident::l130::a::cpu",
              "resident::l130::a::sibling",
              "resident::l130::l120",
              "resident::l130::l120::manager",
              "resident::l130::l120::l110",
              "resident::l130::l120::l110::a",
              "resident::l130::l120::l110::a::engine",
              "resident::l130::l120::l110::l100",
              "resident::l130::l120::l110::l100::planner",
          ]
          .iter()
          .map(|s| path_of(s))
          .collect()
      }

      fn targets(source: &str, text: &str) -> Vec<String> {
          analyze(text, &path_of(source), &sample_modules()).into_iter().map(|(_, target, _)| target.join("::")).collect()
      }

      #[test]
      fn use_trees_are_expanded() {
          let paths = expand_use(" crate::a::{b, c::{d, self}, e as f, g::*,} ");
          let flat: Vec<String> = paths.iter().map(|p| p.join("::")).collect();
          assert_eq!(flat, vec!["crate::a::b", "crate::a::c::d", "crate::a::c", "crate::a::e", "crate::a::g"]);
      }

      #[test]
      fn detector_accepts_downward_and_sibling_dependencies() {
          let cpu = "resident::l130::a::cpu";
          assert!(targets(cpu, "use crate::resident::l130::l120::manager::LazyIndex;").is_empty());
          assert!(targets(cpu, "use super::sibling::Thing;").is_empty());
          assert!(targets(cpu, "use super::super::l120::manager::Manager;").is_empty());
          let engine = "resident::l130::l120::l110::a::engine";
          assert!(targets(engine, "use crate::resident::l130::l120::l110::{self, l100::planner::Plan};").is_empty());
      }

      #[test]
      fn detector_flags_upward_dependencies() {
          let manager = "resident::l130::l120::manager";
          assert_eq!(targets(manager, "use crate::resident::l130::a::cpu::Thing;"), vec!["resident::l130::a::cpu"]);
          assert_eq!(
              targets(manager, "use crate::resident::l130::{a::cpu::Thing, l120::manager as own};"),
              vec!["resident::l130::a::cpu"]
          );
          assert_eq!(
              targets(manager, "fn f() { let _ = crate::resident::l130::a::cpu::build(); }"),
              vec!["resident::l130::a::cpu"]
          );
          assert_eq!(
              targets("resident::l130::l120::l110::l100::planner", "use crate::resident::l130::l120::l110::a::engine::E;"),
              vec!["resident::l130::l120::l110::a::engine"]
          );
      }

      #[test]
      fn detector_flags_enclosing_modules() {
          let cpu = "resident::l130::a::cpu";
          assert_eq!(targets(cpu, "use super::*;"), vec!["resident::l130::a"]);
          assert_eq!(targets(cpu, "use super::Shared;"), vec!["resident::l130::a"]);
          assert_eq!(targets(cpu, "use crate::resident::l130::Shared;"), vec!["resident::l130"]);
      }

      #[test]
      fn detector_ignores_comments_literals_and_test_items() {
          let text = "
      // use crate::resident::l130::a::cpu::X;
      /* use crate::resident::l130::a::cpu::X; */
      const S: &str = \"crate::resident::l130::a::cpu::X\";
      #[cfg(test)]
      mod tests {
          use crate::resident::l130::a::cpu::X;
      }
      #[cfg(test)]
      #[path = \"test/manager.rs\"]
      mod tests;
      #[cfg(test)]
      use crate::resident::l130::a::cpu::{X, Y};
      ";
          assert!(targets("resident::l130::l120::manager", text).is_empty());
      }
  reason: Neuen Architektur-Test tests/dependency_direction.rs anlegen
```
/answer 906f-1b-0 allow
Control Request:
```yaml
id: 906f-1d-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    id: runTests
  reason: Neuen Test kompilieren und gegen den echten Quellbaum laufen lassen
```
/answer 906f-1d-0 allow
Control Request:
```yaml
id: 906f-1e-1d
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 101
      stdout: |
        FAIL (exit 101): 2 failed, 2649 passed - 0 errors, 0 warnings
        Failed tests:
          detector_accepts_downward_and_sibling_dependencies @ tests/dependency_direction.rs:497:5: assertion failed: targets(engine,
          dependencies_never_point_upward @ tests/dependency_direction.rs:439:5: upward dependencies:
        Full details: /tmp/cargo-test-dump-20261125-224528.log
```
/answer 906f-1e-1d allow
Result Stats: id=b1ed5547-549c-4d45-8cb3-07a6daf1a68c, total: 85783, in: 18, out: 32636, reason: 0, read: 147451, write: 53129, start: 1795643128381, end: 1795643128754, time: 370, total: 253203, io: 0, control: 3, sent: 0, recv: 5
You've hit your session limit · resets 1am (Europe/Berlin)
