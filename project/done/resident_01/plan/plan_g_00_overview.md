# Umsetzungsplan Gruppe G — Konkrete Modell-/Index-Integration (Übersicht)

> **Stufe 2 (Umsetzungsplanung).** Dieser Plan konkretisiert Gruppe G aus `resident_planplan.md`. Die offenen Punkte wurden in `context/context_g.md` recherchiert (Stand 10.10.2026) und sind hier in ausführbare Schritte, Entscheidungen und Verifikationen überführt.
>
> **Zielprojekt:** Rust-RAG unter `/home/user/xyan/xy.ai.workbench/rag`.
>
> **Legende:** ✅ = in Quelle verifiziert · ⚠️ = vor Nutzung zur Laufzeit/am eigenen Korpus zu prüfen · 🔌 = Erweiterungspunkt sichern (keine Detailplanung).

---

## 1. Zweck & Einordnung

Gruppe G liefert die **einzige modellgebundene Schicht** der Engine: die konkrete Konfiguration für das Dense-Bi-Encoder-Modell `intfloat/multilingual-e5-small`, das zugehörige Parquet-Index-Schema und den Cosine-Scorer. Alles Generische (Encoder-Adapter D4, Tokenizer D5, Index-Trait B4, Planner C3) wird **vorausgesetzt**; G füllt diese Verträge mit konkreten Werten.

**Scope-Grenze (fix, aus Abgleich in `resident_planplan.md`):**
- **Jetzt (Kern):** Dense e5-small, Masked-Mean-Pooling + L2, Cosine-Similarity über Parquet-Index.
- **Nur Erweiterungspunkt (🔌):** ColBERT/MaxSim-Wechselpfad und Cross-Encoder/Reranker. Laut `resident_first.md` ausdrücklich **„für die aktuelle Umsetzung nicht vorgesehen"** → gehört zu Gruppe H (H3). G darf diese Pfade **nicht verbauen**, aber nicht ausimplementieren.

**`resident_index.md`** ist die Design-Ergänzung zur **Implementierung des Vektorindex** (Index-Trait, Manager, Scope, Synchronisation) — keine Dateiliste. G2/G3 bauen auf dessen Index-Vertrag (B4) auf.

---

## 2. Pakete & Sequenz

| Paket | Inhalt | Plan-Datei | Abhängt von |
|---|---|---|---|
| **G1** | e5-small Modellkonfig + Embedding-Pipeline (Präfix→Tokenize→BertModel→Masked-Mean→L2) | `plan_g1_model_pipeline.md` | D4 (Encoder), D5 (Tokenizer/Normalizer) |
| **G2** | Parquet/Arrow-Index-Schema, Payload, Checksums, Schema-Versionierung, mmap | `plan_g2_index_schema.md` | B4 (Index-Trait), G1 |
| **G3** | Cosine-Scorer, Scope-Subset, top_k, Entscheidungsgrundlagen für die Oberschicht | `plan_g3_similarity_scorer.md` | G1, G2, C3 (Planner) |

**Reihenfolge:** `G1 → G2 → G3`. G1 fixiert Dim/Pooling/Präfix/dtype; G2 schreibt genau diese als Schema-Metadaten; G3 setzt normierte Vektoren voraus.

---

## 3. Gemeinsame, bereits getroffene Entscheidungen (aus `context_g.md`)

Diese Werte sind für alle drei Pakete verbindlich und müssen **konsistent** über Modellkonfig, Index-Header und Scorer geführt werden:

| Eigenschaft | Wert | Status |
|---|---|---|
| Modell-ID | `intfloat/multilingual-e5-small` | ✅ |
| Architektur | BERT → `candle_transformers::models::bert::BertModel` (nicht `xlm_roberta`) | ✅ |
| Dimension | 384 | ✅ |
| Pooling | Masked Mean: `Σ(h·mask)/Σ(mask)` | ✅ |
| Normalisierung | L2 (danach reicht Skalarprodukt) | ✅ |
| Präfixe | `"query: "` / `"passage: "` (Pflicht, inkl. Leerzeichen) | ✅ |
| max_seq_len | Hard-Limit 512; praktisch dynamisches Padding (`BatchLongest`) | ✅ |
| Compute-dtype | GPU f16 (nach Messung), CPU f32 — Forward bevorzugt f32 | ⚠️ |
| Storage-dtype (Vektor im Index) | f16 | ✅ |
| Gewichtsdatei | `model.safetensors` (471 MB) direkt vorhanden, keine Konvertierung | ✅ |

**Konsistenzregel:** Compute-/Storage-dtype und Device-Klasse werden in den Index-Metadaten (G2) festgehalten; gemischte CPU/GPU-Indexierung ist erlaubt, muss aber dokumentiert sein (⚠️ leichte Inkonsistenz, Toleranz in G1 messen).

---

## 4. Querschnitt (Q1–Q4) für Gruppe G

Hier -> `/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_00_overview.md`

### Q1 · Fehler-Taxonomie (`thiserror`)
Neue/berührte Varianten, die G beisteuert (in die projektweite Taxonomie einhängen):
- `IncompatibleSchema { expected, found }` — Index-Header passt nicht zur aktiven Modellkonfig (G2).
- `ChecksumMismatch { entry }` — Payload-Verifizierung fehlgeschlagen (G2).
- `UnsupportedScope` (bereits aus E2) — für G3 beim Scope-Subset wiederverwenden, **kein** stilles Ausweichen auf `All`.
- `InvalidPrecision` (aus F4) — bei ungültigem `precision`-Wert.
- `EmptyEmbedding` / Zero-Vektor — in G1/G3 abfangen (Epsilon in L2), kein Division-durch-0.
- Modell-/Gewichts-Ladefehler → auf die gemeinsame Encoder-Fehlerklasse aus D4 abbilden, nicht neu erfinden.

### Q2 · Logging (`tracing`)
- **G1:** Span pro Embedding-Batch; Felder: Tasktyp (Query/Passage), Batch-Größe, Token-Längen-Bucket, Device, Compute-dtype, Trunkierungen.
- **G2:** Span pro Index-Öffnen/Schreiben; Felder: Schema-Version, Modell-ID/Revision, Row-Group-Zahl, Checksum-Resultat, mmap vs. File.
- **G3:** Span pro Similarity-Task; Felder: Scope-Typ, Kandidatenzahl, top_k, s1, sk, mean, std, Scorer.

### Q3 · Teststrategie & Mocks
- **Golden-Vektoren (G1):** 5–10 Texte DE/EN mit Präfix per Python-Referenz (`sentence-transformers`/HF-Karte) erzeugen; Rust-Pipeline mit Toleranz `~1e-4` (f32) vergleichen. ⚠️ MTEB-Scores sind zu grob.
- **Roundtrip (G2):** `FixedSizeList<Float16,384>` Write→Read in der **gepinnten** `parquet`/`arrow`-Version; optional pyarrow/DuckDB-Interop.
- **f16-Toleranz (G1/G3):** `cos(a_f32, a_f16) ≥ 0.999` und Top-k-Overlap am eigenen Korpus.
- **Scorer (G3):** Cosine-Rechnung gegen normierte Golden-Vektoren; Zero-Vektor-Fall; Entscheidungsgrundlagen (gap/std) auf deterministischem Mini-Korpus.
- **No-GPU-Pfad:** alle Logik-Tests laufen ohne CUDA (Compute-dtype f32 auf CPU).

### Q4 · Feature-Flags
- Reale CUDA-Pfade (f16-Forward auf GPU, Device-Transfer) hinter `cuda`-Feature. Die Pipeline-Logik (Präfix, Pooling-Formel, L2, Cosine, Schema-I/O) ist **ohne GPU** testbar (Compute-dtype f32 auf CPU als Default im Test).

---

## 5. Verifikationsergebnisse

Alle sechs Punkte aus `context_g.md` („Offene Punkte") sind geklärt (Stand 25.10.2026; Umgebung: `parquet`/`arrow` 60.0.0, `candle` 0.11.0, `bytes` 1.12.1, GTX 1660 SUPER, CUDA 13.3). Einzige offene Restprüfung: pyarrow/DuckDB-Interop der geschriebenen Dateien (optional, nicht Teil der Kernentscheidung).

| # | Paket | Ergebnis |
|---|---|---|
| 1 | G1 | ✅ `model.safetensors`: 200 Tensoren, 199× F32 + 1× I64 (`embeddings.position_ids`). Schlüssel ohne Präfix (`embeddings.*`, `encoder.layer.N.*`, `pooler.*`), keine `gamma`/`beta`-Namen → passt zum candle-BERT-Schema (`BertModel::load` direkt auf dem Root-`VarBuilder`). Der I64-Buffer wird von candle nicht angefordert; Positions-IDs werden intern von 0 aus erzeugt (korrekt für `bert`, kein XLM-R-Offset). `word_embeddings` = [250037, 384]. |
| 2 | G2 | ✅ `FixedSizeList<Float16,384>` Write→Read ist **bit-exakt** (4000 Zeilen, 4 Row-Groups, auch mit zusätzlicher `UInt64`-Stampspalte). Arrow-Typ wird über `ARROW:schema` exakt zurückgewonnen. Interop (pyarrow/DuckDB) nicht geprüft. |
| 3 | G1 | ✅ Golden per PyTorch (f32, CPU, `transformers` + Masked-Mean + L2) erzeugt (8 Texte DE/EN/CJK/Umlaute, Kurztext, 700-Wort-Text mit Trunkierung). Rust-Pipeline (candle, f32, CPU, `BatchLongest`, Trunkierung 512) weicht um **max 1.8e-7** je Komponente ab, min-Cosine 0.9999998 → Toleranz `1e-4` ist großzügig, `1e-5` wäre haltbar. Tokenizer-Pad-ID ist 1 (nicht `pad_token_id` 0 aus `config.json`); irrelevant, solange die Attention-Mask übergeben wird. |
| 4 | G1/G3 | ✅ Korpus: 1500 Absätze + 20 Queries. **Storage f16** (Vektor f32→f16→renormalisiert): min-Cosine 0.9999998, Top-10-Overlap 1.0 (auch mit f16-Query). **Forward f16 auf GPU** (PyTorch): min-Cosine 0.9999996, max Komponentenfehler 1.3e-4, Top-10-Overlap 1.0. **Forward f16 in candle auf GPU** gegen f32-Golden: min-Cosine 0.9999993, max Komponentenfehler 1.8e-4. Die Zielschwelle `cos ≥ 0.999` wird um Größenordnungen unterschritten → f16-Forward auf GPU und gemischte CPU(f32)/GPU(f16)-Indexierung sind zulässig. Einschränkung: Passagen ≤ 1500 Zeichen; Verhalten bei 512-Token-Sequenzen nur über den 700-Wort-Text abgedeckt. |
| 5 | G1 | ✅ `sentence_transformers/models/Pooling.py` (4.1.0): `sum_mask = torch.clamp(sum_mask, min=1e-9)`; `sum_embeddings / sum_mask`. `1_Pooling/config.json`: nur `pooling_mode_mean_tokens=true`, `word_embedding_dimension=384`. `sentence_bert_config.json`: `max_seq_length=512`, `do_lower_case=false`. Die Rust-Variante mit `clamp(1e-9)` ist verifiziert (Punkt 3). |
| 6 | G2 | ✅ `memmap2::Mmap` → `Bytes::from_owner` (`bytes` ≥ 1.9; ohne eigene Dependency über `tokio_util::bytes` erreichbar) → `ParquetRecordBatchReaderBuilder::try_new` funktioniert; Schreiben in `*.tmp` + `rename` ebenfalls. Ein Stamp-only-Scan (`ProjectionMask` auf die Stampspalte) lädt **38 KB von 3,1 MB** (1,2 %), ein Voll-Scan 3,1 MB → die Stampspalte lässt sich ohne Vektor-Decode iterieren. |

**Folgerungen für die Pakete:**
- **G2-Header:** Eigene Header-Keys werden per `WriterProperties::set_key_value_metadata` als Footer-Keys geschrieben (`KeyValue::new(String, String)`, Pfad `parquet::file::metadata::KeyValue`; `parquet::format` existiert in 60.0.0 nicht mehr). Über `Schema::with_metadata` gesetzte Keys landen **nicht** als eigene Footer-Keys, sondern nur im `ARROW:schema`-Blob und erscheinen beim Lesen in `schema().metadata()`; für die strikte Header-Prüfung sind die Footer-Keys maßgeblich.
- **G2-Layout:** `FixedSizeList<Float16,384>` ist bestätigt; `FixedSizeBinary(768)` ist nicht nötig.
- **G1:** Config für `bert::Config` kann direkt aus `config.json` deserialisiert werden; `max_seq_length` kommt aus `sentence_bert_config.json`.

---

## 6. Modulvorschlag (konsistent mit A2-Schichtung)

> Vorschlag, final mit A2 abzustimmen. G ist bewusst klein und modellgebunden.

```
rag/src/
└── resident/impl/
    ├── model/
    │   ├── e5.rs          # G1: ModelSpec + Pipeline-Config (reine Daten)
    │   └── pipeline.rs    # G1: Präfix→Tokenize→Forward→MaskedMean→L2 (Stage-Verdrahtung)
    ├── index/
    │   ├── schema.rs      # G2: Arrow-Schema, Header-Metadaten, Versionierung
    │   ├── parquet_io.rs  # G2: Writer/Reader, mmap, Checksums
    │   └── payload.rs     # G2: Zieldatei + Zeichenoffsets + Checksum
    └── score/
        └── cosine.rs      # G3: Cosine-Scorer + Entscheidungsgrundlagen
```

Die konkrete Encoder-Adaption (einzige candle-Stelle) liegt in D4; G1 liefert nur die `ModelSpec`/Config, die D4 konsumiert.

---

## 7. Definition of Done (Gruppe G)

- G1/G2/G3-Pläne vollständig abgearbeitet, je eigene DoD erfüllt.
- Verifikationsergebnisse (§5) in den Paketplänen berücksichtigt.
- Modell-ID, Dim, Pooling, Präfixe, dtype sind an genau einer Stelle definiert (G1) und werden von G2 in den Index-Header und von G3 beim Laden geprüft (`IncompatibleSchema` bei Mismatch).
- Erweiterungspunkte für ColBERT/MaxSim und Cross-Encoder sind gesichert (🔌), aber nicht ausimplementiert.
- Q1–Q4 für G umgesetzt; No-GPU-Tests grün.
