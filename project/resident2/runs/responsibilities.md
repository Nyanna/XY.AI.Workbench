# Responsibilities

- Eine Komponente ist ein Verantwortungsbereich, kein Datentyp.
- Name: Nominalphrase der Fachsprache, eindeutig, in der Sprache des Materials.

```yaml
- Name: Resident-Dienst für Embedding und Ähnlichkeitssuche
  Responsibility:
    - "Texte dauerhaft in Vektoren umwandeln (Embedding)"
    - "Ähnlichkeitssuche gegen Indizes ausführen (Similarity)"
  Children:
    - Name: Dienstzugang
      Responsibility:
        - "Dienst gegenüber der RAG-Anwendung nach außen zugänglich machen"
        - "Lebenszyklus von Konfiguration über Start bis Shutdown steuern"
      Children:
        - Name: Betriebsartenbereitstellung
          Responsibility:
            - "Dienst als Server-Prozess, Daemon oder über die CLI nutzbar machen"
          Children: []
        - Name: Dienstkonfiguration und Start
          Responsibility:
            - "Dienst konfigurieren, Pflichtangaben (Modell) prüfen und erst bei explizitem Start wirksam machen"
            - "Planung mit Subengines, Modellen und Indizes ausstatten, ohne Ressourcen anzulegen"
          Children: []
        - Name: Auftragseinreichung
          Responsibility:
            - "Embedding- und Similarity-Tasks entgegennehmen und einreihen"
          Children: []
        - Name: Indexregistrierung
          Responsibility:
            - "Beliebig viele durchsuchbare Indizes mit eindeutiger ID registrieren, ohne sie zu laden"
          Children: []
        - Name: Geordnetes Beenden
          Responsibility:
            - "Worker anhalten und joinen, nicht übernommene Tasks mit „cancelled“ abbrechen"
            - "Shutdown explizit oder beim Drop ausführen und bei Erfolg bestätigen"
          Children: []
    - Name: Auftragsmodell
      Responsibility:
        - "Aufträge samt Parametern, Zuständen und Ergebnisarten festlegen"
        - "Aufrufern Zugriff auf Zustand und Ergebnis geben"
      Children:
        - Name: Auftragszustand und Ergebnisablage
          Responsibility:
            - "Daten, Parameter, Zustand (enqueued, claimed, completed, cancelled) und Operationen eines Tasks kapseln"
            - "Ergebnis genau einmal als Output oder Fehler setzen und Wartende per Notification wecken"
          Children: []
        - Name: Aufruferseitiger Auftragszugriff
          Responsibility:
            - "Zustand und Ergebnis ohne Warten abfragbar machen (Polling)"
            - "Unbegrenzt oder mit Timeout ohne verlorene Notification warten und beim Drop unfertiger Handles abbrechen"
          Children: []
        - Name: Erweiterbare Auftragsparameter
          Responsibility:
            - "Beliebige typgebundene Zusatzwerte je Typ einmal speichern, abfragen und entfernen"
            - "Werte beim Klonen teilen und thread-safe unverändert durch Queue und Planung reichen"
          Children: []
        - Name: Definition der Embedding-Aufträge
          Responsibility:
            - "Text-Input, Parameter (Modell, Modus, Precision, Ziel-Index) und Vektorergebnis in Eingabereihenfolge festlegen"
            - "Direktes Schreiben des Embeddings samt Metadaten in den Index mit Entry-Referenzen als Ergebnis ermöglichen"
          Children: []
        - Name: Definition der Ähnlichkeitsaufträge
          Responsibility:
            - "Query-Vektoren und Parameter (Index-ID, Top-k, Scope, Scoring-Verfahren, Precision) festlegen"
            - "Textanfrage und Reranking als Folgen normaler Tasks ausdrücken statt als eigene Task-Art"
          Children: []
        - Name: Ergebnisarten
          Responsibility:
            - "Je Task genau eine Ergebnisart liefern (Embeddings, Entry-Referenzen, Treffer, Report, Scores)"
            - "Ergebnisarten erweiterbar und auf Gleichheit vergleichbar halten"
          Children: []
        - Name: Ergebnisbericht zur Ähnlichkeitssuche
          Responsibility:
            - "Rohe Scores und Fakten für äußere Schichten liefern, ohne Entscheidungen oder Thresholds"
          Children:
            - Name: Trefferaufbereitung
              Responsibility:
                - "Ungültige Treffer verwerfen, absteigend nach Score sortieren (Gleichstand: niedrigere Entry-ID) und auf Top-k kürzen"
                - "Stale-Flag übernehmen und zählen, Source-Link per Lookup ermitteln, fehlende Suchparameter als Fehler melden"
              Children: []
            - Name: Kennzahlenberechnung
              Responsibility:
                - "Statistik, Gap, Spread und z-Score über die zurückgegebenen Treffer berechnen"
                - "Softmax-Verteilung, Wahrscheinlichkeitsmasse, Entropie und Knee-Point mit definierten Voraussetzungen berechnen"
              Children: []
    - Name: Auftragsplanung
      Responsibility:
        - "Tasks in geordnete Operationsketten zerlegen"
        - "Operationen anhand der Capabilities den Queues zuweisen"
      Children:
        - Name: Aufgabenzerlegung
          Responsibility:
            - "Tasks genau einmal in eine Kette von Operationen zerlegen und Ergebnisse je Task-Art aufbereiten"
          Children:
            - Name: Zerlegungsprinzip
              Responsibility:
                - "Tasks als reine, zustandslose Funktion zerlegen und Tasks fremder Art mit Meldung ablehnen"
                - "Task-Parameter und Pipeline-Konfiguration pro Modell in die Operationskonfiguration übersetzen"
              Children: []
            - Name: Embedding-Zerlegung
              Responsibility:
                - "Embedding in Tokenisierung, Embedding-Berechnung, Pooling, L2-Normalisierung und optionales Schreiben in den Index ordnen"
                - "Schritte bei Pooling „none“, abgeschalteter Normalisierung oder fehlendem Ziel-Index weglassen"
              Children: []
            - Name: Similarity-Zerlegung
              Responsibility:
                - "Similarity in Similarity-Berechnung und Top-k-Selektion zerlegen und Top-k von null ablehnen"
                - "Cross-Encoder-Similarity in Inferenz auf Paaren zerlegen und nicht verfügbare Scorer ablehnen"
              Children: []
            - Name: Schrittweise Kettenübergabe
              Responsibility:
                - "Zunächst nur die erste Operation samt Collector-Funktion bereitstellen"
                - "Folgeoperation aus dem tatsächlichen Output der vorigen erzeugen und Input je Task-Art aufbereiten"
              Children: []
            - Name: Ergebnisaufbereitung je Task-Art
              Responsibility:
                - "Gesamtergebnis aus dem Output der letzten Operation aufbauen (Vektoren je Text, Treffer, Report, Scores)"
                - "Unpassende oder fehlende Ergebnisse als Konfigurationsfehler der Planung melden"
              Children: []
        - Name: Modellauswahl
          Responsibility:
            - "Default-Modell für Embedding- und Similarity-Tasks wählen"
            - "Nicht konfigurierte Modelle oder fehlende Expander mit „unbekanntes Modell“ abbrechen"
          Children: []
        - Name: Operationszuweisung
          Responsibility:
            - "Operationen anhand kompatibler Capabilities der GPU-, CPU- oder gemeinsamen Queue zuweisen, gemeinsame bevorzugt"
            - "Angebote je Subengine verwalten, verfügbare von geladenen Ressourcen unterscheiden und die erste Ressourcenart je Task vorab nennen"
          Children: []
        - Name: Hardwareaktivierung
          Responsibility:
            - "Benötigte Ressourcenarten den Subengines mitteilen und diese blockierend nur aktivieren, nicht laden"
          Children: []
    - Name: Operationsmodell
      Responsibility:
        - "Berechnungsschritte als Operationen mit Art, Parametern und Requirements beschreiben"
        - "Abschluss und Folgeoperationen je Task koordinieren"
      Children:
        - Name: Operationsbeschreibung
          Responsibility:
            - "Operationsarten, immutable Parameter und Descriptor als vergleichbare Keys festlegen"
            - "Priorität und Task-ID als Metadaten führen und jede Operation genau einem Task zuordnen"
          Children: []
        - Name: Operationsdaten
          Responsibility:
            - "Input und Output als RAM-Daten oder Pin einer Device-Region ausdrücken, nie als rohe Tensoren oder Pointer"
            - "Datenformen im RAM einschließlich gescortem Trefferergebnis definieren"
          Children: []
        - Name: Ressourcenanforderungen
          Responsibility:
            - "Requirements aus Ressourcenart, Modell, Index, Execution-Umgebung, Capabilities und Placement-Hint beschreiben"
            - "Erfüllung durch ein Device prüfen (Art akzeptiert, Capabilities abgedeckt, Placement-Hint ohne Einfluss)"
          Children: []
        - Name: Materialisierungsentscheidung
          Responsibility:
            - "Festlegen, ob ein Ergebnis im VRAM bleibt oder ins RAM zurückgeholt wird"
            - "Das Materialization-Flag bei Fertigstellung der Operation ausführen"
          Children: []
        - Name: Task-Kontext und Abschlusszählung
          Responsibility:
            - "Kontext mit ID, Priorität, Zähler offener Operationen, Cancel-Flag und erstem Fehler nebenläufig führen"
            - "Abschluss zählen, letzte offene Operation erkennen und den ersten Fehler als Task-Abbruch festhalten"
          Children: []
        - Name: Operationsknoten und Folgeoperationen
          Responsibility:
            - "Operation, Input, einmal setzbaren Output, Flag und Task mit optionalem Continuation-Handler bündeln"
            - "Folgeknoten vor dem Einreihen registrieren, damit der Zähler nicht vorzeitig null erreicht"
          Children: []
    - Name: Ausführungssteuerung
      Responsibility:
        - "Task-Queue, Operations-Queues, Worker und Notifications besitzen"
        - "Planung und Ausführung zwischen den Workern koordinieren"
      Children:
        - Name: Worker-Bereitstellung
          Responsibility:
            - "Worker in Höhe der CPU-Cores minus eins ohne Pinning bereitstellen und bei Initialisierung die Mindestzahl idle Threads starten"
            - "Worker parken und bei Zustandsänderung per Notification wecken"
          Children: []
        - Name: Worker-Ausführungsweise
          Responsibility:
            - "CPU-Operationen synchron und GPU-Operationen per Start mit anschließender Ergebnisverarbeitung ausführen"
            - "Blockierende Worker als natürliches Backpressure-Signal ohne Sonderbehandlung belassen"
          Children: []
        - Name: Prioritätsgesteuerte Arbeitsschleife
          Responsibility:
            - "Nach jeder Operation GPU-Ergebnisse, GPU-Ressourcen, CPU-Ressourcen und neue Tasks in dieser Reihenfolge prüfen"
            - "GPU- und CPU-Ergebnisse in den Index zurückspiegeln (Mirror-Back)"
          Children: []
        - Name: GPU-Abschlussmeldung
          Responsibility:
            - "Im Stream-Callback nur Completion-Eintrag ablegen und Notification auslösen, ohne GPU-API-Aufruf"
            - "Per Event-Query und atomarem Swap fertige Lanes übernehmen und verpasste Callbacks abdecken"
          Children: []
        - Name: Unterversorgungssteuerung
          Responsibility:
            - "Bei freier Ressource ohne Treffer eine Idle-Notification je Subengine-Art als Set erzeugen und verfallen lassen"
            - "Nur bei Idle-Notification oder leerer Operations-Queue expandieren und passende Tasks bevorzugen"
          Children: []
    - Name: Nebenläufige Arbeitsübergabe
      Responsibility:
        - "Arbeitseinheiten lock-free zwischen mehreren Producern und Consumern übergeben"
        - "Zustand, Ergebnis und Notification je Einheit bereitstellen"
      Children:
        - Name: Einreihung und Kettenaufbau
          Responsibility:
            - "Einträge in einer einfach verketteten Liste mit Sentinel per Compare-and-Swap anhängen und unlinken, Alter als Reihenfolge wahren"
            - "Enqueue nie blockieren, veralteten Tail-Hint ausgleichen und alle Notification-Channels signalisieren"
          Children: []
        - Name: Eintragszustände
          Responsibility:
            - "Die atomaren Übergänge Claim, Completion und Cancel mit genau einem Gewinner sicherstellen"
            - "Payload per Ownership-Transfer genau einmal übergeben und Ergebnis vor dem Zustand completed sichtbar machen"
          Children: []
        - Name: Eintragsdurchlauf
          Responsibility:
            - "Vollständige und nur aktive Iteration in Enqueue-Reihenfolge ohne Blockieren liefern"
            - "Gelieferte Einträge auch bei gleichzeitigem Enqueue, Claim und Completion gültig halten"
          Children: []
        - Name: Abschlusserwartung
          Responsibility:
            - "Asynchron auf completed oder cancelled warten, ohne einen Thread zu blockieren"
            - "Verlust von Notifications verhindern und mehrere Wartende gleichzeitig wecken"
          Children: []
        - Name: Knotenfreigabe
          Responsibility:
            - "Knoten per Reference Counting freigeben, lange Ketten iterativ ohne Stack-Overflow"
            - "Memory-Safety wahren, nur der atomare Pointer-Swap darf Sprachgarantien umgehen"
          Children: []
    - Name: Bedarfsgetriebene Ressourcenausführung
      Responsibility:
        - "Arbeit selbstorganisiert aus den Queues ziehen (Pull-Prinzip)"
        - "Ressourcen lazy initialisieren und Auslastung melden"
      Children:
        - Name: Bedarfsgetriebene Arbeitsentnahme
          Responsibility:
            - "Pro Subengine auf die Notification warten und die Queue in Altersreihenfolge iterieren, eigene Queue vor der gemeinsamen"
            - "Knoten per Filter, Ressourcenzustand und Kapazität claimen oder überspringen, ohne Reservierung und Head-of-Line-Blocking"
          Children: []
        - Name: Lazy-Initialisierung
          Responsibility:
            - "Ressourcen erst durch die Iteration initialisieren (uninitialized, loading, ready), nie durch Enqueue"
            - "Initialisierungs- und Startfehler in die Task-Ergebnisse des Batches schreiben und die Ressource in den Backoff versetzen"
          Children: []
        - Name: Stapelbildung der Lanes
          Responsibility:
            - "Operationen mit demselben Index oder Modell ab der Ausgangsoperation zu einem Batch aggregieren"
            - "Packing, Length-Bucketing und Streams der Ressourcenlogik überlassen"
          Children: []
        - Name: Benachrichtigungsfluss
          Responsibility:
            - "Append an eine Queue an die Notification jeder Subengine weiterleiten"
            - "Ergebnis, Initialisierung, Eviction-Timer und Iterationsfortschritt als eigene Notification der Subengine auslösen"
          Children: []
        - Name: Auslastungsmeldung
          Responsibility:
            - "Art, Device, Kapazität und geglätteten Durchsatz in Entries pro Sekunde melden"
            - "Zuteilung über den dynamischen Durchsatz der Lanes selbst steuern lassen"
          Children: []
    - Name: Rechenhardware
      Responsibility:
        - "Ressourcen und Execution-Kapazität je Hardwareart besitzen (Speicher, Streams, Kernel)"
        - "Compute-Pfade hardwareoptimal wählen"
      Children:
        - Name: Capability-Bereitstellung
          Responsibility:
            - "Capability-Set je Subengine liefern"
            - "Kompatible Ressourcen samt Lanes in eigenen und gemeinsamen Queues registrieren"
          Children: []
        - Name: Rechenpfad-Rückfall
          Responsibility:
            - "Batch-Verarbeitung Float-Batch zu Float-Batch über austauschbare Schnittstelle bereitstellen"
            - "CPU-Implementierung als Default und Fallback nutzen, wenn die GPU-Beschleunigung nicht initialisierbar ist"
          Children: []
        - Name: Schnittstelleninvarianten
          Responsibility:
            - "Tensoren und Raw-Pointer aus öffentlichen Signaturen heraushalten, nur Region und Layout verwenden"
            - "GPU-Arbeit nur über Lanes ausführen, Kernel nur über das Kernel-Register laden, Planung ohne Kenntnis der Neural-Network-Library"
          Children: []
        - Name: CPU-Recheneinheit
          Responsibility:
            - "CPU-Ressourcen mit Lanes und RAM-Haltung bereitstellen"
          Children:
            - Name: CPU-Geräte und Lanes
              Responsibility:
                - "CPUs (potenziell mehrere Sockets) mit Cores als Ressourcen und Processor-Threads als Lanes bereitstellen"
                - "Ohne Streams und Staging-Memory mit FP32 als Default-Datentyp arbeiten"
              Children: []
            - Name: RAM-Haltung
              Responsibility:
                - "Modelle und Indizes ins RAM laden oder streamen, Indizes per mmap lesbar halten"
                - "Verfügbares RAM kennen und Modelle, Kernel und Ressourcen bei Bedarf aus- und einlagern"
              Children: []
        - Name: GPU-Recheneinheit
          Responsibility:
            - "CUDA-GPU-Ressourcen mit Lanes und VRAM-Haltung bereitstellen"
          Children:
            - Name: GPU-Geräte und Lanes
              Responsibility:
                - "GPUs (potenziell mehrere) mit Compute Units als Ressourcen und Lanes bereitstellen"
                - "Streams, Staging-Memory und Transfers nutzen, FP16 als Default-Datentyp"
              Children: []
            - Name: VRAM-Haltung
              Responsibility:
                - "Modelle, Indizes, Kernel sowie Input-/Output-Arenen ins begrenzte VRAM laden oder streamen und bei Bedarf aus- und eintauschen"
                - "Index zwischen RAM und VRAM-Darstellung für die Kernel brücken, synchronisieren und Batching dafür nutzen"
              Children: []
            - Name: Referenzplattform
              Responsibility:
                - "Referenzumgebung (GTX 1660 SUPER, SM 7.5, CUDA 13.3) mit Acquire-Loads ab Compute Capability 7.0 unterstützen"
                - "Keinen Tensor-Core-Pfad und kein BF16 als Compute-Pfad anbieten"
              Children: []
        - Name: Geräte und Ausführungskontexte
          Responsibility:
            - "Devices, Lanes und Completion-Marker einheitlich für CPU und GPU modellieren"
          Children:
            - Name: Geräteidentität
              Responsibility:
                - "Device-Arten CPU und GPU erweiterbar mit Device-Handle (Ordinal) abbilden"
                - "GPU als einen Prozess mit Primary Context und mehreren Streams abbilden"
              Children: []
            - Name: Ausführungskontexte
              Responsibility:
                - "Lane aus Stream, Priorität, Events, Parameter-Block und Workspace bilden (Query-, Insert-/Update-, Kopier-Lanes)"
                - "Übergaben zwischen Lanes ausschließlich über die Queue abwickeln"
              Children: []
            - Name: Abschlussmarker
              Responsibility:
                - "Abschluss nicht blockierend abfragbar und blockierend abwartbar machen, GPU-Fehler als Event-Error melden"
                - "CPU-Events dauerhaft abgeschlossen halten, Laufzustand je Batch oder per Progress-Counter erfassen, Mock-Events für Tests erlauben"
              Children: []
    - Name: Texteingabeaufbereitung
      Responsibility:
        - "Texte in Token-IDs wandeln"
        - "Token-IDs zu gepaddeten Batches bündeln"
      Children:
        - Name: Tokenisierungs-Laden
          Responsibility:
            - "Tokenizer aus Konfigurationsdatei oder aus geladenem Tokenizer mit ID, Schema und maximaler Sequenzlänge bereitstellen"
            - "Pad-Token bestimmen, automatisches Padding abschalten, Trunkierung einrichten und Ladefehler mit Pfad und Ursache melden"
          Children: []
        - Name: Textnormalisierung
          Responsibility:
            - "Unicode-Normalisierung und Lowercasing parametriert nur auf der CPU ausführen"
            - "Optionalen austauschbaren Text-Normalizer vor Präfix und Tokenisierung anwenden, Doppelnormalisierung vermeiden"
          Children: []
        - Name: Encoding-Modi und Schemata
          Responsibility:
            - "Query- und Passage-Modus mit Präfix-Schema abbilden"
            - "Multi-Vector-Schema mit Markern, Query-Padding per Mask-Token und Dokument-Trunkierung anwenden"
          Children: []
        - Name: Tokenisierung von Textlisten
          Responsibility:
            - "Textliste je Modus als Batch in Token-IDs (u32) in Eingabereihenfolge wandeln, leere Liste ablehnen"
            - "Encoding-Fehler mit Ursache melden und Trunkierungen sowie größte Sequenzlänge loggen"
          Children: []
        - Name: Batch-Bildung
          Responsibility:
            - "Tokenfolgen per Length-Bucketing und Token-Budget zu gepaddeten Batches mit Attention-Mask gruppieren, Batch-Dauer begrenzen"
            - "Batches als u32 Little-Endian (2 × Zeilen × Sequenzlänge) serialisieren und Zeilen den Eingabe-Indizes zuordnen"
          Children: []
    - Name: Modellverarbeitung
      Responsibility:
        - "Modelle beschreiben und zu Encodern laden"
        - "Token-Batches in Vektoren verrechnen"
      Children:
        - Name: Modellbeschreibung
          Responsibility:
            - "Modell als ressourcenlosen Descriptor (Dimension, Pooling, Präfixe, Datentypen, Pfade) mit Default für multilingual-e5-small beschreiben"
            - "Beschreibung validieren und inkompatible Konfigurationen mit Begründung ablehnen"
          Children: []
        - Name: Architekturzuordnung
          Responsibility:
            - "Architektur-IDs Encoder-Implementierungen und Scoring-Arten Scorer-Implementierungen zuordnen"
            - "Neue Modelle über neuen Eintrag aufnehmen und einkompilierte Architekturen (bert per Feature-Flag) bereitstellen"
          Children: []
        - Name: Gewichtsbereitstellung
          Responsibility:
            - "Gewichte im Format safetensors per mmap zero-copy auf Rohbytes lesen"
            - "Gewichte pro Subengine in deren Arena kopieren"
          Children: []
        - Name: Encoder-Bereitstellung
          Responsibility:
            - "Encoder je Device-Klasse laden (validieren, Architektur nachschlagen, konstruieren) und mit Beschreibung bündeln"
            - "Geladenen Encoder als verwaltbare Ressource mit Key, Initialisierungskosten und Entladen führen"
          Children: []
        - Name: Transformer-Hauptmodell
          Responsibility:
            - "Modellkonfiguration prüfen und Gewichte per mmap im gewählten Datentyp auf CPU oder GPU laden"
            - "Forward-Pass zu FP32-Hidden-States ausführen, ohne Typen der Tensor-Library auszutauschen"
          Children: []
        - Name: Encoder-Aufbau
          Responsibility:
            - "Hauptmodell, Projektion, Pooling, Normalisierung und Storage-Datentyp zum Encoder zusammensetzen"
            - "Multi-Vector-Modelle über vorhandenes Backbone mit eigenem Projection-Head abbilden"
          Children: []
        - Name: Projektionsabbildung
          Responsibility:
            - "Token-Vektoren linear von Input- auf Output-Dimension abbilden"
            - "Weight und Bias aus safetensors laden und Shape sowie Typen prüfen"
          Children: []
        - Name: Pooling und Normalisierung
          Responsibility:
            - "Mean-Pooling über unmaskierte Token oder kein Pooling anwenden"
            - "L2-Normalisierung mit Epsilon-Clamp ausführen und degenerierte Vektoren zählen"
          Children: []
        - Name: Ausgabeschreibung
          Responsibility:
            - "Vektoren normalisiert und in FP32, FP16 oder BF16 konvertiert lückenlos in die Output-Region schreiben, auf der GPU per Write-Kernel"
            - "Degenerierte Vektoren warnend melden und Schreibfehler als Memory-Error weiterreichen"
          Children: []
        - Name: Asynchrone Encoder-Ausführung
          Responsibility:
            - "Input-/Output-Regionen und Layouts prüfen und Encoding asynchron auf einer Lane mit Completion-Event ausführen"
            - "Ergebnisse je Task über One-Shot-Channel liefern und Ausführungsfehler im Event hinterlegen"
          Children: []
        - Name: Cross-Encoder-Bewertung
          Responsibility:
            - "Paare aus Query und Textabschnitt per Cross-Encoder-Inferenz zu Scores verrechnen"
          Children: []
    - Name: Speicherverwaltung
      Responsibility:
        - "Device- und Host-Speicher in Arenen nach Lebensdauer einteilen und vergeben"
        - "Zugriff über pointerfreie Regionen und Layouts kapseln"
      Children:
        - Name: Arenenaufteilung
          Responsibility:
            - "Speicher in sechs Arenen mit stabiler ID und konfigurierbarer Kapazität einteilen und lazy anlegen"
            - "Allokation im Betrieb vermeiden, Stream-ordered Reuse sicherstellen und Kopien über eigenen Copy-Stream führen"
          Children: []
        - Name: Speicherbudget
          Responsibility:
            - "Budget aus freiem Speicher, Gesamtspeicher, Generationsreserve und Headroom über austauschbare Abfrage ermitteln"
            - "Fit-Check mit Copy-on-Write-Doppelbedarf durchführen und nutzbaren Anteil berechnen"
          Children: []
        - Name: Regionen und Layouts
          Responsibility:
            - "Regionen als pointerfreie Handles aus Arena-ID, Offset, Länge und Generation führen"
            - "Layout aus Shape, Strides und Elementtyp beschreiben, Views ändern nur das Layout"
          Children: []
        - Name: Elementtypen
          Responsibility:
            - "Eigene Elementtypen (FP32, FP16, BF16, u8, u32, i64) unabhängig von Fremd-Libraries definieren und aus safetensors übernehmen"
            - "Datentyp-Defaults je Device und den endlichen Mask-Wert −10 000 festlegen"
          Children: []
        - Name: Regionszugriff
          Responsibility:
            - "Bytes einer Region über einheitliche nebenläufige Schnittstelle lesen und schreiben"
            - "Tensor-Referenz aus Region und Layout als einzigen Weg für Tensoren über die Encoder-Schnittstelle führen"
          Children: []
        - Name: Freiraumvergabe
          Responsibility:
            - "Bereiche je Arena nach First-Fit in aufsteigender Adressreihenfolge aus fester Kapazität vergeben"
            - "Freie Lücken verschmelzen und Alignment sowie Overflows ohne Absturz behandeln"
          Children: []
        - Name: Vergabeverfolgung
          Responsibility:
            - "Offsets und Generationen je Arena thread-safe verfolgen und Liveness von Regionen prüfen"
            - "Stale Handles mit Warnung ignorieren, Allocation-Fehler per Fault Injection auslösbar machen, Vergaben loggen"
          Children: []
        - Name: Verzögerte Freigabe
          Responsibility:
            - "Regionen erst nach Abschluss ihres Events freigeben (Deferred Free, Reaping)"
            - "Je Arena genau einen Allocator samt einheitlicher Backend-Schnittstelle bereitstellen und zuständigen Allocator zu einer Region finden"
          Children: []
        - Name: CPU-Speicher-Backend
          Responsibility:
            - "Arenen als genullte RAM-Bereiche ohne Streams und Staging führen"
            - "Schreiben, Lesen und Kopieren (memmove, Zwischenpuffer) mit Liveness- und Längenprüfung ausführen"
          Children: []
        - Name: GPU-Speicher-Backend
          Responsibility:
            - "Nur mit Nvidia-Device anlegen, nach Speicherabfrage und Fit-Check Arenen als zusammenhängende genullte Blöcke reservieren"
            - "Mapped Host-Memory als Pinned Block bereitstellen, Budget frisch vom Treiber ermitteln und Belegung loggen"
          Children: []
        - Name: Device-Transfers
          Responsibility:
            - "D2D, asynchronen Upload (H2D) und Download (D2H) zwischen gültigen Regionen mit Transfer-Error bei Fehlern ausführen"
            - "Mapped Host-Memory ohne Transfer les- und schreibbar machen und Device-Adresse für Kernel-Launches bereitstellen"
          Children: []
        - Name: Übertragungspuffer-Vergabe
          Responsibility:
            - "Feste Anzahl Pinned Staging-Slots mit Zuständen frei, belegt und in Übertragung führen"
            - "Slots reihum vergeben und erst nach Event-Abschluss wiederverwenden, thread-safe"
          Children: []
        - Name: Schutz von Device-Regionen
          Responsibility:
            - "Zugriff auf eine Device-Region über Pin abbilden und Eviction solange verhindern"
            - "Pin-Count nebenläufig führen, per RAII freigeben und optional an einen Rückmeldekanal melden"
          Children: []
    - Name: Anbindung externer Bibliotheken
      Responsibility:
        - "Externe Libraries jeweils an genau einer Stelle einbinden und Fremdtypen nur an diesen Rändern konvertieren"
        - "Geprüfte Dependencies aus dem Projekt-Manifest verwenden"
      Children:
        - Name: Tensor-Bibliotheksanbindung
          Responsibility:
            - "Candle für Modellberechnung und Tile-Ketten einbinden, Elementtypen sowie Element- und Bytezahlen in beide Richtungen konvertieren"
            - "Teilbereiche eines Device-Blocks besitzlos als Tensor einhüllen, ohne den Block freizugeben"
          Children: []
        - Name: GPU-Plattformanbindung
          Responsibility:
            - "Einzige Anbindungsstelle der GPU-Library bereitstellen, Anlegen ohne kompatible GPU scheitern lassen"
            - "Treiberabfragen (Speicher, Compute Capability, Treiberversion, GPU-Name) kapseln"
          Children: []
        - Name: Formatanbindung
          Responsibility:
            - "safetensors-Gewichte und Parquet-Dateien mit Arrow-Schema über Bibliotheken lesen und schreiben"
          Children: []
    - Name: Durchsuchbarer Vektorbestand
      Responsibility:
        - "Entries mit Vektoren und Nutzdaten als Source of Truth halten"
        - "Änderungen melden sowie Struktur und Change-Stamps liefern"
      Children:
        - Name: Bestandsvertrag
          Responsibility:
            - "Entries nebenläufig lesen, schreiben (Last-Writer-Wins, Reihenfolge Daten, Stamp, Signal) und entfernen"
            - "Unterstützte Zugriffsmethoden (Centroid, Prefix, Subtree, Range) deklarieren und zugehörige Strukturen liefern"
          Children: []
        - Name: Änderungsverfolgung
          Responsibility:
            - "Eindeutige opake Change-Stamps für jede Schreib- und Löschoperation vergeben und nach Wiederaufnahme fortsetzen"
            - "Level-Signal mit aktuellem Max-Stamp an beliebig viele Observer liefern"
          Children: []
        - Name: Entry-Mengen
          Responsibility:
            - "Entry-Mengen kompakt als Ranges oder Bitmap mit 64-Bit-IDs führen"
            - "Mengenoperationen (Union, Intersection, Iteration, Hash, Runs) anbieten"
          Children: []
        - Name: Suchbereichsarten
          Responsibility:
            - "Scope-Arten all, range, set, subtree, prefix und centroid mit stabilem Namen beschreiben"
            - "Neue Scope-Arten ergänzbar halten"
          Children: []
        - Name: Referenzbestand im RAM
          Responsibility:
            - "Referenz-Index im RAM ohne GPU-Beschleunigung und ohne Machine-Learning-Libraries lauffähig bereitstellen"
          Children: []
        - Name: Dateigestützter Bestand
          Responsibility:
            - "Index als Parquet-Datei öffnen, lazy dekodieren, pflegen und atomar sichern"
          Children:
            - Name: Öffnen und Prüfen
              Responsibility:
                - "Datei per mmap oder File-Read öffnen, fehlende Datei als leeren Index behandeln, Entries ohne Vektor-Dekodierung einlesen"
                - "Header und Spalten-Schema gegen die Erwartung prüfen und optional per Checksum verifizieren"
              Children: []
            - Name: Lazy-Dekodierung
              Responsibility:
                - "Vektoren und Nutzdaten beim ersten Gebrauch genau einmal in Batches von 4096 Zeilen als FP16 dekodieren"
                - "Fehlschläge merken und bei jedem Zugriff melden"
              Children: []
            - Name: Entry-Pflege im Speicher
              Responsibility:
                - "Upsert mit Vektorlängenprüfung, Checksum, Stamp und Dirty-Markierung sowie Remove ausführen"
                - "Nutzdaten, Pfad, Header, Max-Stamp und Dirty-Status abfragbar und explizit verifizierbar machen"
              Children: []
            - Name: Persistentes Sichern
              Responsibility:
                - "Konsistenten Snapshot aller Entries aufsteigend nach ID atomar (Temp-Datei, fsync, Rename) als Parquet schreiben"
                - "Gleichzeitige Saves serialisieren und ohne Änderung nichts schreiben"
              Children: []
        - Name: Dateischema
          Responsibility:
            - "Spaltenfolge (Entry-ID, Change-Stamp, Vektor, Zieldatei, Zeichenanfang, Zeichenende, Checksum) festlegen"
            - "Header als verlustfrei les- und schreibbare Key-Value-Paare im Datei-Footer führen"
          Children: []
        - Name: Modellbindung
          Responsibility:
            - "Header aus der Modellbeschreibung ableiten und Bindung über Model-ID, Dimension und Pipeline-Merkmale prüfen"
            - "Max-Stamp und Compute-Datentyp als informativ ausnehmen und Tokenizer-Fingerprint bilden"
          Children: []
        - Name: Nutzdaten und Prüfsumme
          Responsibility:
            - "Jeden Entry über Zieldatei, Zeichenanfang und Zeichenende auf den Quelltext verweisen lassen"
            - "64-Bit-Checksum aus Vektor und Verweis beim Speichern berechnen und bei Verifikation neu prüfen"
          Children: []
        - Name: Quelländerungserkennung
          Responsibility:
            - "Änderungen an Quelldateien über Fallback-Kette aus Change-Stamp, Größe und Timestamp, Content-Hash erkennen"
            - "Racy Aufzeichnungen erkennen und Content-Prüfung anfordern"
          Children: []
    - Name: Residenzsteuerung des Vektorbestands
      Responsibility:
        - "Zuordnung von Entry zu Segment und Slot, Residency und Updates führen"
        - "Upload, Eviction, Updates und Rückabbildung von Adressen auf Entries verantworten"
      Children:
        - Name: Bestandsanbindung der Residenzsteuerung
          Responsibility:
            - "Index erst bei Bedarf anhängen und Aktivierung mit Once-Semantik initialisieren"
            - "Zugriff ohne Index mit Index-ID abweisen sowie Aktivität und Zahl residenter Entries melden"
          Children: []
        - Name: CPU-Residenz
          Responsibility:
            - "Lesezugriffe direkt an den Index leiten und die Residency dateigestützter Daten im Page-Cache steuern"
            - "Einzel- und Mehrfachlesen mit Zugriffsmuster im Blocking-Pool ausführen und Bereiche nach Leerlauf als cold markieren"
          Children: []
        - Name: GPU-Residenz
          Responsibility:
            - "Mengen scope-getrieben ins VRAM sicherstellen (Ensure) und Reconcile-Iterationen ausführen"
            - "Geladene Segmente publizieren und freigeben sowie residente Bytes und ausstehende Updates melden"
          Children: []
        - Name: Zugriffshinweise an das Betriebssystem
          Responsibility:
            - "Unverbindliche Advice (will-need, sequential, random, cold) je Byte-Bereich über austauschbare Schnittstelle geben"
            - "Ungültige Bereiche abweisen, cold auf Page-Grenzen ausrichten und OS-Fehler weiterreichen"
          Children: []
        - Name: Scope-Auflösung
          Responsibility:
            - "Scope einer Operation in kompakte Entry-Menge auflösen (all, set, range, subtree, prefix, centroid)"
            - "Nicht unterstützte Scopes mit Methodenname ablehnen und bei bekannter Kandidatenmenge nur diese Bereiche laden"
          Children: []
        - Name: Residency-Verzeichnis
          Responsibility:
            - "Je residentem Entry Slot (Segment, Slot-Nummer) und bekannten Stamp führen"
            - "Insert, Remove, Abfrage und Iteration anbieten"
          Children: []
        - Name: Adress-Rückabbildung
          Responsibility:
            - "Kernel-Adressen bei festem Layout per Arithmetik, bei variablem per Offset-Tabelle auf Entries zurückabbilden"
            - "Layout für Segmente, Kandidatenmengen und kernel-gebaute Segmente aus der Index-Struktur bestimmen"
          Children: []
        - Name: Ladestrategie
          Responsibility:
            - "Nur das laden, was ein Task braucht, und Ladevorgänge bündeln"
          Children:
            - Name: Ladeplanung
              Responsibility:
                - "Über austauschbare Geometrie und Policy (Dichteschwelle, Höchstgrößen) zwischen ganzen Segmenten, Gather und Zero-Copy wählen"
                - "Segmentgröße nach Budget bestimmen und Pläne nacheinander mit Abbruch beim ersten Fehler ausführen"
              Children: []
            - Name: Ladebündelung
              Responsibility:
                - "Tasks gegen denselben Index über die Union ihrer Scopes bündeln"
                - "Gleichzeitige Requests mit gleichem Key auf einen gemeinsamen Ladevorgang vereinen (Single-Flight)"
              Children: []
            - Name: Gestaffelte Haltung
              Responsibility:
                - "Summary-Strukturen zuerst und am längsten, Vektorsegmente warm und evictable halten, alles andere nicht laden"
                - "Bei überlappenden Entries entscheiden, ob Vektoren einmal gehalten oder dupliziert werden"
              Children: []
            - Name: Ladekoordination
              Responsibility:
                - "Segmentverwaltung, Table-Swap, Mapping-Store und geladene Segmente je Index-Operation verbinden und selbständig publizieren"
                - "Strukturelle Änderungen serialisieren, bei erschöpfter Arena Platz schaffen und genau einmal wiederholen"
              Children: []
        - Name: Segmenttransfer auf das Device
          Responsibility:
            - "Geladene Segmentbereiche aus dem RAM in den VRAM überführen"
          Children:
            - Name: Direkter Segmenttransfer
              Responsibility:
                - "Lückenlos liegende Entries ganzer Segmente direkt kopieren und Bindings mit vor den Daten gelesenem Stamp erzeugen"
                - "Nicht direkt kopierbare Segmente in den Gather-Pfad geben"
              Children: []
            - Name: Gather-Transfer
              Responsibility:
                - "Entries ausgerichtet in Staging-Slots packen und asynchron über die Copy-Lane kopieren"
                - "Fortschritt und Arena-Erschöpfung absichern und Slots erst nach dem Event wiederverwenden"
              Children: []
            - Name: Zero-Copy-Mapping
              Responsibility:
                - "Sehr kleine Mengen über gemappten Host-Memory ohne Kopie bereitstellen"
                - "Ohne gemappte Arena oder Slot auf den Gather-Pfad zurückfallen"
              Children: []
        - Name: Verdrängung
          Responsibility:
            - "Mirrors in VRAM und RAM ausschließlich explizit und kostenbewusst evicten"
          Children:
            - Name: Verdrängungsrichtlinie
              Responsibility:
                - "High-Watermark, Low-Watermark und Idle-Dauer je VRAM und RAM festlegen und ungültige Marken korrigieren"
                - "Summaries und Pinned Entries nie zum Victim machen und nutzbare Kapazität ableiten"
              Children: []
            - Name: Verdrängungspriorisierung
              Responsibility:
                - "Aufgenommene Segmente mit Größe, Tier und Reload-Kosten nach GreedyDual-Size-Frequency priorisieren"
                - "Zugriffe als Hits erfassen und den Floor bei Pressure- oder Space-Need-Eviction anheben"
              Children: []
            - Name: Verdrängungsausführung
              Responsibility:
                - "Idle-Eviction, Pressure-Relief und Platzbeschaffung (Space-Need) in definierter Victim-Reihenfolge ausführen"
                - "Evictete Segmente aus Tabelle und Zuordnung entfernen und jede Entscheidung loggen"
              Children: []
    - Name: Bestandsabgleich
      Responsibility:
        - "Mirrors und Index konsistent halten, bei Abweichung gewinnt der Index"
        - "Änderungen zustandsbasiert statt ereignisbasiert abgleichen"
      Children:
        - Name: Konsistenzregeln
          Responsibility:
            - "Identität eines Entries als Paar aus ID und Stamp definieren, Stamps nur auf Ungleichheit vergleichen und Änderungen serialisiert anwenden"
            - "Stale Ergebnisse ohne Locks auf den Datenpfaden als Nicht-Fehler zulassen"
          Children: []
        - Name: Zustandsbasierter Abgleich
          Responsibility:
            - "Je Subscriber (GPU, CPU, Persistenz-Timer) eigenen Reconciler führen und Signal vor dem Vergleich quittieren"
            - "Residente Entries nach Stamp als entfernt, nachzuladen oder aktuell einordnen"
          Children: []
        - Name: Konsistentes Laden und Echo-Vermeidung
          Responsibility:
            - "Entry in der Reihenfolge Stamp, Daten, erneuter Stamp laden und bei Bewegung bis zu achtmal wiederholen"
            - "Stamp des eigenen Upserts als bekannt eintragen und Entfernen sowie Neuanlage über Stamp-Abfragen erkennen"
          Children: []
    - Name: Segmentverwaltung im VRAM
      Responsibility:
        - "Index-Segmente im VRAM aufbauen, versionieren und mit Speicher versorgen"
        - "Mutationen unter laufenden Kernels konsistent veröffentlichen"
      Children:
        - Name: Segmentaufbau
          Responsibility:
            - "Segmente in Slots fester Größe gliedern und Slots Entries zuordnen"
            - "Kernel-Treffer auf Entries auflösen"
          Children:
            - Name: Slotgeometrie
              Responsibility:
                - "Slot-Größen aus Token-Obergrenze, Dimension und Datentypgröße in Slot-Klassen definieren"
                - "Segment-View mit Kernel-Descriptor bereitstellen und Slot-Inhalte mit endlichem Mask-Wert gepaddet kodieren"
              Children: []
            - Name: Slot-Belegung und Sichtbarkeit
              Responsibility:
                - "Slots per Append reservieren und sofort oder deferred bis zum Event committen, Valid-Bit erst danach setzen"
                - "Entries per Tombstoning ausblenden, Valid-Bits als Snapshot bereitstellen und Segmente per Compaction neu aufbauen"
              Children: []
            - Name: Slot-Entry-Zuordnung
              Responsibility:
                - "Slots je Segment Entry-Referenzen zuordnen (Bind, Unbind, Lookup) nebenläufig"
                - "Slots in Device-Adressen und absolute Byte-Bereiche umrechnen"
              Children: []
            - Name: Trefferauflösung
              Responsibility:
                - "Kernel-Treffer in Segment-Generation, Slot und Score umrechnen und über das Manifest zu Entry-Referenzen auflösen"
                - "Treffer älterer Manifest-Generationen überführen oder verwerfen und neuesten gültigen Ort ermitteln"
              Children: []
        - Name: Segmentversionierung
          Responsibility:
            - "Segmente als versionierte Manifeste veröffentlichen"
            - "Reader per Lease vor vorzeitiger Freigabe schützen"
          Children:
            - Name: Manifest-Versionierung
              Responsibility:
                - "Manifest als Snapshot der Segmente mit fortlaufender Generation führen und lock-free atomar ersetzen"
                - "Segmenttabelle für Kernel binär kodieren und nicht strikt größere Generationen ablehnen"
              Children: []
            - Name: Lesesicherung durch Leases
              Responsibility:
                - "Aktuelle Generation per Lease sichern und Completion-Marker bei allen Segmenten registrieren"
                - "Nicht mehr gehaltene Segmente samt offener Marker melden und per Reclaim zurückgeben"
              Children: []
            - Name: Veröffentlichung hinter Kopierabschluss
              Responsibility:
                - "Neue Manifeste hinter dem Event ihres Kopiervorgangs vormerken und in Reihenfolge publizieren"
                - "Änderungsereignisse (added, changed) an beliebig viele Subscriber senden"
              Children: []
        - Name: Mutationsschutz im laufenden Betrieb
          Responsibility:
            - "Veränderungen unter laufenden Kernels nebenläufig korrekt veröffentlichen"
          Children:
            - Name: Mutationsfenster-Disziplin
              Responsibility:
                - "Änderungen nur in Gaps (Batch-Ende bzw. Communication-Slot) ausführen, Kernel nie blockieren oder aussperren"
                - "Torn Reads im Persistent-Modus bewusst akzeptieren und dokumentieren"
              Children: []
            - Name: Segmenttabellen-Austausch
              Responsibility:
                - "Nur natürlich ausgerichteten 8-Byte-Pointer atomar tauschen, größere Strukturen per Copy-on-Write neu anlegen"
                - "Root-Pointer (Release/Acquire) und Segmenttabelle mit Epoche per serialisierter Publication tauschen und sichtbar schreiben"
              Children: []
            - Name: Epochenverfolgung der Reader
              Responsibility:
                - "Publizierte und je Reader gemeldete Epochen nur steigend führen, ruhende Reader (Quiescent) als angekommen werten"
                - "Reclaim abgelöster Versionen höchstens bis zur kleinsten Reader-Epoche erlauben"
              Children: []
            - Name: Batch-Quittierung
              Responsibility:
                - "Abgeschlossene Batches mit Release publizieren und vom Host mit Acquire lesen lassen"
                - "Gap bei Abweichung zwischen completed und acked melden und per Ack schließen"
              Children: []
            - Name: Konsistente Lesesnapshots
              Responsibility:
                - "Gleichzeitig veränderbare Daten nach Seqlock-Prinzip wortweise atomar lesen"
                - "Vollständige Kopie samt Retry-Zahl liefern, Writer ausschließen und Zähler erhöhen"
              Children: []
        - Name: Rückschreibe- und Zuordnungsabgleich
          Responsibility:
            - "Kernel-Writes und Ergebnisse in Gaps auf den Bestand abbilden"
            - "Zuordnungsänderungen versioniert und gesammelt einspielen"
          Children:
            - Name: Rückschreibe- und Auflösungsablauf
              Responsibility:
                - "Im Communication-Slot in fester Reihenfolge Kernel-Writes am Index anwenden, Ergebnisse auflösen (verwerfen, stale, unverändert) und Updates einspielen"
                - "Bei weitergegangener Zuordnung stale Mapping melden und Slot mit frischem Snapshot wiederholen, Device-Bereiche über austauschbare Schnittstelle zurücklesen"
              Children: []
            - Name: Adresszuordnung
              Responsibility:
                - "Arena-Adressen per Copy-on-Write und Epoche auf Entries abbilden, Überlappungen abweisen"
                - "Maps versioniert halten, Snapshots nur bei passender Epoche liefern und nur Epoche plus eins publizieren"
              Children: []
            - Name: Ausstehende Zuordnungsänderungen
              Responsibility:
                - "Binds und Unbinds mit Stamp einreihen und je Entry nur das jüngste Update behalten"
                - "Deletes vor Binds anwenden und das Residency-Verzeichnis nachführen"
              Children: []
        - Name: Kernel-gestützter Bestandsaufbau
          Responsibility:
            - "Index-Aufbau durch Kernel ermöglichen, ohne dass der Kernel allokiert"
          Children:
            - Name: Schreibzeiger-Vergabe
              Responsibility:
                - "Vorab beschreibbaren Bereich vergeben und Claims per Write-Pointer-Modell mit Overflow-Erkennung zusammenhängend vergeben"
                - "Warp-Claims per Prefix-Sum mit einer atomaren Aktualisierung bündeln und Ergebnisse für den Host sichtbar machen"
              Children: []
            - Name: Embedding-Batch-Lauf
              Responsibility:
                - "Batch durch Resume-Schritte mit Teilergebnissen und Nachvergabe von Bereichen ausführen und immer beenden"
                - "Bei Fehlern reservierte Bereiche freigeben und zuerst den Lauffehler melden"
              Children: []
            - Name: Write- und Trefferkanäle
              Responsibility:
                - "Embedding-Writes und Treffer über getrennte SPSC-Kanäle führen, Writes zuerst registrieren"
                - "Writes vor Treffern entnehmen, damit kein Treffer ohne Adresszuordnung auftritt"
              Children: []
        - Name: Segmentspeicher
          Responsibility:
            - "Speicher für Segmente vergeben, ersetzen und beschreibbar halten"
          Children:
            - Name: Segmentspeichervergabe
              Responsibility:
                - "Segmente 256-Byte-ausgerichtet unter Reserve und Replacement-Headroom zuteilen und vorab ausstehende Freigaben einsammeln"
                - "Freigabe sofort nur ohne mögliche Reader, sonst nach Event oder Epoche, Budgets overflow-sicher führen"
              Children: []
            - Name: Segmentersetzung
              Responsibility:
                - "Segment per Kopie gleicher Größe neben dem alten ersetzen, höchstens ein Replacement gleichzeitig"
                - "Abschluss mit Epoche publizieren und altes Segment zurückstellen, Discard gibt die Kopie frei"
              Children: []
            - Name: Beschreibbare Segmente
              Responsibility:
                - "Segmente per Bump-Allocation für einen Writer schrittweise beschreibbar machen und bei Erschöpfung ohne Zustandsänderung melden"
              Children: []
    - Name: Ähnlichkeitsberechnung
      Responsibility:
        - "Ähnlichkeitsscores und Top-k-Treffer auf CPU und GPU berechnen"
        - "Kernel zur Laufzeit bereitstellen"
      Children:
        - Name: Kernel-Katalog
          Responsibility:
            - "Kernel zu Name, Datentyp, Architekturstufe und Variante lazy ohne doppelte Kompilierung bereitstellen"
            - "Einträge zählen, verwerfen und als Ressource unter fester ID auffindbar machen"
          Children: []
        - Name: Laufzeit-Kompilierung
          Responsibility:
            - "Kernel per NVRTC mit gemeinsamem Prelude für die aus der Compute Capability abgeleitete Zielarchitektur kompilieren"
            - "Bei Treiber-Ablehnung von PTX als SASS neu kompilieren, vorkompilierte und externe Kernel einbinden, Fehler melden"
          Children: []
        - Name: Unterstützte Precisions
          Responsibility:
            - "Je Device-Art festlegen, für welche Elementtypen Scoring-Kernel existieren (CPU: FP32, GPU: FP32 und FP16)"
            - "Operanden- und Scoring-Achsen nur für unterstützte Typen melden"
          Children: []
        - Name: Query-Bündelung und Dokumentquellen
          Responsibility:
            - "Queries gleicher Dimension zu einem geprüften Query-Batch bündeln"
            - "Dokumentvektoren über Source-Schnittstelle und Workspaces (halboffene Slot-Bereiche) bereitstellen"
          Children: []
        - Name: Dichtes Scoring
          Responsibility:
            - "Cosine Similarity als Dot-Product (im exakten Modus mit Normprodukt) je Query und Slot berechnen"
            - "Leere Embeddings und falsche Slot-Dimensionen als Fehler ablehnen"
          Children: []
        - Name: CPU-Compute-Primitive
          Responsibility:
            - "Dot-Product, Max-Similarity und L2-Normalisierung auf der CPU SIMD-tauglich berechnen"
            - "Slotweisen Scan mit laufendem Maximum je Query-Token ausführen"
          Children: []
        - Name: Max-Similarity-Operation
          Responsibility:
            - "Top-k-Treffer je Query über Slot-Bereich und Query-Batch als austauschbare Operation liefern"
            - "Tile-Kette mit wählbarer Precision und fused Kernel hinter derselben Operation bereitstellen"
          Children: []
        - Name: Top-k-Selektion
          Responsibility:
            - "Begrenzte Menge je Query aus Offset und Score absteigend führen (Gleichstand: niedrigerer Offset) und ungültige Werte verwerfen"
            - "Teilergebnisse je Workspace auf der CPU mergen und mehrere Bereiche nacheinander scoren"
          Children: []
        - Name: GPU-Scoring-Kernel
          Responsibility:
            - "Dichtes Dot-Product und fused Max-Similarity in FP32 und FP16 bereitstellen"
            - "Dimension bei Kompilierung festlegen, mit 128 Threads je Block starten und Launch-Fehler melden"
          Children: []
        - Name: Kernel-Batchlauf
          Responsibility:
            - "Segmente mit festem oder variablem Layout als Batch ohne Pointer beginnen, ausführen und beenden"
            - "Adresse per Stride oder Offset-Intervall einem Entry zuordnen und Fortschritt abfragbar machen"
          Children: []
        - Name: Work-Chunk-Planung
          Responsibility:
            - "Planungseinheiten mit Zeitbudget aus geglättetem Durchsatz (EMA) zwischen Mindest- und Höchstlänge dimensionieren"
            - "Preemption nur an Chunk-Grenzen zulassen und Tiles gegen alle wartenden Queries scoren"
          Children: []
        - Name: Launch-Steuerung
          Responsibility:
            - "Direkten Launch verfügbar machen, Graph-Capture und Persistent-Modus als nicht verfügbar ablehnen"
            - "Launch-Keys mit Shape-Klasse und Region-Generation bilden und Warm-up ermöglichen"
          Children: []
        - Name: Scorer-Referenzvergleich
          Responsibility:
            - "Scorer-Ergebnisse mit absoluter Toleranz gegen einen Referenz-Scorer vergleichen"
            - "Abweichungen als Warnung mit Segment loggen, ohne abzubrechen"
          Children: []
    - Name: Genauigkeitskalibrierung
      Responsibility:
        - "Hardware per Benchmark kalibrieren und daraus Precision-Profile bereitstellen"
        - "Precision-Wunsch auf Profile abbilden, ohne den Nutzer zu blockieren"
      Children:
        - Name: Capability-Beschreibung
          Responsibility:
            - "Achsen und Werte als erweiterbare Strings ohne fest einkodierte Profile beschreiben"
            - "Capability-Sets reihenfolgeunabhängig führen und Requirements dagegen prüfen"
          Children: []
        - Name: Capability-Erkennung
          Responsibility:
            - "CPU- und GPU-Capabilities (F16C, AVX2, VNNI, FP16, DP4A) ermitteln, Tensor Cores nur loggen und fehlende Unterstützung als Capability-Error melden"
            - "Numerische Achsen aus der Konfiguration ableiten und auf vorhandene Kernel reduzieren"
          Children: []
        - Name: Benchmark-Workload und Abweichungsmaße
          Responsibility:
            - "Reproduzierbare synthetische Workload mit FP32-CPU-Referenz erzeugen"
            - "Abweichung (Cosine-Deviation, relativer Fehler, Top-k-Overlap) bewerten und Validity-Zustände bestimmen"
          Children: []
        - Name: Messung
          Responsibility:
            - "Abweichung und Zeit je Kombination über ein austauschbares Mess-Interface erfassen"
            - "CPU- und GPU-Messmittel mit Warm-up, Unterbrechungsprüfung und austauschbarer Clock bereitstellen"
          Children: []
        - Name: Kalibrierungslauf
          Responsibility:
            - "Kombinationen messen, einstufen und zu einem Report verdichten"
          Children:
            - Name: Kalibrierungsplanung
              Responsibility:
                - "Kombinationen aus dem Capability-Set aufzählen und Achsen in Kern- und separierbare Achsen teilen"
                - "Achsenstufe (Pruning) und Verifikationsschritt oder Vollmessung innerhalb der Budgets planen"
              Children: []
            - Name: Gruppenmessung
              Responsibility:
                - "Abweichungsphase mit Verwerfung ausführen, danach Zeit interleaved im Round-Robin gegen den Anchor messen"
                - "Unterbrechungen mit Backoff-Pause behandeln und Versuche zählen"
              Children: []
            - Name: Messbeendigung und Einstufung
              Responsibility:
                - "Kombinationen als dominated, stable, unstable, verworfen, übersprungen oder gemessen einstufen"
                - "Report mit Front, Zählern und Log jeder Einordnung aufbauen"
              Children: []
        - Name: Pareto-Front und Profilableitung
          Responsibility:
            - "Pareto-Front über Zeit und Abweichung bilden und Positionen auf normierter Skala berechnen"
            - "Profile über den Merge-Threshold ableiten"
          Children: []
        - Name: Profilauswahl
          Responsibility:
            - "Precision-Wunsch auf null bis eins begrenzen und das nächstgelegene Profil wählen, bei Gleichstand das präzisere"
            - "Ohne aktives Profil den ersten Task fehlschlagen lassen und das Modell sperren (Latch), Sperre aufhebbar"
          Children: []
        - Name: Kalibrierungspersistenz
          Responsibility:
            - "Ergebnisse je Modell und Subengine mit Fingerprint atomar speichern und laden"
            - "Zustände fehlend, kalibriert und veraltet beim Start fehlerfrei auflösen und im Hintergrund neu kalibrieren"
          Children: []
        - Name: Hintergrundausführung und Statistik
          Responsibility:
            - "Low-Priority-Pools mit Future-Ergebnissen und Drain beim Shutdown bereitstellen"
            - "Schätzer (Median, Minimum) und Streuungsmaße ohne langsamstes Fünftel berechnen"
          Children: []
    - Name: Zusatzfunktionsanbindung
      Responsibility:
        - "Folge-Tasks mit geklonten oder geänderten Parametern über den normalen Weg einreihen"
        - "Iteration, Kaskade und Rekursion ausschließlich außerhalb des Kerns zulassen"
      Children: []
```
