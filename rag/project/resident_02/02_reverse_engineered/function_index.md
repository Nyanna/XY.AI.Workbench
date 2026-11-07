# Funktionale Anforderungen an den dateigestützten Vektorindex

## 1. Änderungsmarken

- Das System muss jede Schreib- und jede Löschoperation mit einer neuen Änderungsmarke versehen. Die Marke ist innerhalb eines Index eindeutig, auch bei gleichzeitigen Zugriffen mehrerer Ausführungsstränge.
- Die Marke ist ein undurchsichtiger Veränderungsmarker. Konsumenten dürfen sie nur auf Gleichheit oder Ungleichheit prüfen. Eine Ordnungsbeziehung wird bewusst nicht angeboten.
- Die Marke muss im exklusiven Schreibabschnitt vergeben werden, unmittelbar vor der Veröffentlichung der Änderung.
- Das Entfernen eines Eintrags verbraucht ebenfalls eine Marke. Wird dieselbe Kennung später erneut angelegt, erhält der Eintrag dadurch eine andere Marke als zuvor.
- Ein neuer Index beginnt bei der kleinsten Marke eins.
- Ein wiederaufgenommener Index setzt oberhalb des größeren der beiden Werte fort: der gespeicherten Höchstmarke und der höchsten tatsächlich in den Daten vorkommenden Marke. Dadurch wird nie eine Marke doppelt vergeben.
- Die Höchstmarke (zuletzt vergebene Marke, null bei einem leeren Zähler) muss abfragbar sein.
- Die Rohdarstellung einer Marke muss für Persistenz und Benachrichtigungen zugänglich sein.

## 2. Erkennung von Änderungen an Dateien (Rückfallkette)

Das System muss Änderungen an Quelldateien in drei Stufen erkennen: zuerst über die Änderungsmarke, dann über Größe und Zeitstempel, zuletzt über einen Inhaltshash.

- Zu einer Datei wird festgehalten:
  - die Größe,
  - die Zeit der letzten Inhaltsänderung,
  - optional die Zeit der letzten Metadatenänderung,
  - der Referenzzeitpunkt des Durchlaufs, bei dem diese Angaben aufgezeichnet wurden.
- Eine Aufzeichnung gilt als unsicher, wenn ihre Änderungszeit höchstens zwei Sekunden vor dem Referenzzeitpunkt liegt oder danach. Dieses Zeitfenster entspricht dem ungünstigsten Fall grober Zeitstempelauflösung, wie sie manche Dateisysteme haben. In diesem Fall könnte eine spätere Änderung den Zeitstempel unverändert gelassen haben.
- Das Ergebnis eines Vergleichs ist entweder „unverändert“ oder „Inhaltsprüfung nötig“.
- „Unverändert“ darf nur gemeldet werden, wenn alle drei Bedingungen erfüllt sind:
  - Größe, Änderungszeit und Metadatenänderungszeit stimmen mit dem aktuellen Stand überein,
  - die Aufzeichnung ist nicht unsicher.
- In allen anderen Fällen ist eine Inhaltsprüfung nötig. Dabei wird zu Diagnosezwecken protokolliert, ob die Aufzeichnung unsicher war und ob die Angaben übereinstimmten.
- Für die Inhaltsprüfung muss ein schneller, nicht kryptografischer 64-Bit-Hash über die Dateibytes berechnet werden können.
- Algorithmus und Version dieses Hashes gehören in den Kopfbereich des Index.

## 3. Allgemeiner Vertrag eines Index

Der Index muss unabhängig vom Kern des Systems sein. Er meldet Änderungen und liefert Struktur und Marken. Er kennt keine Geometrie und löst keine Suchbereiche auf. Er muss über eine gemeinsame, geteilt nutzbare Schnittstelle von mehreren Ausführungssträngen gleichzeitig verwendbar sein.

### Abfragen

- **Marke eines Eintrags:** Das System liefert die aktuelle Marke oder „nicht vorhanden“, wenn der Eintrag entfernt wurde oder nie existierte.
- **Alle Marken:** Das System liefert alle Einträge samt Marken als Momentaufnahme zum Abfragezeitpunkt. Spätere Änderungen beeinflussen sie nicht. Dabei dürfen keine Vektordaten berührt oder geladen werden.
- **Lesen eines Eintrags:**
  - Das System liefert den Vektorbereich des Eintrags.
  - Fehlt der Eintrag, liefert es leere Daten.
  - Der Lesevorgang darf blockieren, etwa durch Seitenfehler oder Plattenzugriffe.
  - Die Daten dürfen entweder geliehen oder gemeinsam referenziert geliefert werden. Ein bereits gelesener Wert darf sich durch spätere Schreibvorgänge nicht verändern.

### Schreiben

- Das Schreiben eines Eintrags ersetzt dessen Daten vollständig.
- Mehrere Schreiber werden serialisiert. Bei Konkurrenz gewinnt der Schreiber, der zuletzt an die Reihe kommt.
- Die Schreibreihenfolge ist verbindlich: zuerst die Daten, dann die Marke, dann das Signal.
- Das Schreiben liefert die Marke genau dieses Schreibvorgangs zurück. Aufrufer registrieren sie als bekannt, um das Echo ihrer eigenen Änderung zu unterdrücken.

### Änderungsbenachrichtigung

- Das System bietet ein Pegelsignal an, das den jeweils aktuellsten Markenstand trägt.
- Beliebig viele Beobachter können sich anmelden.
- Der Anfangswert eines neuen Beobachters ist die aktuelle Höchstmarke.
- Jeder Schreib- oder Löschvorgang aktualisiert das Signal.
- Zusammengefasste Signale sind unschädlich. Beobachter konsumieren den neuesten Stand und müssen nicht jeden Zwischenstand sehen.

## 4. Suchbereiche und Entry-Mengen

### Suchbereiche

Das System muss folgende Arten von Suchbereichen unterscheiden:

- alle Einträge,
- ein numerischer Kennungsbereich,
- eine explizite Menge von Einträgen,
- ein Teilbaum ab einem Knoten,
- ein Byte-Präfix,
- eine Schwerpunktumgebung mit Anzahl und optionalem Radius.

Jede Art hat eine stabile Bezeichnung für Protokollierung und Kennzahlen: „all“, „range“, „set“, „subtree“, „prefix“ und „centroid“. Neue Arten müssen später ergänzbar sein.

### Eintragsmengen

- Eine Menge von Eintragskennungen ist platzsparend abgelegt und unterstützt 64-Bit-Kennungen.
- Sie kann auf zwei Wegen aufgebaut werden:
  - aus einer Folge von Bereichen (leere oder umgekehrte Bereiche werden ignoriert),
  - aus einer Folge einzelner Kennungen.
- Sie bietet folgende Operationen:
  - Prüfung auf Leerheit,
  - Anzahl,
  - Enthaltensein einer Kennung,
  - aufsteigende Iteration,
  - Vereinigung,
  - Schnitt,
  - Gleichheitsvergleich,
  - Ableitung eines Hashwerts, der mit der Gleichheit verträglich ist,
  - Ausgabe als maximale zusammenhängende, aufsteigend sortierte Folgen (diese Ausgabe durchläuft alle Kennungen und ist bei großen Mengen teuer).
- Eintragskennungen sind vollständig geordnet. Knoten und Zellen haben eigene Kennungstypen.

## 5. Referenzindex im Arbeitsspeicher

- Er muss ohne Grafikbeschleunigung und ohne Bibliotheken für maschinelles Lernen lauffähig sein.

## 6. Persistenter, dateigestützter Index (Parquett)

### Öffnen

- Zum Öffnen werden Dateipfad, erwarteter Kopfbereich und Optionen angegeben.
- Existiert die Datei nicht, entsteht ein leerer Index. Die Datei wird erst durch den ersten Schreibvorgang der Persistenz angelegt.
- Der Zugriff auf die Datei erfolgt wahlweise über Speicherabbildung oder über herkömmliches Dateilesen. Bei der Speicherabbildung wird, wo das Betriebssystem es zulässt, wahlfreier Zugriff als Zugriffsmuster angekündigt.
- Beim Öffnen einer vorhandenen Datei prüft das System der Reihe nach:
  - Der Kopfbereich im Dateifuß muss vollständig und lesbar sein. Fehlende Schlüssel, nicht numerische Zahlenwerte und unbekannte Aufzählungswerte werden als Schemainkompatibilität gemeldet. Die Meldung nennt den betroffenen Schlüssel sowie erwarteten und gefundenen Wert.
  - Der Kopfbereich muss mit dem erwarteten übereinstimmen (siehe Abschnitt 7). Alle abweichenden Schlüssel werden gesammelt in einer Fehlermeldung genannt, jeweils mit erwartetem und gefundenem Wert.
  - Die Spaltenstruktur muss im Namen, im Datentyp und in der Nullbarkeit der erwarteten Struktur entsprechen. Metadaten der Spalten werden ignoriert. Bei Abweichung enthält die Fehlermeldung beide Strukturbeschreibungen.
  - Über die Spalten für Kennung und Marke werden alle Einträge samt Marken eingelesen, ohne Vektoren zu dekodieren. Eine doppelt vorkommende Kennung ist ein Fehler („Index nicht verfügbar“).
- Der Markenzähler wird gemäß Abschnitt 1 aus der im Kopfbereich gespeicherten Höchstmarke und der höchsten gefundenen Marke fortgesetzt.
- Optional (Standard: eingeschaltet) werden beim Öffnen alle Daten dekodiert und alle Prüfsummen kontrolliert. Das Ergebnis wird protokolliert, und ein Fehler bricht das Öffnen ab.
- Das Öffnen wird mit einem Diagnoseabschnitt protokolliert, der Pfad, Schemaversion, Modellkennung, Modellrevision, Zugriffsart, Anzahl der Zeilengruppen und das Prüfsummenergebnis enthält.
- Fehler beim Öffnen oder Lesen der Datei werden als „Index nicht verfügbar“ gemeldet, mit Pfad und Ursache.

### Verzögertes Laden

- Vektoren und Nutzdaten werden erst bei der ersten Verwendung vollständig aus der Datei dekodiert und danach im Speicher gehalten. Das Laden erfolgt höchstens einmal. Ein Fehlschlag wird gemerkt und bei jedem weiteren Zugriff als „Index nicht verfügbar“ gemeldet.
- Beim Lesen eines Vektors führt ein solcher Fehler zu leeren Daten und einem Fehlerprotokolleintrag.
- Bei der Abfrage der Nutzdaten führt er zu „nicht vorhanden“ und ebenfalls zu einem Fehlerprotokolleintrag.
- Nach der Dekodierung stehen die Vektoren als 16-Bit-Gleitkommawerte in Little-Endian-Bytefolge zur Verfügung, die Datei wird in Stapeln von 4096 Zeilen gelesen.

### Optionen

- Zugriffsart: Speicherabbildung (Standard) oder Dateilesen. Die Bezeichnungen für Protokolle lauten „mmap“ und „file“.
- Prüfung beim Öffnen: ein oder aus (Standard: ein).
- Zeilen je Zeilengruppe beim Schreiben (Standard: 8192; ein Wert unter eins wird wie eins behandelt).

### Einträge verwalten

- **Einfügen oder Ersetzen:**
  - Ein Eintrag besteht aus Kennung, Vektor und Nutzdaten.
  - Der Vektor muss exakt die erwartete Länge haben: Dimension mal zwei Bytes. Andernfalls wird ein Schemafehler mit erwarteter und gefundener Länge gemeldet.
  - Das System berechnet die Prüfsumme, vergibt eine neue Marke, markiert den Index als ungesichert, löst das Signal aus und protokolliert die Operation.
  - Die Änderung liegt zunächst nur im Speicher und wird erst durch den Schreibvorgang der Persistenz dauerhaft.
- **Schreiben über den allgemeinen Vertrag:**
  - Die übergebenen Daten müssen genau ein Vektor sein.
  - Die Nutzdaten eines vorhandenen Eintrags bleiben erhalten. Bei einem neuen Eintrag sind sie leer.
  - Ein Vektor falscher Länge verletzt den Vertrag und wird als nicht behebbarer Programmfehler behandelt, der die Ausführung abbricht.
- **Entfernen:** Der Eintrag wird entfernt, der Index wird als ungesichert markiert, es wird eine Marke vergeben und ein Signal ausgelöst. War der Eintrag nicht vorhanden, ändert sich nichts und das Ergebnis lautet „nicht vorhanden“.
- **Nutzdaten abfragen:** Das System liefert die Nutzdaten eines Eintrags aus dem Speicher oder aus der geladenen Datei, oder „nicht vorhanden“.
- **Zustandsabfragen:** Pfad, Kopfbereich, Höchstmarke und Status „ungesicherte Änderungen vorhanden“ müssen abfragbar sein. Zusätzlich muss eine ausdrückliche Prüfung angeboten werden, die die Datei vollständig dekodiert und jede Prüfsumme vergleicht. Beim ersten Fehler wird die betroffene Eintragskennung genannt. Ein Index ohne Datei gilt als geprüft.

### Dauerhaftes Sichern

- Mehrere gleichzeitige Sicherungsvorgänge werden hintereinander ausgeführt.
- Gibt es seit der letzten Sicherung keine Änderungen, schreibt das System nichts und meldet „keine Änderung“. Sonst schreibt es und meldet „geschrieben“.
- Der geschriebene Stand ist eine konsistente Momentaufnahme. Änderungen, die während des Schreibens eintreffen, bleiben als ungesichert vermerkt und gehen nicht verloren.
- Die Datei enthält alle aktuellen Einträge, aufsteigend nach Kennung, sowohl aus der Ursprungsdatei als auch aus dem Speicher. Marken, Nutzdaten und Prüfsummen bleiben dabei unverändert. Der Kopfbereich trägt die aktuelle Höchstmarke.
- Das Schreiben muss atomar sein:
  - Zuerst entsteht neben dem Ziel eine Hilfsdatei.
  - Ihr Inhalt wird vollständig auf den Datenträger gesichert.
  - Dann ersetzt sie die Zieldatei durch Umbenennen.
  - Danach wird das übergeordnete Verzeichnis gesichert.
  - Bei jedem Fehler wird die Hilfsdatei gelöscht und die bisherige Datei bleibt unangetastet. Der Fehler wird als „Index nicht verfügbar“ gemeldet.
- Beim Schreiben gelten folgende Eigenschaften:
  - Die Zeilen werden in Zeilengruppen der konfigurierten Größe geschrieben.
  - Es erfolgt keine Kompression.
  - Statistiken werden je Seite geführt.
  - Für die Vektorspalte wird keine Wörterbuchkodierung verwendet.
  - Der Kopfbereich wird als Schlüssel-Wert-Paare im Dateifuß abgelegt.
- Der Schreibvorgang wird mit Pfad, Schemaversion, Modell, Zeilenzahl und Zeilengruppenzahl protokolliert.

## 7. Dateiformat und Bindung an das Modell

### Spalten

Die Datei enthält genau diese Spalten in dieser Reihenfolge, keine davon nullbar:

1. Eintragskennung (vorzeichenlose 64-Bit-Zahl)
2. Änderungsmarke (vorzeichenlose 64-Bit-Zahl)
3. Vektor (Liste fester Länge, entsprechend der Dimension, aus 16-Bit-Gleitkommawerten)
4. Zieldatei (Text)
5. Zeichenanfang (vorzeichenlose 64-Bit-Zahl)
6. Zeichenende (vorzeichenlose 64-Bit-Zahl)
7. Prüfsumme (vorzeichenlose 64-Bit-Zahl)

Die Marke steht in einer eigenen Spalte, damit das Einlesen der Marken keine Vektoren laden muss.

### Kopfbereich

Der Kopfbereich im Dateifuß enthält folgende Angaben:

- Schemaversion (aktuell eins)
- Höchstmarke
- Hashalgorithmus und Hashversion
- Modellkennung und Modellrevision
- Dimension
- Pooling-Verfahren
- Normalisierung
- Präfix für Anfragen und Präfix für Textpassagen
- Speicherdatentyp
- Rechendatentyp
- maximale Sequenzlänge
- Fingerabdruck des Tokenisierers

Bei der Ableitung des Kopfbereichs aus einer Modellbeschreibung gilt:

- Als Speicherdatentyp wird ausschließlich der 16-Bit-Gleitkommatyp akzeptiert. Jeder andere Typ wird als Schemainkompatibilität abgelehnt.
- Der Hashalgorithmus ist der schnelle 64-Bit-Hash, die Hashversion ist eins, die Höchstmarke ist null.
- Der Rechendatentyp wird als Text festgehalten und nennt den Typ für Hauptprozessor und Grafikprozessor.
- Aus dem Kopfbereich lassen sich Dimension und Vektorgröße in Bytes (Dimension mal zwei) ableiten. Außerdem kann er mit einer anderen Höchstmarke kopiert werden.

### Prüfung der Bindung

- Die Bindung zwischen Datei und aktivem Modell gilt nur bei Gleichheit aller dieser Angaben: Schemaversion, Hashalgorithmus, Hashversion, Modellkennung, Modellrevision, Dimension, Pooling, Normalisierung, beide Präfixe, Speicherdatentyp, maximale Sequenzlänge und Tokenisierer-Fingerabdruck.
- Höchstmarke und Rechendatentyp sind rein informativ. Sie werden nicht verglichen, weil gemischtes Indizieren auf Hauptprozessor und Grafikprozessor erlaubt ist.
- Der Kopfbereich muss aus Schlüssel-Wert-Paaren lesbar und schreibbar sein, ohne Informationsverlust.
- Folgende Aufzählungswerte werden unterstützt:
  - Pooling: gemaskiertes Mittel, je Token.
  - Normalisierung: euklidisch (L2), keine.
  - Datentypen: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma, Brain-Floating-Point-16, vorzeichenloses 8-Bit, vorzeichenloses 32-Bit, vorzeichenbehaftetes 64-Bit.
  - Hashalgorithmus: schneller Hash mit 64 Bit, schneller Hash mit 128 Bit, kryptografischer Hash.

### Fingerabdruck des Tokenisierers

Der Fingerabdruck wird aus der Tokenisierer-Definition abgeleitet. Er besteht aus der Algorithmusbezeichnung, einem Doppelpunkt und dem 64-Bit-Hashwert des Inhalts in 16 hexadezimalen Stellen mit führenden Nullen.

## 8. Nutzdaten und Integritätsprüfung

- Zu jedem Eintrag gehört ein Rückverweis auf den Quelltext: Name der Zieldatei, Zeichenanfang und Zeichenende. Der Standardwert ist eine leere Datei mit Anfang und Ende null.
- Zu jedem Eintrag wird eine 64-Bit-Prüfsumme gespeichert. Sie wird aus der Länge des Vektors, den Vektorbytes, der Länge des Dateinamens, dem Dateinamen sowie Zeichenanfang und Zeichenende berechnet. Die vorangestellten Längenangaben schließen mehrdeutige Aneinanderreihungen aus.
- Jede Veränderung eines Bits im Vektor oder in den Nutzdaten muss dadurch erkennbar sein.
- Die Prüfsumme wird beim Speichern berechnet, mitgeschrieben und bei der Prüfung der Datei neu berechnet und verglichen.

## 9. Fehlerverhalten

Das System unterscheidet drei Fehlerarten:

- **Index nicht verfügbar:** Datei nicht lesbar oder beschreibbar, beschädigte Struktur, doppelte Kennung oder Ladefehler. Die Meldung nennt Pfad und Ursache.
- **Schemainkompatibilität:** Kopfbereich, Spaltenstruktur oder Vektorlänge passen nicht. Die Meldung nennt erwartete und gefundene Werte.
- **Prüfsummenabweichung:** Die Meldung nennt die betroffene Eintragskennung.

Fehler beim Lesen einzelner Einträge dürfen den Betrieb nicht abbrechen. Sie führen zu leeren Daten beziehungsweise „nicht vorhanden“ und werden protokolliert.
