# Resident – Requirements

## Zweck und Rahmen

- Resident ist eine dauerhaft laufende Engine für zwei Aufgabenarten:
  - Embedding: Texte werden in Vektoren umgewandelt.
  - Similarity: Ähnlichkeitssuche gegen einen Index.
- Eine RAG-Anwendung (Retrieval-Augmented Generation) startet die Engine. Sie ist als Server-Prozess, als Daemon oder über die CLI nutzbar.
- Gerechnet wird in zwei getrennten Subengines, eine für die CPU und eine für die GPU (CUDA). Beide holen ihre Arbeit selbstorganisiert aus Queues (Pull-Prinzip).
- Nach der Initialisierung ist die Engine bereit, verbraucht aber noch keine Ressourcen.

### Zielumgebung

- GPU: Nvidia GeForce GTX 1660 SUPER, Compute Capability 7.5 (Turing), Treiberversion 610.57.04.
- CUDA-Toolchain: Release 13.3, Version 13.3.73, Build 38244171_0.
- GPU-Kernel werden zur Laufzeit (JIT) für die tatsächlich vorhandene Architektur kompiliert, auf der Referenzkarte also für SM 7.5.
- Turing genügt für die geforderten Loads mit Acquire-Semantik, die ab Compute Capability 7.0 verfügbar sind.
- Die Referenzkarte gehört zur Baureihe ohne Tensor Cores. Ein Tensor-Core-Pfad und BF16 als Compute-Pfad werden daher nicht angeboten.

### Modelle, Formate und Abhängigkeiten

- Das Modell multilingual-e5-small liegt als Repository unter /home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small.
- Die Modelle bilden einen festen Satz und werden vor der Initialisierung der Engine konfiguriert.
- Modelle und Tokenizer werden in kompatiblen Formaten geladen bzw. zum Laden bereitgehalten. Die Gewichte liegen im Format safetensors vor.
- Persistente Indizes liegen im Spaltenformat Parquet mit Arrow-Schema vor.
- Alle Dependencies sind bereits geprüft und in der Manifest-Datei des Projekts eingetragen: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml. Die externe Tensor-/Neural-Network-Library ist Candle.

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

## Komponentenstruktur

Die Wurzel gliedert sich in die Engine und gemeinsam genutzte Bausteine.

- Engine: die Fassade (Facade) nach außen.
  - Processor: Orchestrierung mit Workern, Queues, Notification und Backpressure.
  - Planner: ordnet Operationen den Queues zu.
    - Expander: zerlegt Tasks in Operationsketten.
    - Operation: Operationsarten der Embedding- und Similarity-Ketten.
  - Subengine: Ressourcen- und Speicherverwaltung je Hardwareart.
    - CPU: Device mit Ressourcen (Cores) und Lanes. Speicher mit Kernel, Index (Entry, Datei im Parquet-Format, Manager), Tokenizer und Normalizer.
    - GPU: Device mit Ressourcen (Compute Units, Streaming Multiprocessors) und Lanes. Speicher mit Kernel und Index (Segmenttabelle, Segment, Manager).
    - Gemeinsam: Modell.
  - Task: Embedding-Task und Similarity-Task.
- Gemeinsam: Queue.

## Typischer Anwendungsfall

Der Ablauf folgt dieser Reihenfolge:

- Die Anwendung startet, die Engine wird instanziiert und konfiguriert. Indizes können registriert werden, werden aber weder geladen noch verwendet.
- Die Engine wird initialisiert. Der interne Executor des Processors startet die Mindestzahl idle Threads.
- Die Engine ist bereit, verbraucht aber noch keine Modell- oder Device-Ressourcen.
- Tasks für verschiedene Aufgaben werden über die Engine enqueued.
- Der Expander im Planner zerlegt die Tasks in Operationsketten. Der Planner stellt fest, welche Subengines konfiguriert sind, und startet sie.
- Benötigt eine Operation bestimmte Ressourcen, wird das den Subengines mitgeteilt. Diese registrieren kompatible Ressourcen (Capabilities) in den ihnen zugewiesenen und in den gemeinsamen Queues.
- Der Planner weist die Operationen anhand der kompatiblen Capabilities den Queues zu, vorzugsweise der gemeinsamen.
- Sobald eine aktivierte Lane frei ist, zieht sie eine Operation und initialisiert die Ressourcen:
  - Tokenizer und Normalisierung werden lazy initialisiert.
  - Kernel werden lazy kompiliert und geladen.
  - Indizes und Modelle werden geladen.
  - Wenn sinnvoll, werden weitere Operationen, die mit demselben Index oder Modell laufen können oder müssen, zu einem Batch aggregiert.
- Ergebnisse werden bereitgehalten und können abgeholt werden, per Event oder Notification.
- Nach einer bestimmten Zeit werden unbenutzte Ressourcen entladen, aber nicht deaktiviert. Die Unload-Fristen für Modelle und Indizes im VRAM sind wesentlich länger als im RAM.
- Normalisierung bezeichnet die zum Tokenizer gehörende. Zusätzlich kann es eine externe, vorgeschaltete Textnormalisierung für die Chunks geben.

## Engine

### Konfiguration und Start

- Die Engine wird über einen Builder erzeugt, der erst mit einem expliziten Start-Aufruf wirksam wird.
- Die Angabe eines Modells ist verpflichtend. Fehlt sie, scheitert der Start mit einem Konfigurationsfehler, der das fehlende Feld benennt.
- Die Engine konfiguriert den Planner mit Subengines, Modellen und Indizes.
- Modell- und Device-Ressourcen werden beim Start nicht erzeugt, sondern erst durch den ersten Task angelegt.
- Die Engine bietet Methoden zum Enqueuen der beiden Task-Arten Embedding und Similarity.

### Verwaltung von Indizes

- Beliebig viele durchsuchbare Indizes können registriert werden. Jeder erhält eine eindeutige ID.
- Das Registrieren lädt den Index nicht. Unmittelbar danach gilt er als nicht geladen. Das Laden löst erst ein Task aus.

### Shutdown

- Die Engine bietet ein geordnetes Shutdown, das auch beim automatischen Drop der Instanz abläuft.
- Zuerst werden die Worker angehalten und gejoint.
- Danach werden alle Tasks abgebrochen, die kein Worker übernommen hat. Wartende Aufrufer erhalten den Fehler „cancelled“.
- Das explizite Shutdown meldet bei Erfolg eine positive Rückmeldung.

## Task

### Aufbau, Zustände und Ergebnis

- Ein Task kapselt Daten, Parameter, Ergebnis, Zustand und Operationen. Die Operationen verweisen zurück auf ihren Task.
- Priorität, kompatible Ressource, Modell und Modus sind gewöhnliche Parameter (Metadaten). Die Queue kennt sie nicht.
- Der Expander zerlegt jeden Task in eine Kette von Operationen.
- Zustände eines Tasks: enqueued, claimed, completed, cancelled.
- Das Ergebnis wird genau einmal gesetzt und ist entweder ein Output oder ein Fehler. Wartende werden über eine Notification geweckt.

### Task-Handle und Aufrufer

- Der Zustand ist ohne Warten abfragbar (Polling):
  - completed: das Ergebnis, also Output oder Fehler,
  - cancelled: der Fehler „cancelled“,
  - enqueued oder in Bearbeitung: „noch kein Ergebnis“,
  - completed ohne verfügbares Ergebnis: gilt als cancelled.
- Der Aufrufer kann unbegrenzt oder mit Timeout auf den Abschluss warten. Er wartet auf das Ergebnissignal und wacht zusätzlich periodisch auf, um den Zustand zu prüfen.
- Reihenfolge beim Warten: erst die Registrierung für die Notification aktivieren, dann den Zustand lesen, dann warten. So geht keine Notification verloren.
- Der Timeout ist die eigene Deadline des Aufrufers. Bei Überschreitung wird der Fehler „timeout“ geliefert und das Handle freigegeben.
- Wird ein unfertiges Handle gedroppt, wird der Task abgebrochen, sofern ihn noch kein Worker übernommen hat. Übernommene Tasks laufen weiter. Der Zustand „cancelled“ wird lokal abgeleitet.

### Erweiterbare Zusatzparameter

- Das Parameterobjekt jeder Task-Art ist erweiterbar und nimmt beliebige zusätzliche, typgebundene Werte auf (typindizierte Extension-Map).
- Je Typ ist höchstens ein Wert gespeichert. Erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
- Ein Wert ist nach Typ abfragbar (sonst „nicht vorhanden“) und nach Typ entfernbar, mit Rückmeldung, ob er vorhanden war.
- Anzahl der Werte und Leere der Map sind abfragbar.
- Beim Klonen der Parameter werden die Werte geteilt (shared) statt dupliziert, damit abgeleitete Tasks (etwa von Addons) die Parameter günstig übernehmen.
- Die Werte werden unverändert durch Queue und Planung gereicht und sind thread-safe übertragbar (Send + Sync).

### Embedding-Task

- Input: ein Text oder eine Liste von Texten.
- Parameter: Modell, Modus (Query oder Passage), Precision, optionaler Ziel-Index, erweiterbare Zusatzparameter.
- Ergebnis: die Embeddings als Liste von Float-Vektoren, ein Vektor je Eingabetext in Eingabereihenfolge.
- Ein Embedding- bzw. Indexing-Task kann so konfiguriert werden, dass er das Embedding samt Metadaten direkt in den Index schreibt. Er liefert dann Entry-Referenzen bzw. IDs des Index.

### Similarity-Task

- Input: eine Liste von Query-Vektoren. Eine Textanfrage wird als Folge aus einem Embedding-Task im Query-Modus und einem Similarity-Task mit dem Ergebnisvektor ausgedrückt, gesteuert durch die Anwendung oder ein Addon.
- Parameter:
  - ID des zu durchsuchenden Index (verpflichtend), daraus abgeleitet das Modell,
  - maximale Trefferzahl (Top-k, Default zehn),
  - Scope (Suchbereich), Default ist der gesamte Index, einschränkbar auf eine Teilmenge,
  - Scoring-Verfahren, Default ist Cosine Similarity,
  - Precision und erweiterbare Zusatzparameter.
- Ein Reranking der Treffer ist keine eigene Task-Art. Es wird als zweite Similarity mit anderem Scoring-Verfahren auf einem verengten Scope ausgedrückt, dessen Kandidaten aus einem früheren Ergebnis stammen.
- Ergebnis: Entry-Referenzen oder IDs (Chunk, Zeile, Bereich, Datei oder Index) mit Score. Nach außen erscheint jeder Treffer mit Entry, Score und Markierung „stale“.

### Ergebnisarten

- Das Ergebnis eines Tasks ist genau eine Ergebnisart:
  - Embeddings als Liste von Float-Vektoren,
  - Entry-Referenzen bzw. IDs beim direkten Schreiben in den Index,
  - Treffer als Liste von Paaren aus Entry-ID und Score,
  - ein Similarity-Report,
  - eine Liste von Scores (etwa aus einer Cross-Encoder-Inferenz).
- Die Menge der Ergebnisarten ist erweiterbar. Ergebnisse sind auf Gleichheit vergleichbar.

### Similarity-Report

- Der Report liefert rohe Scores und die Fakten, anhand derer eine äußere Schicht über Folge-Tasks entscheidet. Er enthält Task-ID und Suchparameter.
- Inhalt:
  - Anzahl der Kandidaten, also der tatsächlich gescorten Entries im aufgelösten Scope,
  - Anzahl der verworfenen Entries,
  - Anzahl der stale Entries unter den zurückgegebenen Treffern,
  - Rangliste der Treffer mit Entry-ID, Score, Flag „stale“ und optionalem Source-Link.
- Aufbau:
  - Eingehende Treffer dürfen unsortiert sein und mehr Einträge enthalten als die maximale Trefferzahl.
  - Treffer ohne gültigen Zahlenwert werden verworfen und zur Zahl der upstream verworfenen Entries addiert, etwa wenn ein Entry aus dem Index entfernt wurde.
  - Sortierung absteigend nach Score, bei Gleichstand steht die niedrigere Entry-ID vorn. Danach wird auf Top-k gekürzt.
  - Das Flag „stale“ wird je Treffer übernommen und nur über die zurückgegebenen Treffer gezählt.
  - Der Source-Link wird je zurückgegebenem Treffer über eine vom Aufrufer gelieferte Lookup-Funktion ermittelt und kann fehlen.
  - Fehlen die Suchparameter, endet der Task mit einem Fehler.
- Kennzahlen beziehen sich auf die zurückgegebenen Treffer. Ohne Treffer sind sie nicht vorhanden:
  - Anzahl, bester und schlechtester Score, arithmetisches Mittel, Standardabweichung der Grundgesamtheit.
  - Abstand zwischen erstem und zweitem Treffer (Gap). Ein kleiner Abstand zeigt eine unsichere Top-Reihenfolge an. Mindestens zwei Treffer nötig.
  - Abstand zwischen erstem und letztem Treffer als Maß der Streuung (Spread). Mehr als ein Treffer nötig.
  - Abstand des besten Scores vom Mittelwert in Standardabweichungen (z-Score). Bei einer Standardabweichung nahe null nicht vorhanden.
  - Wahrscheinlichkeitsverteilung über die Scores per Softmax mit einstellbarer Temperatur. Die Temperatur muss endlich und größer als null sein, sonst ist das Ergebnis nicht vorhanden. Die Berechnung ist numerisch stabil, indem sie auf den besten Score bezogen wird. Für die verwendete Modellfamilie liegen Scores in einem engen Band, daher ist eine Temperatur von etwa 0,01 bis 0,05 sinnvoll.
  - Wahrscheinlichkeitsmasse des besten Treffers.
  - Entropie der Verteilung in Nats. Null bedeutet einen eindeutigen Gewinner.
  - Knee-Point der sortierten Score-Kurve: der Treffer mit dem größten Abstand unterhalb der Verbindungslinie vom ersten zum letzten Wert. Mindestens drei Treffer nötig. Bei flacher Kurve oder ohne Punkt unterhalb der Linie nicht vorhanden. Bei Gleichstand gewinnt der frühere Treffer.

## Planner, Expander und Operation

### Planner

- Der Planner nimmt Tasks entgegen, lässt sie vom Expander zerlegen und enqueued die Operationen in passende Queues. Findet sich keine passende Queue, scheitert das Enqueuing mit dem Fehler „fehlende Planungs-Capability“.
- Es gibt getrennte Operations-Queues für GPU, CPU und gemeinsam. Die Zuordnung richtet sich nach den kompatiblen Capabilities der Subengines. Bevorzugt wird die gemeinsame Queue.
- Der Planner verwaltet je Subengine, welche Ressourcenarten sie anbietet. Die Angebote sind schrittweise erweiterbar.
- Beim Erzeugen einer Operation aktiviert der Planner die benötigten Subengines blockierend:
  - Diese aktivieren ihre Ressourcen samt Lanes ebenfalls blockierend, ausschließlich als Aktivierung ohne Laden.
  - Die Lanes registrieren die aktivierten Ressourcen in den Queues.
  - Das Laden schwerer Ressourcen (Modelle, Indizes, Kernel) geschieht ausschließlich lazy durch die Iteration der Subengine.
- Unterschieden wird zwischen verfügbarer Ressource (registrierte Capability) und geladener Ressource (initialisiert und im Speicher).
- Für jede Task-Art ist vorab abfragbar, welche Ressourcenart die erste Operation benötigt: bei Similarity der Scorer, bei Embedding und Inferenz der Tokenizer. Der Processor nutzt das für die Expansion nach Idle-Notification.

### Expander und Planungsprinzip

- Der Expander nimmt einen Task (Daten und Parameter) und zerlegt ihn genau einmal in eine geordnete Kette von Operationen.
- Er übersetzt die Task-Parameter in die konkrete Konfiguration der Operationen: Index, Tokenizer, Modell, Scorer, Pooling, Präfix, Zieldimension, Top-k.
- Er arbeitet nur mit Konfigurationsregeln, den Capabilities der Subengines und der Idle-Notification. Ohne Notification gilt eine leere Notification.
- Er ist eine reine Funktion: zustandslos und wiederholbar, gleiche Eingabe ergibt dieselbe Kette. Das Ergebnis hängt nicht von den Nutzdaten ab. Er hat keine Queues, keine Worker, keine Notification und startet keine Subengines.
- Inputs:
  - der Task mit Daten und Parameterobjekt,
  - die Pipeline-Konfiguration pro Modell: Präfixe, Pooling, Normalisierung, maximale Sequenzlänge, Datentyp.
- Erhält ein Expander einen Task einer fremden Art, lehnt er ihn mit einer Meldung ab, die beide Task-Arten nennt.
- Task-Art und Anzahl der geplanten Operationen werden auf Debug-Level geloggt.

### Modellauswahl

- Embedding- und Similarity-Tasks verwenden das konfigurierte Default-Modell. Die Kompatibilität mit einem Index ist über den Index-Header gesichert.
- Ist das gewählte Modell nicht konfiguriert oder fehlt für die Kombination aus Modell und Task-Art ein Expander, wird mit „unbekanntes Modell“ unter Nennung des Modells abgebrochen.
- Modell, Modellkonfiguration, Subengine-Angebote und Idle-Notification stehen dem Planner gemeinsam zur Verfügung.

### Expansionsregeln

- Embedding, in dieser Reihenfolge:
  - Tokenisierung (CPU, Tokenizer-Ressource),
  - Embedding-Berechnung (CPU oder GPU, Encoder-Ressource), mit konfigurierter Zieldimension,
  - Pooling (Encoder-Ressource), entfällt beim Verfahren „none“, dann bleiben die Token-Vektoren erhalten,
  - L2-Normalisierung auf Länge eins (Encoder-Ressource), entfällt bei abgeschalteter Normalisierung,
  - Schreiben in den Index (Index-Ressource, referenziert den Ziel-Index), entfällt ohne Ziel-Index.
- Similarity, zwei Operationen:
  - Similarity-Berechnung über den Scope mit Zieldimension,
  - Top-k-Selektion mit Top-k, begrenzt auf den Maximalwert von u32.
  - Beide fordern die Scorer-Ressource des Modells an und referenzieren den Index.
  - Ein Top-k von null wird mit einem Konfigurationsfehler abgelehnt.
- Similarity mit Cross-Encoder: Paare aus Query und Text aus der Payload, Cross-Encoder-Inferenz, Scores. Der Scope stammt aus einem früheren Ergebnis. Ist kein Cross-Encoder-Scorer verfügbar, wird der Task mit „Scoring-Verfahren nicht verfügbar“ unter Nennung des Scorers abgelehnt.

### Schrittweise Übergabe und Ergebnisaufbereitung

- Zunächst wird nur die erste Operation der Kette bereitgestellt, zusammen mit einer Collector-Funktion für den endgültigen Output.
- Nach Abschluss einer Operation wird die nächste erzeugt. Ihr Input ist der tatsächliche Output der vorigen. Jede Operation bleibt dem Kontext ihres Tasks zugeordnet. Nach der letzten Operation folgt keine weitere.
- Liefert die letzte Operation ein Ergebnis im RAM, wird es als Gesamtergebnis gespeichert, sonst bleibt es leer. Eine leere Kette stellt keine Operation bereit und endet beim Einsammeln mit dem Fehler „fehlendes Ergebnis“.
- Die Collector-Funktion entnimmt das Ergebnis (take), sodass es nur einmal verfügbar ist.
- Input-Aufbereitung:
  - Embedding: die Textliste ist Input der ersten Operation, die Textanzahl wird für das Splitting des Ergebnisses gemerkt.
  - Similarity: alle Query-Vektoren werden zu einer flachen Folge von Floats zusammengefasst.
  - Inferenz: Query und Textabschnitt werden als geordnetes Textpaar übergeben.
- Ergebnisaufbereitung:
  - Embedding: die flache Folge wird in einen Vektor je Text aufgeteilt. Ist die Textanzahl null oder die Wertanzahl nicht restlos teilbar, endet der Task mit einem Fehler, der Wert- und Textanzahl nennt.
  - Similarity mit einfacher Trefferliste: Paare aus Entry-ID und Score.
  - Similarity mit gescorten Treffern: Similarity-Report.
  - Inferenz: die Floats als Scores.
  - Passt das Ergebnis nicht zur Task-Art oder fehlt es, wird ein Fehler mit Nennung der Task-Art bzw. der fehlenden Ausgabe gemeldet. Ergebnisfehler gelten als Konfigurationsfehler der Planung.

### Operation

- Operationsarten (erweiterbar): Tokenisierung, Embedding, Pooling, L2-Normalisierung, Similarity-Berechnung, Top-k-Selektion, Schreiben in den Index, Inferenz, explizite Übertragung vom Device ins RAM.
- Jede Operation nennt Art, Parameter und Resource-Requirements, wobei die Requirements aus den Parametern folgen. Operationen sind von mehreren Threads gleichzeitig verwendbar.
- Parameter enthalten Resource-Requirements sowie optional Top-k und Dimension (beim Anlegen nicht gesetzt). Sie sind immutable, auf Gleichheit vergleichbar, als Key für Caches oder Dependency-Graphen verwendbar und dienen zum Filtern vor dem Claim.
- Eine Operation lässt sich als reine Beschreibung (Descriptor) aus Art und Parametern darstellen, ebenfalls vergleichbar und als Key nutzbar.
- Priorität und Task-ID sind Metadaten.
- Eine Operation gehört zu genau einem Task. Der Graph ergibt sich implizit aus der Reihenfolge der Operationen.
- Jede Berechnung ist eine Operation mit expliziten Input- und Output-Regionen. Input und Output sind entweder Daten im RAM oder ein Pin einer Device-Region, nie rohe Tensoren oder Pointer.
- Datenformen im RAM (erweiterbar): Textliste, Liste von Tokenfolgen, Float-Folge, Trefferliste aus Entry-ID und Score, gescortes Trefferergebnis.
- Ein gescorter Treffer enthält Entry-ID auf Manager-Ebene, Score, Flag „stale“ und optionalen Source-Link.
- Ein gescortes Trefferergebnis ist ein Zwischenergebnis mit Anzahl gescorter Entries im aufgelösten Scope, Anzahl bei der Auflösung verworfener Treffer (Entry entfernt) und verbleibenden Treffern. Ranking und Zusammenfassung erfolgen danach.
- Ergebnis der Similarity-Operation: Scores sowie die Grundlage für Anzahl, Gap, Spread, Entropie und Knee-Point.

### Resource-Requirements

- Modelle und Indizes werden durch String-IDs, Devices durch eine Zahl identifiziert.
- Ressourcenarten (erweiterbar): Tokenizer, Normalizer, Encoder, Scorer, Index, Cross-Encoder.
- Wahl der Execution-Umgebung (erweiterbar): CPU-only, GPU-only, any. „Any“ akzeptiert jede Device-Art, die anderen nur die passende.
- Placement-Hint (Affinity, erweiterbar) auf eine Region oder ein Device, als Hinweis für Pull-Filter.
- Requirements enthalten Ressourcenart, optional Modell, optional Index, Execution-Umgebung, geforderte Capabilities und optional Placement-Hint. Beim Anlegen sind nur Ressourcenart und Execution-Umgebung verlangt, die übrigen Angaben sind einzeln ergänzbar.
- Requirements sind vergleichbar und als Key verwendbar.
- Ein Device erfüllt die Requirements genau dann, wenn seine Art akzeptiert wird und seine Capabilities alle geforderten abdecken. Der Placement-Hint spielt dabei bewusst keine Rolle.

### Materialisierung

- Das Materialization-Flag legt fest, ob das Ergebnis beim Fertigwerden ins RAM zurückgeholt wird oder als Referenz im VRAM bleibt.
- Das Ergebnis bleibt nur dann im VRAM, wenn diese und die unmittelbar folgende Operation beide ausdrücklich auf der GPU laufen.
- In allen anderen Fällen wird materialisiert: Nachfolger auf der CPU, Task-Ende, Mirroring in den Index, freie Wahl der Execution-Umgebung.
- Der Planner legt das Flag fest, die Operation führt es aus.

### Task-Kontext, Folgeoperationen und Abschluss

- Je Task gibt es einen gemeinsam nutzbaren Kontext mit ID, Priorität, Zähler offener Operationen, Cancel-Flag und erstem Fehler. Bei Erstellung wird die anfängliche Anzahl offener Operationen angegeben.
- Alle Zustände sind nebenläufig korrekt lesbar und veränderbar.
- Folgeoperationen werden registriert, bevor sie enqueued werden und bevor die erzeugende Operation abschließt, damit der Zähler nicht vorzeitig null erreicht. Sie werden am Ende der Queue enqueued.
- Der Abschluss einer Operation verringert den Zähler und meldet, ob es die letzte offene Operation eines nicht abgebrochenen Tasks war. Nur dann ist der Task erfolgreich completed.
- Ein Abschluss ohne offene Operation ist ein Programmierfehler und wird in Entwicklungsumgebungen (Debug-Builds) erkannt.
- Der erste Fehler setzt den Task auf „cancelled“ und das Ergebnis auf Fehler. Spätere Fehler überschreiben ihn nicht. Es wird zurückgemeldet, ob es der erste Fehler war, und der Abbruch wird geloggt. Die übrigen Operationen scheitern beim Claim.

### Operationsknoten

- Ein Knoten bündelt Operation, Input, zunächst leeren Output, Materialization-Flag, zugehörigen Task und optional einen Continuation-Handler.
- Beim Anlegen werden Task-ID, Operationsart, Ressourcenart und Materialization-Flag geloggt.
- Ein Continuation-Handler ist nachträglich anfügbar. Er erzeugt aus dem Output und dem Task-Kontext null oder mehr Folgeknoten. Ohne Folgeknoten endet die Kette und der Output wird Task-Ergebnis. Continuation-Handler sind nebenläufig verwendbar.
- Der Output ist einmal setzbar. Ein zweiter Versuch gibt den Wert unverändert an den Aufrufer zurück.

### Pinning von Device-Regionen

- Der Zugriff auf eine Device-Region wird über ein Pin abgebildet. Solange mindestens ein Pin besteht, darf die Region nicht evicted werden.
- Je Region gibt es einen gemeinsam genutzten Pin-Count, anfangs null, nebenläufig korrekt. Zählerstand und Zustand „unpinned“ (Zähler null) sind abfragbar.
- Erzeugen eines Pins erhöht den Zähler, sein Ende verringert ihn automatisch (RAII) ohne manuelles Freigeben.
- Optional meldet ein Rückmeldekanal die freigegebene Region. Ist der Empfänger nicht mehr erreichbar, endet das Pin trotzdem fehlerfrei.
- Jedes Pin liefert seine Region und ist auch unter der Bezeichnung Device-Handle verfügbar.

## Processor

### Struktur

- Der Processor besitzt die Task-Queue, drei Operations-Queues (CPU, GPU, gemeinsam), den Parkplatz der Worker (Worker-Pool), die Notification und die Idle-Notifications.
- Er ruft den Planner für die Expansion.
- Der interne Executor startet bei der Initialisierung eine Mindestzahl idle Threads (Worker).
- Eine Subengine prüft zuerst ihre eigene Operations-Queue, danach die gemeinsame.
- Subengines besitzen ihre Ressourcen: Tokenizer, Encoder, Scorer, Index-Segmente. Der Tokenizer ist eine Ressource der CPU-Subengine.

### Worker

- Die Zahl der Worker der gesamten Engine ist die Zahl der CPU-Cores minus eins. Das ist eine Obergrenze für Context-Switches, keine Core-Zuordnung (kein Pinning).
- Worker parken im Processor und wachen bei jeder Zustandsänderung per Notification auf.
- CPU: Der Worker führt die Operation synchron aus und kommt mit dem Ergebnis zurück.
- GPU: Der Worker startet die Operation bzw. den Kernel und kehrt zurück. Nach der Completion-Notification verarbeitet ein freier Worker das Ergebnis.
- Blockiert ein Worker, etwa in der Compute-Library oder deren Thread-Pool, ist das das natürliche Backpressure-Signal ohne Sonderbehandlung.

### Worker-Loop

Nach jeder Operation wird in dieser Prioritätsreihenfolge neu geprüft:

- GPU-Ergebnisse verarbeiten und in den Index zurückspiegeln (Mirror-Back).
- GPU-Ressourcen füttern: Index ins VRAM synchronisieren, bei freier Lane starten.
- CPU-Ressourcen füttern, ausführen, Ergebnis zurückgeben, auch an Indizes.
- Neue Tasks expandieren (Planner).

### Completion auf der GPU

- Der Callback am Ende des Streams legt nur einen Completion-Eintrag ab und löst die Notification aus. Im Callback erfolgt kein Aufruf der GPU-API.
- Der Schritt zur Ergebnisverarbeitung prüft zusätzlich per Event-Query und übernimmt eine fertige Lane per atomarem Swap. Das deckt verpasste Callbacks ab.

### Backpressure und Expansion

- Eine Iteration ohne Treffer bei freier Ressource erzeugt eine Idle-Notification mit Ressourcenart und Parametern.
- Die Notifications liegen als Set je Subengine-Art vor, nicht als Queue. Sie verfallen bei einem Treffer oder wenn die Ressource belegt ist.
- Expandiert wird nur bei einer Idle-Notification oder leerer Operations-Queue. Bevorzugt werden Tasks, deren erste Operation die unterversorgte Ressource braucht.

## Queue

### Zweck und Einsatz

- Die Queue ist eine lock-free MPMC-Struktur (Multi-Producer, Multi-Consumer) für Arbeitseinheiten mit eigenem Zustand, Ergebnis und Notification.
- Sie wird viermal eingesetzt: als Task-Queue und als Operations-Queue für CPU, GPU und gemeinsam. Jede arbeitet unabhängig mit eigenem Set von Notification-Channels.
- Typen von Metadaten, Payload, Ergebnis und Fehler legt der Aufrufer fest. Operations-Queues verwenden denselben Aufbau mit eigenen Metadaten und Werten als Payload.
- Bei Operationen wird die Fertigstellung nicht über das Ergebnis der Zustandseinheit verfolgt, sondern über den Zähler offener Operationen des Tasks.

### Struktur

- Jede Queue ist genau eine einfach verkettete Liste mit internem Sentinel-Knoten, ohne getrennte Listen je Priorität oder Ressource.
- Es gibt nur Append und Remove. Append geschieht per atomarem Compare-and-Swap am Next-Pointer des letzten Knotens, Unlink ebenfalls per Compare-and-Swap auf dem Pointer.
- Die Reihenfolge entspricht dem Alter, also dem erfolgreichen Append. Die Reihenfolge eines einzelnen Producers bleibt erhalten.
- Es gibt keine Version, keinen Snapshot und keinen Lock auf der Kette. Nur das Payload-Fach eines einzelnen Knotens ist kurzzeitig gegen gleichzeitigen Zugriff geschützt.
- Metadaten sind für die Queue opak. Sie werden nicht interpretiert, sondern Consumern zum Filtern bereitgestellt.

### Aufbau eines Knotens

- Immutable Parameter zum Filtern vor dem Claim.
- Gemeinsames Zustandsobjekt des Tasks, zugleich Handle des Aufrufers, mit atomarem Zustand, einmal setzbarem Ergebnis und Notification für Wartende.
- Geschütztes, optionales Payload-Fach, das beim Claim geleert wird.
- Atomar austauschbarer Pointer auf den Nachfolger.

### Enqueue

- Beliebig viele Einträge, jeder aus Metadaten, Payload und Zustandseinheit. Der Aufrufer erhält eine Referenz auf den neuen Eintrag.
- Enqueue blockiert nie. Mehrere Producer enqueuen gleichzeitig, ohne dass ein Eintrag verloren geht oder überschrieben wird.
- Ist der interne Tail-Hint veraltet, wird das tatsächliche Ende gesucht und die Reihenfolge gewahrt. Zu Testzwecken lässt sich das Aktualisieren des Tail-Hints gezielt unterdrücken.
- Beim Anlegen nimmt die Queue eine beliebige, auch leere Menge von Notification-Channels entgegen, je Subengine einen. Jedes Enqueue signalisiert alle Channels.

### Iteration

- Es gibt eine Iteration über alle Einträge in Altersreihenfolge und eine, die nur aktive Einträge liefert (weder completed noch cancelled). Der Sentinel erscheint nie.
- Iterationen blockieren nie und bleiben bei gleichzeitigem Enqueue, Claim und Completion gültig. Jeder gelieferte Eintrag bleibt gültig, auch nach logischem Entfernen.
- Die Iteration liefert exakt die Einträge in Enqueue-Reihenfolge, ohne Auslassung und Wiederholung. Eine leere Queue ergibt eine leere Iteration.
- Completed und cancelled Einträge gelten als logisch entfernt und werden in der aktiven Iteration übersprungen. Die vollständige Iteration liefert sie, solange sie nicht entlinkt sind.

### Zustandsmodell und Übergänge

- Vier Zustände: enqueued, claimed, completed, cancelled. Neue Einträge sind enqueued.
- Erlaubt sind ausschließlich diese atomaren Übergänge, bei gleichzeitigen Versuchen gelingt höchstens einer:
  - **Claim:** enqueued → claimed. Einzige Stelle, an der ein Abbruch relevant ist.
  - **Completion:** claimed → completed.
  - **Cancel:** enqueued → cancelled.
- Claim und Cancel schließen sich gegenseitig aus. Der Endzustand passt immer zum Gewinner. Der Zustand ist jederzeit als konsistenter Snapshot abfragbar. Ein ungültiger interner Zustandswert ist ein nicht behebbarer interner Fehler.
- Claim:
  - Der Claimer erhält die Payload per Ownership-Transfer, nie als Kopie, genau einmal.
  - Erneutes Claimen, Claim eines abgebrochenen Eintrags oder des Sentinels scheitert ohne Payload.
  - Bei gleichzeitigen Versuchen gewinnt genau einer. Der Claim blockiert nie und wartet nicht, während das Payload-Fach gesperrt ist.
  - Er ist auch über eine Funktion der Queue verfügbar, mit identischem Verhalten. Jeder Versuch wird mit Ausgang geloggt.
- Completion:
  - Nur der erfolgreiche Claimer darf abschließen, und nur einmal. Das Ergebnis ist Erfolgs- oder Fehlerwert.
  - Das Ergebnis ist gespeichert und sichtbar, bevor der Zustand „completed“ erkennbar wird.
  - Ein zweiter Versuch verändert das Ergebnis nicht, wird mit Warnung geloggt und fällt in Entwicklungsumgebungen als Programmierfehler auf. Bei „cancelled“ wird das Ergebnis verworfen.
  - Wartende werden geweckt. Ohne gesetztes Ergebnis liefert die Abfrage nichts, abgebrochene Einträge haben nie ein Ergebnis.
- Cancel:
  - Nur aus „enqueued“ möglich, mit Rückmeldung über Erfolg. Danach ist kein Claim mehr möglich.
  - In jedem anderen Zustand ändert sich nichts und es wird Misserfolg gemeldet.
  - Ein erfolgreicher Cancel weckt alle Wartenden. Jeder Versuch wird geloggt.

### Warten auf Abschluss

- Asynchrones Warten auf „completed“ oder „cancelled“, ohne einen Thread zu blockieren. Bei bereits erreichtem Endzustand endet es sofort.
- Keine Notification geht verloren, auch wenn der Abschluss zwischen Zustandsprüfung und Wartebeginn eintritt.
- Mehrere Wartende werden gleichzeitig unterstützt und alle geweckt. Ein Wakeup ohne Endzustand führt zu erneuter Prüfung.

### Ownership und Speicherfreigabe

- Ownership-Kette: Queue-Head, Kette, Iterator. Jeder Iterator hält pro Schritt eine Referenz auf den Knoten.
- Nach dem Claim bleibt die leere Hülle in der Kette, bis sie entlinkt wurde und der letzte Iterator sie verlassen hat. Dann wird sie automatisch freigegeben.
- Unlink ist optionale Aufräumarbeit: ein einfacher atomarer Swap auf dem Pointer des Vorgängers. Der entlinkte Knoten behält seinen Nachfolger, damit ein Iterator ohne Neustart weiterläuft. Nicht entlinkte Knoten werden spätestens beim Drop der Queue freigegeben.
- Freigabe über Reference Counting der Knoten. Epoch-based Reclamation, Slab-Verwaltung, Freelist und Generation-Counter entfallen für die Queue.
- Lange Ketten, auch über hunderttausend Einträge, werden iterativ statt rekursiv freigegeben, ohne Stack-Overflow. Von anderen Ownern gehaltene Kettenteile bleiben gültig, bis deren letzter Owner sie freigibt.
- Außerhalb der einen Komponente für den atomaren Pointer-Swap darf kein Code eingesetzt werden, der die Memory-Safety-Garantien der Sprache umgeht (kein `unsafe`).

## Pull-Ausführung der Subengines

### Loop und Iteration

- Pro Subengine läuft ein Loop mit genau einer Wartebedingung, der Notification. Danach folgt eine Iteration über die Queue.
- Die Iteration geht in Altersreihenfolge Knoten für Knoten vor:
  - Sind alle Ressourcen der Subengine ausgelastet, endet die Iteration.
  - Akzeptiert der Filter die Parameter nicht, wird der Knoten übersprungen. Prioritätspräferenz ergibt sich aus Filter und Reihenfolge, also aus Metadaten.
  - Der Ressourceneintrag wird über die Requirements der Operation bestimmt.
  - Ist die Ressource nicht initialisiert, wird die Lazy-Initialisierung angestoßen: atomarer Übergang von „uninitialized“ nach „loading“. Der Task bleibt in der Queue.
  - Ist die Ressource ready, hat freie Kapazität und gelingt der Claim, führt die Ressource den Knoten aus.
  - In allen anderen Zuständen (loading, busy, backoff) wird der Knoten übersprungen.
- Am Ende jeder Iteration werden ungenutzte Ressourcen evicted.

### Lanes, Batches und Ressourcen

- Die Lanes einer Ressource ziehen Operationen. Die Lane der CPU ist ein Thread des Processors.
- Lanes implementieren Batching: Der Batch beginnt mit der Ausgangsoperation. Die Ressource geht von dort vorwärts durch die Liste, übernimmt alles, was der Filter akzeptiert und in den Batch passt, und startet dann den Batch.
- Packing, Length-Bucketing und Streams sind reine Ressourcenlogik.
- Das Ergebnis löst die Notification der Subengine aus.
- Ressourcenzustände: uninitialized, loading, ready, backoff (nach Fehler).

### Notification-Topologie

- Append an eine Queue: Weiterleitung an die Notification jeder Subengine.
- Ergebnis, abgeschlossene Initialisierung und Eviction-Timer: eigene Notification der Subengine.
- Eine Iteration mit Fortschritt löst die eigene Notification aus. Der Loop ist seine eigene Notification.

### Regeln

- Keine Reservierung, kein Vorab-Claim, kein Head-of-Line-Blocking.
- Lazy-Initialisierung geschieht nur durch die Iteration, nie durch das Enqueue.
- Eviction ist nur erlaubt, wenn keine Operation läuft und die Lebensdauer (TTL) abgelaufen ist. Im VRAM ist sie deutlich länger als im RAM.
- Initialisierungs- oder Startfehler schreiben einen Fehler in die Task-Ergebnisse des Batches und versetzen die Ressource in den Backoff. Das wird mit Task-ID geloggt.

### Auslastungsmeldung

- Eine Subengine stellt Ressourcen und Execution-Kapazität eines Compute-Substrats bereit und ist aus mehreren Threads nutzbar.
- Sie gibt Auskunft über Art, Device, Kapazität und geglätteten Durchsatz in Entries pro Sekunde.
- Die Arbeit kommt aus den Queues des Processors. Die Subengine meldet nur, wie viel sie aufnehmen kann.

## Subengines

### Gemeinsame Regeln

- Die Ressourcenverwaltung arbeitet lazy: erst bei Bedarf initialisieren, bei Nichtnutzung evicten. Im VRAM gelten längere Fristen.
- Jede Subengine liefert ein Capability-Set und registriert kompatible Ressourcen samt Lanes in ihren eigenen und den gemeinsamen Queues.
- Jede Subengine kalibriert ihre Hardware per Benchmark und stellt daraus Precision-Profile bereit.
- Jede Subengine implementiert ihre Compute-Pfade hardwareoptimal und entscheidet selbst.
- Für die Batch-Verarbeitung gibt es eine austauschbare Schnittstelle (Trait) von Float-Batch zu Float-Batch:
  - Die CPU-Implementierung ist Default ohne einkompilierte GPU-Beschleunigung und Fallback, wenn die GPU-Beschleunigung nicht initialisierbar ist.
  - Die GPU-Implementierung ist die einzige Anbindungsstelle der GPU-Library. Ihr Anlegen scheitert ohne kompatible GPU, wodurch automatisch auf die CPU zurückgefallen wird.

### Invarianten

- In öffentlichen Signaturen gibt es keinen Tensor und keinen Raw-Pointer, nur Region und Layout.
- Jede GPU-Arbeit läuft über einen Execution-Context (Lane), nie auf dem Default-Stream.
- Kernel werden nur über das Kernel-Register geladen. Kernel der externen Library sind dort eine weitere Implementierung.
- Planung und Zuteilung kennen weder die externe Neural-Network-Library noch GPU-Execution-Graphs (CUDA Graphs).

### CPU-Subengine

- **Device:** die CPU, potenziell mehrere (Sockets). Ressourcen sind die Cores, jeweils mit Lanes.
- **Speicherverwaltung:** lädt Modelle und Indizes ins RAM bzw. streamt Daten hinein, kennt das verfügbare RAM und lagert Modelle, Kernel und Ressourcen bei Bedarf aus und ein.
- **Kernel:** eigene Implementierungen mit eigenem Register.
- **Index:** beim ersten Zugriff geladen, mit Entry, Datei und Manager.
- **Tokenizer und Normalizer:** Ressourcen dieser Subengine.
- **Besonderheiten:**
  - Mehr Speicher steht zur Verfügung, alles nutzt die CPU.
  - Der Scorer ist selbst implementiert und nutzt SIMD.
  - Der Encoder läuft über das CPU-Backend der Library.
  - Der Index ist per mmap lesbar.
  - Keine Streams, kein Staging-Memory für Kopien.
  - Default-Datentyp ist FP32.
  - Kalibrierung und Kernel laufen auf eigenen Threads niedriger Priorität, getrennt vom Async-Runtime.

### GPU-Subengine

- **Plattform:** CUDA. Device ist die GPU, potenziell mehrere, mit Compute Units als Ressourcen und Lanes darauf.
- **VRAM-Manager:**
  - lädt oder streamt Modelle und Indizes ins VRAM,
  - kennt das verfügbare VRAM und tauscht Modelle, Kernel und Ressourcen bei Bedarf aus und ein,
  - ist die Brücke zwischen dem Index im RAM und der im VRAM abgebildeten Darstellung für die Kernel,
  - lädt Kernel sowie Input-/Output-Arenen und Speicherbereiche,
  - synchronisiert dadurch den Index effektiv zwischen RAM und VRAM.
- **Kernel:** mit Register. Das Address-Mapping im VRAM erfolgt über Slots.
- **Index:** Segmenttabelle und Segmente.
- **Besonderheiten:**
  - Das VRAM ist begrenzt.
  - Es gibt Streams, Staging-Memory und Transfers.
  - Batching ist notwendig, damit Index-Segmente und Kernel effizient gestreamt und getauscht werden.
  - Default-Datentyp ist FP16.

### Gemeinsame Ressourcen

Die folgenden Ressourcen liegen CPU-seitig (Host) und werden von beiden Subengines genutzt.

- **Modell:** Die Modellbeschreibung stammt aus der Konfiguration, beschreibt eine Pipeline und besitzt nichts. Ein Register ordnet Architektur-IDs Encoder-Implementierungen und Scoring-Arten Scorer-Implementierungen zu. Neue Modelle bedeuten einen neuen Register-Eintrag. Multi-Vector-Parameter stammen aus der Checkpoint-Konfiguration, nicht aus hartkodiertem Code: maximale Query-Länge, maximale Dokumentlänge, Marker-Token, Dimension, Skiplist.
- **Normalizer:** gehört zum Tokenizer (Unicode-Normalisierung, Lowercasing und weitere Schritte), läuft nur auf der CPU und ist über Parameter konfiguriert.
- **Tokenizer:** läuft nur auf der CPU, modellspezifisch oder geteilt. Er liefert Token-IDs als u32. Mehr braucht die GPU nicht, also kein Gathering auf der CPU und kein Address-Mapping.
- **Weight-Source:** safetensors, per mmap und zero-copy auf Rohbytes. Die Gewichte werden pro Subengine in deren Arena kopiert.

## Tokenizer

### Laden

- Der Tokenizer wird aus seiner Konfigurationsdatei mit festem Dateinamen geladen, die im Verzeichnis der Modellkonfiguration liegt.
- Alternativ entsteht er aus einem bereits geladenen Tokenizer mit frei wählbarer ID, Schema für die Modus-Wahl und maximaler Sequenzlänge.
- Beim Laden aus einer Modellbeschreibung werden das Schema aus den hinterlegten Präfixen gebildet, die maximale Sequenzlänge übernommen und die Ressourcen-ID aus dem Dateipfad abgeleitet, sodass jeder geladene Tokenizer eindeutig identifizierbar ist.
- Fehlt die Datei oder ist sie unlesbar, bricht das Laden mit einem Load-Error ab, der Pfad und Ursache nennt.
- Das Pad-Token wird in dieser Reihenfolge bestimmt: im Tokenizer konfiguriertes Pad-Token, dann das Token `<pad>`, dann das Token `[PAD]`. Fehlt jedes, bricht das Laden mit einem Load-Error ab. Das Pad-Token ist nach außen abfragbar.
- Das eingebaute automatische Padding wird abgeschaltet. Padding ist ausschließlich Aufgabe der Batch-Bildung.
- Jede Eingabe wird auf die maximale Sequenzlänge trunkiert, wobei das abschließende Special-Token erhalten bleibt. Lehnt der Tokenizer die Truncation-Einstellung ab, bricht das Laden mit Ursache ab.

### Textvorbereitung

- Optional wird ein austauschbarer, nebenläufig nutzbarer Text-Normalizer übergeben. Ohne ihn bleibt der Text unverändert. Ein Identity-Normalizer wird mitgeliefert.
- Normalisierungen, die der Tokenizer selbst durchführt, werden nicht doppelt ausgeführt. Ein eigener Normalizer ergänzt nur zusätzliche Transformationen.
- Die Normalisierung läuft vor dem Voranstellen von Präfixen oder Markern und vor der Tokenisierung.

### Encoding-Modi und Schemata

- Zwei Encoding-Modi: Query und Passage (Dokument).
- **Präfix-Schema:** Je Modus wird der Query- bzw. Passage-Präfix vorangestellt. Die Tokenfolge bleibt danach unverändert.
- **Multi-Vector-Schema** (Late Interaction): Je Modus wird ein Query- bzw. Dokument-Marker vorangestellt. Je Modellvariante konfigurierbar sind Markertext für Queries und Dokumente, feste Query-Länge, maximale Dokumentlänge und ID des Mask-Tokens.
  - Query-Modus: Trunkieren auf die Query-Länge, Padding mit dem Mask-Token auf genau diese Länge. Die gepaddeten Positionen werden vom Modell berücksichtigt, nicht ausgeblendet (Query Augmentation).
  - Dokument-Modus: Trunkieren auf die maximale Dokumentlänge, ohne Padding. Pad-Tokens werden verworfen.
  - Diese Trunkierungen gelten zusätzlich zur allgemeinen Trunkierung.

### Encoding

- Eine Liste von Texten beliebiger als String lesbarer Typen wird in einem Modus in Token-IDs umgewandelt. Eine leere Liste wird mit „keine Eingabetexte“ abgelehnt.
- Je Text in dieser Reihenfolge: normalisieren, Präfix oder Marker voranstellen, tokenisieren mit den vorgesehenen Special-Tokens, schemaabhängig nachbearbeiten. Alle Texte werden gemeinsam als Batch tokenisiert.
- Scheitert die Tokenisierung, wird ein Encoding-Error mit Ursache gemeldet.
- Rückgabe in Eingabereihenfolge, ohne Padding außer dem festen Query-Padding im Multi-Vector-Schema.
- Geloggt werden Anzahl trunkierter Texte mit Modus und Textanzahl sowie auf Trace-Level die größte Sequenzlänge.
- Eine kombinierte Funktion kodiert und teilt danach in gepaddete, nach Länge gruppierte Batches gemäß Batch-Konfiguration. Jeder Batch ordnet seine Zeilen den Eingabe-Indizes zu. Encoding-Errors, auch die leere Eingabe, werden unverändert weitergegeben. Die Batch-Anzahl wird geloggt.

## Batch-Bildung für Eingabesequenzen

- Aus einer Liste von Tokenfolgen entsteht eine Liste von Batches. Jede Folge liegt in genau einem Batch. Jeder Batch führt die ursprünglichen Positionen seiner Zeilen mit. Leere Eingabe ergibt eine leere Liste.
- Konfiguration: Token-Budget je Batch (Zeilen mal Sequenzlänge), maximale Sequenzlänge, ID des Pad-Tokens.
- Length-Bucketing:
  - Zu lange Folgen werden auf die maximale Sequenzlänge trunkiert.
  - Die gepaddete Länge ist die kleinste Bucket-Größe aus 32, 64, 128, 256 und 512, die mindestens der trunkierten Länge entspricht. Oberhalb der größten Bucket-Größe gilt die maximale Sequenzlänge.
  - Die gepaddete Länge überschreitet nie die maximale Sequenzlänge und beträgt mindestens eins.
- Gruppierung:
  - Sortierung aufsteigend nach ursprünglicher Länge, bei Gleichheit stabil in Eingabereihenfolge.
  - Vor jedem Hinzufügen wird geprüft, ob das Token-Budget mit der neuen Zeile überschritten würde. Maßgeblich ist deren gepaddete Länge, wegen der Sortierung die größte im Batch. Dann wird ein neuer Batch begonnen.
  - Eine Folge, die das Budget allein überschreitet, bildet einen eigenen Batch.
  - Die Sequenzlänge eines Batches ist die gepaddete Länge seiner längsten Zeile. Alle Zeilen sind gleich lang.
- Inhalt: Token-IDs zeilenweise als zweidimensionales Array, am Zeilenende mit dem Pad-Token aufgefüllt, dazu eine gleich geformte Attention-Mask mit eins für echte Token und null für Padding. Die Zeilenanzahl ist abfragbar.
- Serialisierung: zuerst alle Token-IDs, dann die gesamte Mask, jeder Wert als u32 in Little-Endian. Das entspricht einem dreidimensionalen Array der Form 2 × Zeilen × Sequenzlänge.
- Der Encoder nutzt ein Token-Budget statt einer Task-Anzahl. Die Batch-Größe ergibt sich aus Token-Budget, Length-Bucket und Deadline je Klasse. Die Obergrenze der Batch-Dauer begrenzt die Query-Latenz.

## Modell

### Modellbeschreibung und Validierung

- Ein Modell wird durch eine reine, ressourcenlose Beschreibung (Descriptor) dargestellt:
  - Model-ID und Architektur-ID,
  - Embedding-Dimension,
  - Pooling-Verfahren und Normalisierungsart,
  - Präfixe für Queries und Passages,
  - Storage-Datentyp der Embeddings, unabhängig von den Compute-Datentypen wählbar,
  - Compute-Datentypen des Hauptmodells, getrennt für CPU und GPU und nach Device-Klasse ermittelbar,
  - maximale Sequenzlänge,
  - optionale Projection-Layer mit Tensor-Name,
  - Pfade zur Konfigurationsdatei und zu den Weight-Dateien,
  - optionale Multi-Vector-Konfiguration.
- Default für „intfloat/multilingual-e5-small“ auf Basis eines Verzeichnisses und wählbaren Storage-Datentyps: Architektur „bert“, Dimension 384, Mean-Pooling über unmaskierte Token, L2-Normalisierung, Präfixe „query: “ und „passage: “, maximale Sequenzlänge 512, Compute-Datentyp FP32 auf beiden Device-Klassen, keine Projection-Layer, Konfigurationsdatei „config.json“ und Weight-Datei „model.safetensors“ im Verzeichnis.
- Validierung: Dimension und maximale Sequenzlänge größer null, ein nicht leeres Präfix endet mit einem Leerzeichen. Verstöße ergeben „inkompatible Modellkonfiguration“ mit Begründung. Die Validierung verhindert Konfigurationen, die die Embedding-Qualität unbemerkt verschlechtern.

### Modellregister und Laden

- Das Register bildet Architektur-IDs auf Konstruktor-Funktionen (Factories) für Encoder ab. Erneute Registrierung derselben ID ersetzt die bisherige.
- Ein Register mit allen in den Build einkompilierten Architekturen ist erzeugbar. „bert“ ist nur bei aktivierter Backend-Unterstützung (Feature-Flag) enthalten.
- Laden für eine Device-Klasse in dieser Reihenfolge: Beschreibung validieren, Factory suchen, Encoder konstruieren. Unbekannte Architekturen ergeben „nicht unterstützte Architektur“ mit ID.
- Ergebnis ist ein geladener Encoder aus Architektur (Code) und Gewichten (Daten) auf einem Device, der Beschreibung, Device-Klasse und Encoder bündelt und lesend zugänglich macht.
- Der geladene Encoder ist eine verwaltbare Ressource: Key aus Architektur-ID und Device-Klasse, Initialisierungskosten 100, Freigeben entlädt die Gewichte.

### Laden des Transformer-Hauptmodells

- Die Konfigurationsdatei wird als strukturierter Text (JSON) gelesen. Unlesbarkeit ergibt einen Load-Error mit Begründung.
- Der Modelltyp muss „bert“ lauten, sonst „inkompatibel“ mit gefundenem Typ.
- Die Hidden-Size muss der Beschreibung entsprechen. Die maximale Sequenzlänge darf die Anzahl der Position-Embeddings nicht überschreiten. Verstöße ergeben „inkompatibel“ mit beiden Werten.
- Auf der CPU ist das Modell stets ladbar. Auf der GPU nur bei einkompilierter Unterstützung, dann auf dem ersten GPU-Device, sonst „inkompatibel“ mit Hinweis auf die fehlende Unterstützung.
- Der Compute-Datentyp der Device-Klasse wird in den Backend-Datentyp übersetzt, sonst „inkompatibel“.
- Gewichte werden per mmap ohne vollständiges Kopieren im gewählten Datentyp auf das Device geladen. Ein fehlender Tensor ergibt „Gewichtsname passt nicht“ mit Name, sonstige Fehler einen Load-Error.
- Forward-Pass: aus Token-IDs und Mask (Zeilen × Sequenzlänge) die Hidden-States aller Token. Token-Type-Inputs sind null, die Mask wirkt als Attention-Mask. Ergebnis als flache Liste von FP32 (Zeilen × Sequenzlänge × Dimension), unabhängig vom Compute-Datentyp. Fehler ergeben „Encoding fehlgeschlagen“. Die Ausgabedimension ist abfragbar.
- Zwischen Hauptmodell und übriger Pipeline werden keine Typen der Tensor-Library ausgetauscht, nur einfache Zahlenlisten.

### Aufbau des Encoders

- Aus Beschreibung und Device-Klasse entsteht ein vollständiger Encoder aus Hauptmodell, optionaler Projection-Layer, Pooling, Normalisierung und Storage-Datentyp. Device-Klasse und Compute-Datentyp werden für das Tracing hinterlegt.
- Die Normalisierungsart wird übernommen. Ohne Angabe ist die L2-Normalisierung Default.
- Die Projection-Layer wird aus der ersten Weight-Datei geladen. Ohne Weight-Datei: Load-Error „keine Gewichtsdateien“. Ihre Eingabedimension muss der Ausgabedimension des Hauptmodells entsprechen, sonst „inkompatibel“.
- Für Multi-Vector-Modelle nach Art von ColBERT, für die die externe Library kein Modul hat, wird ein vorhandener Encoder-Backbone (BERT, ModernBERT und weitere) mit eigenem Projection-Head verwendet.

### Projection-Layer

- Optionale lineare Abbildung jedes Token-Vektors von Input- auf Output-Dimension, etwa von 768 auf 128, insbesondere für Late Interaction.
- Weight-Matrix Output × Input, optionaler Bias der Länge Output-Dimension. Laden aus safetensors im Speicher oder per mmap aus einer Datei, über den Tensor-Namen der Beschreibung.
- Der Bias-Name entsteht durch Ersetzen der Endung „weight“ durch „bias“, sonst durch Anhängen von „bias“.
- Fehlender Tensor: „Gewichtsname passt nicht“. Falsche Shape: „inkompatibel“ mit tatsächlicher und erwarteter Shape. Werte in FP32, FP16 oder BF16 werden in FP32 konvertiert, andere Typen ergeben „inkompatibel“ mit Tensor und Typ. Dateifehler ergeben Load-Error.
- Anwendung: Dot-Product mit jeder Weight-Zeile plus Bias, Reihenfolge der Vektoren bleibt. Input- und Output-Dimension sind abfragbar.

### Pooling und Normalisierung

- Pooling-Verfahren: Mean über unmaskierte Token (ein Vektor je Sequenz) oder kein Pooling (ein Vektor je Token).
- Mean: Summe der Hidden-State-Vektoren mit Mask ungleich null geteilt durch deren Anzahl. Padding beeinflusst das Ergebnis nicht. Vollständig maskierte Sequenzen ergeben einen Nullvektor ohne Division durch null.
- Ohne Pooling werden alle Positionen unverändert weitergegeben, einschließlich Padding.
- L2-Normalisierung: jeder Vektor wird durch seine Norm geteilt, damit das Dot-Product dem Cosinus entspricht. Die Norm wird nach unten auf 10⁻¹² begrenzt (Epsilon-Clamp). Vektoren darunter gelten als degeneriert und bleiben praktisch Nullvektoren. Die Anzahl degenerierter Vektoren wird zurückgemeldet. Die Art „none“ lässt Vektoren unverändert.
- Die L2-Normalisierung wird einmal beim Schreiben am Ende des Encoders ausgeführt, nicht im Scorer. Query und Dokument werden gleich behandelt.

### Schreiben in die Output-Region

- Die Ergebnisvektoren werden bei Bedarf normalisiert, in den Output-Datentyp konvertiert und lückenlos in die Output-Region geschrieben.
- Zulässig sind FP32, FP16 und BF16 in Little-Endian, andere Typen ergeben „inkompatibel“.
- Degenerierte Vektoren erzeugen eine Warnung mit Anzahl, dass leere Embeddings als Nullvektor gespeichert wurden. Der Vorgang gilt als erfolgreich.
- Schreibfehler werden als Memory-Error weitergereicht.
- Auf der GPU kann ein dünner Write-Kernel den Device-Pointer des Ergebnis-Tensors lesen und normiert sowie datentypkonvertiert direkt in den vom Manager vorab vergebenen Slot schreiben. Der Tensor bleibt bis zum Event am Ende des Schreibens am Leben.

### Encoder-Schnittstelle

- Abfragbar sind Ausgabedimension (der Projection-Layer, sonst des Hauptmodells) und Storage-Datentyp.
- Input ist eine Region mit Layout im Serialisierungsformat der Batch-Bildung, Output ebenfalls eine Region mit Layout. Vorab wird geprüft, sonst „inkompatibel“ mit erwarteter und tatsächlicher Shape bzw. Datentyp:
  - Input genau 2 × Zeilen × Sequenzlänge als u32,
  - Output bei Mean-Pooling Zeilen × Dimension, sonst Zeilen × Sequenzlänge × Dimension,
  - Output-Datentyp gleich dem Storage-Datentyp.
- Das Encoding läuft asynchron auf einer übergebenen Lane. Der Aufruf kehrt nach dem Enqueue zurück und liefert ein Completion-Event.
- Ablauf: Input-Bytes lesen und in Token-IDs und Mask trennen, Hidden-States berechnen, Projection anwenden, Pooling ausführen, in die Output-Region schreiben.
- Scheitern Enqueue oder Event-Erzeugung, wird der Fehler als Device- oder Memory-Error zurückgegeben. Fehler während der Ausführung werden als Warnung „Embedding-Batch fehlgeschlagen“ geloggt und im Completion-Event hinterlegt. Die Lane selbst gilt als erfolgreich abgeschlossen.
- Das Ergebnis läuft über einen One-Shot-Channel je Task plus Event oder Notification. Der Batch merkt sich die Zeilen-Offsets je Task.
- Das Completion-Event ist nicht blockierend abfragbar (Polling) und blockierend abwartbar. Das Warten liefert einen hinterlegten Verarbeitungsfehler genau einmal. Fehler der Execution-Umgebung beim Warten werden als Device- oder Memory-Error gemeldet.

### Speicherzugriff

- Lese- und Schreibzugriff auf Regionen einer Memory-Domain laufen über eine einheitliche, nebenläufig nutzbare Schnittstelle: Bytes einer Region in einen Buffer lesen und Bytes in eine Region schreiben. Die CPU-Speicherverwaltung stellt sie bereit.
- Eine Tensor-Referenz verbindet Region und Layout und ist der einzige Weg, auf dem Tensoren die Encoder-Schnittstelle passieren.

## Devices, Lanes und Events

- Device-Arten sind CPU und GPU, erweiterbar. Das Device-Handle besteht aus dem Device-Ordinal bzw. der CPU.
- Bei der GPU gibt es einen Prozess, einen Primary Context und mehrere Streams.
- Ein Execution-Context (Lane) besteht aus Stream, Priorität, Events, eigenem Parameter-Block und Workspace.
- Beispiel-Lanes: Query (hohe Priorität, kleine Batches), Insert und Update im Index (Throughput), Kopieren zwischen RAM und VRAM in beide Richtungen (H2D und D2H).
- Übergaben zwischen Lanes laufen ausschließlich über die Queue.
- Ein Event ist ein opaker Completion-Marker für alle vor seiner Aufzeichnung auf einer Lane eingereichte Arbeit:
  - Kopien beziehen sich auf dasselbe Event.
  - Der Abschluss ist nicht blockierend abfragbar und blockierend abwartbar. Bei GPU-Events meldet das Warten Fehler als Event-Error mit Ursache.
  - Ein CPU-Event bleibt nach dem Signalisieren dauerhaft abgeschlossen und weckt alle Wartenden.
  - Die Textdarstellung (Display) zeigt den Completion-Status.
- Der Laufzustand wird über ein Event je Batch erfasst. Im Persistent-Modus dient ein Progress-Counter in gemapptem Host-Memory.

## Speicherverwaltung

### Arenen

- Der Speicher wird in sechs Arenen (Speicherklassen) nach Lebensdauer eingeteilt:
  - Weights: langlebig, nur beim Unload frei.
  - Index-Segmente: gestreamt, evictable. Die Kapazität umfasst alle gleichzeitig residenten Generationen.
  - Workspace je Lane: kurzlebig.
  - Staging: Pinned Memory im Host für Kopien in beide Richtungen, bei der CPU entfallend.
  - Parameter-Block je Lane: kleine feste Region für dynamische Kernel-Parameter.
  - Mapped Host-Memory: Pinned Memory, den das Device über den Systembus (PCIe) direkt liest.
- Jede Arena hat eine stabile, eindeutige numerische ID, aus der sie rückgewinnbar ist. Unbekannte IDs ergeben „keine Arena“. Die Liste ist vollständig in fester Reihenfolge abrufbar und erweiterbar.
- Je Arena ist eine Kapazität in Byte konfigurierbar.
- Der tatsächliche Device-Bedarf ist die Summe aus Weights, Index-Segmenten, Workspace und Parameter-Block, saturierend statt wrapping. Staging und Mapped Host-Memory zählen nicht, sie liegen im Host-Memory.
- Die Reserve für andere Generationen ist die Index-Kapazität abzüglich des Anteils der aktiven Generation bei gleichmäßiger Aufteilung. Eine Generationszahl unter eins gilt als eins.
- Die Arena wird lazy beim ersten Laden nach Budget angelegt. Alle Adressen sind 64-Bit-Offsets innerhalb der Arena.
- Allokation und Freigabe von VRAM im laufenden Betrieb ist zu vermeiden, weil es das Device implizit synchronisiert.
- Stream-ordered Reuse: Eine Region wird erst frei oder überschrieben, wenn alle Streams, die sie nutzen, ihr Event erreicht haben.
- Kopien laufen über einen eigenen Copy-Stream mit Pinned Staging-Memory, getrennt vom Compute-Stream.

### Budget

- Das Budget besteht aus freiem Speicher, Gesamtspeicher und Generationsreserve. Der nutzbare Anteil ist der freie Speicher abzüglich Reserve, nie negativ.
- Freier und gesamter Device-Speicher werden zur Laufzeit über eine austauschbare Abfrage ermittelt. Scheitern ergibt „Speicherabfrage fehlgeschlagen“ mit Begründung.
- Default-Headroom: 512 MiB für Treiberkontext und andere Prozesse, etwa den Display-Compositor.
- Der Fit-Check stellt fest, ob die geplanten Device-Arenen in den freien Speicher abzüglich des saturierend abgezogenen Headrooms passen, sonst Fehler mit angeforderter und nutzbarer Menge.
- Das Budget plant den kurzen Doppelbedarf durch Copy-on-Write ein.

### Region, Layout und Elementtypen

- Eine Region ist ein pointerfreies Handle aus Arena-ID, Offset, Länge und Generation, kopierbar, vergleichbar und als Key verwendbar. Die Generation entwertet Handles nach Eviction und Neuladen.
- Das Auflösen einer Region auf eine Device-Adresse ist ausschließlich Sache des Memory-Backends. Device-Adressen verlassen das Backend nicht.
- Eigene Elementtypen unabhängig von Fremd-Libraries: FP32 (4 Byte), FP16 (2 Byte), BF16 (2 Byte), u8 (1 Byte), u32 (4 Byte), i64 (8 Byte).
- Elementtypen sind aus dem safetensors-Format übernehmbar, nicht unterstützte ergeben „nicht unterstützter Datentyp“ mit Quelle. Mit der Tensor-Library wird in beide Richtungen konvertiert, ebenso zwischen Elementzahl (Library) und Bytezahl (Engine).
- Für additive Attention-Masks gilt der feste endliche Mask-Wert −10 000. Negativ unendlich ergäbe bei FP16 ungültige Zeilen (NaN).
- Datentypen gelten je Device: Default auf der GPU FP16, auf der CPU FP32. Der Datentyp eines Index ist je Index fest.
- Ein Layout besteht aus Shape, Strides in Byte und Elementtyp. Views ändern nur das Layout, nie die Daten:
  - Contiguous Layout: letzte Dimension mit Elementgröße als Stride, jede vordere als Produkt der hinteren Dimensionen. Dimensionen der Größe null zählen als eins.
  - Abfragbar: Elementzahl, Bytelänge, Contiguity.
  - Transponieren zweier Dimensionen tauscht Shape und Strides. Narrowing ändert nur die Shape der Dimension.

### Free-Space-Verwaltung

- Je Arena vergibt ein Offset-Allocator Bereiche aus fester Kapazität, ohne selbst Speicher zu halten, nach dem First-Fit-Prinzip in aufsteigender Adressreihenfolge.
- Beim Freigeben verschmelzen Lücken mit angrenzenden freien Nachbarn auf beiden Seiten (Coalescing).
- Alignment rundet den Offset auf, null gilt als eins. Overflows führen zu „passt nicht“, nie zum Absturz. Reste vorn und hinten bleiben freie Lücken.
- Anforderungen der Länge null werden nicht erfüllt. Kapazität und freie Menge sind abfragbar. Bei Kapazität null gibt es keine Lücke.

### Bookkeeping und Gültigkeit

- Je Arena verfolgt ein thread-safe Bookkeeping-Allocator Offsets und Generationen, ohne Device-Speicher zu besitzen.
- Erstmals vergebene Kombinationen aus Offset und Länge erhalten Generation null. Erneute Vergabe derselben Kombination erhöht die Generation mit Wrap-around.
- Eine Region ist live, wenn Arena, Offset, Länge und Generation einer aktuellen, nicht freigegebenen Vergabe entsprechen.
- Freigabe scheitert bei fremder Arena, unbekannter, bereits freigegebener oder abweichender Region. Das allgemeine Verwaltungs-Interface dereferenziert stale Handles nie, sondern ignoriert sie mit Warnung.
- Platzmangel nennt angeforderte und freie Menge. Für Tests lässt sich ein Allocation-Fehler je Arena künstlich auslösen (Fault Injection), gemeldet wie Erschöpfung.
- Allokationen und Freigaben werden auf Debug-Level mit Arena, Offset und Größe geloggt.

### Allocator je Arena mit Deferred Free

- Jeder Allocator kennt Kapazität, Arena und Device-Art. Vor jeder Allokation sammelt er abgeschlossene Deferred Frees ein.
- Erschöpfung ergibt einen Fehler mit Arena, angeforderter und freier Menge, andere Fehler „Allokation fehlgeschlagen“.
- Sofortige Freigabe wird bei vorgemerkten oder ungültigen Regionen mit „stale Region“ abgelehnt.
- Deferred Free nimmt Region und Event. Die Region kehrt erst nach Abschluss des Events zurück, sodass laufende Arbeit nie wiederverwendeten Speicher sieht. Abgelehnt bei nicht lebenden oder bereits vorgemerkten Regionen. Nach dem Vormerken wird sofort auf Abschluss geprüft.
- Das Einsammeln (Reaping) gibt abgeschlossene Regionen frei und liefert die Anzahl. Ungültige Regionen erzeugen nur eine Warnung.
- Die sofort verfügbare Menge schließt wartende Regionen aus. Anzahl wartender Regionen und Liveness sind abfragbar. Alles ist thread-safe.
- Ein Allocator-Set stellt je Arena genau einen bereit, liefert die Größenvorgabe und findet den zuständigen Allocator zu einer Region. Fremde IDs ergeben „stale Region“.
- Jedes Memory-Backend erfüllt dieselbe nebenläufige Schnittstelle: Allokation in einer Arena mit Größe und Alignment, Freigabe, Freigabe nach Event, Budget-Abfrage.

### CPU-Backend

- Je Arena ein mit Nullen gefüllter Bereich im RAM, ohne Streams und Staging.
- Schreiben, Lesen und Kopieren auf Regionsbasis mit Prüfung auf Liveness und Länge, sonst „stale Region“. Beim Kopieren gilt die Quelllänge, innerhalb einer Arena überlappungssicher (memmove), zwischen Arenen über einen Zwischenpuffer.
- Budget: Summe der verfügbaren Mengen als frei, Summe der Kapazitäten als gesamt, plus Generationsreserve.

### GPU-Backend

- Nur mit einem Nvidia-Device anlegbar, sonst „Device-Speicher nicht verfügbar“.
- Beim Anlegen zuerst Speicherabfrage und Fit-Check mit Headroom. Bei Misserfolg wird nichts reserviert.
- Freigegebener, stream-ordered allokierter Speicher bleibt im Default-Pool des Treibers, damit im Dauerbetrieb keine Treiber-Allokation mehr stattfindet.
- Je Device-Arena ein zusammenhängender, genullter Block in voller Kapazität. Arenen der Größe null erhalten keinen Block. Danach wird auf Abschluss aller Vorgänge gewartet.
- Für Mapped Host-Memory größer null ein Pinned, genullter Block mit eigener Device-Adresse. Scheitert das Mapping, wird er freigegeben. Beim Shutdown wird er freigegeben, sofern der Device-Context noch bindbar ist.
- Das Allocator-Set setzt die Staging-Kapazität auf null, da diese über den Ring verwaltet wird.
- Nach dem Anlegen werden freier und gesamter Speicher sowie die Arena-Größen in MiB geloggt.
- Kopieren und Transfer:
  - D2D zwischen gültigen Regionen verschiedener Arenen auf einem Stream. Gleiche Arena wird mit Transfer-Error abgelehnt, fehlende Blöcke mit „stale Region“. Locks in fester Reihenfolge verhindern Deadlocks.
  - Asynchroner Upload (H2D) aus Pinned Buffer, der bis zum danach aufgezeichneten Event unverändert bleibt. Asynchroner Download (D2H), lesbar erst nach dem Event. Region live, Datenmenge passend.
  - Treiberfehler: „Device-Speicher nicht verfügbar“ oder Transfer-Error mit Richtung und Begründung.
- Mapped Host-Memory ist ohne Transfer beschreib- und lesbar. Voraussetzung sind existierender Block, passende Arena und lebende Region. Offset und Länge werden overflow-sicher gegen Region und Block geprüft. Die Device-Adresse ist für Kernel-Launches bereitstellbar.
- Ein Ring aus Pinned, genullten Host-Buffern mit Anzahl und Größe ist auf Anforderung erzeugbar.
- Das Budget wird bei jedem Aufruf frisch vom Treiber ermittelt und auf Debug-Level geloggt.

### Staging-Ring

- Feste Anzahl von Slots mit Payload im Zustand frei, belegt oder in Übertragung mit Event. Wiederverwendung erst nach Abschluss des Events.
- Acquire reihum hinter dem zuletzt vergebenen Slot, sonst nichts. Blockierendes Acquire wartet auf die älteste laufende Übertragung. Sind alle Slots belegt ohne Übertragung: „Staging erschöpft“. Scheitert das Warten: Transfer-Error Richtung H2D.
- Payload-Zugriff und die Übergänge „in Übertragung“ und „freigeben“ nur aus „belegt“, sonst „ungültiger Slot“. Anzahl und Slots in Übertragung sind abfragbar. Thread-safe.

### Einbindung von Regionen in Tensoren

- Ein Teilbereich eines Device-Blocks ist in einen Tensor der Tensor-Library einhüllbar (Wrapping), sodass sie den korrekten Bereich liest.
- Der gewrappte Tensor besitzt den Speicher nicht und wird bewusst nicht freigegeben, damit Context und Block intakt bleiben.
- Kopien erhalten eigenen Speicher außerhalb des Blocks. Schreiben schlägt auf den Teilbereich durch, Nachbarelemente bleiben unverändert.
- Diese Prüfungen laufen nur auf einem echten Nvidia-Device.

### Übergreifende Eigenschaften

- Alle gemeinsam genutzten Strukturen sind nebenläufig nutzbar.
- Allocators, Event-Dependencies und Staging sind ohne GPU testbar, etwa mit einem künstlich steuerbaren Event (Mock-Event).

## Index

### Grundsätze

- Ein Index kann eine Struktur kapseln, eine memory-mapped Datei sein oder intern Byte-Buffer oder Spaltenformate nutzen.
- Jeder Index implementiert sein eigenes Modell, etwa ganze Dateien, Sliding Windows, AST-Knoten, Chunks, kaskadierende oder fraktale Strukturen. Der Kern kennt diese Modelle nicht.
- Darstellung im RAM und persistente Darstellung sind dieselbe Instanz. Das VRAM hält nur einen abgeleiteten, verwerfbaren Mirror. Der Index ist die Source of Truth. Die Richtungen zwischen RAM, Index und VRAM sind agnostisch.
- Der Index kennt weder Subengine noch Profil noch Kernel, kennt keine Geometrie und löst keine Scopes auf. Er meldet Änderungen und liefert Struktur und Change-Stamps. Er ist über ein gemeinsames Interface von mehreren Threads gleichzeitig nutzbar.
- Vektoren verschiedener Modelle sind nicht vergleichbar. Ein Metadatum „erzeugendes Profil“ ist höchstens optionale Diagnose.
- Ein Index wird beim ersten Zugriff lazy geladen.
- Ein Entry besteht aus ID, Change-Stamp und Payload. Entry-IDs sind 64-Bit-Werte und total geordnet. Knoten und Zellen haben eigene ID-Typen.

### Change-Stamp

- Jede Schreib- und Löschoperation erhält einen neuen Stamp, eindeutig innerhalb des Index, auch bei gleichzeitigen Zugriffen.
- Der Stamp ist ein opaker Veränderungsmarker. Consumer vergleichen nur auf Gleichheit, nie auf Ordnung.
- Er wird im exklusiven Write-Abschnitt unmittelbar vor der Publication vergeben.
- Auch das Entfernen verbraucht einen Stamp. Eine später neu angelegte gleiche ID erhält daher einen anderen Stamp.
- Ein neuer Index beginnt bei eins. Ein wiederaufgenommener setzt oberhalb des Größeren aus gespeichertem Max-Stamp und höchstem tatsächlich vorkommendem Stamp fort.
- Max-Stamp (null bei leerem Zähler) und Rohdarstellung für Persistenz und Notification sind abfragbar.

### Allgemeiner Vertrag

- Stamp eines Entries: aktueller Stamp oder „nicht vorhanden“.
- Alle Stamps: Snapshot aller Entries samt Stamp, ohne Vektordaten zu berühren.
- Lesen eines Entries: Vektorbereich oder leere Daten bei Fehlen. Darf blockieren. Daten geliehen (borrowed) oder gemeinsam referenziert, ein gelesener Wert ändert sich durch spätere Writes nicht.
- Schreiben ersetzt die Daten vollständig. Mehrere Writer werden serialisiert, der zuletzt drankommende gewinnt (Last-Writer-Wins). Verbindliche Reihenfolge: Daten, Stamp, Signal. Rückgabe ist der Stamp genau dieses Writes.
- Change-Notification als Level-Signal mit dem aktuellen Max-Stamp. Beliebig viele Observer, Anfangswert ist der aktuelle Max-Stamp. Jeder Write und Delete aktualisiert das Signal. Koaleszierte Signale sind unschädlich.
- Ein Index deklariert, welche Zugriffsmethoden er unterstützt (Centroid, Prefix, Subtree, Range) und liefert die zugehörigen Strukturen, etwa Summary-Struktur, Kinder eines Knotens oder Byte-Bereiche zusammenhängend gespeicherter Entries.

### Scopes und Entry-Mengen

- Scope-Arten mit stabilem Namen für Logs und Metriken: alle Entries („all“), ID-Bereich („range“), explizite Menge („set“), Teilbaum ab Knoten („subtree“), Byte-Präfix („prefix“), Centroid-Umgebung mit Anzahl und optionalem Radius („centroid“). Neue Arten sind ergänzbar.
- Entry-Mengen sind kompakt als Ranges oder Bitmap gespeichert, nicht als ID-Liste, und unterstützen 64-Bit-IDs.
- Aufbau aus Ranges (leere oder umgekehrte werden ignoriert) oder einzelnen IDs.
- Operationen: Leere, Anzahl, Containment, aufsteigende Iteration, Union, Intersection, Gleichheit, gleichheitsverträglicher Hash, Ausgabe als maximale zusammenhängende aufsteigende Runs (bei großen Mengen teuer).

### Referenz-Index im RAM

- Ein Referenz-Index im RAM ist ohne GPU-Beschleunigung und ohne Machine-Learning-Libraries lauffähig.

### Dateigestützter Index

- **Öffnen** mit Dateipfad, erwartetem Index-Header und Optionen:
  - Fehlt die Datei, entsteht ein leerer Index. Die Datei entsteht erst beim ersten Save.
  - Zugriff per mmap oder File-Read. Bei mmap wird, wo möglich, Random Access angekündigt (madvise).
  - Der Header im Datei-Footer muss vollständig und lesbar sein. Fehlende Keys, nicht numerische Werte und unbekannte Enum-Werte ergeben Schema-Inkompatibilität mit Key, erwartetem und gefundenem Wert.
  - Der Header muss dem erwarteten entsprechen. Alle Abweichungen werden gesammelt gemeldet.
  - Das Spalten-Schema muss in Name, Datentyp und Nullability entsprechen, Spalten-Metadaten werden ignoriert. Abweichungen nennen beide Schemata.
  - Über die ID- und Stamp-Spalten werden alle Entries ohne Vektor-Dekodierung eingelesen. Doppelte IDs ergeben „Index nicht verfügbar“.
  - Der Stamp-Zähler wird fortgesetzt wie beschrieben.
  - Optional (Default ein) werden alle Daten dekodiert und Checksums kontrolliert. Ein Fehler bricht das Öffnen ab.
  - Geloggt werden Pfad, Schema-Version, Model-ID, Model-Revision, Zugriffsart, Anzahl Row-Groups und Checksum-Ergebnis.
  - Datei- oder Lesefehler ergeben „Index nicht verfügbar“ mit Pfad und Ursache.
- **Lazy Loading:** Vektoren und Nutzdaten werden beim ersten Gebrauch genau einmal dekodiert und gehalten, in Batches von 4096 Zeilen, als FP16-Werte in Little-Endian. Ein Fehlschlag wird gemerkt und bei jedem Zugriff gemeldet: beim Vektor-Lesen leere Daten, bei Nutzdaten „nicht vorhanden“, jeweils mit Error-Log.
- **Optionen:** Zugriffsart „mmap“ (Default) oder „file“ (File-Read), Verifikation beim Öffnen (Default ein), Zeilen je Row-Group beim Schreiben (Default 8192, Werte unter eins gelten als eins).
- **Entries verwalten:**
  - Upsert mit ID, Vektor und Nutzdaten. Der Vektor muss genau Dimension × 2 Bytes lang sein, sonst Schema-Fehler mit erwarteter und gefundener Länge. Checksum berechnen, Stamp vergeben, Index als dirty markieren, Signal auslösen, loggen. Die Änderung liegt bis zum Save nur im Speicher.
  - Schreiben über den allgemeinen Vertrag: genau ein Vektor, Nutzdaten eines vorhandenen Entries bleiben erhalten, bei neuen sind sie leer. Falsche Länge ist ein nicht behebbarer Programmfehler.
  - Remove: entfernen, als dirty markieren, Stamp vergeben, Signal auslösen. Bei Fehlen ändert sich nichts und es wird „nicht vorhanden“ gemeldet.
  - Abfragbar: Nutzdaten, Pfad, Header, Max-Stamp, Dirty-Status. Eine explizite Verifikation dekodiert vollständig, vergleicht jede Checksum und nennt beim ersten Fehler die Entry-ID. Ein Index ohne Datei gilt als verifiziert.
- **Persistentes Sichern (Save):**
  - Gleichzeitige Saves laufen hintereinander. Ohne Änderung wird nichts geschrieben und „keine Änderung“ gemeldet, sonst „geschrieben“.
  - Geschrieben wird ein konsistenter Snapshot. Während des Schreibens eintreffende Änderungen bleiben als dirty vermerkt.
  - Die Datei enthält alle aktuellen Entries aufsteigend nach ID, aus Ursprungsdatei und Speicher, mit unveränderten Stamps, Nutzdaten und Checksums. Der Header trägt den aktuellen Max-Stamp.
  - Atomares Schreiben: Temp-Datei neben dem Ziel, vollständig auf den Datenträger sichern (fsync), per Rename ersetzen, übergeordnetes Verzeichnis fsyncen. Bei Fehlern wird die Temp-Datei gelöscht, die bisherige Datei bleibt unangetastet, gemeldet als „Index nicht verfügbar“.
  - Row-Groups der konfigurierten Größe, keine Kompression, Statistiken je Page, keine Dictionary-Encoding der Vektorspalte, Header als Key-Value-Paare im Datei-Footer.
  - Geloggt werden Pfad, Schema-Version, Modell, Zeilenzahl und Row-Group-Zahl.

### Dateiformat und Modellbindung

- Spalten in dieser Reihenfolge, keine nullable:
  - Entry-ID (u64),
  - Change-Stamp (u64), eigene Spalte, damit das Einlesen der Stamps keine Vektoren lädt,
  - Vektor (Fixed-Size-List gemäß Dimension aus FP16-Werten),
  - Zieldatei (Text),
  - Zeichenanfang und Zeichenende (je u64),
  - Checksum (u64).
- Der Header enthält Schema-Version (aktuell eins), Max-Stamp, Hash-Algorithmus und Hash-Version, Model-ID und Model-Revision, Dimension, Pooling, Normalisierung, Query- und Passage-Präfix, Storage-Datentyp, Compute-Datentyp, maximale Sequenzlänge und Fingerprint des Tokenizers.
- Ableitung aus einer Modellbeschreibung: Storage-Datentyp ausschließlich FP16, sonst Schema-Inkompatibilität. Hash-Algorithmus schneller 64-Bit-Hash, Hash-Version eins, Max-Stamp null. Compute-Datentyp als Text für beide Prozessoren. Dimension und Vektorgröße (Dimension × 2) sind ableitbar, der Header ist mit anderem Max-Stamp kopierbar.
- Die Bindung gilt nur bei Gleichheit von Schema-Version, Hash-Algorithmus, Hash-Version, Model-ID, Model-Revision, Dimension, Pooling, Normalisierung, beiden Präfixen, Storage-Datentyp, maximaler Sequenzlänge und Tokenizer-Fingerprint. Max-Stamp und Compute-Datentyp sind informativ, weil gemischtes Indexieren auf beiden Prozessoren erlaubt ist.
- Der Header ist verlustfrei als Key-Value-Paare les- und schreibbar. Enum-Werte: Pooling (masked mean, per token), Normalisierung (L2, none), Datentypen (wie Elementtypen), Hash-Algorithmus (fast 64 bit, fast 128 bit, cryptographic).
- Der Tokenizer-Fingerprint besteht aus Algorithmus-Bezeichnung, Doppelpunkt und dem 64-Bit-Hash der Tokenizer-Definition in 16 hexadezimalen Stellen mit führenden Nullen.

### Nutzdaten und Checksum

- Jeder Entry verweist auf den Quelltext: Zieldatei, Zeichenanfang, Zeichenende. Default ist leere Datei mit null und null.
- Die 64-Bit-Checksum wird aus Vektorlänge, Vektorbytes, Dateinamenlänge, Dateiname, Zeichenanfang und Zeichenende berechnet. Vorangestellte Längen schließen mehrdeutige Konkatenationen aus. Jede Bitänderung ist erkennbar.
- Sie wird beim Speichern berechnet und bei der Verifikation neu berechnet und verglichen.

### Erkennung von Dateiänderungen

- Änderungen an Quelldateien werden über eine Fallback-Kette erkannt: Change-Stamp, dann Größe und Timestamp, zuletzt Content-Hash.
- Je Datei werden Größe, Zeit der letzten Inhaltsänderung (mtime), optional Zeit der letzten Metadatenänderung (ctime) und der Referenzzeitpunkt der Iteration festgehalten.
- Eine Aufzeichnung ist unsicher (racy), wenn ihre Änderungszeit höchstens zwei Sekunden vor dem Referenzzeitpunkt oder danach liegt. Das deckt grobe Timestamp-Auflösung mancher Dateisysteme ab.
- Ergebnis „unverändert“ nur, wenn Größe und beide Zeiten übereinstimmen und die Aufzeichnung sicher ist, sonst „Content-Prüfung nötig“ mit Diagnose-Log.
- Für die Content-Prüfung dient ein schneller, nicht kryptografischer 64-Bit-Hash, dessen Algorithmus und Version im Index-Header stehen.

## Manager

### Rolle und Schichten

- **Index:** kennt Entries, Vektordaten und seine Traversal-Struktur.
- **Manager:** kennt die Zuordnung von Entry zu Segment und Slot, die Residency und die Update-Queue. Verantwortlich für Upload, Eviction, Updates und Rückabbildung von Adressen auf Entries.
- **Kernel:** kennt nur Segmente, Adressen und Offsets, keine Entry-Identität und keine Traversierung.
- Der Manager ist eine Komponente der jeweiligen Subengine, weder der Fassade noch des Index. Er wird mit einer Subengine und einem lazy aktivierten Index konstruiert.

### Gemeinsames Verhalten

- Ein Manager hängt seinen Index erst bei Bedarf an (Attach) und hält bis dahin weder Ressourcen noch Subscriptions.
- Attach liefert „true“ bei Annahme. Ist bereits ein Index angehängt, bleibt dieser bestehen, es wird „false“ geliefert und gewarnt.
- Die Aktivierung mit Init-Funktion initialisiert nur beim ersten Gebrauch (Once-Semantik). Gleichzeitige Aufrufer warten auf dieselbe Initialisierung. Bei Fehlschlag bleibt der Manager inaktiv.
- Zugriff ohne angehängten Index ergibt „Index nicht verfügbar“ mit Index-ID.
- Abfragbar über dieselbe Schnittstelle: Aktivität, Zahl residenter Entries. Zyklen laufen in einem Diagnose-Span mit Index-ID.

### CPU-Variante

- Lesezugriffe gehen direkt an den Index, da das RAM die Primärinstanz ist. Aufgabe ist die Residency dateigestützter Daten im Page-Cache, relevant bei memory-mapped Indizes größer als das RAM, sowie das Unload nach Leerlauf.
- Einzelnes Lesen: Hinweisbereich setzen („will need“), im Blocking-Pool lesen, Kopie der Bytes liefern. Abbruch der Aufgabe ergibt Transfer-Error „Lese-Task abgebrochen“.
- Mehrfaches Lesen mit Zugriffsmuster: bei dichtem Zugriff „sequential“ über den umspannenden Bereich, bei verstreutem „random“ je Entry. Ergebnisse in Eingabereihenfolge. Leere Eingabe liefert leeres Ergebnis, der Index muss angehängt sein.
- Eine Variante erfragt die Hinweisbereiche selbst vom Index. Entries ohne zusammenhängende Speicherung werden ohne Hinweis gelesen.
- Bereiche lassen sich nach Leerlauf als „cold“ markieren. Ohne Hinweisquelle passiert nichts, Ablehnungen werden nur geloggt.

### Page-Cache-Hinweise und blockierende Lesezugriffe

- Empfehlungen (Advice) an das Betriebssystem je Byte-Bereich: will-need (Prefetch), sequential, random, cold. „Cold“ senkt die Reclaim-Priorität, ohne Inhalte zu verwerfen. Alle Empfehlungen sind unverbindlich.
- Das Hinweisziel ist über eine austauschbare, nebenläufige Schnittstelle (Trait) abstrahiert.
- Ungültige Bereiche (Anfang hinter Ende, Ende hinter Mapping-Länge) werden abgewiesen. Leere Bereiche sind wirkungslos erfolgreich (No-op).
- „Cold“ wird auf Page-Grenzen ausgerichtet (Anfang abrunden, Länge vergrößern) und ist auf Plattformen ohne Unterstützung ein erfolgreicher No-op. OS-Fehler werden weitergereicht.
- Blockierende Lesezugriffe (Page-Faults, Disk-I/O) laufen im Blocking-Pool, nie im Thread des Async-Runtime. Der Prefetch-Hinweis sorgt dafür, dass I/O und Kernel-Lauf überlappen.
- Vor dem Lesen wird standardmäßig „will-need“ gesetzt, abweichend etwa „sequential“ für dichte Scans. Lesen ohne Hinweis ist möglich, Prefetch ist eine eigene Funktion. Ohne Hinweisquelle bleibt der Hinweis wirkungslos, ein abgelehnter Hinweis wird nur geloggt. Abbruch oder Panic der blockierenden Aufgabe wird als Fehler gemeldet.

### GPU-Variante

- Scopes werden über die Resolver-Registrierung aufgelöst.
- Ensure einer Menge: angehängter Index, Laden über den Loader, danach alle Segmente als benutzt markieren, bewegte Bytes loggen.
- Reconcile-Iteration: Reconciler beim ersten Mal anlegen, residente Entries mit dem Index vergleichen, Entfernte als Delete-Updates einstellen, Geänderte ohne ausstehendes Update regulär nachladen, Ergebnis zurückgeben.
- Publish eines geladenen Segments:
  - fortlaufende Segment-ID vergeben,
  - Region in eine neue Tabellenversion und Segmentzuordnung in eine neue Mapping-Epoche aufnehmen; weist die Zuordnung ab (etwa Überlappung), Transfer-Error,
  - zuerst Tabelle, dann Zuordnung publishen,
  - bei Fehlern die Region zurückgeben,
  - bei Erfolg das Segment tracken (Größe, Tier Vektor, Reload-Kosten gleich Größe, Zeitpunkt jetzt) und die mitgelieferten Bindings mit Segment-ID und Stamp als Updates einstellen, wirksam in der nächsten Gap.
- Abfragbar: Segmentzahl, residente Bytes aus Eviction-Sicht, ausstehende Updates.
- Freigeben eines Segments: Region zurückgeben, aus der Eviction entfernen, Residency aller Entries löschen. Noch anstehende Bindings in dieses Segment scheitern schadlos.
- Eviction-Schritt und Platzbeschaffung sind verfügbar, jede Eviction wird mit Index, Segment, Bytes und Grund geloggt.
- Der Abschluss eines Communication-Slots verlangt einen angehängten Index, sperrt Updates und Residency und läuft gegen die Zuordnung der Batch-Epoche.

### Scope-Auflösung

- Der Manager löst den Scope einer Operation in eine kompakte Entry-Menge auf, nicht der Index.
- „All“ liefert alle Entries, eine explizite Menge wird übernommen.
- Andere Arten brauchen eine Zugriffsmethode: Der Index deklariert das Verfahren und der Manager hat einen Resolver dafür. Sonst „Scope nicht unterstützt“ mit Methodenname. Ein stilles Ausweichen auf alle Entries gibt es nicht.
- Standard-Resolver für Range, Subtree, Prefix und Centroid. Doppelte Registrierung wird abgewiesen.
  - Range: genau der angegebene Bereich.
  - Subtree: die Kinder des angegebenen Knotens.
  - Prefix: führende Bytes der 64-Bit-ID in Big-Endian als zusammenhängender Bereich. Über acht Bytes trifft nichts, leeres Präfix deckt alles ab, Obergrenze begrenzt auf den größten Wert.
  - Centroid: braucht die Summary-Struktur des Index (sonst „Index nicht verfügbar“) und einen Cell-Selector im Kontext (sonst „nicht unterstützt“). Der Selector wählt die nächsten Zellen mit Anzahl und optionalem Radius. Höchstens die angegebene Anzahl Zellen, Ergebnis ist die Union ihrer Mitglieder.
- Ein falscher Scope für einen Resolver ergibt „Scope nicht unterstützt“. Die Auflösung läuft in einem Diagnose-Span mit Methode und loggt die Anzahl.
- Ablauf am Beispiel Centroid: Der Manager macht die Summary-Struktur resident, Subengine und Kernel rechnen die Query gegen die Centroids, danach fragt der Manager die Mitglieder der gewählten Zellen ab und gleicht gegen den Ladezustand ab (fehlt etwas, ist etwas stale).
- Ist die Kandidatenmenge aus einer Vorverarbeitung bekannt (mehrstufiges Centroid-Verfahren nach Art von PLAID, Pruning, Prefix, Fourier- oder Kompressionsdomäne), lädt der Manager nur diese Bereiche.

### Konsistenzmodell

- Der Index gewinnt bei Abweichung. Die Identität eines Entries ist das Paar aus ID und Stamp. Stamps werden nur auf Ungleichheit verglichen.
- Der Abgleich ist zustandsbasiert (State-based Reconciliation): Das Signal weckt nur, verglichen werden residente Entries. Änderungen an nicht geladenen Entries brauchen keine Behandlung, weil das Laden ohnehin den aktuellen Stand liest. Der Manager-State skaliert mit dem geladenen Teil.
- Segmenttabellen werden per Copy-on-Write verändert und per Swap des Root-Pointers publiziert. Abgelöste Versionen werden erst frei, wenn alle Reader die neue Epoche erreicht haben. Mapping-Änderungen folgen der Tabelle in der Gap zwischen Batches.
- Last-Writer-Wins. Änderungen am Index werden serialisiert angewendet.

### Zustandsbasierter Abgleich und Echo-Vermeidung

- Jeder Subscriber (GPU-Manager, Persistenz-Timer, CPU-Manager) hat seinen eigenen Reconciler mit eigener Sicht.
- Der Reconciler wartet auf ein ungesehenes Signal und meldet „false“, wenn der Index nicht mehr existiert. Ein anstehendes Signal ist abfragbar (Polling).
- Das Signal wird vor dem Vergleich quittiert (Ack). Ein währenddessen eintreffendes Signal löst den nächsten Abgleich aus.
- Vergleich residenter Entries: fehlt im Index, gilt als entfernt; abweichender Stamp, zum Reload vorgemerkt; gleicher Stamp, aktuell, einschließlich Echos eigener Writes.
- Ergebnis: beide Listen aufsteigend sortiert, Zahl aktueller Entries, Watermark des Signals, Aussage „clean“. Entfernungen sind als Delete-Updates einstellbar.
- Laden eines Entries in der Reihenfolge Stamp, Daten, erneuter Stamp. Bewegt sich der Stamp, wird bis zu achtmal wiederholt, danach mit dem zuerst gelesenen Stamp zurückgekehrt. Ergebnis: erster Stamp, Daten, Retries, bei Fehlen nichts. Die umgekehrte Reihenfolge würde stale Daten als aktuell ausweisen und ist die einzige unsichere Richtung. Ein überzähliges Nachladen ist harmlos.
- Echo-Suppression: Der Manager trägt den Stamp aus dem Rückgabewert seines eigenen Upserts als bekannt ein, nie aus einem späteren Lesen. Ohne Rückgabewert entsteht höchstens ein redundanter Upload.
- Zwei Subengines, derselbe Entry: Der andere Manager sieht einen abweichenden Stamp und lädt nach. Beide konvergieren auf den Index-Stand.
- Entfernen und Neuanlage werden über die Gesamtliste der Stamps bzw. die Stamp-Abfrage je Entry erkannt.

### Residency-Verzeichnis

- Je residentem Entry werden Slot (Segment und Slot-Nummer) und bekannter Stamp geführt.
- Insert mit Rückgabe des Vorgängers, Remove, Abfrage von Entry oder Stamp, Anzahl, Leere, Iteration.

### Kernel-Schnittstelle und Layouts

- Der Kernel liefert nur Adresse bzw. Offset und Score. Die Adressdarstellung ist Implementierungsdetail. Während der Berechnung gibt es keine Kommunikation.
- Festes Layout mit Stride: Entry-Grenzen per Arithmetik. Variables Layout: Offset-Tabelle aus Prefix-Sums, vom Manager aus der Index-Struktur gebaut.
- Der Kernel traversiert nie. Die Max-Similarity (MaxSim) aggregiert je Entry über Stride oder Tabelle, ohne Entry-Identität.
- Eine Kandidatenmenge ist ein variables Layout, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.
- Rückabbildung durch den Manager: bei festem Layout per Arithmetik, bei variablem per Binary Search oder Interval-Mapping auf Arena-Offsets.

### Segmente und Slots

- Slots haben feste Größe aus Token-Obergrenze, Dimension und Datentypgröße. Vordefinierte Slot-Klassen für 32, 64 und 128 Token, frei wählbar bis 65 535 Token, damit der Kernel ohne Lookup auskommt. Overflow oder ein Ergebnis über 32 Bit ergibt einen Fehler.
- Eine Segment-View besteht aus Region, Slot-Anzahl, Slot-Klasse, Dimension, Datentyp und Segment-Generation. Beim Anlegen wird geprüft, ob die Region alle Slots fasst, sonst Fehler mit benötigten und verfügbaren Bytes. Gesamtgröße und Teilbereich je Slot sind abfragbar, ungültige Slot-Nummern ergeben Fehler mit Nummer und Anzahl. Für Kernel gibt es einen Descriptor mit Startadresse, Slot-Anzahl und festem Abstand (Stride).
- Encoding von Slot-Inhalten: echte Token-Zeilen in Eingabereihenfolge, danach Padding-Zeilen bis zur Klassengrenze mit endlichem Mask-Wert statt null, damit Padding nie eine Similarity von genau null beiträgt. Datentypen FP32, FP16 und BF16 in Little-Endian, andere werden mit Typnennung abgelehnt. Zu viele Token oder eine Wertezahl ungleich Token × Dimension werden abgelehnt. Das Ergebnis hat volle Slot-Größe.
- Je Slot gibt es gültige Tokenzahl und Valid-Bit (Tombstone, noch nicht befüllt).
- Je Segment eine nebenläufig les- und schreibbare Zuordnung von Slot zu Entry-Referenz (Entry-ID plus Abschnittsnummer), mit fester oder variabler Adressierung. Bind, Unbind, Lookup, Slot zu Device-Adresse, absoluter Byte-Bereich je Slot und umspannender Bereich (leer ohne Ergebnis).
- Segmente wachsen nur durch Append. Belegte Slots eines geladenen Segments werden nie überschrieben. Speicher gelöschter Entries wird durch Rebuild des Segments zurückgewonnen (Compaction).
- Schreiben:
  - Reservierung des nächsten freien Slots mit Entry-Referenz, Tokenzahl, optionaler Centroid-Zuordnung und Zielbereich. Der Slot bleibt bis zum Commit unsichtbar.
  - Abgelehnt bei Tokenzahl über Klassengrenze oder 16-Bit-Bereich sowie ohne freien Slot.
  - Commit sofort, wenn die Übertragung abgeschlossen ist (sonst „noch nicht bereit“), oder deferred bis zum Event. Der Drain wendet deferred Commits in Reihenfolge an, hält beim ersten offenen Vorgang an und liefert die Zahl neu sichtbarer Slots.
  - Commit bindet den Slot, setzt Metadaten (gültig, Tokenzahl, fortlaufende Write-Nummer, Centroid-Zuordnung) und das Valid-Bit erst nach dem Event.
  - Insert oder Update heißt neuer Slot, danach wird das Bit des alten Slots gelöscht. Die alte Zuordnung bleibt für ältere Treffer auflösbar.
- Ein Entry lässt sich ausblenden (Tombstoning), ohne Daten zu entfernen, mit Rückmeldung über sein Vorhandensein. Abfragbar sind Validität (außerhalb des Bereichs ungültig), Anzahl gültiger Slots, Metadaten, Slot eines Entries und ein Snapshot der Valid-Bits in 64-Bit-Wörtern zum Upload. Valid-Bits sind thread-safe.
- Treffer:
  - Kernel-Treffer aus Adresse und Score werden in Segment-Generation, Slot und Score umgerechnet, sonst „Slot außerhalb des Bereichs“.
  - Auflösung über ein Manifest zu einer Entry-Referenz. Fehlt das Segment: „stale Generation“. Ungebundener Slot: „fehlende Zuordnung“ mit Generation und Slot.
  - Der neueste gültige Ort eines Entries wird ermittelt, neuere Segmente haben Vorrang.
  - Treffer einer älteren Manifest-Generation werden überführt: Auflösung im aktuellen, sonst im vorherigen Manifest. Unauflösbare werden verworfen. Ist der Slot noch gültig, bleibt der Treffer, sonst wird der aktuelle Ort gesucht und der Treffer mit unverändertem Score umgesetzt oder verworfen.

### Manifeste und Publication

- Ein Manifest ist ein Snapshot der Segmente mit fortlaufender Generation, beginnend leer bei null. Ein Nachfolger liegt genau eine Generation höher. Abfragbar: Segmente, Generation, Gesamtbytes, gehaltene Leases.
- Die Segmenttabelle für Kernel ist binär mit 24 Byte je Segment in Little-Endian: Segment-Generation (8 Byte), Startadresse (8 Byte), Slot-Anzahl (4 Byte), Slot-Abstand (4 Byte, null bei variablem Layout).
- Das aktuelle Manifest ist für Reader lock-free verfügbar und atomar ersetzbar, mit Rückgabe des vorherigen. Eine nicht strikt größere Generation wird mit „stale Generation“ abgelehnt. Konkurrenz wird durch Retry aufgelöst, Störungen sind für Tests auslösbar.
- Ein Reader sichert die aktuelle Generation per Lease. Solange sie besteht, bleiben Manifest und Segmente erhalten. Über die Lease werden Completion-Marker bei allen Segmenten registriert, bereits abgeschlossene dürfen verworfen werden.
- Der Speicherbedarf eines Segmentwechsels ist aktueller Bestand plus eingehende Menge.
- Nicht mehr gebrauchte Segmente melden ihre Region samt offener Completion-Marker (auch deferred Writes) an die Speicherverwaltung, sobald kein Manifest und keine Lease sie hält. Das Reclaim liefert Regionen mit vollständig abgeschlossenen Markern und hält den Rest zurück.
- Neue Manifeste können hinter dem Event ihres Kopiervorgangs vorgemerkt werden: Copy-Auftrag auf der Copy-Lane einreihen, Event aufzeichnen, Manifest vormerken. Fehler ergeben einen Sync-Error mit Ursache. Eine Abfrage publiziert vorgemerkte Manifeste in Reihenfolge, solange das vorderste abgeschlossen ist, und liefert die Anzahl.
- Change-Notifications an beliebig viele Subscriber mit konfigurierbarem Buffer. Events mit Art, Segment-ID, Segment-Generation und Manifest-Generation: „added“ für neue IDs, „changed“ bei anderer Segment-Generation, keines für unveränderte. Senden ohne Subscriber scheitert nicht.

### Mutationsdisziplin

- Der Manager kennt den Laufzustand des Kernels und verändert nur in Gaps (Batch-Ende bzw. Communication-Slot). Er ändert nichts unter dem Kernel und blockiert oder sperrt ihn nie aus.
- Atomar getauscht wird nur ein natürlich ausgerichteter Pointer von acht Byte Breite. Größere Strukturen werden per Copy-on-Write neu angelegt, danach wird der Pointer getauscht. Hierarchie: Root-Pointer, Segmenttabelle, Segment (Basis, Layout, Offset-Pointer). Das gilt für Lookup-Tabellen und ganze Segmente, getauscht wird, sobald die Subengine dort nicht mehr liest.
- Update in-place ist in einer Gap trivial. Im Persistent-Modus kann der Kernel einen halb aktualisierten Vektor mit gemischten, aber gültigen Werten lesen (Torn Read). Das wird bewusst akzeptiert und dokumentiert.
- **Root-Pointer:** nur der Host schreibt, mit Release-Semantik; das Device liest mit Acquire-Semantik. Vor der ersten Tabelle trägt er den größten darstellbaren Wert als Sentinel. Er kann in fest zugeordnetem, gemapptem Host-Memory liegen.
- **Segmenttabelle:** publizierte Versionen sind immutable, neue sind Kopien mit erhöhter Epoche, mit Epoche und Segment-Regionen. Add, Replace (false bei fehlender alter Region) und Remove mit Rückmeldung. Kodierte Größe: 16 Byte plus 24 Byte je Segment.
- **Epoch-Tracking:** publizierte Epoche und je Reader die zuletzt gemeldete, nur steigend. Ein beendeter Batch meldet seine Epoche. Ein Reader ohne laufenden Batch kann ruhen (Quiescent) und gilt dann als angekommen. Die kleinste Reader-Epoche ist abfragbar, ohne Reader gleich der publizierten.
- **Table-Swap:**
  - Das Schreiben in device-sichtbaren Speicher kehrt erst zurück, wenn der Inhalt vollständig sichtbar ist (Copy-Event oder Flush gemappter Writes).
  - Eine Kopie der aktuellen Tabelle mit nächster Epoche ist verfügbar.
  - Publication serialisiert: Epoche genau aktuell plus eins, sonst „stale Mapping“, sodass zwei Tabellen gleicher Basis nicht beide publiziert werden. Inhalt sichtbar schreiben, Root tauschen, Epoche voranschieben, vorherige Region mit neuer Epoche zurückstellen (Retire), loggen (Epoche, Segmentzahl, Offset), Epoche zurückgeben.
  - Reclaim höchstens bis zur kleinsten Reader-Epoche und gewünschten Obergrenze, mit Anzahl.
  - Normale Launches erhalten Root und Epoche als Kernel-Argument; ein Swap wirkt ab dem nächsten Launch.
  - Im Persistent-Kernel-Modus mit Ring-Buffer liest der Kernel den Pointer einmal je Work-Item mit Acquire-Semantik. Ein Guard-Objekt hält die Version fest und meldet beim Freigeben die Epoche. Der Inhalt der neuen Tabelle muss vorher per Event oder Memory-Barrier sichtbar sein.
  - Auf echter Hardware ist nachzuweisen, dass der Inhalt vor dem Root-Wert sichtbar ist, auch über einen Copy-Stream mit Event.

### Doorbell-Synchronisation zwischen Batches

- Die GPU publiziert die Zahl abgeschlossener Batches mit Release-Semantik, der Host liest mit Acquire-Semantik.
- Der Host verändert nur zwischen Abschluss eines Batches und dessen Ack. Das Device prüft vor jedem weiteren Batch, ob das Ack den Vorgänger abdeckt. Der erste Batch darf immer starten.
- Der Host wartet nie auf den Kernel, nur auf ein Batch-Event.
- Nicht blockierende Abfrage liefert eine Gap bei Abweichung zwischen completed und acked. Koaleszierte Completions öffnen eine Gap mit der höchsten Batch-Nummer. Die wartende Abfrage wartet zuerst auf das Event, Device-Fehler werden weitergereicht.
- Schließen der Gap publiziert das Ack. Eine nicht geschlossene Gap ist ein Programmierfehler. Vorher geschriebene Ergebnisse und Zuordnungen sind sichtbar.
- Beide Zellen können in fest zugeordnetem, mappbarem Host-Memory liegen.

### Communication-Slot

- Feste Reihenfolge:
  - Lesen, was der Kernel geschrieben hat: deferred und neue Writes Slot für Slot verarbeiten. Ist eine Adresse keinem Segment zugeordnet, wird der Rest für den nächsten Slot zurückgestellt. Gebundene Slots werden vom Device zurückgelesen, per Upsert am Index angewendet und der zurückgegebene Stamp im Residency-Verzeichnis vermerkt. Slots ohne Entry werden übersprungen, der Fortschritt beträgt mindestens ein Byte.
  - Ergebnisse auflösen: Adresse auf Entry abbilden. Nicht abbildbar oder im Index entfernt: verwerfen. Update ausstehend: als „stale“ markiert liefern (Diagnose, kein Fehler). Sonst unverändert liefern.
  - Updates einspielen. Das verändert die Abbildung und kommt daher zuletzt.
- Geloggt werden angewandte Writes, aufgelöste, verworfene und stale Treffer, Queue-Tiefe und Änderungen.
- Weil nur in Gaps verändert wird, sieht jedes Ergebnis den Mapping-Stand seines Batches. Eine gemeldete Adresse ist aktuell. Im Kernel-Protokoll gibt es keine Epoche und keine Sequenznummer.
- Läuft der Slot gegen eine weitergegangene Zuordnung, wird nebenwirkungsfrei „stale Mapping“ gemeldet und der Slot unter Structure-Lock mit frischem Snapshot wiederholt. Danach wird zurückgestellter Speicher zurückgewonnen.
- Das Zurücklesen von Device-Bereichen läuft über eine austauschbare Schnittstelle (Trait).
- **Address-Map:** bildet Arena-Adressen auf Entries ab, Segmentmenge per Copy-on-Write verändert, Klone sind stabile Snapshots. Epoche wird mitgeführt, Nachfolger erhöhen sie um eins. Add abgelehnt bei leerem Segment, vergebener ID oder Überlappung; Segmente nach Adresse sortiert. Remove meldet Vorhandensein. Slot-Bindings liegen in gemeinsamen Zuordnungen und ändern sich nur in Gaps.
- **Mapping-Store:** hält die Map versioniert, liefert Snapshots nur bei passender Epoche (sonst „stale Mapping“ mit Basis- und publizierter Epoche) und publiziert nur Maps mit Epoche aktuell plus eins.
- **Update-Queue:** Unbind oder Bind an einen Slot mit Stamp. Je Entry nur das jüngste Update. Abfragbar: Containment, Tiefe, deferred Writes. Anwenden zuerst Deletes, dann Binds: Unbind entfernt aus dem Residency-Verzeichnis und löst die Slot-Bindung nur, wenn sie noch dazugehört; Bind löst zuerst eine alte Bindung; nicht abgebildete Ziele erzeugen eine Warnung. Rückgabe ist die Zahl angewandter Updates. Änderungen bleiben bis zur Ausführung in der Queue.

### Index-Aufbau durch Kernel

- Der Manager vergibt vorab einen beschreibbaren Bereich aus der Arena. Der Kernel schreibt per Cursor hinein und allokiert nie.
- Write-Pointer-Modell: Jeder Claim schiebt den Pointer voran. Passt einer nicht mehr, scheitern alle späteren, sodass die gültigen Bytes ein zusammenhängendes Präfix bilden. Ein gescheiterter Claim schreibt nichts, setzt einen Overflow-Zustand und hält die kleinste Overflow-Position fest; der erste Overflow wird geloggt. Gruppen-Claims schieben mit einer einzigen Aktualisierung voran. Gültige Länge ist das Minimum aus Overflow-Grenze, Pointer und Kapazität.
- Auf der GPU fasst ein Warp aus 32 Threads seine Claims per Prefix-Sum zusammen, mit einer atomaren Aktualisierung je Warp. Bei Overflow wird ein Invalid-Marker ausgegeben, Grenze und Flag gesetzt. Ergebnisse werden für den Host sichtbar gemacht.
- Am Batch-Ende meldet der Kernel die belegten Bereiche. Reicht der Bereich nicht, liefert er ein Teilergebnis (Partial Result) und der Manager vergibt den nächsten.
- Ausführung eines Embedding-Batches: fortsetzen (Resume), bis der Kernel Treffer meldet. Vor jedem Schritt wird die Höchstzahl an Bereichen geprüft, sonst „Arena erschöpft“. Je Schritt ein Bereich reservieren und zuweisen, Input hinter den verarbeiteten Bytes fortsetzen. Teilantworten zählen, kein Fehler. Der Batch wird immer beendet. Erfolg liefert Bereiche, Regionen und Teilantworten. Fehler geben alle reservierten Bereiche frei, gemeldet wird zuerst der Lauffehler.
- Adressierung gebauter Segmente: exklusive Prefix-Sum auf Basisadresse (n Längen ergeben n+1 Offsets), Offset-Tabelle mit Adress-zu-Entry-Abbildung. Das Layout ist fest, wenn der Stride der größten Entry-Länge entspricht, in 32 Bit passt, größer null ist und der Füllanteil eine Promille-Schwelle nicht überschreitet, sonst variabel. Beide Layouts bilden Adressen ab und sind in eine Slot-Zuordnung umwandelbar.
- Embedding-Writes und Treffer laufen über getrennte Queues mit je einem Producer und Consumer (SPSC). Writes werden zuerst registriert, Treffer erst danach. Der Consumer entnimmt zuerst alle Writes, dann alle Treffer, sodass kein Treffer ohne Adresszuordnung beobachtet wird. Indexieren während der Suche funktioniert, weil das Lesen der Writes vor der Ergebnisauflösung liegt. Enqueue und Tiefe sind abfragbar und geloggt.

### Speicherverwaltung für Index-Segmente

- Segmente sind auf mindestens 256 Byte ausgerichtet. Ein konfigurierbarer Reserve-Anteil wird nie vergeben.
- Vor jeder Zuteilung werden ausstehende Freigaben eingesammelt. Zugelassen wird nur, wenn Belegtes plus Anforderung plus Replacement-Headroom plus Reserve in die Kapazität passen, sonst „Arena erschöpft“ mit angeforderter und verfügbarer Menge.
- Fehler des darunterliegenden Speichers: Erschöpfung als „Arena erschöpft“, stale Region als solche, Übriges gekapselt (wrapped).
- Die Arena nutzt eine geliehene Referenz oder geteilten Besitz (Arc) am Speicherverwalter.
- Bookkeeping: lebende Bytes, größtes Segment, zurückgestellte Segmente. Replacement-Headroom ist Zahl gleichzeitiger Replacements (Default eins) mal größtes Segment, bei regulärer Zuteilung unter Berücksichtigung der neuen Anforderung freigehalten, beim Replacement-Beginn nicht erneut verlangt.
- Budgets overflow-sicher: Replacement-Budget (lebende Bytes, Replacements, größtes Segment) und Arena-Budget (Kapazität, Reserve, lebend, zurückgestellt, Headroom, verfügbar). Verfügbar ist Kapazität minus Belegtes, Reserve und Headroom, nie negativ. Auslastung ist loggbar.
- Freigabe sofort nur ohne mögliche Reader, sonst nach Event. Jede Freigabe prüft Liveness und Nicht-Zurückstellung, sonst „stale Region“. Ersetzte Regionen werden mit der Epoche des Replacements zurückgestellt. Das Reclaim gibt alle Regionen frei, deren Epoche nicht über der kleinsten Reader-Epoche liegt; verschwundene erzeugen nur eine Warnung.
- Replacement durch Kopie: neues Segment gleicher Größe neben dem alten, höchstens ein Replacement gleichzeitig, wartend oder nicht wartend („kein Replacement“). Altes Segment muss leben. Abschluss publiziert die Kopie mit Epoche und stellt das alte zurück; scheitert das, bleibt die Kopie beim Vorgang. Discard gibt die Kopie frei. Alte und neue Region sind abfragbar.
- Schrittweise beschreibbare Segmente (Bump-Allocation) für einen Writer: Claims mit Alignment liefern absolute Adressen, passt einer nicht, „Arena erschöpft“ mit Restmenge ohne Zustandsänderung. Geschriebene und verbleibende Bytes sind abfragbar, das Segment ist zerlegbar.

### Residency und Ladestrategie

- Geladen wird lazy und scope-getrieben: nur, was ein Task braucht.
- Eine austauschbare Geometrie liefert Segment eines Entries, Entry- und Segmentgröße. Mitgeliefert: gleich große Entries in gleich großen Segmenten, Kapazität null gilt als eins.
- Zugriffsarten: regulär und selten (kleine, selten gelesene Mengen aus gemapptem Host-Memory).
- Plan-Arten: ganze Segmente laden, Entries gathern und gemeinsam senden, direktes Mapping ohne Kopie (Zero-Copy).
- Policy: Dichteschwelle (Default 0,12 der Segmentbytes, nur endlich, auf null bis eins begrenzt), Segmentgröße (Default 64 MiB), Höchstgröße für direktes Mapping (Default 64 KiB).
- Die Segmentgröße bestimmt der Manager nach Budget: ein Achtel der verfügbaren Menge, begrenzt auf 4 bis 256 MiB, abgerundet auf ganze MiB. Sie ist keine Eigenschaft des Index.
- Die Dichteschwelle ist aus einem kalibrierten Profil mit der Achse „Dichte“ in Promille übernehmbar, sonst unverändert.
- Planung: leere Mengen ergeben keinen Plan. Bei seltenem Zugriff bis zur Höchstgröße genau ein Plan mit direktem Mapping. Sonst je Segment angeforderte Bytes ermitteln; ab Dichteschwelle bei nicht leerem Segment ganzes Segment, Rest gathern. Höchstens ein Plan je Art, Segmente zuerst. Entscheidungen werden geloggt.
- Ausführung: Pläne nacheinander, Abbruch beim ersten Fehler, overflow-sichere Bytesumme.
- Aggregation: Tasks gegen denselben Index werden über die Union ihrer Scopes gebündelt und je Index genau einmal nebenläufig geladen. Der Collector zählt Requests, ist entleerbar und auf Leere prüfbar.
- Single-Flight: Gleichzeitige Requests mit gleichem Key teilen einen Ladevorgang samt Ergebnis, auch im Fehlerfall. Nur der erste startet, die Start-Funktion blockiert nie. Nach Abschluss wird nur der eigene Eintrag entfernt. Fehlschläge wiederholt der nächste neue Aufrufer. Laufende Vorgänge sind abfragbar, Mitläufer (Joiner) werden geloggt.
- Transfers laufen im Blocking-Pool. Ein Abbruch ergibt einen Transfer-Error H2D.
- Gestaffelte Haltung (Tiering): Summary-Strukturen klein, zuerst geladen, am längsten gehalten; Vektorsegmente warm und evictable; alles andere nicht geladen.
- Überlappende Entries: Der Manager entscheidet, ob Vektoren einmal gehalten und mehrfach referenziert oder dupliziert werden. Das ist keine Vertragsfrage.
- Operations-Unit je Index-Operation (Coordinator): verbindet Manager, Table-Swap, Mapping-Store und Queue geladener Segmente und publiziert jedes gemeldete Segment selbständig. Strukturelle Änderungen sind serialisiert. Meldet ein Laden „Arena erschöpft“, werden ausstehende Segmente publiziert, Platz geschaffen und genau einmal wiederholt. Reconcile-Iterationen publizieren Nachgeladenes vor ihrer Rückkehr. Publish versucht alle Segmente und meldet danach den ersten Fehler oder die Anzahl. Verwaltungsfehler werden zu Index-Fehlern: „Arena erschöpft“ bleibt, alle anderen werden Transfer-Error.

### Transfer auf das Device

- Bausteine: Quelle für Segmentbereiche (Index-Arena), gemapptes Host-Memory, geladenes Segment aus Region, Slot-Zuordnung und Bindings mit vor den Daten gelesenem Stamp, austauschbare Sink für geladene Segmente.
- Ganze Segmente: Entries aufsteigend zuordnen. Direktes Kopieren, wenn Entries lückenlos in einem gestützten Bereich liegen, nicht leer und ausgerichtet sind und der Index genau die Bereichslänge liefert. Vorher „sequential“ setzen, Bereich anlegen, kopieren, synchron abwarten, bei Fehler freigeben. Slot-Grenzen sind absolute Adressen mit End-Marker. Nur Entries mit bekanntem Stamp erhalten ein Binding. Nicht direkt kopierbare Segmente gehen in den Gather-Pfad.
- Gather-Pfad: Stamp vor den Daten erfassen, „random“ je Bereich setzen. Wiederholt Staging-Slot bepacken (ohne Fortschritt: „Gather ohne Fortschritt“), Bereich anlegen, eine Kopie absenden, abwarten, melden, Bytes summieren. Beim Bepacken wartet das Gathering auf freie Slots, packt ausgerichtet bis zum ersten nicht passenden Entry, zählt entfernte Entries als erledigt und meldet „Arena erschöpft“, wenn schon der erste nicht passt. Slots werden erst nach dem Event wiederverwendet. Kopien laufen asynchron auf dem Stream einer eigenen Copy-Lane. Gewöhnliches und Pinned Host-Memory werden unterstützt.
- Direktes Mapping: ohne gemappte Arena oder Slot Fallback auf den Gather-Pfad mit Log, sonst Stamp und Daten lesen, ausgerichtet in den gemappten Bereich schreiben und melden. Sehr kleine Mengen werden so zero-copy gelesen.

### Eviction

- Mirrors in VRAM und RAM werden ausschließlich explizit evicted: Idle-Timeout, Füllstandsgrenzen mit Hysterese, kostenbewusste Victim-Auswahl. Gründe: Idle, Pressure, Space-Need.
- Tiers: Summaries und festgehaltene Entries (Pinned) werden nie Victim. Pinning wirkt, solange der Halter lebt.
- Nutzbare Kapazität ist Gesamtkapazität minus Reserve und Headroom, Belegung aus lebenden und zurückgestellten Bytes.
- Policy aus High-Watermark, Low-Watermark und Idle-Dauer. Nicht endliche Marken werden korrigiert, High-Watermark auf null bis eins, Low-Watermark auf null bis High-Watermark begrenzt. VRAM: 0,90 und 0,75 bei 600 Sekunden. RAM: 0,85 und 0,70 bei 60 Sekunden.
- Aufnahme (Track) mit Größe, Tier und Reload-Kosten (nicht endlich oder negativ zählt null). Start mit einem Hit, erneute Aufnahme ersetzt korrekt.
- Priorität nach dem Prinzip GreedyDual-Size-Frequency: Floor plus Hits mal Kosten durch Größe (Größe null als eins). Der Floor steigt bei Pressure- oder Space-Need-Eviction auf die Victim-Priorität.
- Zugriffe erhöhen Hits, setzen den Zeitpunkt und berechnen neu. Remove meldet Bytes. Anzahl, Bytes und Containment sind abfragbar.
- Idle-Eviction evicted deterministisch nach Key. Entlastung (Pressure-Relief) erst über der High-Watermark bis auf die Low-Watermark, Victim-Reihenfolge: niedrigste Priorität, dann am längsten unbenutzt (LRU), dann kleinerer Key. Der periodische Einstieg (Tick) macht beides nacheinander.
- Platzbeschaffung: genug frei, nichts tun; sonst Victims sammeln; reichen alle nicht, „Arena erschöpft“ ohne Eviction; sonst mit Grund Space-Need evicten.
- Entfernen evicteter Segmente: aus neuer Tabelle und Zuordnung entfernen, publizieren, Region hinter der neuen Epoche zurückstellen, freigeben. Unbekannte Regionen überspringen, bei Fehlschlag das Segment wieder unter Kontrolle stellen und abbrechen.
- Jede Entscheidung wird mit Victim, Größe, Hits und Grund geloggt.

### Konsistente Snapshots

- Gleichzeitig veränderbare Daten werden nach dem Prinzip eines Seqlocks wortweise atomar gelesen, nie halb geschrieben.
- Der Reader liefert eine vollständige Kopie und die Zahl der Retries, solange ein Writer aktiv ist oder der Zähler sich änderte. Writer schließen sich aus, decken die gesamte Region ab und erhöhen den Zähler um zwei.
- Aufbau aus rohen Adressen ist erlaubt, wenn Alignment, Gültigkeit und einheitliches Protokoll zugesichert sind.

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

## Precision, Kalibrierung und Profile

### Prinzipien

- Zwischen CPU und GPU gibt es keinen Rounding- oder Datentyp-Vertrag.
- Der Precision-Parameter ist ein Wunsch zwischen null (schnellste) und eins (präziseste Kombination), keine Zusicherung.
- Die Kalibrierung unterbricht oder blockiert den Nutzer nie.

### Capability-Achsen und Capability-Sets

- Achsen und Werte werden über frei wählbare, erweiterbare Strings identifiziert, vergleichbar, sortierbar, als Key nutzbar, günstig kopierbar, lesbar ausgebbar. Werte sind auch aus natürlichen Zahlen erzeugbar.
- Ein Capability-Set ordnet jeder Achse eine nie leere Wertemenge zu, mit Add, Containment, Enumeration und Leere. Enumeration, Vergleich, Hash und Serialisierung sind reihenfolgeunabhängig.
- Ein Set erfüllt ein Requirement, wenn jeder geforderte Wert vorhanden ist. Leere Requirements sind immer erfüllt.
- Vordefinierte Achsen: Operanden, Akkumulator, Scoring, Fusion, Batch-Größe, Tile-Größe, Thread-Anzahl, Dichte. Werte: FP32, FP16, FP16 gespeichert mit FP32 gerechnet, INT8, INT8 über den DP4A-Befehl der GPU, INT8 über VNNI-Befehle der CPU (Integer-Befehle für neuronale Netze), on, off.
- Referenz-Sets als Obergrenzen: GPU mit Operanden und Akkumulator FP32 und FP16, Scoring FP32, FP16 und INT8-DP4A, Fusion on und off. CPU mit Operanden FP32 und FP16 mit Konvertierung, Scoring FP32, INT8 und INT8 über VNNI. Numerische Achsen werden zur Laufzeit ermittelt.
- Es gibt keine Profilnamen und keine fest einkodierte Profil-Enum. Achsen und Werte sind je Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle.

### Capability-Erkennung

- CPU: FP32 immer, FP16 mit Konvertierung bei Half-Precision-Konvertierung (F16C), INT8-Scoring bei AVX2 oder VNNI, Letzteres nur mit diesen Befehlen. Thread-Anzahlen als Zweierpotenzen bis zur Zahl logischer Cores plus die Core-Zahl selbst, mindestens ein Core. Features (AVX2, F16C, FMA, VNNI) werden vom System ermittelt, auf fremden Architekturen gelten sie als fehlend.
- GPU: FP32 immer, FP16 ab Compute Capability 5.3, INT8-DP4A ab 6.1, Fusion on und off. Tensor Cores werden anhand von Capability und Device-Name erkannt (Baureihen TU116 und TU117 haben trotz SM 7.x keine) und nur geloggt. Ohne GPU-Unterstützung oder Device-Context scheitert die Erkennung mit einem Capability-Error.
- Numerische Achsen aus der Konfiguration: Batch-Größen 1, 8, 32, Tile-Größen Zweierpotenzen von 16 bis 512, maximale Sequenzlänge 512. Nur positive Batch-Größen und Tiles bis zur Sequenzlänge. Die Kernel-Menge ist vorgebbar, sonst gilt der eingebaute Katalog.
- Erkannte Capabilities werden auf vorhandene Kernel reduziert. Unbekannte Achsen bleiben, geleerte Katalogachsen entfallen, Entferntes wird geloggt. Das Ergebnis wird mit Subengine, Achsen- und Wertezahl geloggt. Die Auswahl des Verfahrens erfolgt zentral nach Device-Art.

### Benchmark-Workload und Abweichungsmaße

- Reproduzierbare synthetische Workload aus Seed, normiert und duplikatfrei: Default 20 000 Dokumente, 384 Dimensionen, höchstens 32 Queries, die Top-5 für die Rangprüfung. Referenz ist das FP32-Scoring auf der CPU.
- Query-Zahl aus der Batch-Größe, ohne Angabe acht, begrenzt auf die Höchstzahl. Nicht numerische, null oder zu große Werte machen die Kombination unbrauchbar. Kleinere Batches nutzen die ersten Queries.
- Cosine-Deviation ist eins minus Cosine Similarity, mindestens null, bei Nullvektor eins. Werte unter einem Millionstel gelten als Rauschen.
- Relativer Fehler ist Abstand durch Referenzlänge, bei Nullreferenz null oder unendlich.
- Top-k-Overlap ist der Anteil der besten Referenztreffer im Kandidaten, Gleichstände nach Position, bei null gleich eins.
- Gesamtbewertung zeilenweise mit der schlechtesten Zeile, je Zeile das Größere aus Cosine-Deviation und eins minus Overlap.
- Validity-Zustände: valid, non-finite, degenerate, Abweichung über Maximum, Ausführung gescheitert, jeweils mit Text. Ungleiche Längen oder unteilbare Daten sind degenerate. Eine Zeile ist degenerate, wenn sie konstant ist (Varianz unter 10⁻⁹ des mittleren Quadrats) oder ihr Längenverhältnis außerhalb von 0,5 bis 2,0 liegt, konfigurierbar.

### Mess-Interface und Messmittel

- Austauschbares Mess-Interface (Trait): Abweichungsmessung gegen die Referenz (sonst „unverträglich“) und Zeit-Probe mit normierter Zeit je Arbeitseinheit oder „unterbrochen“, wenn eine produktive Aufgabe das Device anfordert.
- Warm-up beim ersten Gebrauch, Wiederholungen auf das Zeitfenster skaliert (Default zehn Millisekunden), Einzel-Launches im Chunk-Budget (Default fünf Millisekunden).
- Eine Kombination ist ein Wert je Achse plus die Sequenzlängen des Test-Sets, die nur Verarbeitbarkeit prüfen. Textform aus Achse-Wert-Paaren.
- Messergebnis: robuste Zeit, Abweichung, Validity, Stabilität, Probenzahl, relative Unsicherheit.
- Austauschbar sind die Abfrage wartender produktiver Aufgaben (Default nie, auch über einen gemeinsamen Schalter) und die monotone Zeitquelle (Clock).
- CPU (4 000 Dokumente): nur FP32, positive Thread-Anzahl (Default eins) mit eigenem Pool, parallele Berechnung, einmaliges Warm-up je Kombination, Zeit je Durchlauf und Query, Unterbrechungsprüfung zwischen Durchläufen.
- GPU (Default-Maße): Cosine-Pfad mit FP32 oder FP16 aus Scoring- oder Operanden-Achse, Dokumente in beiden Precisions auf dem Device, Zeit per Events auf dem Stream, 64 MiB Cache-Flush vor jeder Probe, mindestens 100 Millisekunden Warm-up. Wiederholungen eins bis 10 000, Gruppen im Chunk-Budget, je Kombination gemerkt, Unterbrechungsprüfung zwischen Gruppen. Ohne GPU-Unterstützung Capability-Error.

### Kalibrierungslauf

- Ziel sind Subengine und Model-ID. Der Kalibrierer zählt Kombinationen auf, misst interleaved gegen die Referenz, verwirft ungültige und bildet die Pareto-Front über Zeit und Abweichung. Obergrenzen beenden eine Messung mit dem besten, als instabil markierten Zwischenstand.
- Konfiguration (Default in Klammern): maximale Abweichung (0,05), Mindestwiederholungen (3, kleinere werden angehoben), Zeitfenster (zehn Millisekunden), Stabilitätsschwelle (0,05), Streuungsmaß Variationskoeffizient (CV) oder Spannweite durch Median (CV), Schätzer Median oder Minimum (Median), harte Obergrenze (40 Versuche, optional 30 Sekunden), Chunk-Budget (fünf Millisekunden), Gesamtbudget (optional 600 Sekunden), Dominanz-Marge (zehn Prozent), Backoff-Pause (fünf Millisekunden), separierbare Achsen (Batch-Größe, Tile-Größe, Thread-Anzahl), Vollmessung des kartesischen Produkts als Referenz (aus), Sequenzlängen ab 16 in Viererschritten bis zur Obergrenze einschließlich (Default 16, 64, 256, 512).
- Ablauf: leeres Capability-Set ergibt Capability-Error, leere Achsen werden ignoriert, Werte numerisch aufsteigend, dann alphabetisch. Achsen in Kern- und separierbare Achsen geteilt. Im Normalmodus zuerst die Achsenstufe, dann der Verifikationsschritt mit den Kandidaten, im Vollmodus das ganze Produkt. Die Front stammt nur aus dem Verifikationsschritt. Report mit Produktgröße, Budgetstatus und Laufzeit, Zusammenfassung und Log-Abschnitt.
- Achsenstufe (Pruning): Je Kernkombination wird jede separierbare Achse einzeln variiert, die anderen auf ihrem kleinsten Wert. Einwertige Achsen gewinnen automatisch. Gewinner sind die Pareto-optimalen Werte; ohne Gewinner wird die Kernkombination übersprungen. Kandidaten sind das deduplizierte Produkt. Das Budget wird vor jeder Kernkombination geprüft.
- Messen einer Gruppe: zuerst die billige Abweichungsphase mit Verwerfung (gescheitert, non-finite, über Maximum, sonstiger Grund), andere Fehler brechen ab. Danach rundenweise Zeitmessung Round-Robin über alle aktiven Kombinationen, in ungeraden Runden in umgekehrter Reihenfolge, damit Lastschwankungen alle gleich treffen. Die erste Kombination ist Anchor, Stabilität wird am Zeitverhältnis zum Anchor derselben Runde beurteilt (nur bei positiver Anchor-Probe). Jeder Versuch zählt, Unterbrechungen lösen die Backoff-Pause aus.
- Beendigung: Die Streuung ignoriert das langsamste Fünftel, ist bei weniger als zwei Proben unendlich. Eine Nicht-Anchor-Kombination endet vorrangig als „dominated“ (bei genug Proben ist eine andere bei höchstens gleicher Abweichung um mehr als die Marge schneller), sonst „stable“ bei stabiler Verhältnisreihe, sonst „unstable“ an der Obergrenze. Der Anchor endet zuletzt, stable bei stabilem Mitbewerber ohne instabilen oder eigener stabiler Reihe.
- Ergebnis je Kombination: dominated, verworfen mit Grund, übersprungen (Budget oder Device nie verfügbar) oder gemessen mit Zeit, Abweichung, Stabilität, Proben und Unsicherheit (ersatzweise Dominanz-Marge). Bei Budgetende aktive Kombinationen mit Proben gelten als gemessen und unstable.
- Report: Ziel, Einträge mit Stufe, Kombination und Ergebnis, Front aufsteigend nach Zeit mit streng fallender Abweichung, Zähler und Log jeder Einordnung.

### Pareto-Front, Positionen und Profile

- Die Front minimiert Zeit und Abweichung ohne nicht endliche Punkte, aufsteigend nach Zeit. Ein Punkt bleibt nur bei streng kleinerer Abweichung als alle schnelleren. Berechenbar für Zahlenpaare und beliebige Elemente (generisch).
- Positionen aus kumulierter Bogenlänge auf min-max-normierten Achsen, Zeit optional logarithmisch. Leere Front ohne Positionen, ein Punkt bei 0,5, Gesamtlänge null ergibt gleichmäßige Verteilung.
- Ein Profil ist unbenannt, mit Position zwischen null und eins und einem Wert je Achse. Positionen außerhalb sind „ungültige Precision“.
- Einzige Stellgröße ist der Merge-Threshold auf der normierten Skala (Default 0,1), modell- und hardwareunabhängig. Ableitung: nicht endliche Punkte entfernen; Punkte entfernen, die weniger präzise sind, ohne signifikant schneller zu sein (geloggt), etwa wegen gleicher Emulationspfade oder langsamer emulierter Precision; Positionen logarithmisch berechnen; Nachbarn unter dem Threshold gruppieren; Repräsentant ist der präziseste Frontpunkt in der Gruppe, sonst der schnellste; Positionen neu berechnen; Konfigurationen übernehmen.
- Die Profilzahl folgt allein aus der Streuung. Ein leeres Ergebnis bedeutet kein brauchbares Profil.

### Profilauswahl

- Der Wunsch wird auf null bis eins begrenzt (Clamp), „keine Zahl“ (NaN) ist „ungültige Precision“. Gewählt wird das Profil mit kleinstem Abstand, bei Gleichstand das präzisere.
- Die Skala gilt relativ zur Subengine: 0,5 bedeutet auf beiden Prozessoren unterschiedliche Genauigkeit.
- Solange ein aktives Profil existiert, ist der Wunsch erfüllbar. Ohne aktives Profil scheitert der erste Task mit „kein aktives Profil“ und sperrt das Modell für diese Subengine (Latch). Spätere Tasks scheitern sofort. Ungültige Wünsche sperren nicht. Die Sperre ist aufhebbar mit Rückmeldung. Alles thread-safe, ein Poisoned Lock führt nicht zum Ausfall.

### Persistenz der Kalibrierung

- Ergebnisse je Modell und Subengine mit Fingerprint, Profilen (Position und Konfiguration als Text), Instabilitäts-Flag, verworfenen und geprunten Kombinationen mit Grund.
- Fingerprint: Schema-Version, Subengine, GPU-Name, Compute Capability und Treiberversion, CPU-Features, Model-ID, Engine-Version. Bewusst knapp, ein Löschen der Datei erzwingt Neukalibrierung. Ermittlung zur Laufzeit, GPU-Daten über die Treiber-API. Fehlende Felder werden beim Einlesen mit Defaults belegt.
- Zustände: fehlend (präziseste gültige Kombination mit Warnung oder expliziter Kalibrier-Befehl), kalibriert (laden und verwenden), veraltet (altes Ergebnis nutzen, neues entsteht im Hintergrund). Das Auflösen beim Start scheitert nie, unlesbare Dateien gelten als fehlend.
- Ablage mit Load und atomarem Save: lesbare strukturierte Textdatei je Modell und Subengine, fehlende Verzeichnisse werden angelegt, Schreiben über prozessspezifische Temp-Datei mit fsync und Rename.
- Neukalibrierung läuft im Hintergrund auf dem Low-Priority-Pool, ersetzt das Ergebnis atomar und loggt den Austausch.

### Hintergrundausführung und Statistik

- Low-Priority-Pools mit Namen und konfigurierbarer Thread-Zahl (mindestens eins). Auf Linux wird die niedrigste Scheduling-Priorität gesetzt (nice), sonst geloggt.
- Jobs liefern einen Result-Container (Future), blockierend oder nicht blockierend abfragbar. Panics beenden den Worker nicht. Beim Shutdown laufen eingereichte Jobs zu Ende (Drain).
- Schätzer Median und Minimum (Last verlangsamt nur). Streuung als Variationskoeffizient oder Spannweite durch Median, ohne das langsamste Fünftel, unendlich bei zu wenigen Proben oder nicht positivem Mittelpunkt.

## Addons

- Ein Task kann den Scope eines Folge-Tasks treiben. Iteration und Rekursion entstehen allein durch ein Addon in einer Subengine, der Anwendung oder einer Ressourcenimplementierung (Index, Modell).
- Ein Addon enqueued Tasks über den normalen Weg, mit geklonten oder geänderten Parametern. Gezogen werden sie über die vorhandenen Pull-Filter (Priorität, Affinity).
- Addon-Tasks sind von Anwendungs-Tasks nicht unterscheidbar. Der Kern braucht dafür nur das erweiterbare Parameterobjekt.
- Beispiele: ein Centroid-Task gegen die Summary-Struktur mit einem Folge-Task über die Mitglieder der gewählten Zellen; eine Textanfrage als Embedding-Task gefolgt von einem Similarity-Task; ein Reranking als zweite Similarity auf dem verengten Scope.
