# Resident – Modellkandidat `intfloat/multilingual-e5-small`

Soft-Prompt-Notizen. Zahlen sind Parameter, **[prüfen]** = unverifiziert.

## Modell
- Dense Bi-Encoder: Query und Satz getrennt kodiert, ein Vektor pro Text, Ähnlichkeit per Cosinus.
- 118M Parameter, 384 Dim, XLM-R-Vokabular (~250k), mehrsprachig inkl. Deutsch.
- Architektur: BERT mit XLM-R-Tokenizer, daher `candle-transformers` `BertModel`, nicht `xlm_roberta`.
- Dateien: `model.safetensors`, `tokenizer.json`, `config.json`.
- Kein Rerank im Standardpfad, Kandidatenliste per Cosinus/Top-k genügt.

## Pipeline-Konfiguration (geteilt zwischen CPU- und GPU-Subengine)
- `prefix_query = "query: "`, `prefix_passage = "passage: "`, Pflicht.
- Embedding-Parameterobjekt: Enum `Query | Passage` wählt das Präfix.
- `pooling = Mean` mit Attention-Maske: Σ(h·mask)/Σ(mask).
- Danach L2-Normalisierung, dann reicht Skalarprodukt.
- `max_seq_len` 64–128 für Sätze, Maximum 512.
- dtype: GPU f16, CPU f32.
- Config ist reine Daten, nur der Ressourcen-Adapter einer Stage unterscheidet CPU/GPU.

## Tokenizer
- `tokenizers`-Crate, `tokenizer.json`, CPU-Ressource der CPU-Subengine, genau eine Instanz.
- Normalisierung gehört zum Tokenizer, Parameter-konfiguriert, immer CPU.
- Ausgabe `u32`-IDs plus Attention-Maske (`encoding.get_attention_mask()`).
- Padding auf gemeinsame Länge im Batch, Längen-Bucketing.

## Index
- Spalte `FixedSizeList<f16, 384>` plus Zeilen-/Offset-ID, Parquet.
- Schema versioniert: Modell-ID, Dim, Pooling, Präfixe, dtype.
- Index liefert zu einer ID den Originaltext (Payload), nötig für Cross-Encoder-Scorer.
	- Der Index speichert hierfür Zieldatei und Zeichenoffsets der Zeile. Eine Checksum dient der Verifizierung.
- Modellwechsel = neue Schema-Version und Re-Embedding.

## Similarity
- Scorer ist Parameter: `Cosine` oder `CrossEncoder(modell)`.
- Scope = optionales Index-Subset (ID-Liste, Bereich, Ergebnis eines früheren Tasks).
- Parameter: `top_k`, optionale Vorselektion.
- Rerank ist kein eigener Tasktyp, nur Similarity mit anderem Scorer auf einem Subset.
- Das LLM entscheidet anhand der Ergebnismenge und Intent, ob ein zweiter Task mit CrossEncoder-Scorer eingereiht wird.
- Engine liefert Entscheidungsgrundlagen: Anzahl, Score-Abstand Top-1 zu Top-k, Streuung, Task-ID.
	- Das ist Aufgabe der Schicht vor der Resident Engine, zurückgegeben werden Zieldatei + Offsets und Entry IDs wie im Index hinterlegt. Die äußere Schicht kann hieraus bei Bedarf einen Folgetask mit Index Einschränkung erstellen.
- Cross-Encoder-Modell (z. B. `BAAI/bge-reranker-v2-m3`) wird nur lazy geladen, wenn angefordert, TTL entlädt es.
	- Für die aktuelle Umsetzung nicht vorgesehen

## Wechselpfade
- `BAAI/bge-m3`: 1024 Dim, CLS-Pooling, kein Präfix, optional Multi-Vector. Nur Config-Wechsel plus Re-Embedding.
- ColBERT (`jina-colbert-v2`, `answerai-colbert-small-v1`) für Paper-Pfad, benötigt Projektionskopf und MaxSim.

## Prüfen vor der Implementierung
- [ ] `model.safetensors` im Repo vorhanden, sonst konvertieren.
- [ ] Gewichtsnamen kompatibel mit candle `BertModel`.
- [ ] Mean-Pooling mit Maske und L2 gegen Referenz (sentence-transformers) vergleichen, Cosinus-Abweichung klein.
- [ ] f16 vs. f32 Toleranz bei gemischter CPU/GPU-Indexierung festlegen.
- [ ] Präfix-Behandlung bei Index- und Query-Pfad testen (falsches Präfix verschlechtert Qualität deutlich).