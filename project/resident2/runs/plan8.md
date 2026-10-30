Erzeuge aus dem folgenden Korpus ein Glossar (Nur: Begriff, Synonyme, Bedeutungen, Beziehungen).

Material:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/plan7.md)
Result Stats: id=60f65f05-df0f-4bf8-bed8-c943d17dad8d, total: 83858, in: 65114, out: 18744, reason: 873, read: 0, write: 0, start: 1793383918032, end: 1793384029161, time: 110770, total: 110770, io: 330, control: 359, sent: 1, recv: 2988
SystemInit:  id=60f65f05-df0f-4bf8-bed8-c943d17dad8d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 873
# Glossar: Resident

## A

**Addon**
- Synonyme: Erweiterung (in Subengine, Anwendung oder Ressourcenimplementierung)
- Bedeutung: Komponente außerhalb des Kerns, die Iteration, Kaskade oder Rekursion erzeugt, indem sie Folge-Tasks über den normalen Weg enqueued. Addon-Tasks sind von Anwendungs-Tasks nicht unterscheidbar.
- Beziehungen: nutzt das erweiterbare Parameterobjekt des Task. Ihre Tasks werden über Pull-Filter gezogen. Beispiele sind Centroid-Task mit Folge-Task, Text-Query und Reranking. Der schlanke Kern kennt diese Verhaltensweisen nicht.

**Allocator**
- Synonyme: Offset-Allocator, Bookkeeping-Allocator
- Bedeutung: Verwaltet je Arena freie Bereiche (First-Fit, Coalescing) und die Gültigkeit von Offsets und Generationen, ohne selbst Speicher zu halten. Die Variante mit Deferred Free gibt Regionen erst nach einem Event frei.
- Beziehungen: gehört zu einer Arena. Ein Allocator-Set stellt je Arena genau einen bereit. Bedient wird er vom Memory-Backend. Verwaltet Regionen.

**Arena**
- Synonyme: Speicherklasse
- Bedeutung: Speicherbereich mit eigener Lebensdauer und ID. Es gibt sechs: Weights, Index-Segmente, Workspace je Lane, Staging, Parameter-Block je Lane und Mapped Host-Memory. Adressen sind 64-Bit-Offsets innerhalb der Arena.
- Beziehungen: wird lazy nach Budget angelegt. Enthält Regionen. Wird von Allocatoren verwaltet. Der VRAM-Manager lädt Segmente hinein.

**Attach**
- Synonyme: Anhängen
- Bedeutung: Der Manager hängt seinen Index erst bei Bedarf an (liefert true bei Annahme, false bei bereits angehängtem Index).
- Beziehungen: Teil von Manager und lazy aktiviertem Index. Voraussetzung für Zugriffe.

## B

**Backpressure**
- Synonyme: Rückdruck
- Bedeutung: Natürliches Signal, dass Ressourcen ausgelastet sind. Blockiert ein Worker, entsteht es ohne Sonderbehandlung. Eine Idle-Notification meldet das Gegenteil, nämlich Unterversorgung.
- Beziehungen: gehört zum Processor. Steuert die Expansion zusammen mit der Idle-Notification.

**Batch**
- Synonyme: Stapel
- Bedeutung: Zusammenfassung mehrerer Operationen oder Sequenzen für eine Ressource. Bei der Batch-Bildung sind es Tokenfolgen gleicher gepaddeter Länge samt Attention-Mask. Bei Lanes beginnt er mit der Ausgangsoperation und wächst vorwärts durch die Queue.
- Beziehungen: wird von Lanes gebildet. Der Encoder arbeitet nach Token-Budget. Auf der GPU ist er notwendig für effizientes Streaming von Segmenten und Kerneln.

**Batch-Bildung (Eingabesequenzen)**
- Synonyme: Packing, Length-Bucketing (Teilaspekt)
- Bedeutung: Teilt Tokenfolgen nach Länge in Batches mit Token-Budget. Dabei wird auf Bucket-Größen 32, 64, 128, 256 und 512 gepaddet. Die Serialisierung ist 2 × Zeilen × Sequenzlänge als u32 Little-Endian.
- Beziehungen: folgt auf den Tokenizer, speist den Encoder, nutzt Padding und Pad-Token.

**Budget (Speicher)**
- Synonyme: Speicherbudget
- Bedeutung: Freier Speicher, Gesamtspeicher und Generationsreserve. Nutzbar ist der freie Speicher abzüglich Reserve, nie negativ. Der Default-Headroom beträgt 512 MiB.
- Beziehungen: Grundlage für Fit-Check und Segmentgröße. Wird vom Memory-Backend abgefragt.

## C

**Capability**
- Synonyme: Fähigkeit
- Bedeutung: Wert auf einer Achse (z. B. FP16, INT8-DP4A, Fusion on). Beschreibt, was ein Device oder eine Ressource leisten kann.
- Beziehungen: bildet Capability-Sets. Wird von Requirements gefordert und vom Planner zur Queue-Zuordnung genutzt. Wird durch Capability-Erkennung ermittelt und auf vorhandene Kernel reduziert.

**Capability-Achse / Capability-Set**
- Synonyme: Achse, Wertemenge
- Bedeutung: Eine Achse ist frei wählbar und erweiterbar (Operanden, Akkumulator, Scoring, Fusion, Batch-Größe, Tile-Größe, Thread-Anzahl, Dichte). Ein Set ordnet jeder Achse eine nie leere Wertemenge zu. Es erfüllt ein Requirement, wenn alle geforderten Werte vorhanden sind.
- Beziehungen: ersetzt Profilnamen und Profil-Enum. Ist Grundlage der Kalibrierung. Jede Subengine liefert ein Set.

**Centroid**
- Synonyme: Centroid-Umgebung, Zellen
- Bedeutung: Scope-Art und approximativer Prefilter. Die Query wird gegen die Summary-Struktur gerechnet, dann werden die Mitglieder der nächsten Zellen als Kandidaten genommen.
- Beziehungen: braucht die Summary-Struktur des Index und einen Cell-Selector. Wird vom Manager aufgelöst. Typisches Addon-Beispiel.

**Change-Stamp**
- Synonyme: Stamp, Stempel, Veränderungsmarker
- Bedeutung: Opaker, pro Index eindeutiger Marker jeder Schreib- und Löschoperation. Er wird nur auf Gleichheit verglichen. Auch Entfernen verbraucht einen Stamp.
- Beziehungen: bildet mit der ID die Identität eines Entries. Wird von Reconciler und Manager zum Abgleich genutzt. Max-Stamp steht im Index-Header. Löst die Change-Notification aus.

**Checksum**
- Synonyme: Prüfsumme
- Bedeutung: 64-Bit-Wert aus Vektorlänge, Vektorbytes, Dateinamen, Zeichenanfang und -ende eines Entries. Wird beim Speichern berechnet und bei der Verifikation verglichen.
- Beziehungen: Spalte der Index-Datei. Teil von Upsert und Verifikation.

**Claim**
- Synonyme: Übernahme
- Bedeutung: Atomarer Übergang enqueued → claimed. Der Claimer erhält die Payload per Ownership-Transfer, genau einmal. Er blockiert nie.
- Beziehungen: schließt sich mit Cancel gegenseitig aus. Führt zu Completion. Erfolgt nach dem Pull-Filter in der Iteration.

**Collector**
- Synonyme: Collector-Funktion, Ergebnissammler
- Bedeutung: Funktion des Expanders, die den Output der letzten Operation per take als Task-Ergebnis entnimmt. Beim Laden gleichnamig: Zähler für aggregierte Requests.
- Beziehungen: gehört zum Planner/Expander. Die Ergebnisaufbereitung hängt von der Task-Art ab.

**Communication-Slot**
- Synonyme: Gap-Abarbeitung zwischen Batches
- Bedeutung: Feste Abfolge in einer Gap: Kernel-Writes lesen, Ergebnisse auf Entries auflösen, dann Updates einspielen.
- Beziehungen: wird vom Manager ausgeführt. Läuft gegen die Mapping-Epoche des Batches. Nutzt Address-Map, Update-Queue und Residency-Verzeichnis.

**Completion**
- Synonyme: Fertigstellung, Abschluss
- Bedeutung: Atomarer Übergang claimed → completed, nur durch den Claimer und nur einmal. Das Ergebnis wird vor dem Zustand sichtbar. Bei Operationen verringert die Completion den Zähler offener Operationen des Tasks.
- Beziehungen: weckt Wartende. Auf der GPU wird sie über Callback, Event-Query und atomaren Swap erkannt.

**Continuation-Handler**
- Synonyme: Folgeknoten-Erzeuger
- Bedeutung: Nachträglich anfügbarer Handler eines Operationsknotens. Er erzeugt aus Output und Task-Kontext null oder mehr Folgeknoten. Ohne Folgeknoten wird der Output zum Task-Ergebnis.
- Beziehungen: gehört zum Operationsknoten. Verwendet den Task-Kontext. Folgeoperationen werden vor dem Abschluss der erzeugenden Operation registriert.

**Cosine Similarity**
- Synonyme: Cosinus-Ähnlichkeit, Dot-Product (bei Einheitsvektoren)
- Bedeutung: Default-Scoring-Verfahren, im dichten Modus das Skalarprodukt L2-normalisierter Vektoren. Der exakte Modus teilt durch das Normprodukt.
- Beziehungen: setzt L2-Normalisierung des Encoders voraus. Wird vom Scorer berechnet. Gegenstück zu MaxSim.

**CPU-Subengine**
- Synonyme: CPU-Seite
- Bedeutung: Subengine mit Cores als Ressourcen. Sie hält Tokenizer, Normalizer, SIMD-Scorer, Encoder über das CPU-Backend und mmap-Indizes. Default-Datentyp ist FP32.
- Beziehungen: Gegenstück zur GPU-Subengine. Besitzt eigene Lanes (Threads des Processors), Kernel-Register, Index und Manager.

## D

**Device**
- Synonyme: Gerät, Compute-Substrat
- Bedeutung: CPU (potenziell mehrere Sockets) oder GPU (Device-Ordinal). Erweiterbar.
- Beziehungen: bietet Ressourcen (Cores bzw. Compute Units) und Lanes. Erfüllt Requirements über Art und Capabilities.

**Doorbell-Synchronisation**
- Synonyme: Batch-Ack-Protokoll
- Bedeutung: Die GPU publiziert die Zahl abgeschlossener Batches mit Release, der Host quittiert (Ack) nach dem Verändern. Dazwischen liegt eine Gap, in der der Host verändern darf.
- Beziehungen: Grundlage der Mutationsdisziplin. Hängt mit Gap, Table-Swap und Communication-Slot zusammen.

## E

**Echo-Suppression**
- Synonyme: Echo-Vermeidung
- Bedeutung: Der Manager trägt den Stamp aus dem Rückgabewert seines eigenen Upserts als bekannt ein, damit eigene Writes nicht als fremde Änderung gelten.
- Beziehungen: Teil des zustandsbasierten Abgleichs. Nutzt den Change-Stamp.

**Embedding**
- Synonyme: Einbettung, Vektor
- Bedeutung: Float-Vektor, in den ein Text umgewandelt wird (hier Dimension 384 bei multilingual-e5-small).
- Beziehungen: Ergebnis des Embedding-Tasks. Wird vom Encoder erzeugt und im Index gespeichert. Nur bei gleicher Modellbindung vergleichbar.

**Embedding-Task**
- Synonyme: Indexing-Task (bei Zielindex)
- Bedeutung: Task mit Text oder Textliste. Parameter sind Modell, Modus, Precision und optionaler Ziel-Index. Ergebnis: ein Vektor je Text, oder Entry-Referenzen beim direkten Schreiben.
- Beziehungen: Expansion: Tokenisierung → Embedding → Pooling → L2 → Index-Schreiben. Gegenstück zum Similarity-Task.

**Encoder**
- Synonyme: Hauptmodell mit Pipeline, geladener Encoder
- Bedeutung: Ressource aus Hauptmodell (BERT), optionaler Projection-Layer, Pooling, Normalisierung und Storage-Datentyp. Verarbeitet Token-IDs plus Mask asynchron auf einer Lane in eine Output-Region.
- Beziehungen: wird vom Modellregister über Factories konstruiert. Gehört zur Subengine. Nutzt die Candle-Library (nur über Adapter).

**Engine**
- Synonyme: Resident, Fassade (Facade)
- Bedeutung: Dauerhaft laufende Gesamtkomponente für Embedding und Similarity, per Builder konfiguriert und per Start-Aufruf aktiv. Ressourcen entstehen erst beim ersten Task.
- Beziehungen: enthält Processor, Planner und Subengines, verwaltet Indizes und Tasks, bietet geordnetes Shutdown.

**Entry**
- Synonyme: Eintrag (Index)
- Bedeutung: Index-Einheit aus 64-Bit-ID, Change-Stamp und Payload (Vektor, Zieldatei, Zeichenanfang und -ende). Knoten und Zellen haben eigene ID-Typen.
- Beziehungen: Identität ist das Paar aus ID und Stamp. Wird im Manager einem Segment und Slot zugeordnet. Ist Gegenstand von Scopes und Entry-Mengen.

**Entry-Menge**
- Synonyme: Kandidatenmenge (im Scoring-Kontext)
- Bedeutung: Kompakt als Ranges oder Bitmap gespeicherte Menge von 64-Bit-IDs mit Union, Intersection, Hash und Iteration.
- Beziehungen: Ergebnis der Scope-Auflösung im Manager. Basis für Residency-Planung und Aggregation.

**Eviction**
- Synonyme: Verdrängung, Entladen von Mirrors
- Bedeutung: Explizites Entfernen von Mirrors in VRAM/RAM nach Idle-Timeout, Füllstandsgrenzen mit Hysterese oder Platzbedarf. Gründe: Idle, Pressure, Space-Need. Die Priorität folgt GreedyDual-Size-Frequency.
- Beziehungen: Summaries und gepinnte Entries sind nie Victim. Läuft über Segment-Tracking im Manager. Im VRAM gelten längere Fristen (TTL) als im RAM. Das Gegenstück ist der Unload ohne Deaktivierung.

**Event**
- Synonyme: Completion-Event, Completion-Marker
- Bedeutung: Opaker Marker für alle vor seiner Aufzeichnung auf einer Lane eingereichte Arbeit. Nicht blockierend abfragbar und blockierend abwartbar. Ein CPU-Event bleibt dauerhaft abgeschlossen.
- Beziehungen: gehört zur Lane. Steuert Deferred Free, Staging-Ring und Commit von Slots.

**Expander**
- Synonyme: Task-Zerleger
- Bedeutung: Reine, zustandslose Funktion, die einen Task genau einmal in eine geordnete Kette von Operationen zerlegt und dabei Task-Parameter in Operationskonfiguration übersetzt.
- Beziehungen: Teil des Planners. Nutzt Pipeline-Konfiguration pro Modell, Capabilities und Idle-Notification. Erzeugt Operationen.

## G

**Gap (Mutationsfenster)**
- Synonyme: Lücke
- Bedeutung: Zeitfenster zwischen Abschluss eines Batches und dessen Ack, in dem der Manager Mappings, Tabellen und Segmente verändern darf. Er verändert nie unter dem laufenden Kernel.
- Beziehungen: Teil der Doorbell-Synchronisation. Hier läuft der Communication-Slot. Nicht zu verwechseln mit dem Score-Gap.

**Gap (Score)**
- Synonyme: Abstand erster/zweiter Treffer
- Bedeutung: Kennzahl des Similarity-Reports. Ein kleiner Abstand zeigt eine unsichere Top-Reihenfolge an. Mindestens zwei Treffer nötig.
- Beziehungen: ergänzt Spread, z-Score, Entropie und Knee-Point im Report.

**GPU-Subengine**
- Synonyme: CUDA-Subengine
- Bedeutung: Subengine mit Compute Units und Lanes sowie VRAM-Manager, Streams, Staging und Segmenttabelle. Default-Datentyp ist FP16. Die Anbindung der GPU-Library ist ihre einzige Stelle.
- Beziehungen: Gegenstück zur CPU-Subengine. Fällt bei fehlender GPU auf die CPU zurück. Nutzt Kernel-Register (NVRTC).

## I

**Idle-Notification**
- Synonyme: Unterversorgungshinweis
- Bedeutung: Wird bei einer Iteration ohne Treffer bei freier Ressource erzeugt, mit Ressourcenart und Parametern. Liegt als Set je Subengine-Art vor und verfällt bei Treffer oder belegter Ressource.
- Beziehungen: löst Expansion neuer Tasks aus. Wird vom Processor verwaltet. Gegenstück zu Backpressure.

**Index**
- Synonyme: Vektorindex, Source of Truth
- Bedeutung: Persistente Struktur von Entries mit Stamps, im RAM identisch mit der persistenten Darstellung. Das VRAM hält nur einen verwerfbaren Mirror. Er kennt weder Subengine noch Kernel und löst keine Scopes auf. Dateigestützt liegt er als Parquet mit Arrow-Schema vor.
- Beziehungen: wird vom Manager verwaltet. Der Index-Header bindet ihn an ein Modell. Wird lazy geladen. Sendet Change-Notifications.

**Index-Header**
- Synonyme: Kopf (Index), Footer-Metadaten
- Bedeutung: Key-Value-Paare im Datei-Footer: Schema-Version, Max-Stamp, Hash-Algorithmus und -Version, Model-ID und -Revision, Dimension, Pooling, Normalisierung, Präfixe, Datentypen, maximale Sequenzlänge und Tokenizer-Fingerprint. Max-Stamp und Compute-Datentyp sind informativ.
- Beziehungen: einzige harte Bindung zwischen Index und Modell (Modellbindung). Abgeleitet aus der Modellbeschreibung.

## K

**Kalibrierung**
- Synonyme: Benchmark-Kalibrierung, Kalibrierungslauf
- Bedeutung: Misst Kombinationen aus Capability-Werten gegen eine FP32-CPU-Referenz und bildet die Pareto-Front über Zeit und Abweichung. Sie läuft im Hintergrund auf Low-Priority-Threads und blockiert den Nutzer nie.
- Beziehungen: erzeugt Profile. Ergebnisse werden pro Modell und Subengine mit Fingerprint persistiert. Nutzt Mess-Interface und Capability-Sets.

**Kernel**
- Synonyme: Rechenkern, GPU-Kernel
- Bedeutung: Rechenroutine auf CPU oder GPU, die nur Segmente, Adressen und Offsets kennt, keine Entry-Identität und keine Traversierung. GPU-Kernel sind dichtes Dot-Product und fused MaxSim, Launch mit 128 Threads je Block.
- Beziehungen: wird über das Kernel-Register geladen. Wird von Lane gestartet. Der Manager baut ihm Layouts und Tabellen.

**Kernel-Register**
- Synonyme: Kernel-Registry
- Bedeutung: Liefert zu Name, Datentyp, Architekturstufe und Variante die geladene Funktion. Quellen: NVRTC-Laufzeitkompilierung, vorkompilierter Fallback, Kernel der externen Library. Die Kompilierung erfolgt lazy und nie doppelt.
- Beziehungen: gehört zur jeweiligen Subengine. Wählt die Zielarchitektur aus der Compute Capability (Referenz: SM 7.5).

**Knee-Point**
- Synonyme: Knick-Punkt
- Bedeutung: Treffer der sortierten Score-Kurve mit dem größten Abstand unterhalb der Verbindungslinie von erstem zu letztem Wert. Mindestens drei Treffer nötig.
- Beziehungen: Kennzahl des Similarity-Reports.

## L

**Lane**
- Synonyme: Execution-Context, Ausführungskontext
- Bedeutung: Pull-fähige Ausführungseinheit einer Ressource. Auf der GPU besteht sie aus Stream, Priorität, Events, Parameter-Block und Workspace (z. B. Query, Insert/Update, H2D/D2H). Auf der CPU ist sie ein Thread des Processors.
- Beziehungen: zieht Operationen aus Queues. Implementiert Batching. Übergaben zwischen Lanes laufen nur über die Queue.

**Late Interaction**
- Synonyme: Multi-Vector-Schema, ColBERT-Art
- Bedeutung: Ein Vektor je Token statt je Sequenz. Marker, Query-Länge, Mask-Token und Projection-Head kommen aus der Checkpoint-Konfiguration. Im Query-Modus wird mit dem Mask-Token gepaddet (Query Augmentation).
- Beziehungen: Scoring über MaxSim. Pooling „none“. Nutzt die Projection-Layer. Tokenizer-Schema-Variante.

**Lazy**
- Synonyme: bei Bedarf, Lazy Loading, Lazy-Initialisierung
- Bedeutung: Ressourcen (Modelle, Kernel, Indizes, Tokenizer) werden erst durch die Iteration initialisiert, nie durch Enqueue oder Start. Nach Ablauf einer Frist werden sie entladen, aber nicht deaktiviert.
- Beziehungen: Gegenteil von Eager. Zustände uninitialized → loading → ready (backoff nach Fehler). Gehört zu den Leitprinzipien.

**Length-Bucketing**
- Synonyme: Längenklassen
- Bedeutung: Auffüllen auf die kleinste Bucket-Größe (32/64/128/256/512), die mindestens der trunkierten Länge entspricht.
- Beziehungen: Teil der Batch-Bildung. Bestimmt zusammen mit Token-Budget die Batch-Größe.

## M

**Manager**
- Synonyme: Index-Manager, VRAM-Manager (GPU-Variante), CPU-Manager
- Bedeutung: Komponente einer Subengine, die Entry-Segment/Slot-Zuordnung, Residency, Update-Queue, Upload, Eviction und Rückabbildung von Adressen auf Entries kennt. Er löst Scopes in Entry-Mengen auf. Die CPU-Variante kümmert sich um Page-Cache-Hinweise.
- Beziehungen: steht zwischen Index und Kernel. Nutzt Reconciler, Residency-Verzeichnis, Manifest und Table-Swap.

**Manifest**
- Synonyme: Segment-Snapshot
- Bedeutung: Snapshot der Segmente mit fortlaufender Generation. Für Reader lock-free verfügbar und atomar ersetzbar.
- Beziehungen: Reader sichern es per Lease. Die Segmenttabelle für Kernel leitet sich daraus ab. Treffer älterer Generationen werden überführt.

**Mapping-Epoche**
- Synonyme: Epoche
- Bedeutung: Versionszähler von Segmenttabelle und Address-Map. Die publizierte Epoche und die je Reader zuletzt gemeldete bestimmen, wann abgelöste Versionen frei werden.
- Beziehungen: Teil des Epoch-Tracking und Table-Swap. Fehler: „stale Mapping“.

**Materialization-Flag**
- Synonyme: Materialisierungskennzeichen
- Bedeutung: Legt fest, ob ein Operationsergebnis ins RAM zurückgeholt wird oder als Referenz im VRAM bleibt. Es bleibt nur im VRAM, wenn diese und die folgende Operation beide ausdrücklich auf der GPU laufen.
- Beziehungen: wird vom Planner festgelegt, von der Operation ausgeführt. Gehört zum Operationsknoten.

**MaxSim**
- Synonyme: Max-Similarity
- Bedeutung: Late-Interaction-Scoring. Je Query-Token wird das größte Dot-Product über Dokument-Tokens genommen und summiert. Leere Dokumente ergeben Mask-Wert × Query-Tokens.
- Beziehungen: als Tile-Kette oder Fused Kernel realisiert. Aggregiert je Entry über Stride oder Tabelle, ohne Entry-Identität.

**Modell / Modellbeschreibung**
- Synonyme: Descriptor, Modell-Descriptor
- Bedeutung: Reine ressourcenlose Beschreibung einer Pipeline: Model-ID, Architektur-ID, Dimension, Pooling, Normalisierung, Präfixe, Datentypen, maximale Sequenzlänge, Pfade, optionale Projection-Layer. Default: multilingual-e5-small (bert, 384, Mean-Pooling, L2).
- Beziehungen: Grundlage von Tokenizer, Encoder und Index-Header. Wird validiert. Ein Register ordnet Architektur-IDs Factories zu.

**Modellregister**
- Synonyme: Register der Architekturen
- Bedeutung: Bildet Architektur-IDs auf Encoder-Factories ab. Neue Modelle bedeuten einen neuen Eintrag. „bert“ erfordert das Feature-Flag.
- Beziehungen: konstruiert Encoder aus Modellbeschreibung und Device-Klasse. Fehler: „nicht unterstützte Architektur“.

## N

**Normalizer**
- Synonyme: Text-Normalizer, Normalisierung (Tokenizer-seitig)
- Bedeutung: Teil des Tokenizers (Unicode-Normalisierung, Lowercasing) und nur auf der CPU. Ein optionaler externer Text-Normalizer ergänzt Transformationen. Er ist nicht zu verwechseln mit der L2-Normalisierung der Vektoren.
- Beziehungen: läuft vor Präfix/Marker und Tokenisierung. Ressource der CPU-Subengine.

**Notification**
- Synonyme: Benachrichtigung, Signal, Wakeup
- Bedeutung: Weckmechanismus für Wartende, Worker und Subengines. Der Subengine-Loop hat genau eine Wartebedingung. Auch das Level-Signal der Index-Änderung heißt Change-Notification.
- Beziehungen: wird bei Append, Ergebnis, Initialisierung und Eviction-Timer ausgelöst. Siehe Idle-Notification, Queue und Worker.

## O

**Operation**
- Synonyme: Operationsschritt, Kettenglied
- Bedeutung: Berechnungseinheit eines Tasks mit expliziten Input/Output-Regionen. Arten: Tokenisierung, Embedding, Pooling, L2-Normalisierung, Similarity-Berechnung, Top-k-Selektion, Index-Schreiben, Inferenz, Übertragung Device→RAM. Sie gehört zu genau einem Task.
- Beziehungen: vom Expander erzeugt. Parameter und Resource-Requirements dienen als Filter-Key. Sitzt in einem Operationsknoten. Siehe Descriptor.

**Operationsknoten**
- Synonyme: Knoten (Operation), Node
- Bedeutung: Bündelt Operation, Input, einmal setzbaren Output, Materialization-Flag, Task und optional Continuation-Handler.
- Beziehungen: wird in Operations-Queues enqueued. Verweist auf den Task-Kontext.

## P

**Padding**
- Synonyme: Auffüllung
- Bedeutung: Auffüllen von Sequenzen mit Pad-Token (oder Mask-Token bei Query Augmentation) auf gleiche Länge. Das automatische Tokenizer-Padding ist abgeschaltet, Padding ist Aufgabe der Batch-Bildung.
- Beziehungen: durch Attention-Mask ausgeblendet. Bei Slots wird Padding mit endlichem Mask-Wert (−10 000) statt null codiert.

**Pareto-Front**
- Synonyme: Front
- Bedeutung: Menge nicht dominierter Kombinationen, die Zeit und Abweichung minimieren. Aufsteigend nach Zeit mit streng fallender Abweichung.
- Beziehungen: Ergebnis des Kalibrierungslaufs. Basis der Profilableitung (Positionen und Merge-Threshold).

**Pin**
- Synonyme: Device-Handle (Bezeichnung), RAII-Pin
- Bedeutung: Zugriff auf eine Device-Region, der per Pin-Count Eviction verhindert, solange er besteht. Er endet automatisch (RAII).
- Beziehungen: Teil von Input/Output einer Operation. Nicht zu verwechseln mit Pinned Memory und gepinnten Entries.

**Pinned Memory**
- Synonyme: Seitengesperrter Speicher
- Bedeutung: Host-Speicher, der für schnelle Device-Transfers fixiert ist.
- Beziehungen: Grundlage von Staging-Memory und Mapped Host-Memory.

**Planner**
- Synonyme: Planung
- Bedeutung: Nimmt Tasks entgegen, lässt sie vom Expander zerlegen und ordnet Operationen anhand kompatibler Capabilities den Queues (GPU, CPU, gemeinsam) zu. Er aktiviert Subengines blockierend, aber ohne Laden. Fehler: „fehlende Planungs-Capability“.
- Beziehungen: Teil der Engine, vom Processor aufgerufen. Bevorzugt die gemeinsame Queue. Legt das Materialization-Flag fest.

**Pooling**
- Synonyme: Mean-Pooling (masked mean), per Token (none)
- Bedeutung: Reduktion der Token-Hidden-States zu einem Vektor je Sequenz (Mittel über unmaskierte Token) oder gar nicht („none“, ein Vektor je Token).
- Beziehungen: Operation der Embedding-Kette. Teil des Index-Headers. „None“ gehört zu Late Interaction.

**Precision**
- Synonyme: Precision-Parameter, Genauigkeitswunsch
- Bedeutung: Wunsch zwischen 0 (schnellste) und 1 (präziseste Kombination), keine Zusicherung. Zwischen CPU und GPU gibt es keinen Rounding-Vertrag.
- Beziehungen: wählt ein Profil. Wird auch als Datentyp-Begriff benutzt (FP32, FP16, BF16).

**Processor**
- Synonyme: Orchestrator
- Bedeutung: Besitzt Task-Queue, drei Operations-Queues, Worker-Pool, Notification und Idle-Notifications. Ruft den Planner und betreibt den Worker-Loop.
- Beziehungen: Teil der Engine. Mit den Subengines über Queues und Notifications verbunden.

**Profil**
- Synonyme: Precision-Profil
- Bedeutung: Unbenannte Position zwischen 0 und 1 mit je einem Wert pro Achse, abgeleitet aus der Pareto-Front. Die Anzahl folgt allein aus der Streuung (Merge-Threshold, Default 0,1).
- Beziehungen: Ergebnis der Kalibrierung. Die Auswahl erfolgt über den Precision-Wunsch. Ohne aktives Profil gilt „kein aktives Profil“ (Latch).

**Projection-Layer**
- Synonyme: Projection-Head
- Bedeutung: Optionale lineare Abbildung jedes Token-Vektors von Input- auf Output-Dimension (z. B. 768 → 128), mit Bias, aus safetensors geladen.
- Beziehungen: Teil des Encoders, insbesondere für Late Interaction. Liegt in der Modellbeschreibung.

## Q

**Queue**
- Synonyme: Warteschlange
- Bedeutung: Lock-free MPMC-Struktur als einfach verkettete Liste mit Sentinel. Vier Instanzen: Task-Queue und Operations-Queues für CPU, GPU und gemeinsam. Sie kennt Priorität und Ressource nicht. Zustände: enqueued, claimed, completed, cancelled.
- Beziehungen: Subengines ziehen daraus (Pull-Prinzip). Der Knoten ist zugleich Handle des Aufrufers. Freigabe über Reference Counting.

## R

**Reconciler**
- Synonyme: Abgleicher, State-based Reconciliation
- Bedeutung: Je Subscriber (GPU-Manager, Persistenz-Timer, CPU-Manager) eine eigene Sicht. Er vergleicht residente Entries nach dem Signal mit dem Index. Fehlende gelten als entfernt, abweichender Stamp als Reload, gleicher Stamp als aktuell.
- Beziehungen: arbeitet mit dem Change-Stamp. Der Index gewinnt bei Abweichung. Siehe Echo-Suppression.

**Region**
- Synonyme: Speicherbereich-Handle
- Bedeutung: Pointerfreies Handle aus Arena-ID, Offset, Länge und Generation. Die Generation entwertet es nach Eviction oder Neuladen. Nur das Memory-Backend löst es in Device-Adressen auf.
- Beziehungen: mit Layout bildet es eine Tensor-Referenz. Gehört zu Arena und Allocator. Fehler: „stale Region“.

**Reranking**
- Synonyme: Neubewertung
- Bedeutung: Keine eigene Task-Art, sondern eine zweite Similarity mit anderem Scoring-Verfahren (z. B. Cross-Encoder) auf verengtem Scope aus einem früheren Ergebnis.
- Beziehungen: wird von Anwendung oder Addon gesteuert. Nutzt den Cross-Encoder-Scorer.

**Requirements (Resource-Requirements)**
- Synonyme: Anforderungen
- Bedeutung: Ressourcenart, Execution-Umgebung (CPU-only, GPU-only, any), optional Modell, Index, geforderte Capabilities und Placement-Hint. Ein Device erfüllt sie, wenn seine Art akzeptiert wird und seine Capabilities alle abdecken. Der Placement-Hint zählt dabei nicht.
- Beziehungen: folgen aus den Operationsparametern. Dienen als Pull-Filter und Key. Bestimmen den Ressourceneintrag in der Iteration.

**Ressource**
- Synonyme: Ressourcenart (Tokenizer, Normalizer, Encoder, Scorer, Index, Cross-Encoder)
- Bedeutung: Von einer Subengine besessene, ladbare Einheit. Unterschieden wird verfügbar (Capability registriert) und geladen (initialisiert im Speicher).
- Beziehungen: durchläuft uninitialized, loading, ready und backoff. Wird bei Nichtgebrauch entladen (Unload/Eviction).

**Residency**
- Synonyme: Ladezustand, Residency-Verzeichnis
- Bedeutung: Welche Entries oder Segmente im VRAM/RAM resident sind. Das Verzeichnis führt je Entry Slot und bekannten Stamp.
- Beziehungen: vom Manager gepflegt. Ladestrategie: lazy und scope-getrieben mit Plänen (ganzes Segment, Gather, Zero-Copy).

**Root-Pointer**
- Synonyme: Tabellen-Root
- Bedeutung: Atomarer 8-Byte-Zeiger auf die aktuelle Segmenttabelle. Der Host schreibt mit Release, das Device liest mit Acquire. Vor der ersten Tabelle trägt er einen Sentinel.
- Beziehungen: wird beim Table-Swap getauscht. Hierarchie: Root → Segmenttabelle → Segment.

## S

**Scope**
- Synonyme: Suchbereich
- Bedeutung: Einschränkung der Kandidaten. Arten: all, range, set, subtree, prefix, centroid. Neue Arten sind ergänzbar. Default ist der gesamte Index.
- Beziehungen: wird vom Manager (nicht vom Index) in eine Entry-Menge aufgelöst. Arten außer all und set brauchen eine Zugriffsmethode samt Resolver, sonst „Scope nicht unterstützt“.

**Scorer**
- Synonyme: Scoring-Implementierung
- Bedeutung: Ressource, die Scores berechnet (Cosine, MaxSim, Cross-Encoder). Die CPU-Variante ist selbst implementiert mit SIMD.
- Beziehungen: vom Modellregister einer Scoring-Art zugeordnet. Berechnet Scores für Top-k-Selektion. Referenz-Scorer dienen dem Vergleich.

**Score**
- Synonyme: Bewertung, Wertung
- Bedeutung: Numerischer Ähnlichkeitswert eines Treffers. Höher ist besser, bei Gleichstand gewinnt die niedrigere Entry-ID.
- Beziehungen: Bestandteil von Treffern und Similarity-Report. Entsteht im Scorer.

**Segment**
- Synonyme: Index-Segment, Segment-View
- Bedeutung: Region mit Slots fester Größe (Slot-Klassen 32/64/128 Token), Dimension, Datentyp und Generation. Segmente wachsen nur durch Append, Compaction durch Rebuild.
- Beziehungen: Teil von Manifest und Segmenttabelle. Gehört zur GPU-Index-Struktur. Wird publiziert, geevictet und per Lease gehalten.

**Segmenttabelle**
- Synonyme: Tabelle für Kernel
- Bedeutung: Binäre Tabelle mit 24 Byte je Segment (Generation, Startadresse, Slot-Anzahl, Slot-Abstand). Publizierte Versionen sind immutable (Copy-on-Write mit höherer Epoche).
- Beziehungen: wird per Root-Pointer-Swap veröffentlicht. Kernel lesen sie, der Manager baut sie.

**Similarity-Report**
- Synonyme: Report, Faktenbericht
- Bedeutung: Liefert rohe Scores und Fakten ohne Entscheidung oder Threshold: Kandidaten-, verworfene und stale Anzahl, Rangliste, Kennzahlen (Mittelwert, Standardabweichung, Gap, Spread, z-Score, Softmax-Verteilung, Entropie, Knee-Point).
- Beziehungen: Ergebnisart des Similarity-Tasks. Eine äußere Schicht entscheidet damit über Folge-Tasks. Enthält Task-ID und Suchparameter.

**Similarity-Task**
- Synonyme: Ähnlichkeitssuche
- Bedeutung: Task mit Query-Vektoren gegen einen Index. Parameter: Index-ID, Top-k (Default 10), Scope, Scoring-Verfahren (Default Cosine), Precision. Expansion: Similarity-Berechnung → Top-k-Selektion.
- Beziehungen: Textanfrage = Embedding-Task (Query) + Similarity-Task. Nutzt den Scorer. Das Ergebnis ist eine Trefferliste oder ein Report.

**Slot**
- Synonyme: Segment-Slot
- Bedeutung: Feste Speichereinheit in einem Segment (Token-Obergrenze × Dimension × Datentypgröße) mit Tokenzahl und Valid-Bit (Tombstone). Nicht zu verwechseln mit dem Staging-Slot oder Communication-Slot.
- Beziehungen: wird per Commit sichtbar. Ist einer Entry-Referenz zugeordnet. Der Kernel braucht dadurch keinen Lookup.

**stale**
- Synonyme: veraltet
- Bedeutung: Ergebnis oder Region, die nicht mehr dem aktuellen Stand entspricht. Stale Ergebnisse sind kein Fehler, sondern ein Diagnoseflag am Treffer. „Stale Region“ und „stale Generation“ sind Fehlerarten ungültiger Handles.
- Beziehungen: entsteht, weil auf Datenpfaden keine Locks gelten. Wird im Communication-Slot markiert. Wird im Report gezählt.

**Staging-Ring / Staging-Memory**
- Synonyme: Bereitstellungsspeicher
- Bedeutung: Ring aus Pinned Host-Buffern fester Slot-Anzahl (frei, belegt, in Übertragung mit Event) für Kopien zwischen RAM und VRAM. Bei der CPU entfällt er.
- Beziehungen: wird vom Gather-Pfad genutzt. Wiederverwendung erst nach Event. Fehler: „Staging erschöpft“.

**Subengine**
- Synonyme: Compute-Substrat-Engine (CPU- bzw. GPU-Subengine)
- Bedeutung: Verwaltet Ressourcen, Speicher, Kernel und Lanes je Hardwareart und zieht Arbeit selbstorganisiert per Pull aus den Queues. Sie liefert ein Capability-Set und einen geglätteten Durchsatz.
- Beziehungen: besitzt Tokenizer, Encoder, Scorer und Index-Segmente. Wird vom Planner aktiviert. Kalibriert ihre Hardware.

**Summary-Struktur**
- Synonyme: Zusammenfassungsstruktur
- Bedeutung: Kleine Index-Struktur (z. B. Centroids), die zuerst geladen und am längsten gehalten wird.
- Beziehungen: Grundlage der Centroid-Auflösung. Wird nie evicted (Tier).

## T

**Table-Swap**
- Synonyme: Tabellenwechsel, Copy-on-Write-Publication
- Bedeutung: Veröffentlichung einer neuen Segmenttabelle: Inhalt sichtbar schreiben, Root tauschen, Epoche vorschieben, alte Region zurückstellen (Retire). Publiziert wird nur Epoche aktuell + 1.
- Beziehungen: findet in der Gap statt. Nutzt Root-Pointer, Epoch-Tracking, Reclaim.

**Task**
- Synonyme: Auftrag
- Bedeutung: Kapselt Daten, Parameter, Ergebnis, Zustand und Operationen. Arten: Embedding und Similarity. Das Ergebnis wird genau einmal gesetzt (Output oder Fehler). Priorität, Modell und Modus sind Metadaten.
- Beziehungen: wird vom Expander in Operationen zerlegt. Ergebnisarten: Embeddings, Entry-Referenzen, Treffer, Report, Scores.

**Task-Handle**
- Synonyme: Aufrufer-Handle
- Bedeutung: Pollbarer und abwartbarer Zugang zum Task-Zustand mit Timeout als Deadline des Aufrufers („timeout“). Das Droppen eines unfertigen Handles bricht den Task ab, sofern er nicht übernommen wurde.
- Beziehungen: identisch mit dem Zustandsobjekt des Queue-Knotens. Zustand „cancelled“ wird lokal abgeleitet.

**Task-Kontext**
- Synonyme: Gemeinsamer Task-Zustand
- Bedeutung: Geteilter Kontext je Task mit ID, Priorität, Zähler offener Operationen, Cancel-Flag und erstem Fehler. Der erste Fehler setzt den Task auf „cancelled“.
- Beziehungen: von allen Operationen des Tasks geteilt. Der Zähler bestimmt den Abschluss.

**Tokenizer**
- Synonyme: Textzerleger
- Bedeutung: CPU-Ressource, die Text in Token-IDs (u32) umwandelt, mit Schema (Präfix oder Multi-Vector), Modi Query/Passage, Pad-Token, Trunkierung und optionalem Normalizer.
- Beziehungen: wird aus der Modellkonfiguration geladen. Ein Fingerprint steht im Index-Header. Speist die Batch-Bildung.

**Top-k**
- Synonyme: Trefferanzahl, maximale Trefferzahl
- Bedeutung: Maximale Zahl zurückgegebener Treffer (Default 10). Die Selektion nimmt den höheren Score, bei Gleichheit den niedrigeren Offset/ID. Ein Wert von null wird abgelehnt.
- Beziehungen: Parameter des Similarity-Tasks. Zweite Operation der Similarity-Kette. Lokale Reduktion je Workspace, Merge auf der CPU.

**Trunkierung**
- Synonyme: Kürzen
- Bedeutung: Kürzen auf die maximale Sequenzlänge (Special-Token bleibt erhalten) bzw. Query- und Dokumentlänge im Multi-Vector-Schema.
- Beziehungen: Teil von Tokenizer und Batch-Bildung. Diese Trunkierungen gelten zusätzlich zur allgemeinen.

## U

**Unload**
- Synonyme: Entladen, Ressourcen-Eviction nach Frist
- Bedeutung: Nach Ablauf einer Frist ohne Nutzung werden Ressourcen entladen, aber nicht deaktiviert. Im VRAM sind die Fristen deutlich länger (VRAM 600 s, RAM 60 s) als im RAM.
- Beziehungen: Gegenstück zum Lazy Loading. Realisiert durch Eviction und TTL.

## V

**VRAM**
- Synonyme: Grafikspeicher
- Bedeutung: Begrenzter Device-Speicher der GPU, für Modelle, Index-Mirror und Arbeitsbereiche. Allokation im laufenden Betrieb wird vermieden (implizite Synchronisierung).
- Beziehungen: Gegenstück zum RAM (Arbeitsspeicher). Hält nur einen abgeleiteten Mirror des Index. Verwaltet vom VRAM-Manager.

## W

**Weight-Source**
- Synonyme: Gewichtsquelle, safetensors
- Bedeutung: Gewichte im Format safetensors, per mmap und zero-copy auf Rohbytes. Sie werden pro Subengine in deren Arena kopiert.
- Beziehungen: gemeinsame Ressource beider Subengines, speist den Encoder.

**Worker**
- Synonyme: Thread des Processors, Executor-Thread
- Bedeutung: Idle-Thread im Worker-Pool (Anzahl: Cores − 1, keine Core-Zuordnung), der bei Notification aufwacht und den Worker-Loop abarbeitet. Prioritäten: GPU-Ergebnisse/Mirror-Back, GPU füttern, CPU füttern, neue Tasks expandieren.
- Beziehungen: bedient sich als CPU-Lane selbst an der Queue. Gehört zum Processor. Sein Blockieren ist das Backpressure-Signal.

**Write-Pointer-Modell**
- Synonyme: Cursor-Claim, Bump-Allocation (für beschreibbare Segmente)
- Bedeutung: Beim Index-Aufbau durch Kernel schiebt jeder Claim den Pointer voran. Passt einer nicht, scheitern alle späteren (Overflow), die gültigen Bytes bleiben ein zusammenhängendes Präfix. Ein Warp fasst Claims per Prefix-Sum zusammen.
- Beziehungen: arbeitet in Bereichen, die der Manager vorab aus der Arena vergibt. Teilergebnis (Partial Result) führt zum nächsten Bereich.
