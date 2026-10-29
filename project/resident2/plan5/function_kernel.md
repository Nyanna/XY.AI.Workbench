# Funktionale Anforderungen Kernel

## Unterstützte Rechengenauigkeiten je Ausführungsart

- Das System muss für jede Ausführungsart (Hauptprozessor, Grafikprozessor) festlegen, für welche Elementdatentypen ausführbare Bewertungsrechenkerne existieren. Diese Festlegung gilt unabhängig davon, ob die Unterstützung für Grafikprozessoren aktiviert ist.
- Auf dem Hauptprozessor muss ausschließlich der 32-Bit-Gleitkommatyp unterstützt werden.
- Auf dem Grafikprozessor müssen der 32-Bit- und der 16-Bit-Gleitkommatyp unterstützt werden.
- Für jeden unterstützten Elementdatentyp muss das System die Fähigkeitsachsen „Operanden“ und „Bewertung“ als verfügbar melden.
- Datentypen ohne Rechenkern dürfen nicht als Fähigkeit gemeldet werden.
- Die übrigen Fähigkeitsachsen (Akkumulator, Fusion, Stapelgröße, Kachelgröße, Threadanzahl) dürfen durch den Rechenkernkatalog nicht eingeschränkt werden.

## Anfragenstapel

- Das System muss mehrere Anfragen zu einem Stapel mit gemeinsamer Vektordimension bündeln. So wird jeder Dokumentabschnitt für alle Anfragen des Stapels nur einmal gelesen.
- Jede Anfrage ist eine zeilenweise abgelegte Matrix aus Tokens mal Dimension. Eine dichte Anfrage besteht aus genau einem Token.
- Beim Anlegen muss das System jede Anfrage prüfen. Die Dimension darf nicht null sein, die Anfrage nicht leer, und die Datenlänge muss ein ganzzahliges Vielfaches der Dimension sein. Andernfalls muss ein Formfehler mit erwarteter und tatsächlicher Größe gemeldet werden.
- Das System muss die Dimension, die Anzahl der Anfragen, die Information „Stapel leer“, die Tokenanzahl je Anfrage und die Rohdaten je Anfrage bereitstellen.

## Quelle der Dokumentvektoren

- Das System muss Dokumentvektoren über eine Quellenschnittstelle abrufen. Diese liefert die Dimension und die Anzahl der Slots je Segment. Außerdem liefert sie den Offset eines Slots, der in Trefferergebnissen ausgewiesen wird, und die Vektoren eines Slots (zeilenweise, nur gültige Tokens). Ein Slot ohne Vektoren muss als „nicht vorhanden“ ausgewiesen werden können.
- Das System muss eine Quelle im Arbeitsspeicher anbieten:
  - Sie nimmt je Segment eine Liste von Slots mit jeweils beliebig vielen Tokenvektoren entgegen.
  - Der Offset eines Slots ist die Summe der Bytegrößen aller vorangehenden Slots desselben Segments, wobei jeder Zahlenwert vier Byte belegt.
  - Ein unbekanntes Segment hat null Slots.
  - Ein nicht vorhandener Slot liefert keine Vektoren.
- Vor jeder Bewertung muss das System den angeforderten Slotbereich prüfen. Der Anfang darf nicht hinter dem Ende liegen, und das Ende darf die Slotanzahl des Segments nicht überschreiten. Andernfalls muss ein Fehler mit Segment, Bereichsgrenzen und tatsächlicher Slotanzahl gemeldet werden.

## Arbeitsbereiche (Slotbereiche)

- Ein Arbeitsbereich beschreibt einen halboffenen Slotbereich innerhalb eines Segments. Er ist die Einheit eines einzelnen Rechenkernstarts.
- Das System muss die Länge des Bereichs bereitstellen. Sie ist nie negativ, auch bei umgekehrten Grenzen. Außerdem muss es die Information „Bereich leer“ bereitstellen.
- Das System muss einen Bereich in einen vorderen Teil mit höchstens der gewünschten Anzahl Slots und einen Restbereich aufteilen können. Ist die gewünschte Anzahl größer als der Bereich, umfasst der vordere Teil den ganzen Bereich und der Rest ist leer.

## Dichte Bewertung (Kosinusähnlichkeit)

- Das System muss die Ähnlichkeit einer Anfrage zu einem Slot als Skalarprodukt der Vektoren berechnen. Dieser Standardmodus ist für Vektoren mit Einheitslänge gültig.
- Zusätzlich muss es einen exakten Modus geben. Dort wird das Skalarprodukt durch das Produkt beider Vektornormen geteilt. Er dient Fällen, in denen die Länge der abgelegten Vektoren abgewichen ist, etwa bei Speicherung in 16-Bit-Genauigkeit.
- Im exakten Modus muss der Wert null ausgegeben werden, wenn der Nenner nicht positiv oder nicht endlich ist.
- Jede Anfrage muss genau ein Token enthalten. Andernfalls muss ein Formfehler gemeldet werden.
- Eine Anfrage gilt als leer, wenn das Quadrat ihrer Norm nicht endlich ist oder unter 10⁻¹⁸ liegt (Norm kleiner als 10⁻⁹). Dann muss ein Fehler „leere Einbettung“ mit dem Index der betroffenen Anfrage gemeldet werden.
- Diese Prüfung muss für alle Anfragen vor Beginn der Bewertung abgeschlossen sein.
- Hat ein Slotvektor nicht die Dimension der Anfragen, muss ein Formfehler gemeldet werden. Das gilt auch für einen Slot ohne Vektoren.
- Das Ergebnis besteht je Anfrage aus einer Liste von Bewertungen, eine je Slot des Bereichs in Slotreihenfolge. Der zugehörige Offset kommt von der Vektorquelle.
- Das System muss Bewerter anbieten, die aus der dichten Bewertung (Standardmodus und exakter Modus) die besten Treffer einer vorgegebenen Anzahl liefern.

## Rechenprimitive auf dem Hauptprozessor

- Das System muss das Skalarprodukt zweier gleich langer Vektoren berechnen. Die Schleifen sollen für automatische Vektorisierung mit achtfach parallelen Teilsummen ausgelegt sein.
- Das System muss das Verfahren der späten Interaktion (Maximalähnlichkeit) anbieten:
  - Für jedes Token der Anfrage wird das größte Skalarprodukt über alle Dokument-Tokens ermittelt.
  - Diese Maxima werden über alle Anfrage-Tokens summiert.
  - Hat ein Dokument keine Tokens, erhält jedes Anfrage-Token den Maskenwert des für 16-Bit-Zahlen sicheren Bereichs. Die Bewertung ist dann Maskenwert mal Tokenanzahl der Anfrage. Dies entspricht dem Verhalten auf dem Grafikprozessor.
- Das System muss einen Vektor auf Einheitslänge normalisieren. Ein Vektor der Länge null bleibt unverändert.
- Das System muss einen slotweisen Durchlauf anbieten:
  - Er prüft zuerst den Slotbereich und danach, ob die Dimension der Anfragen mit der Dimension der Quelle übereinstimmt. Andernfalls meldet er einen Formfehler.
  - Die Slots bilden die äußere Schleife, sodass jeder Slot einmal gelesen und mit allen Anfragen bewertet wird.
  - Fehlen die Vektoren eines Slots, werden sie als leer behandelt.
  - Ist die Länge der Vektordaten eines Slots kein Vielfaches der Dimension, muss ein Formfehler gemeldet werden.
  - Fehler der Einzelbewertung brechen den Durchlauf ab.
  - Das Ergebnis ist je Anfrage eine Liste mit einer Bewertung je Slot.
- Das System muss einen Hauptprozessor-Bewerter für die späte Interaktion anbieten, der diesen Durchlauf nutzt.

## Operation für späte Interaktion

- Das System muss die Bewertung durch späte Interaktion als eine einheitliche Operation bereitstellen, die die besten Treffer einer vorgegebenen Anzahl je Anfrage liefert. Die zugrunde liegende Implementierung muss austauschbar sein.
- Die Operation muss mit beliebigen Slot-Bewertern aufgebaut werden können. Es muss eine Variante für den Hauptprozessor geben.
- Optional, wenn die Tensorbibliothek aktiviert ist, muss eine Variante mit Kachelkette angeboten werden. Sie läuft auf dem Hauptprozessor oder, bei zusätzlich aktivierter Grafikprozessorunterstützung, auf einem Grafikprozessor mit wählbarer Gerätenummer. Rechengenauigkeit und Kachelgröße (in Tokens) sind wählbar.
  - Lässt sich die Genauigkeit nicht abbilden oder das Gerät nicht öffnen, muss ein Fehler „nicht verfügbar“ mit Begründung gemeldet werden.
  - Eine Kachelgröße unter eins muss auf eins angehoben werden.
- Die Kachelkette muss je Dokument folgendermaßen arbeiten:
  - Die Dokument-Tokens werden in Kacheln fester Tokenanzahl zerlegt und transponiert.
  - Die letzte, unvollständige Kachel wird mit Nullen aufgefüllt. Die aufgefüllten Spalten werden über eine additive Maske mit dem Maskenwert ausgeschlossen.
  - Je Kachel wird das Matrixprodukt aus Anfrage und Kachel gebildet, gegebenenfalls maskiert, und je Anfrage-Token das Maximum bestimmt.
  - Über alle Kacheln wird ein laufendes Maximum geführt. Zum Schluss werden die Maxima in 32-Bit-Genauigkeit über die Anfrage-Tokens summiert.
  - Die Zwischenmatrizen dürfen höchstens Anfrage-Tokens mal Kachelgröße umfassen.
  - Ein Dokument ohne Tokens erhält Maskenwert mal Tokenanzahl der Anfrage.
- Die Anfragen müssen vor der Bewertung in die gewählte Rechengenauigkeit überführt werden.
- Die Kachelkette muss denselben Prüfungen unterliegen wie der Hauptprozessor-Durchlauf: Slotbereich, Dimensionsgleichheit und Vielfachheit der Dokumentvektorlänge. Fehler der Tensorbibliothek müssen als Fehler beim Rechenkernstart gemeldet werden.

## Auswahl der besten Treffer

- Das System muss je Anfrage eine begrenzte Menge der besten Treffer (Offset und Bewertung) verwalten.
- Ein höherer Wert gewinnt. Bei gleichem Wert gewinnt der niedrigere Offset, sodass das Ergebnis deterministisch ist.
- Treffer mit einem nicht-numerischen Wert müssen verworfen werden. Bei der Anzahl null wird nichts aufgenommen.
- Die Ausgabe muss absteigend sortiert sein (bester zuerst). Gibt es weniger Kandidaten als gewünscht, enthält sie entsprechend weniger Einträge.
- Das System muss die Anzahl der aktuell gehaltenen Treffer, die gewünschte Anzahl und die Information „leer“ bereitstellen. Treffer müssen einzeln oder als Folge aufgenommen werden können.
- Das System muss mehrere Teilergebnisse (zum Beispiel aus einzelnen Arbeitsbereichen) zu einem globalen Ergebnis mit der vorgegebenen Trefferzahl zusammenführen.
- Ein Bewerter für einen Arbeitsbereich muss die Rohbewertungen eines Slot-Bewerters übernehmen. Je Anfrage bildet er daraus die besten Treffer des Bereichs, mit dem Offset des jeweiligen Slots.
- Das System muss mehrere Arbeitsbereiche (zum Beispiel alle Bereiche eines Suchumfangs) nacheinander bewerten und je Anfrage zum globalen Ergebnis zusammenführen.

## Rechenkerne für Grafikprozessoren

- Das System muss, wenn die Grafikprozessorunterstützung aktiviert ist, zwei Rechenkerne bereitstellen. Der eine berechnet das dichte Skalarprodukt, der andere die späte Interaktion in fusionierter Form.
- Dokumentelemente müssen wahlweise als 32-Bit- oder 16-Bit-Gleitkommazahlen vorliegen können. 16-Bit-Werte müssen beim Laden in 32-Bit-Genauigkeit gewandelt werden. Anfragen und Ergebnisse sind stets 32-Bit.
- Der Rechenkern für das dichte Skalarprodukt muss je Slot einen Arbeitsblock einsetzen. Dieser bewertet jede Anfrage gegen den einzelnen Vektor des Slots, wobei die Teilsummen über Threads und Gruppen reduziert werden. Die Ausgabe hat die Form Anfragen mal Slots.
- Der Rechenkern für die späte Interaktion muss je Dokument einen Block einsetzen und genau eine Anfrage bewerten:
  - Dokumente liegen auf eine feste Tokenzahl aufgefüllt vor. Eine Längenliste gibt die gültige Tokenzahl je Dokument an, die auf die aufgefüllte Tokenzahl begrenzt wird.
  - Je Anfrage-Token wird das Maximum über die gültigen Dokument-Tokens gebildet. Hat ein Token kein gültiges Dokument-Token, erhält es den Maskenwert −10 000.
  - Die Maxima werden summiert, und das Ergebnis wird je Slot ausgegeben.
- Die Dimension muss bei der Übersetzung eines Rechenkerns festgelegt werden. Je Dimension und Elementtyp entsteht eine eigene Variante.
- Beim Start eines Rechenkerns:
  - Es werden 128 Threads je Block verwendet.
  - Es wird ein Block je Slot gestartet, mindestens aber einer.
  - Dynamischer gemeinsamer Speicher wird nicht angefordert.
  - Startfehler müssen als Fehler beim Rechenkernstart mit der Ursache gemeldet werden.

## Laufzeitübersetzung und Laden der Rechenkerne

- Das System muss Rechenkerne zur Laufzeit aus Quelltext übersetzen. Dem Quelltext wird eine gemeinsame Einleitung mit Typdefinition, Ladefunktion und Gruppenreduktionen vorangestellt, und der gewählte Rechenkern wird per Name ausgewählt.
- Die Zielarchitektur ergibt sich aus der Rechenfähigkeit des Geräts. Unterstützt werden die Stufen 61, 70, 75, 80, 86 und 89, jede unbekannte Stufe fällt auf 75 zurück.
- Lehnt der Treiber den Zwischencode ab, muss das System eine Warnung protokollieren. Danach muss es den Rechenkern erneut als fertigen Maschinencode für genau die Architektur des Geräts übersetzen und laden.
- Das System muss folgende Fehler unterscheiden:
  - Übersetzungsfehler mit Protokolltext des Übersetzers.
  - Fehler beim Laden des Moduls mit Begründung.
  - Unbekannter Rechenkernname oder Funktion nicht im Modul gefunden.
  - Nicht unterstützter Elementdatentyp, gemeldet als „nicht verfügbar“ mit Angabe des Typs.
- Bei der Einrichtung muss das System die Rechenfähigkeit des Geräts ermitteln und bereitstellen. Gelingt das nicht, muss es den Fehler „nicht verfügbar“ mit Begründung melden.
- Das System muss Rechenkernfunktionen anhand von Name, Elementtyp und Dimension (zusammen mit der Architektur des Geräts) bereitstellen.

## Rechenkernregister (Kernel Registry)

- Das System muss geladene Rechenkerne in einem Register halten, das über Name, Datentyp, Architektur und Variante (zum Beispiel die Einbettungsdimension) identifiziert.
- Rechenkerne müssen erst beim ersten Bedarf geladen werden. Gleichzeitige Anfragen nach demselben Schlüssel dürfen nie zu doppelter Übersetzung führen.
- Das Laden muss mit Name, Datentyp, Architektur, Variante und Dauer protokolliert werden.
- Das System muss die Anzahl geladener Einträge und die Information „leer“ bereitstellen.
- Es muss alle geladenen Rechenkerne verwerfen können und dabei die Anzahl der verworfenen Einträge zurückgeben. Der nächste Zugriff lädt neu.
- Das Register muss als verwaltbare Ressource unter einer festen Kennung auffindbar sein.

## Schnittstelle für Rechenkern-Stapelläufe

- Das System muss Segmente beschreiben können mit Basisadresse, Anzahl der Einträge und Layout. Das Layout ist entweder fest (mit gleicher Schrittweite) oder variabel (mit Offsettabelle aus Einträgen plus eins aufsteigenden Summen in einem Speicherbereich).
- Ein Stapel muss aus Segmenten, der Anzahl der Anfragen, einem optionalen Parameterbereich und einem optionalen Zeitbudget bestehen.
- Ein Rechenkern muss folgenden Lebenszyklus unterstützen:
  - Beginnen eines Stapels, der eine Kennung liefert.
  - Ausführen auf Eingabebereichen, das entweder Treffer (Offset und Wert) oder ein Teilergebnis liefert. Das Teilergebnis bedeutet, dass der zugewiesene beschreibbare Bereich erschöpft ist und die Verwaltung den nächsten zuweist und fortsetzt.
  - Zuteilen eines beschreibbaren Bereichs. Die erste Zuteilung erfolgt vor dem ersten Lauf, weitere folgen als Antwort auf ein Teilergebnis.
  - Abfragen des Fortschritts, der nie abnehmen darf.
  - Beenden des Stapels, das die geschriebenen Bereiche zurückgibt.
- Die öffentliche Schnittstelle darf keine Zeiger tragen. Die Auflösung von Adressen erfolgt nur innerhalb eines Backends.
- Das System muss aus einer Adresse den Eintrag bestimmen können:
  - Bei festem Layout ergibt sich der Index aus Abstand zur Basis geteilt durch die Schrittweite. Es gibt keinen Eintrag, wenn die Schrittweite null ist, die Adresse vor der Basis liegt oder der Index die Anzahl erreicht.
  - Bei variablem Layout deckt Eintrag i den Bereich von seinem Offset bis ausschließlich zum nächsten Offset ab. Adressen außerhalb ergeben keinen Eintrag.

## Zeitlich begrenzte Arbeitsscheiben

- Das System muss einen Arbeitsbereich in aufeinanderfolgende Scheiben zerlegen, die jeweils höchstens eine begrenzte Zeit dauern.
- Das System muss die Scheibenlänge so wählen, dass ein Start etwa der Zielzeit entspricht (Standard zwei Millisekunden). Grundlage ist der gemessene, exponentiell geglättete Durchsatz (Glättungsfaktor 0,3).
- Solange noch keine Messung vorliegt, muss die Mindestlänge verwendet werden. Die Länge muss immer zwischen Mindest- und Höchstlänge liegen (Standard 16 und rund eine Million Slots). Die Mindestlänge beträgt mindestens eins, die Höchstlänge mindestens die Mindestlänge.
- Das System muss den Durchsatz in Slots pro Sekunde bereitstellen. Vor der ersten Messung gibt es keinen Wert.
- Beobachtungen mit null Slots müssen ignoriert werden. Die gemessene Dauer wird nach unten auf eine Mikrosekunde begrenzt.
- Unterbrechungspunkte darf es nur an Scheibengrenzen geben. Nach jeder abgeschlossenen Scheibe, außer der letzten, muss geprüft werden, ob abzubrechen ist.
- Mindestens eine Scheibe muss immer vollständig ausgeführt werden, damit auch bei dauerhafter Verdrängung Fortschritt entsteht.
- Das System muss zwei Abbruchgründe unterscheiden: Das Zeitbudget ist ausgeschöpft, oder die Ausführung wird durch eine andere Arbeit verdrängt. Das Zeitbudget hat Vorrang.
- Das Ergebnis muss die Anzahl erledigter Slots enthalten, bei vorzeitigem Ende den verbleibenden Restbereich zum Fortsetzen und den Abbruchgrund. Bei vollständiger Bearbeitung gibt es weder Rest noch Grund.
- Ein Fehler bei der Ausführung einer Scheibe muss den Ablauf sofort beenden und weitergereicht werden.

## Startmodi für Rechenkerne

- Das System muss die Startart von Rechenkernen vom Bewerter trennen. Vorgesehen sind drei Modi: direkter Start, Aufzeichnung für spätere Wiederholung und ein dauerhaft laufender Rechenkern mit Zufuhr über einen gemeinsamen Ringpuffer.
- Verfügbar muss zunächst nur der direkte Start sein. Der direkte Start gibt den Arbeitsauftrag unverändert an die Ausführungsbahn weiter, Fehler der Bahn werden in Rechenkernfehler umgewandelt, und der Start wird auf feiner Protokollebene nachverfolgt.
- Die Anforderung der beiden anderen Modi muss mit der Meldung „nicht verfügbar“ und der Begründung abgelehnt werden, dass Graphaufzeichnung beziehungsweise der persistente Modus nicht aktiviert ist.
- Ein Start muss durch folgende Merkmale identifiziert werden: Operation, Formklasse, beteiligte Speicherbereiche, Datentyp, Variante und Klasse der Ausführungsbahn. Jeder Speicherbereich trägt eine Generation. Ändert sie sich, ergibt sich ein neuer Schlüssel, sodass veraltete aufgezeichnete Abläufe nie wiederholt werden.
- Die Formklasse ist die nächsthöhere Zweierpotenz der Ausdehnung, mindestens eins. Bei Überlauf gilt 2³¹. Dadurch bleibt die Zahl wiederholbarer Startvarianten klein.
- Vor dem ersten produktiven Start eines Schlüssels muss eine Aufwärmphase ausgeführt werden können. Der erste Start gilt nicht als produktiv, damit aufzeichnende Startarten dort Speicheranforderungen und verzögerte Treiberarbeit erfassen können. Ohne eigene Umsetzung geschieht in der Aufwärmphase nichts.

## Vergleich mit einem Referenzbewerter

- Das System muss die Ergebnisse eines zu prüfenden Bewerters mit denen eines Referenzbewerters vergleichen. Exakte Gleichheit darf nicht gefordert werden.
- Die Toleranz ist ein absoluter Wert: 10⁻⁵ für 32-Bit-Genauigkeit und 10⁻³ für 16-Bit-Genauigkeit.
- Der Vergleich muss drei Arten von Abweichungen melden:
  - Unterschiedliche Trefferanzahl, mit Anfrage, erwarteter und tatsächlicher Anzahl. Für diese Anfrage werden keine weiteren Ränge verglichen. Dasselbe gilt für unterschiedlich viele Anfragen insgesamt.
  - Wertabweichung über der Toleranz, mit Anfrage, Rang und Differenz.
  - Rangabweichung, wenn bei zulässiger Wertabweichung der Offset an einem Rang abweicht, mit Anfrage, Rang sowie erwartetem und tatsächlichem Offset.
- Eine Rangabweichung darf nicht gemeldet werden, wenn die Referenzwerte benachbarter Ränge innerhalb der Toleranz liegen (Beinahe-Gleichstand). Der Wert des Rangs wird dazu mit dem des Vorgängers und des Nachfolgers verglichen.
- Das System muss beide Bewerter über eine Liste von Arbeitsbereichen mit derselben Anfragenmenge ausführen und alle Abweichungen sammeln. Jede Abweichung wird als Warnung mit Segment protokolliert.
- Abweichungen dürfen nie zum Abbruch führen. Fehler der Bewerter selbst müssen weitergereicht werden.
