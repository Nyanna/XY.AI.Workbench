# Funktionale Anforderungen Planer

## 1. Unterstützte Aufgabenarten

- Das System muss zwei Aufgabenarten unterstützen: Texteinbettung, Ähnlichkeitssuche.
- Das System muss jede Aufgabe anhand ihrer Art, ihrer Kennung und ihrer Nutzdaten verarbeiten.
- Das System muss für jede Aufgabenart festlegen, welche Ressourcenart die erste Operation benötigt. Bei der Ähnlichkeitssuche ist es die Bewertungsressource, bei Einbettung und Inferenz die Textzerlegungsressource.
- Das System muss diese Information vorab abfragbar machen, bevor die Aufgabe geplant wird.

## 2. Grundprinzip der Planung

- Das System muss jede Aufgabe genau einmal in eine geordnete Kette von Operationen zerlegen.
- Das Ergebnis der Planung darf ausschließlich von den Aufgabenparametern, der Konfiguration, den Fähigkeiten der Rechenseiten und dem Versorgungshinweis abhängen, nicht von den Nutzdaten.
- Die Planung muss zustandslos und wiederholbar sein: Gleiche Eingaben ergeben dieselbe Operationskette.
- Das System muss jede Operation mit Operationsart, benötigter Ressourcenart, gewählter Rechenseite, zugehörigem Modell und gegebenenfalls zugehörigem Index beschreiben.
- Das System muss optional eine Zieldimension und eine Trefferanzahl an die Operation weitergeben.
- Das System muss eine Aufgabe ablehnen, wenn der zuständige Planer eine Aufgabe einer anderen Art erhält. Die Fehlermeldung muss beide Aufgabenarten nennen.
- Das System muss die Aufgabenart und die Anzahl der geplanten Operationen auf Diagnose-Ebene protokollieren.

## 3. Schrittweise Übergabe der Operationen

- Das System muss zunächst nur die erste Operation der Kette zur Ausführung bereitstellen.
- Nach Abschluss einer Operation muss das System die nächste Operation erzeugen. Deren Eingabe ist das tatsächliche Ausgabeergebnis der vorigen Operation.
- Jede bereitgestellte Operation muss dem Kontext der ursprünglichen Aufgabe zugeordnet bleiben.
- Das System muss nach der letzten Operation keine weitere Operation mehr bereitstellen.
- Liefert die letzte Operation ein Ergebnis auf der Hauptspeicherseite, muss das System es als Gesamtergebnis der Aufgabe speichern.
- Liefert die letzte Operation kein solches Ergebnis, bleibt das Gesamtergebnis leer.
- Ergibt die Planung eine leere Kette, muss das System keine Operation bereitstellen. Die Aufgabe endet dann beim Einsammeln mit einem Fehler wegen fehlenden Ergebnisses.
- Das System muss zusammen mit der ersten Operation eine Sammelfunktion liefern. Diese erzeugt nach Abschluss der Kette die endgültige Ausgabe.
- Die Sammelfunktion muss das gespeicherte Ergebnis dabei entnehmen, sodass es nur einmal verfügbar ist.
- Das System muss die Planung wahlweise mit oder ohne Versorgungshinweis anbieten. Ohne Hinweis gilt ein leerer Hinweis.

## 4. Aufbereitung der Eingabedaten

- Bei Einbettungsaufgaben muss das System die Liste der Texte als Eingabe der ersten Operation verwenden und die Anzahl der Texte für die spätere Ergebnisaufteilung merken.
- Bei Ähnlichkeitsaufgaben muss das System alle Anfragevektoren zu einer einzigen flachen Folge von Fließkommazahlen zusammenfassen. Die Anzahl der Einträge gilt als eins.
- Bei Inferenzaufgaben muss das System Anfrage und Textabschnitt als geordnetes Paar von Texten übergeben. Die Anzahl der Einträge gilt als eins.

## 5. Aufbereitung der Ergebnisse

- **Einbettung:**
  - Das System muss die flache Zahlenfolge in einen Vektor je Text aufteilen. Die Vektorlänge ergibt sich aus der Gesamtzahl der Werte geteilt durch die Anzahl der Texte.
  - Das System muss die Aufgabe mit einem Fehler beenden, wenn die Textanzahl null ist oder die Wertanzahl nicht ohne Rest durch die Textanzahl teilbar ist. Die Fehlermeldung nennt Wert- und Textanzahl.
- **Ähnlichkeitssuche, einfache Trefferliste:** Das System muss die Treffer als Liste von Paaren aus Eintragskennung und Wert ausgeben.
- **Ähnlichkeitssuche, bewertete Treffer:**
  - Das System muss einen Ähnlichkeitsbericht erzeugen.
  - Der Bericht enthält die Aufgabenkennung und die Suchparameter.
  - Er enthält die Anzahl der betrachteten Kandidaten und die Trefferliste mit Eintragskennung, Wert und Kennzeichen, ob der Eintrag veraltet ist.
  - Er enthält die Anzahl der verworfenen Kandidaten.
  - Zu jedem Treffer, der eine Quellenangabe besitzt, muss der Bericht diese Quelle auffindbar machen. Treffer ohne Quellenangabe werden ohne Quelle geführt.
  - Fehlen die Suchparameter, muss das System die Aufgabe mit einem Fehler beenden.
- **Inferenz:** Das System muss die Fließkommazahlen als Bewertungen ausgeben.
- **Unpassende Ergebnisse:**
  - Liefert die Kette ein Ergebnis, das zur Aufgabenart nicht passt, muss das System mit einem Fehler antworten. Die Meldung nennt die Aufgabenart.
  - Liefert die Kette gar kein Ergebnis, muss das System mit einem Fehler antworten, der die fehlende Ausgabe benennt.
- Alle Ergebnisfehler müssen als Konfigurationsfehler der Planung gemeldet werden.

## 6. Operationsketten je Aufgabenart

### Einbettung
- Das System muss die Kette in dieser Reihenfolge aufbauen: Text zerlegen, Einbettung berechnen, Vektoren zusammenfassen, Vektoren normalisieren, in einen Index schreiben.
- Die Einbettungsoperation muss die konfigurierte Zieldimension übernehmen, sofern eine gesetzt ist.
- Der Schritt zum Zusammenfassen entfällt, wenn das Zusammenfassungsverfahren „keines“ konfiguriert ist. Die Vektoren der einzelnen Token bleiben dann unverändert.
- Der Schritt zum Normalisieren entfällt, wenn die Normalisierung abgeschaltet ist. Er normalisiert auf die euklidische Länge eins.
- Der Schritt zum Schreiben in einen Index entfällt, wenn die Aufgabe keinen Zielindex angibt. Andernfalls muss die Operation den Zielindex referenzieren.
- Zerlegung, Einbettung, Zusammenfassung und Normalisierung brauchen jeweils eine passende Ressource des Modells. Die Zerlegung braucht die Textzerlegungsressource, die übrigen Schritte brauchen die Einbettungsressource. Das Schreiben braucht die Indexressource.

### Ähnlichkeitssuche
- Das System muss die Kette aus zwei Operationen aufbauen: zuerst die Ähnlichkeitsberechnung, danach die Auswahl der besten Treffer.
- Beide Operationen müssen die Bewertungsressource des Modells anfordern und den in der Aufgabe genannten Index referenzieren.
- Die Ähnlichkeitsberechnung muss die konfigurierte Zieldimension übernehmen.
- Die Auswahl muss die gewünschte Trefferanzahl übernehmen. Übersteigt sie den darstellbaren Höchstwert eines vorzeichenlosen 32-Bit-Zählers, muss sie auf diesen Höchstwert begrenzt werden.
- Das System muss die Aufgabe mit einem Fehler „Bewerter nicht verfügbar“ ablehnen, wenn ein Cross-Encoder-Bewerter angefordert wird. Die Meldung nennt die Bezeichnung des Bewerters.
- Das System muss die Aufgabe mit einem Konfigurationsfehler ablehnen, wenn die gewünschte Trefferanzahl null beträgt.

## 7. Wahl der Rechenseite und Zwischenergebnisse

- Das System muss pro Rechenseite (Hauptprozessor und Grafikprozessor) verwalten, welche Ressourcenarten sie anbietet. Die Angebote müssen schrittweise erweiterbar sein.
- Das System muss für jede Operation festlegen, ob ihr Ergebnis in den Hauptspeicher übernommen wird. Das Ergebnis bleibt nur dann auf dem Grafikprozessor, wenn diese Operation und die unmittelbar folgende beide ausdrücklich auf dem Grafikprozessor laufen. In allen anderen Fällen, auch bei der letzten Operation und bei freier Seitenwahl, muss das System das Ergebnis in den Hauptspeicher übernehmen.

## 9. Modellauswahl

- Bei Einbettungs- und Ähnlichkeitsaufgaben muss das System immer das Standardmodell verwenden.
- Ist das gewählte Modell nicht konfiguriert, oder ist für die Kombination aus Modell und Aufgabenart kein Planer vorhanden, muss das System mit „unbekanntes Modell“ abbrechen. Die Meldung nennt das Modell.
- Das gewählte Modell und seine Konfiguration müssen dem Planer zusammen mit den Rechenseiten-Angeboten und dem Versorgungshinweis bereitstehen.