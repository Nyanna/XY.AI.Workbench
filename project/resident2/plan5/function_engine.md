# Funktionale Anforderungen Engine

## Konfiguration und Start

- Das System muss über einen schrittweisen Konfigurationsvorgang erzeugt werden, der erst mit einem expliziten Startbefehl wirksam wird.
- Die Angabe eines Modells ist verpflichtend. Fehlt sie, muss der Start mit einem Konfigurationsfehler scheitern, der das fehlende Feld benennt.
- Modell- und Geräteressourcen dürfen nicht beim Start erzeugt werden. Sie werden erst durch die erste Aufgabe angelegt.

## Verwaltung von Indizes

- Das System muss das Registrieren beliebig vieler durchsuchbarer Indizes ermöglichen.
- Das Registrieren darf den Index nicht laden. Das Laden wird erst durch eine Aufgabe ausgelöst.
- Jeder registrierte Index erhält eine eindeutige Kennung.
- Unmittelbar nach der Registrierung gilt ein Index als nicht geladen.

## Einreichen von Aufgaben

- Das System muss zwei Aufgabenarten anbieten: Texteinbettung, Ähnlichkeitssuche.

## Tasks Texteinbettung

- Eingabe sind ein oder mehrere Texte. Es muss möglich sein, eine Liste oder einen einzelnen Text anzugeben.
- Zusätzlich sind eine Genauigkeitsangabe und frei erweiterbare Zusatzparameter möglich.

## Tasks Ähnlichkeitssuche

- Eingabe ist eine Liste von Zahlenvektoren, mit denen gesucht wird.
- Die Kennung des zu durchsuchenden Index ist verpflichtend.
- Die maximale Trefferzahl ist einstellbar. Der Standardwert ist zehn.
- Der Suchbereich ist einstellbar. Standard ist der gesamte Index. Der Bereich kann auf eine Teilmenge der Einträge eingeschränkt werden.
- Das Bewertungsverfahren ist wählbar:
  - Standard ist der Kosinus-Ähnlichkeitswert.
- Eine Neubewertung der Treffer (Reranking) ist keine eigene Aufgabenart. Sie wird als zweite Ähnlichkeitssuche mit anderem Bewertungsverfahren auf einem verengten Suchbereich ausgedrückt.
- Zusätzlich sind eine Genauigkeitsangabe und frei erweiterbare Zusatzparameter möglich.

## Ergebnisarten

Das Ergebnis einer Aufgabe ist genau eine der folgenden Arten:
- Einbettungen als Liste von Zahlenvektoren,
- Treffer als Liste von Paaren aus Eintragskennung und Bewertungswert,
- ein Ähnlichkeitsbericht,
- eine Liste von Bewertungswerten.

Ergebnisse müssen vergleichbar sein, damit sie sich auf Gleichheit prüfen lassen.

## Erweiterbare Zusatzparameter

- Alle drei Parameterarten müssen beliebige zusätzliche, typgebundene Werte aufnehmen können.
- Je Typ ist höchstens ein Wert gespeichert. Ein erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
- Es muss möglich sein, einen Wert nach Typ abzufragen. Fehlt er, ist die Antwort "nicht vorhanden".
- Es muss möglich sein, einen Wert nach Typ zu entfernen. Die Rückmeldung gibt an, ob er vorhanden war.
- Die Anzahl der Werte muss abfragbar sein, ebenso ob die Sammlung leer ist.
- Beim Kopieren der Parameter müssen die Werte geteilt und nicht dupliziert werden, damit abgeleitete Aufgaben die Parameter günstig übernehmen.
- Die Werte müssen unverändert durch Warteschlange und Planung gereicht werden.
- Die Werte müssen sicher zwischen Ausführungssträngen weitergegeben werden können.

## Aufgabenhandle

- Der Zustand der Aufgabe muss ohne Warten abfragbar sein:
  - Ist die Aufgabe abgeschlossen, wird das Ergebnis geliefert, also Erfolg oder Fehler.
  - Ist die Aufgabe abgebrochen, wird der Fehler "abgebrochen" geliefert.
  - Ist die Aufgabe noch eingereiht oder wird gerade bearbeitet, wird "noch kein Ergebnis" gemeldet.
  - Ist die Aufgabe abgeschlossen, aber ohne verfügbares Ergebnis, gilt sie als abgebrochen.
- Der Aufrufer muss auf den Abschluss warten können, wahlweise unbegrenzt oder mit Zeitlimit.
  - Wird das Zeitlimit überschritten, muss der Fehler "Zeitüberschreitung" geliefert werden.
  - Der Handle wird dabei freigegeben. Dadurch wird die Aufgabe abgebrochen, sofern sie noch in der Warteschlange liegt.
- Wird ein noch unfertiger Handle freigegeben, muss die Aufgabe abgebrochen werden, sofern noch kein Arbeiter sie übernommen hat. Bereits übernommene Aufgaben laufen weiter.

## Ähnlichkeitsbericht

Der Bericht liefert rohe Bewertungswerte und die Fakten, anhand derer eine äußere Schicht über Folgeaufgaben entscheidet. Das System selbst trifft keine Entscheidungen und legt keine Schwellenwerte fest.

### Inhalt
- Anzahl der Kandidaten, also der tatsächlich bewerteten Einträge im aufgelösten Suchbereich.
- Anzahl der verworfenen Einträge.
- Anzahl der veralteten Einträge unter den zurückgegebenen Treffern.
- Rangliste der Treffer. Jeder Treffer enthält Eintragskennung, Bewertungswert, Kennzeichen "veraltet" und optional einen Quellverweis.

### Aufbau
- Die eingehenden Treffer dürfen ungeordnet sein und mehr Einträge enthalten als die maximale Trefferzahl.
- Treffer ohne gültigen Zahlenwert werden verworfen und zur Zahl der bereits vorgelagert verworfenen Einträge addiert, etwa wenn ein Eintrag aus dem Index entfernt wurde.
- Die Treffer werden absteigend nach Bewertung sortiert. Bei Gleichstand steht der Eintrag mit der niedrigeren Kennung vorn.
- Die Liste wird auf die maximale Trefferzahl gekürzt.
- Das Kennzeichen "veraltet" wird je Treffer übernommen und nur über die zurückgegebenen Treffer gezählt.
- Der Quellverweis wird je zurückgegebenem Treffer über eine vom Aufrufer gelieferte Nachschlagefunktion ermittelt. Er kann fehlen.

### Kennzahlen
Alle Kennzahlen beziehen sich auf die zurückgegebenen Treffer. Ohne Treffer sind sie nicht vorhanden.
- Anzahl der Treffer.
- Bester und schlechtester Bewertungswert der Liste.
- Arithmetischer Mittelwert.
- Standardabweichung der Grundgesamtheit.
- Abstand zwischen dem ersten und dem zweiten Treffer. Ein kleiner Abstand zeigt an, dass die Reihenfolge der Spitzenreiter unsicher ist. Es sind mindestens zwei Treffer nötig.
- Abstand zwischen erstem und letztem Treffer als Maß für die Streuung der Liste. Es sind mehr als ein Treffer nötig.
- Abstand des besten Werts vom Mittelwert in Standardabweichungen. Bei einer flachen Verteilung, also einer Standardabweichung nahe null, ist die Kennzahl nicht vorhanden.
- Wahrscheinlichkeitsverteilung über die Bewertungen mit einstellbarer Temperatur (Softmax):
  - Die Temperatur muss endlich und größer als null sein, sonst ist das Ergebnis nicht vorhanden.
  - Die Berechnung muss numerisch stabil sein, indem sie auf den besten Wert bezogen wird.
  - Für die verwendete Einbettungsmodellfamilie liegen Bewertungen in einem engen Band, daher ist eine Temperatur von etwa 0,01 bis 0,05 sinnvoll.
- Wahrscheinlichkeitsmasse des besten Treffers.
- Entropie der Verteilung in natürlichen Einheiten. Null bedeutet einen eindeutigen Gewinner.
- Knickpunkt der sortierten Bewertungskurve:
  - Gesucht ist der Treffer mit dem größten Abstand unterhalb der Verbindungslinie vom ersten zum letzten Wert.
  - Es sind mindestens drei Treffer nötig. Bei flacher Kurve oder ohne Punkt unterhalb der Linie ist die Kennzahl nicht vorhanden.
  - Bei Gleichstand gewinnt der frühere Treffer.

## Beenden

- Das System muss ein geordnetes Beenden anbieten, das auch beim automatischen Freigeben der Instanz abläuft.
- Beim Beenden werden zuerst die Arbeiter angehalten und zusammengeführt.
- Danach müssen alle Aufgaben abgebrochen werden, die kein Arbeiter übernommen hat. Wartende Aufrufer erhalten den Fehler "abgebrochen".
- Das explizite Beenden meldet bei Erfolg eine positive Rückmeldung.

## Fehlerverhalten

Das System muss Aufgabenfehler vom Konfigurationsfehler unterscheiden. Aufgabenfehler sind mindestens:
- abgebrochen,
- Zeitüberschreitung,
- ungültige Genauigkeit,
- fehlende Planungsfähigkeit,
- nicht verfügbares Bewertungsverfahren.

Ein Fehler einer einzelnen Aufgabe darf andere Aufgaben nicht beeinträchtigen.