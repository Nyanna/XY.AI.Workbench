# Funktionale Anforderungen Segment

## 1. Slot-Geometrie und Segmentbeschreibung

- Das System muss Speicherplätze (Slots) fester Größe verwalten. Jeder Slot nimmt die Vektoren eines Eintragsabschnitts auf, mit einer begrenzten Zahl von Token-Vektoren.
- Das System muss vordefinierte Slot-Klassen mit höchstens 32, 64 und 128 Token anbieten. Zusätzlich muss es frei wählbare Klassen mit einer beliebigen Obergrenze bis 65535 Token unterstützen.
- Die Größe eines Slots in Byte muss sich aus Token-Obergrenze, Vektordimension und Größe des Datentyps ergeben.
- Das System muss einen Fehler melden, wenn die Berechnung der Slot-Größe überläuft oder das Ergebnis nicht mehr in einen vorzeichenlosen 32-Bit-Wert passt.
- Eine Segmentansicht muss aus einem Speicherbereich, der Slot-Anzahl, der Slot-Klasse, der Dimension, dem Datentyp und einer Segmentgeneration bestehen.
- Beim Anlegen einer Segmentansicht muss das System prüfen, ob der Speicherbereich alle Slots fassen kann. Andernfalls muss es einen Fehler mit benötigter und verfügbarer Byte-Zahl melden.
- Das System muss die Gesamtgröße eines Segments in Byte liefern.
- Das System muss für jeden Slot den zugehörigen Teilbereich im Speicher liefern. Bei einer Slot-Nummer außerhalb des gültigen Bereichs muss es einen Fehler melden, der Slot-Nummer und Slot-Anzahl nennt.
- Das System muss für Rechenkerne auf dem Gerät eine Segmentbeschreibung mit Startadresse, Slot-Anzahl und festem Slot-Abstand bereitstellen.

## 2. Kodierung von Slot-Inhalten

- Das System muss die Vektoren eines Slots in ein Speicherabbild für das Gerät umwandeln. Zuerst kommen die echten Token-Zeilen in Eingabereihenfolge, danach Füllzeilen bis zur Token-Obergrenze der Klasse.
- Füllzeilen müssen einen endlichen Maskenwert enthalten, damit aufgefüllte Positionen nie eine Ähnlichkeit von genau null beitragen können.
- Das System muss die Datentypen 32-Bit-Gleitkomma, 16-Bit-Gleitkomma und das 16-Bit-Gehirn-Gleitkommaformat unterstützen. Die Werte müssen in Little-Endian-Reihenfolge geschrieben werden.
- Das System muss andere Datentypen mit einem Fehler ablehnen, der den Datentyp nennt.
- Das System muss ablehnen, wenn mehr Token angegeben werden, als die Klasse erlaubt. Das gilt auch, wenn die Anzahl der Eingabewerte nicht genau Token-Anzahl mal Dimension entspricht. Der zweite Fehler muss erwartete und erhaltene Wertezahl nennen.
- Das Ergebnis muss die volle Slot-Größe haben.

## 3. Zuordnung von Slots zu Einträgen

- Das System muss pro Segment eine Zuordnung von Slot zu Eintragsverweis führen. Ein Eintragsverweis besteht aus der Eintragskennung und der Nummer des Abschnitts innerhalb des Eintrags.
- Die Zuordnung muss zwei Adressierungsarten unterstützen:
  - Feste Adressierung mit Startadresse, Slot-Abstand und Anzahl.
  - Variable Adressierung mit aufsteigenden Summenwerten von Anzahl plus eins Grenzen.
- Das System muss Slots an Einträge binden und diese Bindung wieder lösen können. Bei ungültiger Slot-Nummer muss es einen Fehler melden.
- Das System muss zu einem Slot den gebundenen Eintragsverweis liefern oder dessen Fehlen anzeigen.
- Das System muss zu einer absoluten Geräteadresse den enthaltenen Slot oder Eintragsindex ermitteln.
- Das System muss zu einem Slot den absoluten Byte-Bereich liefern und für die gesamte Zuordnung den umspannenden Bereich. Bei leerer Zuordnung muss es kein Ergebnis liefern.
- Die Zuordnung muss gleichzeitig von mehreren Aufrufern lesbar und schreibbar sein.

## 4. Treffer und deren Auflösung

- Rechenkerne liefern Treffer nur mit Geräteadresse und Bewertung und kennen keine Einträge. Das System muss einen solchen Treffer in einen Treffer mit Segmentgeneration, Slot und Bewertung umrechnen.
- Liegt die Adresse in keinem Slot des Segments, muss das System einen Fehler wegen Slot außerhalb des Bereichs melden.
- Das System muss einen Treffer anhand eines Manifests zu einem Eintragsverweis auflösen. Fehlt das Segment im Manifest, muss es einen Fehler wegen veralteter Generation melden. Ist der Slot an keinen Eintrag gebunden, muss es einen Fehler wegen fehlender Zuordnung melden, der Generation und Slot nennt.
- Das System muss zu einem Eintrag den neuesten gültigen Ort (Segmentgeneration und Slot) ermitteln. Dabei haben neuere Segmente Vorrang vor älteren.
- Das System muss Treffer, die gegen eine ältere Manifestgeneration erzeugt wurden, in die aktuelle Generation überführen:
  - Ist das Segment im aktuellen Manifest geladen, wird dort aufgelöst. Andernfalls wird im vorherigen Manifest aufgelöst.
  - Ist der Treffer nicht auflösbar, wird er verworfen.
  - Ist das Segment geladen und der Slot noch gültig, bleibt der Eintrag unverändert.
  - Sonst sucht das System den aktuellen Ort des Eintrags. Bei Erfolg liefert es einen umgesetzten Treffer mit neuem Segment und Slot und unveränderter Bewertung. Ohne Erfolg verwirft es den Treffer.
- Das System muss ein Manifest nach einer Segmentgeneration durchsuchen können.

## 5. Schreiben, Bestätigen und Ungültigmachen von Slots

- Segmente werden nur durch Anhängen erweitert. Solange ein Segment geladen ist, darf kein belegter Slot überschrieben werden. Speicher gelöschter Einträge wird nur durch Neuaufbau des Segments zurückgewonnen.
- Das System muss einen Schreibvorgang einleiten können, indem es den nächsten freien Slot reserviert. Die Reservierung hält Eintragsverweis, Token-Anzahl, optionale Zuordnung zu einem Zentrum (Cluster) und den Zielbereich fest. Der Slot bleibt unsichtbar, bis er bestätigt wird.
- Das System muss die Reservierung ablehnen, wenn die Token-Anzahl die Obergrenze der Klasse oder den 16-Bit-Bereich überschreitet. Der Fehler muss Anzahl und Grenze nennen.
- Das System muss die Reservierung ablehnen, wenn kein freier Slot mehr vorhanden ist. Der Fehler muss die Slot-Anzahl nennen.
- Das System muss einen Slot sofort bestätigen können, sofern die Datenübertragung abgeschlossen ist. Andernfalls muss es einen Fehler "noch nicht bereit" melden.
- Das System muss eine Bestätigung auch zurückstellen können, bis die Datenübertragung abgeschlossen ist. Eine Abwicklungsfunktion muss zurückgestellte Bestätigungen in Einreihungsreihenfolge anwenden. Sie muss beim ersten noch nicht abgeschlossenen Vorgang anhalten und die Zahl der neu sichtbar gewordenen Slots liefern.
- Beim Bestätigen muss das System:
  - den Slot an den Eintrag binden,
  - die Metadaten setzen (gültig, Token-Anzahl, fortlaufende Schreibnummer, Zentrumszuordnung),
  - den Slot als gültig markieren.
- Wurde der Eintrag bereits in einem anderen Slot desselben Segments abgelegt, muss das System den alten Slot ungültig machen (Aktualisierung durch Ersetzen). Die Zuordnung des alten Slots zum Eintrag soll bestehen bleiben, damit ältere Treffer noch auflösbar sind.
- Das System muss einen Eintrag in einem Segment ausblenden können, ohne seine Daten zu entfernen. Es muss zurückmelden, ob der Eintrag dort vorhanden war.
- Das System muss abfragbar machen:
  - ob ein Slot gültig ist (Slots außerhalb des Bereichs gelten als ungültig),
  - die Anzahl gültiger Slots,
  - die Metadaten eines Slots,
  - den Slot eines Eintrags,
  - eine Momentaufnahme der Gültigkeitsmarkierungen in 64-Bit-Wörtern, damit sie neben den Slot-Daten auf das Gerät hochgeladen werden kann.
- Die Gültigkeitsmarkierungen müssen nebenläufig sicher lesbar und änderbar sein.

## 6. Manifeste und Generationen

- Das System muss eine Momentaufnahme der Segmente als Manifest mit fortlaufender Generationsnummer führen. Der Ausgangszustand ist ein leeres Manifest mit Generation null.
- Ein Nachfolger muss genau eine Generation höher liegen.
- Ein Manifest muss liefern können:
  - seine Segmente in Reihenfolge,
  - die Generationsnummer,
  - die Gesamtgröße in Byte,
  - die Zahl der aktuell gehaltenen Sicherungen (Leases).
- Das System muss eine Segmenttabelle für Rechenkerne erzeugen und in ein Binärformat überführen. Pro Segment sind es 24 Byte in Little-Endian-Reihenfolge, in dieser Folge: Segmentgeneration (8 Byte), Startadresse (8 Byte), Slot-Anzahl (4 Byte), Slot-Abstand (4 Byte). Bei nicht fester Anordnung ist der Abstand null.

## 7. Veröffentlichung und Lebensdauer

- Das System muss das aktuelle Manifest ohne Sperren für Leser bereitstellen und atomar durch ein neues ersetzen können. Dabei muss es das vorherige Manifest zurückgeben.
- Das System muss eine Veröffentlichung ablehnen, wenn die neue Generation nicht strikt größer ist als die aktuelle. Es muss den Fehler wegen veralteter Generation melden.
- Konkurrierende Veröffentlichungen müssen durch Wiederholung korrekt aufgelöst werden. Für Tests muss sich eine Störung der Veröffentlichung gezielt auslösen lassen.
- Ein Leser muss die aktuelle Generation sichern können. Solange die Sicherung besteht, bleiben Manifest und Segmente erhalten, und die Zahl der Sicherungen ist am Manifest sichtbar.
- Eine Sicherung muss es erlauben, Abschlussmarker von Geräteoperationen bei allen Segmenten des gesicherten Manifests zu registrieren.
- Das System muss Abschlussmarker pro Segment registrieren, damit die Freigabe auf Geräteoperationen wartet, die das Segment noch lesen. Bereits abgeschlossene Marker dürfen dabei verworfen werden.
- Das System muss den für einen Segmentwechsel nötigen Gerätespeicher berechnen: aktueller Bestand plus eingehende Menge.

## 8. Verzögerte Freigabe von Speicher

- Wird ein Segment nicht mehr gebraucht, muss es seinen Speicherbereich zusammen mit allen noch offenen Abschlussmarkern an die Speicherverwaltung melden. Dazu zählen auch die Marker von zurückgestellten, noch nicht angewendeten Schreibvorgängen. Segmente ohne Meldekanal melden nichts.
- Die Meldung ist nur möglich, wenn kein Manifest und keine Sicherung das Segment mehr hält.
- Eine Rückforderungsfunktion muss alle gemeldeten Bereiche liefern, deren Marker vollständig abgeschlossen sind, damit der Aufrufer sie dem Speicherbereich zurückgibt. Alle übrigen Bereiche muss sie für spätere Aufrufe zurückhalten.

## 9. Gesteuerte Veröffentlichung hinter Kopiervorgängen

- Das System muss neue Manifeste mit einem Abschlussmarker ihres Kopiervorgangs vormerken können und die Zahl der vorgemerkten Manifeste angeben.
- Das System muss einen Kopierauftrag auf der Kopierspur einreihen, ein Ereignis für dessen Abschluss aufzeichnen und das Manifest hinter diesem Ereignis vormerken. Schlägt Einreihen oder Aufzeichnen fehl, muss es einen Synchronisierungsfehler mit der ursprünglichen Ursache melden.
- Eine Abfragefunktion muss vorgemerkte Manifeste in Vormerkreihenfolge veröffentlichen, solange das jeweils vorderste abgeschlossen ist. Sie liefert die Zahl der veröffentlichten Manifeste. Ein noch nicht abgeschlossenes Manifest blockiert alle dahinter liegenden. Scheitert eine Veröffentlichung, muss der Fehler weitergereicht werden.

## 10. Änderungsbenachrichtigungen

- Das System muss Änderungen am Segmentbestand an beliebig viele Abonnenten verteilen. Der Puffer ist in seiner Größe konfigurierbar.
- Ein Ereignis muss Art, Segmentkennung, Segmentgeneration und Manifestgeneration enthalten.
- Nach jeder Veröffentlichung muss das System Ereignisse für den Unterschied zum vorherigen Manifest senden:
  - "Hinzugefügt" für Segmente, deren Kennung zuvor nicht vorhanden war.
  - "Geändert" für Segmente mit gleicher Kennung, aber anderer Segmentgeneration.
  - Kein Ereignis für unveränderte Segmente.
- Das Senden darf nicht scheitern, wenn kein Abonnent vorhanden ist.