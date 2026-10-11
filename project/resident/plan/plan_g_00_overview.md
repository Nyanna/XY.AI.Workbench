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

## 5. Gemeinsame Verifikationscheckliste (vor Abschluss von Gruppe G)

Aus `context_g.md`, Abschnitt „Offene Punkte". Jeder Punkt ist einem Paket zugeordnet und muss abgehakt werden:

1. ⚠️ Safetensors-Header von e5-small auslesen (`SafeTensors::deserialize(...).names()`), Key-Namen gegen candle-BERT-Schema prüfen, I64-Tensor identifizieren. → **G1**
2. ⚠️ Roundtrip-Test `FixedSizeList<Float16,384>` in gepinnter parquet/arrow-Version (ggf. Interop). → **G2**
3. ⚠️ Golden-Embeddings per Python-Referenz erzeugen; Rust-Pipeline dagegen testen. → **G1**
4. ⚠️ f16-Toleranz am eigenen Korpus messen (Cosine-Abweichung, Top-k-Overlap). → **G1/G3**
5. ⚠️ sentence-transformers `Pooling.py` bei Bedarf erneut prüfen (Abruf scheiterte 404) — Clamp `min=1e-9` im Nenner bestätigen. → **G1**
6. ⚠️ Parquet-mmap-Weg (`Bytes::from_owner`/`memmap2`) praktisch prüfen. → **G2**

---

## 6. Modulvorschlag (konsistent mit A2-Schichtung)

> Vorschlag, final mit A2 abzustimmen. G ist bewusst klein und modellgebunden.

```
rag/
└── src/
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
- Verifikationscheckliste (§5) vollständig abgehakt.
- Modell-ID, Dim, Pooling, Präfixe, dtype sind an genau einer Stelle definiert (G1) und werden von G2 in den Index-Header und von G3 beim Laden geprüft (`IncompatibleSchema` bei Mismatch).
- Erweiterungspunkte für ColBERT/MaxSim und Cross-Encoder sind gesichert (🔌), aber nicht ausimplementiert.
- Q1–Q4 für G umgesetzt; No-GPU-Tests grün.
