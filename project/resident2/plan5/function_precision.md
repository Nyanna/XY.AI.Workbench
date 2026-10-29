# Funktionale Anforderungen Precision

## 1. Bezeichner für Fähigkeitsachsen und Werte

- Das System muss Fähigkeitsachsen (zum Beispiel Operanden, Bewertung) und deren Werte (zum Beispiel Gleitkomma mit 16 Bit) über frei wählbare Zeichenketten identifizieren. Beliebige Zeichenketten sind gültig, die Menge ist erweiterbar.
- Achsen- und Wertbezeichner müssen untereinander vergleichbar, sortierbar, als Schlüssel verwendbar und kostengünstig kopierbar sein.
- Bezeichner müssen aus Text- und Zeichenkettenwerten erzeugbar sein und sich lesbar ausgeben lassen. Die Debug-Darstellung muss den Typ kenntlich machen.
- Ein Wertbezeichner muss sich aus einer natürlichen Zahl erzeugen lassen, damit numerische Achsen wie Stapelgröße, Kachelgröße oder Threadanzahl abbildbar sind.

## 2. Fähigkeitsmengen

- Eine Fähigkeitsmenge ordnet jeder Achse eine Menge von Werten zu. Sie beschreibt, was ein Anbieter leisten kann oder was eine Aufgabe verlangt.
- Eine Achse darf nie mit einer leeren Wertemenge vorkommen.
- Die Menge muss Werte hinzufügen, auf Enthaltensein prüfen, die Werte einer Achse liefern, alle Achsen und alle Achse-Werte-Paare aufzählen und auf Leere prüfen können.
- Aufzählung, Vergleich, Streuwertbildung und Serialisierung müssen reproduzierbar sein, also unabhängig von der Einfügereihenfolge.
- Eine Fähigkeitsmenge erfüllt eine Anforderung genau dann, wenn jeder geforderte Wert auf jeder geforderten Achse vorhanden ist. Eine leere Anforderung wird immer erfüllt.

## 3. Bekannte Achsen und Werte der Zielhardware

- Das System muss folgende Achsen vordefinieren: Operanden, Akkumulator, Bewertung, Fusion, Stapelgröße, Kachelgröße, Threadanzahl und Dichte.
- Es muss folgende Werte vordefinieren:
  - Gleitkomma mit 32 Bit
  - Gleitkomma mit 16 Bit
  - 16 Bit gespeichert, mit 32 Bit gerechnet
  - Ganzzahl mit 8 Bit
  - Ganzzahl mit 8 Bit über den Vier-Wege-Skalarprodukt-Befehl der Grafikkarte
  - Ganzzahl mit 8 Bit über die Ganzzahl-Befehle für neuronale Netze des Hauptprozessors
  - An und Aus
- Das Format Brain-Float mit 16 Bit wird bewusst nicht angeboten, weil es auf der Zielgeneration (Grafikkarte GTX 1660, Turing-Architektur) keinen nativen Pfad hat.
- Das System muss eine Referenzfähigkeitsmenge als Obergrenze für die Zielgrafikkarte bereitstellen:
  - Operanden und Akkumulator: 32 und 16 Bit.
  - Bewertung: 32 Bit, 16 Bit und 8-Bit-Skalarprodukt.
  - Fusion: an und aus.
- Numerische Achsen sind darin nicht enthalten, weil ihre Werte zur Laufzeit ermittelt werden.
- Es muss eine entsprechende Referenzmenge für den Hauptprozessor geben:
  - Operanden: 32 Bit sowie 16 Bit mit Umwandlung auf 32 Bit.
  - Bewertung: 32 Bit, 8 Bit und 8 Bit mit den Ganzzahl-Befehlen für neuronale Netze.

## 4. Fähigkeitserkennung

### Hauptprozessor

- Operanden, Akkumulator und Bewertung mit 32 Bit werden immer angeboten.
- Operanden mit 16 Bit und Umwandlung auf 32 Bit werden angeboten, wenn der Prozessor die Halbpräzisions-Umwandlung beherrscht.
- Bewertung mit 8 Bit wird angeboten, wenn die 256-Bit-Vektorerweiterung oder die Ganzzahl-Befehle für neuronale Netze vorhanden sind.
- Bewertung mit 8 Bit über diese Ganzzahl-Befehle wird nur angeboten, wenn sie vorhanden sind.
- Die Threadanzahlen sind 1, 2, 4 und so weiter in Zweierpotenzen bis zur Zahl der logischen Kerne. Die Kernzahl selbst ist stets enthalten, auch wenn sie keine Zweierpotenz ist. Doppelte Werte entfallen. Mindestens ein Kern wird angenommen.
- Die Kachelgrößen stammen aus der Konfiguration.
- Die Merkmale des Prozessors (Vektorerweiterung, Halbpräzisions-Umwandlung, Fused-Multiply-Add, Ganzzahl-Befehle für neuronale Netze) müssen vom echten System ermittelt werden. Auf fremden Prozessorarchitekturen gelten sie als nicht vorhanden.
- Die Anzahl der logischen Kerne muss vom System ermittelt werden, mit mindestens einem Kern als Rückfall.

### Grafikkarte

- 32 Bit wird immer angeboten.
- 16 Bit für Operanden, Akkumulator und Bewertung wird ab Rechenfähigkeitsstufe 5.3 angeboten.
- Bewertung mit dem 8-Bit-Skalarprodukt wird ab Stufe 6.1 angeboten.
- Fusion wird immer mit An und Aus angeboten.
- Stapelgrößen und Kachelgrößen kommen aus der Konfiguration.
- Weder ein Tensorkern-Pfad noch Brain-Float werden angeboten. Der Nutzen des 8-Bit-Skalarprodukts in einem echten Bewertungskern bleibt der Kalibrierung überlassen.
- Das System muss anhand von Stufe und Gerätename erkennen, ob Tensorkerne vorhanden sind. Karten der Baureihen TU116 und TU117 (zum Beispiel GTX 16xx, MX450, T400 bis T1200) haben trotz Stufe 7.x keine Tensorkerne. Das Ergebnis dient der Protokollierung.
- Ohne Grafikkartenunterstützung im Programmpaket oder ohne Gerätekontext muss die Erkennung mit einem Fähigkeitsfehler scheitern.

### Konfiguration der numerischen Achsen

- Stapelgrößen sind standardmäßig 1, 8 und 32.
- Kachelgrößen sind standardmäßig die Zweierpotenzen von 16 bis 512.
- Die maximale Sequenzlänge beträgt standardmäßig 512.
- Nur positive Stapelgrößen werden angeboten.
- Kacheln werden nur angeboten, wenn sie positiv und nicht größer als die maximale Sequenzlänge sind.
- Optional lässt sich die Menge verfügbarer Rechenkerne vorgeben. Fehlt sie, gilt der eingebaute Katalog der Teilmodul-Art.

### Abgleich mit vorhandenen Rechenkernen

- Erkannte Fähigkeiten werden auf Werte reduziert, für die ein Rechenkern existiert.
- Achsen, die der Kernkatalog nicht kennt, bleiben unverändert.
- Eine vom Katalog verwaltete Achse, die keinen Wert behält, entfällt vollständig.
- Entfernte Werte werden protokolliert.
- Das Ergebnis der Erkennung wird mit Teilmodul-Art, Achsenzahl und Gesamtzahl der Werte protokolliert.
- Die Auswahl des Erkennungsverfahrens erfolgt zentral anhand der Geräteart.

## 5. Synthetische Messlast

- Das System muss eine reproduzierbare, synthetische Messlast aus Dokumentvektoren und Anfragevektoren erzeugen. Die Erzeugung ist von einem Startwert abhängig, die Vektoren sind auf Länge eins normiert und duplikatfrei, weil Rangmetriken bei Gleichständen keinen Sinn ergeben.
- Standard sind 20 000 Dokumente, 384 Dimensionen, höchstens 32 Anfragen und die 5 besten Treffer für die Rangprüfung. Die Plausibilitätsgrenzen sind frei einstellbar.
- Als Referenz müssen die Bewertungen jeder Anfrage gegen jedes Dokument in 32-Bit-Genauigkeit auf dem Hauptprozessor berechnet werden.
- Die Anzahl der Anfragen einer Kombination folgt aus deren Stapelgröße.
  - Fehlt die Stapelgröße, gilt der Standard von 8, begrenzt auf die Höchstzahl.
  - Eine nicht numerische, null betragende oder über der Höchstzahl liegende Stapelgröße macht die Kombination unbrauchbar.
  - Kleinere Stapel verwenden die ersten Anfragen des größeren Stapels.
- Ein Ergebnis muss mit der passenden Referenz verglichen werden. Der Vergleich liefert die Abweichung und eine Gültigkeitsaussage, siehe Abschnitt 6.

## 6. Abweichungsmaße und Plausibilitätsprüfung

- Die Kosinus-Abweichung eines Vektors ist eins minus der Kosinus-Ähnlichkeit zur Referenz, mindestens null. Ist einer der Vektoren ein Nullvektor, beträgt sie eins.
- Abweichungen unterhalb von einem Millionstel gelten als Rundungsrauschen und werden als null behandelt.
- Der relative Fehler ist der Abstand zur Referenz geteilt durch deren Länge. Bei Nullreferenz ist er null bei Gleichheit, sonst unendlich.
- Die Treffer-Überlappung gibt den Anteil der besten k Referenztreffer an, die der Kandidat ebenfalls unter seinen besten k führt. Gleichstände werden nach Position aufgelöst, k wird auf die Länge begrenzt, bei k gleich null ist das Ergebnis eins.
- Die Gesamtbewertung einer Kandidatenmenge geschieht zeilenweise, mit der schlechtesten Zeile als Ergebnis. Eine Zeile bewertet sich aus der Kosinus-Abweichung und, falls gefordert, aus eins minus Überlappung, jeweils mit Rauschgrenze. Der Größere zählt.
- Das System muss folgende Gültigkeitszustände unterscheiden: gültig, nicht endliche Werte, entartet, Abweichung über dem Maximum und Ausführung gescheitert. Jeder Zustand braucht einen lesbaren Text.
- Ungleiche Längen, eine Zeilenlänge von null oder eine nicht auf die Zeilenlänge teilbare Datenmenge gelten als entartet mit unendlicher Abweichung.
- Enthält das Ergebnis nicht endliche Werte, gilt es als nicht endlich mit unendlicher Abweichung.
- Eine Zeile gilt als entartet, wenn sie konstant ist (Varianz kleiner als 10⁻⁹ des mittleren Quadrats) oder wenn das Längenverhältnis zur Referenz außerhalb von 0,5 bis 2,0 liegt. Die Grenzen sind konfigurierbar.
- Das System muss prüfen können, ob eine Menge duplikatfrei ist.

## 7. Messschnittstelle und Messmittel

- Der Kalibrierer erhält Abweichung und Zeit ausschließlich über eine austauschbare Messschnittstelle.
- Die Schnittstelle bietet zwei Operationen:
  - Abweichungsmessung: Die Kombination läuft einmal gegen die 32-Bit-Referenz, die Ausgabe wird beurteilt. Ist die Kombination unbrauchbar, meldet die Operation "unverträglich".
  - Zeitprobe: Eine Probe liefert entweder die Zeit pro Arbeitseinheit (bereits normiert) oder die Meldung "unterbrochen", wenn eine Produktivaufgabe das Gerät angefordert hat und nichts gemessen wurde.
- Die Implementierung wärmt sich beim ersten Gebrauch auf, skaliert die Wiederholungen auf das Zeitfenster und hält einzelne Starts innerhalb des Scheibenbudgets.
- Eine Kombination besteht aus einem Wert je Achse und den Sequenzlängen des Testsatzes. Die Sequenzlängen prüfen nur die Verarbeitbarkeit (Speicher, Überlauf), nicht die Qualität. Die Darstellung als Text besteht aus Achse-Wert-Paaren, durch Kommas getrennt.
- Ein aggregiertes Messergebnis enthält:
  - Zeit (robuster Schätzer),
  - Abweichung,
  - Gültigkeit,
  - Stabilitätsmarke (falsch, wenn Wiederholungs- oder Zeitobergrenze die Messung beendet haben),
  - Probenanzahl,
  - relative Unsicherheit (Zeiten innerhalb dieser Unsicherheit gelten als gleich).
- Die Abfrage, ob eine Produktivaufgabe wartet, muss austauschbar sein. Standard ist "nie unterbrechen". Ein gemeinsamer Schalter muss als Quelle dienen können.
- Die Zeitquelle muss austauschbar sein (monoton, Standard ist die Systemuhr), damit Budgets testbar sind.
- Das Zeitfenster beträgt standardmäßig 10 Millisekunden, das Scheibenbudget 5 Millisekunden.
- Die Auswahl des Messmittels erfolgt zentral anhand der Geräteart.
  - Für den Hauptprozessor wird die Messlast mit 4 000 Dokumenten erzeugt.
  - Für die Grafikkarte werden die Standardmaße verwendet.
  - Ohne Grafikkartenunterstützung im Programmpaket muss ein Fähigkeitsfehler gemeldet werden.

### Messung auf dem Hauptprozessor

- Bewertet wird der 32-Bit-Pfad. Aus einer Kombination werden nur Stapelgröße, Threadanzahl sowie Operanden- und Bewertungstyp gelesen.
- Jeder Operanden- oder Bewertungswert außer 32 Bit macht die Kombination unbrauchbar, weil dafür keine Rechenkerne existieren.
- Die Threadanzahl muss eine positive ganze Zahl sein, sonst ist die Kombination unbrauchbar. Fehlt sie, gilt ein Thread.
- Für jede Threadanzahl wird ein eigener, wiederverwendbarer Arbeitsthread-Pool angelegt. Scheitert das Anlegen, ist die Kombination unbrauchbar.
- Die Bewertung läuft parallel über die Dokumente. Jedes Dokument wird per Skalarprodukt gegen alle Anfragen bewertet.
- Für die Abweichungsmessung wird einmal gerechnet, das Ergebnis in Zeilen je Anfrage umgeordnet und gegen die Referenz beurteilt.
- Pro Kombination aus Threadanzahl und Anfragezahl wird einmal aufgewärmt.
- Eine Zeitprobe wiederholt vollständige Durchläufe, bis das Zeitfenster erreicht ist, und liefert die Zeit pro Durchlauf und Anfrage.
- Zwischen den Durchläufen wird auf Unterbrechungswünsche geprüft. Bei einem Wunsch wird "unterbrochen" gemeldet.

### Messung auf der Grafikkarte

- Gemessen wird der Kosinus-Bewertungspfad. Aus einer Kombination werden nur Stapelgröße (Anfragen je Start) und der Dokumenttyp gelesen, aus der Bewertungsachse, ersatzweise aus der Operandenachse. Ohne Angabe gilt 32 Bit.
- Erlaubt sind 32 und 16 Bit. Jeder andere Wert macht die Kombination unbrauchbar. Jeder Fehler der Grafikkarte wird ebenfalls als Unverträglichkeit gemeldet.
- Dokumente liegen in beiden Genauigkeiten auf dem Gerät vor.
- Die Zeit wird über Ereignisse auf dem Ausführungsstrom des Geräts gemessen, damit der Startaufwand nicht enthalten ist.
- Vor jeder Zeitprobe wird der Zwischenspeicher der Grafikkarte durch Überschreiben eines Speicherbereichs von 64 Mebibyte geleert.
- Vor der ersten Messung einer Kombination aus Typ und Anfragezahl wird mindestens 100 Millisekunden lang aufgewärmt, damit die Taktfrequenz den Leerlauf verlässt.
- Aus einem einzelnen gemessenen Start werden zwei Größen abgeleitet:
  - Die Wiederholungszahl entspricht dem Zeitfenster, mindestens 1 und höchstens 10 000.
  - Die Gruppengröße bleibt innerhalb des Scheibenbudgets, mindestens 1 und höchstens die Wiederholungszahl.
- Der Plan wird je Kombination aus Typ und Anfragezahl gemerkt.
- Die Wiederholungen laufen in Gruppen, weil ein laufender Kern nicht unterbrechbar ist. Zwischen den Gruppen wird auf Unterbrechungswünsche geprüft, wenn noch Gruppen ausstehen.
- Das Ergebnis ist die Zeit pro Start und Anfrage.
- Die Abweichungsmessung startet einmal, holt das Ergebnis zurück und beurteilt es.

## 8. Kalibrierung

### Zweck und Grundhaltung

- Der Kalibrierer muss die Kombinationen einer Fähigkeitsmenge aufzählen, verschränkt gegen die 32-Bit-Referenz messen, ungültige verwerfen und die Pareto-Front über Zeit und Abweichung ermitteln.
- Die Kalibrierung ist "bestmöglich" und darf nie blockieren. Obergrenzen beenden eine Messung mit dem besten Zwischenstand, der als instabil markiert wird.
- Maßstab: Position 0 ist die schnellste, Position 1 die präziseste Kombination.
- Ziel der Kalibrierung sind ein Teilmodul und eine Modellkennung.

### Konfiguration (Standardwerte in Klammern)

- Maximale Abweichung (0,05). Darüber gilt eine Kombination als unverträglich und wird nicht aktiviert.
- Mindestwiederholungen (3). Kleinere Vorgaben werden auf 3 angehoben.
- Zeitfenster (10 Millisekunden).
- Stabilitätsschwelle für die relative Streuung (0,05).
- Streuungsmaß: Variationskoeffizient oder Spannweite relativ zum Median (Variationskoeffizient).
- Schätzer: Median oder Minimum (Median).
- Harte Obergrenze je Kombination (40 Versuche, 30 Sekunden). Versuche zählen auch unterbrochene. Es gilt, was zuerst eintritt. Die Zeitgrenze ist optional.
- Scheibenbudget (5 Millisekunden).
- Gesamtbudget eines Laufs (600 Sekunden, optional). Ist es verbraucht, wird der Rest übersprungen.
- Dominanzspielraum (10 Prozent).
- Pause nach einer unterbrochenen Probe (5 Millisekunden), damit eine wartende Produktivaufgabe das Gerät erhält.
- Trennbare Achsen (Stapelgröße, Kachelgröße, Threadanzahl).
- Schalter für vollständige Messung des kartesischen Produkts als Referenz zum beschnittenen Ergebnis (aus).
- Sequenzlängen des Testsatzes.
  - Sie beginnen bei 16 und wachsen in Viererschritten, solange sie unter der Obergrenze bleiben.
  - Die Obergrenze selbst ist immer enthalten.
  - Standard ist eine Obergrenze von 512, also die Längen 16, 64, 256 und 512.

### Ablauf eines Laufs

- Eine leere Fähigkeitsmenge oder eine Menge nur mit leeren Achsen führt zu einem Fähigkeitsfehler.
- Achsen ohne Werte werden ignoriert.
- Die Werte jeder Achse werden geordnet: numerische aufsteigend nach Zahlenwert, danach die übrigen alphabetisch.
- Die Achsen werden in Kernachsen und trennbare Achsen geteilt.
- Der Bericht enthält die Größe des vollständigen kartesischen Produkts (Produkt der Wertezahlen aller Achsen), um den Gewinn durch Beschneidung zu zeigen.
- Beim vollständigen Modus wird das gesamte Produkt aller Achsen im Prüfschritt gemessen.
- Im Normalmodus läuft zuerst die Achsenstufe, deren Kandidaten dann im Prüfschritt gemeinsam gemessen werden.
- Die Front wird nur aus den im Prüfschritt gemessenen Einträgen gebildet (Zeit, Abweichung, Stabilität, Unsicherheit).
- Der Bericht vermerkt, ob das Budget erschöpft wurde, und die verstrichene Gesamtzeit.
- Am Ende wird eine Zusammenfassung protokolliert (Fronteinträge, gemessen, instabil, verworfen, dominiert, Produktgröße, Budgetstatus).
- Jeder Lauf trägt einen Protokollabschnitt mit Teilmodul und Modellkennung.

### Achsenstufe (Beschneidung)

- Für jede Kombination der Kernachsen wird jede trennbare Achse einzeln variiert, während die anderen trennbaren Achsen auf ihrem ersten Wert (dem kleinsten) stehen.
- Trennbare Achsen mit nur einem Wert müssen nicht gemessen werden. Der Wert gewinnt automatisch.
- Die Gewinner einer Achse sind die Pareto-optimalen Werte über Zeit und Abweichung unter den erfolgreich gemessenen Varianten.
- Hat eine trennbare Achse keinen Gewinner, wird diese Kernkombination übersprungen.
- Die Kandidaten sind das kartesische Produkt aus der Kernkombination und den Gewinnern je Achse. Doppelte Kandidaten werden entfernt.
- Vor jeder Kernkombination wird das Budget geprüft. Ist es verbraucht, wird abgebrochen und der Bericht entsprechend markiert.
- Einträge der Achsenstufe werden nicht doppelt berichtet, wenn dieselbe Konfiguration erneut vorkommt.

### Messen einer Gruppe

- Zuerst kommt die Abweichungsphase. Sie ist billig und entscheidet über Verwerfung, bevor Messzeit verbraucht wird.
  - Eine Unverträglichkeit ergibt "verworfen, Ausführung gescheitert" mit unendlicher Abweichung.
  - Ein nicht endliches Ergebnis ergibt "verworfen, nicht endlich".
  - Eine Abweichung über dem Maximum ergibt "verworfen, Abweichung über Maximum".
  - Jede andere ungültige Beurteilung führt ebenfalls zur Verwerfung mit dem gemeldeten Grund.
  - Andere Fehler brechen den Lauf mit diesem Fehler ab.
- Verworfene Kombinationen werden nicht zeitlich vermessen.
- Die Zeitmessung geschieht rundenweise reihum über alle aktiven Kombinationen, damit Lastschwankungen alle gleich treffen. In ungeraden Runden wird die Reihenfolge umgekehrt (A-B-B-A-Verfahren).
- Die erste aktive Kombination ist der Anker.
- Stabilität wird am Zeitverhältnis zum Anker derselben Runde beurteilt, weil nur die Beziehungen zwischen Kombinationen zählen. Verhältnisse werden nur gebildet, wenn die Ankerprobe positiv ist.
- Jeder Versuch zählt, auch ein unterbrochener.
- Jede Kombination erhält ihren Startzeitpunkt beim ersten Versuch.
- Ein Unterbrechungswunsch verhindert die Probe und löst die Rückzugspause aus.
- Wird eine Kombination beim Messen unverträglich, wird sie als "Ausführung gescheitert" verworfen.
- Andere Fehler brechen den Lauf ab.
- Die Schleife endet, wenn keine Kombination aktiv ist oder das Gesamtbudget verbraucht ist.

### Beendigung einzelner Kombinationen

- Die Streuungsreihe gilt als stabil, wenn mindestens die Mindestwiederholungen vorliegen und die Streuung die Schwelle nicht überschreitet.
- Die Streuung ignoriert das langsamste Fünftel der Proben, weil Last Läufe nur verlangsamt. Bei weniger als zwei Proben oder ohne positiven Mittelpunkt ist sie unendlich.
- Eine Nichtanker-Kombination endet in dieser Rangfolge:
  1. "Dominiert", falls Beschneidung aktiv ist. Das trifft zu, wenn sie schon genug Proben hat und eine andere noch gültige, ebenfalls ausreichend vermessene Kombination mit höchstens gleicher Abweichung um mehr als den Dominanzspielraum schneller ist.
  2. "Fertig und stabil", wenn die Verhältnisreihe zum Anker stabil ist.
  3. "Fertig und instabil", wenn die harte Obergrenze erreicht ist.
  4. Sonst misst sie weiter.
- Der Anker wird erst abgeschlossen, wenn keine andere Kombination mehr aktiv ist.
  - Gab es Mitbewerber, gilt der Anker als stabil, wenn mindestens ein Mitbewerber stabil und keiner instabil endete oder wenn seine eigene Reihe stabil ist.
  - Ohne Mitbewerber ist er stabil bei stabiler eigener Reihe, instabil bei erreichter Obergrenze, sonst misst er weiter.

### Ergebnis je Kombination

- Dominiert, verworfen (mit Grund) oder übersprungen. Übersprungen heißt: keine Probe, weil das Budget verbraucht war oder das Gerät nie verfügbar wurde.
- Andernfalls gemessen:
  - Zeit gleich dem Schätzer der Proben, mindestens null,
  - Abweichung,
  - Stabilität,
  - Probenanzahl,
  - Unsicherheit gleich der Streuung der Proben; ist diese nicht endlich, gilt ersatzweise der Dominanzspielraum.
- Kombinationen, die bei Budgetende noch aktiv waren und Proben haben, gelten als gemessen und instabil.

### Bericht und Protokollierung

- Der Bericht enthält Ziel, alle Einträge mit Stufe (Achse oder Prüfung), Kombination und Ergebnis, die Front (aufsteigend nach Zeit, streng fallend in der Abweichung), Budgetstatus, Produktgröße und Laufzeit.
- Es müssen Zähler für gemessene, instabile, verworfene und dominierte Einträge verfügbar sein.
- Jede Einordnung wird protokolliert: gemessen, verworfen (mit Grund), dominiert, übersprungen. Instabile Messungen werden mit dem Hinweis protokolliert, dass die Obergrenze erreicht und der beste Zwischenstand verwendet wurde.
- Konfiguration und Messmittel müssen nach dem Anlegen einsehbar sein.

## 9. Pareto-Front und Positionen

- Die Front minimiert Zeit und Abweichung. Punkte mit nicht endlichen Werten entfallen.
- Das Ergebnis ist nach aufsteigender Zeit geordnet. Bei gleicher Zeit zählt die kleinere Abweichung.
- Ein Punkt bleibt nur, wenn seine Abweichung streng kleiner ist als die aller schnelleren Punkte. Die Abweichung ist also streng fallend.
- Die Front muss für einfache Zahlenpaare und für beliebige Elemente mit einer Schlüsselvorschrift berechenbar sein.
- Positionen im Bereich von 0 bis 1 ergeben sich aus der kumulierten Bogenlänge entlang der Front.
  - Beide Achsen werden vorher per Minimum-Maximum normiert. Konstante Achsen werden zu null.
  - Zeit kann logarithmisch normiert werden, wenn alle Zeiten positiv sind.
  - Eine leere Front liefert keine Positionen, ein einzelner Punkt die Position 0,5.
  - Ist die Gesamtlänge null, werden die Punkte gleichmäßig verteilt.

## 10. Profile

- Ein Profil ist unbenannt und besteht aus einer Position auf der Skala von schnell (0) bis präzise (1) und einer Konfiguration mit genau einem Wert je gewählter Achse.
- Eine Position außerhalb von 0 bis 1 ist ein Fehler wegen ungültiger Präzision.
- Die Position eines einzelnen Profils ist 0,5.
- Die Profilbildung hat einen einzigen Stellwert, die Zusammenführungsschwelle auf der normierten Bogenlängenskala (Standard 0,1).
- Aus der Front werden die Profile in folgenden Schritten abgeleitet:
  1. Punkte mit nicht endlicher Abweichung oder Unsicherheit werden entfernt.
  2. Entfernt wird jeder Punkt, der weniger präzise ist als ein Nachbar, ohne signifikant schneller zu sein. Das heißt: Es gibt einen Punkt mit kleinerer Abweichung, dessen Zeit höchstens der eigenen Zeit mal eins plus beider Unsicherheiten entspricht. Solche Entfernungen werden protokolliert.
  3. Die Positionen der übrigen Punkte werden mit logarithmischer Zeit berechnet.
  4. Benachbarte Punkte mit einem Positionsabstand unter der Schwelle werden zu einer Gruppe verschmolzen.
  5. Als Vertreter einer Gruppe gilt der präziseste Punkt der gesamten Front, falls er in der Gruppe liegt, sonst der schnellste Punkt der Gruppe.
  6. Die Positionen der Vertreter werden neu berechnet, wieder mit logarithmischer Zeit.
  7. Jedes Profil erhält die Konfiguration seines Vertreters.
- Die Zahl der Profile folgt allein aus der Streuung. Ein leeres Ergebnis bedeutet, dass es kein brauchbares Profil gibt.
- Die Gruppenbildung und die abgeleiteten Profile werden protokolliert.

## 11. Profilauswahl

- Eine Aufgabe nennt einen Präzisionswunsch zwischen 0 (schnellste) und 1 (präziseste Kombination). Die Skala gilt relativ zum Teilmodul: 0,5 bedeutet auf Hauptprozessor und Grafikkarte unterschiedliche Genauigkeit.
- Der Wunsch wird auf den Bereich von 0 bis 1 begrenzt. Ein Wert, der keine Zahl ist, wird vorher mit einem Fehler wegen ungültiger Präzision abgelehnt.
- Gewählt wird das Profil mit dem kleinsten Abstand zwischen Wunsch und Position. Bei gleichem Abstand gewinnt das präzisere.
- Gibt es kein aktives Profil, entsteht der Fehler "kein aktives Profil".
- Je Teilmodul wird festgehalten, für welche Modelle es kein aktives Profil gibt.
  - Die erste Aufgabe für ein solches Modell scheitert mit dem Fehler "kein aktives Profil" und sperrt das Modell.
  - Spätere Aufgaben scheitern sofort, ohne erneute Profilsuche.
  - Ein ungültiger Präzisionswunsch sperrt das Modell nicht.
  - Die Sperre lässt sich aufheben, zum Beispiel nach erfolgreicher Neukalibrierung. Die Aufhebung meldet, ob eine Sperre bestand.
  - Abfrage, Sperre und Aufhebung müssen nebenläufig sicher sein. Eine vergiftete Sperre darf nicht zum Ausfall führen.
- Auswahl und Sperre werden protokolliert.

## 12. Persistenz der Kalibrierergebnisse

- Ergebnisse werden je Kombination aus Modell und Teilmodul abgelegt.
- Ein Eintrag enthält:
  - Fingerabdruck des Systems,
  - Profile, jeweils mit Position und Konfiguration als Text,
  - Instabilitätsmarke, gesetzt, wenn mindestens eine Messung instabil war,
  - Liste verworfener Kombinationen mit Konfiguration und lesbarem Grund,
  - Liste beschnittener Kombinationen, also dominiert oder übersprungen, mit Grund. Vollständig gemessene Kombinationen erscheinen in keiner der beiden Listen.
- Der Fingerabdruck enthält:
  - Schemaversion,
  - Teilmodul,
  - Grafikkartenname, Rechenfähigkeitsstufe und Treiberversion (nur mit Grafikkarte),
  - vorhandene Prozessormerkmale (Vektorerweiterung, Halbpräzisions-Umwandlung, Fused-Multiply-Add, Ganzzahl-Befehle für neuronale Netze),
  - Modellkennung,
  - Version der Software.
- Der Fingerabdruck ist bewusst knapp gehalten. Ein Treiberupdate ohne neue Treiberversion bleibt unbemerkt, das Löschen der Datei erzwingt eine neue Kalibrierung.
- Ohne Grafikkarte bleiben die Grafikfelder leer.
- Der Fingerabdruck des laufenden Systems muss ermittelt werden können, die Grafikdaten über die Treiberschnittstelle.
- Fehlende Felder beim Einlesen werden mit Standardwerten belegt, damit ältere oder unvollständige Dateien lesbar bleiben.
- Gespeicherte Profile müssen sich wieder in nutzbare Profile umwandeln lassen. Ungültige Positionen führen zu einem Fehler.
- Der Zustand eines Eintrags im Vergleich zum aktuellen System:
  - Kein Eintrag bedeutet "fehlend".
  - Gleicher Fingerabdruck bedeutet "kalibriert".
  - Abweichender Fingerabdruck bedeutet "veraltet". Das alte Ergebnis bleibt nutzbar, während im Hintergrund ein neues entsteht.
- Das Auflösen beim Start darf nie fehlschlagen.
  - Eine unlesbare Datei wird wie eine fehlende behandelt und mit einer Warnung protokolliert.
  - Bei "fehlend" gibt es keinen Eintrag, und die präziseste gültige Kombination wird verwendet.
  - Eine Neukalibrierung ist nötig, solange der Zustand nicht "kalibriert" ist.
- Die Ablage ist eine Schnittstelle mit zwei Operationen: Laden (kein Ergebnis, wenn nichts gespeichert ist) und atomares Speichern. Leser sehen dabei entweder den alten oder den neuen Eintrag.
- Die Dateiablage nutzt eine lesbare, strukturierte Textdatei je Modell und Teilmodul. Den Pfad bestimmt die Anwendung.
  - Eine fehlende Datei ist kein Fehler.
  - Fehlende Verzeichnisse werden angelegt.
  - Geschrieben wird zuerst in eine temporäre, prozessspezifisch benannte Datei, die auf den Datenträger gesichert und dann umbenannt wird.
  - Bei Fehlern wird die temporäre Datei entfernt.
  - Alle Ablagefehler werden als Ablagefehler gemeldet.
- Die Neukalibrierung läuft im Hintergrund auf dem Pool niedriger Priorität.
  - Zuerst wird kalibriert, das Ergebnis dann atomar ersetzt.
  - Fehler beider Schritte werden über das Auftragsergebnis gemeldet.
  - Der Austausch wird protokolliert (Modell, Schemaversion, Instabilität).

## 13. Hintergrundausführung

- Kalibrierung und Rechenkerne des Hauptprozessors laufen auf eigenen Threads niedriger Priorität, getrennt von der asynchronen Laufzeit der Anwendung, damit Messungen deren Arbeiter nie belegen.
- Der Prioritätsabbau gilt für den aufrufenden Thread. Auf Linux wird die niedrigste Planungspriorität gesetzt und gemeldet, ob das gelang. Auf anderen Systemen bleibt die Priorität unverändert, was protokolliert wird.
- Der Pool hat einen Namen und eine konfigurierbare Threadanzahl, mindestens einen Thread.
- Aufträge lassen sich einreichen und liefern einen Ergebnisbehälter.
  - Blockierendes Warten liefert das Ergebnis, oder nichts, wenn der Auftrag abgestürzt ist oder der Pool vorher beendet wurde.
  - Nicht blockierendes Abfragen ist möglich.
- Ein abstürzender Auftrag darf den Arbeiter nicht beenden.
- Beim Beenden des Pools werden keine neuen Aufträge mehr angenommen. Die bereits eingereichten laufen zu Ende, und das System wartet auf alle Arbeiter.

## 14. Robuste Statistik

- Als Schätzer für Zeitreihen stehen Median und Minimum zur Verfügung. Das Minimum eignet sich, weil Last Läufe nur verlangsamt. Der Median einer leeren Reihe ist keine Zahl.
- Als Streuungsmaße stehen der Variationskoeffizient (Standardabweichung durch Mittelwert) und Spannweite durch Median zur Verfügung.
- Beide Streuungsmaße ignorieren das langsamste Fünftel der Proben.
- Sie liefern unendlich, wenn weniger als zwei Proben bleiben, ein Wert nicht endlich ist oder der Mittelpunkt nicht positiv ist.