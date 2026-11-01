# Funktionale Anforderungen an den gemeinsam genutzten Tokenisierer

## Bereitstellung und Laden

- Das System muss einen Tokenisierer aus der Konfigurationsdatei des Tokenisierers laden. Diese Datei muss im selben Verzeichnis liegen wie die Konfigurationsdatei des gewählten Modells und einen festen, vorgegebenen Dateinamen tragen.
- Das System muss den Tokenisierer alternativ aus einem bereits geladenen Tokenisierer erzeugen können. Dafür sind eine frei wählbare Kennung, ein Schema für die Moduswahl und eine maximale Sequenzlänge anzugeben.
- Beim Laden aus einer Modellbeschreibung muss das System das Schema zur Textvorbereitung aus den in der Modellbeschreibung hinterlegten Präfixen bilden und die maximale Sequenzlänge aus der Modellbeschreibung übernehmen.
- Das System muss beim Laden aus einer Modellbeschreibung die Ressourcenkennung aus dem Dateipfad des Tokenisierers ableiten, sodass jeder geladene Tokenisierer eindeutig identifizierbar ist.
- Wenn die Datei fehlt oder nicht lesbar ist, muss das System den Ladevorgang mit einem Ladefehler abbrechen. Die Fehlermeldung muss den Dateipfad und die ursprüngliche Ursache enthalten.
- Das System muss das Auffüllzeichen des Tokenisierers in dieser Reihenfolge bestimmen:
   - zuerst das im Tokenisierer konfigurierte Auffüllzeichen,
   - dann das Zeichen mit der Bezeichnung in spitzen Klammern,
   - dann das Zeichen mit der Bezeichnung in eckigen Klammern in Großbuchstaben.
- Wenn keines dieser Auffüllzeichen existiert, muss das System den Ladevorgang mit einem Ladefehler abbrechen, der das Fehlen eines Auffüllzeichens benennt.
- Das System muss die im Tokenisierer eingebaute automatische Auffüllung abschalten. Das Auffüllen auf gleiche Länge ist ausschließlich Aufgabe der späteren Batch-Bildung.
- Das System muss jede Eingabe auf die konfigurierte maximale Sequenzlänge kürzen. Dabei muss das abschließende Sondertoken erhalten bleiben.
- Wenn die Kürzungseinstellung vom Tokenisierer abgelehnt wird, muss das System den Ladevorgang mit einem Ladefehler und der ursprünglichen Ursache abbrechen.
- Das System muss das ermittelte Auffüllzeichen nach außen abfragbar machen, damit nachgelagerte Schritte damit auffüllen können.

## Textvorbereitung

- Das System muss optional einen austauschbaren Textnormalisierer entgegennehmen. Ohne Normalisierer bleibt der Text vor der weiteren Verarbeitung unverändert.
- Der Normalisierer muss von mehreren Verarbeitungsfäden gleichzeitig nutzbar sein.
- Das System muss einen Normalisierer mitliefern, der den Text unverändert lässt.
- Das System darf Normalisierungen, die der Tokenisierer selbst durchführt, nicht ein zweites Mal ausführen. Ein eigener Normalisierer ergänzt nur zusätzliche Umformungen.
- Das System muss die Normalisierung vor dem Voranstellen von Präfixen oder Markierungen und vor der Tokenisierung ausführen.

## Kodierungsmodi und Schemata

- Das System muss zwei Kodierungsmodi unterscheiden: Suchanfrage und Textpassage (Dokument). Je Modus wird der Eingabetext anders vorbereitet.
- Das System muss zwei Schemata unterstützen, nach denen ein Modus den Text vor der Tokenisierung und die Zahlenfolge nach der Tokenisierung verändert.
- Beim Präfix-Schema muss das System dem Text je nach Modus einen festen Textpräfix voranstellen: den Anfragepräfix im Anfragemodus und den Passagenpräfix im Passagenmodus. Die Zahlenfolge nach der Tokenisierung bleibt unverändert.
- Beim Schema für späte Interaktion (mehrvektorielle Einbettung) muss das System dem Text je nach Modus eine Anfragemarkierung oder eine Dokumentmarkierung voranstellen.
- Dieses Schema muss folgende Einstellungen je Modellvariante vorsehen:
    - den Markierungstext für Anfragen,
    - den Markierungstext für Dokumente,
    - die feste Länge für Anfragen,
    - die Höchstlänge für Dokumente,
    - die Kennung des Maskierungstokens.
- Im Anfragemodus muss das System die Zahlenfolge beim Schema für späte Interaktion auf die Anfragelänge kürzen. Kürzere Folgen füllt es mit dem Maskierungstoken auf genau diese Länge auf. Die aufgefüllten Positionen werden vom Modell berücksichtigt und nicht ausgeblendet.
- Im Passagenmodus muss das System die Zahlenfolge beim Schema für späte Interaktion auf die Dokumenthöchstlänge kürzen und nicht auffüllen.
- Die Kürzungen des Schemas für späte Interaktion gelten zusätzlich zur allgemeinen Kürzung auf die maximale Sequenzlänge.

## Kodierung von Texten

- Das System muss eine Liste von Texten in einem gewählten Modus in Zahlenfolgen (Token-Kennungen) umwandeln. Dabei sind beliebige Texttypen zulässig, die sich als Zeichenkette lesen lassen.
- Wenn die Liste leer ist, muss das System die Kodierung mit einem Fehler „keine Eingabetexte“ ablehnen.
- Das System muss jeden Text in dieser Reihenfolge verarbeiten: optional normalisieren, modusabhängigen Präfix oder Markierung voranstellen, tokenisieren, schemaabhängig nachbearbeiten.
- Bei der Tokenisierung muss das System die vom Tokenisierer vorgesehenen Sondertoken hinzufügen.
- Das System muss alle Texte gemeinsam als Stapel tokenisieren.
- Wenn die Tokenisierung scheitert, muss das System mit einem Kodierungsfehler abbrechen, der die ursprüngliche Ursache enthält.
- Das System muss die Zahlenfolgen in der Reihenfolge der Eingabetexte zurückgeben, ohne Auffüllung, außer der festen Auffüllung der Anfragen beim Schema für späte Interaktion.
- Das System muss ermitteln, wie viele Eingabetexte wegen der Längenbegrenzung gekürzt wurden, und diese Anzahl zusammen mit Modus und Textanzahl für Diagnosezwecke protokollieren.
- Das System muss nach der Kodierung die größte Sequenzlänge für Diagnosezwecke auf feinster Protokollstufe festhalten.

## Bildung von Batches

- Das System muss Texte kodieren und das Ergebnis anschließend in aufgefüllte Batches aufteilen, die nach Sequenzlänge gruppiert sind. Die Aufteilung richtet sich nach einer übergebenen Batch-Konfiguration.
- Jeder Batch muss Angaben enthalten, mit denen sich jede Zeile dem Index des zugehörigen Eingabetexts zuordnen lässt. So kann die ursprüngliche Reihenfolge wiederhergestellt werden.
- Fehler der Kodierung, einschließlich der leeren Eingabe, muss das System unverändert an den Aufrufer weitergeben.
- Das System muss die Anzahl der gebildeten Batches für Diagnosezwecke protokollieren.