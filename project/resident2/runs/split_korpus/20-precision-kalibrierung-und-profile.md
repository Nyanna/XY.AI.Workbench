## Precision, Kalibrierung und Profile

### Prinzipien

- Zwischen CPU und GPU gibt es keinen Rounding- oder Datentyp-Vertrag.
- Der Precision-Parameter ist ein Wunsch zwischen null (schnellste) und eins (präziseste Kombination), keine Zusicherung.
- Die Kalibrierung unterbricht oder blockiert den Nutzer nie.

### Capability-Achsen und Capability-Sets

- Achsen und Werte werden über frei wählbare, erweiterbare Strings identifiziert, vergleichbar, sortierbar, als Key nutzbar, günstig kopierbar, lesbar ausgebbar. Werte sind auch aus natürlichen Zahlen erzeugbar.
- Ein Capability-Set ordnet jeder Achse eine nie leere Wertemenge zu, mit Add, Containment, Enumeration und Leere. Enumeration, Vergleich, Hash und Serialisierung sind reihenfolgeunabhängig.
- Ein Set erfüllt ein Requirement, wenn jeder geforderte Wert vorhanden ist. Leere Requirements sind immer erfüllt.
- Vordefinierte Achsen: Operanden, Akkumulator, Scoring, Fusion, Batch-Größe, Tile-Größe, Thread-Anzahl, Dichte. Werte: FP32, FP16, FP16 gespeichert mit FP32 gerechnet, INT8, INT8 über den DP4A-Befehl der GPU, INT8 über VNNI-Befehle der CPU (Integer-Befehle für neuronale Netze), on, off.
- Referenz-Sets als Obergrenzen: GPU mit Operanden und Akkumulator FP32 und FP16, Scoring FP32, FP16 und INT8-DP4A, Fusion on und off. CPU mit Operanden FP32 und FP16 mit Konvertierung, Scoring FP32, INT8 und INT8 über VNNI. Numerische Achsen werden zur Laufzeit ermittelt.
- Es gibt keine Profilnamen und keine fest einkodierte Profil-Enum. Achsen und Werte sind je Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle.

### Capability-Erkennung

- CPU: FP32 immer, FP16 mit Konvertierung bei Half-Precision-Konvertierung (F16C), INT8-Scoring bei AVX2 oder VNNI, Letzteres nur mit diesen Befehlen. Thread-Anzahlen als Zweierpotenzen bis zur Zahl logischer Cores plus die Core-Zahl selbst, mindestens ein Core. Features (AVX2, F16C, FMA, VNNI) werden vom System ermittelt, auf fremden Architekturen gelten sie als fehlend.
- GPU: FP32 immer, FP16 ab Compute Capability 5.3, INT8-DP4A ab 6.1, Fusion on und off. Tensor Cores werden anhand von Capability und Device-Name erkannt (Baureihen TU116 und TU117 haben trotz SM 7.x keine) und nur geloggt. Ohne GPU-Unterstützung oder Device-Context scheitert die Erkennung mit einem Capability-Error.
- Numerische Achsen aus der Konfiguration: Batch-Größen 1, 8, 32, Tile-Größen Zweierpotenzen von 16 bis 512, maximale Sequenzlänge 512. Nur positive Batch-Größen und Tiles bis zur Sequenzlänge. Die Kernel-Menge ist vorgebbar, sonst gilt der eingebaute Katalog.
- Erkannte Capabilities werden auf vorhandene Kernel reduziert. Unbekannte Achsen bleiben, geleerte Katalogachsen entfallen, Entferntes wird geloggt. Das Ergebnis wird mit Subengine, Achsen- und Wertezahl geloggt. Die Auswahl des Verfahrens erfolgt zentral nach Device-Art.

### Benchmark-Workload und Abweichungsmaße

- Reproduzierbare synthetische Workload aus Seed, normiert und duplikatfrei: Default 20 000 Dokumente, 384 Dimensionen, höchstens 32 Queries, die Top-5 für die Rangprüfung. Referenz ist das FP32-Scoring auf der CPU.
- Query-Zahl aus der Batch-Größe, ohne Angabe acht, begrenzt auf die Höchstzahl. Nicht numerische, null oder zu große Werte machen die Kombination unbrauchbar. Kleinere Batches nutzen die ersten Queries.
- Cosine-Deviation ist eins minus Cosine Similarity, mindestens null, bei Nullvektor eins. Werte unter einem Millionstel gelten als Rauschen.
- Relativer Fehler ist Abstand durch Referenzlänge, bei Nullreferenz null oder unendlich.
- Top-k-Overlap ist der Anteil der besten Referenztreffer im Kandidaten, Gleichstände nach Position, bei null gleich eins.
- Gesamtbewertung zeilenweise mit der schlechtesten Zeile, je Zeile das Größere aus Cosine-Deviation und eins minus Overlap.
- Validity-Zustände: valid, non-finite, degenerate, Abweichung über Maximum, Ausführung gescheitert, jeweils mit Text. Ungleiche Längen oder unteilbare Daten sind degenerate. Eine Zeile ist degenerate, wenn sie konstant ist (Varianz unter 10⁻⁹ des mittleren Quadrats) oder ihr Längenverhältnis außerhalb von 0,5 bis 2,0 liegt, konfigurierbar.

### Mess-Interface und Messmittel

- Austauschbares Mess-Interface (Trait): Abweichungsmessung gegen die Referenz (sonst „unverträglich“) und Zeit-Probe mit normierter Zeit je Arbeitseinheit oder „unterbrochen“, wenn eine produktive Aufgabe das Device anfordert.
- Warm-up beim ersten Gebrauch, Wiederholungen auf das Zeitfenster skaliert (Default zehn Millisekunden), Einzel-Launches im Chunk-Budget (Default fünf Millisekunden).
- Eine Kombination ist ein Wert je Achse plus die Sequenzlängen des Test-Sets, die nur Verarbeitbarkeit prüfen. Textform aus Achse-Wert-Paaren.
- Messergebnis: robuste Zeit, Abweichung, Validity, Stabilität, Probenzahl, relative Unsicherheit.
- Austauschbar sind die Abfrage wartender produktiver Aufgaben (Default nie, auch über einen gemeinsamen Schalter) und die monotone Zeitquelle (Clock).
- CPU (4 000 Dokumente): nur FP32, positive Thread-Anzahl (Default eins) mit eigenem Pool, parallele Berechnung, einmaliges Warm-up je Kombination, Zeit je Durchlauf und Query, Unterbrechungsprüfung zwischen Durchläufen.
- GPU (Default-Maße): Cosine-Pfad mit FP32 oder FP16 aus Scoring- oder Operanden-Achse, Dokumente in beiden Precisions auf dem Device, Zeit per Events auf dem Stream, 64 MiB Cache-Flush vor jeder Probe, mindestens 100 Millisekunden Warm-up. Wiederholungen eins bis 10 000, Gruppen im Chunk-Budget, je Kombination gemerkt, Unterbrechungsprüfung zwischen Gruppen. Ohne GPU-Unterstützung Capability-Error.

### Kalibrierungslauf

- Ziel sind Subengine und Model-ID. Der Kalibrierer zählt Kombinationen auf, misst interleaved gegen die Referenz, verwirft ungültige und bildet die Pareto-Front über Zeit und Abweichung. Obergrenzen beenden eine Messung mit dem besten, als instabil markierten Zwischenstand.
- Konfiguration (Default in Klammern): maximale Abweichung (0,05), Mindestwiederholungen (3, kleinere werden angehoben), Zeitfenster (zehn Millisekunden), Stabilitätsschwelle (0,05), Streuungsmaß Variationskoeffizient (CV) oder Spannweite durch Median (CV), Schätzer Median oder Minimum (Median), harte Obergrenze (40 Versuche, optional 30 Sekunden), Chunk-Budget (fünf Millisekunden), Gesamtbudget (optional 600 Sekunden), Dominanz-Marge (zehn Prozent), Backoff-Pause (fünf Millisekunden), separierbare Achsen (Batch-Größe, Tile-Größe, Thread-Anzahl), Vollmessung des kartesischen Produkts als Referenz (aus), Sequenzlängen ab 16 in Viererschritten bis zur Obergrenze einschließlich (Default 16, 64, 256, 512).
- Ablauf: leeres Capability-Set ergibt Capability-Error, leere Achsen werden ignoriert, Werte numerisch aufsteigend, dann alphabetisch. Achsen in Kern- und separierbare Achsen geteilt. Im Normalmodus zuerst die Achsenstufe, dann der Verifikationsschritt mit den Kandidaten, im Vollmodus das ganze Produkt. Die Front stammt nur aus dem Verifikationsschritt. Report mit Produktgröße, Budgetstatus und Laufzeit, Zusammenfassung und Log-Abschnitt.
- Achsenstufe (Pruning): Je Kernkombination wird jede separierbare Achse einzeln variiert, die anderen auf ihrem kleinsten Wert. Einwertige Achsen gewinnen automatisch. Gewinner sind die Pareto-optimalen Werte; ohne Gewinner wird die Kernkombination übersprungen. Kandidaten sind das deduplizierte Produkt. Das Budget wird vor jeder Kernkombination geprüft.
- Messen einer Gruppe: zuerst die billige Abweichungsphase mit Verwerfung (gescheitert, non-finite, über Maximum, sonstiger Grund), andere Fehler brechen ab. Danach rundenweise Zeitmessung Round-Robin über alle aktiven Kombinationen, in ungeraden Runden in umgekehrter Reihenfolge, damit Lastschwankungen alle gleich treffen. Die erste Kombination ist Anchor, Stabilität wird am Zeitverhältnis zum Anchor derselben Runde beurteilt (nur bei positiver Anchor-Probe). Jeder Versuch zählt, Unterbrechungen lösen die Backoff-Pause aus.
- Beendigung: Die Streuung ignoriert das langsamste Fünftel, ist bei weniger als zwei Proben unendlich. Eine Nicht-Anchor-Kombination endet vorrangig als „dominated“ (bei genug Proben ist eine andere bei höchstens gleicher Abweichung um mehr als die Marge schneller), sonst „stable“ bei stabiler Verhältnisreihe, sonst „unstable“ an der Obergrenze. Der Anchor endet zuletzt, stable bei stabilem Mitbewerber ohne instabilen oder eigener stabiler Reihe.
- Ergebnis je Kombination: dominated, verworfen mit Grund, übersprungen (Budget oder Device nie verfügbar) oder gemessen mit Zeit, Abweichung, Stabilität, Proben und Unsicherheit (ersatzweise Dominanz-Marge). Bei Budgetende aktive Kombinationen mit Proben gelten als gemessen und unstable.
- Report: Ziel, Einträge mit Stufe, Kombination und Ergebnis, Front aufsteigend nach Zeit mit streng fallender Abweichung, Zähler und Log jeder Einordnung.

### Pareto-Front, Positionen und Profile

- Die Front minimiert Zeit und Abweichung ohne nicht endliche Punkte, aufsteigend nach Zeit. Ein Punkt bleibt nur bei streng kleinerer Abweichung als alle schnelleren. Berechenbar für Zahlenpaare und beliebige Elemente (generisch).
- Positionen aus kumulierter Bogenlänge auf min-max-normierten Achsen, Zeit optional logarithmisch. Leere Front ohne Positionen, ein Punkt bei 0,5, Gesamtlänge null ergibt gleichmäßige Verteilung.
- Ein Profil ist unbenannt, mit Position zwischen null und eins und einem Wert je Achse. Positionen außerhalb sind „ungültige Precision“.
- Einzige Stellgröße ist der Merge-Threshold auf der normierten Skala (Default 0,1), modell- und hardwareunabhängig. Ableitung: nicht endliche Punkte entfernen; Punkte entfernen, die weniger präzise sind, ohne signifikant schneller zu sein (geloggt), etwa wegen gleicher Emulationspfade oder langsamer emulierter Precision; Positionen logarithmisch berechnen; Nachbarn unter dem Threshold gruppieren; Repräsentant ist der präziseste Frontpunkt in der Gruppe, sonst der schnellste; Positionen neu berechnen; Konfigurationen übernehmen.
- Die Profilzahl folgt allein aus der Streuung. Ein leeres Ergebnis bedeutet kein brauchbares Profil.

### Profilauswahl

- Der Wunsch wird auf null bis eins begrenzt (Clamp), „keine Zahl“ (NaN) ist „ungültige Precision“. Gewählt wird das Profil mit kleinstem Abstand, bei Gleichstand das präzisere.
- Die Skala gilt relativ zur Subengine: 0,5 bedeutet auf beiden Prozessoren unterschiedliche Genauigkeit.
- Solange ein aktives Profil existiert, ist der Wunsch erfüllbar. Ohne aktives Profil scheitert der erste Task mit „kein aktives Profil“ und sperrt das Modell für diese Subengine (Latch). Spätere Tasks scheitern sofort. Ungültige Wünsche sperren nicht. Die Sperre ist aufhebbar mit Rückmeldung. Alles thread-safe, ein Poisoned Lock führt nicht zum Ausfall.

### Persistenz der Kalibrierung

- Ergebnisse je Modell und Subengine mit Fingerprint, Profilen (Position und Konfiguration als Text), Instabilitäts-Flag, verworfenen und geprunten Kombinationen mit Grund.
- Fingerprint: Schema-Version, Subengine, GPU-Name, Compute Capability und Treiberversion, CPU-Features, Model-ID, Engine-Version. Bewusst knapp, ein Löschen der Datei erzwingt Neukalibrierung. Ermittlung zur Laufzeit, GPU-Daten über die Treiber-API. Fehlende Felder werden beim Einlesen mit Defaults belegt.
- Zustände: fehlend (präziseste gültige Kombination mit Warnung oder expliziter Kalibrier-Befehl), kalibriert (laden und verwenden), veraltet (altes Ergebnis nutzen, neues entsteht im Hintergrund). Das Auflösen beim Start scheitert nie, unlesbare Dateien gelten als fehlend.
- Ablage mit Load und atomarem Save: lesbare strukturierte Textdatei je Modell und Subengine, fehlende Verzeichnisse werden angelegt, Schreiben über prozessspezifische Temp-Datei mit fsync und Rename.
- Neukalibrierung läuft im Hintergrund auf dem Low-Priority-Pool, ersetzt das Ergebnis atomar und loggt den Austausch.

### Hintergrundausführung und Statistik

- Low-Priority-Pools mit Namen und konfigurierbarer Thread-Zahl (mindestens eins). Auf Linux wird die niedrigste Scheduling-Priorität gesetzt (nice), sonst geloggt.
- Jobs liefern einen Result-Container (Future), blockierend oder nicht blockierend abfragbar. Panics beenden den Worker nicht. Beim Shutdown laufen eingereichte Jobs zu Ende (Drain).
- Schätzer Median und Minimum (Last verlangsamt nur). Streuung als Variationskoeffizient oder Spannweite durch Median, ohne das langsamste Fünftel, unendlich bei zu wenigen Proben oder nicht positivem Mittelpunkt.

