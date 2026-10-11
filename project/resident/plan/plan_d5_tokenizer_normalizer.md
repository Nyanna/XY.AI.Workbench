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
