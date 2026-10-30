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

