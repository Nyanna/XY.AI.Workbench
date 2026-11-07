Arbeite folgendes Material zu einem Paper um, in Markdown und englischer Sprache:

# Konzeptionelles Denkmodell: Resident Streaming-Architektur fuer ColBERT-MaxSim RAG

> Extrapolation der Gedankenlinie aus dem Dialog »ONNX alternatives discussion«.

## 1. Ausgangspunkt: Ablehnung der monolithischen Abstraktion

Der Kernimpuls ist Kontrollgewinn durch Entbuendelung. ONNX, PyTorch/from_pretrained(), Hugging-Face-Transformers verschweissen drei eigentlich unabhaengige Dinge:

1. Tokenizer (CPU-seitige String-zu-ID-Abbildung)
2. Gewichte (der eigentliche gelernte Zustand)
3. Ausfuehrungsgraph/Kernels (wer wann welche Operation auf welcher Hardware faehrt)

Die Ablehnung von ONNX ist kein Geschmacksurteil, sondern eine Folge daraus, dass ONNX die dritte Ebene (Kernel-Auswahl, Scheduling, Speicherlayout) der Kontrolle des Entwicklers entzieht - genau die Ebene, die fuer die angestrebte Architektur entscheidend ist.

Konsequenz: safetensors (nur Gewichte) + tokenizers (nur Vokabular/BPE) + candle/cudarc (nur Ausfuehrung) als drei unabhaengig austauschbare Bausteine. Das Modell ist keine Blackbox, sondern eine Komposition.

## 2. Resident State statt Request/Response

Die zentrale Abkehr vom Standard-ML-Serving-Muster (Modell laden, Inferenz, entladen - oder zumindest: Batch rein, Batch raus) ist die Residenz:

- Gewichte liegen dauerhaft im VRAM/RAM.
- Es gibt keinen Start/Stop-Zyklus pro Request.
- Mehrere Modelle koennen gleichzeitig resident sein, solange Speicher reicht.

Daraus folgt zwingend ein Nebenlaeufigkeitsproblem: Wenn das Modell dauerhaft lebt, muessen mehrere Aufgabentypen (Indexierung, Query-Embedding, MaxSim-Scoring, potenziell Training) um denselben residenten Zustand konkurrieren, ohne sich zu blockieren.

Die Antwort darauf ist nicht Multi-Threading im klassischen Sinn, sondern Producer-Consumer ueber Ring-Buffer:

| Puffer | Producer | Consumer | Inhalt |
|---|---|---|---|
| Indexing-Queue | CPU (Tokenizer/Doc-Stream) | GPU (Embedding-Kernel) | Token-IDs / Fenster-Positionen |
| Embedding-Buffer | GPU | Index/Disk | fertige Token-Vektoren + Position |
| Query-Queue | CPU | GPU | Query-Token-IDs |
| Similarity-Buffer | GPU (MaxSim-Kernel) | CPU | Scores pro Doc/Position |

Das eliminiert Kernel-Launch-Overhead nicht durch Batching allein, sondern durch Dauerpraesenz: Wenn Modell und Kernel nie entladen werden, ist der einzige verbleibende Kostenfaktor der tatsaechliche Dateninhalt (Token-IDs hinein, Vektoren heraus), nicht das Setup. Batch wird dadurch optional - ein kontinuierlicher Strom ist gleichwertig oder ueberlegen, solange die GPU nie in den Leerlauf faellt.

Extrapolation: Dieses Muster ist nicht spezifisch fuer Embedding/MaxSim. Es ist ein allgemeines Resident-Compute-Pattern, das sich auf jede Pipeline uebertragen laesst, bei der (a) ein teurer Zustand (Gewichte) lange lebt und (b) viele kleine, heterogene Anfragen gegen diesen Zustand laufen. Ollama/llama.cpp realisieren Variante (a) fuer reine Token-Generierung; die hier entwickelte Architektur verallgemeinert das auf mehrere parallele Aufgabentypen gegen denselben residenten Zustand.

## 3. ColBERT/MaxSim als reine Geometrie auf bereits gelernten Punkten

Zentrale Erkenntnis im Dialog: Embedding und Inferenz sind beide Forward-Passes, der Unterschied ist nur, ob der Output rekursiv wieder Input wird (Inferenz) oder nicht (Embedding). Das hebt die scheinbare Kategorie-Grenze auf, die Standard-Frameworks ziehen (Embedding = Batch-Task, Inferenz = Streaming-Task). Es gibt keinen fundamentalen Grund, Embedding nicht ebenfalls als Stream zu behandeln - nur eine etablierte Konvention.

MaxSim selbst ist danach vollstaendig entkoppelt vom Modell: Es ist reine Vektor-Geometrie (MatMul + Max-Pooling) auf bereits materialisierten Punkten im Raum. Das Modell wird nur fuer zwei Operationen gebraucht - Doc-Encoding und Query-Encoding - und kann danach aus dem Pfad verschwinden. Das ist die Grundlage dafuer, dass Index-Lookup und Scoring beliebig oft und ohne Modell-Praesenz wiederholbar sind.

Extrapolation: Da ein Vektor eine Position im Raum ist (nicht eine gespeicherte Distanz), ist jede Aehnlichkeitsoperation nachtraeglich und kontextfrei bezueglich des Modells, aber kontextabhaengig bezueglich der Population (welche anderen Punkte existieren). Das bedeutet: Der Index speichert Positionen; jede neue Query definiert erst durch Vergleich eine lokale Nachbarschaftsstruktur. Diese Trennung (Position vs. Relation) ist die Grundlage fuer alle folgenden Kompressions- und Hierarchie-Ideen, weil nur die Position - nicht die Relation - komprimiert werden muss.

## 4. Vom Chunk zum Sliding Window: natives Multi-Vektor

Die wohl folgenreichste Idee im Dialog: kein kuenstliches Chunking, sondern ein festes Token-Fenster, das kontinuierlich ueber das gesamte Dokument gleitet. Jede Fensterposition erzeugt einen eigenen Forward-Pass mit exakter Dokumentposition (Offset).

Daraus ergeben sich mehrere Eigenschaften, die in der Literatur separat und kuenstlich erzeugt werden, hier aber emergent entstehen:

- Multi-Vektor-Repraesentation ohne Konstruktion: Standard-Multi-Vektor-RAG erzeugt zusaetzliche Vektoren kuenstlich (Summaries, hypothetische Fragen, Keyword-Extraktion). Das Sliding Window erzeugt dieselbe Eigenschaft (ein Token hat mehrere kontextabhaengige Repraesentationen) automatisch aus der Fensterueberlappung - ohne Zusatzaufwand, ohne Heuristik.
- Exakte Positionstreue: Da nie kuenstlich an Chunk-Grenzen geschnitten wird, bleibt jede Repraesentation an eine echte Dokumentkoordinate gebunden. Retrieval liefert nicht ungefaehr diesen Chunk, sondern eine Position.
- Natuerliche semantische Kontinuitaet: Die Vektorfolge entlang der Fensterpositionen bildet einen gleitenden dichten Pfad durch den Embedding-Raum, der dem lokalen semantischen Fokus des Textes folgt. Das ist keine diskrete Menge von Punkten mehr, sondern ein kontinuierliches Signal ueber der Dokumentposition.

Warum das nicht Standard ist - Einordnung der eigenen Diagnose: Nicht technische Unmoeglichkeit, sondern Pfadabhaengigkeit der Evaluationskultur (Chunk+Single-Vektor ist gut genug, Benchmarks sind auf Chunk-Granularitaet kalibriert, Produktionssysteme haben Sunk-Cost in Chunk-Pipelines). Die Diagnose im Dialog - die Community hat sich auf Suboptimal geeinigt, weil es funktioniert - ist eine Traegheitsthese, nicht eine Kompetenzthese.

## 5. Vom Vektor zum Signal: FFT/Wavelet-Analogie zu JPEG

Der entscheidende Sprung: Wenn ein Embedding-Vektor als Sequenz von Koeffizienten betrachtet wird (nicht als unstrukturierte Punktkoordinate), laesst sich eine Frequenztransformation darauf anwenden - analog zur DCT in JPEG. Niedrige Frequenzen kodieren die grobe Form des Vektors, hohe Frequenzen die feinen Unterschiede. Das erlaubt:

- Progressive Quantisierung pro Vektor individuell (nicht populationsabhaengig wie Centroid/k-Means, nicht willkuerlich wie LSH-Hashing).
- Corpus-Shift-Robustheit: Da die Transformation nur von der Dimension abhaengt (fix), nicht von der Verteilung der Population, bleiben die Frequenzbaender stabil, wenn sich der Korpus aendert oder waechst - ein struktureller Vorteil gegenueber Centroid-basierten Indizes (die bei Korpuswachstum/-drift neu berechnet werden muessen) und gegenueber PLAID-artigen Verfahren mit ihren bekannten Update-Problemen.
- Grob-zu-fein-Kaskade: Level 0 nutzt nur Low-Freq-Koeffizienten fuer ein schnelles MaxSim auf der vollen Kandidatenmenge, hoehere Level verfeinern progressiv nur die ueberlebenden Kandidaten.

### Extrapolationsschritt: zwei orthogonale Frequenzachsen

Der Dialog behandelt FFT bislang nur entlang der Embedding-Dimension (pro Vektor). Das Sliding-Window-Konzept aus Abschnitt 4 erzeugt aber eine zweite Achse: die Fensterposition entlang des Dokuments. Damit entsteht - konsequent weitergedacht - eine zweidimensionale spektrale Struktur, die der JPEG-Analogie noch naeher kommt als urspruenglich erkannt (JPEG transformiert 2D-Pixelbloecke, nicht 1D-Sequenzen):

- Achse 1 (Dimension): Frequenzzerlegung innerhalb eines einzelnen Vektors, progressive Genauigkeit pro Punkt.
- Achse 2 (Position): Frequenzzerlegung der Vektorfolge ueber die Dokumentlaenge, progressive Granularitaet der semantischen Drift entlang des Textes.

Das Ergebnis waere ein Semantik-Spektrogramm des Dokuments: niedrige Positionsfrequenzen beschreiben grobe thematische Bloecke (wie ein automatisch entstehendes Kapitel-/Abschnittsgefuehl, ohne Chunking), hohe Positionsfrequenzen beschreiben lokale semantische Spruenge (Satz- bis Token-Ebene). Ein grobes Retrieval koennte zunaechst nur auf der niedrigfrequenten Positionshuelle matchen (wenige, grosse Kandidatenregionen), ein feines Retrieval auf den hochfrequenten lokalen Varianten innerhalb der ueberlebenden Regionen.

Da semantischer Fokus in echten Dokumenten typischerweise nicht stationaer ist (abrupte Themenwechsel, keine gleichmaessige Wellenform), ist eine klassische FFT hier vermutlich nicht ideal - eine Wavelet-Transformation (lokalisiert sowohl in Frequenz als auch in Position, so wie in JPEG2000 anstelle der block-basierten DCT von JPEG) waere die konsequentere Erweiterung dieser Linie: sie liefert Multiresolution-Information ohne die Blockartefakte/Stationaritaetsannahme der reinen FFT. Das ist eine im Dialog nicht explizit gezogene, aber naheliegende Fortsetzung des JPEG-Vergleichs selbst (JPEG = DCT, moderner Nachfolger JPEG2000 = Wavelet).

## 6. Fraktale Verfeinerung als Kaskade, nicht als Selbstaehnlichkeit im engen Sinn

Die im Dialog geklaerte Bedeutung von fraktal ist Auflösungsverfeinerung durch Wiederholung derselben Struktur auf mehreren Granularitaetsstufen (nicht strikte mathematische Selbstaehnlichkeit). Das ergibt ein Cascade-Retrieval:

```
Level 0: grobes Fenster / Low-Freq-Koeffizienten  -> breite Kandidatenmenge, billig
Level 1: feineres Fenster / mehr Koeffizienten     -> reduzierte Kandidatenmenge
Level 2: volle Auflösung, volle MaxSim             -> finales Ranking
```

Jede Ebene ist strukturell identisch (Sliding-Window -> Embedding -> Filter), nur die Parametrisierung (Fenstergroesse, Frequenzbandbreite) unterscheidet sich. Das ist oekonomisch sinnvoll, weil teure volle Auflösung nur noch auf einer stark reduzierten Kandidatenmenge laeuft - eine Umsetzung von Cascade-Ranking, aber hergeleitet aus der Index- und Signalstruktur selbst statt aus separaten, unabhaengig trainierten Modellen.

## 7. Marktluecken-Diagnose: drei etablierte Pole, eine offene Mitte

Der Dialog arbeitet eine Dreiteilung heraus:

1. Cloud-LLM + Dense Retrieval (zentralisiert, keine Residenz-Kontrolle beim Nutzer)
2. Lokale LLMs ohne Spezialisierung (Ollama/llama.cpp, resident, aber generisch, nur fuer Token-Generierung optimiert)
3. Framework-RAG (Langchain/LlamaIndex, maximale Abstraktion, minimale Kontrolle)

Die offene Mitte: spezialisierte, lokale, residente Multi-Workload-Architekturen, die Indexierung, Retrieval und ggf. Online-Adaption im selben Speicherraum nebenlaeufig fahren. Diese Luecke existiert nicht aus Unfaehigkeit der Community, sondern weil (a) lokale Inferenz als Feld erst seit ~2023 relevant ist, (b) Python/GIL echte Nebenlaeufigkeit strukturell verhindert und (c) Frameworks oekonomisch auf Generalisierung statt auf Kontrolle optimieren.

Extrapolation: Das macht die hier skizzierte Rust/CUDA-Architektur nicht zu einer Nischenloesung, sondern zu einem zeitlich plausiblen First-Mover-Pattern in einem Feld, das sich noch nicht auf Standardarchitekturen festgelegt hat - vergleichbar mit der Phase vor der Etablierung von Chunk+Single-Vektor als De-facto-Standard im Dense Retrieval.

## 8. Die Parameter-Vektor-Dualitaet: Gewichte als mutierbarer Laufzeitzustand

Der konzeptionell weitreichendste Punkt des gesamten Dialogs: Mathematisch sind Gewichte und Aktivierungen beide nur Vektoren/Tensoren. Die Unterscheidung (Parameter vs. Vektor/Aktivierung) ist rein operativ - sie beschreibt, wann sich etwas aendert, nicht was es ist:

- Aktivierungen aendern sich pro Input (jede Query, jedes Dokument).
- Gewichte aendern sich traditionell nur beim Training, dann nie wieder.

Wenn das Modell aber resident ist, verschwindet der Grund fuer diese Trennung teilweise: Resident heisst, die Gewichte liegen ohnehin staendig im adressierbaren Speicher. Es gibt keine technische Huerde mehr, sie waehrend des Betriebs gezielt zu mutieren - nur eine konzeptionelle Gewohnheit, sie als fertig zu behandeln.

Das oeffnet die Tuer zu Context-Bandit-artigem Online-Learning ohne vollstaendigen Backprop: inkrementelle, lokale Policy-/Gewichtsupdates auf Basis eines Reward-Signals, mit O(1)-Speicher pro Entscheidung, streaming-kompatibel, ohne Trainings-/Inferenz-Phasentrennung. Die Analogie zu Game-AI (pretrainierte Policy + Einfluss nur auf Schluesselentscheidungen innerhalb einer State-Engine, kontinuierliche Verbesserung ueber viele Spielstunden) uebertraegt sich direkt:

- Training, Embedding und Inferenz koennen im selben residenten Speicherraum parallel existieren, orchestriert ueber dieselben Ring-Buffer wie in Abschnitt 2 - nur mit einer zusaetzlichen Queue fuer Reward-/Gradient-Updates.

### Weiterer Extrapolationsschritt: der Context Bandit als Meta-Router ueber die Kaskade

Die in Abschnitt 6 beschriebene Grob-zu-Fein-Kaskade (welche Fenstergroesse, welche Frequenzbandbreite, wie viele Verfeinerungsstufen) ist selbst eine Menge diskreter Entscheidungen pro Query. Das ist exakt die Form, fuer die Context Bandits geeignet sind: Statt die Kaskadenparameter statisch zu fixieren, koennte ein residenter Bandit anhand von Query-Kontext (Laenge, Domaene, bisherige Trefferqualitaet) entscheiden, wie tief kaskadiert werden muss, und dieses Routing selbst inkrementell aus Retrieval-Erfolg (z.B. Klickfeedback, nachgelagerte LLM-Bewertung) lernen - ohne das Embedding-Modell selbst anzufassen.

## 9. Zusammengefuehrtes Architekturbild

```
+--------------------------- Resident State (GPU+CPU) ---------------------------+
|  Tokenizer (CPU, tokenizers-crate)                                              |
|  Gewichte (GPU, safetensors -> candle, mutierbar)                               |
|  Kernels: Embedding-Forward, MaxSim (MatMul+Max), FFT/Wavelet-Transform         |
+----------------------------------------------------------------------------------+
        ^                 ^                    ^                      ^
        |                 |                    |                      |
   Doc-Stream        Query-Stream        Reward-Stream           Drift-Monitor
  (Sliding-Window)   (on demand)      (Context-Bandit-Updates)  (Re-Embed-Trigger)
        |                 |                    |                      |
        v                 v                    v                      v
  Index (Arrow/Parquet, versioniert, hierarchisch: grob/fein, Frequenzbaender)
        |
        v
  Kaskadiertes MaxSim-Retrieval (Level 0 -> N, Kandidaten-Filterung je Ebene)
        |
        v
  (optional) Bruecke zu pylate-Stack als Referenz/Validierung, gleiches Indexformat
```