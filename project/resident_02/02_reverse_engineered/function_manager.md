# Funktionale Anforderungen Manager

## Seitencache-Hinweise für dateigestützte Indexdaten

- Das System muss für einen Byte-Bereich der Indexdaten folgende Empfehlungen an das Betriebssystem geben können:
  - Der Bereich wird bald benötigt.
  - Der Bereich wird sequentiell gelesen.
  - Der Bereich wird wahlfrei gelesen.
  - Der Bereich ist "kalt".
- Eine "kalt"-Empfehlung senkt die Priorität der Seiten bei der Speicherrückgewinnung, ohne ihren Inhalt zu verwerfen.
- Alle Empfehlungen sind unverbindlich. Das Betriebssystem darf sie ignorieren.
- Das Ziel der Empfehlungen muss über eine austauschbare, von mehreren Threads nutzbare Schnittstelle abstrahiert sein.
- Ein Bereich mit Anfang hinter dem Ende oder mit Ende hinter der Länge der Datenabbildung muss als ungültige Eingabe abgewiesen werden.
- Ein leerer Bereich muss erfolgreich und ohne Wirkung beantwortet werden.
- Die "kalt"-Empfehlung muss auf die Seitengrenzen des Betriebssystems ausgerichtet werden. Der Anfang wird abgerundet und die Länge entsprechend vergrößert.
- Auf Plattformen ohne Unterstützung der "kalt"-Empfehlung muss sie erfolgreich und ohne Wirkung beantwortet werden.
- Fehler des Betriebssystems müssen an den Aufrufer weitergereicht werden.

## Blockierende Lesezugriffe

- Lesezugriffe auf Indexdaten, die Seitenfehler oder Plattenzugriffe auslösen können, müssen im dafür vorgesehenen Pool für blockierende Arbeit laufen. Sie dürfen nie auf den asynchronen Arbeitern laufen.
- Vor dem Lesen muss der Seitencache-Hinweis für den Bereich gesetzt werden. Standardmäßig lautet er "wird bald benötigt". Der Aufrufer kann einen anderen Hinweis angeben, beispielsweise "sequentiell" für dichte Durchläufe.
- Ein Lesezugriff muss auch ohne Hinweis ausführbar sein.
- Ein Vorabladen für einen Bereich muss als eigene Funktion verfügbar sein.
- Ist keine Hinweisquelle konfiguriert, muss das Setzen eines Hinweises ohne Wirkung bleiben.
- Ein abgelehnter Hinweis darf nur protokolliert werden. Er darf den Lesezugriff nicht verhindern.
- Wird die blockierende Aufgabe abgebrochen oder schlägt sie ab, muss dies dem Aufrufer als Fehler gemeldet werden.

## Speicherverwaltung für Indexsegmente

### Zulassung und Zuteilung

- Jedes Segment muss auf mindestens 256 Bytes ausgerichtet sein. Eine kleinere geforderte Ausrichtung muss angehoben werden.
- Ein konfigurierbarer Reserveanteil der Arena darf nie vergeben werden.
- Vor jeder Zuteilung muss das System zuerst ausstehende Freigaben des zugrunde liegenden Speichers einsammeln.
- Eine Zuteilung muss nur zugelassen werden, wenn gilt: Belegtes, plus angeforderte Bytes, plus Ersetzungsspielraum, plus Reserve passen in die Gesamtkapazität. Andernfalls muss der Fehler "Arena erschöpft" mit angeforderter und tatsächlich verfügbarer Menge gemeldet und protokolliert werden.
- Fehler des zugrunde liegenden Speichers müssen auf die Fehler der Verwaltung abgebildet werden:
  - Erschöpfung wird als "Arena erschöpft" mit der freien Menge gemeldet.
  - Ein veralteter Bereich wird als solcher gemeldet.
  - Alle übrigen Fehler werden gekapselt weitergereicht.
- Die Arena muss entweder eine geliehene Referenz oder einen geteilten Besitz am zugrunde liegenden Speicherverwalter nutzen können. Das ermöglicht den Einsatz hinter dauerhaft lebenden Schnittstellenobjekten.

### Buchführung und Budgets

- Das System muss die lebenden Bytes, das größte jemals zugeteilte Segment und die zurückgestellten Segmente führen.
- Der Ersetzungsspielraum ist die Anzahl gleichzeitig erlaubter Ersetzungen (Standard: eine) multipliziert mit dem größten Segment. Er muss bei jeder regulären Zuteilung freigehalten werden und dabei das neu angeforderte Segment als mögliches größtes berücksichtigen.
- Beim Beginn einer Ersetzung darf der Spielraum nicht nochmals verlangt werden, weil er dafür vorgesehen ist.
- Das System muss ein Budget zur Ersetzung bereitstellen mit lebenden Bytes, Anzahl gleichzeitiger Ersetzungen und größtem Segment. Daraus werden der Spielraum und der Gesamtbedarf berechnet, jeweils überlaufsicher.
- Das System muss ein Arena-Budget bereitstellen mit:
  - Kapazität
  - Reserve
  - lebenden Bytes
  - zurückgestellten Bytes
  - Ersetzungsspielraum
  - verfügbarer Menge
- Die verfügbare Menge ist Kapazität abzüglich Belegtes, Reserve und Spielraum, ohne negativ zu werden. Vor der Berechnung müssen ausstehende Freigaben eingesammelt werden.
- Die Auslastung muss auf Anforderung protokolliert werden können.

### Freigabe und Zurückstellung

- Eine sofortige Freigabe darf nur für Bereiche erfolgen, die weder Leser noch laufende Arbeit referenzieren kann.
- Eine Freigabe nach einem Ereignis muss möglich sein. Sie wird wirksam, sobald alle vor dem Ereignis erfasste Arbeit abgeschlossen ist.
- Jede Freigabe und Zurückstellung muss prüfen, ob der Bereich noch lebt und nicht bereits zurückgestellt ist. Andernfalls muss der Fehler "veralteter Bereich" gemeldet werden.
- Ein ersetzter Bereich muss mit der Epoche der ersetzenden Version zurückgestellt werden können. Dabei sinken die lebenden Bytes.
- Die Rückgewinnung muss alle zurückgestellten Bereiche freigeben, deren Epoche nicht über der kleinsten Epoche aller Leser liegt, und die Anzahl der freigegebenen melden.
- Ist ein zurückzugewinnender Bereich bereits verschwunden, muss das nur gewarnt werden. Der Vorgang darf nicht abbrechen.

### Ersetzung durch Kopie

- Eine Ersetzung muss ein neues Segment gleicher Größe wie das alte bereitstellen, damit die neue Version neben der alten aufgebaut werden kann.
- Es darf immer nur eine Ersetzung gleichzeitig laufen. Es gibt zwei Varianten:
  - Die wartende Variante wartet, bis eine laufende Ersetzung beendet ist.
  - Die nicht wartende Variante liefert "keine Ersetzung", wenn bereits eine läuft.
- Der Beginn einer Ersetzung muss prüfen, dass das alte Segment lebt und nicht zurückgestellt ist.
- Der Abschluss mit Epochenangabe muss die Kopie als Version dieser Epoche veröffentlichen und das alte Segment zurückstellen. Die Freigabe erfolgt später durch die Rückgewinnung.
- Scheitert die Zurückstellung beim Abschluss, muss der Fehler gemeldet werden. Die Kopie bleibt dann unter Kontrolle des Vorgangs.
- Wird eine Ersetzung ohne Abschluss verworfen, muss die Kopie freigegeben werden. Scheitert das, wird nur gewarnt.
- Alter und neuer Bereich müssen jederzeit abfragbar sein.

### Schrittweise beschreibbare Segmente

- Das System muss ein Segment der gewünschten Kapazität mit der Mindestausrichtung für einen einzelnen Schreiber bereitstellen. Dieser füllt es von vorn nach hinten.
- Ein Anspruch auf eine Länge mit Ausrichtung muss die absoluten Arena-Adressen des Bereichs liefern. Eine Ausrichtung unter eins gilt als eins.
- Passt der Anspruch nicht mehr in das Segment, muss "Arena erschöpft" mit der verbleibenden Restmenge gemeldet werden. Der Schreibstand darf sich dabei nicht ändern.
- Geschriebene und verbleibende Bytes müssen abfragbar sein.
- Das Segment muss in Bereich und Schreibstand zerlegbar sein.

## Aufbau neuer Segmente

### Schreibzeiger

- Der Aufbau muss ein Modell des geräteseitigen Schreibzeigers enthalten. Der Kernel hängt Segmente an einen vorab zugeteilten Bereich an und teilt selbst nie Speicher zu.
- Jeder Anspruch auf Platz muss den Zeiger voranschieben. Sobald ein Anspruch nicht mehr passt, müssen alle späteren Ansprüche ebenfalls scheitern. So bilden die gültigen Bytes stets ein zusammenhängendes Präfix.
- Ein gescheiterter Anspruch darf nichts schreiben. Er muss einen Überlaufzustand setzen und die kleinste Überlaufposition als Grenze festhalten. Der erste Überlauf wird protokolliert.
- Ein Gruppenanspruch muss den Zeiger mit einer einzigen Aktualisierung für die ganze Gruppe voranschieben. Die einzelnen Ansprüche werden in Gruppenreihenfolge bedient.
- Die gültige Länge ist das Minimum aus Überlaufgrenze, aktuellem Zeigerstand und Kapazität. Der Überlaufzustand muss abfragbar sein.
- Der Grafikprozessor-Kernel muss das gleiche Verhalten bieten:
  - Eine Gruppe von 32 parallelen Ausführungseinheiten fasst ihre Ansprüche über Präfixsummen zusammen.
  - Pro Gruppe erfolgt nur eine atomare Aktualisierung.
  - Bei Überlauf wird statt einer Schreibposition ein Ungültig-Marker ausgegeben, die Grenze auf die kleinste Überlaufposition gesetzt und ein Überlauf-Kennzeichen gesetzt.
  - Die Ergebnisse müssen für den Host sichtbar gemacht werden.

### Adressierung gebauter Segmente

- Das System muss aus Längen eine exklusive Präfixsumme bilden, verschoben auf eine Basisadresse. Bei n Längen liefert sie n+1 aufsteigende Offsets.
- Eine Offsettabelle muss Einträge samt Längen aufnehmen und zu jeder Adresse den Eintrag liefern, in dessen Bereich sie liegt. Größe und Leere müssen abfragbar sein.
- Die Planung des Layouts muss entscheiden zwischen:
  - Festem Schritt: Er gilt, wenn der Schritt der größten Eintragslänge entspricht, in 32 Bit passt, größer als null ist und der Anteil des Füllmaterials am aufgefüllten Gesamtumfang eine vorgegebene Promille-Schwelle nicht überschreitet.
  - Variabler Tabelle: Sie gilt in allen anderen Fällen.
- Beide Layouts müssen Adressen auf Einträge abbilden können.
- Das Layout muss in eine Platzzuordnung umwandelbar sein, in der jeder Platz an seinen Eintrag gebunden ist.

### Zwei Warteschlangen und ihre Reihenfolge

- Einbettungs-Schreibvorgänge und Bewertungstreffer müssen über getrennte Warteschlangen laufen. Jede hat genau einen Erzeuger und einen Verbraucher.
- Schreibvorgänge müssen immer zuerst registriert werden. Ein Treffer darf erst nach dem Schreibvorgang eingestellt werden, den er liest.
- Der Verbraucher muss zuerst alle Schreibvorgänge und danach alle Treffer entnehmen. So gibt es keinen beobachteten Treffer ohne Adresszuordnung.
- Das Einstellen und die Anzahl wartender Einträge je Warteschlange müssen abfragbar und protokolliert sein.
- Ein Durchlauf über beide Warteschlangen muss die Treffer des Plattenslots liefern, indem er die Verarbeitung eines Kommunikations-Slots mit den entnommenen Daten startet.

### Ausführung eines Einbettungsstapels

- Ein Stapel muss so lange fortgesetzt werden, bis der Kernel Treffer meldet.
- Vor jedem Schritt muss geprüft werden, ob die zulässige Höchstzahl an Bereichen erreicht ist. Dann bricht der Vorgang mit "Arena erschöpft" ab.
- Pro Schritt muss ein beschreibbarer Bereich der vorgegebenen Größe reserviert und dem Kernel zugewiesen werden.
- Die Eingabe muss hinter den Bytes fortgesetzt werden, die der Kernel als verarbeitet meldet.
- Eine Teilantwort des Kernels ist kein Fehler. Der Zähler der Teilantworten wird erhöht und der nächste Bereich zugewiesen.
- Der Stapel muss immer beendet werden, auch bei Fehlern.
- Bei Erfolg müssen die geschriebenen Bereiche, die verwendeten Regionen und die Anzahl der Teilantworten geliefert werden.
- Bei einem Fehler müssen alle reservierten Bereiche freigegeben werden. Gemeldet wird zuerst der Fehler des Laufs, sonst der des Stapelendes.

## Torklingel zwischen Stapeln und Host-Änderungen

- Der Grafikprozessor muss die Zahl der abgeschlossenen Stapel mit Freigabe-Semantik veröffentlichen. Der Host liest sie mit Erwerbs-Semantik.
- Der Host darf nur zwischen dem Abschluss eines Stapels und seiner Bestätigung verändern.
- Das Gerät muss vor dem Start des nächsten Stapels prüfen, ob die Lücke geschlossen wurde. Der erste Stapel darf immer starten, jeder weitere erst, wenn die Bestätigung seinen Vorgänger abdeckt.
- Der Host darf nie auf den Kernel selbst warten, nur auf ein Stapel-Ereignis.
- Eine nicht blockierende Abfrage muss eine Lücke liefern, wenn die Zahl der abgeschlossenen Stapel von der bestätigten abweicht. Mehrere zusammengefallene Abschlüsse öffnen nur eine Lücke mit der höchsten Stapelnummer.
- Eine wartende Abfrage muss zuerst das Stapel-Ereignis abwarten und danach abfragen. Gerätefehler müssen weitergereicht werden.
- Das Schließen einer Lücke muss die Bestätigung veröffentlichen und damit den nächsten Stapel freigeben. Eine Lücke, die nicht geschlossen wird, ist ein Programmierfehler.
- Alle vor der Veröffentlichung geschriebenen Ergebnisse und Zuordnungen müssen für den Host sichtbar sein.
- Die beiden Zellen müssen in fest zugeordnetem, für das Gerät abbildbarem Hostspeicher liegen können.

## Verdrängung

- Das System muss Einträge für Spiegel im Grafikspeicher und im Hauptspeicher nach drei Kriterien verdrängen können: Leerlaufablauf, Füllstandsgrenzen mit Hysterese und kostenbewusste Opferwahl. Die Verdrängung erfolgt ausschließlich ausdrücklich.
- Es muss zwei Stufen geben. Zusammenfassungen (Schwerpunkte, Präfixtabellen) sind klein und werden am längsten gehalten. Vektoren sind die übrigen Einträge.
- Zusammenfassungen und festgehaltene Einträge dürfen nie Opfer werden.
- Jede Verdrängung hat einen von drei Gründen: Leerlauf, Druck oder Platzbedarf.
- Die nutzbare Kapazität ist die Gesamtkapazität abzüglich Reserve und Ersetzungsspielraum. Die Belegung ergibt sich aus lebenden und zurückgestellten Bytes.
- Eine Verdrängungsrichtlinie besteht aus Hochmarke, Tiefmarke und Leerlaufdauer:
  - Nicht endliche Marken werden korrigiert. Die Hochmarke fällt dann auf eins, die Tiefmarke auf die Hochmarke.
  - Die Hochmarke wird auf null bis eins begrenzt, die Tiefmarke auf null bis zur Hochmarke.
  - Für den Grafikspeicher gilt als Voreinstellung 0,90 und 0,75 bei 600 Sekunden Leerlauf. Wiederladen ist teuer und belastet den Ersetzungsspielraum.
  - Für den Hauptspeicher gilt als Voreinstellung 0,85 und 0,70 bei 60 Sekunden Leerlauf, weil das Betriebssystem ihn ohnehin transparent auslagert.
- Ein Festhalten eines Eintrags muss so lange wirken, wie der zurückgegebene Halter lebt. Beim Freigeben des Halters endet es. Für unbekannte Einträge gibt es keinen Halter.
- Beim Aufnehmen eines Eintrags sind Größe, Stufe und Wiederladekosten anzugeben. Wiederladekosten gelten in beliebigen, aber einheitlichen Einheiten. Nicht endliche oder negative Kosten zählen als null.
- Ein neu aufgenommener Eintrag startet mit einem Treffer. Wird ein bereits bekannter Schlüssel erneut aufgenommen, ersetzt er den alten Eintrag und die Summe der Bytes bleibt korrekt.
- Die Priorität eines Eintrags ist der aktuelle Boden zuzüglich Trefferzahl mal Wiederladekosten geteilt durch Größe. Eine Größe von null zählt als eins.
- Der Boden steigt bei jeder Verdrängung unter Druck oder Platzbedarf auf die Priorität des Opfers. So altern unberührte Einträge ohne eigene Zugriffsliste.
- Ein Zugriff muss die Trefferzahl erhöhen, den Zeitpunkt der letzten Nutzung setzen und die Priorität neu berechnen. Er meldet, ob der Eintrag bekannt war.
- Ein Eintrag muss entfernbar sein. Seine Bytes werden zurückgemeldet.
- Anzahl, verfolgte Bytes und Enthaltensein müssen abfragbar sein.
- Der Leerlaufablauf muss alle verdrängbaren Einträge verdrängen, die mindestens die Leerlaufdauer unbenutzt sind. Die Reihenfolge ist deterministisch nach Schlüssel.
- Die Entlastung nach dem Hystereseprinzip:
  - Solange die Belegung die Hochmarke nicht übersteigt, passiert nichts.
  - Sonst werden Opfer gewählt, bis die Belegung auf oder unter der Tiefmarke liegt.
  - Die Opferreihenfolge ist: niedrigste Priorität zuerst, bei Gleichstand der am längsten unbenutzte, danach der kleinere Schlüssel.
- Der periodische Einstieg muss zuerst den Leerlaufablauf und danach die Entlastung ausführen. Die Belegung wird dabei um die schon freigegebenen Bytes verringert.
- Die Platzbeschaffung für einen Bedarf muss so arbeiten:
  - Ist genug frei, wird nichts verdrängt.
  - Sonst werden Opfer in Opferreihenfolge gesammelt.
  - Reichen alle verdrängbaren Einträge nicht, muss "Arena erschöpft" gemeldet werden, und zwar ohne vorher etwas zu verdrängen.
  - Andernfalls werden die gewählten Opfer mit dem Grund Platzbedarf verdrängt.
- Jede Verdrängungsentscheidung muss mit Opfer, Größe, Trefferzahl und Grund protokolliert werden.

## Zusammenfassen gleichzeitiger Ladevorgänge

- Gleichzeitige Anfragen mit demselben Schlüssel müssen sich einen einzigen Ladevorgang samt Ergebnis teilen, auch im Fehlerfall.
- Die Startfunktion darf nur für den ersten Aufrufer ausgeführt werden. Sie darf nur den Vorgang aufbauen und nie blockieren.
- Nach Abschluss muss der Eintrag entfernt werden. Aber nur der Eintrag des eigenen Vorgangs, nicht der eines inzwischen neu gestarteten.
- Ein fehlgeschlagener Ladevorgang muss vom nächsten neuen Aufrufer wiederholt werden, nie von den bereits Wartenden.
- Die Zahl laufender Ladevorgänge muss abfragbar sein. Das Mitlaufen eines weiteren Wartenden wird mit der Wartezahl protokolliert.

## Konsistenzmodell der Indexverwaltung

- Der Index ist die Quelle der Wahrheit. Der Grafikspeicher ist ein abgeleiteter, verwerfbarer Spiegel. Bei Abweichung gewinnt der Index.
- Die Identität eines Eintrags ist das Paar aus Kennung und Stempel. Stempel dürfen ausschließlich auf Ungleichheit verglichen werden, nie auf Größer oder Kleiner.
- Der Abgleich muss zustandsbasiert erfolgen. Das Änderungssignal weckt nur den Abgleich. Verglichen werden nur residente Einträge.
- Im Index gilt beim Schreiben die Reihenfolge Daten, Stempel, Signal. Beim Laden gilt die Reihenfolge Stempel, Daten, Stempel erneut.
- Segmenttabellen müssen kopierend verändert und über einen Tausch des Wurzelzeigers veröffentlicht werden. Eine abgelöste Version wird erst freigegeben, wenn alle Leser die neue Epoche erreicht haben. Änderungen der Zuordnung folgen der Tabelle in der Lücke zwischen den Stapeln.
- Es gilt: Der letzte Schreiber gewinnt. Die Anwendung von Änderungen auf den Index ist serialisiert.

## Ladestrategie

### Geometrie und Richtlinie

- Das System muss über eine austauschbare Schnittstelle erfragen können, zu welchem Segment ein Eintrag gehört, wie groß ein Eintrag und wie groß ein Segment ist.
- Eine einfache Geometrie gleich großer Einträge in aufeinanderfolgenden, gleich großen Segmenten muss mitgeliefert werden. Eine Segmentkapazität von null wird dabei als eins behandelt.
- Zwei Zugriffsarten müssen unterschieden werden. Regulärer Zugriff ist der Normalfall. Seltener Zugriff betrifft kleine, selten gelesene Mengen, die aus abgebildetem Hostspeicher bedient werden dürfen.
- Es gibt drei Planarten: ganze Segmente laden, einzelne Einträge sammeln und gemeinsam senden, und direkte Abbildung ohne Kopie.
- Die Ladericheinrichtung enthält drei Größen:
  - Eine Dichteschwelle, Standard 0,12. Sie gibt den Anteil der Segmentbytes an, ab dem das ganze Segment geladen wird.
  - Eine Segmentgröße, Standard 64 Mebibyte.
  - Eine Höchstgröße für direkte Abbildung, Standard 64 Kibibyte.
- Die Dichteschwelle darf nur auf endliche Werte gesetzt werden und wird auf null bis eins begrenzt.
- Die Segmentgröße muss aus dem freien Gerätebudget ableitbar sein. Sie beträgt ein Achtel der verfügbaren Menge, begrenzt auf 4 bis 256 Mebibyte und abgerundet auf ganze Mebibyte.
- Die Schwelle muss aus einem kalibrierten Profil übernommen werden können, wenn dieses die Kalibrierachse "Dichte" als ganze Zahl in Promille enthält. Sonst bleibt die Richtlinie unverändert. Der Wert der Achse muss als gerundeter Promillewert abrufbar sein.

### Planung

- Für eine leere Menge darf kein Plan entstehen.
- Bei seltenem Zugriff und einer Gesamtgröße bis zur Höchstgröße für direkte Abbildung muss ein einziger Plan mit direkter Abbildung entstehen.
- Sonst müssen die angeforderten Bytes je Segment ermittelt werden.
  - Erreicht der angeforderte Anteil die Dichteschwelle und ist das Segment nicht leer, wird das ganze Segment geladen.
  - Alle übrigen Einträge werden gesammelt.
- Es muss höchstens ein Plan für ganze Segmente und höchstens ein Plan für gesammelte Einträge entstehen. Die Segmente kommen zuerst.
- Jede Entscheidung muss mit Planart, Bytes und Anzahl protokolliert werden.

### Ausführung und Zusammenfassung

- Ein Übertragungsweg muss die drei Planarten ausführen und jeweils die bewegten Bytes melden.
- Die Pläne müssen nacheinander ausgeführt werden. Beim ersten Fehler bricht der Vorgang ab. Die Summe der Bytes ist überlaufsicher.
- Ein Sammler muss die Mengen mehrerer Aufgaben gegen denselben Index zu einer Vereinigung zusammenführen und die Zahl der zusammengeführten Anfragen mitführen.
- Der Sammler muss seinen Inhalt entleeren können. Ob er leer ist, muss abfragbar sein.
- Ein Durchspülen muss jeden Index genau einmal über die Vereinigung laden, nebenläufig, und je Index Anfrage und Ergebnis liefern.
- Identische Anfragen (gleicher Index, gleiche Menge) müssen sich einen einzigen Transfer teilen. Ein Fehler erreicht jeden Wartenden.
- Der Transfer muss im Pool für blockierende Arbeit laufen. Ein Abbruch dieser Aufgabe wird als Übertragungsfehler vom Host zum Gerät mit Begründung gemeldet.
- Das Ergebnis enthält die ausgeführten Pläne und die bewegten Bytes.

## Zusammenspiel von Tabelle, Zuordnung und geladenen Segmenten

- Eine Betriebseinheit je Indexoperation muss Manager, Tabellentausch, Zuordnungsspeicher und die Warteschlange geladener Segmente verbinden. Jedes vom Transfer gemeldete Segment muss dort veröffentlicht werden, ohne dass Aufrufer daran denken müssen.
- Strukturelle Änderungen müssen serialisiert sein, weil Tabellen- und Zuordnungsepoche gemeinsam voranschreiten.
- Beim Sicherstellen einer Menge gilt:
  - Der Ladevorgang wird ausgeführt.
  - Meldet er "Arena erschöpft", werden zuerst ausstehende Segmente veröffentlicht und dann Platz in der angeforderten Größe geschaffen. Danach wird der Vorgang genau einmal wiederholt.
  - Anschließend werden geladene Segmente veröffentlicht.
- Ein Abgleichdurchlauf muss die nachgeladenen Einträge noch vor seiner Rückkehr veröffentlichen und die Auslastung im Kontext des Durchlaufs protokollieren.
- Beim Abschluss eines Kommunikations-Slots gilt:
  - Hat sich die Zuordnung seit Stapelbeginn weiterentwickelt, wird der Slot unter der Strukturperre mit frischer Momentaufnahme wiederholt. Ohne diese Sperre könnte eine Momentaufnahme zwischen Tabellen- und Zuordnungsveröffentlichung fallen.
  - Nach dem Abschluss wird zurückgestellter Speicher zurückgewonnen.
- Die Verdrängung muss aus dem Arena-Budget die Belegung bilden, die Verdrängung ausführen und die verdrängten Segmente aus dem Betrieb nehmen.
- Das Entfernen verdrängter Segmente erfolgt je Segment so:
  - Das Segment wird aus der neuen Tabelle und aus der neuen Zuordnung entfernt.
  - Beide werden veröffentlicht.
  - Der Bereich wird hinter der Epoche der neuen Tabelle zurückgestellt.
  - Der Manager gibt das Segment frei.
  - Segmente ohne bekannte Region werden übersprungen.
  - Schlägt das Entfernen fehl, kommt das Segment wieder unter Verdrängungskontrolle und der Vorgang bricht mit dem Fehler ab.
- Das Veröffentlichen geladener Segmente muss alle seit dem letzten Aufruf gemeldeten Segmente versuchen. Erst danach wird der erste aufgetretene Fehler gemeldet, sonst die Anzahl.
- Die Rückgewinnung muss alles freigeben, was bis zur zuletzt veröffentlichten Epoche abgelöst wurde und kein Leser mehr sieht.
- Fehler der Verwaltung müssen in Fehler des Index übersetzt werden. "Arena erschöpft" bleibt erhalten, alle anderen werden zu einem Übertragungsfehler mit Fehlertext.

## Auflösung des Aufgabenumfangs

- Der Manager muss den Umfang einer Aufgabe in eine kompakte Menge von Einträgen auflösen.
- Es gibt sechs Arten von Umfängen: alle Einträge, eine explizite Menge, ein Bereich, ein Teilbaum, ein Präfix und eine Schwerpunktauswahl.
- "Alle" liefert alle Einträge des Index. Eine explizite Menge wird unverändert übernommen.
- Alle anderen Arten brauchen eine Zugriffsmethode. Unterstützt der Index sie nicht oder ist kein Auflöser dafür registriert, muss der Fehler "nicht unterstützter Umfang" mit Methodenname gemeldet werden.
- Eine Registrierung muss vier Standardauflöser enthalten können: Bereich, Teilbaum, Präfix und Schwerpunkt. Mehrere Auflöser für dieselbe Methode sind unzulässig, ein Doppelregistrieren muss abgewiesen werden.
- Der Bereichsauflöser liefert genau den angegebenen Bereich.
- Der Teilbaumauflöser liefert die Kinder des angegebenen Knotens.
- Der Präfixauflöser interpretiert das Präfix als führende Bytes der 64-Bit-Kennung in Big-Endian-Reihenfolge. Daraus entsteht ein zusammenhängender Bereich:
  - Ein Präfix über 8 Bytes trifft nichts.
  - Ein leeres Präfix deckt den gesamten Kennungsraum ab.
  - Die Obergrenze wird auf den größten darstellbaren Wert begrenzt.
- Der Schwerpunktauswahlauflöser arbeitet wie folgt:
  - Er braucht die Schwerpunktzusammenfassung des Index. Fehlt sie, meldet er "Index nicht verfügbar".
  - Er braucht einen Zellenwähler im Kontext. Fehlt er, meldet er "nicht unterstützt".
  - Der Zellenwähler wählt anhand der Zusammenfassung die nächstgelegenen Zellen. Die Eingaben sind die Anzahl k und ein optionaler Radius.
  - Es werden höchstens k Zellen berücksichtigt. Das Ergebnis ist die Vereinigung ihrer Mitglieder.
- Erhält ein Auflöser einen Umfang der falschen Art, muss er "nicht unterstützter Umfang" melden.
- Die Auflösung muss in einem Diagnosebereich mit Methode ablaufen und die Anzahl aufgelöster Einträge protokollieren.

## Konsistente Momentaufnahmen gemeinsam genutzter Daten

- Das System muss Indexdaten, die ein anderer Schreiber gleichzeitig verändern darf, so lesen können, dass nie ein halb geschriebener Zustand sichtbar wird. Der Zugriff erfolgt wortweise atomar.
- Der Leser muss eine vollständige Kopie liefern und die Zahl der notwendigen Wiederholungen zurückmelden. Er wiederholt, solange ein Schreiber aktiv ist oder sich der Zähler während des Lesens geändert hat.
- Mehrere Schreiber müssen sich gegenseitig ausschließen. Ein Schreibvorgang muss die gesamte Region abdecken. Eine abweichende Länge ist ein Programmierfehler.
- Pro Schreibvorgang muss der Zähler um zwei steigen.
- Das System muss aus rohen Adressen aufgebaut werden können, sofern Ausrichtung, Gültigkeit und das einheitliche Protokoll aller Schreiber zugesichert sind.

## Kommunikations-Slot zwischen Stapeln

### Ablauf

- Die Phasen müssen in fester Reihenfolge laufen: Zuerst werden Schreibvorgänge des Kernels im Index registriert. Dann werden Ergebnisse aufgelöst. Erst danach wird die Zuordnung verändert.
- Zu Beginn des Slots müssen zuvor zurückgestellte, nicht zuordenbare Schreibvorgänge erneut versucht und mit den neuen zusammen verarbeitet werden.
- Ein Schreibvorgang wird Platz für Platz abgearbeitet:
  - Ist die Adresse keinem Segment zugeordnet, wird der Rest des Schreibvorgangs für den nächsten Slot zurückgestellt.
  - Ist der Platz an einen Eintrag gebunden, werden die geschriebenen Bytes des Platzes vom Gerät zurückgelesen und auf den Index angewendet. Der Stempel wird zusammen mit dem Platz im Residenzverzeichnis vermerkt.
  - Plätze ohne Eintrag werden übersprungen.
  - Der Fortschritt beträgt mindestens ein Byte.
- Danach wird jeder Treffer aufgelöst:
  - Lässt sich seine Adresse nicht auf einen Eintrag abbilden, wird er verworfen.
  - Kennt der Index den Eintrag nicht mehr, wird er verworfen.
  - Liegt für den Eintrag ein Update an, wird der Treffer mit Kennzeichen "veraltet" trotzdem geliefert.
  - Sonst wird er unverändert geliefert.
- Zuletzt müssen die anstehenden Updates angewendet werden. Der Ablauf wird mit Anzahl angewandter Schreibvorgänge, aufgelöster, verworfener und veralteter Treffer, Warteschlangentiefe und Zahl der Änderungen protokolliert.
- Ein Treffer enthält den Eintrag, die Bewertung und das Veraltet-Kennzeichen.
- Die Ausführung gegen die Zuordnung der Stapelepoche darf nur laufen, wenn die Zuordnung nicht weitergegangen ist. Sonst muss "veraltete Zuordnung" ohne Nebenwirkung gemeldet werden, damit der Aufrufer es mit frischer Momentaufnahme erneut versucht.
- Das Zurücklesen geschriebener Bytes eines Gerätebereichs muss über eine austauschbare Schnittstelle erfolgen.

### Adresskarte

- Die Adresskarte muss Arena-Adressen auf Einträge abbilden. Ihre Segmentmenge wird kopierend verändert. Ein Klon ist die stabile Momentaufnahme eines Stapels, spätere Änderungen erreichen ihn nicht.
- Eine Epoche muss mitgeführt werden. Ein Nachfolger übernimmt die Segmente und erhöht die Epoche um eins.
- Das Hinzufügen eines Segments muss abgelehnt werden, wenn das Segment leer ist, seine Kennung schon vergeben ist oder sein Adressbereich einen anderen überlappt. Segmente werden nach Adressen sortiert gehalten.
- Das Entfernen meldet, ob das Segment vorhanden war.
- Die Bindungen von Plätzen an Einträge liegen in gemeinsam genutzten Platzzuordnungen und dürfen nur innerhalb der Lücke zwischen den Stapeln verändert werden.
- Ein Zuordnungsspeicher muss die aktuelle Karte versioniert halten und Momentaufnahmen liefern. Eine Momentaufnahme für einen Stapel wird nur geliefert, wenn die Epoche übereinstimmt, sonst "veraltete Zuordnung" mit Basis- und veröffentlichter Epoche. Eine neue Karte darf nur veröffentlicht werden, wenn ihre Epoche genau der aktuellen plus eins entspricht.

### Update-Warteschlange

- Es muss zwei Arten von Updates geben: das Lösen der Bindung eines Eintrags und das Binden eines Eintrags an einen Platz mit Stempel.
- Je Eintrag darf nur das jüngste Update gehalten werden. Enthaltensein, Tiefe und Zahl zurückgestellter Schreibvorgänge müssen abfragbar sein.
- Beim Anwenden werden zuerst alle Löschungen, dann alle Bindungen ausgeführt:
  - Beim Lösen wird der Eintrag aus dem Residenzverzeichnis entfernt. Die Platzbindung wird nur gelöst, wenn sie noch zu diesem Eintrag gehört.
  - Beim Binden wird zuerst eine alte Bindung desselben Eintrags gelöst. Dann wird neu gebunden und das Verzeichnis aktualisiert.
  - Ist das Ziel nicht abgebildet, wird gewarnt und der Eintrag nicht als residente vermerkt.
- Die Zahl angewendeter Updates muss zurückgegeben werden.

## Indexmanager

### Gemeinsames Verhalten

- Ein Manager muss einen Index erst bei Bedarf anhängen. Bis dahin hält er keine Ressourcen und hat keine Abonnements.
- Ein Anhängen muss "wahr" liefern, wenn der Index angenommen wurde. Ist bereits einer angehängt, bleibt der erste bestehen und es wird "falsch" geliefert und gewarnt.
- Eine Aktivierung mit Initialisierungsfunktion darf die Initialisierung nur beim ersten Gebrauch ausführen. Gleichzeitige Aufrufer warten auf dieselbe. Schlägt sie fehl, bleibt der Manager inaktiv.
- Der Zugriff auf einen noch nicht angehängten Index muss mit "Index nicht verfügbar" und der Indexkennung abgewiesen werden.
- Ob ein Manager aktiv ist, muss abfragbar sein. Das gilt für beide Varianten über dieselbe Schnittstelle.
- Der Manager muss die Zahl residenter Einträge liefern und für Zyklen einen Diagnosebereich mit der Indexkennung bereitstellen.

### Variante für den Hauptspeicher

- Im Hauptspeicher gehen Lesezugriffe direkt an den Index. Die einzige Aufgabe ist die Residenz der dateigestützten Daten im Seitencache.
- Das Lesen eines Eintrags muss einen Hinweisbereich entgegennehmen, den Hinweis "wird bald benötigt" setzen und das Lesen im Pool für blockierende Arbeit ausführen. Geliefert wird eine Kopie der Bytes. Bricht die Aufgabe ab, muss ein Übertragungsfehler "Lesetask abgebrochen" gemeldet werden.
- Das Lesen mehrerer Einträge muss ein Zugriffsmuster entgegennehmen:
  - Bei dichtem Zugriff wird über den umspannenden Bereich aller Einträge der Hinweis "sequentiell" gesetzt.
  - Bei verstreutem Zugriff wird für jeden Eintragsbereich einzeln der Hinweis "wahlfrei" gesetzt.
  - Die Ergebnisse folgen der Eingabereihenfolge.
  - Eine leere Eingabe liefert ein leeres Ergebnis. Der Index muss dennoch angehängt sein.
- Eine weitere Variante muss die Hinweisbereiche selbst vom Index erfragen. Einträge ohne zusammenhängende Speicherung werden ohne Hinweis gelesen.
- Der Manager muss einen Bereich als bald nicht benötigt markieren können, beispielsweise nach Leerlauf. Ohne Hinweisquelle passiert nichts, eine Ablehnung wird nur protokolliert.

### Variante für den Grafikprozessor

- Der Manager muss Umfänge über seine Auflöserregistrierung auflösen.
- Das Sicherstellen einer Menge verlangt einen angehängten Index, führt den Ladevorgang über die Ladeeinrichtung aus und markiert danach alle Segmente als benutzt. Bei bewegten Bytes wird das protokolliert.
- Ein Abgleichdurchlauf muss folgendes tun:
  - Der Abgleich wird beim ersten Mal angelegt.
  - Der Abgleich vergleicht die residenten Einträge mit dem Index.
  - Entfernte Einträge werden als Lösch-Updates eingestellt.
  - Geänderte Einträge ohne ausstehendes Update werden regulär nachgeladen.
  - Das Ergebnis des Abgleichs wird zurückgegeben.
- Ein Verdrängungsschritt und eine Platzbeschaffung müssen verfügbar sein. Jede Verdrängung wird mit Index, Segment, Bytes und Grund protokolliert.
- Das Veröffentlichen eines geladenen Segments muss so ablaufen:
  - Es erhält eine fortlaufende Segmentkennung.
  - Die Region wird in eine neue Tabellenversion aufgenommen.
  - Die Segmentzuordnung wird in eine neue Zuordnungsepoche aufgenommen. Weist die Zuordnung das Segment ab, etwa wegen Überlappung, wird ein Übertragungsfehler gemeldet.
  - Zuerst wird die Tabelle veröffentlicht, danach die Zuordnung.
  - Bei jedem Fehler wird die Region zurückgegeben.
  - Bei Erfolg wird das Segment verfolgt: Größe als Eintragsgröße, Stufe Vektor, Wiederladekosten gleich der Größe, Zeitstempel jetzt.
  - Die mitgelieferten Bindungen werden mit Segmentkennung und Stempel als Updates eingestellt, sie wirken in der nächsten Lücke.
- Die Zahl der Segmente, die gesamten residenten Bytes aus Sicht der Verdrängung und die Zahl ausstehender Updates müssen abfragbar sein.
- Das Freigeben eines Segments muss dessen Region zurückgeben, es aus der Verdrängung entfernen und die Residenz aller seiner Einträge löschen. Noch anstehende Bindungen in dieses Segment scheitern dann ohne Schaden.
- Der Abschluss eines Slots muss einen angehängten Index verlangen, Updates und Residenz sperren und den Slot gegen die Zuordnung der Stapelepoche ausführen.

## Mutationsdisziplin: Tabellentausch und Epochen

### Wurzelzeiger

- Der Wurzelzeiger muss eine natürlich ausgerichtete 8-Byte-Zelle sein, die nur der Host beschreibt und das Gerät mit Erwerbs-Semantik liest.
- Das Veröffentlichen muss mit Freigabe-Semantik erfolgen. Alles zuvor Geschriebene ist für jeden sichtbar, der den neuen Wert sieht.
- Vor der ersten Tabelle muss die Wurzel einen festen Leerwert tragen, den größten darstellbaren Wert.
- Die Zelle muss in fest zugeordnetem, abgebildetem Hostspeicher liegen können.

### Segmenttabelle

- Eine veröffentlichte Tabelle ist unveränderlich. Jede neue Version ist eine Kopie mit erhöhter Epoche.
- Sie enthält Epoche und die Regionen der Segmente.
- Operationen:
  - Eine Region lässt sich hinzufügen.
  - Eine Region lässt sich durch eine andere ersetzen. Ist die alte nicht enthalten, wird "falsch" geliefert.
  - Eine Region lässt sich entfernen. Das Ergebnis zeigt, ob sie enthalten war.
- Die kodierte Größe der geräteseitigen Tabelle beträgt 16 Bytes zuzüglich 24 Bytes je Segment.

### Epochenverfolgung

- Das System muss die veröffentlichte Epoche und für jeden Leser die zuletzt gemeldete Epoche führen.
- Ein beendeter Stapel meldet die gelesene Epoche. Gemeldete Epochen dürfen nur steigen.
- Ein Leser ohne laufenden Stapel kann in den Ruhezustand versetzt werden. Er gilt dann als auf der veröffentlichten Epoche angekommen, weil alles Folgende die veröffentlichte oder eine neuere Wurzel liest.
- Die kleinste Epoche aller Leser muss abfragbar sein. Ohne Leser entspricht sie der veröffentlichten Epoche.

### Tabellentausch

- Das Schreiben einer Tabelle in gerätesichtbaren Speicher muss erst zurückkehren, wenn der Inhalt vollständig und für das Gerät sichtbar ist. Das gilt nach Abwarten des Kopierereignisses oder nach der Freigabe der abgebildeten Schreibvorgänge.
- Eine Kopie der aktuellen Tabelle mit nächster Epoche muss bereitgestellt werden.
- Das Veröffentlichen muss serialisiert erfolgen:
  - Die Epoche der neuen Tabelle muss genau der aktuellen plus eins entsprechen. Sonst wird "veraltete Zuordnung" gemeldet. Zwei auf gleicher Basis gebaute Tabellen können deshalb nicht beide veröffentlicht werden.
  - Zuerst wird der Inhalt sichtbar geschrieben, dann die Wurzel getauscht, dann die Epoche vorangeschoben und die aktuelle Tabelle ersetzt.
  - Die Region der vorherigen Tabelle wird mit der neuen Epoche zurückgestellt.
  - Epoche, Segmentzahl und Offset werden protokolliert.
  - Die Epoche wird zurückgegeben.
- Die Rückgewinnung bis zu einer Epoche muss höchstens bis zur kleinsten Leserepoche und höchstens bis zur gewünschten Obergrenze freigeben. Sie meldet die Zahl der Freigaben.
- Eine Momentaufnahme aus Wurzel und Epoche muss für normale Starts verfügbar sein. Die Version bleibt für die Dauer eines Starts konstant, ein Tausch wirkt erst beim nächsten Start.
- Ein dauerhaft laufender Kernel darf pro Stapel nur einmal lesen. Ein Schutzobjekt hält die gelesene Version fest und meldet beim Freigeben die Epoche des Lesers.
- Der Kernel darf nie blockiert oder ausgesperrt werden.
- Wurzel und aktuelle Tabelle müssen abfragbar sein.
- Auf einem echten Gerät muss nachweisbar sein, dass der Inhalt vor dem Wurzelwert sichtbar ist. Das gilt auch für Tabellen, die über einen Kopierstrom mit Ereignis übertragen werden.

## Zustandsbasierter Abgleich mit dem Index

- Der Abgleich muss mit einem Änderungssignal des Index aufgeweckt werden. Die Korrektheit stützt sich ausschließlich auf den Vergleich der Stempel.
- Jeder Abonnent (Grafikmanager, Persistenz-Zeitgeber, Hostspiegel) hat seinen eigenen Abgleich mit eigener Sicht darauf, was er schon gesehen hat.
- Der Abgleich muss warten können, bis ein ungesehenes Signal vorliegt. Er meldet "falsch", wenn der Index nicht mehr existiert. Ob ein Signal ansteht, muss abfragbar sein.
- Beim Vergleich wird das Signal vor dem Vergleich bestätigt. Ein während des Abgleichs eintreffendes Signal bleibt ungesehen und löst den nächsten Abgleich aus.
- Verglichen werden nur Einträge, die residente sind:
  - Fehlt ein Eintrag im Index, gilt er als entfernt.
  - Weicht der Stempel ab, wird er zum Nachladen vorgemerkt.
  - Stimmt er überein, zählt er als aktuell. Das schließt Echos eigener Schreibvorgänge ein.
- Das Ergebnis enthält die beiden Listen aufsteigend sortiert, die Zahl aktueller Einträge und den Wasserstand des Signals. Es muss sagen können, ob es sauber ist, und die Entfernungen als Lösch-Updates einstellen können. Entfernungen sind Änderungen der Zuordnung.
- Das Laden eines Eintrags muss in der Reihenfolge Stempel, Daten, erneuter Stempel erfolgen. Ein nach den Daten gelesener Stempel könnte veraltete Daten als aktuell ausweisen. Die umgekehrte Reihenfolge riskiert höchstens ein überflüssiges Nachladen.
- Hat sich der Stempel während des Ladens bewegt, wird wiederholt, höchstens achtmal. Danach wird mit dem zuerst gelesenen Stempel zurückgekehrt.
- Das Ergebnis enthält den zuerst gelesenen Stempel, die Daten und die Zahl der Wiederholungen. Existiert der Eintrag nicht, wird nichts geliefert.

## Residenzverzeichnis

- Das System muss für jeden residenten Eintrag den Platz (Segment und Platznummer) und den bekannten Stempel führen.
- Es muss ein Eintrag eingefügt werden können, wobei ein vorheriger Eintrag zurückgegeben wird. Außerdem müssen Entfernen, Abfragen des Eintrags oder nur des Stempels, Anzahl, Leere und Durchlaufen aller Einträge möglich sein.

## Übertragung auf das Gerät

### Bausteine

- Es muss eine Quelle für Segmentbereiche geben, die Bereiche zuteilt und freigibt. Die Indexarena erfüllt diese Aufgabe.
- Es muss einen abgebildeten Hostspeicher geben, der Bereiche anlegt, beschreibt und freigibt. Auf dem Grafikprozessor wird dafür der Speicherverwalter verwendet.
- Ein geladenes Segment besteht aus Region, ungebundener Platzzuordnung und der Liste einzustellender Bindungen mit Eintrag, Platz und dem vor den Daten gelesenen Stempel.
- Die Meldung geladener Segmente muss über eine austauschbare Senke laufen. Eine Warteschlange sammelt sie und liefert ihren gesamten Inhalt auf einmal. Anzahl und Leere sind abfragbar.

### Ganze Segmente

- Beim Laden ganzer Segmente werden dem Segment die Einträge zugeordnet, die die Geometrie ihm zuweist, aufsteigend sortiert.
- Je Segment wird ein direktes Kopieren versucht. Das setzt voraus, dass die Einträge lückenlos hintereinander in einem gestützten Bereich liegen, jeder nicht leer ist und jeder Start relativ zum Bereichsanfang ein Vielfaches der Ausrichtung ist.
- Der Index muss die Quelldaten mit genau der Länge des Bereichs liefern.
- Vor dem Kopieren wird der Hinweis "sequentiell" gesetzt, falls eine Hinweisquelle existiert.
- Ein Bereich wird angelegt, kopiert und synchron abgewartet. Bei Fehlern wird er freigegeben.
- Das Segment wird gemeldet. Die Platzgrenzen sind absolute Adressen aus Region und Platzierung mit einer Endmarke.
- Nur Einträge mit bekanntem Stempel erhalten eine Bindung. Der Platz ist der Index der Platzierung.
- Lässt sich ein Segment nicht direkt kopieren, werden seine Einträge dem Sammelpfad übergeben.
- Zurückgegeben wird die Summe aller bewegten Bytes.

### Sammelpfad

- Die Stempel aller Einträge werden vor dem Lesen der Daten erfasst.
- Gibt es eine Hinweisquelle, wird für jeden Eintragsbereich der Hinweis "wahlfrei" gesetzt. Ablehnungen werden nur protokolliert.
- In einer Schleife wird Folgendes wiederholt, bis alle Einträge erledigt sind:
  - Ein Staging-Platz wird bepackt.
  - Macht ein Durchgang keinen Fortschritt, wird ein Übertragungsfehler "Sammeln ohne Fortschritt" gemeldet.
  - Bei mehr als null Bytes wird ein Bereich angelegt, die Kopie abgesandt und abgewartet.
  - Das Segment wird gemeldet.
  - Bei Fehlern wird der Bereich freigegeben.
  - Die Bytes werden aufsummiert.
- Das Bepacken eines Staging-Platzes ist eine Aufgabe der gestuften Sammlung:
  - Sie erwirbt einen freien Platz. Sind alle noch in Übertragung, wartet sie.
  - Sie packt Einträge von vorn, jeden auf die geforderte Ausrichtung, bis der nächste nicht mehr passt.
  - Entfernte Einträge mit leeren Daten werden ohne Platzierung als erledigt gezählt.
  - Passt bereits der erste Eintrag nicht in den Platz, wird "Arena erschöpft" mit Eintragslänge und Platzkapazität gemeldet.
  - Bei jedem Fehler wird der Platz freigegeben.
  - Das Lesen aus dem Index kann dabei blockieren.
- Das vorbereitete Paket enthält Platzierungen, benötigte Zielgröße und Zahl erledigter Eingabeeinträge. Platzierungen sind Bytebereiche relativ zum Zielbereichsanfang.
- Das Absenden führt genau eine Kopie der gepackten Bytes ins Ziel aus. Der Platz wird erst nach dem zugehörigen Ereignis wiederverwendet. Bei Fehlern wird er sofort freigegeben.
- Wird das vorbereitete Paket verworfen, wird der Platz freigegeben.
- Die Kopie auf das Gerät ist über eine austauschbare Schnittstelle asynchron. Die Quelle muss bis zum Abschluss des Ereignisses unverändert bleiben.
- Die Kopie läuft auf dem Strom einer eigenen Kopierlane. Danach wird ein Ereignis aufgezeichnet.
- Als Staging-Speicher müssen gewöhnlicher Hostspeicher und fest zugeordneter Hostspeicher unterstützt werden. Fehler beim Zugriff werden als Übertragungsfehler gemeldet.

### Direkte Abbildung ohne Kopie

- Ohne angeschlossene abgebildete Arena oder wenn diese keinen Platz hat, muss auf den Sammelpfad zurückgefallen und dies protokolliert werden.
- Sonst werden Stempel und Daten gelesen. Leere Einträge werden ignoriert. Die übrigen werden mit Ausrichtung platziert.
- Ergibt sich keine Byte, werden null Bytes gemeldet.
- Die Daten werden in den abgebildeten Bereich geschrieben. Bei Fehlern wird er freigegeben und der Fehler gemeldet.
- Das Segment wird gemeldet.
