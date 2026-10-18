Plan A, B und Q sind umgesetzt. Fehlende Punkte wurden umgesetzt. Ich habe `nvcc` mit minimalen dependencies installiert. Run the `cuda` build on a GPU runner.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

**Not done**
- The `cuda` build and the `cudarc`/`candle` duplicate check need a GPU runner with `nvcc`. The shuttle part of the queue/memory item is also not done; those tests use proptest and loom only.

Der CUDA-13-Build läuft jetzt bis zum Linken durch, scheitert dort aber an fehlenden Bibliotheken. Die Installation müssen Sie machen.

**Erledigt**
- `third_party/candle-kernels` ist eine Kopie von `candle-kernels 0.11.0`, in deren `src/compatibility.cuh` ich die doppelten `__hmax_nan`/`__hmin_nan` entfernt habe.
- Die Kopie ist in `Cargo.toml` per `[patch.crates-io]` eingebunden.
- Alle `.cu`-Dateien kompilieren mit `nvcc` 13.4, `CUDA_COMPUTE_CAP=75` und `CUDARC_CUDA_VERSION=13030`. Auch `xy-ai-rag` (lib) baut. Es gibt nur zwei Dead-Code-Warnungen in `memory/dtype.rs`: `elems_to_bytes` und `bytes_to_elems` werden nirgends benutzt.

**Offen**
- Der Linker findet `-lnvrtc`, `-lcurand`, `-lcublas` und `-lcublasLt` nicht.
- Die minimale 13.4-Installation enthält diese Bibliotheken nicht.
- In `/usr/local/cuda-13.0/lib64` liegen nur versionierte Laufzeit-Libs (`libcublas.so.13`, `libcublasLt.so.13`, `libcurand.so.10`). Dort fehlen die unversionierten `.so`-Links und `nvrtc` ganz.

Bitte installieren Sie für 13.4 die Pakete `cuda-nvrtc-dev`, `libcurand-dev` und `libcublas-dev`, zum Beispiel mit `apt install cuda-nvrtc-dev-13-4 libcurand-dev-13-4 libcublas-dev-13-4`. Die genauen Paketnamen hängen von Ihrem Repo ab. Danach linke ich neu und mache den Duplikatcheck mit `cargo tree -d`.

Ich habe `third_party/` und die Änderung an `Cargo.toml` noch nicht committed.
