# Dependency Verification & Risk List (A3)

> Part of Group A (Foundation & Context). Package A3 — Stack & Version Compatibility.
> Target hardware: GTX 1660, `sm_75` (Turing), 6 GB VRAM. CUDA 12.x, `CUDA_COMPUTE_CAP=75`.

## 1. Verified Versions (pinned in `Cargo.toml`)

Legend: **[V]** live verified (crates.io) · **[W]** domain knowledge, not live verified · **[?]** open.

| Crate | Pinned version | Origin |
|---|---|---|
| candle-core / candle-transformers | 0.11.0 | [V] |
| cudarc | `=0.19.9` (max stable) | [V], candle requires `^0.19.8` |
| tokenizers | 0.23.2, `default-features = false` | [V] |
| safetensors | 0.8.0 | [V] |
| parquet / arrow | 60.0.0 (identical major) | [V] |
| tokio | 1, features `rt-multi-thread,sync,macros,time` | [V] |
| crossbeam / crossbeam-queue | 0.8.5 / 0.3.14 | [V] |
| parking_lot | 0.12.5 | [V] |
| arc-swap | 1.9.2 | [V] |
| thiserror | 2.0.20 | [V] |
| tracing / tracing-subscriber | 0.1.44 / 0.3.23 | [V] |
| half | 2.7.1 | [V] |
| mockall | 0.15.0 (dev-dependency) | [V] |
| memmap2 | 0.9.3 | [V], required by safetensors `^0.9.3` |

All CUDA-only crates (`candle-core`, `candle-transformers`, `cudarc`) are `optional = true` and
enabled solely via the `cuda` feature, matching the no-GPU build requirement from A2.

## 2. cudarc Duplicate Check

A direct, exactly-pinned `cudarc = "=0.19.9"` dependency was added alongside candle so the
resolver unifies on a single version instead of letting candle's `^0.19.8` range float onto a
potential 0.19.10 release from candle's main branch. No `default-features = false` was set for
`cudarc`, so Cargo's feature unification keeps whatever feature set candle itself requires.

**GPU CI step:** run `cargo tree -i cudarc` and `cargo tree -d` with `--features cuda` to confirm exactly one
resolved `cudarc` version and record it from `Cargo.lock`. Not possible on hosts without the CUDA
toolkit. Duplicates without `cuda` are reviewed in §5.

**Status: residual risk** (see §5).

## 3. CUDA Toolkit / Compute Capability

- Decision: CUDA **12.x** (not 13.x).
- Build-time env var: `CUDA_COMPUTE_CAP=75`.
- Consequence: bf16 kernels are excluded (`-DNO_BF16_KERNEL`); only f16/f32 paths are planned
  for D4 (model/encoder) and D6 (kernels/scorer).
- This is a build-environment setting (not expressible purely in `Cargo.toml`); it must be set
  wherever `cargo build --features cuda` runs. `.cargo/config.toml` sets it as a non-forced default;
  CI sets it explicitly.

## 4. Roundtrip / Compatibility Tests

`tests/deps_smoke.rs` (runs in the default suite):

- `parquet_f16_fixed_size_list_round_trips_bit_exact`: writes and reads a `FixedSizeList<Float16>` column
  through parquet and compares the raw bits.
- `tokenizer_loads_from_serialized_json`: builds a word-level tokenizer, serializes it, loads it with
  `Tokenizer::from_file` (`tokenizers` with `default-features = false` plus `fancy-regex`) and encodes.

Not covered by an automated test (residual risk):

- Tensor-name prefixes of the target BERT model against `candle_transformers::models::bert::BertModel::load`
  (needs `model.safetensors`/`config.json`).
- `VarBuilder::from_mmaped_safetensors` transfer to the GPU (needs a CUDA device).

## 5. Open Points / Risk List

| # | Open point | Status |
|---|---|---|
| 1 | cudarc 0.19.9 vs. 0.19.10 + candle re-export | **Residual risk.** Pinned to `=0.19.9`; `cargo tree -i cudarc` can only be resolved with the `cuda` feature on the GPU CI job (no `nvcc` on the dev host). The exact pin prevents silent drift. |
| 2 | parquet f16 round-trip | **Resolved.** `tests/deps_smoke.rs` round-trips a `FixedSizeList<Float16>` column bit-exactly. Zero-copy mmap only holds for uncompressed columns. |
| 3 | CUDA toolkit version + `CUDA_COMPUTE_CAP=75` | **Resolved.** Anchored in `.cargo/config.toml` (`[env]`, not forced) and in the GPU CI job. |
| 4 | `tokenizers` without default features | **Resolved for loading.** Requires the `fancy-regex` feature; `tests/deps_smoke.rs` loads a serialized tokenizer. **Residual risk:** the XLM-R `tokenizer.json` of the target model is not covered (needs a fixture). |
| 5 | thiserror 1→2 | **Resolved.** Fresh dependency, taxonomy uses 2.x syntax. |
| 6 | Model tensor-name prefixes, `VarBuilder` mmap transfer to GPU | **Residual risk.** Needs model download and a CUDA device. |
| 7 | dp4a / bf16 behaviour on sm_75 | **Residual risk.** Needs hardware; marked with `TODO` in `precision::known`. |

### Duplicate crates (`cargo tree -d`, default features)

Reviewed and accepted: all duplicates are transitive and pure Rust, and none of their types cross a
public boundary of this crate.

| Crate | Versions | Origin |
|---|---|---|
| `rand`, `rand_core`, `rand_chacha`, `getrandom` | 0.8 / 0.9 (and 0.2 / 0.3 / 0.4) | direct `rand` 0.8 vs. parquet/arrow/tokenizers stack |
| `hashbrown`, `ahash`, `foldhash` | several | arrow, rusqlite, lru, tokenizers |
| `syn` | 1 / 2 / 3 | proc-macro build dependencies only |
| `base64`, `itertools` | two each | parquet/arrow vs. tokenizers/criterion |

Re-run `cargo tree -d` when bumping `arrow`/`parquet`/`tokenizers`; any duplicate of `cudarc` or `candle-*`
is a blocker.

### Vendored crates (`third_party/`, `[patch.crates-io]`)

| Crate | Change | Remove when |
|---|---|---|
| `candle-kernels` | build configuration for the installed toolkit | upstream builds with the pinned toolkit |
| `candle-transformers` 0.11.0 | `bert.rs`: `get_extended_attention_mask` uses a finite minimum per dtype (-65504 for f16). Upstream uses `f32::MIN`, which is `-inf` in f16 and yields `0 * -inf = NaN`. Examples and tests were dropped from the copy. | upstream fixes the mask; diff against the registry copy on every candle bump |

The BERT backbone runs in f16 on the GPU and f32 on the CPU (`backbone_dtype`). Reference tolerance against PyTorch f32: cosine > 0.9999 on CPU, > 0.999 on GPU; measured GPU values exceed 0.99999.

## 6. Acceptance Status

1. Duplicate review done and recorded above; `cudarc`/`candle` single-version check is a GPU CI step.
2. Round-trip and tokenizer-load tests are green and run in the default suite.
3. Toolkit/compute-cap decision is wired into the build configuration.
4. Every open point carries an explicit status; residual risks are listed.
