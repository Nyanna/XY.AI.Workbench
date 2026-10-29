# Funktionale Anforderungen: Speicherverwaltung der residenten Engine

## 1. Speicherklassen (Arenen)

- Das System muss den Speicher in sechs Klassen mit unterschiedlicher Lebensdauer einteilen: Gewichte, Indexsegment, Arbeitsbereich, Bereitstellungsspeicher für Übertragungen, Parameterblock und abgebildeter Hostspeicher.
- Der abgebildete Hostspeicher ist seitengesperrter Arbeitsspeicher des Rechners, den das Gerät über den Systembus direkt lesen kann.
- Jede Klasse muss eine stabile, eindeutige numerische Kennung haben. Aus einer Kennung muss sich die Klasse zurückgewinnen lassen. Eine unbekannte Kennung ergibt "keine Klasse".
- Die Liste der Klassen muss vollständig und in fester Reihenfolge abrufbar sein. Die Klassenmenge soll später erweiterbar sein.
- Für jede Klasse muss eine Kapazität in Byte konfigurierbar sein. Die Kapazität des Indexsegments umfasst alle gleichzeitig residenten Generationen des Index.
- Das System muss die Gesamtgröße des tatsächlich auf dem Gerät liegenden Speichers berechnen. Sie besteht aus Gewichten, Indexsegment, Arbeitsbereich und Parameterblock. Bereitstellungsspeicher und abgebildeter Hostspeicher zählen nicht dazu, weil sie im Hostspeicher liegen. Die Summe darf bei Überlauf nicht umlaufen, sondern muss am Maximum sättigen.
- Das System muss die Reserve für andere Generationen berechnen. Das sind die Indexbytes, die nicht auf die aktive Generation entfallen: Kapazität des Indexsegments minus ihr Anteil bei gleichmäßiger Aufteilung auf die Generationszahl. Eine Generationszahl kleiner als eins gilt als eins.

## 2. Speicherbudget

- Das Budget besteht aus freiem Speicher, Gesamtspeicher und der für Generationen reservierten Menge.
- Der nutzbare Anteil ist der freie Speicher abzüglich der Generationsreserve. Er darf nie negativ werden.
- Das System muss über eine austauschbare Abfrage freien und gesamten Gerätespeicher in Byte ermitteln. Ein Fehlschlag der Abfrage wird als eigener Fehler "Speicherabfrage fehlgeschlagen" mit Begründung gemeldet.
- Es muss einen Standardwert für freizuhaltenden Spielraum geben: 512 Mebibyte für den Treiberkontext und andere Prozesse, etwa den Bildschirmkompositor.
- Eine Passprüfung muss feststellen, ob die geplanten Geräte-Speicherklassen in den freien Speicher abzüglich des Spielraums passen. Der Spielraum wird dabei sättigend abgezogen. Passt der Plan nicht, wird ein Fehler mit angeforderter und nutzbarer Menge gemeldet.

## 3. Speicherbereich als Handle

- Ein Speicherbereich muss durch ein zeigerfreies Handle beschrieben werden. Es besteht aus der Kennung der Klasse, dem Versatz in Byte, der Länge in Byte und einer Generationsnummer.
- Handles müssen kopierbar, vergleichbar und als Schlüssel in Sammlungen verwendbar sein.
- Die Auflösung eines Handles auf eine echte Geräteadresse ist ausschließlich Sache des jeweiligen Speicher-Backends. Geräteadressen dürfen das Backend nicht verlassen.

## 4. Freispeicherverwaltung

- Pro Klasse muss ein Versatz-Allokator Bereiche aus einer festen Kapazität vergeben, ohne selbst Speicher zu halten.
- Er muss nach dem Prinzip "erste passende Lücke" in aufsteigender Adressreihenfolge arbeiten.
- Freie Lücken müssen verwaltet werden. Beim Freigeben verschmelzen sie mit unmittelbar angrenzenden freien Nachbarn auf beiden Seiten.
- Jede Anforderung kann eine Ausrichtung angeben. Der Versatz wird auf ein Vielfaches der Ausrichtung aufgerundet. Eine Ausrichtung von null gilt als eins. Überläufe bei der Berechnung führen dazu, dass die Lücke nicht passt, nicht zu einem Absturz.
- Entsteht durch die Ausrichtung vorne oder durch eine kleinere Anforderung hinten ein Rest, bleibt dieser als freie Lücke erhalten.
- Anforderungen der Länge null werden nicht erfüllt.
- Der Allokator muss Kapazität und aktuell insgesamt freie Menge angeben können. Bei Kapazität null gibt es keine freie Lücke.

## 5. Buchführung und Gültigkeit von Bereichen

- Pro Klasse muss es einen reinen Buchführungs-Allokator geben, der Versätze und Generationen verfolgt, aber keinen Gerätespeicher besitzt. Er muss nebenläufig sicher benutzbar sein.
- Jede erfolgreiche Vergabe liefert einen Bereich mit der Kennung der Klasse, dem Versatz, der angeforderten Länge und einer Generationsnummer.
- Ein Versatz mit einer Länge, die zum ersten Mal vergeben wird, erhält die Generation null. Wird später genau derselbe Versatz mit genau derselben Länge erneut vergeben, wird die Generation um eins erhöht und läuft bei Überlauf um. So lässt sich ein veraltetes Handle von einem neuen unterscheiden.
- Ein Bereich gilt nur als lebendig, wenn Klasse, Versatz, Länge und Generation mit einer aktuell vergebenen, nicht freigegebenen Eintragung übereinstimmen.
- Das Freigeben muss einen Fehler liefern, wenn der Bereich zu einer anderen Klasse gehört, unbekannt, bereits freigegeben oder in Generation, Länge oder Versatz abweichend ist. Bei Erfolg wird der Bereich als nicht lebendig markiert und an den Freispeicher zurückgegeben.
- Das allgemeine Verwaltungs-Interface (Vergabe mit Gerätetyp, Bytezahl und Ausrichtung, Freigabe) muss für alle Varianten gelten. Die Freigabe darf veraltete Handles nie dereferenzieren. Sie ignoriert sie und schreibt eine Warnung ins Protokoll.
- Scheitert eine Vergabe wegen Platzmangel, muss der Fehler angeforderte und aktuell freie Menge nennen.
- Für Tests muss sich ein Fehlschlag der Vergabe künstlich auslösen lassen, abhängig von der Klasse. Er wird wie Erschöpfung gemeldet.
- Vergaben und Freigaben müssen auf Debug-Ebene protokolliert werden, mit Klasse, Versatz und Größe.

## 6. Allokator pro Klasse mit verzögerter Freigabe

- Pro Klasse muss ein Allokator Kapazität, Klasse und Gerätetyp kennen und Bereiche vergeben.
- Vor jeder Vergabe muss er bereits abgeschlossene verzögerte Freigaben einsammeln, damit deren Platz verfügbar ist.
- Scheitert die Vergabe wegen Erschöpfung, wird ein Fehler mit Klasse, angeforderter und freier Menge geliefert. Jeder andere Fehlschlag wird als "Vergabe fehlgeschlagen" mit Klasse und Begründung gemeldet.
- Eine sofortige Freigabe eines Bereichs muss abgelehnt werden, wenn er bereits für verzögerte Freigabe vorgemerkt ist oder nicht mehr gültig ist. In beiden Fällen lautet der Fehler "veralteter Bereich".
- Die verzögerte Freigabe nimmt einen Bereich und ein Ereignis entgegen. Der Bereich kehrt erst in den Freispeicher zurück, wenn das Ereignis abgeschlossen ist. So sieht laufende Arbeit niemals wiederverwendeten Speicher.
- Die verzögerte Freigabe muss abgelehnt werden, wenn der Bereich nicht lebendig oder schon vorgemerkt ist. Nach dem Vormerken wird sofort geprüft, ob das Ereignis schon abgeschlossen ist.
- Das Einsammeln gibt alle vorgemerkten Bereiche frei, deren Ereignis abgeschlossen ist, und liefert die Anzahl zurück. Noch wartende Bereiche bleiben vorgemerkt. Findet sich dabei ein bereits ungültiger Bereich, wird nur gewarnt und der Vorgang läuft weiter.
- Die "sofort verfügbare" Menge schließt noch wartende Bereiche aus. Die Anzahl der wartenden Bereiche und die Lebendigkeit eines Bereichs müssen abfragbar sein.
- Alle Operationen müssen nebenläufig sicher sein.
- Die Vergabe muss auch über das allgemeine Verwaltungs-Interface möglich sein. Erschöpfung wird dort als Erschöpfung gemeldet, jeder andere Fehler als allgemeiner Fehlschlag mit Begründung.
- Vergaben und verzögerte Freigaben sollen mit Klasse, Versatz, Länge und Generation protokolliert werden.

## 7. Menge aller Klassen

- Eine Menge von Allokatoren muss pro Klasse genau einen Allokator für einen gegebenen Gerätetyp und gegebene Größen bereitstellen.
- Es muss möglich sein, die Größenvorgabe abzufragen, den Allokator einer Klasse zu erhalten und den zuständigen Allokator zu einem Bereich zu finden. Gehört die Kennung eines Bereichs zu keiner Klasse, lautet der Fehler "veralteter Bereich".

## 8. Geräteübergreifende Schnittstelle

- Jedes Speicher-Backend muss dieselbe nebenläufig nutzbare Schnittstelle erfüllen:
  - Vergabe in einer bestimmten Klasse mit Größe und Ausrichtung
  - Freigabe eines Bereichs
  - Freigabe nach einem Ereignis
  - Abfrage des Budgets

## 9. Prozessor-Backend

- Das Prozessor-Backend muss für jede Klasse einen mit Nullen gefüllten Bereich im Arbeitsspeicher anlegen. Es kennt keine Datenströme und keine Bereitstellung.
- Es muss die Allokatormenge und die Generationszahl halten und die allgemeine Geräteschnittstelle erfüllen.
- Schreiben, Lesen und Kopieren müssen auf Bereichsbasis möglich sein. Jeder Zugriff prüft vorab, dass der Bereich lebendig ist und die Datenmenge die Bereichslänge nicht überschreitet. Andernfalls lautet der Fehler "veralteter Bereich".
- Beim Kopieren gilt die Länge des Quellbereichs. Beide Bereiche müssen gültig und mindestens so groß sein. Liegen sie in derselben Klasse, muss überlappungssicher kopiert werden. Bei verschiedenen Klassen wird über einen Zwischenpuffer kopiert.
- Das Budget ergibt sich aus der Summe der verfügbaren Mengen aller Klassen als frei, der Summe aller Kapazitäten als gesamt und der Generationsreserve.

## 10. Backend für den Nvidia-Grafikprozessor

### Einrichtung

- Das Backend muss sich nur mit einem Nvidia-Gerät anlegen lassen. Bei einem anderen Gerät lautet der Fehler "Gerätespeicher nicht verfügbar".
- Beim Anlegen muss es zuerst freien und gesamten Speicher abfragen und die Passprüfung mit dem übergebenen Spielraum durchführen. Bei Misserfolg wird nichts alloziert.
- Freigegebener, stromgeordnet vergebener Speicher soll im Standard-Pool des Treibers bleiben und nicht an den Treiber zurückgegeben werden. So findet im Dauerbetrieb keine Allokation auf Treiberebene mehr statt.
- Für jede auf dem Gerät liegende Klasse (Gewichte, Indexsegment, Arbeitsbereich, Parameterblock) muss ein zusammenhängender, mit Nullen gefüllter Speicherblock in voller Kapazität vorab angelegt werden. Klassen mit Größe null erhalten keinen Block. Bereitstellungsspeicher und abgebildeter Hostspeicher bekommen keinen Geräteblock.
- Nach dem Anlegen muss das Backend auf den Abschluss aller Vorgänge warten.
- Hat der abgebildete Hostspeicher eine Größe größer als null, muss ein seitengesperrter, mit Nullen gefüllter Hostspeicherblock angelegt werden, den das Gerät über eine eigene Adresse direkt lesen kann. Scheitert die Adressabbildung, wird der Block wieder freigegeben. Beim Beenden wird der Block freigegeben, sofern der Gerätekontext noch gebunden werden kann.
- Die Allokatormenge des Backends muss die Kapazität des Bereitstellungsspeichers auf null setzen, weil dieser über den Ring verwaltet wird.
- Nach erfolgreichem Anlegen soll das Backend freien und gesamten Speicher sowie die Größe der Geräteklassen in Mebibyte protokollieren.

### Bereitstellungsring

- Das Backend muss auf Anforderung einen Ring aus seitengesperrten Hostpuffern erzeugen. Angegeben werden Anzahl und Größe der Plätze. Die Puffer werden mit Nullen gefüllt. Der Ring dient Übertragungen vom Rechner zum Gerät.

### Kopieren und Übertragen

- Kopie von Gerät zu Gerät: zwischen zwei gültigen Bereichen verschiedener Klassen, geordnet auf einem übergebenen Datenstrom. Die Länge ist die des Quellbereichs, der Zielbereich muss mindestens so groß sein.
  - Liegen Quelle und Ziel in derselben Klasse, wird mit einem Übertragungsfehler abgelehnt.
  - Fehlt der Speicherblock einer der Klassen, lautet der Fehler "veralteter Bereich".
  - Beide Blöcke müssen in einer festen Reihenfolge gesperrt werden, damit sich nebenläufige Kopien nicht gegenseitig blockieren.
- Asynchrones Hochladen: aus einem seitengesperrten Hostpuffer in einen gültigen Geräte-Bereich, geordnet auf einem Datenstrom. Der Aufrufer muss garantieren, dass der Quellpuffer seitengesperrt ist und unangetastet bleibt, bis ein danach auf dem Datenstrom aufgezeichnetes Ereignis abgeschlossen ist.
- Asynchrones Herunterladen: aus einem gültigen Geräte-Bereich in einen seitengesperrten Hostpuffer. Der Puffer darf erst gelesen werden, wenn ein danach aufgezeichnetes Ereignis abgeschlossen ist.
- Bei beiden asynchronen Übertragungen muss der Bereich lebendig sein und die Datenmenge in den Bereich passen.
- Treiberfehler werden je nach Situation gemeldet: als "Gerätespeicher nicht verfügbar" mit Begründung oder als Übertragungsfehler mit Richtung (Host zum Gerät, Gerät zum Host, Gerät zu Gerät) und Begründung.

### Abgebildeter Hostspeicher

- Schreiben in einen Bereich der Klasse "abgebildeter Hostspeicher" muss ohne Übertragung möglich sein, weil das Gerät die Bytes direkt sieht. Ebenso muss Lesen aus einem solchen Bereich möglich sein.
- Der Zugriff ist nur erlaubt, wenn ein abgebildeter Block existiert, der Bereich zur Klasse "abgebildeter Hostspeicher" gehört und lebendig ist. Sonst lautet der Fehler "veralteter Bereich".
- Versatz und Länge innerhalb des Bereichs müssen die Bereichslänge einhalten. Überläufe sind abzufangen. Der resultierende Bereich muss außerdem im gesamten abgebildeten Block liegen. Verletzungen werden als Übertragungsfehler mit erklärender Meldung gemeldet.
- Für Startvorgänge innerhalb der Bibliothek muss die geräteseitige Adresse eines abgebildeten Bereichs bereitgestellt werden, nach denselben Prüfungen.

### Budget

- Das Budget muss bei jedem Aufruf frisch vom Treiber ermittelt werden: freier und gesamter Speicher plus Generationsreserve. Es soll auf Debug-Ebene protokolliert werden.

## 11. Bereitstellungsring

- Der Ring verwaltet eine feste Zahl von Plätzen mit je einer Nutzlast. Jeder Platz ist frei, belegt oder in Übertragung mit einem zugehörigen Ereignis.
- Ein Platz darf erst wieder verwendet werden, wenn das Ereignis seiner letzten Übertragung abgeschlossen ist.
- Platzanzahl und Anzahl der Plätze in Übertragung müssen abfragbar sein.
- Das Anfordern liefert reihum, beginnend hinter dem zuletzt vergebenen Platz, den nächsten freien oder abgeschlossenen Platz und markiert ihn als belegt. Ist keiner verfügbar, wird nichts geliefert.
- Das blockierende Anfordern wartet bei vollem Ring auf das Ereignis der ältesten laufenden Übertragung und versucht es danach erneut. Gibt es keine laufende Übertragung, weil alle Plätze belegt sind, lautet der Fehler "Bereitstellung erschöpft" mit angeforderter Menge eins und freier Menge null. Scheitert das Warten, wird ein Übertragungsfehler in Richtung Host zum Gerät gemeldet.
- Zugriff auf die Nutzlast eines Platzes ist nur für einen belegten Platz erlaubt. Bei ungültigem Index oder falschem Zustand lautet der Fehler "ungültiger Platz".
- Ein belegter Platz kann mit einem Ereignis als "in Übertragung" veröffentlicht werden. Ein belegter, nie genutzter Platz kann wieder freigegeben werden. Beide Übergänge sind nur aus dem Zustand "belegt" erlaubt, sonst lautet der Fehler "ungültiger Platz".
- Der Ring muss nebenläufig sicher sein.

## 13. Gerätetypen und Rechengenauigkeit

- Das System muss zwischen Prozessor und Grafikprozessor als Gerätetyp unterscheiden. Die Auswahl soll später erweiterbar sein.
- Der Standard-Datentyp für Berechnungen ist auf dem Grafikprozessor die 16-Bit-Gleitkommazahl und auf dem Prozessor die 32-Bit-Gleitkommazahl.

## 14. Elementtypen

- Das System muss einen eigenen, von Fremdbibliotheken unabhängigen Elementtyp führen. Fremde Typen werden nur an den Rändern umgewandelt.
- Unterstützt werden 32-Bit-Gleitkomma, 16-Bit-Gleitkomma, Brain-Gleitkomma mit 16 Bit, vorzeichenlose 8-Bit-Ganzzahl, vorzeichenlose 32-Bit-Ganzzahl und vorzeichenbehaftete 64-Bit-Ganzzahl.
- Die Größe pro Element muss abfragbar sein: 1 Byte für 8 Bit, 2 Byte für die beiden 16-Bit-Gleitkommatypen, 4 Byte für 32-Bit-Gleitkomma und die vorzeichenlose 32-Bit-Ganzzahl, 8 Byte für die 64-Bit-Ganzzahl.
- Elementtypen müssen aus dem Format der Gewichtsdateien (Safetensors) übernommen werden können. Für alle nicht unterstützten Typen entsteht ein Fehler "nicht unterstützter Datentyp" mit Angabe der Quelle.
- Ist die Anbindung an das Tensor-Framework Candle aktiviert, muss in beide Richtungen umgewandelt werden können. Nicht unterstützte Typen ergeben ebenfalls einen entsprechenden Fehler.
- Candle zählt Elemente, die Engine zählt Bytes. Für beide Richtungen muss eine Umrechnung zwischen Elementzahl und Bytezahl anhand des Elementtyps bereitstehen.
- Für additive Aufmerksamkeitsmasken muss ein fester endlicher Maskenwert von minus 10000 bereitstehen. Negativ unendlich würde bei 16-Bit-Gleitkomma zu ungültigen Zeilen führen.

## 15. Tensor-Anordnung

- Eine Anordnung beschreibt Form, Schrittweiten in Byte und Elementtyp. Sichten ändern nur die Anordnung, nie die Daten.
- Eine zusammenhängende Anordnung wird aus Form und Elementtyp berechnet. Die letzte Dimension hat die Elementgröße als Schrittweite. Jede weitere Dimension nach vorn ergibt sich aus dem Produkt der Dimensionen dahinter. Dimensionen der Größe null zählen dafür als eins.
- Abfragbar sein müssen die Elementzahl (Produkt der Formgrößen), die Bytelänge und die Zusammenhängigkeit, also die Übereinstimmung mit der berechneten zusammenhängenden Anordnung.
- Vertauschen zweier Dimensionen tauscht Form und Schrittweiten dieser Dimensionen, ohne Daten zu bewegen.
- Verkürzen einer Dimension auf eine gegebene Länge ändert nur die Form dieser Dimension, nicht die Schrittweiten.

## 16. Einbindung von Speicherbereichen in Tensoren (Machbarkeit)

- Ein Teilbereich eines Geräteblocks muss sich in ein Tensor-Objekt von Candle einhüllen lassen, sodass Candle den korrekten Bereich liest.
- Der eingehüllte Tensor darf den Speicher nicht besitzen. Gibt Candle ihn frei, wird der Gerätekontext unbrauchbar. Daher muss ein solcher Tensor bewusst nicht freigegeben, also absichtlich "vergessen" werden, damit der Kontext sauber bleibt und der Block unversehrt bleibt.
- Kopien eines eingehüllten Tensors müssen eigenen, getrennten Speicher außerhalb des Geräteblocks erhalten.
- Das Schreiben in einen eingehüllten Tensor muss durchschlagen: Der Teilbereich im Block ändert sich, benachbarte Elemente bleiben unverändert.
- Diese Prüfungen benötigen ein echtes Nvidia-Gerät und laufen nur dort.

## 17. Übergreifende Eigenschaften

- Alle gemeinsam genutzten Strukturen (Buchführung, Vormerkliste, Speicherblöcke, Bereitstellungsring) müssen von mehreren Threads gleichzeitig benutzt werden können.
- Das Verhalten von Allokatoren, Ereignisabhängigkeiten und Bereitstellung muss ohne Grafikprozessor testbar sein, etwa über ein künstlich steuerbares Ereignis, das erst auf Anforderung abgeschlossen wird.
- Warnungen und Debug-Meldungen sollen strukturiert mit Klasse, Versatz, Länge und Generation protokolliert werden.
