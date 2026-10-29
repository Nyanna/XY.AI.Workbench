Führe alle Anforderungen strukturiert zusammen.

- Verwende keine Kürzel oder führe Code an (Code agnostisch)
- Kein Inhaltsverzeichnis, Lesehilfe, ID-Schemata, Glossare

# Anforderungen

[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/structure.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/requirements.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/resident_index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/resident_planner.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/resident_precision.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/resident_processor.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/plan0/resident_subengine.md)
Result Stats: id=a3655d3b-fcb4-43cf-8f26-b50c47b5978d, total: 144759, in: 19895, out: 124864, reason: 99324, read: 0, write: 0, start: 1793262352816, end: 1793263099476, time: 746083, total: 746083, io: 274, control: 577, sent: 1, recv: 4526
SystemInit:  id=a3655d3b-fcb4-43cf-8f26-b50c47b5978d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 99324
# Resident – zusammengeführte Anforderungen

## Zweck und Rahmen

- Resident ist eine dauerhaft laufende Engine für zwei Aufgabenarten: Embedding (Texte werden in Vektoren umgewandelt) und Similarity (Ähnlichkeitssuche gegen einen Index).
- Ein Anwendungsprogramm für abrufgestützte Generierung (englisch Retrieval-Augmented Generation) startet die Engine. Sie kann als Serverprozess, als Hintergrunddienst oder über die Kommandozeile genutzt werden.
- Gerechnet wird in zwei getrennten Subengines, eine für den Zentralprozessor und eine für den Grafikprozessor (Nvidia-Rechenplattform). Beide holen ihre Arbeit selbstorganisiert aus Warteschlangen.
- Nach der Initialisierung ist die Engine bereit, verbraucht aber noch keine Ressourcen.

### Zielumgebung

- Grafikprozessor: Nvidia GeForce GTX 1660 SUPER, Rechenfähigkeit 7.5 (Turing-Generation), Treiberversion 610.57.04.
- Compiler-Werkzeugkette der Nvidia-Rechenplattform: Release 13.3, Version 13.3.73, Build 38244171_0.
- Kernel für den Grafikprozessor werden zur Laufzeit für die tatsächlich vorhandene Architektur übersetzt, auf der Referenzkarte also für Stufe 7.5.
- Die Turing-Generation genügt für die geforderten Lesezugriffe mit Acquire-Semantik, die ab Rechenfähigkeit 7.0 verfügbar sind.

### Modelle, Formate und Abhängigkeiten

- Das Modell multilingual-e5-small liegt als Repository unter /home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small.
- Die Modelle bilden einen festen Satz. Sie werden vor der Initialisierung der Engine konfiguriert.
- Modelle und Tokenizer werden in kompatiblen Formaten geladen beziehungsweise zum Laden bereitgehalten. Die Gewichte liegen im Format safetensors vor.
- Alle Abhängigkeiten sind bereits geprüft und in der Abhängigkeitsdatei des Projekts eingetragen: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml.

## Leitprinzipien

- **Selbstorganisation:** Ein Thread des Processors bedient sich als Lane selbst an der Warteschlange. Die Zuteilung steuert sich über die dynamische Leistung der Lanes: Wer schneller ist, holt sich mehr Tasks.
- **Entkopplung:** Subengines, Kernel, Modelle und Indizes sind nicht aneinander gekoppelt. Die einzige harte Bindung zwischen Index und Modell ist die Kombination aus Modellkennung und Dimension im Indexkopf.
- **Akzeptierte Variablen statt Garantien:**
  - Ergebnisse von Embedding und Vergleich sind keine Garantien.
  - Varianz und Indeterminismus sind zugunsten von Auslastung und Leistung erlaubt, etwa durch Fusion, atomare Operationen oder hardwareabhängige Rundung.
  - Veraltete Ergebnisse sind kein Fehler. Zugunsten von Konsistenz gibt es keine Sperren.
- **Keine Qualitätszusage der Engine:**
  - Die Engine rechnet auf der ihr übergebenen Kandidatenmenge. Sie verbürgt weder Vollständigkeit noch Größe der Menge noch Ergebnisqualität.
  - Genauigkeitsverluste (Zerlegung in Chunks, Anfrage, Indexstruktur, Neubewertung, Kandidatenanzahl) sind Sache der äußeren Schichten.
  - Das Gleiche gilt für verlorene Randtreffer approximativer Vorfilter wie Zentroid, Fourier-Transformation oder Kompression.
- **Schlanker Kern:** Der Kern kennt keine Iteration, Kaskade, Rekursion, Eltern-Kind-Beziehung und keine Zwischenzustände zwischen Tasks. Solches Verhalten entsteht nur durch Addons.
- **Nichts blockiert:**
  - Die Warteschlange kennt kein Kopfblockieren (Head-of-Line-Blocking), keine Reservierung und kein Vorab-Übernehmen.
  - Der Manager blockiert nie einen Kernel.
  - Die Kalibrierung unterbricht oder blockiert den Nutzer nie.
- **Lazy statt vorab:** Ressourcen werden erst bei Bedarf initialisiert. Nach Ablauf einer Frist ohne Nutzung werden sie entladen, aber nicht deaktiviert. Im Grafikspeicher gelten deutlich längere Fristen als im Arbeitsspeicher.
- **Klare Besitzverhältnisse:** Alles, was Speicher, Streams oder Kernel besitzt, gehört der Subengine. Externe Bibliotheken sind jeweils an genau einer Stelle als Adapter angebunden.

## Komponentenstruktur

Die Wurzel gliedert sich in die Engine und gemeinsam genutzte Bausteine.

- Engine: die Fassade nach außen.
  - Processor: Orchestrierung mit Workern, Warteschlangen, Benachrichtigung und Rückdruck.
  - Planner: ordnet Operationen den Warteschlangen zu.
    - Expander: zerlegt Tasks in Operationsketten.
    - Operation: Embedding-Operationen und Similarity-Operationen.
  - Subengine: Ressourcen- und Speicherverwaltung je Hardwareart.
    - Zentralprozessor: Gerät mit Ressourcen (Kerne) und Lanes. Speicher mit Kernel, Index (Eintrag, Datei im Parquet-Format, Manager), Tokenizer und Normalizer.
    - Grafikprozessor: Gerät mit Ressourcen (Recheneinheiten) und Lanes. Speicher mit Kernel und Index (Tabelle, Segment).
    - Gemeinsam: Modell.
  - Task: Embedding-Task und Similarity-Task.
- Gemeinsam: Warteschlange.

## Typischer Anwendungsfall

1. Das Anwendungsprogramm startet, die Engine wird instanziiert und konfiguriert. Auch Indizes können konfiguriert werden, werden aber noch nicht geladen oder verwendet.
2. Die Engine wird initialisiert. Der interne Executor des Processors startet die Mindestzahl untätiger Threads.
3. Die Engine ist bereit, verbraucht aber noch keine Ressourcen.
4. Tasks für verschiedene Aufgaben werden über die Engine eingereiht.
5. Der Expander im Planner expandiert die Tasks zu Operationsketten. Der Planner stellt dabei fest, dass Zentralprozessor- und Grafikprozessor-Subengine konfiguriert sind, und startet sie.
6. Benötigt eine Operation bestimmte Ressourcen, wird das den Subengines mitgeteilt. Diese registrieren daraufhin kompatible Ressourcen (Fähigkeiten) in den ihnen zugewiesenen und in den gemeinsamen Warteschlangen.
7. Der Planner weist die expandierten Operationen anhand der kompatiblen Fähigkeiten den Warteschlangen zu, vorzugsweise der gemeinsamen.
8. Sobald eine aktivierte Lane frei ist, zieht sie eine Operation und initialisiert die Ressourcen:
   - Tokenizer und Normalisierung werden lazy initialisiert.
   - Kernel werden lazy kompiliert und geladen.
   - Indizes und Modelle werden geladen.
   - Wenn sinnvoll, werden weitere Operationen, die mit demselben Index oder Modell laufen können oder müssen, zu einem Batch aggregiert.
9. Ergebnisse werden bereitgehalten und können abgeholt werden (Ereignis oder Benachrichtigung).
10. Nach einer bestimmten Zeit werden unbenutzte Ressourcen entladen, aber nicht deaktiviert. Die Entladezeiten für Modelle und Indizes im Grafikspeicher sind wesentlich länger als im Arbeitsspeicher.
11. Normalisierung bezeichnet die zum Tokenizer gehörende. Zusätzlich kann es eine externe, vorgeschaltete Textnormalisierung für die Chunks geben.

## Engine

- Die Engine ist nach der Instanziierung konfigurierbar und wird nach Abschluss der Konfiguration initialisiert.
- Sie bietet Methoden, um die beiden Task-Arten Embedding und Similarity einzureihen.
- Sie konfiguriert den Planner (Subengines, Indizes).

## Task

- Ein Task kapselt Daten, Parameter, Ergebnis, Zustand und Operationen. Die Operationen zeigen zurück auf ihren Task.
- Das Parameterobjekt muss erweiterbar sein, damit künftig weitere Einstellungen möglich sind.
- Priorität, kompatible Ressource, Modell und Modus sind gewöhnliche Parameter (Metadaten). Die Warteschlange kennt sie nicht.
- Der Expander zerlegt jeden Task in eine Kette von Operationen.
- Zustände eines Tasks: eingereiht, übernommen, erledigt, abgebrochen.
- Das Ergebnis wird genau einmal gesetzt und ist entweder eine Ausgabe oder ein Fehler. Wartende werden über eine Benachrichtigung geweckt.

### Aufrufer

- Der Aufrufer, der einen Task abgibt, ist blockiert und wartet auf das Ergebnissignal. Zusätzlich wacht er periodisch auf und prüft den Zustand.
- Reihenfolge beim Warten: erst die Anmeldung für die Benachrichtigung aktivieren, dann den Zustand lesen, dann warten. So geht keine Benachrichtigung verloren.
- Das Timeout ist die eigene Deadline des Aufrufers. Der Zustand „abgebrochen“ wird lokal abgeleitet.

### Embedding-Task

- Eingabe: Text.
- Parameter: Zielmodell, Präzision, weitere.
- Ergebnis: Rückgabe von Eintragsreferenzen oder Kennungen des Index.
- Ein Embedding- beziehungsweise Indizierungs-Task kann so konfiguriert werden, dass er das Embedding samt Metadaten direkt in den Index schreibt.

### Similarity-Task

- Eingabe: Text.
- Parameter: Index, das daraus abgeleitete Modell, Präzision.
- Ergebnis: Eintragsreferenzen oder Kennungen (Chunk, Zeile, Bereich, Datei oder Index) mit Bewertung.
- Nach außen erscheint jeder Treffer mit Eintrag, Bewertung und einer Markierung „veraltet“.

## Planner, Expander und Operation

### Planner

- Der Planner nimmt Tasks entgegen, lässt sie vom Expander zerlegen und reiht die Operationen in passende Warteschlangen ein. Findet sich keine passende Warteschlange, scheitert die Einreihung mit einem Fehler.
- Es gibt getrennte Operations-Warteschlangen für Grafikprozessor, Zentralprozessor und gemeinsam. Die Zuordnung richtet sich nach den kompatiblen Fähigkeiten der Subengines. Bevorzugt wird die gemeinsame Warteschlange.
- Beim Erzeugen einer Operation aktiviert der Planner die benötigten Subengines (blockierend).
  - Diese aktivieren ihre Ressourcen samt Lanes, ebenfalls blockierend (nur aktivierung kein laden).
  - Die Lanes registrieren die aktivierten Ressourcen in den Warteschlangen.
  - Das Laden schwerer Ressourcen (Modelle, Indizes, Kernel) geschieht dagegen ausschließlich lazy durch die Iteration der Subengine.
- Es wird unterschieden zwischen einer verfügbaren Ressource (registrierte Fähigkeit) und einer geladenen Ressource (initialisiert und im Speicher).
- Die Engine konfiguriert den Planner mit Subengines und Indizes.

### Expander

- Der Expander nimmt einen Task (Daten und Parameter) und expandiert ihn zu einer Folge von Operationen.
- Er übersetzt die Task-Parameter in die konkrete Konfiguration der Operationen: welcher Index, welcher Tokenizer, welches Modell, welcher Scorer, welches Pooling, welches Präfix.
- Er arbeitet nur mit Konfigurationsregeln und den Fähigkeiten der Subengines.
- Er ist eine reine Funktion: gleiche Eingabe, gleiche Operationen. Er hat keine Warteschlangen, keine Worker, keine Benachrichtigung und startet keine Subengines.
- Eingaben:
  - der Task mit Daten und Parameterobjekt (Embedding oder Similarity),
  - die Pipeline-Konfiguration pro Modell: Präfixe, Pooling, Normalisierung, maximale Sequenzlänge, Datentyp.

### Operation

- Die Parameter einer Operation sind unveränderlich und dienen zum Filtern vor der Übernahme. Sie liefern die Anforderungsangabe: Ressourcenart, Modell, Index, erlaubte Subengine.
- Priorität und Task-Kennung sind Metadaten.
- Das Kennzeichen „Materialisieren“ legt fest, ob das Ergebnis beim Fertigwerden in den Arbeitsspeicher zurückgeholt wird oder als Verweis im Grafikspeicher liegen bleibt.
- Ein- und Ausgabe sind entweder ein Wert im Arbeitsspeicher oder ein Verweis auf ein Gerät.
- Eine Operation gehört zu genau einem Task. Der Graph ergibt sich implizit aus der Reihenfolge der Operationen des Tasks.
- Jede Berechnung ist eine Operation mit expliziten Ein- und Ausgabe-Regionen.
- Embedding-Operation:
  - Eingabe: Text.
  - Parameter: Zielmodell, Präzision, weitere.
  - Ergebnis: Vektordaten.
- Similarity-Operation:
  - Eingabe: Vektordaten.
  - Parameter: Index, das daraus abgeleitete Modell, Präzision.
  - Ergebnis: Bewertungen sowie Anzahl, Lücken, Streuung, Entropie und Knickpunkt.

### Expansionsregeln (Beispiele)

- Embedding: Tokenisierung (Zentralprozessor), Embedding-Berechnung (Zentralprozessor oder Grafikprozessor), Pooling mit euklidischer Normierung, optional Schreiben in den Index.
- Anfrage-Similarity: Tokenisierung, Embedding, Similarity über den Scope, Auswahl der besten Treffer.
- Similarity mit Cross-Encoder: Paare aus Anfrage und Text aus der Nutzlast, Inferenz des Cross-Encoders, Bewertungen. Der Scope stammt aus einem früheren Ergebnis.

### Regel für das Materialisieren

- Wahr, wenn der Nachfolger auf dem Zentralprozessor läuft, der Task endet oder das Ergebnis in den Index gespiegelt werden muss.
- Falsch, wenn ein Grafikprozessor-Nachfolger folgt. Das Ergebnis bleibt dann als Verweis im Grafikspeicher.
- Der Planner legt das Kennzeichen fest, die Operation führt es aus.

### Folgeoperationen und Abschluss

- Beim Abschluss einer Operation werden ihre Folgeoperationen am Ende der Warteschlange eingereiht, bevor der Zähler offener Operationen für den Vorgänger verringert wird.
- Der Task führt einen Zähler offener Operationen. Bei null ist er erledigt.
- Der erste Fehler setzt den Task auf „abgebrochen“ und das Ergebnis auf Fehler. Die übrigen Operationen scheitern bei der Übernahme.

## Processor

### Struktur

- Der Processor besitzt:
  - die Task-Warteschlange,
  - drei Operations-Warteschlangen (Zentralprozessor, Grafikprozessor, gemeinsam),
  - den Parkplatz der Worker,
  - die Benachrichtigung und die Unterversorgungshinweise.
- Er ruft den Planner für die Expansion.
- Der interne Executor startet bei der Initialisierung eine Mindestzahl untätiger Threads (Worker).
- Eine Subengine prüft zuerst ihre eigene Operations-Warteschlange, danach die gemeinsame.
- Subengines besitzen ihre Ressourcen: Tokenizer, Encoder, Scorer, Index-Segmente. Der Tokenizer ist eine Ressource der Zentralprozessor-Subengine.

### Worker

- Die Zahl der Worker für die gesamte Engine ist die Zahl der Zentralprozessor-Kerne minus eins. Das ist eine Obergrenze für Kontextwechsel, keine Kernzuordnung.
- Worker parken im Processor und wachen bei jeder Zustandsänderung per Benachrichtigung auf.
- Zentralprozessor: Der Worker führt die Operation synchron aus und kommt mit dem Ergebnis zurück.
- Grafikprozessor: Der Worker startet die Operation beziehungsweise den Kernel und kehrt zum Processor zurück. Nach der Benachrichtigung über den Abschluss verarbeitet ein freier Worker das Ergebnis.
- Blockiert ein Worker, etwa in der Rechenbibliothek oder deren Thread-Pool, ist das das natürliche Rückdrucksignal. Es gibt dafür keine Sonderbehandlung.

### Worker-Schleife

Nach jeder Operation wird in dieser Prioritätsreihenfolge neu geprüft:

1. Ergebnisse des Grafikprozessors verarbeiten und den Index zurückspiegeln.
2. Grafikprozessor-Ressourcen füttern: Index in den Grafikspeicher synchronisieren, bei freier Lane starten.
3. Zentralprozessor-Ressourcen füttern, ausführen, Ergebnis zurückgeben (auch Indizes).
4. Neue Tasks expandieren (Planner).

### Abschluss auf dem Grafikprozessor

- Der Rückruf am Ende des Streams legt nur einen Abschlusseintrag ab und löst die Benachrichtigung aus. Im Rückruf erfolgt kein Aufruf der Grafikprozessor-Schnittstelle.
- Schritt 1 der Worker-Schleife prüft zusätzlich per Ereignisabfrage und übernimmt per atomarem Austausch eine fertige Lane. Das deckt verpasste Rückrufe ab.

### Rückdruck und Expansion

- Eine Iteration ohne Treffer bei freier Ressource erzeugt einen Unterversorgungshinweis mit Ressourcenart und Parametern.
- Die Hinweise liegen als Menge pro Subengine-Art vor, nicht als Warteschlange. Sie verfallen bei einem Treffer oder wenn die Ressource belegt ist.
- Die Expansion läuft nur bei einem Hinweis oder einer leeren Operations-Warteschlange. Sie bevorzugt Tasks, deren erste Operation die unterversorgte Ressource braucht.

## Warteschlange

### Struktur

- Jede Warteschlange ist genau eine einfach verkettete Liste, ohne getrennte Listen je Priorität oder Ressource.
- Es gibt nur Anhängen und Entfernen. Anhängen geschieht per atomarem Vergleichs-und-Austausch-Schritt am Nachfolger-Verweis des letzten Knotens. Entfernen geschieht durch Entlinken, ebenfalls per atomarem Vergleichs-und-Austausch-Schritt auf dem Verweis.
- Die Reihenfolge entspricht dem Alter. Es gibt keine Version, keinen Schnappschuss und keine Sperre.
- Priorität, kompatible Ressource, Modell und Modus sind Task-Parameter. Die Warteschlange kennt sie nicht.

### Aufbau eines Knotens

- Unveränderliche Parameter, zum Filtern vor der Übernahme.
- Das gemeinsame Zustandsobjekt des Tasks, zugleich Handle des Aufrufers.
  - Es enthält den atomaren Zustand (eingereiht, übernommen, erledigt, abgebrochen).
  - Es enthält das einmal setzbare Ergebnis (Ausgabe oder Fehler).
  - Es enthält die Benachrichtigung für Wartende.
- Ein geschütztes, optionales Nutzdatenfach, das bei der Übernahme geleert wird.
- Ein atomar austauschbarer Verweis auf den Nachfolger.

### Zustandsübergänge

- **Übernahme:** atomarer Übergang von „eingereiht“ nach „übernommen“. Das ist die einzige Stelle, an der ein Abbruch relevant ist. Die Übernahme scheitert bei „abgebrochen“.
- **Fertigstellung:** atomarer Übergang von „übernommen“ nach „erledigt“, Ergebnis setzen, Wartende benachrichtigen. Bei „abgebrochen“ wird das Ergebnis verworfen.
- **Abbruch:** atomarer Übergang von „eingereiht“ nach „abgebrochen“, danach Wartende benachrichtigen. War der Task schon übernommen, hat der Abbruch keine Wirkung.

### Besitz und Speicherfreigabe

- Besitzkette: Warteschlangenkopf, Kette, Durchläufer. Jeder Durchläufer hält pro Schritt eine Referenz auf den Knoten.
- Nach erfolgreicher Übernahme übernimmt der Übernehmer die Nutzdaten. Die leere Hülle bleibt in der Kette, bis sie entlinkt wurde und der letzte Durchläufer sie verlassen hat. Dann wird sie automatisch freigegeben.
- Entlinken ist reine Aufräumarbeit: ein einfacher atomarer Austausch auf dem Verweis des Vorgängers. Der entlinkte Knoten behält seinen Nachfolger, damit ein Durchläufer ohne Neustart weiterlaufen kann.
- Die Speicherfreigabe erfolgt über Referenzzählung der Knoten. Epochenbasierte Freigabe, Slab-Verwaltung, Freiliste und Generationszähler entfallen für die Warteschlange.
- Lange Ketten müssen iterativ statt rekursiv freigegeben werden, damit kein Stapelüberlauf entsteht.
- Außerhalb der einen Komponente für den atomaren Zeigertausch darf kein Programmcode eingesetzt werden, der die Speichersicherheitsgarantien der Sprache umgeht.

## Pull-Ausführung der Subengines

### Schleife und Iteration

- Pro Subengine läuft eine Schleife mit genau einer Wartebedingung, nämlich der Benachrichtigung. Danach folgt eine Iteration über die Warteschlange.
- Die Iteration geht in Altersreihenfolge Knoten für Knoten vor:
  - Sind alle Ressourcen der Subengine ausgelastet, endet die Iteration.
  - Akzeptiert der Filter die Parameter des Knotens nicht, wird er übersprungen. Prioritätspräferenz ergibt sich über Filter und Reihenfolge, also aus Metadaten.
  - Der Ressourceneintrag wird über die Anforderungsangabe der Operation bestimmt.
  - Ist die Ressource nicht initialisiert, wird die Lazy-Initialisierung angestoßen: atomarer Übergang von „nicht initialisiert“ nach „wird geladen“. Der Task bleibt in der Warteschlange.
  - Ist die Ressource bereit, hat freie Kapazität und gelingt die Übernahme, führt die Ressource den Knoten aus.
  - In allen anderen Zuständen (wird geladen, belegt, Rückzug) wird der Knoten übersprungen.
- Am Ende jeder Iteration werden ungenutzte Ressourcen verdrängt.

### Lanes, Batches und Ressourcen

- Die Lanes einer Ressource ziehen Operationen. Die Lane des Zentralprozessors ist ein Thread des Processors, der sich selbst an der Warteschlange bedient.
- Lanes implementieren Batching: Sie greifen die nächste Operation und prüfen, ob weitere kompatible Operationen in der Warteschlange im selben Batch verarbeitet werden können.
- Der Batch beginnt mit der Ausgangsoperation. Die Ressource geht von dort vorwärts durch die Liste und übernimmt alles, was der Filter akzeptiert und was in den Batch passt. Dann startet sie den Batch.
- Packen, Längen-Bucketing und Streams sind reine Ressourcenlogik.
- Das Ergebnis löst die Benachrichtigung der Subengine aus.
- Ressourcenzustände: nicht initialisiert, wird geladen, bereit, Rückzug nach Fehler (Backoff).
- Die Zuteilung ist selbstgesteuert: Schnellere Lanes holen sich mehr Arbeit.

### Benachrichtigungstopologie

- Anhängen an eine Warteschlange: Weiterleitung an die Benachrichtigung jeder Subengine.
- Ergebnis, abgeschlossene Initialisierung und Verdrängungstimer: eigene Benachrichtigung der Subengine.
- Eine Iteration mit Fortschritt: eigene Benachrichtigung. Die Schleife ist ihre eigene Benachrichtigung.

### Regeln

- Keine Reservierung, kein Vorab-Übernehmen, kein Kopfblockieren.
- Lazy-Initialisierung geschieht nur durch die Iteration, nie durch das Einfügen.
- Verdrängung ist nur erlaubt, wenn keine Operation läuft und die Lebensdauer abgelaufen ist. Im Grafikspeicher ist sie deutlich länger als im Arbeitsspeicher.
- Initialisierungs- oder Startfehler schreiben einen Fehler in die Task-Ergebnisse des Batches und versetzen die Ressource in den Rückzug. Das wird mit Task-Kennung protokolliert.

## Subengines

### Gemeinsame Regeln

- Die Ressourcenverwaltung arbeitet lazy: erst bei Bedarf initialisieren, bei Nichtnutzung verdrängen. Im Grafikspeicher gelten längere Fristen als im Arbeitsspeicher.
- Jede Subengine liefert eine Fähigkeitenmenge und registriert kompatible Ressourcen samt Lanes in ihren eigenen und in den gemeinsamen Warteschlangen.
- Jede Subengine kalibriert ihre Hardware per Benchmark und stellt daraus Präzisionsprofile bereit.
- Jede Subengine implementiert ihre Rechenpfade hardwareoptimal und entscheidet selbst.

### Invarianten

- In öffentlichen Signaturen gibt es keinen Tensor und keinen Rohzeiger, nur Region und Layout.
- Jede Grafikprozessor-Arbeit läuft über einen Ausführungskontext, nie auf dem Standard-Stream.
- Kernel werden nur über das Kernel-Register geladen. Kernel der externen Bibliothek sind dort nur eine weitere Implementierung.
- Planung und Zuteilung kennen weder die externe Bibliothek für neuronale Netze noch Ausführungsgraphen des Grafikprozessors.

### Zentralprozessor-Subengine

- **Gerät:** Der Zentralprozessor(potenziell mehrere) ist das Gerät. Seine Ressourcen sind die Kerne, jede mit Lanes.
- **Speicherverwaltung (Arbeitsspeicher):**
  - Sie lädt Modelle und Indizes in den Arbeitsspeicher beziehungsweise streamt Daten hinein.
  - Sie kennt den verfügbaren Arbeitsspeicher und lagert Modelle, Kernel und Ressourcen nach Bedarf aus und ein.
- **Kernel:** Zentralprozessor-Implementierungen mit eigenem Register.
- **Index:** Er wird beim ersten Zugriff geladen (siehe Abschnitt Index). Dazu gehören Eintrag, Datei und Manager.
- **Tokenizer und Normalizer:** Sie sind Zentralprozessor-Ressourcen (siehe gemeinsame Ressourcen).
- **Besonderheiten:**
  - Mehr Speicher steht zur Verfügung, alles nutzt den Zentralprozessor.
  - Der Scorer ist selbst implementiert und nutzt Vektorbefehle.
  - Der Encoder läuft über das Zentralprozessor-Backend der Bibliothek.
  - Der Index ist per Speicherabbildung lesbar.
  - Es gibt keine Streams und keinen Zwischenspeicher für Kopien.
  - Standarddatentyp ist 32-Bit-Gleitkomma.

### Grafikprozessor-Subengine

- **Plattform:** Nvidia-Rechenplattform. Gerät ist der Grafikprozessor (potenziell mehrere), mit Recheneinheiten als Ressourcen und Lanes darauf.
- **Grafikspeicher-Manager:**
  - Er lädt oder streamt Modelle und Indizes in den Grafikspeicher.
  - Er kennt den verfügbaren Grafikspeicher und tauscht Modelle, Kernel und Ressourcen nach Bedarf aus und ein.
  - Er ist die Brücke zwischen dem Index im Arbeitsspeicher und der im Grafikspeicher abgebildeten Darstellung für die Kernel.
  - Er lädt außerdem Kernel sowie Ein- und Ausgabe-Arenen und Speicherbereiche.
  - Dadurch wird der Index effektiv zwischen Arbeitsspeicher und Grafikspeicher synchronisiert.
- **Kernel:** mit Register. Die Adressabbildung im Grafikspeicher erfolgt über Slots.
- **Index:** Segmenttabelle und Segmente (siehe Abschnitt Index).
- **Besonderheiten:**
  - Der Grafikspeicher ist begrenzt.
  - Es gibt Streams, Zwischenspeicher und Transfers.
  - Batching ist notwendig, damit Index-Segmente und Kernel effizient gestreamt und getauscht werden.
  - Standarddatentyp ist 16-Bit-Gleitkomma.

### Gemeinsame Ressourcen (zentralprozessorseitig, von beiden Subengines genutzt)

- **Modell:**
  - Die Modellbeschreibung stammt aus der Konfiguration: Architekturkennung, Mehrvektor-Konfiguration, Pfade. Sie beschreibt eine Pipeline und besitzt nichts.
  - Ein Register ordnet Architekturkennungen Encoder-Implementierungen zu (etwa dem BERT-Encoder) und Bewertungsarten Scorer-Implementierungen (etwa der Maximalähnlichkeit). Neue Modelle bedeuten einen neuen Registereintrag.
  - Mehrvektor-Parameter stammen aus der Konfiguration des Prüfpunkts, nicht aus festem Programmcode: maximale Anfragelänge, maximale Dokumentlänge, Markierungs-Token, Dimension, Ausschlussliste.
- **Normalizer:**
  - Er gehört zum Tokenizer: Unicode-Normalisierung, Kleinschreibung und weitere Schritte.
  - Er läuft nur auf dem Zentralprozessor und ist über Parameter konfiguriert.
- **Tokenizer:**
  - Er läuft nur auf dem Zentralprozessor und ist modellspezifisch oder geteilt.
  - Er unterstützt die Kodierungsarten Anfrage und Passage sowie die Modi Präfix und Mehrvektor.
  - Weitere Einstellungen sind die maximale Länge und die Maske.
  - Anfrage-Modus im Mehrvektor-Modus: Markierung plus Auffüllung mit Maskentoken auf feste Länge.
  - Dokument-Modus im Mehrvektor-Modus: Markierung, Kürzen auf die maximale Dokumentlänge, Auffüll-Tokens werden verworfen.
  - Ausgabe sind Token-Kennungen als 32-Bit-Ganzzahlen. Mehr braucht der Grafikprozessor nicht, also kein Zusammensammeln auf dem Zentralprozessor und keine Adressabbildung.
- **Gewichtequelle:** Das Format ist safetensors, speicherabgebildet und kopierfrei auf Rohbytes. Die Gewichte werden pro Subengine in deren Arena kopiert.

## Geräte, Lanes und Speicher

### Geräte und Ausführungskontexte

- Das Gerätehandle besteht aus dem Geräteordinal beziehungsweise dem Zentralprozessor.
- Beim Grafikprozessor gibt es einen Prozess, einen primären Kontext und mehrere Streams.
- Ein Ausführungskontext (Lane) besteht aus Stream, Priorität, Ereignissen, eigenem Parameterblock und Arbeitsbereich.
- Beispiel-Lanes:
  - Anfrage: hohe Priorität, kleine Batches.
  - Einfügen und Aktualisieren im Index: Durchsatz.
  - Kopieren zwischen Arbeitsspeicher und Grafikspeicher in beide Richtungen.
- Übergaben zwischen Lanes laufen ausschließlich über die Warteschlange.

### Speicherverwaltung

- Die Speicherverwaltung reserviert Speicher (Gerät, Bytes, Ausrichtung) als Region und gibt ihn frei. Das Auflösen einer Region ist nur intern erlaubt.
- Eine Region ist ein Handle aus Arena, Offset, Länge und Generation, kein Zeiger. Die Generation entwertet Handles nach Verdrängen und Neuladen.
- Ein Layout besteht aus Form, Schrittweiten und Datentyp, mit eigener Datentyp-Aufzählung.
- Arenen sind nach Lebensdauer getrennt:
  - Gewichte: langlebig, nur beim Entladen frei.
  - Index-Segmente: gestreamt, verdrängbar.
  - Arbeitsbereich pro Lane: kurzlebig.
  - Seitenfester Zwischenspeicher im Arbeitsspeicher für Kopien in beide Richtungen. Beim Zentralprozessor entfällt er.
  - Parameterblock pro Lane: kleine, feste Region für dynamische Kernel-Parameter.
- Das Grafikspeicher-Budget wird zur Laufzeit abgefragt.
- Die Arena wird lazy beim ersten Laden nach Budget angelegt. Die Vergabe läuft über eine Freiliste. Alle Adressen sind 64-Bit-Offsets innerhalb der Arena.
- Das Budget plant den kurzen Doppelbedarf durch Copy-on-Write ein.
- Reservieren und Freigeben von Grafikspeicher im laufenden Betrieb ist zu vermeiden, weil es das Gerät implizit synchronisiert.
- Stream-geordnete Wiederverwendung: Eine Region wird erst frei oder überschrieben, wenn alle Streams, die sie nutzen, ihr Ereignis erreicht haben.
- Datentypen gelten pro Gerät. Der Datentyp eines Index ist pro Index fest.
- Kopien laufen über einen eigenen Kopier-Stream mit seitenfestem Zwischenspeicher, getrennt vom Rechen-Stream.
- Der Laufzustand wird über ein Ereignis pro Batch erfasst. Im persistenten Modus dient ein Fortschrittszähler in abgebildetem Arbeitsspeicher dazu.

## Index, Manager und Synchronisation

### Index

- Ein Index kann eine Struktur kapseln, eine speicherabgebildete Datei sein oder intern bereits Bytepuffer oder Spaltenformate verwenden.
- Jeder Index implementiert sein eigenes Modell, etwa ganze Dateien oder Gleitfenster, Syntaxbaum-Knoten, Chunks, kaskadierende oder fraktale Strukturen. Der Kern kennt diese Modelle nicht.
- Der Index ist ein eigenes Objekt. Seine Darstellung im Arbeitsspeicher und seine persistente Darstellung sind dieselbe Instanz. Der Grafikspeicher hält nur einen abgeleiteten Spiegel. Die Richtungen zwischen Arbeitsspeicher, Index und Grafikspeicher sind agnostisch.
- Der Index kennt weder Subengine noch Profil noch Kernel und bleibt unabhängig nutzbar. Er meldet zunächst nur Änderungen und liefert Struktur und Änderungsstempel, sonst nichts.
- Die einzige Kompatibilitätsbedingung zu einem Modell sind Modellkennung und Dimension im Indexkopf. Vektoren verschiedener Modelle sind nicht vergleichbar. Ein Metadatum „erzeugendes Profil“ ist höchstens optionale Diagnose.
- Ein Index wird beim ersten Zugriff lazy geladen.
- Ein Eintrag besteht aus Kennung, Änderungsstempel und Nutzlast. Die Dateinutzlast besteht aus Dateiname sowie Start- und Längen-Offset.
- Dateien liegen im Parquet-Format mit Arrow-Schema vor.
- Die Zugriffsmethoden Zentroid, Präfix, Teilbaum und Bereich sind auszuarbeiten.
- Approximative Vorfilter können Randtreffer verlieren. Das ist eine Eigenschaft der Oberschicht, nicht der Engine.

### Drei Schichten

- **Index:** kennt Einträge, deren Vektordaten und seine eigene Traversalstruktur.
- **Manager:**
  - Er kennt die Zuordnung von Eintrag zu Segment und Slot, die Residenz und die Update-Warteschlange.
  - Er ist verantwortlich für Hochladen, Verdrängen, Aktualisieren und die Rückabbildung von Adressen auf Einträge.
- **Kernel:** kennt nur Segmente, Adressen und Offsets, keine Eintragsidentität und keine Traversierung.
- Der Manager ist eine Komponente der jeweiligen Subengine, weder der Fassade noch des Index. Er wird mit einer Subengine und einem lazy aktivierten Index konstruiert.
- Der Manager der Zentralprozessor-Subengine liest den Index direkt, denn der Arbeitsspeicher ist die Primärinstanz. Er hat fast nichts zu tun. Ausnahmen sind die Residenz bei speicherabgebildeten Indizes, die größer als der Arbeitsspeicher sind, und das Entladen nach Leerlaufzeit. Dafür gibt es Vorabladen mit Vorabladehinweis an das Betriebssystem.

### Scope und Auflösung

- Der Manager löst den Scope einer Operation für kompatible Indextypen auf, nicht der Index.
- Kompatibel heißt: Der Index deklariert das Verfahren, und der Manager hat einen Resolver dafür. Sonst entsteht der Fehler „Scope nicht unterstützt“. Ein stilles Ausweichen auf alle Einträge gibt es nicht.
- Beispiel Zentroid:
  - Der Manager holt die Zusammenfassungsstruktur vom Index und macht sie resident.
  - Subengine und Kernel rechnen die Anfrage gegen die Zentroide.
  - Der Manager fragt den Index nach den Mitgliedern der gewählten Zellen und gleicht gegen den Ladezustand ab: Fehlt etwas oder ist etwas veraltet?
- Ein Scope kann Teilbäume oder Bereiche einschränken. Ist die Kandidatenmenge schon aus der Vorverarbeitung bekannt (nach Art des mehrstufigen Zentroid-Verfahrens PLAID, Beschneiden, Präfix, Fourier- oder Kompressionsdomäne), lädt der Manager nur diese Bereiche.
- Der Scope wird als kompakte Eintragsmenge dargestellt, als Bereiche oder Bitmap, nicht als Kennungsliste.

### Kernel-Schnittstelle und Layouts

- Der Kernel liefert nur Adresse beziehungsweise Offset und Bewertung. Ob Segmentkennung plus Offset, roher Zeiger oder Arena-Offset, ist Implementierungsdetail. Während der Berechnung gibt es keine Kommunikation.
- Layout fest (mit Schrittweite): Eintragsgrenzen ergeben sich per Arithmetik.
- Layout variabel: eine Offset-Tabelle aus Präfixsummen, vom Manager aus der Indexstruktur gebaut.
- Der Kernel traversiert nie. Die Maximalähnlichkeit aggregiert pro Eintrag. Dafür braucht der Kernel Schrittweite oder Tabelle, nicht die Eintragsidentität.
- Eine Kandidatenmenge ist ein variables Layout. Der Kernel rechnet darüber, egal ob die Tabelle auf kompakte Kopien oder auf bestehende Segmente zeigt.
- Die Rückabbildung von Adresse auf Eintrag macht der Manager: bei festem Layout per Arithmetik, bei variablem per Binärsuche in seiner Tabelle oder über eine Intervallabbildung auf Arena-Offsets.

### Segmente und Slots im Grafikspeicher

- Eine Segmentsicht besteht aus Basisadresse (als Region), Slot-Anzahl, Schrittweite, Datentyp und Generation.
- Ein Slot hat feste Größe: maximale Tokenzahl mal Dimension mal Datentypgröße. Bei variablen Längen gibt es pro Segment Slot-Klassen (etwa 32, 64, 128 Tokens), damit der Kernel ohne Nachschlagen auskommt.
- Pro Slot gibt es die gültige Tokenzahl und ein Gültigkeitsbit (Löschmarkierungen, noch nicht befüllt). Die Auffüllung wird mit negativ unendlich maskiert, nicht mit null.
- Die Basisadressen der Segmente gehen als kleine Tabelle über Segmente, nicht über Einträge, an den Kernel.
- Schreiben: Der Manager vergibt den Slot vorab, der Encoder schreibt direkt dorthin, das Gültigkeitsbit wird erst nach dem Ereignis gesetzt.
- Einfügen oder Aktualisieren heißt: neuer Slot, danach wird das Bit des alten Slots gelöscht.

### Manager-Disziplin (nie den Kernel blockieren)

- Der Manager kennt den Laufzustand des Kernels und verändert nur in Lücken (Batch-Ende beziehungsweise Kommunikationsslot). Er ändert nichts unter dem Kernel und blockiert ihn nicht.
- Atomar getauscht wird nur ein Zeiger von acht Byte Breite.
  - Größere Strukturen werden nie an Ort und Stelle geändert, sondern per Copy-on-Write neu angelegt. Danach wird der Zeiger getauscht.
  - Hierarchie: Wurzelzeiger, Segmenttabelle, Segment (Basis, Layout, Offset-Zeiger).
  - Das gilt für Nachschlagetabellen und für ganze Segmente. Ein geändertes Segment wird komplett neu angelegt und der Zeiger getauscht, sobald sicher ist, dass die Subengine dort nicht mehr liest.
- Aktualisieren an Ort und Stelle ist in einer Lücke trivial. Im persistenten Modus kann der Kernel einen halb aktualisierten Vektor lesen (gemischte, aber gültige Werte). Das wird bewusst akzeptiert und dokumentiert.
- Lesezeitpunkt des Zeigers:
  - Bei normalen Starts wird der Zeiger als Kernel-Argument übergeben. Ein Tausch wirkt ab dem nächsten Start.
  - Im persistenten Ringpuffer-Modus liest der Kernel den Zeiger einmal pro Arbeitseinheit mit Acquire-Semantik. Der Inhalt der neuen Tabelle muss vorher sichtbar sein, durch Ereignis oder Speicherbarriere.
- Blockierende Lesezugriffe auf den Index (Seitenfehler, Ein- und Ausgabe) laufen in einem Pool für blockierende Arbeit, nicht im Thread der asynchronen Laufzeitumgebung. Ein Vorabladehinweis sorgt dafür, dass Ein- und Ausgabe und Kernel-Lauf überlappen.

### Kommunikationsslot

Die Reihenfolge ist fest:

1. Lesen, was der Kernel geschrieben hat: gemeldete Bereiche den Einträgen zuordnen, die Übernahme am Index aufrufen, eigene Stempel eintragen.
2. Ergebnisse auflösen: Adresse auf Eintrag abbilden.
3. Updates einspielen: die Update-Warteschlange anwenden. Das verändert die Abbildung und kommt daher zuletzt.

- Weil nur in Lücken verändert wird, sieht jedes Ergebnis den Abbildungsstand seines Batches. Der Manager repräsentiert den Grafikspeicher, daher ist eine vom Kernel gemeldete Adresse aktuell. Im Kernel-Protokoll gibt es keine Epoche und keine Sequenznummer.
- Ergebnisstatus beim Auflösen:
  - Ist die Kennung im Manager vorhanden, aber im Index entfernt, löst sie nicht mehr auf und das Ergebnis wird verworfen.
  - Liegt für den Eintrag ein Update in der Warteschlange, wird das Ergebnis als „veraltet“ markiert. Das dient der Diagnose und ist kein Fehler.
  - Sonst ist es ein normales Ergebnis.
- Änderungen bleiben in der Update-Warteschlange, bis sie ausgeführt werden.

### Indexaufbau durch Kernel

- Der Manager vergibt vorab einen beschreibbaren Bereich aus der Arena. Der Kernel schreibt per Cursor hinein und allokiert nie.
- Am Batch-Ende meldet der Kernel die belegten Bereiche. Reicht der Bereich nicht, liefert er ein Teilergebnis und der Manager vergibt den nächsten Bereich.
- Im nächsten Kommunikationsslot ordnet der Manager die Bereiche den Einträgen zu und ruft die Übernahme am Index auf.
- Embedding-Ringpuffer liefern neue Segmente, Ringpuffer der Maximalähnlichkeit liefern Treffer. Indexieren während der Suche funktioniert, weil Schritt 1 vor Schritt 2 liegt.

### Synchronisation und Echo-Vermeidung

- Der Abgleich ist zustandsbasiert statt ereignisbasiert: Das Signal weckt nur, der Manager vergleicht Stempel.
- Es werden nur residente Einträge verglichen. Änderungen an nicht geladenen Einträgen brauchen keine Behandlung, weil das Laden ohnehin den aktuellen Stand liest. Der Zustand des Managers skaliert mit dem geladenen Teil, nicht mit der Indexgröße.
- Das Signal wird vor dem Scan quittiert, nicht danach. Sonst geht eine Änderung verloren, die während des Scans eintrifft.
- Lesereihenfolge beim Laden:
  - Zuerst den Stempel lesen, dann die Daten, dann bei Speicherabbildung den Stempel erneut prüfen. Weicht er ab, wird wiederholt.
  - Die falsche Reihenfolge (erst Daten, dann Stempel) trüge einen neueren Stempel als bekannt ein, obwohl veraltete Daten geladen wurden. Das ist die einzige unsichere Richtung. Ein überzähliger Reload ist harmlos.
- Echo-Unterdrückung: Der Manager trägt den von der Übernahme zurückgegebenen Stempel seines eigenen Schreibzugriffs als bekannt ein (Gleichheitsvergleich).
  - Der Stempel muss aus dem Rückgabewert stammen, nicht aus einem späteren Lesen.
  - Ohne Rückgabewert entsteht höchstens ein redundanter Upload, also die sichere Richtung.
- Zwei Subengines, derselbe Eintrag: Der andere Manager sieht einen abweichenden Stempel und lädt nach. Beide konvergieren auf den Indexstand.
- Entfernen und Neuanlage: Der Manager vergleicht seine Abbildung mit der Gesamtliste der Stempel beziehungsweise mit der Stempelabfrage pro Eintrag, die für entfernte Einträge nichts liefert.
- Mehrere Abonnenten der Indexänderungen (Grafikprozessor-Manager, Persistenz-Zeitgeber, Zentralprozessor-Manager) haben jeweils eigenen Zustand.

### Residenz und Ladestrategie

- Geladen wird lazy und scope-getrieben: nur das, was ein Task braucht.
- Ladegranularität:
  - Dichte Scopes laden ganze Segmente.
  - Dünne Kandidatenmengen werden in einen temporären, kompakten Segmentpuffer im seitenfesten Zwischenspeicher gesammelt und in einem einzigen Kopiervorgang übertragen.
  - Sehr kleine Mengen können kopierfrei aus abgebildetem Arbeitsspeicher gelesen werden.
  - Die Dichteschwelle zwischen Sammeln und Segmentladen liegt in der Implementierung des Managers.
- Gestaffelte Haltung:
  - Zusammenfassungsstrukturen (Zentroide, Präfixtabellen) sind klein, werden zuerst geladen und am längsten gehalten.
  - Vektorsegmente sind warm und per Verdrängung ersetzbar.
  - Alles andere ist nicht geladen und kostet nichts.
- Aggregation: Tasks gegen denselben Index werden über die Vereinigung ihrer Scopes gebündelt und einmal geladen.
- Single-Flight: Gleichzeitige Ladeanforderungen auf denselben Bereich laufen als ein einziger Ladevorgang mit mehreren Wartenden, analog zur Lazy-Initialisierung von Modellen.
- Die Segmentgröße bestimmt der Manager nach Grafikspeicher-Budget. Sie ist keine Eigenschaft des Index.
- Überlappende Einträge: Der Manager entscheidet, ob Vektoren einmal gehalten und von mehreren Slots referenziert werden (spart Grafikspeicher, komplexer) oder dupliziert werden (einfach, teuer). Das ist eine Design-Entscheidung des Managers, kein Vertrag.
- Entladen: Die Leerlaufzeiten im Grafikspeicher sind wesentlich länger als im Arbeitsspeicher.

## Encoder, Kernel und Bewertung

### Encoder

- Ein geladener Encoder besteht aus Architektur (Code) und Gewichten (Daten) auf einem Gerät. Er ist ein Objekt und entladbar.
- Phase 1 nutzt vorhandene Encoder-Rümpfe der externen Bibliothek (BERT, ModernBERT und weitere) über den Adapter. Für Mehrvektor-Modelle nach Art von ColBERT existiert dort kein Modul, daher wird ein Rumpf mit eigenem Projektionskopf verwendet.
- Der Projektionskopf ist eine lineare Schicht (zum Beispiel von 768 auf 128 Dimensionen). Die Gewichte stammen aus dem Prüfpunkt, der Ort dieses Moduls ist je Prüfpunkt noch zu prüfen.
- Die euklidische Normierung ist ein eigener Kernel am Ende des Encoders. Sie wird einmal beim Schreiben ausgeführt, nicht im Scorer. Anfrage und Dokument werden gleich behandelt.
- Ausgabe: Der Encoder schreibt aus einer Token-Region in eine Ausgabe-Region.
  - In Phase 1 rechnet die Bibliothek. Ein dünner Schreib-Kernel liest den Geräte-Zeiger des Ergebnis-Tensors und schreibt normiert und datentypkonvertiert in den Slot.
  - Der Tensor bleibt bis zum Ereignis am Ende des Schreibens am Leben.
- Batching: Token-Budget statt Task-Anzahl, Längen-Bucketing, Auffüllung auf gemeinsame Sequenzlänge.
- Batchgröße: Token-Budget, Längen-Bucket und Deadline pro Klasse. Die Obergrenze der Batchdauer begrenzt die Anfragelatenz.
- Das Ergebnis läuft über einen einmaligen Ergebniskanal pro Task plus Ereignis oder Benachrichtigung. Der Batch merkt sich die Zeilen-Offsets pro Task.

### Kernel

- Das Kernel-Register liefert zu Name, Datentyp und Architekturstufe die geladene Funktion. Quellen:
  - Laufzeitkompilierung mit Zielarchitektur,
  - vorkompilierter Binär-Fallback bei zu altem Treiber,
  - Kernel der externen Bibliothek als weitere Implementierung.
- Zur Übersetzungszeit festgelegt sind Datentyp, Dimension und Kachelgröße, als Varianten in eigenen Funktionen.
- Dynamisch über den Parameterblock kommen Slot-Indizes, Zähler, Segmenttabelle, Anfragelänge sowie Kopf und Ende.

### Maximalähnlichkeit und Auswahl

- Die Operation bekommt einen Slot-Bereich (Segment, Slot von, Slot bis) und einen Anfrage-Batch. Sie liefert Bewertungen je Slot.
- Phase 1 verwendet eine Kachelkette aus Bibliotheksoperationen: Matrixmultiplikation, Maske, Maximum, laufendes Maximum über Token-Kacheln, Summe über Anfrage-Token. Die Zwischenmatrix ist durch die Kachelgröße begrenzt.
- Später folgt ein fusionierter Kernel, bei dem die Zwischenmatrix nie im Grafikspeicher liegt. Der Austausch geschieht hinter derselben Operation.
- Zentralprozessor: eine Schleife pro Slot mit laufendem Maximum je Anfrage-Token und Vektorbefehlen, ohne Matrix über Slots.
- Die Dokumentseite ist maskiert. Die Maskentoken der Anfrage zählen mit.
- Die Auswahl der besten Treffer wird pro Abschnitt lokal reduziert, die Zusammenführung erfolgt auf dem Zentralprozessor.

### Arbeitsabschnitte und Anfrage-Batching

- Ein Arbeitsabschnitt ist die Planungseinheit mit Zeitbudget als Parameter. Seine Größe wird adaptiv aus dem gemessenen Durchsatz bestimmt.
- Unterbrechung ist nur an Abschnittsgrenzen möglich. Anfrage-Tasks können Indexier-Abschnitte überholen.
- Eine Kachel wird gegen alle wartenden Anfragen bewertet, um die Lesekosten zu amortisieren.

## Präzision, Kalibrierung und Profile

### Prinzipien

- Zwischen Zentralprozessor und Grafikprozessor gibt es keinen Rundungspunkt- oder Datentypvertrag.
- Der Präzisionsparameter eines Tasks ist ein Wunsch, keine Zusicherung (0-1f; 0 am schnellsten, 1 maximale Präzision).
- Die Kalibrierung unterbricht oder blockiert den Nutzer nie.

### Fähigkeitenmenge

- Jede Subengine liefert eine Fähigkeitenmenge: pro Parameterachse die verfügbaren Werte.
- Grafikprozessor, Beispiele:
  - Operanden: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma.
  - Akkumulator: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma.
  - Bewertung: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma, 8-Bit-Ganzzahl mit Vier-Wert-Skalarprodukt.
  - Fusion: an oder aus.
  - Batchgröße und Kachelgröße.
- Zentralprozessor, Beispiele:
  - Operanden: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma mit Umrechnung auf 32 Bit.
  - Bewertung: 32-Bit-Gleitkomma, 8-Bit-Ganzzahl.
  - Threadzahl und Kachelgröße.
- Es gibt keine vordefinierten Profilnamen und keine fest einkodierte Profilaufzählung. Achsen und Werte sind pro Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle.

### Profilbildung

- Profile werden allein aus der Streuung der Pareto-Front abgeleitet. Ihre Anzahl ist variabel (eins bis n).
- Liegen Punkte nicht ausreichend auseinander, werden sie zusammengefasst. Es werden keine künstlichen Profile erzeugt.
- Der Streuungsschwellwert ist die einzige Konfiguration, die die Profilanzahl bestimmt. Er ist auf der normierten Skala definiert und damit modell- und hardwareunabhängig.
- Redundante Profile werden nicht aktiviert, etwa bei praktisch identischer Zeit und identischem Ergebnis, zum Beispiel wegen eines gleichen Emulationspfads.
- Profile sind unbenannte Indizes mit einer Position zwischen null und eins:
  - Koordinaten werden normiert: Zeit (zum Beispiel logarithmisch) und Abweichung jeweils auf null bis eins.
  - Die Bogenlänge entlang der Front wird berechnet.
  - Das schnellste Profil liegt bei null, das genaueste bei eins. Die übrigen liegen proportional zur kumulierten Distanz, nicht nach reiner Rangnummer, weil diese Abstände verzerrt.
  - Bei genau einem Profil gilt die Position 0,5 per Konvention.
- Die Profilordnung ergibt sich aus der Messung. Auch der umgekehrte Fall wird erkannt: Reduzierte Genauigkeit, die durch Emulation langsamer ist, wird herabgestuft oder nicht aktiviert.
- Eine Subengine ohne aktives Profil für ein Modell ist für dieses Modell gesperrt.

### Wunsch und Abgleich

- Der Präzisionsparameter des Tasks ist eine Gleitkommazahl. Skala: null bedeutet schnell, eins bedeutet präzise.
- Werte außerhalb von null bis eins werden auf diesen Bereich geklemmt. Ein Wert „keine Zahl“ ist ein Fehler.
- Abgleich: Es gewinnt das Profil mit dem kleinsten Abstand zwischen Wunschwert und Profilposition. Bei Gleichstand gewinnt deterministisch das genauere Profil.
- Jede Subengine gleicht gegen ihre eigenen Profile ab. Die Skala ist relativ zur Subengine: Ein Wunsch von 0,5 bedeutet auf Zentralprozessor und Grafikprozessor nicht dasselbe Genauigkeitsniveau.
- Ein Wunsch ist immer erfüllbar, solange ein aktives Profil existiert. Es gibt keinen Fehler und kein Warten wegen eines nicht verfügbaren Profils.

### Persistenz der Kalibrierung

- Die Kalibrierung läuft initial einmal und wird auf derselben Maschine in einer Datei persistiert.
- Sie gilt pro Kombination aus Modell und Subengine, weil das Verhalten von Dimension und Sequenzlänge abhängt.
- Fingerabdruck als Schlüssel: Grafikprozessor-Modell, Treiber- und Plattformversion, Zentralprozessor-Modell, Modellkennung, Engine-Version.
- Zustände:
  - **Kalibriert:** laden und verwenden.
  - **Veraltet** (Fingerabdruck weicht ab): mit dem alten Ergebnis starten.
  - **Fehlend:** konservativer Standard (genaueste gültige Kombination, mit Warnung) oder ein ausdrücklicher Kalibrierbefehl.
- Zusätzlich persistiert werden Instabil-Markierungen, Verwerfungen wegen Überschreitung der Maximalabweichung und Ergebnisse des Aussortierens (Pruning), für Diagnose und Nachmessung.

## Addons (Iteration, Kaskaden, Rekursion)

- Ein Task kann den Scope eines Folgetasks treiben. Iteration und Rekursion entstehen allein durch ein Addon in einer Subengine, in der Anwendung oder in einer konkreten Ressourcenimplementierung (Index, Modell).
- Ein Addon reiht Tasks über den normalen Weg ein, mit kopierten oder geänderten Parametern. Die Subengine selbst oder eine andere Subengine zieht sie, gesteuert über die vorhandenen Pull-Filter (Priorität, Affinität).
- Ein von einem Addon erzeugte Task ist von einem Anwendungs-Task nicht unterscheidbar.
- Der Kern benötigt dafür nur das erweiterbare Parameterobjekt.
- Beispiel, nur zur Illustration: ein Zentroid-Task gegen die Zusammenfassungsstruktur, danach ein Folgetask mit den Mitgliedern der gewählten Zellen als Scope.
 