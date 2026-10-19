Setze D5 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST Tools vor

# D5 — Tokenizer & Normalizer (CPU, geteilt)

**Zweck:** `tokenizers`-Crate, genau eine Instanz, Query/Doc-Modus (Marker/`[MASK]`/doc_maxlen), Normalizer CPU, Ausgabe `u32`-IDs + Attention-Maske, Längen-Bucketing. **Widerspruch GPU-Tokenizer aufgelöst.**

**Einordnung:** Abhängt von D1 (Lifecycle/Lease der geteilten CPU-Ressource). Downstream: D4 (liefert IDs+Maske+Präfix), G1. CPU-only, kann parallel zu D2/D3 entstehen. Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §3; `resident_first.md` (Tokenizer); `resident.md` (GPU-Tokenizer-Idee); `context/context_d.md` D5 (#19–#21), #20/#34.

---

## Festgelegte Entscheidungen

- **Widerspruch aufgelöst (`[V-Argumentation]`, #19):** **Kein** GPU-Tokenizer, **kein** ID→Adresse-Gather-Kernel. Begründung: Tokenisierung ist String-Verarbeitung (CPU); der Embedding-Lookup ist ein `seq×384`-Gather, gegenüber 12 Transformer-Layern unmessbar; ein eigener Gather-Kernel spart nichts, bricht candle-Kompatibilität und erzwingt Adresspflege bei Modell-Eviction. **`resident_subengine.md` („nur Ausblick") gewinnt gegenüber `resident.md`.** Pipeline: **CPU-Tokenizer → `u32`-IDs → H2D → `index_select`.**
- **tokenizers (`[V]`, #20):** `Encoding::get_ids() -> &[u32]`, `get_attention_mask() -> &[u32]`, `get_type_ids()`. **Normalisierung liegt im Tokenizer** (`tokenizer.json`: Normalizer → Pre-Tokenizer → Model → Post-Processor) — nicht separat nachbauen. Batch: `encode_batch` + `with_padding`/`with_truncation`.
- **Genau eine Instanz:** geteilte CPU-Ressource beider Subengines (D1-Lease), modellspezifisch oder geteilt.
- **Modi (`[U]`, #21):**
  - **e5:** `"query: "`/`"passage: "` als **Klartext vor** der Eingabe; Special Tokens `<s>…</s>` fügt der Tokenizer hinzu.
  - **ColBERT (Wechselpfad):** Marker (`[Q]`/`[D]` bzw. Prefix-Tokens je Checkpoint); Query-Padding mit `[MASK]` bis `query_maxlen` (32); Dokument-Kürzung auf `doc_maxlen`. Das ist **Anwendungslogik nach `encode`**, keine Tokenizer-Funktion; je Checkpoint-Doku prüfen.
- **Präfix (#34):** Pflicht für e5; für symmetrische Aufgaben `query: ` beidseitig. Falsches/fehlendes Präfix verschlechtert Qualität deutlich → Tests im Query- und Index-Pfad.
- **Bucketing:** Padding auf gemeinsame Länge im Batch, Längen-Bucketing (koordiniert mit D4-Batcher).

---

## Zu erstellende Artefakte (`rag/src/tokenize/`)

- `mod.rs`
  - `struct SharedTokenizer` — eine `tokenizers::Tokenizer`-Instanz (geteilt, D1-Lease).
  - `enum EncodeMode { Query, Passage }` (e5) / erweiterbar für ColBERT-Marker.
  - `struct TokenBatch { ids: Vec<u32>, mask: Vec<u32>, seq_len, offsets_per_task }`.
  - `fn encode(mode, texts: &[&str]) -> Result<TokenBatch, TokenizeError>` — Präfix anwenden, `encode_batch`, Padding/Truncation, Bucketing.
- `modes.rs`
  - e5-Präfixlogik; ColBERT-Marker/`[MASK]`-Padding/`doc_maxlen` (vorbereitet, Wechselpfad).
- `normalize.rs`
  - Verweis: Normalisierung bleibt **im** Tokenizer; nur optionale **vorgeschaltete** externe Chunk-Textnormalisierung als separater, konfigurierbarer Hook (nicht Teil der Engine-Kernlogik).

---

## Arbeitsschritte

1. `SharedTokenizer` aus `tokenizer.json` laden; genau eine Instanz, als D1-Ressource mit Lease.
2. `EncodeMode` + Präfix-Anwendung (e5 `query:`/`passage:`).
3. `encode`/`encode_batch` mit `with_padding`/`with_truncation`; Ausgabe `u32`-IDs + Maske + Offsets pro Task.
4. Längen-Bucketing mit D4-Batcher abstimmen (gemeinsame Buckets).
5. ColBERT-Modus (Marker, `[MASK]`-Padding bis `query_maxlen`, `doc_maxlen`) als vorbereiteten Pfad implementieren, inaktiv für e5.
6. Dokumentieren, dass GPU-Tokenizer/Adressmapping **nicht** umgesetzt wird (Widerspruch-Auflösung in Code-Doku).

## Fehler (Q1) & Logging (Q2)

- `enum TokenizeError { Load{source}, EncodeFailed{source}, EmptyInput, Truncated{at} }`.
- Span pro Batch (`mode`, `n_texts`, `seq_len`); Events `encoded`, `bucketed`. Texte **nicht** loggen (PII/Größe), nur Längen.

## Feature-Flags (Q4)

- Komplett **GPU-frei**; baut ohne `cuda`. IDs gehen erst in D4 (H2D) auf die GPU.

## Tests (Q3)

- IDs/Maske gegen Referenz-Encodings (Fixtures) für e5 (query/passage).
- Präfix-Effekt: falsches/fehlendes Präfix erkennbar (Test prüft, dass Präfix tatsächlich vorangestellt wird).
- Bucketing/Padding: korrekte `seq_len`, Maske 0/1, Offsets pro Task.
- ColBERT-Modus: `[MASK]`-Padding bis `query_maxlen`, `doc_maxlen`-Kürzung.

## Definition of Done

- Eine geteilte Tokenizer-Instanz; e5 Query/Passage korrekt; Bucketing mit D4 abgestimmt; GPU-Tokenizer-Verzicht dokumentiert; ColBERT-Modus vorbereitet.

## Result

D5 is implemented in `rag/src/resident/tokenize/`, next to the existing `model/` and `subengine/` modules, not in `rag/src/tokenize/` as the plan says. The 8 new tokenizer tests pass, as do the other 211 `resident::` lib tests. I ran only those lib tests, not the full suite, and I did not build with `cuda`.

- **`SharedTokenizer`:** one instance loaded from the `tokenizer.json` next to the model's config file. It implements `Resource`, so it can be leased. Padding is left to the existing batcher, and truncation to `max_seq_len` happens inside the tokenizer, so the closing `</s>` token is kept.
- **Modes:** `EncodeMode` is `Query` or `Passage`. For e5, the prefixes come from `ModelSpec.prefixes`. The ColBERT path is prepared but unused for e5: markers, `[MASK]` padding up to `query_maxlen`, and `doc_maxlen` cutting.
- **Normalizer hook:** `TextNormalizer` is an optional hook that runs before the prefix is added. It does no normalization of its own, because that stays inside the tokenizer.
- **Errors:** `TokenizeError` has `Load`, `EncodeFailed` and `EmptyInput`. I left out `Truncated{at}` because truncation happens silently inside the tokenizer. The error is also wired into `ResidentError`, including retryability and the Send/Sync test.
- **Bucketing:** I did not add a separate `TokenBatch` type. `encode` returns unpadded id sequences, and `encode_batches` hands them to the existing `plan_batches`, so bucket sizes are shared with the encoder batcher. `Batch.indices` maps rows back to the input order and takes the place of `offsets_per_task`.
- **Tests:** a synthetic word-level tokenizer covers prefixes, truncation, empty input, padding and mask, ColBERT mode, the normalizer hook and the resource key. One more test runs against the real e5-small `tokenizer.json` and returns early if that file is missing.

I did not write the doc comment explaining why there is no GPU tokenizer. The module header only says ids reach the device as `u32` inputs to the encoder.
