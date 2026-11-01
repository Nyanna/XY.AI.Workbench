## Leitprinzipien

- **Selbstorganisation:** Ein Thread des Processors bedient sich als Lane selbst an der Queue. Die Zuteilung steuert sich über den dynamischen Durchsatz der Lanes: Wer schneller ist, holt sich mehr Arbeit.
- **Entkopplung:** Subengines, Kernel, Modelle und Indizes sind nicht aneinander gekoppelt. Die einzige harte Bindung zwischen Index und Modell ist die Kombination aus Model-ID und Dimension im Index-Header, ergänzt um die Pipeline-Merkmale, die Vektoren vergleichbar machen.
- **Akzeptierte Variablen statt Garantien:**
  - Ergebnisse von Embedding und Similarity sind keine Garantien.
  - Varianz und Indeterminismus sind zugunsten von Auslastung und Performance erlaubt, etwa durch Fusion, Atomics oder hardwareabhängiges Rounding.
  - Stale Ergebnisse sind kein Fehler. Zugunsten konsistenten Durchsatzes gibt es keine Locks auf den Datenpfaden.
- **Keine Qualitätszusage der Engine:**
  - Die Engine rechnet auf der ihr übergebenen Kandidatenmenge. Sie verbürgt weder Vollständigkeit noch Größe der Menge noch Ergebnisqualität.
  - Genauigkeitsverluste durch Chunking, Query-Formulierung, Index-Struktur, Reranking oder Kandidatenanzahl sind Sache der äußeren Schichten.
  - Das Gleiche gilt für verlorene Randtreffer approximativer Prefilter wie Centroid, Fourier-Transformation oder Kompression.
  - Der Similarity-Report liefert Fakten, die Engine trifft keine Entscheidungen und legt keine Thresholds fest.
- **Schlanker Kern:** Der Kern kennt keine Iteration, Kaskade, Rekursion, Parent-Child-Beziehung und keine Zwischenzustände zwischen Tasks. Solches Verhalten entsteht nur durch Addons.
- **Nichts blockiert:**
  - Die Queue kennt kein Head-of-Line-Blocking, keine Reservierung und kein Vorab-Übernehmen.
  - Der Manager blockiert nie einen Kernel.
  - Die Kalibrierung unterbricht oder blockiert den Nutzer nie.
- **Lazy statt Eager:** Ressourcen werden erst bei Bedarf initialisiert. Nach Ablauf einer Frist ohne Nutzung werden sie entladen (Unload), aber nicht deaktiviert. Im VRAM gelten deutlich längere Fristen als im RAM.
- **Klare Ownership:** Alles, was Speicher, Streams oder Kernel besitzt, gehört der Subengine. Externe Libraries sind jeweils an genau einer Stelle als Adapter angebunden. Typen externer Libraries werden nur an diesen Rändern konvertiert.

