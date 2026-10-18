Noch offen:
- Es gibt weiterhin keine Python-Referenzwerte, weil `sentence-transformers` fehlt.
- Der Zero-Copy-Pfad über den Device-Pointer ist nicht umgesetzt.

- **f16 im Candle-BERT:** Der Candle-`BertModel` liefert für e5-small in f16 durchgehend NaN. Das passiert auf CPU und GPU, mit und ohne Maske und auch mit größerem `layer_norm_eps`. Die Ursache habe ich nicht gefunden. Der Adapter rechnet das Backbone deshalb in f32. Die Embeddings werden trotzdem im dtype des Slots gespeichert, auf der GPU also f16. Das weicht von der Vorgabe "GPU rechnet f16" ab. Die Konstante `BACKBONE_DTYPE` steht mit Begründung im Code, das Backbone kann später wieder auf f16 umgestellt werden.
- **PTX-Version:** Mit `nvcc` 13.4 erzeugt der Build PTX, das der Treiber (CUDA 13.3) ablehnt (`CUDA_ERROR_UNSUPPORTED_PTX_VERSION`). Mit `nvcc` 12.9 läuft es. Ich habe das mit `NVCC=/usr/local/cuda-12.9/bin/nvcc` und `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64` in einem eigenen Target-Verzeichnis (`/tmp/claude-1000/target-cu130`) gebaut. Die Projektkonfiguration habe ich nicht angefasst. Wenn du im Projekt mit `cuda` testest, brauchst du entweder diese Einstellung oder einen Treiber, der CUDA 13.4 kann.
