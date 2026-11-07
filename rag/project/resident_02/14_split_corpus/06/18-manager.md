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

