## Kernel und Scoring

### Kernel-Register und Laufzeit-Kompilierung

- Das Register liefert zu Name, Datentyp, Architekturstufe und Variante (etwa Dimension) die geladene Funktion. Quellen: Runtime-Compilation (NVRTC) für die Zielarchitektur, vorkompilierter Maschinencode als Fallback bei zu altem Treiber, Kernel der externen Library.
- Laden erst bei Bedarf, gleichzeitige Requests führen nie zu doppelter Kompilierung. Log mit Name, Datentyp, Architektur, Variante und Dauer. Anzahl und Leere abfragbar, Verwerfen aller Einträge (Clear) mit Anzahl. Auffindbar als Ressource unter fester ID.
- Kompilierung aus Quelltext mit gemeinsamem Prelude (Typdefinition, Load-Funktion, Warp-Reduktionen), Auswahl per Name.
- Zielarchitektur aus der Compute Capability: Stufen 6.1, 7.0, 7.5, 8.0, 8.6, 8.9, unbekannte fallen auf 7.5 zurück. Lehnt der Treiber den Zwischencode (PTX) ab, wird gewarnt und als Maschinencode (SASS/cubin) für genau die Device-Architektur neu kompiliert.
- Fehler: Compile-Error mit Log, Module-Load-Error, unbekannter Name oder fehlende Funktion, nicht unterstützter Elementtyp als „nicht verfügbar“. Die Compute Capability wird bei Einrichtung ermittelt, sonst „nicht verfügbar“.
- Zur Kompilierzeit festgelegt: Datentyp, Dimension, Tile-Größe als eigene Varianten. Dynamisch über den Parameter-Block: Slot-Indizes, Zähler, Segmenttabelle, Query-Länge, Head und Tail.

### Unterstützte Precisions

- Je Device-Art ist festgelegt, für welche Elementtypen Scoring-Kernel existieren, unabhängig von aktivierter GPU-Unterstützung: CPU nur FP32, GPU FP32 und FP16.
- Für jeden unterstützten Typ werden die Achsen Operanden und Scoring gemeldet, Typen ohne Kernel nie. Andere Achsen schränkt der Katalog nicht ein.

### Query-Batch, Quelle und Workspaces

- Mehrere Queries gleicher Dimension werden gebündelt, damit jeder Dokumentabschnitt einmal für alle gelesen wird. Jede Query ist eine zeilenweise Matrix aus Token × Dimension, dicht genau ein Token. Prüfung: Dimension ungleich null, nicht leer, Länge Vielfaches der Dimension, sonst Shape-Error. Abfragbar: Dimension, Anzahl, Leere, Tokens und Rohdaten je Query.
- Dokumentvektoren kommen über eine Source-Schnittstelle: Dimension, Slots je Segment, Slot-Offset für Treffer, Vektoren eines Slots (nur gültige Tokens) oder „nicht vorhanden“. Eine RAM-Source berechnet Offsets als Summe der vorangehenden Slot-Bytes (vier Byte je Wert), unbekannte Segmente haben null Slots.
- Vor jedem Scoring wird der Slot-Bereich geprüft, sonst Fehler mit Segment, Grenzen und Slot-Anzahl.
- Ein Workspace ist ein halboffener Slot-Bereich und Einheit eines Kernel-Launches. Länge nie negativ, Leere abfragbar, teilbar in Vorderteil höchstens gewünschter Größe und Rest.

### Dichtes Scoring

- Cosine Similarity als Dot-Product, gültig für Einheitsvektoren. Exakter Modus teilt durch das Normprodukt, für abweichende Längen etwa durch FP16-Speicherung; nicht positiver oder nicht endlicher Nenner ergibt null.
- Jede Query genau ein Token. Leer, wenn das Norm-Quadrat nicht endlich oder unter 10⁻¹⁸ ist: Fehler „leeres Embedding“ mit Query-Index, für alle Queries vor Beginn geprüft. Falsche Slot-Dimension oder leere Slots ergeben Shape-Error.
- Ergebnis je Query ein Score je Slot in Slot-Reihenfolge. Scorer für beide Modi liefern die Top-k-Treffer.

### Compute-Primitive auf der CPU

- Dot-Product mit achtfach parallelen Teilsummen für Auto-Vektorisierung.
- Max-Similarity (Late Interaction): je Query-Token das größte Dot-Product über Dokument-Token, summiert. Ein Dokument ohne Tokens ergibt Mask-Wert × Query-Tokens, wie auf der GPU.
- L2-Normalisierung, Nullvektoren bleiben.
- Slotweiser Scan: Bereich und Dimension prüfen, Slots als äußere Schleife gegen alle Queries, laufendes Maximum je Query-Token ohne Matrix über Slots. Fehlende Vektoren gelten als leer, unteilbare Längen ergeben Shape-Error, Einzelfehler brechen ab.

### Max-Similarity als Operation

- Eine einheitliche Operation liefert die Top-k-Treffer je Query über einen Slot-Bereich und einen Query-Batch, mit austauschbarer Implementierung.
- Tile-Kette aus Library-Operationen auf CPU oder GPU mit wählbarer Precision und Tile-Größe (unter eins wird eins). Nicht abbildbare Precision oder nicht öffnendes Device ergeben „nicht verfügbar“.
  - Dokument-Tokens in Tiles fester Größe zerlegen und transponieren, letztes Tile mit Nullen auffüllen und per additiver Mask ausschließen.
  - Je Tile Matrix-Produkt (GEMM), Mask, Maximum je Query-Token, laufendes Maximum über Tiles, Summe in FP32.
  - Zwischenmatrizen höchstens Query-Tokens × Tile-Größe. Leere Dokumente ergeben Mask-Wert × Query-Tokens.
  - Queries werden vorher in die Precision überführt. Gleiche Prüfungen wie der Scan, Library-Fehler als Launch-Error.
- Fused Kernel hinter derselben Operation, bei dem die Zwischenmatrix nie im VRAM liegt.
- Die Dokumentseite ist maskiert, die Mask-Tokens der Query zählen mit.

### Top-k-Selektion

- Begrenzte Menge je Query aus Offset und Score. Höherer Wert gewinnt, bei Gleichheit niedrigerer Offset. Nicht numerische Werte werden verworfen, bei k gleich null wird nichts aufgenommen.
- Ausgabe absteigend, bei weniger Kandidaten entsprechend kürzer. Anzahl, Soll-Größe und Leere abfragbar, einzeln oder als Folge aufnehmbar.
- Lokale Reduktion je Workspace, Merge der Teilergebnisse auf der CPU. Ein Range-Scorer übernimmt Roh-Scores mit Slot-Offsets, mehrere Bereiche werden nacheinander gescort und gemerged.

### GPU-Kernel

- Zwei Kernel: dichtes Dot-Product und fused Max-Similarity. Dokumente in FP32 oder FP16, FP16 beim Laden auf FP32 konvertiert, Queries und Ergebnisse in FP32.
- Dot-Product: ein Thread-Block je Slot, Teilsummen über Threads und Warps reduziert, Output Queries × Slots.
- Max-Similarity: ein Block je Dokument für genau eine Query. Gepaddete Dokumente mit Längenliste, auf die Padding-Länge begrenzt. Token ohne gültiges Dokument-Token erhalten −10 000.
- Dimension wird bei der Kompilierung festgelegt. Launch mit 128 Threads je Block, ein Block je Slot (mindestens einer), kein dynamischer Shared Memory. Launch-Fehler werden mit Ursache gemeldet.

### Schnittstelle für Kernel-Batchläufe

- Segmente mit Basisadresse, Entry-Anzahl und festem oder variablem Layout (Offset-Tabelle mit Entries plus eins). Ein Batch aus Segmenten, Query-Anzahl, optionalem Parameter-Bereich und Zeitbudget.
- Lebenszyklus: Batch beginnen (Begin) mit ID, auf Inputs ausführen (Run) mit Treffern oder Partial Result, beschreibbare Bereiche zuteilen (vor dem ersten Lauf und nach Partial Results), nie abnehmenden Fortschritt abfragen, Batch beenden (End) mit Rückgabe der Bereiche.
- Die Schnittstelle trägt keine Pointer. Adresse zu Entry: fest per Abstand durch Stride (kein Entry bei Stride null, Adresse vor Basis oder Index über Anzahl), variabel per Offset-Intervall.

### Work-Chunks

- Ein Work-Chunk ist die Planungseinheit mit Zeitbudget. Die Länge zielt auf eine Launch-Dauer von standardmäßig zwei Millisekunden, aus dem exponentiell geglätteten Durchsatz (EMA, Glättungsfaktor 0,3).
- Ohne Messung gilt die Mindestlänge. Die Länge liegt zwischen Mindest- und Höchstlänge (Default 16 und rund eine Million Slots), Mindestlänge mindestens eins. Durchsatz in Slots pro Sekunde ist abfragbar, vorher nicht vorhanden. Beobachtungen mit null Slots werden ignoriert, Dauern auf eine Mikrosekunde nach unten begrenzt.
- Preemption nur an Chunk-Grenzen, geprüft nach jedem außer dem letzten. Mindestens ein Chunk läuft immer vollständig. Query-Tasks können Indexing-Chunks überholen.
- Abbruchgründe: Zeitbudget erschöpft (Vorrang) oder Preemption. Ergebnis: erledigte Slots, Restbereich, Grund. Fehler beenden sofort.
- Ein Tile wird gegen alle wartenden Queries gescort, um die Lesekosten zu amortisieren.

### Launch-Modi

- Launch-Modus und Scorer sind getrennt. Modi: direkter Launch, Graph-Capture für Replay (CUDA Graphs), Persistent Kernel mit Ring-Buffer.
- Verfügbar ist der direkte Launch: Work-Item unverändert an die Lane, Lane-Fehler als Kernel-Error, Tracing auf Trace-Level. Die anderen Modi werden mit „nicht verfügbar“ abgelehnt, mit dem Grund, dass Graph-Capture bzw. Persistent-Modus nicht aktiviert ist.
- Launch-Key: Operation, Shape-Klasse, beteiligte Regionen mit Generation, Datentyp, Variante, Lane-Klasse. Geänderte Generationen ergeben neue Keys, stale Captures laufen nie.
- Shape-Klasse ist die nächsthöhere Zweierpotenz, mindestens eins, bei Overflow 2³¹.
- Vor dem ersten produktiven Launch ist eine Warm-up-Phase möglich, der erste Launch gilt als nicht produktiv. Ohne eigene Umsetzung geschieht nichts.

### Vergleich mit einem Referenz-Scorer

- Ergebnisse werden ohne exakte Gleichheit verglichen, absolute Toleranz 10⁻⁵ bei FP32 und 10⁻³ bei FP16.
- Abweichungsarten: unterschiedliche Trefferanzahl (keine weiteren Ränge dieser Query), Wertabweichung mit Rang und Differenz, Rangabweichung mit Offsets. Keine Rangabweichung bei Beinahe-Gleichstand benachbarter Referenzwerte.
- Beide Scorer laufen über dieselben Workspaces, Abweichungen werden als Warnung mit Segment geloggt und führen nie zum Abbruch. Fehler der Scorer werden weitergereicht.

