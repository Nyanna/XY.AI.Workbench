Setze D4 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert

# D4 — Modell-Ressourcen (Encoder)

**Zweck:** `ModelSpec`+Registry, `LoadedEncoder` (Candle-Adapter als **einzige** Candle-Stelle), `ProjectionHead`, L2-Kernel, `encode_into`/`WriteSlotOp`, Batching per Token-Budget/Bucketing.

**Einordnung:** Abhängt von D2 (Lane), D3 (Weights-/Workspace-Arena, Staging). Downstream: D6 (Scorer liest Slot-Embeddings), D7 (Encoder schreibt in Slots), G1 (konkrete e5-Config). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §3, §4.3; `resident.md` (Modelle, Stack); `resident_first.md` (e5-small); `context/context_d.md` D4 (#14–#18), #32/#33/#34.

---

## Festgelegte Entscheidungen

- **Modell jetzt:** dense **e5-small** (G1), `BertModel` (candle-transformers), **nicht** `xlm_roberta`. ColBERT = Wechselpfad.
- **Candle = Adapter an genau einer Stelle (§2):** nur in `model/candle_adapter.rs`. Kein `Tensor` in öffentlichen Signaturen; Ein-/Ausgabe über `Region`+`Layout`.
- **Namen (`[U]`, #14):** e5-small speichert BERT-Schlüssel ohne `bert.`-Präfix (`embeddings.*`, `encoder.layer.N.*`). `BertModel::load` hat Fallback über `config.model_type` → meist kein Mapping nötig. Prüfen: `config.json` (`model_type:"bert"`, `vocab_size≈250037`, `hidden_size 384`, 12 Layer); Pooler-Gewichte werden **nicht** gebraucht.
- **safetensors (`[V/U]`, #15):** HF-Repo enthält `model.safetensors`. Falls nicht: `save_pretrained(safe_serialization=True)` oder Candle `VarBuilder::from_pth`. mmap, Zero-Copy auf Rohbytes, pro Engine in deren Weights-Arena.
- **f16-Laden (#13):** `VarBuilder::from_mmaped_safetensors(&[..], DType::F16, &dev)`; Attention-Maske f16-sicher (**−1e4**).
- **Projektionskopf (`[U]`, #16/#17):** candle-transformers hat **kein ColBERT-Textmodul** → Backbone + eigener `candle_nn::Linear` aus `VarBuilder`, danach L2 pro Token. Kopf-Ort je Checkpoint: `jina-colbert-v2` `linear.weight` (1024→128, ohne Bias) im Hauptcheckpoint; `answerai-colbert-small-v1` separates `1_Dense/` (384→96). Tensor-Namen per `safetensors`-Header **vorab lesen**. Für e5: **kein** Projektionskopf (384-Dim direkt), stattdessen Mean-Pooling+L2.
- **Pooling/L2 (#33, e5):** Mean-Pooling über `attention_mask`, dann L2. L2 als **eigener Kernel am Encoder-Ende** (einmal beim Schreiben, nicht im Scorer). Query und Doc gleich behandeln. Referenzwerte aus Python (`sentence-transformers`) als Fixtures; Erwartung f32 Cosinus ≥ 0,9999, f16 ≈ 1e-3.
- **Präfix (#34):** `query: `/`passage: ` Pflicht (Model Card warnt vor Qualitätsverlust); für symmetrische Aufgaben `query: ` beidseitig. Präfix ist **Anwendungslogik vor `encode`** (D5), nicht Tokenizer-Funktion.
- **`encode_into` (§4.3):** Phase 1 rechnet Candle; dünner Kernel `WriteSlotOp` liest Device-Pointer des Candle-Tensors (G-PTR) und schreibt **normiert + dtype-konvertiert** in den Slot. Tensor bleibt bis zum End-Event am Leben. **Fallback bei negativem G-WRAP:** D2D-Copy in Slot statt Zero-Copy.
- **Batching (`[V]`, #18):** nach Länge sortieren, Buckets (32/64/128/256/512), Batch bis `batch*max_len ≤ Token-Budget`, Padding mit `pad_token_id` (XLM-R: 1), Maske 0/1, Zusammenbau `Tensor::stack`/`from_vec`.

---

## Zu erstellende Artefakte (`rag/src/model/`)

- `mod.rs`
  - `struct ModelSpec { arch_id, dim, pooling, prefixes, dtype, max_seq_len, head: Option<HeadSpec>, paths }` (reine Daten, besitzt nichts).
  - `struct ModelRegistry` — `"bert" → BertEncoder`, `"maxsim" → MaxSimScorer` (Scorer in D6). Neues Modell = Registry-Eintrag.
  - `trait Encoder`: `encode_into(tokens: Region, out: Region, lane: &dyn ExecContext) -> Result<EncodeEvent, ModelError>`; `dim()`, `dtype()`.
  - `struct LoadedEncoder` — Architektur (Code) + Gewichte (Region in Weights-Arena), entladbar (D1-Lease).
  - `struct ProjectionHead { linear }` (optional; nur ColBERT-Pfad).
- `candle_adapter.rs` (hinter `cuda` für GPU-Pfad; CPU-Pfad `candle-core` CPU)
  - Backbone-Load (`BertModel::load` + `VarBuilder::from_mmaped_safetensors`), Forward, Pointer-Zugriff für `WriteSlotOp`. **Einzige** Stelle mit `candle_core::Tensor`.
- `pooling.rs`: Mean-Pooling über Maske; L2-Kernel (CPU + CUDA via D6-Registry).
- `batch.rs`: Token-Budget-Batcher, Längen-Bucketing, Padding/Maske.

---

## Prototyp-Gates (zuerst)

- **G-CKPT:** `safetensors`-Header lesen; e5 `model.safetensors`+`config.json` vorhanden/kompatibel; (für ColBERT-Pfad) Kopf-Ort/Format.
- **G-PTR** (aus D2) als Voraussetzung für `WriteSlotOp`.

---

## Arbeitsschritte

1. `ModelSpec`/`ModelRegistry` + `Encoder`-Trait definieren (GPU-frei).
2. `candle_adapter`: e5 `BertModel` laden (f16 GPU / f32 CPU), Config-Checks (model_type/vocab/hidden/layers), Pooler ignorieren.
3. Mean-Pooling+L2 implementieren; Referenz-Fixtures aus `sentence-transformers` ablegen; Cosinus-Toleranz prüfen.
4. `encode_into` + `WriteSlotOp` (normiert + dtype-konvertiert in Slot); Tensor-Lebensdauer bis End-Event; Fallback D2D falls G-WRAP negativ.
5. Batching per Token-Budget/Bucketing; Präfix-Hook (von D5 geliefert).
6. (Vorbereitet, nicht aktiv) `ProjectionHead` + per-Token-L2 für ColBERT-Wechselpfad; Namen per safetensors-Header.

## Fehler (Q1) & Logging (Q2)

- `enum ModelError { Load{source}, WeightNameMismatch{expected}, UnsupportedArch, Incompatible{..}, EncodeFailed{source} }`.
- Span pro Modell (`model_id`, `dtype`, `batch_tokens`); Encode-Events `loaded`, `batch`, `write_slot`. Große Argumente nie loggen (`skip_all`).

## Feature-Flags (Q4)

- GPU-Forward + `WriteSlotOp`-CUDA hinter `cuda`; CPU-Forward über candle-CPU immer verfügbar. `Encoder`-Trait entkoppelt Logik.

## Tests (Q3)

- Golden-Tests e5: f32 vs. Python-Referenz (Cosinus ≥ 0,9999), f16-Toleranz (~1e-3).
- Batcher: Bucketing, Token-Budget-Einhaltung, Padding/Maske korrekt.
- `WriteSlotOp` gegen Mock-Slot/Mock-Lane (normiert, richtiger dtype).
- Fehlerpfade: fehlende Gewichte, Namensmismatch.

## Definition of Done

- e5-Encoder lädt und kodiert korrekt (Golden-Tests grün); `encode_into`/`WriteSlotOp` schreibt in Slots; Candle nur im Adapter; ColBERT-Pfad als Registry-/Head-Hook vorbereitet.

## Result

D4 ist als Modul `rag/src/resident/model/` umgesetzt. Die Tests laufen auf der CPU durch, aber der GPU-Pfad und die Golden-Fixtures fehlen noch. `cargo test --lib resident::` ist grün (211 Tests). Mit `--features candle` laufen die 22 Modelltests grün, darunter zwei gegen den echten e5-small-Checkpoint auf der CPU.

Offen:
- **Golden-Tests:** Es gibt keine Python-Referenzwerte, weil `sentence-transformers` hier nicht installiert ist. Stattdessen prüfen die Tests Einheitsnorm, Padding-Invarianz (Batch gegen Einzelaufruf, Cosinus über 0,9999) und die Rangfolge verwandter Texte.
- **CUDA-Pfad:** Er ist nur geschrieben, weder gebaut noch getestet, weil `nvcc` fehlt. Der f16-Pfad ist damit ungeprüft.
- **f16-Maske:** Candles `get_extended_attention_mask` multipliziert `0 * f32::MIN`. In f16 kann das `0 * -inf = NaN` ergeben, und die Funktion ist privat. Auf der GPU bitte als Erstes mit f16 prüfen. Die Alternative wäre ein Patch in `third_party`.
- **`WriteSlotOp`:** Der Zero-Copy-Pfad über den Device-Pointer (G-PTR/G-WRAP) fehlt. `write_slot` arbeitet host-seitig: Der Encoder liefert f32, dann folgen L2, dtype-Konvertierung und Schreiben über `RegionIo`. Das gilt für beide Geräte und entspricht dem Fallback aus dem Plan.
- **Gewichte:** Sie werden von Candle per mmap geladen und liegen nicht in der Weights-Arena.
- **Präfixe:** `ModelSpec` trägt sie, angewendet werden sie erst in D5.

Geändert habe ich außerdem:
- **`Cargo.toml`:** `candle-nn` ist neu. Das Feature `candle` zieht jetzt `candle-core`, `candle-nn` und `candle-transformers` mit, damit der Encoder ohne CUDA läuft. `cuda` baut darauf auf.
- **Fehler:** `ResidentError` hat die neue Variante `Model`.
- **Modulregistrierung:** `resident/mod.rs` bekommt `pub mod model;`.

Du hattest recht: CUDA ist vorhanden. `nvcc` war nur nicht im PATH, `CUDA_PATH` kommt aus `.cargo/config.toml`. Der `cuda`-Build läuft jetzt, und der Encoder erzeugt auf der GTX 1660 SUPER dieselben Embeddings wie auf der CPU (Cosinus über 0,9999).

- **Tests:** Das gilt für den neuen Test `e5_small_gpu_matches_cpu`. Die drei e5-Tests laufen mit `--features cuda` durch, die gesamte `resident::`-Suite ebenfalls (225 Tests inklusive der GPU-Tests). Der Build ohne Features bleibt grün (211 Tests).
- **f16 im Candle-BERT:** Der Candle-`BertModel` liefert für e5-small in f16 durchgehend NaN. Das passiert auf CPU und GPU, mit und ohne Maske und auch mit größerem `layer_norm_eps`. Die Ursache habe ich nicht gefunden. Der Adapter rechnet das Backbone deshalb in f32. Die Embeddings werden trotzdem im dtype des Slots gespeichert, auf der GPU also f16. Das weicht von der Vorgabe "GPU rechnet f16" ab. Die Konstante `BACKBONE_DTYPE` steht mit Begründung im Code, das Backbone kann später wieder auf f16 umgestellt werden.
- **PTX-Version:** Mit `nvcc` 13.4 erzeugt der Build PTX, das der Treiber (CUDA 13.3) ablehnt (`CUDA_ERROR_UNSUPPORTED_PTX_VERSION`). Mit `nvcc` 12.9 läuft es. Ich habe das mit `NVCC=/usr/local/cuda-12.9/bin/nvcc` und `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64` in einem eigenen Target-Verzeichnis (`/tmp/claude-1000/target-cu130`) gebaut. Die Projektkonfiguration habe ich nicht angefasst. Wenn du im Projekt mit `cuda` testest, brauchst du entweder diese Einstellung oder einen Treiber, der CUDA 13.4 kann.
- **Fehlermeldung:** Ladefehler in Candle wurden zuerst pauschal als `WeightNameMismatch` gemeldet. Jetzt heißt es nur noch so, wenn Candle wirklich einen Tensor nicht findet. Alles andere kommt als `Load` mit der echten Ursache.

Noch offen:
- Es gibt weiterhin keine Python-Referenzwerte, weil `sentence-transformers` fehlt.
- Der Zero-Copy-Pfad über den Device-Pointer ist nicht umgesetzt.

### Zusätzlich

Die Referenzwertgenerierung liegt jetzt im Repo unter `rag/scripts/gen_e5_reference.py`.

- **Aufruf:** `python3 scripts/gen_e5_reference.py [model_dir] [output_json]`. Ohne Argumente nimmt das Skript `model/multilingual-e5-small` und schreibt nach `tests/data/e5_small_reference.json`.
- **Reproduzierbarkeit:** Ich habe es neu laufen lassen. Die erzeugte JSON ist byte-identisch mit der bisherigen (`cmp`).
- **Interpreter-Hinweis:** Der Hinweis, nicht mit `-I` zu starten, steht im Docstring des Skripts. Nur so findet es `torch`.
- **f16-Check:** Das Skript gibt bei vorhandener GPU zusätzlich aus, ob PyTorch in f16 endliche Werte liefert und wie nah es an f32 liegt.
- **Noch nicht eingecheckt:** `scripts/gen_e5_reference.py`, `tests/data/` und die Änderungen an `candle_adapter.rs` sind ungetrackt oder geändert, aber nicht committet.

Die Referenzprüfung ist jetzt modellübergreifend parametrisiert und dokumentiert. Die CPU-Tests sind grün, die GPU-Tests liefen mit `--ignored` ebenfalls grün.

- **Test:** `installed_models_cpu_match_reference` und `installed_models_gpu_match_reference` laufen über alle Unterordner von `model/` mit einer `config.json`. Der Test schlägt fehl, wenn kein Modell installiert ist, wenn für ein installiertes Modell die Referenzdatei fehlt oder wenn in `spec_for` kein `ModelSpec` registriert ist. Der Hilfsfunktion `embed_with` übergibt man jetzt ein beliebiges Modell, und die Dimension kommt aus dem Spec statt fest 384.
- **Generator:** `scripts/gen_model_reference.py` ersetzt das e5-spezifische Skript. Ohne Argumente verarbeitet es alle Modelle unter `model/`. Mean- oder CLS-Pooling und die Normalisierung liest es aus den sentence-transformers-Metadaten des Modells. Prefixe stehen in der Tabelle `PREFIXES`. Die gemeinsamen Texte stehen in `scripts/reference_texts.json` und haben eine Rolle (`query` oder `passage`).
- **Ablage:** Die Referenzen liegen unter `tests/data/reference/<modell>.json`. Die e5-Datei wurde dorthin verschoben, und die Neuerzeugung ist bitgleich zur alten.
- **Dokumentation:** `model/README.md` enthält die Schritt-für-Schritt-Anleitung für neue Modelle und wann man neu generiert. Ein Absatz in `docs/architecture/conventions.md` (Q3) verweist darauf. `.gitignore` ignoriert jetzt `/model/*` bis auf `model/README.md`, damit die README getrackt wird, die Gewichte aber nicht.

- **Nicht committet:** Alles ist noch ungetrackt oder geändert. `check_d4.md` und `.project` zeigen sich in `git status` ebenfalls als geändert. Das stammt nicht von mir.
- **Fehlschlag ohne Gewichte:** Auf einer CI ohne Gewichte schlägt `installed_models_cpu_match_reference` jetzt fehl, wie von Ihnen akzeptiert. Die Gewichte müssen dort also bereitgestellt werden oder der Test wird dort ausgeschlossen.

Tasks 3 and 4 are done. `cargo test --features candle` passes (244 tests). `cargo test --features cuda -- --ignored` passes (8 ignored GPU tests, including the adapter and wrap-gate ones).

**Task 3: f16 backbone**
- **Approach:** I vendored `candle-transformers` 0.11.0 into `third_party/candle-transformers` and registered it in `[patch.crates-io]`, like `candle-kernels`. The only code change is in `bert.rs`: `get_extended_attention_mask` now uses a finite minimum per dtype (-65504 for f16) instead of `f32::MIN`.
- **Why vendoring:** I chose it over writing a BERT forward pass myself because it is a one-line change. The cost is that every candle upgrade needs a diff against the registry copy. I dropped the examples and tests from the copy and removed the test targets from its `Cargo.toml`.
- **dtype policy:** `BACKBONE_DTYPE` is replaced by `backbone_dtype(kind)`: f32 on the CPU, f16 on the GPU.
- **Accuracy:** The GPU tests now require cosine > 0.999 against the PyTorch reference. In a temporary run with the threshold raised to 0.99999, the reference test still passed. I set the threshold back to 0.999. The CPU tests keep 0.9999.

**Task 4: G-WRAP and G-PTR**
- **Gate result:** The gate had already run and was documented in `gpu_memory.md`. Wrapping an arena range as a `CudaStorage` works for reads, but dropping the wrapped tensor frees the pointer and breaks the CUDA context. The decision stands: arenas serve engine kernels only, and Candle results move into arenas with `copy_d2d`.
- **New test:** `slice_set_writes_through_a_wrapped_view_into_the_arena` checks the one item that was still unverified, in-place writes. It passes. The earlier PTX-version error no longer occurs.
- **Docs:** I replaced the "Not verified" paragraph in `gpu_memory.md` with this result.
- **Not done:** the adapter does not yet use `copy_d2d`. It still returns host `Vec<f32>` data.

I also added a "Vendored crates" section to `dependency_verification.md` with the patch and the condition for removing it. Task 6 stays open until a second model arrives.
