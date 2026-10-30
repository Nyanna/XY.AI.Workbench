Result Stats: id=47333c1a-2c2a-4bbc-842f-c7c492d6406a, total: 34779, in: 21568, out: 13211, reason: 7297, read: 0, write: 0, start: 1793397901884, end: 1793397975978, time: 74005, total: 74005, io: 18, control: 89, sent: 1, recv: 909
SystemInit:  id=47333c1a-2c2a-4bbc-842f-c7c492d6406a, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 7297
```yaml
- Name: Resident
  Requirements:
    - Resident ist eine dauerhaft laufende Engine für zwei Aufgabenarten, Embedding (Texte in Vektoren umwandeln) und Similarity (Ähnlichkeitssuche gegen einen Index).
    - Eine RAG-Anwendung startet Resident. Resident ist als Server-Prozess, als Daemon oder über die CLI nutzbar.
    - Resident verbürgt weder Vollständigkeit noch Größe der übergebenen Kandidatenmenge noch Ergebnisqualität.
    - Genauigkeitsverluste durch Chunking, Query-Formulierung, Index-Struktur, Reranking oder Kandidatenanzahl sind Sache der äußeren Schichten.
    - Ergebnisse von Embedding und Similarity sind keine Garantien. Varianz und Indeterminismus sind zugunsten von Auslastung und Performance erlaubt.
    - Stale Ergebnisse sind kein Fehler. Zugunsten konsistenten Durchsatzes gibt es keine Locks auf den Datenpfaden.
- Name: Engine-Fassade
  Requirements:
    - Die Engine-Fassade ist die Fassade (Facade) nach außen.
    - Die Engine-Fassade wird instanziiert und konfiguriert. Die Modelle bilden einen festen Satz und werden vor der Initialisierung der Engine über den Builder konfiguriert.
    - Die Engine-Fassade erlaubt es, Indizes zu registrieren, ohne sie zu laden oder zu verwenden.
    - Nach der Initialisierung ist die Engine-Fassade bereit, verbraucht aber noch keine Modell- oder Device-Ressourcen.
    - Tasks für verschiedene Aufgaben werden über die Engine-Fassade enqueued.
- Name: Task-Modell
  Requirements:
    - Das Task-Modell kennt keine Iteration, Kaskade, Rekursion, Parent-Child-Beziehung und keine Zwischenzustände zwischen Tasks.
    - Ein Task kann den Scope eines Folge-Tasks treiben, indem sein Ergebnis als Parameter des Folge-Tasks verwendet wird. Das Task-Modell kennt diese Beziehung selbst nicht.
- Name: Task-Handle
  Requirements:
    - Ergebnisse eines Tasks werden bereitgehalten und sind über das Task-Handle abholbar, per Zustandsabfrage, Event oder Notification.
- Name: Erweiterbare Zusatzparameter
  Requirements:
    - Addon-Tasks sind von Anwendungs-Tasks nicht unterscheidbar. Der Kern braucht dafür nur das erweiterbare Parameterobjekt.
- Name: Embedding-Task
  Requirements:
    - Der Embedding-Task beschreibt den Auftrag, Texte in Vektoren umzuwandeln.
    - Das Ergebnis eines Embedding-Tasks ist keine Garantie. Varianz und Indeterminismus sind erlaubt.
- Name: Similarity-Task
  Requirements:
    - Der Similarity-Task beschreibt eine Ähnlichkeitssuche gegen einen Index.
    - Der Similarity-Task rechnet auf der ihm übergebenen Kandidatenmenge und verbürgt weder Vollständigkeit noch Größe der Menge noch Ergebnisqualität.
    - Stale Treffer im Ergebnis eines Similarity-Tasks sind kein Fehler.
    - Ein Reranking wird als zweiter Similarity-Task auf dem verengten Scope ausgedrückt.
- Name: Similarity-Report
  Requirements:
    - Der Similarity-Report liefert Fakten. Er trifft keine Entscheidungen und legt keine Thresholds fest.
- Name: Addons
  Requirements:
    - Iteration, Kaskade und Rekursion entstehen allein durch ein Addon in einer Subengine, der Anwendung oder einer Ressourcenimplementierung (Index, Modell) und nicht im Kern.
    - Ein Addon enqueued Folge-Tasks über den normalen Weg, mit geklonten oder geänderten Parametern.
    - Ein Addon kann ein Centroid-Task gegen die Summary-Struktur mit einem Folge-Task über die Mitglieder der gewählten Zellen verbinden.
    - Ein Addon kann eine Textanfrage als Embedding-Task gefolgt von einem Similarity-Task ausdrücken.
    - Ein Addon kann ein Reranking als zweite Similarity auf dem verengten Scope enqueuen.
- Name: Processor
  Requirements:
    - Der Processor übernimmt die Orchestrierung mit Workern, Queues, Notification und Backpressure.
- Name: Planner
  Requirements:
    - Der Planner ordnet Operationen den Queues zu.
    - Der Planner weist die Operationen anhand der kompatiblen Capabilities den Queues zu, vorzugsweise der gemeinsamen Queue.
    - Der Planner stellt fest, welche Subengines konfiguriert sind, und startet sie.
    - Benötigt eine Operation bestimmte Ressourcen, teilt der Planner das den Subengines mit.
- Name: Expander
  Requirements:
    - Der Expander zerlegt Tasks in Operationsketten.
- Name: Operation
  Requirements:
    - Die Operation beschreibt die Operationsarten der Embedding- und Similarity-Ketten.
- Name: Resource-Requirements
  Requirements:
    - Resource-Requirements legen fest, welche bestimmten Ressourcen (Capabilities) eine Operation benötigt, sodass Subengines kompatible Ressourcen erkennen können.
- Name: Queue
  Requirements:
    - Die Queue ist ein gemeinsam genutzter Baustein für Task- und Operations-Queues.
    - Die Queue kennt kein Head-of-Line-Blocking, keine Reservierung und kein Vorab-Übernehmen.
- Name: Worker-Pool
  Requirements:
    - Nach der Initialisierung startet der Executor des Worker-Pools die Mindestzahl idle Threads.
    - Ein Thread des Worker-Pools bedient sich als Lane selbst an der Queue.
- Name: Subengine
  Requirements:
    - Gerechnet wird in zwei getrennten Subengines, einer für die CPU und einer für die GPU (CUDA).
    - Subengines sind für Ressourcen- und Speicherverwaltung je Hardwareart zuständig.
- Name: Gemeinsame Subengine-Regeln
  Requirements:
    - Wird einer Subengine mitgeteilt, dass eine Operation bestimmte Ressourcen benötigt, registriert sie ihre kompatiblen Ressourcen (Capabilities) in den ihr zugewiesenen und in den gemeinsamen Queues.
- Name: Ressourcen-Lebenszyklus
  Requirements:
    - Ressourcen werden erst bei Bedarf initialisiert (Lazy statt Eager).
    - Nach Ablauf einer Frist ohne Nutzung werden Ressourcen entladen (Unload), aber nicht deaktiviert.
- Name: Auslastungsmeldung
  Requirements:
    - Die Auslastungsmeldung liefert den dynamischen Durchsatz der Lanes, über den sich die Zuteilung steuert.
- Name: Precision, Kalibrierung und Profile
  Requirements:
    - Die Kalibrierung unterbricht oder blockiert den Nutzer nie.
- Name: Capability-Erkennung
  Requirements:
    - Die Capability-Erkennung meldet für die Referenzkarte ohne Tensor Cores keinen Tensor-Core-Pfad.
- Name: Kernel und Scoring
  Requirements:
    - Kernel sind nicht an Subengines, Modelle oder Indizes gekoppelt.
    - Varianz und Indeterminismus beim Scoring sind zugunsten von Auslastung und Performance erlaubt, etwa durch Fusion, Atomics oder hardwareabhängiges Rounding.
- Name: Unterstützte Precisions
  Requirements:
    - BF16 wird nicht als Compute-Pfad angeboten.
- Name: Pull-Ausführung
  Requirements:
    - Subengines holen ihre Arbeit selbstorganisiert aus Queues (Pull-Prinzip).
- Name: Subengine-Loop
  Requirements:
    - Sobald eine aktivierte Lane frei ist, zieht sie eine Operation. Die Zuteilung steuert sich über den dynamischen Durchsatz der Lanes: Wer schneller ist, holt sich mehr Arbeit.
    - Die Subengine-Loop zieht Tasks über die vorhandenen Pull-Filter (Priorität, Affinity). Addon-Tasks werden dabei genauso gezogen wie Anwendungs-Tasks.
    - Die Subengine-Loop kennt kein Head-of-Line-Blocking, keine Reservierung und kein Vorab-Übernehmen.
    - Nach dem Ziehen einer Operation initialisiert die Lane die benötigten Ressourcen lazy.
- Name: Lane und Batching
  Requirements:
    - Wenn sinnvoll, aggregiert die Lane weitere Operationen, die mit demselben Index oder Modell laufen können oder müssen, zu einem Batch.
    - Ergebnisse werden bereitgehalten und per Event oder Notification gemeldet.
- Name: Notification-Topologie
  Requirements:
    - Ergebnisse sind per Notification abholbar.
- Name: CPU-Subengine
  Requirements:
    - Die CPU-Subengine ist ein Device mit Ressourcen (Cores) und Lanes. Ihr Speicher umfasst Kernel, Index, Tokenizer und Normalizer.
    - Die CPU-Subengine ist nicht an Kernel, Modelle oder Indizes gekoppelt.
    - Alles, was Speicher oder Kernel auf der CPU besitzt, gehört der CPU-Subengine.
    - Die Kalibrierung der CPU-Subengine unterbricht oder blockiert den Nutzer nie.
- Name: GPU-Subengine
  Requirements:
    - Die GPU-Subengine ist ein Device mit Ressourcen (Compute Units, Streaming Multiprocessors) und Lanes. Ihr Speicher umfasst Kernel und Index (Segmenttabelle, Segment, Manager).
    - Die GPU-Subengine ist nicht an Kernel, Modelle oder Indizes gekoppelt.
    - Alles, was Speicher, Streams oder Kernel auf der GPU besitzt, gehört der GPU-Subengine.
    - Die Kalibrierung der GPU-Subengine unterbricht oder blockiert den Nutzer nie.
- Name: Kernel-Register
  Requirements:
    - GPU-Kernel werden zur Laufzeit (JIT) für die tatsächlich vorhandene Architektur kompiliert, auf der Referenzkarte also für SM 7.5.
    - Kernel werden lazy kompiliert und geladen.
- Name: Kernel-Synchronisation
  Requirements:
    - Der Manager blockiert nie einen Kernel.
- Name: Doorbell-Synchronisation
  Requirements:
    - Die Doorbell-Synchronisation nutzt Loads mit Acquire-Semantik, die ab Compute Capability 7.0 verfügbar sind. Turing genügt dafür.
- Name: Modell
  Requirements:
    - Das Modell ist ein gemeinsam genutzter Baustein von CPU- und GPU-Subengine.
    - Modelle sind nicht an Subengines, Kernel oder Indizes gekoppelt.
- Name: Modellbeschreibung
  Requirements:
    - Das Modell multilingual-e5-small liegt als Repository unter /home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small und dient als Default der Modellbeschreibung.
- Name: Modellregister
  Requirements:
    - Die Modelle bilden einen festen Satz und werden vor der Initialisierung der Engine konfiguriert.
    - Modelle werden erst bei Bedarf geladen.
- Name: Gewichtsquelle
  Requirements:
    - Die Modellgewichte liegen im Format safetensors vor und werden in diesem Format geladen.
- Name: Tokenizer-Laden
  Requirements:
    - Tokenizer werden in kompatiblen Formaten geladen bzw. zum Laden bereitgehalten.
    - Der Tokenizer wird lazy initialisiert.
- Name: Textnormalisierung
  Requirements:
    - Die Normalisierung bezeichnet die zum Tokenizer gehörende Normalisierung.
    - Zusätzlich kann es eine externe, vorgeschaltete Textnormalisierung für die Chunks geben, die lazy initialisiert wird.
- Name: Library-Adapter
  Requirements:
    - Externe Libraries, insbesondere die Tensor-/Neural-Network-Library Candle, sind jeweils an genau einer Stelle als Adapter angebunden.
    - Typen externer Libraries werden nur an diesen Rändern konvertiert.
- Name: Zielumgebung und Abhängigkeiten
  Requirements:
    - Die Zielhardware ist eine Nvidia GeForce GTX 1660 SUPER mit Compute Capability 7.5 (Turing) und Treiberversion 610.57.04.
    - Die CUDA-Toolchain ist Release 13.3, Version 13.3.73, Build 38244171_0.
    - GPU-Kernel werden zur Laufzeit (JIT) für die tatsächlich vorhandene Architektur kompiliert, auf der Referenzkarte also für SM 7.5.
    - Die Referenzkarte gehört zur Baureihe ohne Tensor Cores. Ein Tensor-Core-Pfad und BF16 als Compute-Pfad werden daher nicht angeboten.
    - Alle Dependencies sind geprüft und in der Manifest-Datei /home/user/xyan/xy.ai.workbench/rag/Cargo.toml eingetragen.
    - Die externe Tensor-/Neural-Network-Library ist Candle.
    - Gewichte liegen im Format safetensors vor, persistente Indizes im Spaltenformat Parquet mit Arrow-Schema.
- Name: Index
  Requirements:
    - Indizes sind nicht an Subengines, Kernel oder Modelle gekoppelt.
    - Indizes werden bei der Registrierung nicht geladen, sondern erst bei Bedarf.
- Name: Dateigestützter Index
  Requirements:
    - Persistente Indizes liegen im Spaltenformat Parquet mit Arrow-Schema vor.
- Name: Dateiformat und Modellbindung
  Requirements:
    - Die einzige harte Bindung zwischen Index und Modell ist die Kombination aus Model-ID und Dimension im Index-Header, ergänzt um die Pipeline-Merkmale, die Vektoren vergleichbar machen.
- Name: Konsistenzmodell
  Requirements:
    - Stale Ergebnisse sind kein Fehler. Zugunsten konsistenten Durchsatzes gibt es keine Locks auf den Datenpfaden, die Manager-Sicht wird per Copy-on-Write und Root-Pointer-Swap publiziert.
- Name: Scope-Auflösung
  Requirements:
    - Verlorene Randtreffer approximativer Prefilter wie Centroid, Fourier-Transformation oder Kompression verantwortet die Scope-Auflösung nicht. Sie sind Sache der äußeren Schichten.
- Name: Residency-Steuerung
  Requirements:
    - Indizes und Modelle werden bei Bedarf geladen, wenn eine gezogene Operation sie benötigt.
- Name: Eviction
  Requirements:
    - Nach Ablauf einer Frist ohne Nutzung werden Ressourcen entladen (Unload), aber nicht deaktiviert.
    - Die Unload-Fristen für Modelle und Indizes im VRAM sind deutlich länger als im RAM.
```
