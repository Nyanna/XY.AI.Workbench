# precision

Capability sets, calibration, profile building, matching of a precision wish to a profile, and persistence of the results.

## Concept

- **Capabilities, not profile names.** A capability set maps open axis identifiers (operand type, accumulator, scoring type, fusion, batch, tile, threads, …) to the values a provider offers or a task requires. There is no profile enum; axes and values are extensible per hardware without touching facade or task interface. Each subengine decides its compute paths itself; there is no rounding-point or dtype contract between CPU and GPU, and results are accepted variables, not guarantees. `known.rs` lists the axes for the target hardware (GTX 1660, sm_75; bf16 deliberately absent). Numeric axes (`batch`, `tile`, `threads`) carry measured values and are not listed statically; the same holds for `density`, the index manager's gather-versus-segment load threshold (permille of a segment's bytes that must be requested for the whole segment to be loaded). A profile configuration containing `density` overrides the manager default.
- **Probe, then calibrate.** Probing is optimistic and measures nothing; its result is intersected with the kernels that exist. Unusable combinations are removed by calibration.
- **Calibrator.** Enumerates combinations (with pruning), measures them interleaved against the fp32 reference, rejects invalid ones (NaN/Inf, degenerate output, deviation above the configurable maximum; such a combination counts as incompatible and is merely not activated) and keeps the Pareto front over (time, deviation).
  - *Measurement:* every combination is a mini-benchmark with at least 3 repetitions. Rounds run round-robin over all active combinations, odd rounds in reverse order. Only relations count: stability is judged on the time ratio to the first combination, not on single times. Estimates are robust (median or minimum), spread is a coefficient of variation or relative range with the slowest fifth ignored.
  - *Termination:* measurement ends on stability, not after a fixed time. A hard cap (repetitions, time per combination, budget of the whole run) ends it otherwise; then the best intermediate state is kept, logged and marked `unstable`. Calibration never blocks or aborts.
  - *Load:* calibration runs on dedicated low-priority threads, not on tokio workers. As a GPU kernel cannot be preempted, a timing window is split into launch groups within a slice budget; between groups a pending production task is polled and interrupts the sample.
  - *Pruning:* axes with independent effect (`batch`, `tile`, `threads`) are measured one at a time per remaining combination, only their winners are combined and verified. A combination slower than a peer by more than a margin without being more precise is stopped early.
- **Profiles are unnamed positions.** Their number follows from the spread of the front alone: the spread threshold in the normalised arc-length scale is the only knob; neighbours closer than it merge. Points that are not faster than a more precise peer beyond the measurement uncertainty are not activated. Each profile has a position in [0, 1] derived from arc length on the normalised front (time logarithmic): 0 fastest, 1 most precise, the rest proportional to the cumulative distance. A single profile sits at 0.5. A model without any active profile is blocked for the subengine.
- **Matching.** The task's wish is only a wish. It is clamped to [0, 1] (NaN is an error) and mapped to the profile with minimal distance in the engine's own set; a tie goes to the more precise one. The scale is relative per subengine, and a wish is always satisfiable.
- **Store.** Results are kept per model × subengine as a JSON file with a fingerprint of the system (engine, GPU name, compute capability and CUDA driver version, CPU features, model ID, engine version). Equal fingerprint = calibrated, different = stale (old result stays usable while a background run replaces it), no file or unreadable file = missing (warning, no profiles). Besides the profiles the record holds the `unstable` flag, the combinations rejected with reason, and the pruned or skipped ones.
- **Measurement is an interface** with CPU, CUDA and scripted implementations; a shared synthetic workload and fp32 reference serve the real ones.

## Files

| File | Role |
|---|---|
| `mod.rs` | Overview, re-exports |
| `axis.rs` | Open axis and value identifiers |
| `capability.rs` | Capability sets |
| `known.rs` | Well-known axes and values for the target hardware |
| `probe.rs` | Probe interface, kernel intersection, probe configuration |
| `probe_cpu.rs` | CPU probe with injectable feature source |
| `probe_cuda.rs` | CUDA probe; hardware table testable without GPU |
| `measure.rs` | Measurement abstraction, validity, deviation report, clock |
| `measure_cpu.rs` | Real CPU scoring measurement |
| `measure_cuda.rs` | Real CUDA scoring measurement (event-timed, sliced) |
| `measure_mock.rs` | Scripted fake measurement |
| `workload.rs` | Synthetic scoring workload and fp32 reference |
| `metrics.rs` | Deviation metrics, plausibility checks |
| `stats.rs` | Robust estimators and spread |
| `calibrate.rs` | The calibrator: caps, stages, outcomes, report |
| `pareto.rs` | Pareto front and arc-length positions |
| `profile.rs` | Profile type, configuration, derivation from the front |
| `matching.rs` | Wish → profile selection |
| `store.rs` | Fingerprint, records, store interface and file store, background recalibration |
| `pool.rs` | Low-priority thread pool for calibration and CPU kernels |

Tests: `test/` per file; CUDA measurement and probe tests need feature `cuda`.

## Deviations
D1 Measurement ignores sequence length. Cfg.seq_lens (CalibratorConfig.seq_lens, up to max 512) is carried but not read by CpuScoringMeasure or CudaScoringMeasure. Design: test set with length variance up to configured max sequence length (processability: memory, overflow).
D2 Measurement is not model-dependent. Workload is synthetic scoring with fixed dim 384 (WorkloadSpec); CalibrationTarget.model_id is only a label. Design: calibration per model x subengine, behaviour depends on dimension and sequence length.
D3 Several axes have no effect on measurement. `tile`, `fusion`, `accumulator` are not read by CPU/CUDA measures; `threads` is not read by CUDA measure. Probes offer them; calibration measures identical behaviour for all their values.
D4 Adaptive stability decision incomplete. Code only repeats rounds until ratio series is stable or cap is hit. Missing: (a) more samples per repetition when samples are internally noisy (window_ms fixed); (b) repeating the whole run when repetitions drift but samples are stable.
D5 Fingerprint lacks CPU model. Fields: engine, gpu_name, gpu_cc, cuda_driver_version, cpu_features (flags only), model_id, engine_version. Design requires CPU model.
D6 `unstable` is persisted only as one record-level bool (CalibrationRecord.unstable). Per-combination/per-profile unstable marking is not stored; no basis to select which results to remeasure.
D7 CPU measurement uses rayon pools (thread name `cpu-measure-*`) that are not lowered in priority. Only LowPriorityPool threads get nice 19. Design: low thread priority for calibration.

## Not mapped
N1 Missing state: no conservative default is built. resolve() returns record=None with a warning; nothing derives "most precise valid combination" from capabilities/kernels.
N2 No explicit `calibrate` command exists in the package or elsewhere in rag/src.
N3 No orchestration from probe -> Calibrator.run -> derive_profiles -> CalibrationRecord::from_report -> store. Nothing outside the package calls resolve, recalibrate_in_background or derive_profiles.
N4 Lazy model lock is only reachable via ProfileSelector with an empty profile list (NoActiveProfile). No path marks a subengine for a model after an Incompatible/CalibrationUnstable error at first task; PrecisionError::Incompatible/CalibrationUnstable are not returned to waiting tasks (CalibrationUnstable is only logged).
N5 Pull mechanism (claim predicates: priority, model/index affinity, model availability; no profile predicate; no calibration throughput for load distribution) is not in this package; no counterpart found.