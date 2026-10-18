I deviated from the plan in three places:
- **Time:** it is passed in as `now_ms: u64`, as in the existing `processor::ResourceSlot`. `Instant` and the `Clock` trait don't fit, because `Clock` only exists under `cfg(test)` or the `mock` feature. The eviction idle clock starts on the first `evict_idle` call after the last lease ends, so dropping a lease needs no clock.
- **`OnceCell`:** eviction replaces the whole `Arc<OnceCell>` inside an `RwLock`, because a `OnceCell` can't be reset in place. Taking a lease and evicting are therefore mutually exclusive, which also means the `Evicting` state is only briefly visible.
- **`Subengine` trait:** it has no `device()`, `resource()` or `run_ops()` yet, since D2 and the op types aren't wired in. There is also no `Subengine` implementation yet, only the trait.

D2 ist umgesetzt, aber ich konnte weder `cargo build` noch `cargo test` ausführen. Geprüft ist nur die Syntax der geänderten Dateien. Die Gates G-STREAM und G-PTR sind nicht durchlaufen, weil dafür eine GPU nötig ist.

- Der bestehende Trait `MemoryManager` kennt keine Arena-Klasse. Deshalb gibt es daneben `DeviceMemory` mit `alloc_in`, `release_after` und `budget`. `MemoryManager` habe ich nicht geändert.
- `StagingRing` ist generisch und ohne `cuda` testbar. Nur die pinned Variante (`PinnedStaging`) liegt hinter `cuda`.
- `copy_d2d` verlangt, dass Quelle und Ziel in verschiedenen Arena-Klassen liegen.



Noch offen:
- Es gibt weiterhin keine Python-Referenzwerte, weil `sentence-transformers` fehlt.
- Der Zero-Copy-Pfad über den Device-Pointer ist nicht umgesetzt.

- **f16 im Candle-BERT:** Der Candle-`BertModel` liefert für e5-small in f16 durchgehend NaN. Das passiert auf CPU und GPU, mit und ohne Maske und auch mit größerem `layer_norm_eps`. Die Ursache habe ich nicht gefunden. Der Adapter rechnet das Backbone deshalb in f32. Die Embeddings werden trotzdem im dtype des Slots gespeichert, auf der GPU also f16. Das weicht von der Vorgabe "GPU rechnet f16" ab. Die Konstante `BACKBONE_DTYPE` steht mit Begründung im Code, das Backbone kann später wieder auf f16 umgestellt werden.
- **PTX-Version:** Mit `nvcc` 13.4 erzeugt der Build PTX, das der Treiber (CUDA 13.3) ablehnt (`CUDA_ERROR_UNSUPPORTED_PTX_VERSION`). Mit `nvcc` 12.9 läuft es. Ich habe das mit `NVCC=/usr/local/cuda-12.9/bin/nvcc` und `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64` in einem eigenen Target-Verzeichnis (`/tmp/claude-1000/target-cu130`) gebaut. Die Projektkonfiguration habe ich nicht angefasst. Wenn du im Projekt mit `cuda` testest, brauchst du entweder diese Einstellung oder einen Treiber, der CUDA 13.4 kann.
