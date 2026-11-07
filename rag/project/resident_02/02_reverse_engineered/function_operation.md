# Funktionale Anforderungen Operations

## Belegung von Geräte-Speicherbereichen

- Das System muss den Zugriff auf einen Speicherbereich eines Geräts über eine Belegung abbilden. Solange mindestens eine Belegung für den Bereich besteht, darf der Bereich nicht verdrängt werden.
- Das System muss für einen Speicherbereich einen Fixierungszähler bereitstellen. Mehrere Belegungen und mehrere Beteiligte müssen ihn gemeinsam nutzen können.
- Der Fixierungszähler muss bei der Erstellung den Wert null haben.
- Das System muss den aktuellen Zählerstand abfragbar machen.
- Das System muss abfragbar machen, ob ein Bereich nicht fixiert ist. Das ist genau dann der Fall, wenn der Zählerstand null beträgt.
- Beim Erzeugen einer Belegung muss das System den Fixierungszähler um eins erhöhen.
- Beim Beenden einer Belegung muss das System den Fixierungszähler automatisch um eins verringern. Ein manuelles Freigeben darf dafür nicht nötig sein.
- Beim Erzeugen einer Belegung muss optional ein Rückmeldekanal angegeben werden können. Wenn er angegeben ist, muss das System beim Beenden der Belegung den freigegebenen Speicherbereich über diesen Kanal melden.
- Ist der Empfänger der Rückmeldung nicht mehr erreichbar, muss das System das Beenden der Belegung dennoch ohne Fehler abschließen.
- Das System muss von jeder Belegung den zugehörigen Speicherbereich abfragbar machen.
- Das System muss die Belegung auch unter einer alternativen Bezeichnung als Gerätehandle anbieten.
- Zähleränderungen und Zählerabfragen müssen nebenläufig korrekt funktionieren.

## Arten von Operationen

- Das System muss folgende Operationsarten unterscheiden:
  - Zerlegung von Text in Tokens
  - Erzeugung von Einbettungen
  - Zusammenfassung (Pooling)
  - Normalisierung auf Einheitslänge
  - Ähnlichkeitsberechnung
  - Auswahl der besten Treffer
  - Schreiben in einen Index
  - Inferenz
  - Ausdrückliche Übertragung von Daten vom Gerät in den Hauptspeicher
- Die Menge der Operationsarten muss um weitere Arten erweiterbar sein.
- Jede Operation muss ihre Art, ihre Parameter und ihre Ressourcenanforderungen nennen können. Die Anforderungen müssen sich aus den Parametern ergeben.
- Operationen müssen von mehreren Ausführungssträngen gleichzeitig verwendet werden können.

## Operationsparameter

- Das System muss Parameter einer Operation enthalten: die Ressourcenanforderungen, optional die Anzahl der gewünschten besten Treffer und optional eine Dimensionsangabe.
- Beim Anlegen von Parametern müssen Trefferanzahl und Dimension zunächst nicht gesetzt sein.
- Parameter müssen unveränderlich, auf Gleichheit vergleichbar und als Schlüssel für Zwischenspeicher oder Abhängigkeitsgraphen verwendbar sein.
- Das System muss eine Operation als reine Beschreibung aus Art und Parametern darstellen können. Diese Beschreibung muss ebenfalls vergleichbar und als Schlüssel verwendbar sein.

## Werte und Daten

- Jede Eingabe und Ausgabe einer Operation muss entweder Daten im Hauptspeicher oder eine Belegung eines Geräte-Speicherbereichs sein. Rohe Tensoren oder Speicherzeiger dürfen nie weitergegeben werden.
- Hauptspeicher-Daten müssen in folgenden Formen vorliegen können (erweiterbar):
  - Liste von Texten
  - Liste von Tokenfolgen
  - Folge von Gleitkommazahlen
  - Liste von Treffern aus Eintragskennung und Bewertung
  - Bewertetes Trefferergebnis
- Ein bewerteter Treffer muss enthalten:
  - die Kennung des Eintrags auf Verwaltungsebene
  - die Bewertung
  - die Angabe, ob der Treffer veraltet ist
  - optional einen Rückverweis auf die Quelle
- Ein bewertetes Trefferergebnis muss enthalten:
  - die Anzahl der bewerteten Einträge im aufgelösten Suchbereich
  - die Anzahl der Treffer, die schon bei der Auflösung verworfen wurden, weil der Eintrag entfernt wurde
  - die Liste der verbleibenden Treffer
- Dieses Ergebnis ist ein Zwischenergebnis. Die Rangbildung und Zusammenfassung erfolgen erst danach durch die aufrufende Schicht.

## Aufgabenkontext (Task)

- Das System muss pro Aufgabe einen gemeinsam nutzbaren Kontext führen. Er enthält Kennung, Priorität, Zähler offener Operationen, Abbruchkennzeichen und den ersten aufgetretenen Fehler.
- Bei der Erstellung muss die Anzahl der anfangs offenen Operationen angegeben werden. Das Abbruchkennzeichen ist zunächst nicht gesetzt, und es ist noch kein Fehler vermerkt.
- Das System muss Kennung, Priorität, Anzahl offener Operationen, Abbruchstatus und den ersten Fehler abfragbar machen.
- Das System muss das Registrieren zusätzlicher Folgeoperationen erlauben. Die Registrierung muss vor dem Einreihen der Folgeoperationen und vor dem Abschluss der erzeugenden Operation erfolgen, damit der Zähler nicht vorzeitig null erreicht.
- Beim Abschluss einer Operation muss das System den Zähler offener Operationen um eins verringern. Es muss melden, ob dies die letzte offene Operation der Aufgabe war und die Aufgabe nicht abgebrochen ist. Nur dann gilt die Aufgabe als erfolgreich abgeschlossen.
- Der Abschluss einer Operation ohne offene Operation gilt als Programmierfehler und muss in Entwicklungsumgebungen erkannt werden.
- Beim Fehlschlagen einer Aufgabe muss das System die Aufgabe als abgebrochen markieren und nur den ersten Fehler behalten. Spätere Fehler dürfen ihn nicht überschreiben.
- Das System muss zurückmelden, ob der Aufruf der erste Fehler war. Beim ersten Fehler muss es den Abbruch zu Diagnosezwecken protokollieren.
- Alle Zustände des Aufgabenkontexts müssen nebenläufig korrekt gelesen und verändert werden können.

## Operationsknoten und Verkettung

- Das System muss eine Operation in einem Knoten zusammenführen. Der Knoten enthält:
  - die Operation
  - ihre Eingabe
  - ihre Ausgabe, die zunächst leer ist
  - ein Kennzeichen, ob das Ergebnis in den Hauptspeicher zurückgegeben wird oder als Geräte-Belegung auf dem Gerät bleibt
  - die zugehörige Aufgabe
  - optional einen Nachfolger
- Beim Anlegen eines Knotens muss das System Aufgabenkennung, Operationsart, Ressourcenart und Materialisierungskennzeichen protokollieren.
- Das System muss das nachträgliche Anfügen eines Nachfolgers an einen Knoten erlauben.
- Die Ausgabe eines Knotens darf nur einmal gesetzt werden. Bei einem zweiten Versuch muss das System den Wert unverändert an den Aufrufer zurückgeben, statt die Ausgabe zu überschreiben.
- Ein Nachfolger muss aus der Ausgabe einer abgeschlossenen Operation und dem Aufgabenkontext null oder mehr Folgeknoten erzeugen.
- Liefert der Nachfolger keine Folgeknoten, endet die Kette, und die Ausgabe wird zum Ergebnis der Aufgabe.
- Nachfolger müssen nebenläufig verwendbar sein.

## Ressourcenanforderungen

- Das System muss Modelle, Indizes und Geräte über eigene Kennungen identifizieren. Modell und Index werden durch einen Text bezeichnet, ein Gerät durch eine Zahl.
- Das System muss folgende Ressourcenarten unterscheiden (erweiterbar): Tokenisierer, Normalisierer, Encoder, Bewerter, Index und Kreuz-Encoder.
- Das System muss als Ausführungsumgebung drei Auswahlen kennen: nur Prozessor, nur Grafikprozessor oder beliebig (erweiterbar).
- Die Auswahl „beliebig" muss jede Geräteart akzeptieren. Die Auswahl „nur Prozessor" und die Auswahl „nur Grafikprozessor" müssen jeweils nur die passende Geräteart akzeptieren.
- Das System muss einen Platzierungshinweis (Affinität) unterstützen, der sich entweder auf einen konkreten Speicherbereich oder auf ein bestimmtes Gerät bezieht (erweiterbar). Der Hinweis wird derzeit nicht ausgewertet.
- Die Anforderungen einer Operation müssen enthalten:
  - die Ressourcenart
  - optional ein Modell
  - optional einen Index
  - die Auswahl der Ausführungsumgebung
  - eine Menge geforderter Fähigkeiten
  - optional einen Platzierungshinweis
- Beim Anlegen werden nur Ressourcenart und Ausführungsumgebung verlangt. Modell, Index und Platzierungshinweis sind zunächst nicht gesetzt, und die Menge der Fähigkeiten ist leer.
- Das System muss Modell, Index, Fähigkeiten und Platzierungshinweis nachträglich einzeln ergänzen lassen.
- Anforderungen müssen auf Gleichheit vergleichbar und als Schlüssel verwendbar sein.
- Das System muss prüfen können, ob ein Gerät die Anforderungen erfüllt. Das gilt genau dann, wenn die Geräteart von der gewählten Ausführungsumgebung akzeptiert wird und die verfügbaren Fähigkeiten des Geräts alle geforderten Fähigkeiten abdecken.
- Der Platzierungshinweis darf bei dieser Prüfung bewusst keine Rolle spielen.
