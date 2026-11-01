## Task

### Aufbau, Zustände und Ergebnis

- Ein Task kapselt Daten, Parameter, Ergebnis, Zustand und Operationen. Die Operationen verweisen zurück auf ihren Task.
- Priorität, kompatible Ressource, Modell und Modus sind gewöhnliche Parameter (Metadaten). Die Queue kennt sie nicht.
- Der Expander zerlegt jeden Task in eine Kette von Operationen.
- Zustände eines Tasks: enqueued, claimed, completed, cancelled.
- Das Ergebnis wird genau einmal gesetzt und ist entweder ein Output oder ein Fehler. Wartende werden über eine Notification geweckt.

### Task-Handle und Aufrufer

- Der Zustand ist ohne Warten abfragbar (Polling):
  - completed: das Ergebnis, also Output oder Fehler,
  - cancelled: der Fehler „cancelled“,
  - enqueued oder in Bearbeitung: „noch kein Ergebnis“,
  - completed ohne verfügbares Ergebnis: gilt als cancelled.
- Der Aufrufer kann unbegrenzt oder mit Timeout auf den Abschluss warten. Er wartet auf das Ergebnissignal und wacht zusätzlich periodisch auf, um den Zustand zu prüfen.
- Reihenfolge beim Warten: erst die Registrierung für die Notification aktivieren, dann den Zustand lesen, dann warten. So geht keine Notification verloren.
- Der Timeout ist die eigene Deadline des Aufrufers. Bei Überschreitung wird der Fehler „timeout“ geliefert und das Handle freigegeben.
- Wird ein unfertiges Handle gedroppt, wird der Task abgebrochen, sofern ihn noch kein Worker übernommen hat. Übernommene Tasks laufen weiter. Der Zustand „cancelled“ wird lokal abgeleitet.

### Erweiterbare Zusatzparameter

- Das Parameterobjekt jeder Task-Art ist erweiterbar und nimmt beliebige zusätzliche, typgebundene Werte auf (typindizierte Extension-Map).
- Je Typ ist höchstens ein Wert gespeichert. Erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
- Ein Wert ist nach Typ abfragbar (sonst „nicht vorhanden“) und nach Typ entfernbar, mit Rückmeldung, ob er vorhanden war.
- Anzahl der Werte und Leere der Map sind abfragbar.
- Beim Klonen der Parameter werden die Werte geteilt (shared) statt dupliziert, damit abgeleitete Tasks (etwa von Addons) die Parameter günstig übernehmen.
- Die Werte werden unverändert durch Queue und Planung gereicht und sind thread-safe übertragbar (Send + Sync).

### Embedding-Task

- Input: ein Text oder eine Liste von Texten.
- Parameter: Modell, Modus (Query oder Passage), Precision, optionaler Ziel-Index, erweiterbare Zusatzparameter.
- Ergebnis: die Embeddings als Liste von Float-Vektoren, ein Vektor je Eingabetext in Eingabereihenfolge.
- Ein Embedding- bzw. Indexing-Task kann so konfiguriert werden, dass er das Embedding samt Metadaten direkt in den Index schreibt. Er liefert dann Entry-Referenzen bzw. IDs des Index.

### Similarity-Task

- Input: eine Liste von Query-Vektoren. Eine Textanfrage wird als Folge aus einem Embedding-Task im Query-Modus und einem Similarity-Task mit dem Ergebnisvektor ausgedrückt, gesteuert durch die Anwendung oder ein Addon.
- Parameter:
  - ID des zu durchsuchenden Index (verpflichtend), daraus abgeleitet das Modell,
  - maximale Trefferzahl (Top-k, Default zehn),
  - Scope (Suchbereich), Default ist der gesamte Index, einschränkbar auf eine Teilmenge,
  - Scoring-Verfahren, Default ist Cosine Similarity,
  - Precision und erweiterbare Zusatzparameter.
- Ein Reranking der Treffer ist keine eigene Task-Art. Es wird als zweite Similarity mit anderem Scoring-Verfahren auf einem verengten Scope ausgedrückt, dessen Kandidaten aus einem früheren Ergebnis stammen.
- Ergebnis: Entry-Referenzen oder IDs (Chunk, Zeile, Bereich, Datei oder Index) mit Score. Nach außen erscheint jeder Treffer mit Entry, Score und Markierung „stale“.

### Ergebnisarten

- Das Ergebnis eines Tasks ist genau eine Ergebnisart:
  - Embeddings als Liste von Float-Vektoren,
  - Entry-Referenzen bzw. IDs beim direkten Schreiben in den Index,
  - Treffer als Liste von Paaren aus Entry-ID und Score,
  - ein Similarity-Report,
  - eine Liste von Scores (etwa aus einer Cross-Encoder-Inferenz).
- Die Menge der Ergebnisarten ist erweiterbar. Ergebnisse sind auf Gleichheit vergleichbar.

### Similarity-Report

- Der Report liefert rohe Scores und die Fakten, anhand derer eine äußere Schicht über Folge-Tasks entscheidet. Er enthält Task-ID und Suchparameter.
- Inhalt:
  - Anzahl der Kandidaten, also der tatsächlich gescorten Entries im aufgelösten Scope,
  - Anzahl der verworfenen Entries,
  - Anzahl der stale Entries unter den zurückgegebenen Treffern,
  - Rangliste der Treffer mit Entry-ID, Score, Flag „stale“ und optionalem Source-Link.
- Aufbau:
  - Eingehende Treffer dürfen unsortiert sein und mehr Einträge enthalten als die maximale Trefferzahl.
  - Treffer ohne gültigen Zahlenwert werden verworfen und zur Zahl der upstream verworfenen Entries addiert, etwa wenn ein Entry aus dem Index entfernt wurde.
  - Sortierung absteigend nach Score, bei Gleichstand steht die niedrigere Entry-ID vorn. Danach wird auf Top-k gekürzt.
  - Das Flag „stale“ wird je Treffer übernommen und nur über die zurückgegebenen Treffer gezählt.
  - Der Source-Link wird je zurückgegebenem Treffer über eine vom Aufrufer gelieferte Lookup-Funktion ermittelt und kann fehlen.
  - Fehlen die Suchparameter, endet der Task mit einem Fehler.
- Kennzahlen beziehen sich auf die zurückgegebenen Treffer. Ohne Treffer sind sie nicht vorhanden:
  - Anzahl, bester und schlechtester Score, arithmetisches Mittel, Standardabweichung der Grundgesamtheit.
  - Abstand zwischen erstem und zweitem Treffer (Gap). Ein kleiner Abstand zeigt eine unsichere Top-Reihenfolge an. Mindestens zwei Treffer nötig.
  - Abstand zwischen erstem und letztem Treffer als Maß der Streuung (Spread). Mehr als ein Treffer nötig.
  - Abstand des besten Scores vom Mittelwert in Standardabweichungen (z-Score). Bei einer Standardabweichung nahe null nicht vorhanden.
  - Wahrscheinlichkeitsverteilung über die Scores per Softmax mit einstellbarer Temperatur. Die Temperatur muss endlich und größer als null sein, sonst ist das Ergebnis nicht vorhanden. Die Berechnung ist numerisch stabil, indem sie auf den besten Score bezogen wird. Für die verwendete Modellfamilie liegen Scores in einem engen Band, daher ist eine Temperatur von etwa 0,01 bis 0,05 sinnvoll.
  - Wahrscheinlichkeitsmasse des besten Treffers.
  - Entropie der Verteilung in Nats. Null bedeutet einen eindeutigen Gewinner.
  - Knee-Point der sortierten Score-Kurve: der Treffer mit dem größten Abstand unterhalb der Verbindungslinie vom ersten zum letzten Wert. Mindestens drei Treffer nötig. Bei flacher Kurve oder ohne Punkt unterhalb der Linie nicht vorhanden. Bei Gleichstand gewinnt der frühere Treffer.

