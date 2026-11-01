Aufgabe: Leite aus dem Material einen Modulbaum ab, der alle darin enthaltenen Anforderungen aufnehmen kann.

- Ein Modul ist ein Verantwortungsbereich
- Name: Nominalphrase der Fachsprache, eindeutig, in der Sprache des Materials.
- Responsibility: nur aus dem Material ableiten.
- Jede Anforderung muss sich mindestens einem Modul zuordnen lassen. Lege Module an, bis das gilt, und fasse zusammen, was vereinbare Verantwortung hat.
- Hierarchie: ein Baum mit einer Wurzel, jedes Modul hat genau einen Elternknoten. Untergliedere nur, wenn ein Modul mehrere unabhängige Verantwortungen hat.

Ausgabe: genau ein YAML-Block, kein Text davor oder danach, keine YAML-Kommentare.
Struktur: Die Ausgabe ist eine Liste mit genau einem Eintrag, dem Wurzelmodul. Jedes Modul hat genau die Felder Name, Responsibility, Children.
- Name: Zeichenkette.
- Responsibility: Liste von Verbphrasen (1-2), kompakt.
- Children: Liste von Modulen derselben Struktur (wenn vorhanden).

Form:
```yaml
- Name: Wurzel
  Responsibility:
    - ...
  Children:
    - Name: Kind A
      Responsibility:
        - ...
      Children: []
    - Name: Kind B
      Responsibility:
        - ...
      Children: []
```

Material:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/plan7.md)
Result Stats: id=35189a8b-a015-425d-a853-82187dbdeb6d, total: 125293, in: 65623, out: 59670, reason: 43548, read: 0, write: 0, start: 1793394650587, end: 1793394995871, time: 345039, total: 345039, io: 823, control: 245, sent: 1, recv: 2570
SystemInit:  id=35189a8b-a015-425d-a853-82187dbdeb6d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 43548
```yaml
- Name: Resident
  Responsibility:
    - Embedding und Similarity als dauerhaft laufende Engine für RAG-Anwendungen (Server, Daemon, CLI) bereitstellen
    - Selbstorganisation, Entkopplung, Nicht-Blockieren, schlanken Kern und Lazy-Ressourcen als Leitprinzipien durchsetzen
  Children:
    - Name: Engine-Fassade
      Responsibility:
        - Engine über Builder mit verpflichtendem Modell konfigurieren, starten und geordnet herunterfahren
        - Planner mit Subengines, Modellen und Indizes versorgen, Tasks entgegennehmen und Indizes ohne Laden unter eindeutiger ID registrieren
      Children: []
    - Name: Task-Verwaltung
      Responsibility:
        - Tasks als Arbeitseinheiten mit Parametern, Zustand und Ergebnis modellieren
      Children:
        - Name: Task-Modell
          Responsibility:
            - Daten, Parameter, Zustand, Operationen und Ergebnis eines Tasks kapseln
            - Ergebnis genau einmal als Output oder Fehler setzen und Wartende wecken
          Children: []
        - Name: Task-Handle
          Responsibility:
            - Zustand ohne Warten abfragen und unbegrenzt oder mit Timeout auf den Abschluss warten
            - unfertige, noch nicht übernommene Tasks beim Drop abbrechen
          Children: []
        - Name: Erweiterbare Zusatzparameter
          Responsibility:
            - typindizierte Extension-Map mit höchstens einem Wert je Typ führen
            - Werte beim Klonen teilen und thread-safe durch Queue und Planung reichen
          Children: []
        - Name: Embedding-Task
          Responsibility:
            - Texte mit Modell, Modus, Precision und optionalem Ziel-Index als Embedding-Auftrag beschreiben
            - Embeddings oder bei Index-Ziel Entry-Referenzen als Ergebnis liefern
          Children: []
        - Name: Similarity-Task
          Responsibility:
            - Query-Vektoren mit Index-ID, Top-k, Scope, Scoring-Verfahren und Precision als Suchauftrag beschreiben
            - Reranking als zweite Similarity auf verengtem Scope ausdrücken und Treffer mit Entry, Score und stale-Markierung liefern
          Children: []
        - Name: Ergebnisarten
          Responsibility:
            - Embeddings, Entry-Referenzen, Treffer, Similarity-Report und Score-Listen als Ergebnisarten unterscheiden
            - Ergebnisarten erweiterbar und auf Gleichheit vergleichbar halten
          Children: []
        - Name: Similarity-Report
          Responsibility:
            - gescorte Treffer sortieren, auf Top-k kürzen und mit Kandidaten-, Verwerfungs- und stale-Zählern sowie Source-Link berichten
            - Kennzahlen wie Gap, Spread, z-Score, Softmax-Verteilung, Entropie und Knee-Point berechnen
          Children: []
    - Name: Planner
      Responsibility:
        - Tasks entgegennehmen, zerlegen lassen und Operationen anhand kompatibler Capabilities in CPU-, GPU- oder gemeinsame Queue einordnen
        - benötigte Subengines blockierend aktivieren, Ressourcenangebote verwalten und die Ressourcenart der ersten Operation je Task-Art vorab melden
      Children:
        - Name: Expander
          Responsibility:
            - Task als reine Funktion genau einmal in eine geordnete Operationskette zerlegen und fremde Task-Arten ablehnen
            - Task-Parameter, Modellauswahl und Pipeline-Konfiguration nach Expansionsregeln für Embedding, Similarity und Cross-Encoder in Operationskonfiguration übersetzen
          Children: []
        - Name: Schrittweise Übergabe und Ergebnisaufbereitung
          Responsibility:
            - erste Operation mit Collector bereitstellen und Folgeoperationen aus dem tatsächlichen Output der vorigen erzeugen
            - Input je Task-Art aufbereiten und das Gesamtergebnis einmalig einsammeln
          Children: []
        - Name: Operation
          Responsibility:
            - Berechnungsschritte mit Art, Parametern, Input-/Output-Regionen und Datenformen beschreiben
            - Parameter und Descriptor als vergleichbare Keys für Caches, Filter und Dependency-Graphen bereitstellen
          Children:
            - Name: Resource-Requirements
              Responsibility:
                - Ressourcenart, Modell, Index, Execution-Umgebung, Capabilities und Placement-Hint einer Operation festlegen
                - Erfüllbarkeit durch ein Device prüfen
              Children: []
            - Name: Materialisierung
              Responsibility:
                - Materialization-Flag festlegen und Ergebnis ins RAM zurückholen oder als Referenz im VRAM halten
              Children: []
            - Name: Task-Kontext
              Responsibility:
                - Priorität, Zähler offener Operationen, Cancel-Flag und ersten Fehler je Task nebenläufig führen
                - Folgeoperationen vor dem Enqueue registrieren und Task-Abschluss oder Abbruch bestimmen
              Children: []
            - Name: Operationsknoten
              Responsibility:
                - Operation, Input, Output, Materialization-Flag und Task zu einem Knoten bündeln
                - Continuation-Handler Folgeknoten erzeugen lassen und Output einmalig setzen
              Children: []
    - Name: Processor
      Responsibility:
        - Task-Queue, Operations-Queues, Worker-Pool, Notification und Idle-Notifications besitzen
        - Planner für die Expansion aufrufen
      Children:
        - Name: Worker-Pool
          Responsibility:
            - Mindestzahl idle Worker starten, auf CPU-Cores minus eins begrenzen und per Notification aus dem Parkplatz wecken
            - CPU-Operationen synchron und GPU-Operationen nur startend ausführen und Blockieren als natürliches Backpressure belassen
          Children: []
        - Name: Worker-Loop
          Responsibility:
            - nach jeder Operation in fester Prioritätsfolge GPU-Ergebnisse, GPU-Ressourcen, CPU-Ressourcen und Task-Expansion prüfen
          Children: []
        - Name: GPU-Completion
          Responsibility:
            - im Stream-Callback nur Completion-Eintrag ablegen und Notification auslösen, ohne GPU-API aufzurufen
            - fertige Lanes per Event-Query und atomarem Swap übernehmen
          Children: []
        - Name: Backpressure und Expansionssteuerung
          Responsibility:
            - Idle-Notifications als Set je Subengine-Art erzeugen und bei Treffer oder belegter Ressource verfallen lassen
            - Tasks nur bei Idle-Notification oder leerer Operations-Queue expandieren und unterversorgte Ressourcen bevorzugen
          Children: []
    - Name: Queue
      Responsibility:
        - lock-free MPMC-Queue für Arbeitseinheiten mit Zustand, Ergebnis und Notification bereitstellen
        - als Task-Queue und Operations-Queues für CPU, GPU und gemeinsam unabhängig einsetzen
      Children:
        - Name: Queue-Struktur und Enqueue
          Responsibility:
            - Einträge in einfach verketteter Liste mit Sentinel per Compare-and-Swap in Altersreihenfolge anhängen
            - Notification-Channels bei jedem Enqueue signalisieren, ohne zu blockieren
          Children: []
        - Name: Queue-Iteration
          Responsibility:
            - alle oder nur aktive Einträge in Enqueue-Reihenfolge nicht blockierend und gültig bei Nebenläufigkeit liefern
          Children: []
        - Name: Queue-Zustandsmodell
          Responsibility:
            - Zustände enqueued, claimed, completed und cancelled mit atomaren Übergängen Claim, Completion und Cancel verwalten
            - Payload per Ownership-Transfer genau einmal übergeben und Ergebnis vor dem Zustand sichtbar machen
          Children: []
        - Name: Abschluss-Warten
          Responsibility:
            - asynchron auf completed oder cancelled warten, ohne Thread zu blockieren oder Notification zu verlieren
            - mehrere Wartende gleichzeitig wecken
          Children: []
        - Name: Queue-Speicherfreigabe
          Responsibility:
            - Knoten per Reference Counting freigeben, optional entlinken und lange Ketten iterativ abbauen
            - Memory-Safety ohne unsafe außerhalb des atomaren Pointer-Swaps wahren
          Children: []
    - Name: Subengine
      Responsibility:
        - Ressourcen- und Speicherverwaltung je Hardwareart bereitstellen
        - Arbeit selbstorganisiert nach dem Pull-Prinzip aus den Queues ziehen
      Children:
        - Name: Gemeinsame Subengine-Regeln
          Responsibility:
            - Capability-Set liefern und kompatible Ressourcen samt Lanes in eigenen und gemeinsamen Queues registrieren
            - Compute-Pfade hardwareoptimal wählen und Invarianten wahren (keine Tensoren oder Raw-Pointer in Signaturen, Lane statt Default-Stream, Kernel nur über das Register)
          Children: []
        - Name: Batch-Compute-Schnittstelle
          Responsibility:
            - austauschbare Float-Batch-zu-Float-Batch-Schnittstelle bereitstellen
            - CPU-Implementierung als Default und Fallback nutzen und GPU-Implementierung als einzige Anbindung der GPU-Library anlegen
          Children: []
        - Name: Ressourcen-Lebenszyklus
          Responsibility:
            - Ressourcen lazy initialisieren (uninitialized, loading, ready, backoff) und bei Nichtnutzung nach TTL evicten
            - Initialisierungs- oder Startfehler in Batch-Ergebnisse schreiben und Ressource in Backoff versetzen
          Children: []
        - Name: Pull-Ausführung
          Responsibility:
            - Loop mit Notification als einziger Wartebedingung und anschließender Queue-Iteration betreiben
          Children:
            - Name: Subengine-Loop
              Responsibility:
                - eigene vor gemeinsamer Queue in Altersreihenfolge iterieren, per Filter und Requirements Ressource bestimmen und Knoten claimen
                - Reservierung, Vorab-Claim und Head-of-Line-Blocking vermeiden und Lazy-Initialisierung nur durch die Iteration anstoßen
              Children: []
            - Name: Lane und Batching
              Responsibility:
                - Operationen über Lanes ziehen und passende Operationen zu einem Batch aggregieren
                - Packing, Length-Bucketing und Streams als Ressourcenlogik halten und Ergebnis per Notification melden
              Children: []
            - Name: Notification-Topologie
              Responsibility:
                - Queue-Append an die Notification jeder Subengine weiterleiten
                - Ergebnis, Initialisierung, Eviction-Timer und Iteration mit Fortschritt als eigene Notification melden
              Children: []
        - Name: Auslastungsmeldung
          Responsibility:
            - Art, Device, Kapazität und geglätteten Durchsatz in Entries pro Sekunde melden
            - aus mehreren Threads nutzbar sein, ohne selbst Arbeit zuzuteilen
          Children: []
        - Name: CPU-Subengine
          Responsibility:
            - Cores mit Lanes als Ressourcen führen, Modelle und Indizes ins RAM laden und Tokenizer, Normalizer, Kernel und Index bereitstellen
            - Scorer per SIMD und Encoder über das CPU-Backend mit FP32-Default ausführen und Kalibrierung auf Threads niedriger Priorität laufen lassen
          Children: []
        - Name: GPU-Subengine
          Responsibility:
            - CUDA-Devices mit Compute Units als Ressourcen samt Lanes, Streams, Staging-Memory und Batching führen
            - VRAM-Manager für Laden, Streamen und Austauschen von Modellen, Kernel und Indizes samt Brücke zwischen RAM und VRAM betreiben, FP16 als Default
          Children: []
    - Name: Gemeinsame Host-Ressourcen
      Responsibility:
        - CPU-seitige Ressourcen bereitstellen, die CPU- und GPU-Subengine gemeinsam nutzen
      Children:
        - Name: Modell
          Responsibility:
            - Modelle als ressourcenlose Pipeline-Beschreibung führen und über Register zu Encodern und Scorern auflösen
          Children:
            - Name: Modellbeschreibung
              Responsibility:
                - Model-ID, Architektur, Dimension, Pooling, Präfixe, Datentypen, Sequenzlänge und Multi-Vector-Konfiguration beschreiben
                - Beschreibung validieren und Default für multilingual-e5-small aus dem Repository-Verzeichnis ableiten
              Children: []
            - Name: Modellregister
              Responsibility:
                - festen Modellsatz vor der Initialisierung führen, Architektur-IDs auf Encoder-Factories und Scoring-Arten auf Scorer abbilden
                - Encoder für eine Device-Klasse laden und als verwaltbare Ressource mit Key und Initialisierungskosten bereitstellen
              Children: []
            - Name: Gewichtsquelle
              Responsibility:
                - safetensors-Gewichte per mmap zero-copy auf Rohbytes lesen
                - Gewichte pro Subengine in deren Arena kopieren
              Children: []
            - Name: Transformer-Hauptmodell
              Responsibility:
                - BERT-Konfiguration prüfen und Gewichte per mmap im gewählten Datentyp auf CPU oder GPU laden
                - Forward-Pass von Token-IDs und Mask zu Hidden-States als FP32-Zahlenliste ausführen
              Children: []
            - Name: Encoder-Aufbau
              Responsibility:
                - Hauptmodell, optionale Projection-Layer, Pooling, Normalisierung und Storage-Datentyp zu einem Encoder zusammensetzen
                - Multi-Vector-Modelle über Encoder-Backbone mit eigenem Projection-Head aufbauen
              Children: []
            - Name: Projection-Layer
              Responsibility:
                - Token-Vektoren linear von Input- auf Output-Dimension abbilden
                - Weight und Bias aus safetensors laden und auf Shape und Datentyp prüfen
              Children: []
            - Name: Pooling und Normalisierung
              Responsibility:
                - Mean-Pooling über unmaskierte Token oder kein Pooling ausführen
                - L2-Normalisierung mit Epsilon-Clamp einmal am Encoder-Ende anwenden und degenerierte Vektoren zählen
              Children: []
            - Name: Embedding-Ausgabe
              Responsibility:
                - Ergebnisvektoren normalisieren, in den Output-Datentyp konvertieren und lückenlos in die Output-Region schreiben
                - Schreibfehler als Memory-Error weiterreichen und auf der GPU per Write-Kernel direkt in den vergebenen Slot schreiben
              Children: []
            - Name: Encoder-Schnittstelle
              Responsibility:
                - Input- und Output-Regionen mit Layout prüfen und Encoding asynchron auf einer übergebenen Lane ausführen
                - Completion-Event mit Verarbeitungsfehler liefern und Ergebnis je Task über One-Shot-Channel verteilen
              Children: []
        - Name: Tokenizer
          Responsibility:
            - Texte modellspezifisch oder geteilt auf der CPU in Token-IDs als u32 umwandeln
          Children:
            - Name: Tokenizer-Laden
              Responsibility:
                - Tokenizer aus Konfigurationsdatei oder geladenem Tokenizer mit ID, Schema und Sequenzlänge erzeugen
                - Pad-Token bestimmen, automatisches Padding abschalten und Trunkierung einrichten
              Children: []
            - Name: Textnormalisierung
              Responsibility:
                - Normalizer des Tokenizers über Parameter konfigurieren und doppelte Normalisierung vermeiden
                - optionalen externen Text-Normalizer vor Präfix und Tokenisierung anwenden
              Children: []
            - Name: Encoding-Schemata
              Responsibility:
                - Query- und Passage-Modus mit Präfix-Schema abbilden
                - Multi-Vector-Schema mit Markern, Query-Padding durch Mask-Token und Dokument-Trunkierung umsetzen
              Children: []
            - Name: Text-Encoding
              Responsibility:
                - Textliste in einem Modus als Batch zu Token-IDs in Eingabereihenfolge kodieren
                - Encoding mit Batch-Bildung kombinieren und Encoding-Fehler samt Logs weiterreichen
              Children: []
        - Name: Batch-Bildung
          Responsibility:
            - Tokenfolgen per Length-Bucketing und Token-Budget zu gepaddeten Batches mit Attention-Mask gruppieren
            - Batches mit ursprünglichen Positionen als u32 in Little-Endian serialisieren und Batch-Größe aus Token-Budget, Bucket und Deadline ableiten
          Children: []
    - Name: Device, Lane und Event
      Responsibility:
        - Device-Arten, Device-Handles und Execution-Contexts (Lanes) mit Stream, Priorität, Events und Workspace abbilden
        - Events als nicht blockierend abfragbare und blockierend abwartbare Completion-Marker bereitstellen
      Children: []
    - Name: Speicherverwaltung
      Responsibility:
        - Speicher in Arenen nach Lebensdauer über pointerfreie Regionen verwalten
        - Allokation im laufenden Betrieb vermeiden und Stream-ordered Reuse sicherstellen
      Children:
        - Name: Arenen
          Responsibility:
            - sechs Arenen mit stabiler ID und konfigurierbarer Kapazität führen und lazy nach Budget anlegen
            - Device-Bedarf und Generationsreserve berechnen
          Children: []
        - Name: Speicherbudget
          Responsibility:
            - freien und gesamten Speicher abfragen und nutzbaren Anteil nach Headroom und Generationsreserve ermitteln
            - Fit-Check durchführen und Copy-on-Write-Doppelbedarf einplanen
          Children: []
        - Name: Region-Handle
          Responsibility:
            - pointerfreie Regionen aus Arena-ID, Offset, Länge und Generation als Key bereitstellen
            - Handles nach Eviction und Neuladen über die Generation entwerten und Auflösung auf Device-Adressen dem Memory-Backend überlassen
          Children: []
        - Name: Layout und Elementtypen
          Responsibility:
            - eigene Elementtypen und Layouts aus Shape, Strides und Typ führen sowie Views ohne Datenänderung bilden
            - Elementtypen aus safetensors übernehmen, Device-Datentypen und additiven Mask-Wert festlegen
          Children: []
        - Name: Free-Space-Verwaltung
          Responsibility:
            - Bereiche per Offset-Allocator nach First-Fit aus fester Kapazität vergeben
            - Lücken beim Freigeben verschmelzen und Alignment sowie Overflows sicher behandeln
          Children: []
        - Name: Bookkeeping und Gültigkeit
          Responsibility:
            - Offsets und Generationen je Arena thread-safe verfolgen und Liveness von Regionen prüfen
            - stale Handles ignorieren, Fault Injection ermöglichen und Allokationen loggen
          Children: []
        - Name: Allocator mit Deferred Free
          Responsibility:
            - Allokation je Arena mit Fehlerunterscheidung bereitstellen und Regionen erst nach Event-Abschluss freigeben
            - Allocator-Set mit genau einem Allocator je Arena führen und zuständigen Allocator zu einer Region finden
          Children: []
        - Name: CPU-Backend
          Responsibility:
            - Arenen als genullte RAM-Bereiche führen und Schreiben, Lesen und Kopieren auf Regionsbasis prüfen
            - Budget aus Summen der Arenen liefern
          Children: []
        - Name: GPU-Backend
          Responsibility:
            - VRAM-Arenen als zusammenhängende genullte Blöcke mit Fit-Check anlegen und Mapped Host-Memory bereitstellen
            - D2D-, H2D- und D2H-Transfers mit Pinned Buffern und Events ausführen
          Children: []
        - Name: Staging-Ring
          Responsibility:
            - feste Zahl Pinned Staging-Slots mit den Zuständen frei, belegt und in Übertragung vergeben
            - Slots erst nach Event wiederverwenden und thread-safe verwalten
          Children: []
        - Name: Speicherzugriff
          Responsibility:
            - einheitliche nebenläufige Schnittstelle zum Lesen und Schreiben von Regionsbytes bereitstellen
            - Tensor-Referenz aus Region und Layout als einzigen Weg für Tensoren durch die Encoder-Schnittstelle führen
          Children: []
        - Name: Region-Pinning
          Responsibility:
            - Zugriff auf Device-Regionen als Pin mit Pin-Count führen und Eviction gepinnter Regionen verhindern
            - Pin per RAII freigeben und optional über Rückmeldekanal melden
          Children: []
    - Name: Library-Adapter
      Responsibility:
        - externe Libraries je an genau einer Stelle anbinden, Typen nur an den Rändern konvertieren und Planung davon frei halten
        - Device-Regionen in Library-Tensoren einhüllen und Elementtypen in beide Richtungen konvertieren
      Children: []
    - Name: Index
      Responsibility:
        - Entries, Vektordaten und Traversal-Struktur als Source of Truth in RAM und persistent halten
        - Änderungen melden, ohne Subengine, Kernel oder Geometrie zu kennen
      Children:
        - Name: Allgemeiner Index-Vertrag
          Responsibility:
            - Entries aus ID, Stamp und Payload nebenläufig lesen, schreiben (Last-Writer-Wins) und löschen
            - Change-Notification als Level-Signal und deklarierte Zugriffsmethoden samt Strukturen bereitstellen
          Children: []
        - Name: Change-Stamp
          Responsibility:
            - eindeutigen, opaken Stamp je Schreib- und Löschoperation vergeben
            - Max-Stamp führen und beim Wiederaufnehmen fortsetzen
          Children: []
        - Name: Scopes und Entry-Mengen
          Responsibility:
            - Scope-Arten all, range, set, subtree, prefix und centroid mit stabilem Namen führen
            - Entry-Mengen kompakt als Ranges oder Bitmap mit Mengenoperationen darstellen
          Children: []
        - Name: Referenz-Index
          Responsibility:
            - ohne GPU-Beschleunigung und Machine-Learning-Libraries lauffähigen Index im RAM bereitstellen
          Children: []
        - Name: Dateigestützter Index
          Responsibility:
            - Index auf Parquet-Datei per mmap oder File-Read öffnen, verwalten und atomar sichern
          Children:
            - Name: Index-Öffnung und Lazy Loading
              Responsibility:
                - Datei, Header, Schema, IDs und Stamps ohne Vektor-Dekodierung einlesen und optional verifizieren
                - Vektoren und Nutzdaten beim ersten Gebrauch in Batches lazy dekodieren
              Children: []
            - Name: Entry-Verwaltung im Dateiindex
              Responsibility:
                - Upsert und Remove mit Checksum, Stamp, Dirty-Markierung und Signal ausführen
                - Nutzdaten, Pfad, Header, Max-Stamp und Dirty-Status abfragen sowie Verifikation anbieten
              Children: []
            - Name: Persistentes Sichern
              Responsibility:
                - konsistenten Snapshot atomar per Temp-Datei, fsync und Rename als Parquet schreiben
                - gleichzeitige Saves serialisieren
              Children: []
        - Name: Dateiformat und Modellbindung
          Responsibility:
            - Spalten-Schema und Index-Header samt Tokenizer-Fingerprint festlegen
            - Bindung an das Modell über Model-ID, Dimension und Pipeline-Merkmale prüfen
          Children: []
        - Name: Nutzdaten und Checksum
          Responsibility:
            - Verweis auf Quelltext aus Zieldatei und Zeichenbereich je Entry führen
            - 64-Bit-Checksum berechnen und verifizieren
          Children: []
        - Name: Erkennung von Dateiänderungen
          Responsibility:
            - Änderungen an Quelldateien über Fallback-Kette aus Change-Stamp, Größe und Timestamp, Content-Hash erkennen
            - racy Aufzeichnungen erkennen und Content-Prüfung anfordern
          Children: []
    - Name: Manager
      Responsibility:
        - Zuordnung von Entry zu Segment und Slot, Residency und Update-Queue verwalten
        - Upload, Eviction, Updates und Rückabbildung von Adressen auf Entries verantworten
      Children:
        - Name: Manager-Lebenszyklus
          Responsibility:
            - Index erst bei Bedarf anhängen und Manager mit Once-Semantik aktivieren
            - Aktivität und residente Entries abfragbar machen und Zyklen in Diagnose-Spans laufen lassen
          Children: []
        - Name: CPU-Manager
          Responsibility:
            - Residency dateigestützter Daten im Page-Cache sichern und nach Leerlauf als cold markieren
            - einzelne und mehrere Entries mit Zugriffsmuster im Blocking-Pool lesen
          Children:
            - Name: Page-Cache-Hinweise und blockierende Lesezugriffe
              Responsibility:
                - Advice will-need, sequential, random und cold je Byte-Bereich unverbindlich setzen und Bereiche prüfen
                - blockierende Lesezugriffe im Blocking-Pool ausführen und Abbruch als Fehler melden
              Children: []
        - Name: GPU-Manager
          Responsibility:
            - Entry-Mengen per Ensure laden, Reconcile-Iterationen ausführen und Segmente publizieren oder freigeben
            - Segmentzahl, residente Bytes und ausstehende Updates melden
          Children: []
        - Name: Scope-Auflösung
          Responsibility:
            - Scope einer Operation über Resolver in eine kompakte Entry-Menge auflösen und nicht unterstützte Scopes ablehnen
            - Standard-Resolver für Range, Subtree, Prefix und Centroid bereitstellen und aus Vorverarbeitung bekannte Kandidaten gezielt laden
          Children: []
        - Name: Index-Konsistenz
          Responsibility:
            - Manager-Sicht mit dem Index abgleichen, wobei der Index gewinnt
          Children:
            - Name: Konsistenzmodell
              Responsibility:
                - Entry-Identität als Paar aus ID und Stamp festlegen und Index bei Abweichung gewinnen lassen
                - Segmenttabellen per Copy-on-Write und Root-Pointer-Swap publizieren und Änderungen serialisiert anwenden
              Children: []
            - Name: Zustandsbasierter Abgleich
              Responsibility:
                - je Subscriber residente Entries per Reconciler gegen den Index vergleichen und Signal vor dem Vergleich quittieren
                - Entries stamp-konsistent laden und Echos eigener Writes unterdrücken
              Children: []
            - Name: Residency-Verzeichnis
              Responsibility:
                - Slot und bekannten Stamp je residentem Entry führen
              Children: []
            - Name: Konsistente Snapshots
              Responsibility:
                - gleichzeitig veränderbare Daten nach dem Seqlock-Prinzip wortweise atomar und nie halb geschrieben lesen
                - Retries zählen und Writer gegenseitig ausschließen
              Children: []
        - Name: Segmentverwaltung
          Responsibility:
            - Segmente, Slots, Layouts und Manifeste für Kernel verwalten
          Children:
            - Name: Kernel-Schnittstelle und Layouts
              Responsibility:
                - Kernel nur Adresse und Score liefern lassen und feste oder variable Layouts (Stride, Offset-Tabelle) abbilden
                - Adressen auf Entries zurückabbilden und gebaute Segmente per Prefix-Sum adressieren
              Children: []
            - Name: Segmente und Slots
              Responsibility:
                - Segmente aus Slots fester Klasse als Region mit Zuordnung von Slot zu Entry führen
              Children:
                - Name: Slot-Layout und Segment-View
                  Responsibility:
                    - Slotgrößen und Klassen festlegen, Segment-View mit Descriptor bilden und Slot-Inhalte kodieren
                    - Gültigkeit je Slot (Valid-Bit, Tokenzahl) und Zuordnung von Slot zu Entry nebenläufig führen
                  Children: []
                - Name: Segment-Schreibpfad
                  Responsibility:
                    - Slots reservieren und per Commit sofort oder deferred bis zum Event sichtbar machen
                    - Insert und Update als neuen Slot führen, Entries per Tombstoning ausblenden und Speicher per Rebuild zurückgewinnen
                  Children: []
                - Name: Treffer-Auflösung
                  Responsibility:
                    - Kernel-Treffer in Segment-Generation, Slot und Score umrechnen und über das Manifest zu Entry-Referenzen auflösen
                    - Treffer älterer Manifest-Generationen überführen oder verwerfen
                  Children: []
            - Name: Manifeste und Publication
              Responsibility:
                - Manifeste als Snapshots der Segmente mit Generation, Kernel-Segmenttabelle, lock-free Reader-Zugriff und Leases führen
                - Manifeste hinter Kopier-Events vormerken und publizieren, nicht mehr gebrauchte Segmente zurückgeben und Change-Notifications senden
              Children: []
            - Name: Segment-Arena
              Responsibility:
                - Segmente ausgerichtet mit Reserve und Replacement-Headroom zuteilen und freigeben
                - Replacement durch Kopie, schrittweise beschreibbare Segmente und Budgets verwalten
              Children: []
        - Name: Kernel-Synchronisation
          Responsibility:
            - Änderungen nur in Gaps und ohne den Kernel zu blockieren durchführen
          Children:
            - Name: Mutationsdisziplin
              Responsibility:
                - nur in Gaps verändern, nie unter dem Kernel, und atomar ausschließlich 8-Byte-Pointer tauschen
                - größere Strukturen per Copy-on-Write ersetzen und Torn Reads bewusst akzeptieren
              Children: []
            - Name: Table-Swap
              Responsibility:
                - Root-Pointer mit Release/Acquire, immutable Segmenttabellen und Epoch-Tracking führen
                - Tabellen sichtbar schreiben, publizieren, Vorversionen zurückstellen und bis zur kleinsten Reader-Epoche freigeben
              Children: []
            - Name: Doorbell-Synchronisation
              Responsibility:
                - abgeschlossene Batches mit Release/Acquire publizieren und Gap zwischen completed und acked erkennen
                - Gap schließen und Ack publizieren, ohne auf den Kernel zu warten
              Children: []
            - Name: Communication-Slot
              Responsibility:
                - in fester Reihenfolge Kernel-Writes zurücklesen, Ergebnisse auflösen und Updates einspielen
                - bei weitergegangener Zuordnung nebenwirkungsfrei melden und den Slot wiederholen
              Children:
                - Name: Address-Map und Mapping-Store
                  Responsibility:
                    - Arena-Adressen per Copy-on-Write-Map mit Epoche auf Entries abbilden
                    - Maps versioniert halten und nur Nachfolger-Epochen publizieren
                  Children: []
                - Name: Update-Queue
                  Responsibility:
                    - Bind- und Unbind-Updates mit Stamp führen, je Entry nur das jüngste
                    - Deletes vor Binds anwenden
                  Children: []
            - Name: Index-Aufbau durch Kernel
              Responsibility:
                - Kernel Embeddings in vorab vergebene Arena-Bereiche schreiben lassen, ohne dass der Kernel allokiert
              Children:
                - Name: Write-Pointer-Bereich
                  Responsibility:
                    - Claims per Write-Pointer mit Overflow-Grenze und Warp-Prefix-Sum vergeben
                    - bei Überlauf Teilergebnis melden, sodass der Manager den nächsten Bereich vergibt
                  Children: []
                - Name: Embedding-Batch-Ausführung
                  Responsibility:
                    - Batch per Resume bis zu Treffern ausführen und Bereiche je Schritt reservieren
                    - Batch immer beenden und im Fehlerfall alle Bereiche freigeben
                  Children: []
                - Name: Write- und Treffer-Queues
                  Responsibility:
                    - Embedding-Writes und Treffer über getrennte SPSC-Queues führen
                    - Writes vor Treffern entnehmen, sodass Indexieren während der Suche funktioniert
                  Children: []
        - Name: Residency-Steuerung
          Responsibility:
            - Index-Daten lazy und scope-getrieben auf das Device laden und wieder verdrängen
          Children:
            - Name: Ladestrategie
              Responsibility:
                - Geometrie, Policy und Zugriffsart in Lade-Pläne (ganze Segmente, Gather, Zero-Copy) übersetzen und Segmentgröße nach Budget bestimmen
                - Summary-Strukturen, Vektorsegmente und nicht geladene Bereiche gestaffelt (Tiering) halten
              Children: []
            - Name: Lade-Aggregation und Single-Flight
              Responsibility:
                - Tasks gegen denselben Index über die Union ihrer Scopes bündeln und je Index genau einmal laden
                - gleichzeitige Requests mit gleichem Key auf einen Ladevorgang vereinen
              Children: []
            - Name: Index-Operations-Koordinator
              Responsibility:
                - Manager, Table-Swap, Mapping-Store und Queue geladener Segmente verbinden und Segmente selbständig publizieren
                - strukturelle Änderungen serialisieren und bei erschöpfter Arena Platz schaffen und einmal wiederholen
              Children: []
            - Name: Device-Transfer
              Responsibility:
                - ganze Segmente direkt kopieren, sonst Entries über den Gather-Pfad mit Staging-Slots übertragen
                - sehr kleine Mengen per direktem Mapping zero-copy lesen
              Children: []
            - Name: Eviction
              Responsibility:
                - Mirrors in VRAM und RAM nach Idle, Pressure und Space-Need mit Watermarks und Victim-Auswahl nach GreedyDual-Size-Frequency evicten
                - evictete Segmente aus Tabelle und Zuordnung entfernen und Entscheidungen loggen
              Children: []
    - Name: Kernel und Scoring
      Responsibility:
        - Scores über Segmente und Slots berechnen, ohne Entry-Identität oder Traversierung zu kennen
        - Kernel über ein Register bei Bedarf bereitstellen
      Children:
        - Name: Kernel-Register
          Responsibility:
            - Kernel nach Name, Datentyp, Architekturstufe und Variante bei Bedarf kompilieren, laden und ohne Doppelkompilierung zwischenspeichern
            - Zielarchitektur aus Compute Capability bestimmen und Kompilier- und Ladefehler melden
          Children: []
        - Name: Unterstützte Precisions
          Responsibility:
            - Elementtypen mit Scoring-Kernel je Device-Art festlegen (CPU FP32, GPU FP32 und FP16)
            - Operanden- und Scoring-Achsen nur für unterstützte Typen melden
          Children: []
        - Name: Query-Batch und Dokumentquelle
          Responsibility:
            - Queries gleicher Dimension bündeln und prüfen
            - Dokumentvektoren über Source-Schnittstelle liefern, Slot-Bereiche prüfen und Workspaces als Launch-Einheit bilden
          Children: []
        - Name: Dichtes Scoring
          Responsibility:
            - Cosine Similarity als Dot-Product oder im exakten Modus berechnen und leere Embeddings ablehnen
            - Score je Query und Slot liefern und Top-k-Treffer ermitteln
          Children: []
        - Name: CPU-Compute-Primitive
          Responsibility:
            - Dot-Product, Max-Similarity, L2-Normalisierung und slotweisen Scan auf der CPU bereitstellen
          Children: []
        - Name: Max-Similarity-Operation
          Responsibility:
            - Top-k-Treffer je Query über Slot-Bereich per Tile-Kette aus Library-Operationen auf CPU oder GPU berechnen
            - Fused Kernel hinter derselben Operation anbieten
          Children: []
        - Name: Top-k-Selektion
          Responsibility:
            - begrenzte Treffermenge je Query aus Offset und Score nach Score, dann Offset ordnen
            - lokal je Workspace reduzieren und Teilergebnisse auf der CPU mergen
          Children: []
        - Name: GPU-Kernel
          Responsibility:
            - dichtes Dot-Product und fused Max-Similarity mit Dimension zur Kompilierzeit ausführen
            - mit festen Blockparametern starten und Launch-Fehler mit Ursache melden
          Children: []
        - Name: Kernel-Batchlauf-Schnittstelle
          Responsibility:
            - Batch beginnen, ausführen, Bereiche zuteilen, Fortschritt abfragen und beenden
            - Segmente ohne Pointer beschreiben und Adressen auf Entries zuordnen
          Children: []
        - Name: Work-Chunks
          Responsibility:
            - Planungseinheiten mit Zeitbudget aus geglättetem Durchsatz dimensionieren
            - Preemption an Chunk-Grenzen erlauben, sodass Query-Tasks Indexing-Chunks überholen
          Children: []
        - Name: Launch-Modi
          Responsibility:
            - direkten Launch ausführen und Graph-Capture sowie Persistent-Modus als nicht verfügbar ablehnen
            - Launch-Keys bilden und Warm-up vor dem ersten produktiven Launch ermöglichen
          Children: []
        - Name: Referenz-Scorer-Vergleich
          Responsibility:
            - Scorer-Ergebnisse mit Toleranz gegen einen Referenz-Scorer vergleichen
            - Abweichungen als Warnung loggen und nie zum Abbruch führen lassen
          Children: []
    - Name: Precision, Kalibrierung und Profile
      Responsibility:
        - Precision-Wunsch zwischen null und eins ohne Rounding-Vertrag auf Profile je Subengine abbilden
        - Hardware per Benchmark kalibrieren, ohne den Nutzer zu unterbrechen oder zu blockieren
      Children:
        - Name: Capability-Modell
          Responsibility:
            - Achsen und Werte als erweiterbare Capability-Sets ohne feste Profil-Enum führen
            - Requirements gegen Sets prüfen und Referenz-Sets als Obergrenzen vorgeben
          Children: []
        - Name: Capability-Erkennung
          Responsibility:
            - CPU- und GPU-Capabilities (FP16, INT8, Fusion, Tensor Cores) vom System ermitteln
            - Capabilities auf vorhandene Kernel reduzieren und numerische Achsen aus der Konfiguration übernehmen
          Children: []
        - Name: Benchmark-Workload und Abweichungsmaße
          Responsibility:
            - reproduzierbare synthetische Workload mit FP32-Referenz auf der CPU aufbauen
            - Cosine-Deviation, relativen Fehler, Top-k-Overlap und Validity-Zustände bewerten
          Children: []
        - Name: Messung
          Responsibility:
            - Abweichung und normierte Zeit je Kombination über ein austauschbares Mess-Interface erfassen
            - CPU- und GPU-Messmittel mit Warm-up, Unterbrechungsprüfung und Clock bereitstellen
          Children: []
        - Name: Kalibrierungslauf
          Responsibility:
            - Kombinationen interleaved gegen die Referenz messen, ungültige verwerfen und Kandidaten per Achsenstufe prunen
            - Messungen nach Stabilität, Dominanz, Obergrenzen und Budget beenden und Report erzeugen
          Children: []
        - Name: Pareto-Front und Profilableitung
          Responsibility:
            - Pareto-Front über Zeit und Abweichung berechnen und Positionen auf normierter Skala bestimmen
            - Profile über den Merge-Threshold ableiten
          Children: []
        - Name: Profilauswahl
          Responsibility:
            - Profil mit kleinstem Abstand zum begrenzten Wunsch wählen und ungültige Precision ablehnen
            - ohne aktives Profil das Modell je Subengine sperren (Latch) und Sperre aufhebbar halten
          Children: []
        - Name: Kalibrierungs-Persistenz
          Responsibility:
            - Ergebnisse je Modell und Subengine mit Fingerprint als Textdatei atomar ablegen und laden
            - Zustände fehlend, kalibriert und veraltet auflösen und Neukalibrierung im Hintergrund auslösen
          Children: []
        - Name: Hintergrundausführung und Statistik
          Responsibility:
            - Jobs auf benannten Low-Priority-Pools mit Futures ausführen
            - robuste Schätzer und Streuungsmaße bereitstellen
          Children: []
    - Name: Addons
      Responsibility:
        - Folge-Tasks mit geklonten oder geänderten Parametern über den normalen Weg enqueuen
        - Iteration, Kaskade und Rekursion außerhalb des Kerns realisieren
      Children: []
    - Name: Zielumgebung und Abhängigkeiten
      Responsibility:
        - Zielhardware (Turing, CUDA 13.3) mit JIT-Kompilierung für die vorhandene Architektur voraussetzen und Tensor-Core-Pfad sowie BF16-Compute nicht anbieten
        - Abhängigkeiten im Cargo-Manifest, Candle und die Formate safetensors sowie Parquet mit Arrow-Schema festlegen
      Children: []
```
