# Umsetzungsplan G1 — e5-small Modell & Embedding-Pipeline

> Paket **G1** aus `resident_planplan.md`. Quellen: `resident_first.md`, `context/context_g.md` (§G1). Abhängt von: **D4** (Encoder-Ressource / Candle-Adapter), **D5** (Tokenizer & Normalizer). Ergebnis: konkrete Modellkonfiguration + Verifikationscheckliste.
>
> **Legende:** ✅ verifiziert · ⚠️ zur Laufzeit prüfen · 🔌 Erweiterungspunkt.

---

## 1. Zweck

Konkrete Konfiguration des Dense-Bi-Encoders `intfloat/multilingual-e5-small` und die geteilte Embedding-Pipeline (CPU- und GPU-Subengine nutzen **dieselbe** reine Daten-Config; nur der Ressourcen-Adapter einer Stage unterscheidet CPU/GPU). Ein Vektor pro Text, Ähnlichkeit später per Cosine (G3).

---

## 2. Fixierte Fakten (✅ aus Quelle)

### 2.1 Dateien & Lizenz
- `model.safetensors` (471 MB) **direkt vorhanden** → **keine Konvertierung**. Zusätzlich `pytorch_model.bin` (ignorieren).
- `tokenizer.json` (17,1 MB), `config.json` (655 B) vorhanden.
- Lizenz MIT.

### 2.2 Architektur / `config.json`
```json
"architectures": ["BertModel"], "model_type": "bert",
"hidden_size": 384, "intermediate_size": 1536,
"num_hidden_layers": 12, "num_attention_heads": 12,
"max_position_embeddings": 512, "type_vocab_size": 2,
"vocab_size": 250037, "hidden_act": "gelu", "layer_norm_eps": 1e-12,
"pad_token_id": 0, "position_embedding_type": "absolute",
"tokenizer_class": "XLMRobertaTokenizer", "torch_dtype": "float32"
```
- Architektur = **BERT** + XLM-R-Tokenizer → **`candle_transformers::models::bert::BertModel`**, NICHT `xlm_roberta`. (Einzige candle-Stelle liegt in D4.)
- 384 Dim, 12 Layer, Vokabular 250 037, mehrsprachig inkl. Deutsch.

### 2.3 candle-Gewichtskompatibilität
- candle lädt Standard-HF-BERT-Schema: `embeddings.{word_embeddings, position_embeddings, token_type_embeddings, LayerNorm}`, `encoder.layer.{i}.attention.self.{query,key,value}`, `attention.output.{dense,LayerNorm}`, `intermediate.dense`, `output.{dense,LayerNorm}`.
- **Fallback-Präfix** `"bert."` wird automatisch probiert → kein manuelles Remapping.
- **Pooler** wird nicht geladen (candle liefert nur `last_hidden_state`); `pooler.*` wird ignoriert → Pooling machen wir selbst (Masked Mean).
- `Config` ist serde-kompatibel; unbekannte Felder (`tokenizer_class`, `torch_dtype`, `_name_or_path`) werden ignoriert.
- Position-IDs: candle nutzt `0..seq_len` (BERT-absolut korrekt, **nicht** RoBERTa-Offset). Passt, da e5-small ein BERT ist.
- `token_type_ids` = Nullen übergeben (`type_vocab_size = 2` vorhanden).
- `hidden_act: gelu` → candle `gelu_erf` (exakt, = HF). `--approximate-gelu` nur bei Bedarf.

---

## 3. Zu implementierende Artefakte

### 3.1 `ModelSpec` / Config (reine Daten) — `src/model/e5.rs`
Struktur, die D4 (Registry) konsumiert. Keine Logik, nur Werte:
```text
model_id            = "intfloat/multilingual-e5-small"
arch                = Bert
dim                 = 384
max_position        = 512
type_vocab_size     = 2
hidden_act          = GeluErf      // exakt, nicht approximate
layer_norm_eps      = 1e-12
pad_token_id        = 0
prefix_query        = "query: "    // Pflicht, inkl. Leerzeichen
prefix_passage      = "passage: "  // Pflicht, inkl. Leerzeichen
pooling             = MaskedMean
normalize           = L2
storage_dtype       = F16          // Vektor im Index
compute_dtype_cpu   = F32
compute_dtype_gpu   = F16          // ⚠️ erst nach Messung aktivieren, Default F32
max_seq_len         = 512          // Hard-Limit; praktisch dynamisches Padding
```

### 3.2 Embedding-Pipeline — `src/model/pipeline.rs`
Stage-Kette (geteilt CPU/GPU; nur Adapter pro Stage gerätespezifisch):
1. **Präfix anwenden:** `Query → "query: " + text`, `Passage → "passage: " + text`. Enum kommt aus dem Embedding-Parameterobjekt (`resident.md`: `Query | Passage`). Präfix ist **Pflicht** (⚠️ falsches/fehlendes Präfix verschlechtert Qualität deutlich).
2. **Tokenize (CPU, D5):** `tokenizers`-Crate, eine Instanz; Ausgabe `u32`-IDs + Attention-Maske; Padding `BatchLongest`; Längen-Bucketing. `token_type_ids` = Nullen.
3. **Forward (D4):** `BertModel` → `last_hidden_state` `[B, L, 384]`.
4. **Masked Mean:** `last_hidden.masked_fill(~mask, 0).sum(dim=1) / mask.sum(dim=1)` — Maske → `dtype float`, `unsqueeze(2)`. **Nenner clampen** `min = 1e-9` (⚠️ Punkt 5 Checkliste).
5. **L2-Normalisierung:** `v / sqrt(sum(v², keepdim))` + **Epsilon** gegen Zero-Vektor (candle-Beispiel hat kein Epsilon → ergänzen). Danach reicht Skalarprodukt (G3).

**Wichtig (✅):** NICHT `--include-padding-embeddings` — Mittelung inkl. Padding ist für e5 **falsch**. Immer maskiert.

---

## 4. Entscheidungen zu offenen Punkten (aus `context_g.md`)

| Punkt | Entscheidung |
|---|---|
| Padding-Token-ID (`<pad>`=1 vs. `config.pad_token_id`=0) | Mit korrekter Attention-Maske wirkungslos; Padding im Code **explizit** setzen (wie candle-Beispiel). ⚠️ nicht blind auf Tokenizer-Default verlassen. |
| max_seq_len 64–128 vs. 512 | Keine offizielle 64–128-Empfehlung gefunden. **512 als Hard-Limit**, praktisch **dynamisches Padding `BatchLongest`** statt fixer Länge. Kurze Sätze unkritisch (nur Rechenzeit). |
| Compute-dtype f16 vs. f32 | **Default Forward in f32.** f16/bf16 nur nach eigener Messung (LayerNorm/Softmax in f16 kritischer). Storage-dtype f16 ist davon getrennt. |
| Gemischte CPU/GPU-Indexierung | Erlaubt; `dtype`/Device-Klasse in Index-Metadaten festhalten (G2). Toleranz messen (§5). |
| Referenz-Verifikation | HF-Modellkarte enthält Beispielcode, aber **keine erwarteten Zahlen** im Text; MTEB zu grob. → **eigene Golden-Vektoren** erzeugen. |

---

## 5. Verifikationsschritte (verpflichtend, ⚠️)

1. **Safetensors-Header-Dump:** `SafeTensors::deserialize(mmap).names()` lesen, gegen candle-BERT-Key-Liste (§2.3) prüfen; I64-Tensor identifizieren (vermutlich `embeddings.position_ids`-Buffer — wird von `VarBuilder::from_mmaped_safetensors` nicht angefragt).
2. **Golden-Embeddings:** 5–10 Texte (DE/EN) **mit Präfix** per Python-Referenz (HF-Karte `average_pool` + `F.normalize`, `max_length=512, padding=True, truncation=True`) erzeugen. Rust-Pipeline dagegen testen, Toleranz `~1e-4` (f32).
3. **Pooling-Clamp:** `sentence-transformers Pooling.py` erneut prüfen (Abruf scheiterte 404); Nenner-Clamp `min=1e-9` bestätigen.
4. **f16-Toleranz:** am eigenen Korpus `cos(a_f32, a_f16) ≥ 0.999` und Top-k-Overlap messen; Schwelle dokumentieren.
5. **Präfix-Pfad:** Index- vs. Query-Präfix testen (asymmetrisch: query/passage; symmetrisch: `query: ` auf beiden Seiten).

---

## 6. Fehlerbehandlung (Q1) & Logging (Q2)

- **Fehler:** Gewichts-/Config-Ladefehler auf D4-Encoder-Fehlerklasse abbilden; `EmptyEmbedding` bei Zero-Vektor (Epsilon greift, aber loggen); ungültige Präfix-Konfig → Config-Fehler.
- **Logging:** Span pro Embedding-Batch mit Feldern `kind(Query|Passage)`, `batch_size`, `len_bucket`, `device`, `compute_dtype`, `truncated_count`.

---

## 7. Feature-Flags (Q4) & Teststrategie (Q3)

- **Flags:** f16-GPU-Forward + Device-Transfer hinter `cuda`. Pipeline-Logik (Präfix, Masked-Mean-Formel, L2) auf CPU/f32 ohne GPU testbar.
- **Tests:** Golden-Vektor-Vergleich (CPU/f32); Masked-Mean mit künstlicher Maske (prüft, dass Padding ignoriert wird); Zero-Vektor → kein NaN; Präfix-Auswahl über Enum; Längen-Bucketing-Determinismus.

---

## 8. Erweiterungspunkte sichern (🔌, nicht implementieren)

- `ModelSpec.arch` als Enum, damit **ColBERT**-Wechselpfad (Projektionskopf + MaxSim) und `BAAI/bge-m3` (1024 Dim, CLS-Pooling, kein Präfix, Multi-Vector) später nur Config-Wechsel + Re-Embedding sind.
- Pooling/Normalize als Enum (nicht hartkodiert), damit CLS-Pooling o. Ä. addierbar ist.
- Embedding-Parameterobjekt erweiterbar halten (`resident.md`: zukünftige Einstellungen, Cross-Encoder-Paarwertung).

---

## 9. Definition of Done

- `ModelSpec`/Config als reine Daten vorhanden, von D4 ladbar; BertModel lädt `model.safetensors` ohne Remapping (Header-Dump bestätigt).
- Pipeline erzeugt L2-normierte 384-dim-Vektoren; Golden-Vektor-Test grün (Toleranz ~1e-4, f32).
- Masked Mean ignoriert Padding nachweislich; Zero-Vektor abgefangen (Epsilon).
- f16-Toleranz gemessen und dokumentiert.
- Verifikationscheckliste §5 abgehakt; Q1/Q2/Q3/Q4 erfüllt; No-GPU-Tests grün.
