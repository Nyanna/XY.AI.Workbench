Plan A, B und Q sind umgesetzt. Fehlende Punkte wurden umgesetzt. Ich habe `nvcc` mit minimalen dependencies installiert. Run the `cuda` build on a GPU runner. Ich habe die fehlenden dependencies installiert.

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


SystemInit:  id=f52daea6-55b6-4ea9-956f-c3a43708b22e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 I'll check the CUDA setup and then run the build.
ReasoningToken: 29
Control Request:
```yaml
id: 4eac-86
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: "ls -d /usr/local/cuda* ; which nvcc; nvcc --version | tail -2; nvidia-smi -L 2>&1 | head -3; ls /usr/local/cuda/lib64 2>/dev/null | grep -E 'nvrtc|curand|cublas' | head -20; grep -n -A8 '^\\[features\\]' Cargo.toml"
  reason: "Verify nvcc, GPU availability, installed CUDA libs, and cuda feature name before building"
```
/answer 4eac-86 allow
Control Request:
```yaml
id: 4eac-87
result:
  structuredContent:
    stdout: |
      /usr/local/cuda
      /usr/local/cuda-12
      /usr/local/cuda-12.9
      /usr/local/cuda-13
      /usr/local/cuda-13.0
      /usr/local/cuda-13.4
      GPU 0: NVIDIA GeForce GTX 1660 SUPER (UUID: GPU-aed215a7-f66f-9b18-b9d4-385029f6b481)
      libcublasLt.so
      libcublasLt.so.13
      libcublasLt.so.13.8.0.4
      libcublasLt_static.a
      libcublas.so
      libcublas.so.13
      libcublas.so.13.8.0.4
      libcublas_static.a
      libcurand.so
      libcurand.so.10
      libcurand.so.10.4.4.72
      libcurand_static.a
      libnvrtc-builtins.so
      libnvrtc-builtins.so.13.4
      libnvrtc-builtins.so.13.4.92
      libnvrtc-builtins_static.a
      libnvrtc.so
      libnvrtc.so.13
      libnvrtc.so.13.4.92
      libnvrtc_static.a
      52:[features]
      53-default = []
      54-candle = ["dep:candle-core"]
      55-cuda = ["candle", "dep:candle-transformers", "dep:cudarc", "candle-core/cuda", "candle-transformers/cuda"]
      56-mock = ["dep:rand"]
      57-json = ["tracing-subscriber/json"]
      58-
      59-[dev-dependencies]
      60-proptest = "1"
    stderr: |
      bash: line 1: nvcc: command not found
```
/answer 4eac-87 allow
Text:
 Die Bibliotheken sind da, `nvcc` ist aber nicht im PATH. Ich setze die Umgebung und baue.
Control Request:
```yaml
id: 4eac-88
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: |-
    export CUDA_HOME=/usr/local/cuda-13.4 CUDA_PATH=/usr/local/cuda-13.4
    export PATH=$CUDA_HOME/bin:$PATH
    export LD_LIBRARY_PATH=$CUDA_HOME/lib64:$LD_LIBRARY_PATH
    export LIBRARY_PATH=$CUDA_HOME/lib64:$LIBRARY_PATH
    export CUDA_COMPUTE_CAP=75 CUDARC_CUDA_VERSION=13030
    nvcc --version | tail -2
    cargo build --features cuda 2>&1 | tail -30
    echo ---; cargo tree -d --features cuda 2>&1 | grep -E '^(cudarc|candle)'
  reason: Run the cuda feature build with CUDA 13.4 on the GTX 1660 SUPER and check duplicate cudarc/candle crates
```
/answer 4eac-88 allow Liegt hier: `/usr/local/cuda-13.4/bin`
Control Request:
```yaml
id: 4eac-89
result:
  structuredContent:
    exit_code: 1
    stdout: |
      Cuda compilation tools, release 13.4, V13.4.92
      Build cuda_13.4.r13.4/compiler.38855100_0
         Compiling cudarc v0.19.9
      warning: candle-kernels@0.11.0: Compiling 11 of 11 PTX kernels
      warning: candle-kernels@0.11.0: Using 8 threads for compilation
      warning: candle-kernels@0.11.0: Compiling 15 of 15 kernels
         Compiling candle-core v0.11.0
         Compiling candle-nn v0.11.0
         Compiling candle-transformers v0.11.0
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
      warning: function `elems_to_bytes` is never used
        --> src/resident/memory/dtype.rs:86:19
         |
      86 |     pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {
         |                   ^^^^^^^^^^^^^^
         |
         = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

      warning: function `bytes_to_elems` is never used
        --> src/resident/memory/dtype.rs:90:19
         |
      90 |     pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {
         |                   ^^^^^^^^^^^^^^

      warning: `xy-ai-rag` (lib) generated 2 warnings
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 24.95s
      ---
```
/answer 4eac-89 allow
Text:
 Der Build läuft durch. Der Exit-Code 1 kommt von `grep`, weil es keine Duplikate gibt. Jetzt lasse ich noch die Tests mit dem Feature auf der GPU laufen.
ReasoningToken: 79
Control Request:
```yaml
id: 4eac-8a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: |-
    export CUDA_HOME=/usr/local/cuda-13.4 CUDA_PATH=/usr/local/cuda-13.4
    export PATH=$CUDA_HOME/bin:$PATH LD_LIBRARY_PATH=$CUDA_HOME/lib64:$LD_LIBRARY_PATH LIBRARY_PATH=$CUDA_HOME/lib64
    export CUDA_COMPUTE_CAP=75 CUDARC_CUDA_VERSION=13030
    cargo test --features cuda 2>&1 | grep -E 'test result|FAILED|failed|panicked|error' | head -20
    echo ---; cargo tree -d --features cuda 2>&1 | head -20
  reason: Run tests with cuda feature on the GPU and list all duplicate crates
```
/answer 4eac-8a allow
Control Request:
```yaml
id: 4eac-8b
result:
  structuredContent:
    stdout: |
      test resident::error::tests::all_errors_are_send_sync_static ... ok
      test resident::error::tests::retryability_classification ... ok
      test resident::error::tests::transparent_wrapper_keeps_message ... ok
      test resident::error::tests::layer_errors_convert_into_resident_error ... ok
      test resident::op::tests::first_error_cancels_and_is_kept ... ok
      test result: ok. 111 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.98s
      test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
      test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
      test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
      test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
      ---
      ahash v0.7.8
      └── hashbrown v0.12.3
          └── rkyv v0.7.46
              └── xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)

      ahash v0.8.12
      ├── arrow-array v60.0.0
      │   ├── arrow v60.0.0
      │   │   └── xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
      │   ├── arrow-arith v60.0.0
      │   │   └── arrow v60.0.0 (*)
      │   ├── arrow-cast v60.0.0
      │   │   ├── arrow v60.0.0 (*)
      │   │   ├── arrow-csv v60.0.0
      │   │   │   └── arrow v60.0.0 (*)
      │   │   └── arrow-json v60.0.0
      │   │       └── arrow v60.0.0 (*)
      │   ├── arrow-cmp v60.0.0
      │   │   ├── arrow-ord v60.0.0
      │   │   │   ├── arrow v60.0.0 (*)
```
/answer 4eac-8b allow
Result Stats: id=f52daea6-55b6-4ea9-956f-c3a43708b22e, total: 8824, in: 8, out: 1882, reason: 0, read: 37077, write: 6934, start: 1792317112231, end: 1792317117057, time: 4773, total: 198478, io: 0, control: 53, sent: 0, recv: 108
The `cuda`-Build läuft auf der GTX 1660 SUPER durch, und es gibt keine doppelten `cudarc`- oder `candle`-Versionen.

- **Umgebung:** `nvcc` war nicht im PATH. Ich habe `CUDA_HOME=/usr/local/cuda-13.4` gesetzt, dazu `CUDA_COMPUTE_CAP=75` und `CUDARC_CUDA_VERSION=13030`. Damit findet der Linker `nvrtc`, `curand`, `cublas` und `cublasLt`.
- **Build:** `cargo build --features cuda` ist durchgelaufen. Es gibt nur die zwei Dead-Code-Warnungen in `memory/dtype.rs` (`elems_to_bytes`, `bytes_to_elems`).
- **Tests:** `cargo test --features cuda` ist grün, 111 Unit-Tests plus 6 weitere in den restlichen Test-Targets.
- **Duplikate:** `cargo tree -d` zeigt keine doppelten `cudarc`- oder `candle*`-Crates. Duplikate gibt es nur bei anderen Crates, zum Beispiel `ahash` 0.7 und 0.8 (0.7 über `rkyv`/`hashbrown`). Das habe ich nicht weiter geprüft.

Für künftige Builds müssen Sie diese Variablen in Ihrer Shell oder im CI setzen. Auch `/usr/local/cuda-13.4/bin` muss im PATH liegen.
Hm, gibt es eine alternative zum setzen im PATH?
Result Stats: id=f52daea6-55b6-4ea9-956f-c3a43708b22e, total: 10465, in: 10, out: 2930, reason: 277, read: 49932, write: 7525, start: 1792317188283, end: 1792317197035, time: 8690, total: 207168, io: 0, control: 62, sent: 1, recv: 171
SystemInit:  id=f52daea6-55b6-4ea9-956f-c3a43708b22e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 277
Ja, es gibt mehrere Wege. Ich habe sie nicht ausprobiert, außer dass `.cargo/` im `rag`-Verzeichnis schon existiert.

1. **`.cargo/config.toml` mit `[env]`** (projektlokal, am bequemsten)
   ```toml
   [env]
   CUDA_PATH = "/usr/local/cuda-13.4"
   CUDA_HOME = "/usr/local/cuda-13.4"
   CUDA_COMPUTE_CAP = "75"
   CUDARC_CUDA_VERSION = "13030"
   ```
   Cargo reicht diese Variablen an die Build-Skripte weiter. `candle-kernels` findet `nvcc` dann über `CUDA_PATH`, ohne dass PATH geändert werden muss. Den Pfad sollten Sie nicht ins Repo einchecken. Legen Sie die Datei deshalb besser global unter `~/.cargo/config.toml` ab oder lassen Sie sie in `.gitignore` stehen.

2. **Symlink `/usr/local/cuda` auf 13.4** (`sudo ln -sfn /usr/local/cuda-13.4 /usr/local/cuda`)
   Dort sucht der Build standardmäßig, wenn `nvcc` nicht im PATH ist. Das gilt dann systemweit, also auch für andere Projekte, die gerade 12.x nutzen.

3. **Linker und Laufzeit ohne Umgebungsvariablen**
   - Für `-lnvrtc` und die anderen Libs können Sie in `config.toml` unter `[build]` `rustflags = ["-L", "/usr/local/cuda-13.4/lib64"]` setzen.
   - Zur Laufzeit hilft eine Datei `/etc/ld.so.conf.d/cuda-13-4.conf` mit dem `lib64`-Pfad, danach `sudo ldconfig`.

Ich würde Variante 1 mit `[env]` nehmen und die Linker-Flags aus 3 dazu. Soll ich das in eine `.cargo/config.toml` schreiben und danach einen sauberen Build ohne gesetzte Variablen testen?
Variante 1 aber in der Projekt toml, es soll mit eingechecked werden
SystemInit:  id=f52daea6-55b6-4ea9-956f-c3a43708b22e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 4eac-8c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: ls -la .cargo; cat .cargo/*; git check-ignore -v .cargo/config.toml; git ls-files .cargo
  reason: Inspect existing .cargo config before editing
```
/answer 4eac-8c allow
Control Request:
```yaml
id: 4eac-8d
result:
  structuredContent:
    stdout: |
      total 12
      drwxrwxr-x  2 user user 4096 Oct 12 22:33 .
      drwxrwxr-x 12 user user 4096 Oct 18 11:40 ..
      -rw-rw-r--  1 user user  124 Oct 18 11:13 config.toml
      [build]
      target-dir = ".bin"
      rustflags = ["--check-cfg=cfg(loom)"]

      [env]
      CUDA_COMPUTE_CAP = { value = "75", force = false }
      .cargo/config.toml
```
/answer 4eac-8d allow
Control Request:
```yaml
id: 4eac-8e
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/.cargo/config.toml
    overwrite: true
    source: |
      [build]
      target-dir = ".bin"
      rustflags = ["--check-cfg=cfg(loom)", "-L", "native=/usr/local/cuda-13.4/lib64"]

      [env]
      CUDA_PATH = { value = "/usr/local/cuda-13.4", force = false }
      CUDA_HOME = { value = "/usr/local/cuda-13.4", force = false }
      CUDA_COMPUTE_CAP = { value = "75", force = false }
      CUDARC_CUDA_VERSION = { value = "13030", force = false }
  reason: Add CUDA env and linker search path to the tracked project cargo config
```
/answer 4eac-8e allow
ReasoningToken: 95
Control Request:
```yaml
id: 4eac-8f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: env -u CUDA_HOME -u CUDA_PATH -u CUDA_COMPUTE_CAP -u CUDARC_CUDA_VERSION -u LIBRARY_PATH -u RUSTFLAGS PATH=$(echo $PATH | tr ':' '\n' | grep -v cuda | paste -sd:) bash -c 'which nvcc; cargo build --features cuda 2>&1 | tail -5'
  reason: Verify the build works without any CUDA variables or nvcc in PATH
```
/answer 4eac-8f allow
Control Request:
```yaml
id: 4eac-90
result:
  structuredContent:
    stdout: |
      90 |     pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {
         |                   ^^^^^^^^^^^^^^

      warning: `xy-ai-rag` (lib) generated 2 warnings
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 2m 43s
```
/answer 4eac-90 allow
Result Stats: id=f52daea6-55b6-4ea9-956f-c3a43708b22e, total: 14620, in: 16, out: 4527, reason: 45, read: 92891, write: 10077, start: 1792317484857, end: 1792317490989, time: 6076, total: 384353, io: 0, control: 56, sent: 0, recv: 127
ReasoningToken: 45
Die Einstellungen stehen jetzt in `rag/.cargo/config.toml`, die schon eingecheckt ist. Der Build läuft damit auch ohne `nvcc` im PATH und ohne gesetzte CUDA-Variablen. Ich habe das mit `cargo build --features cuda` in einer Shell ohne diese Variablen geprüft.

- **`[env]`:** `CUDA_PATH`, `CUDA_HOME`, `CUDA_COMPUTE_CAP=75` und `CUDARC_CUDA_VERSION=13030` sind gesetzt. Alle haben `force = false`, eine Variable aus der Shell oder dem CI hat also Vorrang. Ein anderer Rechner kann den Pfad so überschreiben, ohne die Datei zu ändern.
- **`[build] rustflags`:** Dort steht jetzt zusätzlich `-L native=/usr/local/cuda-13.4/lib64` für den Linker.
- **Pfad:** Die Pfade sind fest auf `/usr/local/cuda-13.4` gesetzt. Auf Rechnern mit anderem Installationsort müssen Sie sie überschreiben. Die Linker-Flags gehen nur über `RUSTFLAGS` oder eine lokale Config, und `RUSTFLAGS` ersetzt dabei auch das `--check-cfg` für `loom`.
- **Laufzeit:** Bei den Tests brauchte ich zuvor `LD_LIBRARY_PATH`. Die Tests selbst habe ich mit der neuen Config nicht erneut laufen lassen. Falls sie die CUDA-Libs nicht finden, hilft `/etc/ld.so.conf.d/cuda-13-4.conf` mit anschließendem `ldconfig`.
