# Gruppe G – Modell-/Index-Integration (Resident, Rust-RAG)

Stand: 10.10.2026. Legende: ✅ = in Quelle verifiziert · ⚠️ = unverifiziert (Allgemeinwissen/Schlussfolgerung, vor Nutzung prüfen).

---

## G1 – Modell `intfloat/multilingual-e5-small`

### Dateien (Frage 1)
Quelle: https://huggingface.co/intfloat/multilingual-e5-small/tree/main
- ✅ `model.safetensors` **direkt vorhanden** (471 MB), zusätzlich `pytorch_model.bin` (471 MB) → keine Konvertierung nötig.
- ✅ `tokenizer.json` (17,1 MB) und `config.json` (655 B) vorhanden; außerdem `sentencepiece.bpe.model`, `special_tokens_map.json`, `tokenizer_config.json`, `sentence_bert_config.json`, `modules.json`, `1_Pooling/`, `onnx/`, `openvino/`.
- ✅ Lizenz MIT.

### Architektur / config.json (Fragen 2–3)
Quelle: https://huggingface.co/intfloat/multilingual-e5-small/raw/main/config.json
```json
"architectures": ["BertModel"], "model_type": "bert",
"hidden_size": 384, "intermediate_size": 1536,
"num_hidden_layers": 12, "num_attention_heads": 12,
"max_position_embeddings": 512, "type_vocab_size": 2,
"vocab_size": 250037, "hidden_act": "gelu", "layer_norm_eps": 1e-12,
"pad_token_id": 0, "position_embedding_type": "absolute",
"tokenizer_class": "XLMRobertaTokenizer", "torch_dtype": "float32"
```
- ✅ 384 Dimensionen, 12 Layer, Vokabular 250 037 (~250k), 94 Sprachen laut HF-Tag (Modellkarte: „100 languages from xlm-roberta“, schwache Leistung bei Low-Resource). Deutsch wird unterstützt (MTEB-Einträge u. a. für `de`).
- ✅ Architektur = **BERT** (`BertModel`) + XLM-R-Tokenizer. In candle ist daher `candle_transformers::models::bert::BertModel` zu nutzen, **nicht** `xlm_roberta`.
- ✅ Initialisiert aus `microsoft/Multilingual-MiniLM-L12-H384`.
- ⚠️ „118M Parameter“: HF zeigt nur „0.1B params“ (✅). 118M ist die gängige Angabe (davon ~96M Embedding-Matrix 250037×384); nicht direkt in den gelesenen Quellen belegt.
- ✅ Referenz in candle: https://github.com/huggingface/candle/blob/main/candle-examples/examples/bert/main.rs (Default-Modell all-MiniLM-L6-v2, aber `--model-id` frei wählbar). Ein e5-spezifisches candle-Beispiel wurde nicht verifiziert; auf der HF-Seite ist der Space `radames/Candle-BERT-Semantic-Similarity-Wasm` als Nutzer des Modells gelistet (✅ gelistet, Code nicht geprüft).

### Gewichtskompatibilität mit candle (Fragen 4–5)
Quelle: https://github.com/huggingface/candle/blob/main/candle-transformers/src/models/bert.rs
- ✅ candle lädt: `embeddings.{word_embeddings, position_embeddings, token_type_embeddings, LayerNorm}`, `encoder.layer.{i}.attention.self.{query,key,value}`, `attention.output.{dense,LayerNorm}`, `intermediate.dense`, `output.{dense,LayerNorm}`. Das ist das Standard-HF-BERT-Schema.
- ✅ Fallback: schlägt das Laden fehl, versucht `BertModel::load` das Präfix `"{model_type}."` (hier `bert.`) → kein manuelles Remapping für Varianten mit Präfix nötig.
- ✅ **Pooler wird nicht geladen** (candle `BertModel` liefert nur `last_hidden_state`); vorhandene `pooler.*`-Gewichte werden schlicht ignoriert.
- ✅ `Config` ist serde-kompatibel mit der e5-`config.json` (`position_embedding_type` default `absolute`; unbekannte Felder wie `tokenizer_class`, `torch_dtype`, `_name_or_path` werden von serde standardmäßig ignoriert).
- ✅ Position-IDs: candle nutzt `0..seq_len` (Kommentar im Code: „TODO: Proper absolute positions?“). Für BERT-absolute korrekt; **nicht** die RoBERTa-Konvention (Offset padding_idx+1). Passt, weil e5-small ein BERT ist.
- ✅ `token_type_ids` müssen übergeben werden (Nullen), `type_vocab_size` = 2 ist im Modell vorhanden.
- ✅ `hidden_act: gelu` → candle `gelu_erf` (exakt, entspricht HF). `--approximate-gelu` nur bei Bedarf.
- ✅ HF-Seite: Tensortypen im safetensors „I64 · F32“ → neben F32-Gewichten existiert ein I64-Tensor (vermutlich `embeddings.position_ids`-Buffer, ⚠️ nicht direkt geprüft). Mit `VarBuilder::from_mmaped_safetensors` wird nur angefordert, was candle braucht; der I64-Buffer wird nicht abgefragt.
- ⚠️ Exakte Key-Liste der `model.safetensors` nicht per Header-Dump geprüft. Empfehlung: einmalig Header lesen (`safetensors::SafeTensors::deserialize(...).names()`) und gegen die oben genannte Liste testen.
- ⚠️ Padding-Token-ID: XLM-R-Tokenizer nutzt `<pad>`=1, `config.pad_token_id`=0. Mit korrekter Attention-Maske wirkungslos; nicht verifiziert, ob `tokenizer.json` Padding bereits vorkonfiguriert hat → Padding im Code explizit setzen (candle-Beispiel tut das, s. u.).

### Pipeline: Präfixe, Pooling, Normalisierung (Fragen 6–8)
Quelle: https://huggingface.co/intfloat/multilingual-e5-small (Modellkarte, ✅)
- ✅ Präfixe: `"query: "` und `"passage: "` (jeweils mit Leerzeichen), „even for non-English texts“. Für Nicht-Retrieval-Aufgaben genügt `"query: "`.
- ✅ FAQ: Ohne Präfix „performance degradation“ (keine Zahl genannt). Faustregeln: asymmetrisch (Passage-Retrieval) → query/passage; symmetrisch (Ähnlichkeit, Paraphrase) → `query: ` auf beiden Seiten; Features/Clustering → `query: `.
- ✅ Offizielle Mean-Pooling-Referenz (HF-Karte):
```python
def average_pool(last_hidden_states, attention_mask):
    last_hidden = last_hidden_states.masked_fill(~attention_mask[..., None].bool(), 0.0)
    return last_hidden.sum(dim=1) / attention_mask.sum(dim=1)[..., None]
# danach: embeddings = F.normalize(embeddings, p=2, dim=1)
# scores = (embeddings[:2] @ embeddings[2:].T) * 100
```
- ✅ Tokenisierung in der Referenz: `max_length=512, padding=True, truncation=True`.
- ✅ Rust-Referenz (candle-Beispiel, `bert/main.rs`): Padding `BatchLongest`, Maske → `to_dtype(F32).unsqueeze(2)`, `sum_mask = mask.sum(1)`, `(emb * mask).sum(1) / sum_mask`, dann `normalize_l2`: `v / sqrt(sum(v², keepdim))`. Kommentar im Code: ergibt dasselbe Ergebnis wie `sentence_transformers`.
- ✅ Wichtig für Abgleich: Beispiel mit `--include-padding-embeddings` (Mittelung inkl. Padding) ist **falsch** für e5; Standard (Maske) verwenden.
- ⚠️ sentence-transformers `Pooling.py` konnte nicht geladen werden (404). Aus Erinnerung: `sum_embeddings / torch.clamp(sum_mask, min=1e-9)`; e5-Modul-Konfig (`1_Pooling/`) ist mean-pooling – Dateiinhalt nicht geprüft.
- ✅ Nach Normalisierung genügt das Skalarprodukt (Referenz multipliziert `embeddings @ embeddings.T`).

### Sequenzlänge & Präzision (Fragen 9–11)
- ✅ `max_position_embeddings` = 512; Modellkarte: Texte werden auf max. 512 Tokens gekürzt. `tokenizer_config.json` wurde laut Commit-Historie auf `model_max_length` passend zu den Positions-Embeddings angepasst (✅ Commit-Titel „change `model_max_length` value to fit positional embeddings number“).
- ⚠️ Offizielle Empfehlung für 64–128 Tokens bei Sätzen: **nicht gefunden**. Aufgrund Training mit längeren Passagen ist Kürzen für kurze Sätze unkritisch (nur Rechenzeit/Padding); Qualität bei Truncation ab 64/128 für lange Passagen sinkt (⚠️ Schlussfolgerung). Empfehlung: 512 als Hard-Limit, praktisch dynamisches Padding (`BatchLongest`) statt fixer Länge.
- ⚠️ f16 vs. f32: Keine offiziellen Angaben gefunden. Rechnerisch: f16 hat 10 Mantissenbits (rel. Rundungsfehler ≈ 4,9·10⁻⁴ pro Komponente). Gesamtfehler im Transformer (LayerNorm/Softmax in f16 ggf. kritischer) ist höher als reine Speicherrundung. Praxiswert (⚠️ Erfahrungswert, nicht belegt): Cosine-Abweichung f16-Modell vs. f32 typisch < 1e-3, Ranking-Änderungen nur bei knappen Scores. **Empfehlung:** Modell-Forward möglichst in f32 (oder bf16/f16 nur nach eigener Messung), Speicherung der Vektoren in f16 ist davon getrennt. Toleranzschwelle für Tests: z. B. `cos(a_f32, a_f16) ≥ 0.999` und Top-k-Overlap messen – selbst kalibrieren an eigenem Korpus.
- ⚠️ Gemischte CPU/GPU-Indexierung: Embeddings verschiedener Geräte/Präzisionen im selben Index = leichte Inkonsistenz; empfohlen: `dtype`/Device-Klasse in den Index-Metadaten festhalten (siehe G2).
- Referenz-Verifikation (Frage 11): ✅ Modellkarte enthält Beispielcode (MS-MARCO-Beispiel mit 2 queries/2 passages, Scores ×100), aber die erwarteten Zahlenwerte stehen im gelesenen Text **nicht**. ✅ MTEB-Ergebnisse liegen als „Eval Results“ vor (z. B. `.eval_results`-Ordner, MTEB-Leaderboard https://huggingface.co/spaces/mteb/leaderboard). Paper: https://arxiv.org/abs/2402.05672. **Empfehlung (⚠️):** Golden-Vektoren selbst mit dem Python-Referenzcode erzeugen (5–10 Texte, DE/EN, mit Präfix) und in Rust-Test mit Toleranz (~1e-4 f32) vergleichen; MTEB-Scores sind für Pipeline-Verifikation zu grob.

---

## G2 – Index-Schema (Parquet / Arrow)

Basis: Crate `parquet` 60.0.0 (docs.rs, Stand 15.09.2026) – https://docs.rs/parquet/latest/parquet/arrow/index.html

### f16 / FixedSizeList (Frage 12)
- ✅ `parquet` hat Abhängigkeit auf `half` (f16-Typ), `crc32fast` (optional), `twox-hash`, `ring` (optional).
- ✅ Arrow-Schema-„Hint“: Der Arrow-Writer speichert das Original-Arrow-Schema in `ARROW_SCHEMA_META_KEY` (`"ARROW:schema"`), damit Typen beim Zurücklesen exakt wiederhergestellt werden; Reader fällt sonst auf Inferenz aus dem Parquet-Schema zurück.
- ⚠️ `Float16` → Parquet `FIXED_LEN_BYTE_ARRAY(2)` mit Logical Type `FLOAT16` (Parquet-Format ≥ 2.10/2023): seit arrow-rs ~v50+ unterstützt; Interop mit Readern (pyarrow ≥ ~15/16, DuckDB, Spark) variiert → **Reifegrad mittel**, nicht in den gelesenen Docs bestätigt.
- ⚠️ `FixedSizeList<Float16, 384>`: arrow-rs schreibt FixedSizeList als Parquet-List (repeated group) mit Definition/Repetition-Levels – funktional, aber Overhead und keine echte Zero-Copy-Lesbarkeit. Alternativen: (a) `FixedSizeBinary(768)` mit rohen f16-Bytes (kompakt, schneller, aber nicht „selbstbeschreibend“); (b) `List<Float16>`. Empfehlung: FixedSizeList, falls Interop wichtig; sonst FixedSizeBinary mit Schema-Metadaten (dim, dtype, Endianness). **Vor Festlegung Mini-Roundtrip-Test in der gepinnten Crate-Version schreiben.**
- ✅ Nützliche Writer-Einstellungen (`WriterPropertiesBuilder`, https://docs.rs/parquet/latest/parquet/file/properties/struct.WriterPropertiesBuilder.html): `set_column_encoding`, `set_column_dictionary_enabled(false)` für Vektorspalte (Dictionary bringt bei Floats nichts), `set_compression`/`set_column_compression` (Default UNCOMPRESSED), `set_max_row_group_row_count`, `set_statistics_enabled`, `set_offset_index_disabled(false)` (Default; ✅ Doku: Offset-Index hilft beim Zugriff per Zeilennummer, Deaktivieren verschlechtert Leseperformance), `set_data_page_row_count_limit` (Default 20 000).
- ✅ `RowNumber`-Virtual-Column existiert (experimental) für Zeilenindex beim Lesen.

### mmap-Zugriff (Frage 13)
- ⚠️ Nicht in den gelesenen Docs belegt: Der `parquet`-Reader arbeitet über `ChunkReader` (implementiert u. a. für `File` und `bytes::Bytes`). Üblicher Weg: Datei mit `memmap2` mappen und als `Bytes` (z. B. `Bytes::from_owner(mmap)`, bytes ≥ 1.9) an `ParquetRecordBatchReaderBuilder::try_new` übergeben. Parquet ist komprimiert/encodiert → **kein echtes Zero-Copy** auf Vektoren; es wird dekodiert (bei UNCOMPRESSED + PLAIN-Encoding von FixedLenByteArray ist der Decode-Aufwand gering, aber Arrow-Arrays werden dennoch allokiert/kopiert).
- Page-Fault-Verhalten (⚠️ allgemein): Footer wird zuerst gelesen, Spaltenchunks nur bei Zugriff; Random-Access über Offset-Index/`RowSelection` lädt nur nötige Pages. `madvise` (Random/WillNeed) über `memmap2::Advice`.
- ⚠️ `memmap2`-Risiko: Datei darf während Mapping nicht verkürzt/überschrieben werden (UB/SIGBUS) → Index immer als neue Datei schreiben + atomisch umbenennen (passt zur Persistenz-Vorgabe).
- Alternative für Brute-Force-Scan: Vektoren zusätzlich als flaches binäres Sidecar-Format (echtes mmap, Zero-Copy, SIMD-Dot) – Architekturentscheidung, nicht belegt.

### Checksums (Frage 14)
- ✅ `crc32fast` ist im `parquet`-Crate optional enthalten (⚠️ Zweck: Page-CRC; Verifikation nicht in gelesenen Docs).
- ⚠️ Stand der Praxis (Allgemeinwissen): CRC32 (hardwarebeschleunigt, ~GB/s, nur Fehlererkennung, nicht gegen Manipulation); xxHash3/XXH64 (sehr schnell, 64/128 Bit, nicht kryptografisch, kollisionsarm für Integrität); BLAKE3 (kryptografisch, sehr schnell, SIMD/Multithread); SHA-256 (langsamer, `ring` bereits optional im Dependency-Baum). Für „Verifizierung Zieldatei + Zeichenoffsets pro Zeile“ reicht xxh3-64/128 bzw. CRC32C; bei Sicherheitsanforderung BLAKE3. Kleine Payloads (Pfad + 2 Offsets) → Geschwindigkeit irrelevant, 64-Bit-Hash genügt.

### Schema-Versionierung (Frage 15)
- ✅ `WriterPropertiesBuilder::set_key_value_metadata(Option<Vec<KeyValue>>)`; `KeyValue { key: String, value: Option<String> }` (https://docs.rs/parquet/latest/parquet/file/metadata/struct.KeyValue.html). Speicherort: Parquet-Footer (`FileMetaData.key_value_metadata`), Lesen über `ParquetMetaData::file_metadata().key_value_metadata()`; Arrow-Schema-Metadaten (`Schema::with_metadata`) werden vom ArrowWriter ebenfalls in den Footer geschrieben (⚠️ Verhalten aus Erfahrung, Detail nicht in Docs geprüft).
- Empfohlene Schlüssel (⚠️ Vorschlag): `resident.schema_version`, `embedding.model_id` (`intfloat/multilingual-e5-small`), `embedding.model_revision` (HF-Commit-Hash), `embedding.dim` (384), `embedding.pooling` (`mean_masked`), `embedding.normalize` (`l2`), `embedding.prefix_query` / `embedding.prefix_passage`, `embedding.storage_dtype` (`f16`), `embedding.compute_dtype`, `embedding.max_seq_len`, `tokenizer.sha` (Hash von `tokenizer.json`). Beim Öffnen strikt prüfen; Mismatch → Re-Embedding (laut Projektvorgabe neue Schema-Version).

---

## G3 – Similarity / Scorer

### Cosine = Skalarprodukt (Frage 16)
- ✅ Mathematisch: für ‖a‖=‖b‖=1 gilt cos(a,b)=a·b. Referenz nutzt genau das (`embeddings @ embeddings.T`).
- Fallstricke (⚠️ Analyse): (1) f16-Speicherung zerstört exakte Norm 1 (Norm weicht um ~1e-3 ab) → Dot-Product leicht ≠ Cosine; Akkumulation in f32 ausführen (f16→f32 upcasten vor Summe). (2) Optional nach dem Upcast re-normalisieren oder Norm beim Scoring mitführen, wenn exakte Cosine nötig. (3) Zero-Vektor (leerer Text) vermeiden → Division durch 0 in `normalize_l2` (candle-Beispiel ohne Epsilon, ✅ Code gelesen; Epsilon ergänzen).
- ✅ Modellkarte FAQ: Scores clustern bei e5 zwischen ~0,7 und 1,0 (InfoNCE-Temperatur 0,01); „es zählt die Rangfolge, nicht der Absolutwert“. → **Absolute Schwellwerte sind schlecht übertragbar**; relative Maße verwenden.

### Entscheidungsgrundlage Top-k / Rerank-Steuerung (Frage 17)
Keine standardisierte „Rerank-Trigger“-Norm gefunden (⚠️). Etablierte, einfache Kennzahlen:
- **Margin/Gap**: `s1 − s2` und `s1 − sk` (kleiner Gap → Reihenfolge unsicher → Rerank sinnvoll).
- **Streuung**: Standardabweichung/IQR der Top-k-Scores; **z-Score** von s1 gegen Verteilung der Top-k (oder gegen Stichprobe des Gesamtkorpus).
- **Softmax mit Temperatur** über Top-k (T≈0,01–0,05 passend zur e5-Skalierung) → Wahrscheinlichkeitsmasse von Top-1 bzw. **Entropie** der Verteilung.
- **Elbow/Knick-Erkennung** in der sortierten Scorekurve.
- Kalibrierung: Schwellen pro Modell/Korpus empirisch bestimmen (Score-Verteilungen sind modellabhängig; s. e5-Hinweis oben).
Das Rerank-Entscheidungslogik liegt laut Vorgabe in der äußeren Schicht; die Engine liefert nur Rohwerte (s1, sk, mean, std, Top-k-Liste).

### Cross-Encoder (Frage 18, optional)
Quelle: https://huggingface.co/BAAI/bge-reranker-v2-m3 (✅)
- ✅ `BAAI/bge-reranker-v2-m3`: Cross-Encoder (Basis `bge-m3`, XLM-R-Architektur laut Tags), multilingual, Apache-2.0, ~0,6B Parameter, F32-Safetensors. Weitere: `bge-reranker-base/large` (CN/EN), `-v2-gemma`, `-v2-minicpm-layerwise` (LLM-basiert).
- ✅ Unterschied zum Bi-Encoder: Query und Passage werden **gemeinsam** als Paar in das Modell gegeben, Ausgabe ist **ein Relevanz-Logit pro Paar** (kein Embedding, kein Cosine); höher = relevanter. Optional `sigmoid` → [0,1] (`normalize=True`). HF-Beispiel: `AutoModelForSequenceClassification`, `tokenizer(pairs, padding=True, truncation=True, max_length=512)`, `logits.view(-1).float()`.
- Konsequenz: Kosten O(Paare) pro Query (kein Vorab-Index), nur auf kleinem Top-k-Subset sinnvoll; Scores nicht mit Bi-Encoder-Cosine vergleichbar. Die Einbindung in candle (XLM-R-Sequence-Classification) wurde nicht geprüft (⚠️).

---

## Offene Punkte / Empfohlene Verifikation vor Implementierung
1. Safetensors-Header von e5-small auslesen (Key-Namen, I64-Tensor).
2. Roundtrip-Test `FixedSizeList<Float16,384>` in der gepinnten parquet/arrow-Version (Write → Read, evtl. pyarrow/DuckDB-Interop).
3. Golden-Embeddings per Python-Referenz erzeugen; Rust-Pipeline (Präfix → Tokenizer → BertModel → maskierter Mean → L2) dagegen testen.
4. f16-Toleranz am eigenen Korpus messen (Cosine-Abweichung, Top-k-Overlap).
5. sentence-transformers `Pooling.py` bei Bedarf erneut prüfen (Abruf scheiterte: 404).
6. Parquet-mmap-Weg (`Bytes::from_owner`/`memmap2`) praktisch prüfen.