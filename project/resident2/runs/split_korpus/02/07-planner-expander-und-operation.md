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

