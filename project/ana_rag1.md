Sortiert nach grober Gesamtkosten (Aufbau plus Abfrage). Die Reihenfolge ist meine Einschätzung, nicht gemessen. Rolle: **G** = generiert Kandidaten aus dem ganzen Scope, **V** = bewertet nur vorhandene Kandidaten.

| #  | Layer                                     | Werkzeuge                                                                                  |
| -- | ----------------------------------------- | ------------------------------------------------------------------------------------------ |
| 1  | Dateinamen                                | `fd`, glob                                                                                 |
| 2  | Textsuche, exakt/Regex                    | grep → `ripgrep` → `ugrep` (Fuzzy, boolesch)                                               |
| 3  | Textsuche über Formate                    | `ripgrep-all`, `pdfgrep`                                                                   |
| 4  | Fuzzy-Nachbewertung                       | `rapidfuzz`                                                                                |
| 5  | Strukturelle Suche                        | `ast-grep` (parst zur Laufzeit)                                                            |
| 6  | Trigramm-Index                            | `csearch`/`cindex`, Zoekt, `fcs`, FTS5-Trigramm, `ugrep-indexer`                           |
| 7  | BM25 auf Wortebene                        | FTS5, `bm25s`, Tantivy, Recoll, `qmd search`, mit eigener Tokenisierung (camelCase, LaTeX) |
| 8  | Symbol-/AST-Index                         | tree-sitter (Definitionen, Aufrufer, Spans)                                                |
| 9  | Formel-Struktur                           | Tangent-CFT, SSEmb, Approach0                                                              |

| 10 | Gelernt spärlich                          | SPLADE, BGE-M3 sparse                                                                      |
| 11 | Statische Embeddings                      | Model2Vec + NumPy-Matrix                                                                   |
| 12 | Dichter Bi-Encoder                        | BGE-M3 dense, Jina, Qwen3-Embedding, SPECTER2 (Paper-Abstracts)                            |
| 13 | Dichte Vektoren auf LLM-Sichten           | Kontextzeilen, Doc2Query, Formelbeschreibungen                                             |
| 14 | Multivektor-Rescoring                     | ColBERT-MaxSim auf Kandidaten, BGE-M3 multi, PyLate `rank`                                 |
| 15 | Late Interaction mit eigenem Index        | PyLate/FastPLAID mit LateOn-Code-edge (17M) → LateOn-Code (130M), GTE-ModernColBERT        |
| 16 | Mehrere Late-Interaction-Modelle parallel | z. B. BGE-M3 multi plus domänenspezifisches ColBERT                                        |

**Außerhalb der Reihe:** Cross-Encoder-Reranker und LLM-Reranking (`qmd`) bewerten Query und Text gemeinsam und gehören ans Ende, kosten aber pro Kandidat deutlich mehr als Late Interaction. Der Agent selbst ist der letzte und teuerste Layer, mit dem größten Kontext.

**Zur Lesart**

* Die Stufen 1 bis 5 brauchen keinen Index und sind immer aktuell, damit sind sie dein Änderungsdetektor und der Rückfall bei Lücken der Folgelayer.
* Stufe 13 ist am Indexierungsende teuer, bei der Abfrage aber so billig wie Stufe 12.
* Die Verfeinern-Layer (4, 14) sind in kleinen Timeboxen nutzbar, weil sie nur Kandidaten laden. Sie sind durch den Recall der Vorstufen begrenzt, 15 und 16 können dagegen eigene Treffer finden.
* Lage und Reihenfolge von 8 bis 11 hängen von Korpus und Hardware ab. Das Logging zeigt, ab welcher Stufe ein weiterer Layer pro KB nichts mehr findet, was die vorherigen nicht schon lieferten.
