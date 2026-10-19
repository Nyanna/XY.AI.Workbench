# Umsetzungsplan Q4 · Feature-Flags

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren (hängt an A2 Modulstruktur). Voraussetzung dafür, dass die Logik ohne GPU testbar bleibt.

## 1. Zweck

Reale CUDA-Pfade hinter ein Feature-Flag legen, sodass (a) der Default-Build **ohne** GPU/`nvcc` kompiliert und alle Unit-Tests ausführt, (b) `#[cfg]` nur an wenigen Modulgrenzen auftritt, (c) der Backend-Austausch über Traits/Factory statt `cfg`-Streuung läuft.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q4)

- **Best Practices:** Features sind **additiv** (aktivieren nur Code), Default **ohne** `cuda`. Optionale Dependencies mit `dep:`-Syntax; `#[cfg(feature = "cuda")]` nur an wenigen Stellen (Modulgrenze). (Cargo Book „Features".)
- **`candle-core` `cuda`:** aktiviert `cudarc`, `candle-kernels` u. a.; Bauen braucht CUDA-Toolkit/`nvcc` **[prüfen aktuelle Cargo.toml]**. Durchreichen:
  ```toml
  [features]
  default = []
  cuda = ["candle-core/cuda", "dep:cudarc"]
  [dependencies]
  candle-core = "…"
  cudarc = { version = "…", optional = true }
  ```
  Bei optionalem `candle-core`: Weak-Dependency `candle-core?/cuda`.
- **No-GPU-Pfad:** Trait-Objekt-/Generics-Austausch statt `cfg`-Streuung: `cfg` nur beim **Konstruieren des Backends** (eine Factory), alles andere feature-unabhängig.
- **CI:** Matrix **mit und ohne** `cuda`. No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests ausführen; `cuda`-Build mindestens per `cargo check`.

## 3. Verbindliche Konvention (Umsetzung)

- **`default = []`** (kein `cuda`). `cuda = ["candle-core/cuda", "dep:cudarc"]` (bzw. Weak-Syntax falls `candle-core` optional).
- **`#[cfg(feature = "cuda")]` nur an der Backend-Factory** und an der einen Candle-Adapter-Stelle (Schicht 4, `resident_subengine.md` §2: „Candle ist ein Adapter an genau einer Stelle"). Keine `cfg`-Streuung in Scheduler/Planner/Index-Logik.
- **Backend-Factory** konstruiert `ComputeBackend` (Q3): ohne `cuda` → CPU-Backend; mit `cuda` und verfügbarer GPU → CUDA-Backend, sonst Fallback CPU (`resident_subengine.md` §6: GPU-Initfehler → CPU). Der Rest der Engine arbeitet nur gegen das Trait.
- **Reale CUDA-abhängige Pfade hinter `cuda`:** Acquire-Load/Pinned-Staging/Arena/Streams (`resident_index.md` Schlusssatz), persistenter Kernel (H1), Graph-Capture (H4). Deren **Logik** (Mapping, Disziplin, Scheduling) bleibt feature-unabhängig und gegen Mocks testbar.
- **Invarianten bleiben feature-unabhängig** (`resident_subengine.md` §2): `Region`/`Op`/`ExecContext`-Signaturen ohne `Tensor`/Raw-Pointer — unabhängig von `cuda`.

## 4. Umsetzungsschritte

1. `Cargo.toml`: `default`/`cuda`-Features, `cudarc` optional, Passthrough an `candle-core`.
2. Backend-Factory mit dem einzigen `#[cfg(feature = "cuda")]`-Entscheidungspunkt.
3. CUDA-spezifische Module hinter `#[cfg(feature = "cuda")]`; CPU/Mock-Pendants immer gebaut.
4. CI-Matrix: Job A `--no-default-features`/default → `cargo build` + `cargo test` (ohne `nvcc`); Job B `--features cuda` → mindestens `cargo check`.
5. Doku: „Default baut GPU-frei; `cuda` nur für reale CUDA-Pfade."

## 5. Tests

- Default-Build kompiliert ohne `nvcc` und führt alle Unit-Tests aus (Q3).
- Backend-Factory liefert ohne `cuda` das CPU-Backend; mit `cuda` ohne GPU erfolgt der dokumentierte Fallback.
- Kein `#[cfg(feature = "cuda")]` außerhalb von Factory + Candle-Adapter (Lint/Grep-Check im CI).

## 6. Offene Verifikationen ([prüfen])

- Aktuelle `candle-core`-`Cargo.toml`: was das `cuda`-Feature zieht; `cudarc`-Version = die von `candle` verwendete (`resident_subengine.md` §10: sonst inkompatible Typen).

## 7. Definition of Done

- Feature-Matrix wie §3; einziger `cfg`-Entscheidungspunkt in der Factory; CI-Matrix mit/ohne `cuda` grün; No-GPU-Default führt alle Unit-Tests aus; `cfg`-Streuungs-Check besteht.

## Result

Q4 ist umgesetzt:

- **`src/resident/backend.rs`** (neu): `ComputeBackend`-Trait (feature-unabhängig) und `create_backend()` als einziger `#[cfg(feature = "cuda")]`-Entscheidungspunkt mit Fallback auf `CpuBackend`.
- **`src/resident/subengine/mod.rs`**: `CpuBackend` (immer gebaut, Default + Fallback-Ziel).
- **`src/resident/subengine/device.rs`**: `CudaBackend` als einziger Candle/CUDA-Adapterpunkt der Subengine; `try_new()` liefert bei fehlender GPU einen Fehler, den die Factory abfängt.
- **`src/resident/mock.rs`**: doppelte `ComputeBackend`-Definition entfernt, importiert jetzt die produktive Trait-Definition aus `backend.rs`; `MockBackend` bleibt unverändert.
- **`src/resident/mod.rs`**: `backend`-Modul registriert.
- **`docs/architecture/conventions.md`**: Q4-Abschnitt auf die tatsächlich implementierte Factory aktualisiert.

`Cargo.toml`-Feature-Matrix und `docs/architecture/feature_matrix.md` (CI-Jobs A–D) waren bereits aus einem vorherigen Durchlauf korrekt vorhanden.
