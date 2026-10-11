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
