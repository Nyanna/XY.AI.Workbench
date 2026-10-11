Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe G aus `resident_planplan.md`.

* `resident_index.md` ist nur relevant für die Implementierung eines Vektorindexes und keine Indexdatei.

Projektpläne:
```json
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
Kontext Dateien für Gruppen:
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident/context",
      "entries" : [ ".:", "context_h_q.md", "context_a.md", "context_b.md", "context_c.md", "context_d.md", "context_e.md", "context_f.md", "context_g.md" ]
    } ]
  }
}
```
SystemInit:  id=f6efd126-c192-4b8c-8dba-e75ef7d17017, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich werde zunächst die relevanten Dateien lesen, um die Aufgabe zu verstehen. Ich beginne mit dem `resident_planplan.md` und der Kontextdatei für Gruppe G.
Control Request:
```yaml
id: 64b3-3
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_g.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  reason: "Lesen der Plan-of-Plans-Datei, der Kontextdatei für Gruppe G und der Hauptprojektdatei, um die Anforderungen für die Umsetzungspläne der Gruppe G zu verstehen."
```
/answer 64b3-3 allow
ReasoningToken: 188
Text:
 Ich habe den Metaplan und den Kontext für Gruppe G gelesen. Nun lese ich die primären Quelldokumente für Gruppe G (`resident_first.md`, `resident_index.md`) sowie die Dependency-Dokumente, um die referenzierten Verträge (B4, C3, D4, D5) zu verstehen.
Control Request:
```yaml
id: 64b3-4
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  reason: "Lesen der primären Quelldokumente für Gruppe G: resident_first.md (Modell-Notizen, Index, Similarity) und resident_index.md (Index-Vertrag B4, Scope, Persistenz), um die Umsetzungspläne G1-G3 fundiert zu erstellen."
```
/answer 64b3-4 allow
ReasoningToken: 782
Text:
 Ich habe nun alle relevanten Quellen gelesen:

- **Metaplan** (`resident_planplan.md`): Gruppe G = „Konkrete Modell-/Index-Integration" mit Paketen **G1** (e5-small Modell & Pipeline), **G2** (Index-Schema), **G3** (Similarity/Scorer), inkl. Abhängigkeiten (D4, D5, B4, C3) und Querschnitt Q1–Q4.
- **Kontext** (`context_g.md`): die bereits durchgeführte, verifizierte Recherche zu allen offenen Punkten von G1–G3.
- **Primärquellen** (`resident_first.md`, `resident.md`, `resident_index.md`): die Konzept-/Vertragsgrundlagen (Index-Trait B4, Scope, Persistenz-Verantwortung, Fassade, Pull-Queue).

Ich erstelle jetzt vier Umsetzungspläne im Verzeichnis `plan`: eine Übersicht plus je ein ausführbarer Agenten-Plan für G1, G2 und G3. Die `⚠️`-Punkte aus dem Kontext werden als verpflichtende Verifikationsschritte in die Pläne überführt.
Control Request:
```yaml
id: 64b3-5
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g_overview.md
    mode: replace
    content: |
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
  reason: "Schreiben der Übersichtsdatei plan_g_overview.md für Gruppe G einzeln, nachdem der kombinierte Schreibvorgang fehlschlug."
```
/answer 64b3-5 allow
Control Request:
```yaml
id: 64b3-7
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g1_model_pipeline.md
    mode: replace
    content: |
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
  reason: Schreiben des Detailplans G1 (Modell & Embedding-Pipeline) einzeln.
```
/answer 64b3-7 allow
ReasoningToken: 4180
Text:
 Der Schreibvorgang schlug fehl (vermutlich wegen der Größe des kombinierten Payloads). Ich schreibe die Dateien einzeln.
Control Request:
```yaml
id: 64b3-8
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g2_index_schema.md
    mode: replace
    content: |
      # Umsetzungsplan G2 — Index-Schema (Parquet / Arrow)

      > Paket **G2** aus `resident_planplan.md`. Quellen: `resident_first.md` (Index), `resident.md`/`resident_index.md` (Persistenz außerhalb der Engine), `context/context_g.md` (§G2). Abhängt von: **B4** (Index-Trait), **G1** (Modellkonfig). Ergebnis: Index-Schema-Spezifikation + Roundtrip-Verifikation.
      >
      > Basis-Crate: **`parquet` 60.0.0** (gepinnt). **Legende:** ✅ verifiziert · ⚠️ zur Laufzeit prüfen.

      ---

      ## 1. Zweck

      Konkretes, persistierbares Index-Schema für e5-small-Vektoren: Vektorspalte + Zeilen-/Offset-ID, Payload (Zieldatei + Zeichenoffsets + Checksum) für Rückverweis auf den Originaltext, Schema-Versionierung (Modell-ID, Dim, Pooling, Präfixe, dtype) und mmap-Lesezugriff.

      **Vertragslage (✅):** Persistenz **außerhalb** der Engine. Das **Index-Objekt besitzt den Mutex/die Persistenz selbst** (`resident.md`, `resident_index.md`). G2 implementiert ein konkretes `Index` (B4-Trait), nicht den Manager. Modellwechsel = **neue Schema-Version + Re-Embedding**.

      ---

      ## 2. Zu implementierende Artefakte

      ### 2.1 Arrow-Schema — `src/index/schema.rs`
      Spalten:
      - **`vector`** — Vektordaten, 384×f16. Layout-Entscheidung siehe §3.
      - **`entry_id`** — stabile Entry-Identität (B4 `EntryId`).
      - **`row`/`offset_id`** — Zeilen-/Offset-ID (Zugriff per Zeilennummer).
      - **Payload-Spalten** (§2.2).
      - **`stamp`** — monotoner Zähler-Wert pro Entry (B4). **Eigene Spalte**, damit der Scan **keine Vektorseiten einpaged** (harte Vorgabe `resident_index.md`). ⚠️ Prüfen, dass `stamps()` ohne Vektor-Decode iterierbar ist (ggf. separates Row-Group-Layout / separate Datei je nach Reifegrad).

      ### 2.2 Payload — `src/index/payload.rs`
      Pro Entry (für Rückverweis / späteren Cross-Encoder-Scorer):
      - `target_file: String` (Zieldatei)
      - `char_start: u64`, `char_end: u64` (Zeichenoffsets der Zeile/Chunk)
      - `checksum: u64` (Verifizierung, siehe §4)

      ### 2.3 Header / Schema-Metadaten — `src/index/schema.rs`
      In den Parquet-Footer via `WriterPropertiesBuilder::set_key_value_metadata(Vec<KeyValue>)` (`KeyValue { key, value }`). Lesen über `ParquetMetaData::file_metadata().key_value_metadata()`.

      ### 2.4 Parquet-I/O — `src/index/parquet_io.rs`
      Writer + Reader inkl. mmap, implementiert den B4-`Index`-Trait (`stamp`, `stamps`, `read`, `apply`, `changed`, `capabilities`=`[Range]` initial, usw.).

      ---

      ## 3. Vektor-Layout-Entscheidung (⚠️ vor Festlegung Roundtrip-Test!)

      Drei Optionen aus `context_g.md`:

      | Option | Pro | Contra |
      |---|---|---|
      | **`FixedSizeList<Float16, 384>`** | selbstbeschreibend, interoperabel | arrow-rs schreibt als repeated group (Def/Rep-Levels) → Overhead, **kein** echtes Zero-Copy |
      | **`FixedSizeBinary(768)`** (rohe f16-Bytes) | kompakt, schnell | nicht „selbstbeschreibend" → dim/dtype/Endianness in Metadaten nötig |
      | **`List<Float16>`** | flexibel | variabler Overhead, langsamer |

      **Entscheidungsregel:**
      - Interop wichtig (pyarrow/DuckDB/Spark) → **FixedSizeList**.
      - Sonst → **FixedSizeBinary(768)** + Schema-Metadaten (dim, dtype, Endianness).

      ⚠️ Hintergründe: `Float16` → Parquet `FIXED_LEN_BYTE_ARRAY(2)` mit Logical Type `FLOAT16` (Parquet ≥ 2.10); Reifegrad **mittel**, Interop variiert (pyarrow ≥ ~15/16, DuckDB, Spark). **Verpflichtend:** Mini-Roundtrip-Test (Write→Read) in der **gepinnten** `parquet`/`arrow`-Version, bevor das Layout final festgelegt wird.

      **Brute-Force-Scan-Alternative (Architekturoption, nicht belegt):** Vektoren zusätzlich als flaches binäres Sidecar (echtes mmap, Zero-Copy, SIMD-Dot). Nur erwägen, wenn Parquet-Decode beim Scan zum Engpass wird; als Option dokumentieren, nicht initial bauen.

      ---

      ## 4. Checksums (Payload-Verifizierung) — Frage 14

      - `crc32fast` ist optional im `parquet`-Crate (⚠️ dort Page-CRC, nicht Payload).
      - **Empfehlung:** Für „Zieldatei + Zeichenoffsets pro Zeile" genügt **xxh3-64/128** oder **CRC32C**. Kleine Payloads → Geschwindigkeit irrelevant, **64-Bit-Hash genügt**.
      - Nur bei Sicherheitsanforderung (Manipulationsschutz) → **BLAKE3** (kryptografisch, SIMD/MT) oder SHA-256 (`ring` ist bereits optional im Dependency-Baum).
      - **Entscheidung:** xxh3-64 für `payload.checksum`. Bei Mismatch → Fehler `ChecksumMismatch` (Q1), kein stiller Fallback.

      ---

      ## 5. Schema-Versionierung (Header-Keys) — Frage 15

      Footer-Metadaten (strikt beim Öffnen prüfen; Mismatch → `IncompatibleSchema` → Re-Embedding):

      | Key | Wert (aus G1) |
      |---|---|
      | `resident.schema_version` | z. B. `1` |
      | `embedding.model_id` | `intfloat/multilingual-e5-small` |
      | `embedding.model_revision` | HF-Commit-Hash |
      | `embedding.dim` | `384` |
      | `embedding.pooling` | `mean_masked` |
      | `embedding.normalize` | `l2` |
      | `embedding.prefix_query` | `query: ` |
      | `embedding.prefix_passage` | `passage: ` |
      | `embedding.storage_dtype` | `f16` |
      | `embedding.compute_dtype` | `f32`/`f16` (gemischt dokumentieren) |
      | `embedding.max_seq_len` | `512` |
      | `tokenizer.sha` | Hash von `tokenizer.json` |

      ⚠️ ArrowWriter schreibt auch `Schema::with_metadata` in den Footer (Verhalten aus Erfahrung, Detail nicht in Docs geprüft) — Roundtrip bestätigen.

      ---

      ## 6. Writer-Einstellungen (✅ `WriterPropertiesBuilder`)

      - `set_column_dictionary_enabled(false)` für die **Vektorspalte** (Dictionary bringt bei Floats nichts).
      - `set_compression` / `set_column_compression` (Default UNCOMPRESSED) — bewusst wählen; für FixedLenByteArray + PLAIN ist Decode-Aufwand gering.
      - `set_max_row_group_row_count` — Row-Group-Größe nach Zugriffsmuster.
      - `set_statistics_enabled` — für Range/Pruning.
      - `set_offset_index_disabled(false)` (Default; Offset-Index hilft Zugriff per Zeilennummer — **nicht** deaktivieren).
      - `set_data_page_row_count_limit` (Default 20 000).
      - `RowNumber`-Virtual-Column (experimental) für Zeilenindex beim Lesen erwägen.
      - `ARROW:schema`-Meta (`ARROW_SCHEMA_META_KEY`) stellt Arrow-Typen beim Rücklesen exakt wieder her (Reader fällt sonst auf Inferenz zurück) — sicherstellen, dass ArrowWriter es schreibt.

      ---

      ## 7. mmap-Zugriff — Frage 13 (⚠️ praktisch prüfen)

      - Reader arbeitet über `ChunkReader` (impl. für `File` und `bytes::Bytes`).
      - **Weg:** Datei mit `memmap2` mappen → als `Bytes` (`Bytes::from_owner(mmap)`, `bytes ≥ 1.9`) an `ParquetRecordBatchReaderBuilder::try_new`.
      - **Kein echtes Zero-Copy** auf Vektoren: Parquet dekodiert, Arrow-Arrays werden allokiert/kopiert (bei UNCOMPRESSED + PLAIN FixedLenByteArray gering).
      - Random-Access: Footer zuerst, Spaltenchunks nur bei Zugriff; `RowSelection`/Offset-Index lädt nur nötige Pages. `madvise(Random/WillNeed)` über `memmap2::Advice`.
      - **⚠️ memmap2-Risiko:** Datei während Mapping **nicht** kürzen/überschreiben (UB/SIGBUS) → **Index immer als neue Datei schreiben + atomisch umbenennen** (passt zur Persistenz-Vorgabe; sichert auch CoW-Disziplin des Managers).

      ---

      ## 8. Fehlerbehandlung (Q1) & Logging (Q2)

      - **Fehler:** `IncompatibleSchema { expected, found }` (Header-Mismatch), `ChecksumMismatch { entry }`, Lade-/I-O-Fehler auf B4/`IndexUnavailable` abbilden. Kein stilles Ausweichen.
      - **Logging:** Span pro Öffnen/Schreiben mit `schema_version`, `model_id`, `model_revision`, `row_groups`, `checksum_ok`, `access=mmap|file`.

      ---

      ## 9. Teststrategie (Q3) & Verifikation (⚠️)

      1. **Roundtrip** `FixedSizeList<Float16,384>` (bzw. FixedSizeBinary) Write→Read in gepinnter Version; Werte bit-/toleranzgleich. Optional pyarrow/DuckDB-Interop.
      2. **Header-Roundtrip:** alle Keys (§5) zurücklesbar; Mismatch löst `IncompatibleSchema` aus.
      3. **Payload/Checksum:** xxh3-64 Roundtrip; manipulierter Payload → `ChecksumMismatch`.
      4. **Stamp-Spalte:** `stamps()` iterierbar **ohne** Vektor-Decode (keine Vektorseiten eingepaged).
      5. **mmap-Weg:** `Bytes::from_owner(mmap)` praktisch; atomisches Rename beim Schreiben.
      6. **B4-Vertrag:** `apply` gibt eigenen Stamp zurück; Schreibreihenfolge Daten→Stamp→Signal; `changed()`-Watch feuert.

      ---

      ## 10. Definition of Done

      - Vektor-Layout per Roundtrip-Test final entschieden und dokumentiert (FixedSizeList **oder** FixedSizeBinary + Metadaten).
      - `Index`-Trait (B4) für den Parquet-Index implementiert; `stamps()` ohne Vektor-Decode.
      - Payload (Zieldatei + Zeichenoffsets + xxh3-Checksum) schreib-/prüfbar.
      - Header-Keys (§5) aus G1 übernommen, strikt geprüft; Mismatch → `IncompatibleSchema`.
      - mmap-Lesepfad praktisch verifiziert; Schreiben via neue Datei + atomisches Rename.
      - Q1/Q2/Q3 erfüllt; No-GPU-Tests grün.
  reason: Schreiben des Detailplans G2 (Index-Schema Parquet/Arrow) einzeln.
```
/answer 64b3-8 allow
ReasoningToken: 327
Text:
 Übersicht geschrieben. Jetzt die drei Detailpläne G1, G2, G3.
Control Request:
```yaml
id: 64b3-9
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_g3_similarity_scorer.md
    mode: replace
    content: |
      # Umsetzungsplan G3 — Similarity / Cosine-Scorer

      > Paket **G3** aus `resident_planplan.md`. Quellen: `resident_first.md` (Similarity), `context/context_g.md` (§G3). Abhängt von: **G1** (normierte Vektoren), **G2** (Index/Scope-Substrat), **C3** (Planner-Expansion). Ergebnis: Similarity-Konfiguration + Scorer.
      >
      > **Legende:** ✅ verifiziert · ⚠️ analysiert/zu messen · 🔌 Erweiterungspunkt.

      ---

      ## 1. Zweck

      Standard-Ähnlichkeitspfad: **Cosine** über L2-normierte e5-Vektoren, optional eingeschränkt auf ein **Scope-Subset**, mit `top_k` und den **Entscheidungsgrundlagen** (Anzahl, Score-Abstand, Streuung, Task-ID), aus denen die **äußere Schicht** entscheidet, ob ein Folgetask (z. B. Rerank) eingereiht wird.

      **Vertragslage (✅):**
      - Scorer ist **Parameter**: `Cosine` | `CrossEncoder(modell)`. **CrossEncoder ist für die aktuelle Umsetzung nicht vorgesehen** (`resident_first.md`) → nur 🔌 Erweiterungspunkt (Gruppe H3).
      - **Rerank ist kein eigener Tasktyp**, nur Similarity mit anderem Scorer auf einem Subset.
      - Die Engine liefert **Rohwerte + Entscheidungsgrundlagen**; die **Entscheidungslogik liegt in der äußeren Schicht** (vor der Resident-Engine). Zurückgegeben werden **Zieldatei + Offsets + Entry-IDs** wie im Index hinterlegt (G2-Payload).

      ---

      ## 2. Cosine = Skalarprodukt (✅) + Fallstricke (⚠️)

      Für `‖a‖=‖b‖=1` gilt `cos(a,b)=a·b` (Referenz: `embeddings @ embeddings.T`). Konkrete Umsetzungsregeln:

      1. **f16-Speicherung zerstört exakte Norm 1** (Abweichung ~1e-3) → Dot-Product leicht ≠ Cosine. **Akkumulation in f32:** f16→f32 **upcasten vor** der Summe.
      2. **Optional** nach Upcast re-normalisieren oder Norm beim Scoring mitführen, wenn exakte Cosine nötig.
      3. **Zero-Vektor** (leerer Text) vermeiden → sonst Division durch 0 in `normalize_l2`. **Epsilon** ergänzen (greift bereits in G1; im Scorer defensiv prüfen).

      **Score-Skala (✅):** e5-Scores clustern zwischen ~0,7 und 1,0 (InfoNCE-Temperatur 0,01). **Es zählt die Rangfolge, nicht der Absolutwert** → **absolute Schwellwerte sind schlecht übertragbar**; relative Maße verwenden.

      ---

      ## 3. Zu implementierende Artefakte

      ### 3.1 Scorer — `src/score/cosine.rs`
      - Cosine über Scope-Kandidatenmenge; f32-Akkumulation; `top_k`-Reduktion (Heap/Partial-Sort).
      - Rückgabe pro Treffer: `Hit { entry, score, outdated }` (B4-/Manager-Vertrag), angereichert um G2-Payload (Zieldatei + Zeichenoffsets + Entry-ID) auf der Ergebnis-Ebene.

      ### 3.2 Similarity-Parameterobjekt
      - `scorer: Scorer` — Enum `Cosine` | `CrossEncoder(ModelId)` (letzteres 🔌, nicht aktiv).
      - `scope: Scope` — optionales Index-Subset. Repräsentation als **kompakte Entry-Menge** (Range/Bitmap) gemäß `resident_index.md` (**nicht** als ID-Liste). Kann ID-Liste, Bereich oder **Ergebnis eines früheren Tasks** sein (Addon-Muster, kein Kern).
      - `top_k: u32`.
      - `precision: f32` (aus F4, optional) — durchreichen, Scorer selbst ist präzisionsneutral.
      - Erweiterbar halten (`resident.md`: zukünftige Einstellungen).

      ### 3.3 Entscheidungsgrundlagen (für die äußere Schicht)
      Die Engine berechnet und liefert (keine Entscheidung!):
      - `count` — Kandidatenzahl.
      - `s1`, `sk` — Top-1 und Top-k Score.
      - `mean`, `std` — über die Top-k-Scores.
      - `task_id`.
      - Top-k-Liste mit `(entry, score, payload)`.

      Optionale Kennzahlen (⚠️ keine Norm, als Diagnose anbieten; Schwellen bleiben außen):
      - **Margin/Gap:** `s1 − s2`, `s1 − sk` (kleiner Gap → Reihenfolge unsicher).
      - **Streuung:** Std/IQR der Top-k; **z-Score** von s1 gegen Top-k-Verteilung.
      - **Softmax/Temperatur** über Top-k (T≈0,01–0,05 passend zur e5-Skala) → Masse Top-1 bzw. **Entropie**.
      - **Elbow/Knick** in der sortierten Scorekurve.

      ---

      ## 4. Integration mit C3 (Planner)

      - Der **Planner** expandiert einen Similarity-Task zustandslos in eine Op-Folge; G3 liefert die **Scorer-Regel** für die Expansion (`Cosine` → Dot-Product-Op über Scope-Segment, dann Top-k-Reduktion/Merge).
      - **Rerank** entsteht nur dadurch, dass die äußere Schicht einen **zweiten Similarity-Task** mit anderem Scorer und eingeschränktem Scope einreiht — über den **normalen** Queue-Weg (Addon-Muster, `resident_index.md`). Kein neuer Tasktyp, keine Scheduler-seitige Task-Beziehung.
      - Scorer-Austausch liegt **hinter demselben Op-/Scorer-Trait** → 🔌 CrossEncoder/MaxSim später ohne API-Bruch.

      ---

      ## 5. Scope-Behandlung

      - Scope-Auflösung macht der **Manager** (E2), nicht G3. G3 bekommt eine aufgelöste Kandidatenmenge (`Variable`-Layout / Entry-Set).
      - **Kein stilles Ausweichen:** nicht deklariertes Verfahren oder fehlender Resolver → `UnsupportedScope` (Q1).
      - Ergebnisse auf entfernte Entries werden verworfen; Entries mit anstehendem Update → `outdated` (Diagnose).

      ---

      ## 6. Cross-Encoder — nur Erweiterungspunkt (🔌, NICHT implementieren)

      Für spätere Umsetzung (H3) dokumentieren, Hooks sichern:
      - Modell z. B. `BAAI/bge-reranker-v2-m3` (Cross-Encoder, XLM-R-`SequenceClassification`, multilingual, Apache-2.0, ~0,6B, F32).
      - Query+Passage **gemeinsam** als Paar → **ein Relevanz-Logit pro Paar** (kein Embedding, kein Cosine), optional `sigmoid`→[0,1]. Höher = relevanter.
      - Kosten O(Paare), kein Vorab-Index → nur auf kleinem Top-k-Subset; Scores **nicht** mit Cosine vergleichbar.
      - Lazy laden, TTL entlädt. candle-Einbindung (XLM-R-SeqClass) ⚠️ nicht geprüft.
      - **Hook:** Scorer-Enum-Variante `CrossEncoder(ModelId)` existiert im Parameterobjekt, bleibt in Phase 1 unimplementiert / liefert klaren Fehler.

      ---

      ## 7. Fehlerbehandlung (Q1) & Logging (Q2)

      - **Fehler:** `UnsupportedScope`, `EmptyEmbedding`/Zero-Vektor, `InvalidPrecision` (NaN); CrossEncoder-Variante → klarer „nicht verfügbar"-Fehler.
      - **Logging:** Span pro Similarity-Task mit `scope_kind`, `candidate_count`, `top_k`, `s1`, `sk`, `mean`, `std`, `scorer`, `discarded`/`outdated`-Zählern.

      ---

      ## 8. Teststrategie (Q3)

      - **Cosine-Korrektheit:** gegen normierte Golden-Vektoren (aus G1); f32-Akkumulation; Ergebnis = Python-Referenz `embeddings @ embeddings.T` (×100 optional).
      - **f16-Pfad:** Scoring mit f16-Storage → f32-Upcast; `cos ≥ 0.999` ggü. f32-Referenz; Top-k-Overlap.
      - **Zero-Vektor:** kein NaN/Inf.
      - **Entscheidungsgrundlagen:** gap/std/entropy auf deterministischem Mini-Korpus; Werte stabil.
      - **Scope:** Subset schränkt Kandidaten korrekt ein; `UnsupportedScope` bei fehlendem Resolver.
      - **Ergebnisstatus:** entfernt → verworfen; anstehendes Update → `outdated`.

      ---

      ## 9. Definition of Done

      - Cosine-Scorer mit f32-Akkumulation und Epsilon-Schutz; `top_k`-Reduktion.
      - Similarity-Parameterobjekt mit `scorer`/`scope`/`top_k`/`precision`, erweiterbar; Scope als kompakte Entry-Menge.
      - Engine liefert Rohwerte + Entscheidungsgrundlagen (count, s1, sk, mean, std, task_id, Top-k-Liste mit Payload); **keine** Rerank-Entscheidung in der Engine.
      - Rerank nur als zweiter Similarity-Task über die normale Queue (Addon), kein neuer Tasktyp.
      - CrossEncoder/MaxSim als 🔌 Erweiterungspunkt gesichert, nicht implementiert.
      - Q1/Q2/Q3 erfüllt; No-GPU-Tests grün.
  reason: Schreiben des Detailplans G3 (Similarity/Cosine-Scorer) einzeln.
```
/answer 64b3-9 allow
Result Stats: id=f6efd126-c192-4b8c-8dba-e75ef7d17017, total: 135319, in: 131, out: 45664, reason: 20, read: 397882, write: 89524
ReasoningToken: 20
Alle vier Umsetzungspläne für Gruppe G wurden erstellt in `/home/user/xyan/xy.ai.workbench/project/resident/plan/`:

| Datei | Inhalt |
|---|---|
| `plan_g_overview.md` | Übersicht: Scope-Grenze, Sequenz G1→G2→G3, gemeinsame fixierte Werte, Querschnitt Q1–Q4, gemeinsame Verifikationscheckliste, Modulvorschlag, DoD |
| `plan_g1_model_pipeline.md` | e5-small Modellkonfig + Embedding-Pipeline (Präfix→Tokenize→BertModel→Masked-Mean→L2) |
| `plan_g2_index_schema.md` | Parquet/Arrow-Index-Schema, Vektor-Layout, Checksums, Schema-Versionierung, mmap |
| `plan_g3_similarity_scorer.md` | Cosine-Scorer, Scope-Subset, top_k, Entscheidungsgrundlagen für die Oberschicht |

## Vorgehen & Kernentscheidungen

**Grundlage:** Ich habe den Metaplan (`resident_planplan.md`, Gruppe G + Querschnitt Q), die verifizierte Recherche (`context_g.md`) sowie die Vertragsquellen (`resident_first.md`, `resident.md`, `resident_index.md` als Vektorindex-Design, **nicht** Dateiliste) ausgewertet. Die Pläne überführen Stufe 2: die bereits recherchierten offenen Punkte wurden in ausführbare Schritte, Entscheidungen und verpflichtende Verifikationen umgesetzt.

**Wichtige in die Pläne eingegangene Festlegungen:**
- **G1:** `BertModel` (candle), nicht `xlm_roberta`; `model.safetensors` direkt nutzbar; Masked-Mean + L2 mit Epsilon; Forward default **f32** (f16 erst nach Messung), Storage f16; Golden-Vektor-Verifikation (~1e-4).
- **G2:** Vektor-Layout-Entscheidung (`FixedSizeList<f16,384>` vs. `FixedSizeBinary(768)`) **erst nach Roundtrip-Test** in gepinnter `parquet` 60.0.0; xxh3-64-Checksum; Schema-Header-Keys aus G1; mmap via `Bytes::from_owner` + atomisches Rename.
- **G3:** Cosine mit **f32-Akkumulation**; relative Kennzahlen statt absoluter Schwellen; Entscheidungslogik bleibt in der äußeren Schicht; Rerank nur als zweiter Task (Addon).

**Scope-Treue:** ColBERT/MaxSim und Cross-Encoder sind gemäß `resident_first.md` ausdrücklich „für die aktuelle Umsetzung nicht vorgesehen" → in allen Plänen nur als Erweiterungspunkt (🔌, Gruppe H3) gesichert, nicht ausimplementiert. Alle `⚠️`-Punkte aus dem Kontext sind als verpflichtende Verifikationsschritte den jeweiligen Paketen zugeordnet.

Hinweis: Mein erster kombinierter Schreibversuch (alle vier Dateien in einem Aufruf) schlug mit einem Tool-Fehler fehl; ich habe die Dateien daraufhin einzeln geschrieben – alle vier Schreibvorgänge waren erfolgreich.
Was war der Fehler?
Result Stats: id=f6efd126-c192-4b8c-8dba-e75ef7d17017, total: 137460, in: 133, out: 46513, reason: 267, read: 488764, write: 90814
SystemInit:  id=f6efd126-c192-4b8c-8dba-e75ef7d17017, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
ReasoningToken: 267
Der Fehler bei meinem ersten (kombinierten) Schreibversuch lautete wörtlich:

```
MCP error -32602: Argument 'items' must be of type array
```

**Einordnung:**
- `-32602` ist der JSON-RPC-Standardcode für **„Invalid params"** (ungültige Aufrufparameter).
- Die Meldung besagt, dass der Parameter `items` beim Server **nicht als Array** ankam.

**Wahrscheinliche Ursache:** Ich hatte zwar formal ein Array mit allen vier Dateien übergeben, aber der kombinierte Payload war sehr groß (vier vollständige Markdown-Dokumente mit viel Markup in einem einzigen `content`-Feld). Mit hoher Wahrscheinlichkeit wurde das Argument dabei nicht als wohlgeformtes Array übertragen/geparst (z. B. abgeschnitten oder nicht vollständig serialisiert), sodass der Server es nicht als gültiges Array erkennen konnte.

**Konsequenz / Lösung:** Ich habe daraufhin jede Datei in einem eigenen `write`-Aufruf (jeweils ein einzelnes Array-Element) geschrieben – alle vier Vorgänge waren dann erfolgreich.

Mit letzter Sicherheit kann ich die interne Ursache nicht belegen (die Fehlermeldung nennt nur das Symptom „kein Array"); die Payload-Größe des Sammelaufrufs ist aber die plausibelste Erklärung, zumal identisch strukturierte Einzelaufrufe anschließend fehlerfrei durchliefen.
