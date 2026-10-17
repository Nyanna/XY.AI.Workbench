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

**Action required before first GPU build:** run `cargo tree -i cudarc` and `cargo tree -d` on a
machine with network access to confirm exactly one resolved `cudarc` version and no other
duplicated crate; record the resolved version from `Cargo.lock` here. Not executed in this
authoring environment (no network/CUDA toolchain available).

**Status: open**, tracked as restrisk #1 (see §5).

## 3. CUDA Toolkit / Compute Capability

- Decision: CUDA **12.x** (not 13.x).
- Build-time env var: `CUDA_COMPUTE_CAP=75`.
- Consequence: bf16 kernels are excluded (`-DNO_BF16_KERNEL`); only f16/f32 paths are planned
  for D4 (model/encoder) and D6 (kernels/scorer).
- This is a build-environment setting (not expressible purely in `Cargo.toml`); it must be set
  wherever `cargo build --features cuda` runs, e.g. in CI job config or a local `.cargo/config.toml`
  `[env]` section.

## 4. Roundtrip / Compatibility Tests

Minimal, `#[ignore]`d verification tests were added in `tests/stack_verification.rs`:

- `parquet_f16_roundtrip`: writes/reads a `FixedSizeList<Float16, 384>` column via parquet 60.0.0
  and reads it back through `memmap2` + `bytes::Bytes`, confirming the mmap-based read path.
- `tokenizer_loads_without_onig`: loads a `tokenizer.json` via `Tokenizer::from_file` with
  `tokenizers` compiled `default-features = false`, and exercises `encode`/`get_ids`/
  `get_attention_mask`.

Both are `#[ignore]`d because they require local fixture files (a `tokenizer.json` for the
XLM-R-based `intfloat/multilingual-e5-small` tokenizer) not present in this environment; they are
CI-tauglich (Q3) once such a fixture is added under `tests/fixtures/`.

Not covered by an automated test here (documented as restrisk instead):

- BertModel/e5-small tensor-name prefix check against `candle_transformers::models::bert::BertModel::load`
  — requires downloading `model.safetensors`/`config.json`, deferred to D4.
- `VarBuilder::from_mmaped_safetensors` GPU transfer copy — requires a CUDA device, deferred to D4.

## 5. Open Points / Risk List (from context_a.md)

| # | Open point | Status |
|---|---|---|
| 1 | cudarc 0.19.9 vs. 0.19.10 + candle re-export | **Open.** Pinned to `=0.19.9`; `cargo tree -i cudarc` must be run in a networked build environment before D-group work starts. Mitigation: exact pin prevents silent drift even if unresolved. |
| 2 | parquet-f16 roundtrip + mmap strategy | **Partially resolved.** Test written (`#[ignore]`); not executed here. Zero-copy only holds for uncompressed columns — documented in D7 handoff. |
| 3 | CUDA toolkit version + `CUDA_COMPUTE_CAP=75` | **Resolved** as a decision (§3); still needs to be wired into the actual CI/build config in A4/D-group. |
| 4 | `tokenizers` `default-features = false` vs. XLM-R `tokenizer.json` | **Partially resolved.** Feature flag set in `Cargo.toml`; test written (`#[ignore]`), pending fixture + execution. |
| 5 | thiserror 1→2 migration notes | **Resolved for this project**: `thiserror` is a fresh dependency here (A2 did not yet use thiserror), so no 1→2 migration applies. The A4 error taxonomy must use thiserror 2.0 syntax directly (named captures use `{field}`; raw identifiers use `{r#type}`). |

## 6. Acceptance Status

1. No known duplicate-crate issue by construction (exact `cudarc` pin); live `cargo tree -d`
   confirmation remains an open action item for the first networked build (#1 above).
2. Roundtrip/load tests exist as code but are `#[ignore]`d pending fixtures — documented workaround,
   not yet a green run.
3. Toolkit/compute-cap decision is recorded (§3); wiring into CI/build config is a follow-up for A4.
4. All five open points from `context_a.md` carry an explicit status above; none are silently
   unaddressed.
