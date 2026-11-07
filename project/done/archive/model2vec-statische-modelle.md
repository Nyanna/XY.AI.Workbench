# Model2Vec, statische Modelle und das Kontinuum zum Multi-Layer Retrieval

> Extrapolation des konzeptionellen Gesprächsverlaufs: von "Was ist Model2Vec" zu einer generischen, agentengesteuerten Multi-Layer-Retrieval-Architektur.

## 1. Ausgangspunkt: Statische vs. kontextuelle Embeddings

**Model2Vec** destilliert ein Sentence-Encoder-Modell (Lehrer) in eine reine Lookup-Tabelle: Jedes Vokabular-Token wird isoliert (ohne Satzkontext) durch den Lehrer geschickt, das Ergebnis per PCA reduziert und Zipf-gewichtet. Zur Laufzeit gibt es keinen Transformer-Forward-Pass mehr — nur Lookup + Mittelung.

**Kernunterschied zu kontextuellen Modellen:** Ein Transformer berechnet den Vektor eines Tokens abhängig vom gesamten Satz (Self-Attention). Ein statisches Modell hat *einen* Vektor pro Token, unabhängig vom Kontext. Der Satzvektor ist ein (gewichteter) Mittelwert — permutationsinvariant, ohne Wortreihenfolge, ohne Negation, ohne Disambiguierung.

**Vorteile:** 100–500x schneller, winzig (MB statt GB), CPU-only, günstige Distillation, für grobe Themenähnlichkeit oft ausreichend (~85–90 % der Lehrer-Qualität auf MTEB).

**Nachteile:** keine Polysemie-Auflösung, keine Syntax/Skopus ("nicht gut" ≈ "gut"), schwächer bei Paraphrase/NLI/Reranking, Mittelung verwäscht lange Texte.

**Faustregel:** Statisch für Vorfilter/Clustering/Dedup/große Indizes; kontextuell für Nuancen oder als zweite Stufe (Rerank) auf statisch vorgefilterten Kandidaten.

## 2. Teilweise Kontext-Kompensation bei statischen Modellen

Lokaler Kontext lässt sich in die Lookup-Tabelle verlagern, ohne echte Kontextabhängigkeit zu erreichen:

- **N-Gramme/Phrasen als Einträge** (fastText-Bigramme, Hashing-Trick) — fängt Negation/feste Wendungen lokal ab, kostet Tabellengröße.
- **Kontext einbacken:** Mittelung kontextueller Embeddings über viele Vorkommen im Korpus (Bommasani et al.) — ergibt Durchschnitt über alle Bedeutungen.
- **End-to-end trainierte Tabellen:** Contrastive Loss direkt auf der Embedding-Tabelle (`StaticEmbedding` in Sentence-Transformers).
- **Gelernte Token-Gewichte** (SIF, Model2Vec-Gewichtung) — steuern Beitrag, ersetzen keinen Kontext.
- **Sense-Embeddings** (sense2vec) — mehrere Vektoren pro Token nach Heuristik gewählt.

**Grenze:** Solange der Satzvektor eine Summe/Mittelwert von Lookups ist, bleibt er permutationsinvariant. Mittelweg: statische Embeddings + flache Schicht (CNN, Gating, Mini-Attention) — nicht mehr rein statisch, aber Größenordnungen billiger als ein voller Encoder.

## 3. Das Kontinuum: mehrere unabhängige Achsen

Statt eines einzigen Kontinuums gibt es mehrere Achsen:

- **Zeitpunkt:** Wann fließt Kontext ein? (Training vs. Laufzeit)
- **Reichweite:** Token → Bigramm → Fenster (CNN) → ganzer Satz (Attention)
- **Mischmechanismus:** feste Operation (Mittelwert) vs. eingabeabhängig (Attention)

| Stufe | Kontext | Kosten pro Text |
|---|---|---|
| Lookup + Mittelwert | keiner | linear, minimal |
| + N-Gramm-Einträge | lokal, vorberechnet | linear, größere Tabelle |
| + CNN/Gating | festes Fenster | linear |
| RNN | Reihenfolge, sequenziell | linear, nicht parallel |
| Transformer | beliebige Paare, eingabeabhängig | quadratisch |

Der qualitative Sprung liegt bei der **Eingabeabhängigkeit der Mischung**: Erst Attention entscheidet zur Laufzeit, welches Token für welches relevant ist. Faustformel: Je mehr Kontext vorab in Parameter verlagert wird, desto billiger die Inferenz, desto weniger reagiert das Modell auf unvorhergesehene Konstellationen.

## 4. Zweite Achse: Interaktion zwischen Query und Dokument

Unabhängig von "wie entstehen Token-Vektoren" gibt es die Achse "wie wird gematcht":

| | Token-Vektoren | Dokument-Repräsentation | Matching |
|---|---|---|---|
| Model2Vec | statisch | 1 Vektor (Mittelwert) | Cosinus |
| Dense Bi-Encoder | kontextuell | 1 Vektor (Pooling) | Cosinus |
| ColBERT (Late Interaction) | kontextuell | n Vektoren (je Token) | MaxSim |
| Cross-Encoder | kontextuell, gemeinsam | keine, pro Paar | Transformer-Score |

**ColBERT / Late Interaction:** Dokumente werden offline encodiert (query-unabhängig, daher vorberechenbar), nur die Query wird zur Laufzeit encodiert. MaxSim: für jeden Query-Token der ähnlichste Dokument-Token, Summe der Maxima. Das ist ein **weicher invertierter Index** — statt exaktem Term-Match ein Vektor-Nähe-Match.

**Cross-Encoder** ist NICHT vorberechenbar: Query und Dokument laufen gemeinsam durch Attention, die Dokument-Repräsentation hängt von der Query ab.

## 5. BM25 und invertierter Index als Fundament

**Invertierter Index:** Wort → Liste der Dokumente (dreht Dokument→Wörter um). **BM25** ist die Scoring-Formel darauf: Termhäufigkeit (mit Sättigung) × inverse Dokumenthäufigkeit (IDF) × Längennormalisierung.

**word2vec vs. BM25:** word2vec ist eine gelernte, dichte Repräsentation (semantische Ähnlichkeit, aber kein Dokument-Score). BM25 ist eine ungelernte, spärliche Scoring-Formel (nur exakte Terme, aber IDF-Gewichtung und Längennormalisierung eingebaut). Komplementär — daher Hybrid-Suche.

## 6. Embeddings sind keine "exakten Exporte"

Ein Embedding ist eine **Projektion** eines bestimmten Abgriffs (meist letzte Schicht, gepoolt), nicht ein vollständiger Export interner Repräsentationen. Rohe Hidden States generativer Decoder sind für Ähnlichkeitssuche schlecht geeignet (anisotroper Raum, auf Next-Token-Vorhersage optimiert). Gute Embedding-Modelle brauchen zusätzliches **kontrastives Training**.

Model2Vec exportiert nur den **kontextfreien** Ausschnitt eines Lehrermodells: Vektor(Token | kein Kontext), nicht Vektor(Token | Kontext). Der Attention-Mechanismus, der Kontext verarbeitet, lässt sich nicht in eine Tabelle kopieren.

## 7. Erweiterung des Vektorraums: Mehrere Vektoren pro Chunk

- **Nach Textstruktur:** Propositionen/Sätze, Late Chunking (ganzer Text durch Encoder, dann Pooling pro Abschnitt), hierarchisch (RAPTOR).
- **Nach erwarteten Queries:** Doc2Query (generierte Fragen indexieren), HyDE (Query → hypothetisches Antwortdokument).
- **Nach Token (ColBERT):** Extremfall, ein Vektor pro Token.
- **Feste Zwischenformen:** Poly-Encoder, ME-BERT, MUVERA (Multi-Vektor → fester Vektor).

**Grenzen:** Speicher/Latenz wachsen linear mit Vektorzahl, Aggregation muss definiert werden, Redundanz erzeugt Dedup-Bedarf, Abdeckung ist immer eine Annahme über künftige Queries.

## 8. Zentrale These: Retrieval ist Lokalisierung ("wo steht etwas über X")

Der Nutzer formuliert die Kernthese: Eine Suche im Index beantwortet im Kern nur einen Fragetyp — Lokalisierung. Komplexere Fragetypen (Antwortform, Relation, Aggregation) lassen sich oft **query-seitig** lösen (Zerlegung in Teilfragen, Schnittmengenbildung), nicht zwingend index-seitig (z. B. Doc2Query).

Das führt zur **Achse "Wann und wo wird verstanden?"**:

| | Index macht Conclusion | Agent macht Conclusion |
|---|---|---|
| Kosten | einmal, amortisiert über alle Queries | pro Query wiederholt |
| Latenz | niedrig | hoch |
| Passung | nur für antizipierte Fragen optimal | passt sich der Query an |
| Aktualität | veraltet bei Quelländerung | immer auf Rohdaten |
| Fehler | eingefroren, unsichtbar | sichtbar, korrigierbar |
| Auditierbarkeit | Herkunft geht verloren | Belegstelle im Blick |

Die Verteilung der Queries entscheidet, wo man auf dieser Achse steht. Rohtext ist das einzige verlustfreie Ende; jede Vorab-Conclusion ist eine Kompressions-Annahme.

## 9. Regler statt Kategorien: ein gemeinsamer Parameterraum

Die scheinbar verschiedenen Verfahren (statisch, Bi-Encoder, ColBERT) sind **Punkte auf denselben Reglern**, nicht verschiedene Kategorien:

| Regler | Wirkt auf | Tausch |
|---|---|---|
| Chunk-Größe | Adressierungsgranularität | präziser vs. mehr Vektoren |
| Pooling-Faktor / Pruning | Vektoren pro Chunk | kleiner Index vs. weniger Late Interaction |
| Dimension, Quantisierung | Kapazität pro Vektor | Speicher vs. Auflösung |
| Encoder-Kontext (isoliert → Late Chunking) | vorab eingebackener Kontext | Indexierkosten vs. Disambiguierung |
| Encoder-Größe | Qualität der Vorabrepräsentation | Indexierkosten vs. Güte |

Extrempunkte: Model2Vec (alle Regler grob) ↔ Dense Bi-Encoder (mittel) ↔ ColBERT (alle Regler fein).

**Einschränkungen:** Regler sind nicht unabhängig (Wechselwirkungen), und nicht jedes Modell ist für jeden Reglerpunkt gleich gut trainiert (Matryoshka-Training, ColBERT-Pooling-Toleranz als Lösungen).

## 10. Routing/Kaskade als praktischer Kompromiss

Statt eines festen Reglerpunkts: **Umschalten im Kontinuum basierend auf Fit und Schwellwert.**

- **Indexseitig:** Fit-Signal (Länge, Dichte, Domäne) wählt Auflösung pro Chunk.
- **Querysseitig (Kaskade):** billige Stufe liefert Kandidaten + Konfidenz; bei niedriger Trennschärfe (Margin, Score-Entropie, Disagreement zwischen BM25 und Embedding) Eskalation zu feinerem Matching.

**Fallstricke:** Kalibrierung zwischen Stufen, falsche Sicherheit der billigen Stufe, Recall-Grenze (Stufe 1 muss auf Recall getrimmt sein), Konsistenz (Stufen sequenziell, nicht in gemischten Vektorräumen).

## 11. Existierende Bausteine (Stand der Recherche)

- **Vektorzahl-Regler:** Token Pooling/Pruning bei ColBERT.
- **Kapazität-Regler:** Matryoshka-Training, Quantisierung, ColBERTv2-Residual-Kompression.
- **Kontext-Regler:** Late Chunking (Jina).
- **Verbindende Modelle:** BGE-M3 (dicht + spärlich + Multivektor aus einem Modell), SPLADE (gelernt spärlich), MUVERA (Multi-Vektor → fest).
- **Infrastruktur:** Vespa (volle Freiheit, steile Lernkurve, Ranking-Ausdrücke im Cluster), Qdrant (schlank, Prefetch-Ketten, Rust), Weaviate (Komfort, Module, Multi-Tenancy, Go).

Kein gefundenes Framework behandelt alle Regler als bewusst entworfene, gemeinsame Tauschfläche.

## 12. Wendepunkt: Agent als universeller Retriever

Der Nutzer bringt den entscheidenden Kontrapunkt ein: Ein bereits etabliertes **2-Stage-Konzept** — Tools (grep, bash, context7, OpenAlex, Exa …) werden gecacht, Stage 1 liefert nur gefilterte, ID-assoziierte Auszüge, Stage 2 liefert Details auf Nachfrage — ist in der Praxis **nie in den Worst Case gefallen** und bestehenden Retrieval-Verfahren überlegen.

**Warum das robust ist:**
- **Verlustfrei per Referenz:** Volle Ausgabe bleibt im Cache, Stage 1 ist nur eine deterministische Sicht darauf. Nichts geht verloren, es wird nur aufgeschoben.
- **Deterministisch statt gelernt:** Kein unvorhersehbares Modellversagen.
- **Der Agent ist der Ranker:** Er sieht vollen Query-Kontext und Zwischenstand — strukturell überlegen gegenüber jedem vorab trainierten Bi-Encoder (100 Mio.–wenige Mrd. Parameter, einmalige Entscheidung ohne Rückfrage).

**Kostenlogik (korrigiert im Dialog):** Ein zusätzlicher Turn kostet einen Cache Read, aber jeder Decode-Schritt liest ohnehin den gesamten KV-Cache — das ist bei langem Kontext der dominierende Aufwand. Eine gefilterte, kleine Sicht verkürzt **jeden** Folgeturn (nicht nur den aktuellen), während eine große Rohausgabe ihn dauerhaft verlangsamt. Das Netto fällt bei langen Sessions und großen Ausgaben klar zugunsten der Filterung aus.

**Vokabular-Lücken (privater Jargon):** Hier hat der Agent einen strukturellen Vorteil, den kein Retriever hat — er lernt das Vokabular *während* der Suche (Pseudo-Relevanz-Feedback mit Modell als Auswerter), ohne Training und ohne Index. Bei exakten Begriffen schlägt BM25/grep ohnehin wörtlich, dichte Embeddings sind hier die eigentlich schwache Stelle.

**Grenzen des Musters (verbleibend, nicht widerlegt):**
- Recall-Obergrenze: was die unterliegenden Tools nicht liefern, kann der Agent nicht finden.
- Reduktionsregel (Top-N, Kürzung) muss erkennbar machen, was abgeschnitten wurde (keine stille Verlustquelle).
- Aggregationsfragen ("alle Stellen, die X widersprechen") erhöhen Rundenzahl und Kosten.

## 13. Finale Architektur: Multi-Layer Anytime Retrieval Engine

Aus dem Dialog entsteht eine konkrete, generische Architektur, die alle vorherigen Achsen vereint — als **additive, agentengesteuerte Komposition unabhängiger Layer** über einem gemeinsamen Speicher (dem Dateisystem).

### 13.1 Grundprinzipien

1. **Gemeinsamer Speicher:** Das Dateisystem selbst ist die "Tabelle". Keine Datenbank nötig, solange Layer unabhängig bleiben.
2. **Layer sind vollständig unabhängig:** Jeder Layer wählt eigene Chunk-Granularität, eigenes Modell, eigenen Index — oder gar keinen (grep).
3. **Einziges geteiltes Koordinatensystem:** `(Inhalts-Hash, Start, Ende)` — Spans innerhalb einer Datei. Kein gemeinsames Chunk-Schema nötig.
4. **Additiv, nie subtraktiv:** Ein fehlender, veralteter oder unvollständiger Layer kann nur *weniger* Signal liefern, nie ein falsches. Teilabdeckung ist daher unkritisch (nur im Status sichtbar machen, damit "nichts gefunden" nicht mit "nicht gelaufen" verwechselt wird).
5. **Signale werden getaggt, nicht fusioniert.** Jeder Treffer trägt eine Liste von Signalnamen (`["bm25", "zusammenfassung"]`). Die Bedeutung eines Signals (Herkunft: Text vs. abgeleitet; Granularität; Abdeckung) steht einmal im Status, nicht pro Treffer. Fusion/Gewichtung bleibt beim Agenten, der die Signale mit vollem Query-Kontext interpretiert.
6. **Timebox und Qualitätsziel kommen vom Aufrufer (Agenten).** Er kennt Korpusgröße, Promptziel (präzises kleines Set vs. große Kandidatenmenge vs. schnelle Antwort) und wählt Parameter entsprechend. Die Engine liefert ausschließlich Signale, sie antizipiert nicht.
7. **Lazy, nutzungsgetriebene Materialisierung:** Layer werden nicht präventiv für den ganzen Korpus gebaut, sondern on-demand, priorisiert nach Nutzung (Dateien aus dem letzten Request zuerst), bei freien Ressourcen bis zur Timebox oder dem nächsten Request.
8. **Änderungserkennung ist lazy, kein separater Dienst.** Der immer-aktuelle Textsuch-Layer (grep/ripgrep) läuft zuerst, erkennt per `stat`/Hash-Abweichung geänderte Dateien im Scope und reiht sie für die teureren Folgelayer ein. Kein Watcher, kein periodischer Scan nötig.

### 13.2 Layer-Rollen

- **Generieren (G):** liefert eigene Kandidaten aus dem gesamten Scope.
- **Verfeinern (V):** bewertet nur vorhandene Kandidaten (günstig, aber durch Recall der Vorstufe begrenzt).

### 13.3 Sortierte Layer-Liste (grob nach Gesamtkosten)

| # | Layer | Werkzeuge/Ansatz | Rolle |
|---|---|---|---|
| 1 | Dateinamen | `fd`, glob | G |
| 2 | Textsuche exakt/Regex | grep → ripgrep → ugrep (Fuzzy, boolesch) | G |
| 3 | Textsuche über Formate | ripgrep-all, pdfgrep | G |
| 4 | Fuzzy-Nachbewertung | rapidfuzz | V |
| 5 | Strukturelle Suche | ast-grep | G |
| 6 | Trigramm-Index | csearch/cindex, Zoekt, FTS5-Trigramm | G |
| 7 | BM25 Wortebene | FTS5, bm25s, Tantivy, Recoll | G |
| 8 | Symbol-/AST-Index | tree-sitter | G |
| 9 | Formel-Struktur | Tangent-CFT, SSEmb | G |
| 10 | Gelernt spärlich | SPLADE, BGE-M3 sparse | G |
| 11 | Statische Embeddings | Model2Vec + NumPy-Matrix | G |
| 12 | Dichter Bi-Encoder | BGE-M3 dense, Jina, Qwen3-Embedding | G |
| 13 | Dichte Vektoren auf LLM-Sichten | Kontextzeilen, Doc2Query, Zusammenfassungen | G |
| 14–15 | Late Interaction (MaxSim) | ColBERT, PyLate-Encoder + Brute-Force-MaxSim (kein PLAID mehr nötig, da Inkrementalität wichtiger als Geschwindigkeit) | G/V |
| — | Cross-Encoder / LLM-Rerank | qmd, Reranker-Modelle | V, teuerste Stufe vor dem Agenten |

### 13.4 Separate Layer für Metadaten und abgeleitete Sichten

- **Metadaten sind keine Index-Last, sondern Anreicherung beim Match.** Pfad, mtime, Sprache, Symbolkontext werden erst beim Treffer aus dem aktuellen Dateistand abgeleitet — kein Reindex bei Umbenennung, kein Schema-Zwang im Layer.
- **Zusammenfassungen, generierte Fragen und Metadaten sind eigene Layer**, nicht in den Chunktext eingebacken. Dadurch bleibt der Vertrag einheitlich (`hash, span, signals`).
- **Erkenntnisgewinn:** Die Konstellation der Signale selbst ist Information für den Agenten:

| Treffer-Konstellation | Bedeutung |
|---|---|
| nur Chunk-Layer (`source`) | Stelle sagt es wörtlich, Dokument nicht erkennbar thematisch passend |
| nur Zusammenfassung/Frage-Layer (`derived`) | Abstraktion/Schlussfolgerung, nicht wörtlich belegt — Hypothese zur Prüfung |
| beide | stärkste Evidenz, direkt zitierfähig |

Dies macht abgeleitete Layer zu **Wegweisern**, nicht zu Belegen — der Agent kann über die Herkunftsmarkierung (`source` vs. `derived`) unterscheiden, was im Korpus steht und was nur interpretiert wurde.

### 13.5 Change Detection & Upserts als gemeinsamer Baustein

- **Kein Merkle-Baum, kein zentraler Watcher nötig** — stattdessen: der immer aktuelle Textsuch-Layer *ist* der Change-Detektor. Beim Durchlauf über den Scope vergleicht er `stat` (Größe, mtime) jeder Treffer-/Scope-Datei gegen den zentralen Index; bei Abweichung wird gehasht und die Datei mit hoher Priorität in die Aufbau-Warteschlange der Folgelayer eingereiht.
- Treffer der Folgelayer auf zwischenzeitlich geänderte Dateien werden verworfen oder als veraltet markiert (Span passt nicht mehr).
- Inhaltsadressierung (Hash als Schlüssel statt Pfad) macht Umbenennen/Verschieben kostenlos und dedupliziert identische Dateien automatisch.
- Idempotentes `apply(add, remove)` pro Layer, geschlüsselt über `(layer, hash)`, macht Abbrüche und Wiederholungen unkritisch — passt zu abbrechbarer, lazy Hintergrundarbeit.

### 13.6 Layer-Vertrag (Protocol)

```python
class Layer(Protocol):
    name: str
    roles: set[Literal["generate", "refine"]]
    def applies(self, scope) -> bool: ...
    def status(self, scope) -> LayerStatus:  # Herkunft, Granularität, Abdeckung, Query-Format, Kosten
        ...
    async def search(self, query, scope, budget, candidates=None) -> list[Hit]: ...
    async def build(self, units, cancel: asyncio.Event) -> None: ...
```

`Hit = (hash, start, end, excerpt, signals: list[str])`. Status wird einmal pro Layer kommuniziert, nicht pro Treffer — hält den Vertrag minimal und beliebig erweiterbar (neuer Layer = neuer Name, keine Schemaänderung).

### 13.7 Einordnung gegenüber bestehenden Ansätzen

Die Einzelbausteine haben Vorläufer (Cursor: zwei feste Layer — lokaler Trigramm-Index + semantischer Index, Merkle-Diff-Updates; mcp-context/context-compress: Tool-Output-Filterung mit ID-Nachforderung; Anytime Ranking in der IR-Forschung: Budget-gesteuerter Kaskaden-Abbruch). Die Kombination aus (a) offener, nicht fest verdrahteter Layer-Menge, (b) aufrufergesteuerter Timebox/Qualitätsziel über heterogene Layer hinweg, (c) dem Textsuch-Layer als impliziten Change-Detektor für alle teureren Folgelayer und (d) generischem Tool-Filter-Protokoll über beliebige externe Quellen (OpenAlex, Exa) wurde in der Recherche in dieser Form nicht gefunden.

## 14. Praktische Konsequenzen für die Umsetzung

- **Start schlank, lokal, eingebettet:** SQLite+FTS5 (Wort- und Trigramm-Tabelle), NumPy/FAISS für dichte Vektoren, PyLate für ColBERT-Encoding, Brute-Force-MaxSim über Sidecar-Dateien (kein PLAID-Index nötig, da Inkrementalität wichtiger ist als Abfragegeschwindigkeit).
- **Qdrant/Vespa erst bei Bedarf** (mehrere Nutzer/Dienste auf demselben Index, starke Filter, sehr große Korpora) — nicht als Ausgangspunkt, wenn Fusion/Steuerung ohnehin beim Agenten liegt.
- **Domänenspezifische Layer nach Bedarf zuschalten:** Code (tree-sitter, ast-grep, LateOn-Code), wissenschaftliche Texte (LaTeX-bewusste Tokenisierung, Formel-zu-Prosa-Informalisierung via LLM, SPECTER2 auf Abstract-Ebene), Prosa (Standard-Hybrid).
- **Evaluation pro Wissensbasis statt global:** Pro KB ein kleines Testset (synthetisch per LLM oder aus echten Agenten-Nachforderungen als implizite Relevanzlabels), Sweep über wenige Regler, Konfiguration ablegen statt Architektur ändern.
- **Logging der Agenten-Entscheidungen ist das eigentliche Steuerungsinstrument:** welche Layer/Parameter gewählt wurden, wie oft nachgefordert oder umformuliert wurde, zeigt empirisch, wo sich teurere Layer lohnen — nicht a priori Architekturentscheidung.
