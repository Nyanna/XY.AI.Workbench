# precision

Capability sets, calibration, profile building, matching of a precision wish to a profile, and persistence of the results.

## Concept

- **Capabilities, not profile names.** A capability set maps open axis identifiers (operand type, accumulator, scoring type, batch, tile, threads, …) to the values a provider offers or a task requires. Axes and values are extensible per hardware without touching facade or task interface. `known.rs` lists the axes for the target hardware (GTX 1660, sm_75; bf16 deliberately absent) and also the axis for the index manager's gather-versus-segment load threshold.
- **Probe, then calibrate.** Probing is optimistic and measures nothing; its result is intersected with the kernels that exist. Unusable combinations are removed by calibration.
- **Calibrator.** Enumerates combinations (with pruning), measures them interleaved against the fp32 reference, rejects invalid ones (NaN/Inf, degenerate output, deviation above the maximum) and keeps the Pareto front over (time, deviation). Only relations count, estimates are robust (median/minimum). Measurement stops on stability or at a hard cap; then the best intermediate state is kept and marked `unstable`. Calibration never blocks the user and runs on dedicated low-priority threads, not on tokio workers.
- **Profiles are unnamed positions.** Their number follows from the spread of the front alone. Each has a position in [0, 1] derived from arc length on the normalised front: 0 fastest, 1 most precise. A single profile sits at 0.5.
- **Matching.** The task's wish is clamped to [0, 1] (NaN is an error) and mapped to the nearest profile of the engine's own set; a tie goes to the more precise one. The scale is relative per subengine, and a wish is always satisfiable.
- **Store.** Results are kept per model × subengine with a fingerprint of the system. Equal fingerprint = calibrated, different = stale (old result stays usable while a background run replaces it), no file = missing.
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
