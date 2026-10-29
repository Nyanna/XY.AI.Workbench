# Funktionale Anforderungen Subengine

## 1. Rechenverfahren für Stapelverarbeitung

- Das System stellt eine austauschbare Schnittstelle bereit, die einen Stapel von Gleitkommazahlen entgegennimmt und einen Stapel von Gleitkommazahlen als Ergebnis zurückgibt.
- Es gibt ein Verfahren auf dem Hauptprozessor. Es ist der Standard, wenn keine Grafikbeschleunigung einkompiliert ist, und das Ausweichziel, wenn die Grafikbeschleunigung nicht initialisiert werden kann.
- Es gibt ein Verfahren auf dem Grafikprozessor. Es ist die einzige Stelle, an der die Grafikbibliothek angebunden wird.
- Das Anlegen des Grafikprozessor-Verfahrens muss scheitern, wenn kein kompatibler Grafikprozessor verfügbar ist. Dadurch greift die Auswahl automatisch auf das Hauptprozessor-Verfahren zurück.

## 2. Gerätezugang

- Ein Gerätezugang beschreibt das Rechengerät einer Teilengine, nämlich Hauptprozessor oder Grafikprozessor.

## 4. Ereignisse

- Ein Ereignis ist ein undurchsichtiger Abschlussmarker für alle Arbeit, die vor seiner Aufzeichnung auf einer Bahn eingereicht wurde.
- Ereignisse sind kopierbar. Alle Kopien beziehen sich auf dasselbe Ereignis.
- Man kann abfragen, ob ein Ereignis abgeschlossen ist, ohne zu blockieren. Außerdem kann man den aufrufenden Faden blockieren, bis es abgeschlossen ist.
- Das Blockieren meldet bei Grafikereignissen Fehler als Ereignisfehler mit Ursache.
- Ein Hauptprozessor-Ereignis bleibt nach dem Signalisieren dauerhaft abgeschlossen und weckt alle Wartenden.
- Die Textdarstellung zeigt den Abschlussstatus.

## 8. Lebenszyklus verwalteter Ressourcen

### Ressourcen und Schlüssel

- Jede Ressource hat einen Schlüssel. Er ist eine Zeichenkette, die vergleichbar, als Index nutzbar und anzeigbar ist.
- Jede Ressource kann einen Kostenhinweis für das Wiederaufbauen nach einer Entladung liefern (Standard 0). Aufrufer können damit günstige Kandidaten zum Entladen bevorzugen.
- Jede Ressource kann das Entladen aus eigenen Gründen verweigern (Standard: erlaubt).

### Entladen im Leerlauf

- Ein Entladeversuch erhält die aktuelle Zeit und meldet, ob entladen wurde. Entladen wird nur, wenn alle folgenden Bedingungen erfüllt sind:
  - Es gibt keine Ausleihen.
  - Es läuft keine angemeldete Operation.
  - Die Ressource ist geladen.
  - Die Leerlaufdauer hat die konfigurierte Grenze erreicht.
  - Die Ressource verweigert das Entladen nicht.
- Der erste Versuch nach der letzten Rückgabe startet nur die Leerlaufmessung und entlädt noch nicht.

## 9. Auslastungsmeldung nach Abrufprinzip

### Teilengine

- Eine Teilengine gehört zu einem Hauptprozessor oder einer Grafikbeschleunigung. Sie stellt Ressourcen und Ausführungskapazität eines Rechensubstrats bereit.
- Sie ist aus mehreren Fäden nutzbar und gibt Auskunft über:
  - ihre Art,
  - ihr Gerät,
  - ihre Kapazität,
  - ihren geglätteten Durchsatz in Einträgen pro Sekunde,
- Die Arbeit kommt aus den Arbeitsschlangen des Prozessors. Die Teilengine meldet nur, wie viel sie aufnehmen kann.