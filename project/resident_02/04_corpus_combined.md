# Resident – Anforderungen

## Zweck und Rahmen

- Resident ist eine dauerhaft laufende Engine für zwei Aufgabenarten:
  - Embedding: Texte werden in Vektoren umgewandelt.
  - Similarity: Ähnlichkeitssuche gegen einen Index.
- Ein Anwendungsprogramm für abrufgestützte Generierung (Retrieval-Augmented Generation) startet die Engine. Sie ist als Serverprozess, als Hintergrunddienst oder über die Kommandozeile nutzbar.
- Gerechnet wird in zwei getrennten Subengines, eine für den Zentralprozessor und eine für den Grafikprozessor (Nvidia-Rechenplattform). Beide holen ihre Arbeit selbstorganisiert aus Warteschlangen (Pull-Prinzip).
- Nach der Initialisierung ist die Engine bereit, verbraucht aber noch keine Ressourcen.

### Zielumgebung

- Grafikprozessor: Nvidia GeForce GTX 1660 SUPER, Rechenfähigkeit 7.5 (Turing-Generation), Treiberversion 610.57.04.
- Compiler-Werkzeugkette der Nvidia-Rechenplattform: Release 13.3, Version 13.3.73, Build 38244171_0.
- Kernel für den Grafikprozessor werden zur Laufzeit für die tatsächlich vorhandene Architektur übersetzt, auf der Referenzkarte also für Stufe 7.5.
- Die Turing-Generation genügt für die geforderten Lesezugriffe mit Acquire-Semantik, die ab Rechenfähigkeit 7.0 verfügbar sind.
- Die Referenzkarte gehört zur Baureihe ohne Tensorkerne. Ein Tensorkern-Pfad und das 16-Bit-Brain-Gleitkommaformat als Rechenpfad werden daher nicht angeboten.

### Modelle, Formate und Abhängigkeiten

- Das Modell multilingual-e5-small liegt als Repository unter /home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small.
- Die Modelle bilden einen festen Satz und werden vor der Initialisierung der Engine konfiguriert.
- Modelle und Tokenizer werden in kompatiblen Formaten geladen beziehungsweise zum Laden bereitgehalten. Die Gewichte liegen im Format safetensors vor.
- Persistente Indizes liegen im Spaltenformat Parquet mit Arrow-Schema vor.
- Alle Abhängigkeiten sind bereits geprüft und in der Abhängigkeitsdatei des Projekts eingetragen: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml. Die externe Tensorbibliothek für neuronale Netze ist Candle.

## Leitprinzipien

- **Selbstorganisation:** Ein Thread des Processors bedient sich als Lane selbst an der Warteschlange. Die Zuteilung steuert sich über die dynamische Leistung der Lanes: Wer schneller ist, holt sich mehr Arbeit.
- **Entkopplung:** Subengines, Kernel, Modelle und Indizes sind nicht aneinander gekoppelt. Die einzige harte Bindung zwischen Index und Modell ist die Kombination aus Modellkennung und Dimension im Indexkopf, ergänzt um die Pipeline-Merkmale, die Vektoren vergleichbar machen.
- **Akzeptierte Variablen statt Garantien:**
  - Ergebnisse von Embedding und Vergleich sind keine Garantien.
  - Varianz und Indeterminismus sind zugunsten von Auslastung und Leistung erlaubt, etwa durch Fusion, atomare Operationen oder hardwareabhängige Rundung.
  - Veraltete Ergebnisse sind kein Fehler. Zugunsten der Durchsatzkonsistenz gibt es keine Sperren auf den Datenpfaden.
- **Keine Qualitätszusage der Engine:**
  - Die Engine rechnet auf der ihr übergebenen Kandidatenmenge. Sie verbürgt weder Vollständigkeit noch Größe der Menge noch Ergebnisqualität.
  - Genauigkeitsverluste durch Zerlegung in Chunks, Anfrageformulierung, Indexstruktur, Neubewertung oder Kandidatenanzahl sind Sache der äußeren Schichten.
  - Das Gleiche gilt für verlorene Randtreffer approximativer Vorfilter wie Zentroid, Fourier-Transformation oder Kompression.
  - Der Ähnlichkeitsbericht liefert Fakten, die Engine trifft keine Entscheidungen und legt keine Schwellenwerte fest.
- **Schlanker Kern:** Der Kern kennt keine Iteration, Kaskade, Rekursion, Eltern-Kind-Beziehung und keine Zwischenzustände zwischen Tasks. Solches Verhalten entsteht nur durch Addons.
- **Nichts blockiert:**
  - Die Warteschlange kennt kein Kopfblockieren (Head-of-Line-Blocking), keine Reservierung und kein Vorab-Übernehmen.
  - Der Manager blockiert nie einen Kernel.
  - Die Kalibrierung unterbricht oder blockiert den Nutzer nie.
- **Lazy statt vorab:** Ressourcen werden erst bei Bedarf initialisiert. Nach Ablauf einer Frist ohne Nutzung werden sie entladen, aber nicht deaktiviert. Im Grafikspeicher gelten deutlich längere Fristen als im Arbeitsspeicher.
- **Klare Besitzverhältnisse:** Alles, was Speicher, Streams oder Kernel besitzt, gehört der Subengine. Externe Bibliotheken sind jeweils an genau einer Stelle als Adapter angebunden. Typen externer Bibliotheken werden nur an diesen Rändern umgewandelt.

## Komponentenstruktur

Die Wurzel gliedert sich in die Engine und gemeinsam genutzte Bausteine.

- Engine: die Fassade nach außen.
  - Processor: Orchestrierung mit Workern, Warteschlangen, Benachrichtigung und Rückdruck.
  - Planner: ordnet Operationen den Warteschlangen zu.
    - Expander: zerlegt Tasks in Operationsketten.
    - Operation: Operationsarten der Embedding- und Similarity-Ketten.
  - Subengine: Ressourcen- und Speicherverwaltung je Hardwareart.
    - Zentralprozessor: Gerät mit Ressourcen (Kerne) und Lanes. Speicher mit Kernel, Index (Eintrag, Datei im Parquet-Format, Manager), Tokenizer und Normalizer.
    - Grafikprozessor: Gerät mit Ressourcen (Recheneinheiten, Streaming-Multiprozessoren) und Lanes. Speicher mit Kernel und Index (Segmenttabelle, Segment, Manager).
    - Gemeinsam: Modell.
  - Task: Embedding-Task und Similarity-Task.
- Gemeinsam: Warteschlange.

## Typischer Anwendungsfall

Der Ablauf folgt dieser Reihenfolge:

- Das Anwendungsprogramm startet, die Engine wird instanziiert und konfiguriert. Indizes können registriert werden, werden aber weder geladen noch verwendet.
- Die Engine wird initialisiert. Der interne Executor des Processors startet die Mindestzahl untätiger Threads.
- Die Engine ist bereit, verbraucht aber noch keine Modell- oder Geräteressourcen.
- Tasks für verschiedene Aufgaben werden über die Engine eingereiht.
- Der Expander im Planner zerlegt die Tasks in Operationsketten. Der Planner stellt fest, welche Subengines konfiguriert sind, und startet sie.
- Benötigt eine Operation bestimmte Ressourcen, wird das den Subengines mitgeteilt. Diese registrieren kompatible Ressourcen (Fähigkeiten) in den ihnen zugewiesenen und in den gemeinsamen Warteschlangen.
- Der Planner weist die Operationen anhand der kompatiblen Fähigkeiten den Warteschlangen zu, vorzugsweise der gemeinsamen.
- Sobald eine aktivierte Lane frei ist, zieht sie eine Operation und initialisiert die Ressourcen:
  - Tokenizer und Normalisierung werden lazy initialisiert.
  - Kernel werden lazy übersetzt und geladen.
  - Indizes und Modelle werden geladen.
  - Wenn sinnvoll, werden weitere Operationen, die mit demselben Index oder Modell laufen können oder müssen, zu einem Batch aggregiert.
- Ergebnisse werden bereitgehalten und können abgeholt werden, per Ereignis oder Benachrichtigung.
- Nach einer bestimmten Zeit werden unbenutzte Ressourcen entladen, aber nicht deaktiviert. Die Entladefristen für Modelle und Indizes im Grafikspeicher sind wesentlich länger als im Arbeitsspeicher.
- Normalisierung bezeichnet die zum Tokenizer gehörende. Zusätzlich kann es eine externe, vorgeschaltete Textnormalisierung für die Chunks geben.

## Engine

### Konfiguration und Start

- Die Engine wird über einen schrittweisen Konfigurationsvorgang (Builder-Prinzip) erzeugt, der erst mit einem expliziten Startbefehl wirksam wird.
- Die Angabe eines Modells ist verpflichtend. Fehlt sie, scheitert der Start mit einem Konfigurationsfehler, der das fehlende Feld benennt.
- Die Engine konfiguriert den Planner mit Subengines, Modellen und Indizes.
- Modell- und Geräteressourcen werden beim Start nicht erzeugt, sondern erst durch den ersten Task angelegt.
- Die Engine bietet Methoden zum Einreihen der beiden Task-Arten Embedding und Similarity.

### Verwaltung von Indizes

- Beliebig viele durchsuchbare Indizes können registriert werden. Jeder erhält eine eindeutige Kennung.
- Das Registrieren lädt den Index nicht. Unmittelbar danach gilt er als nicht geladen. Das Laden löst erst ein Task aus.

### Beenden

- Die Engine bietet ein geordnetes Beenden, das auch beim automatischen Freigeben der Instanz abläuft.
- Zuerst werden die Worker angehalten und zusammengeführt.
- Danach werden alle Tasks abgebrochen, die kein Worker übernommen hat. Wartende Aufrufer erhalten den Fehler „abgebrochen“.
- Das explizite Beenden meldet bei Erfolg eine positive Rückmeldung.

## Task

### Aufbau, Zustände und Ergebnis

- Ein Task kapselt Daten, Parameter, Ergebnis, Zustand und Operationen. Die Operationen verweisen zurück auf ihren Task.
- Priorität, kompatible Ressource, Modell und Modus sind gewöhnliche Parameter (Metadaten). Die Warteschlange kennt sie nicht.
- Der Expander zerlegt jeden Task in eine Kette von Operationen.
- Zustände eines Tasks: eingereiht, übernommen, erledigt, abgebrochen.
- Das Ergebnis wird genau einmal gesetzt und ist entweder eine Ausgabe oder ein Fehler. Wartende werden über eine Benachrichtigung geweckt.

### Task-Handle und Aufrufer

- Der Zustand ist ohne Warten abfragbar:
  - erledigt: das Ergebnis, also Ausgabe oder Fehler,
  - abgebrochen: der Fehler „abgebrochen“,
  - eingereiht oder in Bearbeitung: „noch kein Ergebnis“,
  - erledigt ohne verfügbares Ergebnis: gilt als abgebrochen.
- Der Aufrufer kann unbegrenzt oder mit Zeitlimit auf den Abschluss warten. Er wartet auf das Ergebnissignal und wacht zusätzlich periodisch auf, um den Zustand zu prüfen.
- Reihenfolge beim Warten: erst die Anmeldung für die Benachrichtigung aktivieren, dann den Zustand lesen, dann warten. So geht keine Benachrichtigung verloren.
- Das Zeitlimit ist die eigene Deadline des Aufrufers. Bei Überschreitung wird der Fehler „Zeitüberschreitung“ geliefert und der Handle freigegeben.
- Wird ein unfertiger Handle freigegeben, wird der Task abgebrochen, sofern ihn noch kein Worker übernommen hat. Übernommene Tasks laufen weiter. Der Zustand „abgebrochen“ wird lokal abgeleitet.

### Erweiterbare Zusatzparameter

- Das Parameterobjekt jeder Task-Art ist erweiterbar und nimmt beliebige zusätzliche, typgebundene Werte auf (typindizierte Erweiterungsmenge).
- Je Typ ist höchstens ein Wert gespeichert. Erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
- Ein Wert ist nach Typ abfragbar (sonst „nicht vorhanden“) und nach Typ entfernbar, mit Rückmeldung, ob er vorhanden war.
- Anzahl der Werte und Leere der Sammlung sind abfragbar.
- Beim Kopieren der Parameter werden die Werte geteilt statt dupliziert, damit abgeleitete Tasks (etwa von Addons) die Parameter günstig übernehmen.
- Die Werte werden unverändert durch Warteschlange und Planung gereicht und sind sicher zwischen Threads übertragbar.

### Embedding-Task

- Eingabe: ein Text oder eine Liste von Texten.
- Parameter: Modell, Modus (Anfrage oder Passage), Präzision, optionaler Zielindex, erweiterbare Zusatzparameter.
- Ergebnis: die Embeddings als Liste von Zahlenvektoren, ein Vektor je Eingabetext in Eingabereihenfolge.
- Ein Embedding- beziehungsweise Indizierungs-Task kann so konfiguriert werden, dass er das Embedding samt Metadaten direkt in den Index schreibt. Er liefert dann Eintragsreferenzen beziehungsweise Kennungen des Index.

### Similarity-Task

- Eingabe: eine Liste von Anfragevektoren. Eine Textanfrage wird als Folge aus einem Embedding-Task im Anfragemodus und einem Similarity-Task mit dem Ergebnisvektor ausgedrückt, gesteuert durch die Anwendung oder ein Addon.
- Parameter:
  - Kennung des zu durchsuchenden Index (verpflichtend), daraus abgeleitet das Modell,
  - maximale Trefferzahl (Standard zehn),
  - Scope (Suchbereich), Standard ist der gesamte Index, einschränkbar auf eine Teilmenge,
  - Bewertungsverfahren, Standard ist die Kosinusähnlichkeit,
  - Präzision und erweiterbare Zusatzparameter.
- Eine Neubewertung der Treffer (Reranking) ist keine eigene Task-Art. Sie wird als zweite Similarity mit anderem Bewertungsverfahren auf einem verengten Scope ausgedrückt, dessen Kandidaten aus einem früheren Ergebnis stammen.
- Ergebnis: Eintragsreferenzen oder Kennungen (Chunk, Zeile, Bereich, Datei oder Index) mit Bewertung. Nach außen erscheint jeder Treffer mit Eintrag, Bewertung und Markierung „veraltet“.

### Ergebnisarten

- Das Ergebnis eines Tasks ist genau eine Ergebnisart:
  - Embeddings als Liste von Zahlenvektoren,
  - Eintragsreferenzen beziehungsweise Kennungen beim direkten Schreiben in den Index,
  - Treffer als Liste von Paaren aus Eintragskennung und Bewertung,
  - ein Ähnlichkeitsbericht,
  - eine Liste von Bewertungen (etwa aus einer Cross-Encoder-Inferenz).
- Die Menge der Ergebnisarten ist erweiterbar. Ergebnisse sind auf Gleichheit vergleichbar.

### Ähnlichkeitsbericht

- Der Bericht liefert rohe Bewertungen und die Fakten, anhand derer eine äußere Schicht über Folgetasks entscheidet. Er enthält Task-Kennung und Suchparameter.
- Inhalt:
  - Anzahl der Kandidaten, also der tatsächlich bewerteten Einträge im aufgelösten Scope,
  - Anzahl der verworfenen Einträge,
  - Anzahl der veralteten Einträge unter den zurückgegebenen Treffern,
  - Rangliste der Treffer mit Eintragskennung, Bewertung, Kennzeichen „veraltet“ und optionalem Quellverweis.
- Aufbau:
  - Eingehende Treffer dürfen ungeordnet sein und mehr Einträge enthalten als die maximale Trefferzahl.
  - Treffer ohne gültigen Zahlenwert werden verworfen und zur Zahl der vorgelagert verworfenen Einträge addiert, etwa wenn ein Eintrag aus dem Index entfernt wurde.
  - Sortierung absteigend nach Bewertung, bei Gleichstand steht die niedrigere Eintragskennung vorn. Danach wird auf die maximale Trefferzahl gekürzt.
  - Das Kennzeichen „veraltet“ wird je Treffer übernommen und nur über die zurückgegebenen Treffer gezählt.
  - Der Quellverweis wird je zurückgegebenem Treffer über eine vom Aufrufer gelieferte Nachschlagefunktion ermittelt und kann fehlen.
  - Fehlen die Suchparameter, endet der Task mit einem Fehler.
- Kennzahlen beziehen sich auf die zurückgegebenen Treffer. Ohne Treffer sind sie nicht vorhanden:
  - Anzahl, bester und schlechtester Wert, arithmetischer Mittelwert, Standardabweichung der Grundgesamtheit.
  - Abstand zwischen erstem und zweitem Treffer (Lücke). Ein kleiner Abstand zeigt eine unsichere Spitzenreihenfolge an. Mindestens zwei Treffer nötig.
  - Abstand zwischen erstem und letztem Treffer als Maß der Streuung. Mehr als ein Treffer nötig.
  - Abstand des besten Werts vom Mittelwert in Standardabweichungen (Standardwert). Bei einer Standardabweichung nahe null nicht vorhanden.
  - Wahrscheinlichkeitsverteilung über die Bewertungen per Softmax mit einstellbarer Temperatur. Die Temperatur muss endlich und größer als null sein, sonst ist das Ergebnis nicht vorhanden. Die Berechnung ist numerisch stabil, indem sie auf den besten Wert bezogen wird. Für die verwendete Modellfamilie liegen Bewertungen in einem engen Band, daher ist eine Temperatur von etwa 0,01 bis 0,05 sinnvoll.
  - Wahrscheinlichkeitsmasse des besten Treffers.
  - Entropie der Verteilung in natürlichen Einheiten. Null bedeutet einen eindeutigen Gewinner.
  - Knickpunkt der sortierten Bewertungskurve: der Treffer mit dem größten Abstand unterhalb der Verbindungslinie vom ersten zum letzten Wert. Mindestens drei Treffer nötig. Bei flacher Kurve oder ohne Punkt unterhalb der Linie nicht vorhanden. Bei Gleichstand gewinnt der frühere Treffer.

## Planner, Expander und Operation

### Planner

- Der Planner nimmt Tasks entgegen, lässt sie vom Expander zerlegen und reiht die Operationen in passende Warteschlangen ein. Findet sich keine passende Warteschlange, scheitert die Einreihung mit dem Fehler „fehlende Planungsfähigkeit“.
- Es gibt getrennte Operations-Warteschlangen für Grafikprozessor, Zentralprozessor und gemeinsam. Die Zuordnung richtet sich nach den kompatiblen Fähigkeiten der Subengines. Bevorzugt wird die gemeinsame Warteschlange.
- Der Planner verwaltet je Subengine, welche Ressourcenarten sie anbietet. Die Angebote sind schrittweise erweiterbar.
- Beim Erzeugen einer Operation aktiviert der Planner die benötigten Subengines blockierend:
  - Diese aktivieren ihre Ressourcen samt Lanes ebenfalls blockierend, ausschließlich als Aktivierung ohne Laden.
  - Die Lanes registrieren die aktivierten Ressourcen in den Warteschlangen.
  - Das Laden schwerer Ressourcen (Modelle, Indizes, Kernel) geschieht ausschließlich lazy durch die Iteration der Subengine.
- Unterschieden wird zwischen verfügbarer Ressource (registrierte Fähigkeit) und geladener Ressource (initialisiert und im Speicher).
- Für jede Task-Art ist vorab abfragbar, welche Ressourcenart die erste Operation benötigt: bei Similarity der Scorer, bei Embedding und Inferenz der Tokenizer. Der Processor nutzt das für die Expansion nach Unterversorgungshinweis.

### Expander und Planungsprinzip

- Der Expander nimmt einen Task (Daten und Parameter) und zerlegt ihn genau einmal in eine geordnete Kette von Operationen.
- Er übersetzt die Task-Parameter in die konkrete Konfiguration der Operationen: Index, Tokenizer, Modell, Scorer, Pooling, Präfix, Zieldimension, Trefferanzahl.
- Er arbeitet nur mit Konfigurationsregeln, den Fähigkeiten der Subengines und dem Unterversorgungshinweis. Ohne Hinweis gilt ein leerer Hinweis.
- Er ist eine reine Funktion: zustandslos und wiederholbar, gleiche Eingabe ergibt dieselbe Kette. Das Ergebnis hängt nicht von den Nutzdaten ab. Er hat keine Warteschlangen, keine Worker, keine Benachrichtigung und startet keine Subengines.
- Eingaben:
  - der Task mit Daten und Parameterobjekt,
  - die Pipeline-Konfiguration pro Modell: Präfixe, Pooling, Normalisierung, maximale Sequenzlänge, Datentyp.
- Erhält ein Planer einen Task einer fremden Art, lehnt er ihn mit einer Meldung ab, die beide Task-Arten nennt.
- Task-Art und Anzahl der geplanten Operationen werden auf Diagnose-Ebene protokolliert.

### Modellauswahl

- Embedding- und Similarity-Tasks verwenden das konfigurierte Standardmodell. Die Kompatibilität mit einem Index ist über den Indexkopf gesichert.
- Ist das gewählte Modell nicht konfiguriert oder fehlt für die Kombination aus Modell und Task-Art ein Planer, wird mit „unbekanntes Modell“ unter Nennung des Modells abgebrochen.
- Modell, Modellkonfiguration, Subengine-Angebote und Unterversorgungshinweis stehen dem Planer gemeinsam zur Verfügung.

### Expansionsregeln

- Embedding, in dieser Reihenfolge:
  - Tokenisierung (Zentralprozessor, Tokenizer-Ressource),
  - Embedding-Berechnung (Zentralprozessor oder Grafikprozessor, Encoder-Ressource), mit konfigurierter Zieldimension,
  - Pooling (Encoder-Ressource), entfällt beim Verfahren „keines“, dann bleiben die Token-Vektoren erhalten,
  - euklidische Normierung auf Länge eins (Encoder-Ressource), entfällt bei abgeschalteter Normalisierung,
  - Schreiben in den Index (Index-Ressource, referenziert den Zielindex), entfällt ohne Zielindex.
- Similarity, zwei Operationen:
  - Similarity-Berechnung über den Scope mit Zieldimension,
  - Auswahl der besten Treffer mit Trefferanzahl, begrenzt auf den Höchstwert einer vorzeichenlosen 32-Bit-Ganzzahl.
  - Beide fordern die Scorer-Ressource des Modells an und referenzieren den Index.
  - Eine Trefferanzahl von null wird mit einem Konfigurationsfehler abgelehnt.
- Similarity mit Cross-Encoder: Paare aus Anfrage und Text aus der Nutzlast, Inferenz des Cross-Encoders, Bewertungen. Der Scope stammt aus einem früheren Ergebnis. Ist kein Cross-Encoder-Scorer verfügbar, wird der Task mit „Bewertungsverfahren nicht verfügbar“ unter Nennung des Scorers abgelehnt.

### Schrittweise Übergabe und Ergebnisaufbereitung

- Zunächst wird nur die erste Operation der Kette bereitgestellt, zusammen mit einer Sammelfunktion für die endgültige Ausgabe.
- Nach Abschluss einer Operation wird die nächste erzeugt. Ihre Eingabe ist die tatsächliche Ausgabe der vorigen. Jede Operation bleibt dem Kontext ihres Tasks zugeordnet. Nach der letzten Operation folgt keine weitere.
- Liefert die letzte Operation ein Ergebnis im Arbeitsspeicher, wird es als Gesamtergebnis gespeichert, sonst bleibt es leer. Eine leere Kette stellt keine Operation bereit und endet beim Einsammeln mit dem Fehler „fehlendes Ergebnis“.
- Die Sammelfunktion entnimmt das Ergebnis, sodass es nur einmal verfügbar ist.
- Eingabeaufbereitung:
  - Embedding: die Textliste ist Eingabe der ersten Operation, die Textanzahl wird für die Ergebnisaufteilung gemerkt.
  - Similarity: alle Anfragevektoren werden zu einer flachen Folge von Gleitkommazahlen zusammengefasst.
  - Inferenz: Anfrage und Textabschnitt werden als geordnetes Textpaar übergeben.
- Ergebnisaufbereitung:
  - Embedding: die flache Folge wird in einen Vektor je Text aufgeteilt. Ist die Textanzahl null oder die Wertanzahl nicht restlos teilbar, endet der Task mit einem Fehler, der Wert- und Textanzahl nennt.
  - Similarity mit einfacher Trefferliste: Paare aus Eintragskennung und Bewertung.
  - Similarity mit bewerteten Treffern: Ähnlichkeitsbericht.
  - Inferenz: die Gleitkommazahlen als Bewertungen.
  - Passt das Ergebnis nicht zur Task-Art oder fehlt es, wird ein Fehler mit Nennung der Task-Art beziehungsweise der fehlenden Ausgabe gemeldet. Ergebnisfehler gelten als Konfigurationsfehler der Planung.

### Operation

- Operationsarten (erweiterbar): Tokenisierung, Embedding, Pooling, Normalisierung auf Einheitslänge, Similarity-Berechnung, Auswahl der besten Treffer, Schreiben in den Index, Inferenz, ausdrückliche Übertragung vom Gerät in den Arbeitsspeicher.
- Jede Operation nennt Art, Parameter und Ressourcenanforderungen, wobei die Anforderungen aus den Parametern folgen. Operationen sind von mehreren Threads gleichzeitig verwendbar.
- Parameter enthalten Ressourcenanforderungen sowie optional Trefferanzahl und Dimension (beim Anlegen nicht gesetzt). Sie sind unveränderlich, auf Gleichheit vergleichbar, als Schlüssel für Zwischenspeicher oder Abhängigkeitsgraphen verwendbar und dienen zum Filtern vor der Übernahme.
- Eine Operation lässt sich als reine Beschreibung aus Art und Parametern darstellen, ebenfalls vergleichbar und als Schlüssel nutzbar.
- Priorität und Task-Kennung sind Metadaten.
- Eine Operation gehört zu genau einem Task. Der Graph ergibt sich implizit aus der Reihenfolge der Operationen.
- Jede Berechnung ist eine Operation mit expliziten Ein- und Ausgabe-Regionen. Ein- und Ausgabe sind entweder Daten im Arbeitsspeicher oder eine Belegung einer Geräte-Region, nie rohe Tensoren oder Zeiger.
- Datenformen im Arbeitsspeicher (erweiterbar): Textliste, Liste von Tokenfolgen, Folge von Gleitkommazahlen, Trefferliste aus Eintragskennung und Bewertung, bewertetes Trefferergebnis.
- Ein bewerteter Treffer enthält Eintragskennung auf Manager-Ebene, Bewertung, Kennzeichen „veraltet“ und optionalen Quellverweis.
- Ein bewertetes Trefferergebnis ist ein Zwischenergebnis mit Anzahl bewerteter Einträge im aufgelösten Scope, Anzahl bei der Auflösung verworfener Treffer (Eintrag entfernt) und verbleibenden Treffern. Rangbildung und Zusammenfassung erfolgen danach.
- Ergebnis der Similarity-Operation: Bewertungen sowie die Grundlage für Anzahl, Lücken, Streuung, Entropie und Knickpunkt.

### Ressourcenanforderungen

- Modelle und Indizes werden durch Textkennungen, Geräte durch eine Zahl identifiziert.
- Ressourcenarten (erweiterbar): Tokenizer, Normalizer, Encoder, Scorer, Index, Cross-Encoder.
- Auswahl der Ausführungsumgebung (erweiterbar): nur Zentralprozessor, nur Grafikprozessor, beliebig. „Beliebig“ akzeptiert jede Geräteart, die anderen nur die passende.
- Platzierungshinweis (Affinität, erweiterbar) auf eine Region oder ein Gerät, als Hinweis für Pull-Filter.
- Anforderungen enthalten Ressourcenart, optional Modell, optional Index, Ausführungsumgebung, geforderte Fähigkeiten und optional Platzierungshinweis. Beim Anlegen sind nur Ressourcenart und Ausführungsumgebung verlangt, die übrigen Angaben sind einzeln ergänzbar.
- Anforderungen sind vergleichbar und als Schlüssel verwendbar.
- Ein Gerät erfüllt die Anforderungen genau dann, wenn seine Art akzeptiert wird und seine Fähigkeiten alle geforderten abdecken. Der Platzierungshinweis spielt dabei bewusst keine Rolle.

### Materialisieren

- Das Kennzeichen „Materialisieren“ legt fest, ob das Ergebnis beim Fertigwerden in den Arbeitsspeicher zurückgeholt wird oder als Verweis im Grafikspeicher bleibt.
- Das Ergebnis bleibt nur dann im Grafikspeicher, wenn diese und die unmittelbar folgende Operation beide ausdrücklich auf dem Grafikprozessor laufen.
- In allen anderen Fällen wird materialisiert: Nachfolger auf dem Zentralprozessor, Task-Ende, Spiegelung in den Index, freie Wahl der Ausführungsumgebung.
- Der Planner legt das Kennzeichen fest, die Operation führt es aus.

### Task-Kontext, Folgeoperationen und Abschluss

- Je Task gibt es einen gemeinsam nutzbaren Kontext mit Kennung, Priorität, Zähler offener Operationen, Abbruchkennzeichen und erstem Fehler. Bei Erstellung wird die anfängliche Anzahl offener Operationen angegeben.
- Alle Zustände sind nebenläufig korrekt lesbar und veränderbar.
- Folgeoperationen werden registriert, bevor sie eingereiht werden und bevor die erzeugende Operation abschließt, damit der Zähler nicht vorzeitig null erreicht. Sie werden am Ende der Warteschlange eingereiht.
- Der Abschluss einer Operation verringert den Zähler und meldet, ob es die letzte offene Operation eines nicht abgebrochenen Tasks war. Nur dann ist der Task erfolgreich erledigt.
- Ein Abschluss ohne offene Operation ist ein Programmierfehler und wird in Entwicklungsumgebungen erkannt.
- Der erste Fehler setzt den Task auf „abgebrochen“ und das Ergebnis auf Fehler. Spätere Fehler überschreiben ihn nicht. Es wird zurückgemeldet, ob es der erste Fehler war, und der Abbruch wird protokolliert. Die übrigen Operationen scheitern bei der Übernahme.

### Operationsknoten

- Ein Knoten bündelt Operation, Eingabe, zunächst leere Ausgabe, Materialisierungskennzeichen, zugehörigen Task und optional einen Nachfolger.
- Beim Anlegen werden Task-Kennung, Operationsart, Ressourcenart und Materialisierungskennzeichen protokolliert.
- Ein Nachfolger ist nachträglich anfügbar. Er erzeugt aus der Ausgabe und dem Task-Kontext null oder mehr Folgeknoten. Ohne Folgeknoten endet die Kette und die Ausgabe wird Task-Ergebnis. Nachfolger sind nebenläufig verwendbar.
- Die Ausgabe ist einmal setzbar. Ein zweiter Versuch gibt den Wert unverändert an den Aufrufer zurück.

### Belegung von Geräte-Regionen

- Der Zugriff auf eine Geräte-Region wird über eine Belegung (Pin) abgebildet. Solange mindestens eine Belegung besteht, darf die Region nicht verdrängt werden.
- Je Region gibt es einen gemeinsam genutzten Fixierungszähler, anfangs null, nebenläufig korrekt. Zählerstand und Zustand „nicht fixiert“ (Zähler null) sind abfragbar.
- Erzeugen einer Belegung erhöht den Zähler, ihr Ende verringert ihn automatisch ohne manuelles Freigeben.
- Optional meldet ein Rückmeldekanal die freigegebene Region. Ist der Empfänger nicht mehr erreichbar, endet die Belegung trotzdem fehlerfrei.
- Jede Belegung liefert ihre Region und ist auch unter der Bezeichnung Gerätehandle verfügbar.

## Processor

### Struktur

- Der Processor besitzt die Task-Warteschlange, drei Operations-Warteschlangen (Zentralprozessor, Grafikprozessor, gemeinsam), den Parkplatz der Worker, die Benachrichtigung und die Unterversorgungshinweise.
- Er ruft den Planner für die Expansion.
- Der interne Executor startet bei der Initialisierung eine Mindestzahl untätiger Threads (Worker).
- Eine Subengine prüft zuerst ihre eigene Operations-Warteschlange, danach die gemeinsame.
- Subengines besitzen ihre Ressourcen: Tokenizer, Encoder, Scorer, Index-Segmente. Der Tokenizer ist eine Ressource der Zentralprozessor-Subengine.

### Worker

- Die Zahl der Worker der gesamten Engine ist die Zahl der Zentralprozessor-Kerne minus eins. Das ist eine Obergrenze für Kontextwechsel, keine Kernzuordnung.
- Worker parken im Processor und wachen bei jeder Zustandsänderung per Benachrichtigung auf.
- Zentralprozessor: Der Worker führt die Operation synchron aus und kommt mit dem Ergebnis zurück.
- Grafikprozessor: Der Worker startet die Operation beziehungsweise den Kernel und kehrt zurück. Nach der Abschlussbenachrichtigung verarbeitet ein freier Worker das Ergebnis.
- Blockiert ein Worker, etwa in der Rechenbibliothek oder deren Thread-Pool, ist das das natürliche Rückdrucksignal ohne Sonderbehandlung.

### Worker-Schleife

Nach jeder Operation wird in dieser Prioritätsreihenfolge neu geprüft:

- Ergebnisse des Grafikprozessors verarbeiten und in den Index zurückspiegeln.
- Grafikprozessor-Ressourcen füttern: Index in den Grafikspeicher synchronisieren, bei freier Lane starten.
- Zentralprozessor-Ressourcen füttern, ausführen, Ergebnis zurückgeben, auch an Indizes.
- Neue Tasks expandieren (Planner).

### Abschluss auf dem Grafikprozessor

- Der Rückruf am Ende des Streams legt nur einen Abschlusseintrag ab und löst die Benachrichtigung aus. Im Rückruf erfolgt kein Aufruf der Grafikprozessor-Schnittstelle.
- Der Schritt zur Ergebnisverarbeitung prüft zusätzlich per Ereignisabfrage und übernimmt eine fertige Lane per atomarem Austausch. Das deckt verpasste Rückrufe ab.

### Rückdruck und Expansion

- Eine Iteration ohne Treffer bei freier Ressource erzeugt einen Unterversorgungshinweis mit Ressourcenart und Parametern.
- Die Hinweise liegen als Menge je Subengine-Art vor, nicht als Warteschlange. Sie verfallen bei einem Treffer oder wenn die Ressource belegt ist.
- Expandiert wird nur bei einem Hinweis oder leerer Operations-Warteschlange. Bevorzugt werden Tasks, deren erste Operation die unterversorgte Ressource braucht.

## Warteschlange

### Zweck und Einsatz

- Die Warteschlange ist eine sperrfreie (lock-free) Mehrfach-Erzeuger-Mehrfach-Verbraucher-Struktur für Arbeitseinheiten mit eigenem Zustand, Ergebnis und Benachrichtigung.
- Sie wird viermal eingesetzt: als Task-Warteschlange und als Operations-Warteschlange für Zentralprozessor, Grafikprozessor und gemeinsam. Jede arbeitet unabhängig mit eigener Menge von Benachrichtigungskanälen.
- Typen von Metadaten, Nutzlast, Ergebnis und Fehler legt der Aufrufer fest. Operations-Warteschlangen verwenden denselben Aufbau mit eigenen Metadaten und Werten als Nutzlast.
- Bei Operationen wird die Fertigstellung nicht über das Ergebnis der Zustandseinheit verfolgt, sondern über den Zähler offener Operationen des Tasks.

### Struktur

- Jede Warteschlange ist genau eine einfach verkettete Liste mit internem Ankerknoten (Sentinel), ohne getrennte Listen je Priorität oder Ressource.
- Es gibt nur Anhängen und Entfernen. Anhängen geschieht per atomarem Compare-and-Swap am Nachfolger-Verweis des letzten Knotens, Entlinken ebenfalls per Compare-and-Swap auf dem Verweis.
- Die Reihenfolge entspricht dem Alter, also dem erfolgreichen Anhängen. Die Reihenfolge eines einzelnen Erzeugers bleibt erhalten.
- Es gibt keine Version, keinen Schnappschuss und keine Sperre auf der Kette. Nur das Nutzdatenfach eines einzelnen Knotens ist kurzzeitig gegen gleichzeitigen Zugriff geschützt.
- Metadaten sind für die Warteschlange undurchsichtig. Sie werden nicht interpretiert, sondern Verbrauchern zum Filtern bereitgestellt.

### Aufbau eines Knotens

- Unveränderliche Parameter zum Filtern vor der Übernahme.
- Gemeinsames Zustandsobjekt des Tasks, zugleich Handle des Aufrufers, mit atomarem Zustand, einmal setzbarem Ergebnis und Benachrichtigung für Wartende.
- Geschütztes, optionales Nutzdatenfach, das bei der Übernahme geleert wird.
- Atomar austauschbarer Verweis auf den Nachfolger.

### Einreihen

- Beliebig viele Einträge, jeder aus Metadaten, Nutzlast und Zustandseinheit. Der Aufrufer erhält einen Verweis auf den neuen Eintrag.
- Einreihen blockiert nie. Mehrere Erzeuger reihen gleichzeitig ein, ohne dass ein Eintrag verloren geht oder überschrieben wird.
- Ist der interne Endehinweis veraltet, wird das tatsächliche Ende gesucht und die Reihenfolge gewahrt. Zu Testzwecken lässt sich das Aktualisieren des Endehinweises gezielt unterdrücken.
- Beim Anlegen nimmt die Warteschlange eine beliebige, auch leere Menge von Benachrichtigungskanälen entgegen, je Subengine einen. Jedes Einreihen signalisiert alle Kanäle.

### Durchlaufen

- Es gibt einen Durchlauf aller Einträge in Altersreihenfolge und einen, der nur aktive Einträge liefert (weder erledigt noch abgebrochen). Der Ankerknoten erscheint nie.
- Durchläufe blockieren nie und bleiben bei gleichzeitigem Einreihen, Übernehmen und Abschließen gültig. Jeder gelieferte Eintrag bleibt gültig, auch nach logischem Entfernen.
- Der Durchlauf liefert exakt die Einträge in Einreihungsreihenfolge, ohne Auslassung und Wiederholung. Eine leere Warteschlange ergibt einen leeren Durchlauf.
- Erledigte und abgebrochene Einträge gelten als logisch entfernt und werden im aktiven Durchlauf übersprungen. Der vollständige Durchlauf liefert sie, solange sie nicht entlinkt sind.

### Zustandsmodell und Übergänge

- Vier Zustände: eingereiht, übernommen, erledigt, abgebrochen. Neue Einträge sind eingereiht.
- Erlaubt sind ausschließlich diese atomaren Übergänge, bei gleichzeitigen Versuchen gelingt höchstens einer:
  - **Übernahme:** eingereiht nach übernommen. Einzige Stelle, an der ein Abbruch relevant ist.
  - **Fertigstellung:** übernommen nach erledigt.
  - **Abbruch:** eingereiht nach abgebrochen.
- Übernahme und Abbruch schließen sich gegenseitig aus. Der Endzustand passt immer zum Gewinner. Der Zustand ist jederzeit als konsistente Momentaufnahme abfragbar. Ein ungültiger interner Zustandswert ist ein nicht behebbarer interner Fehler.
- Übernahme:
  - Der Übernehmer erhält die Nutzlast als Besitzübergabe, nie als Kopie, genau einmal.
  - Erneutes Übernehmen, Übernahme eines abgebrochenen Eintrags oder des Ankerknotens scheitert ohne Nutzlast.
  - Bei gleichzeitigen Versuchen gewinnt genau einer. Die Übernahme blockiert nie und wartet nicht, während das Nutzdatenfach gesperrt ist.
  - Sie ist auch über eine Funktion der Warteschlange verfügbar, mit identischem Verhalten. Jeder Versuch wird mit Ausgang protokolliert.
- Fertigstellung:
  - Nur der erfolgreiche Übernehmer darf fertigstellen, und nur einmal. Das Ergebnis ist Erfolgs- oder Fehlerwert.
  - Das Ergebnis ist gespeichert und sichtbar, bevor der Zustand „erledigt“ erkennbar wird.
  - Ein zweiter Versuch verändert das Ergebnis nicht, wird mit Warnung protokolliert und fällt in Entwicklungsumgebungen als Programmierfehler auf. Bei „abgebrochen“ wird das Ergebnis verworfen.
  - Wartende werden geweckt. Ohne gesetztes Ergebnis liefert die Abfrage nichts, abgebrochene Einträge haben nie ein Ergebnis.
- Abbruch:
  - Nur aus „eingereiht“ möglich, mit Rückmeldung über Erfolg. Danach ist keine Übernahme mehr möglich.
  - In jedem anderen Zustand ändert sich nichts und es wird Misserfolg gemeldet.
  - Ein erfolgreicher Abbruch weckt alle Wartenden. Jeder Versuch wird protokolliert.

### Warten auf Abschluss

- Asynchrones Warten auf „erledigt“ oder „abgebrochen“, ohne einen Thread zu blockieren. Bei bereits erreichtem Endzustand endet es sofort.
- Keine Benachrichtigung geht verloren, auch wenn der Abschluss zwischen Zustandsprüfung und Wartebeginn eintritt.
- Mehrere Wartende werden gleichzeitig unterstützt und alle geweckt. Ein Aufwecken ohne Endzustand führt zu erneuter Prüfung.

### Besitz und Speicherfreigabe

- Besitzkette: Warteschlangenkopf, Kette, Durchläufer. Jeder Durchläufer hält pro Schritt eine Referenz auf den Knoten.
- Nach der Übernahme bleibt die leere Hülle in der Kette, bis sie entlinkt wurde und der letzte Durchläufer sie verlassen hat. Dann wird sie automatisch freigegeben.
- Entlinken ist optionale Aufräumarbeit: ein einfacher atomarer Austausch auf dem Verweis des Vorgängers. Der entlinkte Knoten behält seinen Nachfolger, damit ein Durchläufer ohne Neustart weiterläuft. Nicht entlinkte Knoten werden spätestens beim Verwerfen der Warteschlange freigegeben.
- Freigabe über Referenzzählung der Knoten. Epochenbasierte Freigabe, Slab-Verwaltung, Freiliste und Generationszähler entfallen für die Warteschlange.
- Lange Ketten, auch über hunderttausend Einträge, werden iterativ statt rekursiv freigegeben, ohne Überlauf des Aufrufstapels. Von anderen Besitzern gehaltene Kettenteile bleiben gültig, bis deren letzter Besitzer sie freigibt.
- Außerhalb der einen Komponente für den atomaren Zeigertausch darf kein Programmcode eingesetzt werden, der die Speichersicherheitsgarantien der Sprache umgeht.

## Pull-Ausführung der Subengines

### Schleife und Iteration

- Pro Subengine läuft eine Schleife mit genau einer Wartebedingung, der Benachrichtigung. Danach folgt eine Iteration über die Warteschlange.
- Die Iteration geht in Altersreihenfolge Knoten für Knoten vor:
  - Sind alle Ressourcen der Subengine ausgelastet, endet die Iteration.
  - Akzeptiert der Filter die Parameter nicht, wird der Knoten übersprungen. Prioritätspräferenz ergibt sich aus Filter und Reihenfolge, also aus Metadaten.
  - Der Ressourceneintrag wird über die Anforderungsangabe der Operation bestimmt.
  - Ist die Ressource nicht initialisiert, wird die Lazy-Initialisierung angestoßen: atomarer Übergang von „nicht initialisiert“ nach „wird geladen“. Der Task bleibt in der Warteschlange.
  - Ist die Ressource bereit, hat freie Kapazität und gelingt die Übernahme, führt die Ressource den Knoten aus.
  - In allen anderen Zuständen (wird geladen, belegt, Rückzug) wird der Knoten übersprungen.
- Am Ende jeder Iteration werden ungenutzte Ressourcen verdrängt.

### Lanes, Batches und Ressourcen

- Die Lanes einer Ressource ziehen Operationen. Die Lane des Zentralprozessors ist ein Thread des Processors.
- Lanes implementieren Batching: Der Batch beginnt mit der Ausgangsoperation. Die Ressource geht von dort vorwärts durch die Liste, übernimmt alles, was der Filter akzeptiert und in den Batch passt, und startet dann den Batch.
- Packen, Längen-Bucketing und Streams sind reine Ressourcenlogik.
- Das Ergebnis löst die Benachrichtigung der Subengine aus.
- Ressourcenzustände: nicht initialisiert, wird geladen, bereit, Rückzug nach Fehler (Backoff).

### Benachrichtigungstopologie

- Anhängen an eine Warteschlange: Weiterleitung an die Benachrichtigung jeder Subengine.
- Ergebnis, abgeschlossene Initialisierung und Verdrängungstimer: eigene Benachrichtigung der Subengine.
- Eine Iteration mit Fortschritt löst die eigene Benachrichtigung aus. Die Schleife ist ihre eigene Benachrichtigung.

### Regeln

- Keine Reservierung, kein Vorab-Übernehmen, kein Kopfblockieren.
- Lazy-Initialisierung geschieht nur durch die Iteration, nie durch das Einfügen.
- Verdrängung ist nur erlaubt, wenn keine Operation läuft und die Lebensdauer abgelaufen ist. Im Grafikspeicher ist sie deutlich länger als im Arbeitsspeicher.
- Initialisierungs- oder Startfehler schreiben einen Fehler in die Task-Ergebnisse des Batches und versetzen die Ressource in den Rückzug. Das wird mit Task-Kennung protokolliert.

### Auslastungsmeldung

- Eine Subengine stellt Ressourcen und Ausführungskapazität eines Rechensubstrats bereit und ist aus mehreren Threads nutzbar.
- Sie gibt Auskunft über Art, Gerät, Kapazität und geglätteten Durchsatz in Einträgen pro Sekunde.
- Die Arbeit kommt aus den Warteschlangen des Processors. Die Subengine meldet nur, wie viel sie aufnehmen kann.

## Subengines

### Gemeinsame Regeln

- Die Ressourcenverwaltung arbeitet lazy: erst bei Bedarf initialisieren, bei Nichtnutzung verdrängen. Im Grafikspeicher gelten längere Fristen.
- Jede Subengine liefert eine Fähigkeitenmenge und registriert kompatible Ressourcen samt Lanes in ihren eigenen und den gemeinsamen Warteschlangen.
- Jede Subengine kalibriert ihre Hardware per Benchmark und stellt daraus Präzisionsprofile bereit.
- Jede Subengine implementiert ihre Rechenpfade hardwareoptimal und entscheidet selbst.
- Für die Stapelverarbeitung gibt es eine austauschbare Schnittstelle von Gleitkommastapel zu Gleitkommastapel:
  - Das Zentralprozessor-Verfahren ist Standard ohne einkompilierte Grafikbeschleunigung und Ausweichziel, wenn die Grafikbeschleunigung nicht initialisierbar ist.
  - Das Grafikprozessor-Verfahren ist die einzige Anbindungsstelle der Grafikbibliothek. Sein Anlegen scheitert ohne kompatiblen Grafikprozessor, wodurch automatisch auf den Zentralprozessor zurückgefallen wird.

### Invarianten

- In öffentlichen Signaturen gibt es keinen Tensor und keinen Rohzeiger, nur Region und Layout.
- Jede Grafikprozessor-Arbeit läuft über einen Ausführungskontext, nie auf dem Standard-Stream.
- Kernel werden nur über das Kernel-Register geladen. Kernel der externen Bibliothek sind dort eine weitere Implementierung.
- Planung und Zuteilung kennen weder die externe Bibliothek für neuronale Netze noch Ausführungsgraphen des Grafikprozessors.

### Zentralprozessor-Subengine

- **Gerät:** der Zentralprozessor, potenziell mehrere. Ressourcen sind die Kerne, jeweils mit Lanes.
- **Speicherverwaltung:** lädt Modelle und Indizes in den Arbeitsspeicher beziehungsweise streamt Daten hinein, kennt den verfügbaren Arbeitsspeicher und lagert Modelle, Kernel und Ressourcen bei Bedarf aus und ein.
- **Kernel:** eigene Implementierungen mit eigenem Register.
- **Index:** beim ersten Zugriff geladen, mit Eintrag, Datei und Manager.
- **Tokenizer und Normalizer:** Ressourcen dieser Subengine.
- **Besonderheiten:**
  - Mehr Speicher steht zur Verfügung, alles nutzt den Zentralprozessor.
  - Der Scorer ist selbst implementiert und nutzt Vektorbefehle.
  - Der Encoder läuft über das Zentralprozessor-Backend der Bibliothek.
  - Der Index ist per Speicherabbildung lesbar.
  - Keine Streams, kein Bereitstellungsspeicher für Kopien.
  - Standarddatentyp ist 32-Bit-Gleitkomma.
  - Kalibrierung und Kernel laufen auf eigenen Threads niedriger Priorität, getrennt von der asynchronen Laufzeitumgebung.

### Grafikprozessor-Subengine

- **Plattform:** Nvidia-Rechenplattform. Gerät ist der Grafikprozessor, potenziell mehrere, mit Recheneinheiten als Ressourcen und Lanes darauf.
- **Grafikspeicher-Manager:**
  - lädt oder streamt Modelle und Indizes in den Grafikspeicher,
  - kennt den verfügbaren Grafikspeicher und tauscht Modelle, Kernel und Ressourcen bei Bedarf aus und ein,
  - ist die Brücke zwischen dem Index im Arbeitsspeicher und der im Grafikspeicher abgebildeten Darstellung für die Kernel,
  - lädt Kernel sowie Ein- und Ausgabe-Arenen und Speicherbereiche,
  - synchronisiert dadurch den Index effektiv zwischen Arbeits- und Grafikspeicher.
- **Kernel:** mit Register. Die Adressabbildung im Grafikspeicher erfolgt über Slots.
- **Index:** Segmenttabelle und Segmente.
- **Besonderheiten:**
  - Der Grafikspeicher ist begrenzt.
  - Es gibt Streams, Bereitstellungsspeicher und Transfers.
  - Batching ist notwendig, damit Index-Segmente und Kernel effizient gestreamt und getauscht werden.
  - Standarddatentyp ist 16-Bit-Gleitkomma.

### Gemeinsame Ressourcen

Die folgenden Ressourcen liegen zentralprozessorseitig und werden von beiden Subengines genutzt.

- **Modell:** Die Modellbeschreibung stammt aus der Konfiguration, beschreibt eine Pipeline und besitzt nichts. Ein Register ordnet Architekturkennungen Encoder-Implementierungen und Bewertungsarten Scorer-Implementierungen zu. Neue Modelle bedeuten einen neuen Registereintrag. Mehrvektor-Parameter stammen aus der Konfiguration des Prüfpunkts, nicht aus festem Programmcode: maximale Anfragelänge, maximale Dokumentlänge, Markierungstoken, Dimension, Ausschlussliste.
- **Normalizer:** gehört zum Tokenizer (Unicode-Normalisierung, Kleinschreibung und weitere Schritte), läuft nur auf dem Zentralprozessor und ist über Parameter konfiguriert.
- **Tokenizer:** läuft nur auf dem Zentralprozessor, modellspezifisch oder geteilt. Er liefert Token-Kennungen als 32-Bit-Ganzzahlen. Mehr braucht der Grafikprozessor nicht, also kein Zusammensammeln auf dem Zentralprozessor und keine Adressabbildung.
- **Gewichtequelle:** safetensors, speicherabgebildet und kopierfrei auf Rohbytes. Die Gewichte werden pro Subengine in deren Arena kopiert.

## Tokenizer

### Laden

- Der Tokenizer wird aus seiner Konfigurationsdatei mit festem Dateinamen geladen, die im Verzeichnis der Modellkonfiguration liegt.
- Alternativ entsteht er aus einem bereits geladenen Tokenizer mit frei wählbarer Kennung, Schema für die Moduswahl und maximaler Sequenzlänge.
- Beim Laden aus einer Modellbeschreibung werden das Schema aus den hinterlegten Präfixen gebildet, die maximale Sequenzlänge übernommen und die Ressourcenkennung aus dem Dateipfad abgeleitet, sodass jeder geladene Tokenizer eindeutig identifizierbar ist.
- Fehlt die Datei oder ist sie unlesbar, bricht das Laden mit einem Ladefehler ab, der Pfad und Ursache nennt.
- Das Auffülltoken wird in dieser Reihenfolge bestimmt: im Tokenizer konfiguriertes Auffülltoken, dann das Token mit der Bezeichnung in spitzen Klammern, dann das Token mit der Bezeichnung in eckigen Klammern in Großbuchstaben. Fehlt jedes, bricht das Laden mit einem Ladefehler ab. Das Auffülltoken ist nach außen abfragbar.
- Die eingebaute automatische Auffüllung wird abgeschaltet. Auffüllen ist ausschließlich Aufgabe der Batch-Bildung.
- Jede Eingabe wird auf die maximale Sequenzlänge gekürzt, wobei das abschließende Sondertoken erhalten bleibt. Lehnt der Tokenizer die Kürzungseinstellung ab, bricht das Laden mit Ursache ab.

### Textvorbereitung

- Optional wird ein austauschbarer, nebenläufig nutzbarer Textnormalisierer übergeben. Ohne ihn bleibt der Text unverändert. Ein unverändernder Normalisierer wird mitgeliefert.
- Normalisierungen, die der Tokenizer selbst durchführt, werden nicht doppelt ausgeführt. Ein eigener Normalisierer ergänzt nur zusätzliche Umformungen.
- Die Normalisierung läuft vor dem Voranstellen von Präfixen oder Markierungen und vor der Tokenisierung.

### Kodierungsmodi und Schemata

- Zwei Kodierungsmodi: Anfrage und Passage (Dokument).
- **Präfix-Schema:** Je Modus wird der Anfrage- beziehungsweise Passagenpräfix vorangestellt. Die Tokenfolge bleibt danach unverändert.
- **Mehrvektor-Schema** (späte Interaktion): Je Modus wird eine Anfrage- beziehungsweise Dokumentmarkierung vorangestellt. Je Modellvariante konfigurierbar sind Markierungstext für Anfragen und Dokumente, feste Anfragelänge, Dokumenthöchstlänge und Kennung des Maskentokens.
  - Anfragemodus: Kürzen auf die Anfragelänge, Auffüllen mit dem Maskentoken auf genau diese Länge. Die aufgefüllten Positionen werden vom Modell berücksichtigt, nicht ausgeblendet (Anfrageerweiterung).
  - Dokumentmodus: Kürzen auf die Dokumenthöchstlänge, ohne Auffüllen. Auffülltokens werden verworfen.
  - Diese Kürzungen gelten zusätzlich zur allgemeinen Kürzung.

### Kodierung

- Eine Liste von Texten beliebiger als Zeichenkette lesbarer Typen wird in einem Modus in Token-Kennungen umgewandelt. Eine leere Liste wird mit „keine Eingabetexte“ abgelehnt.
- Je Text in dieser Reihenfolge: normalisieren, Präfix oder Markierung voranstellen, tokenisieren mit den vorgesehenen Sondertoken, schemaabhängig nachbearbeiten. Alle Texte werden gemeinsam als Batch tokenisiert.
- Scheitert die Tokenisierung, wird ein Kodierungsfehler mit Ursache gemeldet.
- Rückgabe in Eingabereihenfolge, ohne Auffüllung außer der festen Anfrageauffüllung im Mehrvektor-Schema.
- Protokolliert werden Anzahl gekürzter Texte mit Modus und Textanzahl sowie auf feinster Stufe die größte Sequenzlänge.
- Eine kombinierte Funktion kodiert und teilt danach in aufgefüllte, nach Länge gruppierte Batches gemäß Batch-Konfiguration. Jeder Batch ordnet seine Zeilen den Eingabeindizes zu. Kodierungsfehler, auch die leere Eingabe, werden unverändert weitergegeben. Die Batch-Anzahl wird protokolliert.

## Batch-Bildung für Eingabesequenzen

- Aus einer Liste von Tokenfolgen entsteht eine Liste von Batches. Jede Folge liegt in genau einem Batch. Jeder Batch führt die ursprünglichen Positionen seiner Zeilen mit. Leere Eingabe ergibt eine leere Liste.
- Konfiguration: Token-Budget je Batch (Zeilen mal Sequenzlänge), maximale Sequenzlänge, Kennung des Auffülltokens.
- Längen-Bucketing:
  - Zu lange Folgen werden auf die maximale Sequenzlänge gekürzt.
  - Die aufgefüllte Länge ist die kleinste Längenklasse aus 32, 64, 128, 256 und 512, die mindestens der gekürzten Länge entspricht. Oberhalb der größten Klasse gilt die maximale Sequenzlänge.
  - Die aufgefüllte Länge überschreitet nie die maximale Sequenzlänge und beträgt mindestens eins.
- Gruppierung:
  - Sortierung aufsteigend nach ursprünglicher Länge, bei Gleichheit stabil in Eingabereihenfolge.
  - Vor jedem Hinzufügen wird geprüft, ob das Token-Budget mit der neuen Zeile überschritten würde. Maßgeblich ist deren aufgefüllte Länge, wegen der Sortierung die größte im Batch. Dann wird ein neuer Batch begonnen.
  - Eine Folge, die das Budget allein überschreitet, bildet einen eigenen Batch.
  - Die Sequenzlänge eines Batches ist die aufgefüllte Länge seiner längsten Zeile. Alle Zeilen sind gleich lang.
- Inhalt: Token-Kennungen zeilenweise als zweidimensionales Feld, am Zeilenende mit dem Auffülltoken aufgefüllt, dazu eine gleich geformte Maske mit eins für echte Token und null für Auffüllung. Die Zeilenanzahl ist abfragbar.
- Serialisierung: zuerst alle Token-Kennungen, dann die gesamte Maske, jeder Wert als vorzeichenlose 32-Bit-Ganzzahl in Little-Endian-Reihenfolge. Das entspricht einem dreidimensionalen Feld der Form zwei mal Zeilen mal Sequenzlänge.
- Der Encoder nutzt ein Token-Budget statt einer Task-Anzahl. Batchgröße ergibt sich aus Token-Budget, Längen-Bucket und Deadline je Klasse. Die Obergrenze der Batchdauer begrenzt die Anfragelatenz.

## Modell

### Modellbeschreibung und Validierung

- Ein Modell wird durch eine reine, ressourcenlose Beschreibung dargestellt:
  - Modellkennung und Architekturkennung,
  - Embedding-Dimension,
  - Pooling-Verfahren und Normalisierungsart,
  - Präfixe für Anfragen und Passagen,
  - Speicherdatentyp der Embeddings, unabhängig von den Rechendatentypen wählbar,
  - Rechendatentypen des Hauptmodells, getrennt für Zentral- und Grafikprozessor und nach Geräteklasse ermittelbar,
  - maximale Sequenzlänge,
  - optionale Projektionsschicht mit Tensorname,
  - Pfade zur Konfigurationsdatei und zu den Gewichtsdateien,
  - optionale Mehrvektor-Konfiguration.
- Vorgabe für „intfloat/multilingual-e5-small“ auf Basis eines Verzeichnisses und wählbaren Speicherdatentyps: Architektur „bert“, Dimension 384, Mittelwert-Pooling über unmaskierte Token, euklidische Normalisierung, Präfixe „query: “ und „passage: “, maximale Sequenzlänge 512, Rechendatentyp 32-Bit-Gleitkomma auf beiden Geräteklassen, keine Projektionsschicht, Konfigurationsdatei „config.json“ und Gewichtsdatei „model.safetensors“ im Verzeichnis.
- Validierung: Dimension und maximale Sequenzlänge größer null, ein nicht leeres Präfix endet mit einem Leerzeichen. Verstöße ergeben „inkompatible Modellkonfiguration“ mit Begründung. Die Validierung verhindert Konfigurationen, die die Embedding-Qualität unbemerkt verschlechtern.

### Modellregister und Laden

- Das Register bildet Architekturkennungen auf Konstruktionsfunktionen für Encoder ab. Erneute Registrierung derselben Kennung ersetzt die bisherige.
- Ein Register mit allen in den Build einkompilierten Architekturen ist erzeugbar. „bert“ ist nur bei aktivierter Backend-Unterstützung enthalten.
- Laden für eine Geräteklasse in dieser Reihenfolge: Beschreibung validieren, Konstruktionsfunktion suchen, Encoder konstruieren. Unbekannte Architekturen ergeben „nicht unterstützte Architektur“ mit Kennung.
- Ergebnis ist ein geladener Encoder aus Architektur (Code) und Gewichten (Daten) auf einem Gerät, der Beschreibung, Geräteklasse und Encoder bündelt und lesend zugänglich macht.
- Der geladene Encoder ist eine verwaltbare Ressource: Schlüssel aus Architekturkennung und Geräteklasse, Initialisierungskosten 100, Freigeben entlädt die Gewichte.

### Laden des Transformer-Hauptmodells

- Die Konfigurationsdatei wird als strukturierter Text gelesen. Unlesbarkeit ergibt einen Ladefehler mit Begründung.
- Der Modelltyp muss „bert“ lauten, sonst „inkompatibel“ mit gefundenem Typ.
- Die versteckte Dimension muss der Beschreibung entsprechen. Die maximale Sequenzlänge darf die Anzahl der Positions-Embeddings nicht überschreiten. Verstöße ergeben „inkompatibel“ mit beiden Werten.
- Auf dem Zentralprozessor ist das Modell stets ladbar. Auf dem Grafikprozessor nur bei einkompilierter Unterstützung, dann auf dem ersten Grafikgerät, sonst „inkompatibel“ mit Hinweis auf die fehlende Unterstützung.
- Der Rechendatentyp der Geräteklasse wird in den Backend-Datentyp übersetzt, sonst „inkompatibel“.
- Gewichte werden speicherabgebildet ohne vollständiges Kopieren im gewählten Datentyp auf das Gerät geladen. Ein fehlender Tensor ergibt „Gewichtsname passt nicht“ mit Name, sonstige Fehler einen Ladefehler.
- Vorwärtsberechnung: aus Token-Kennungen und Maske (Zeilen mal Sequenzlänge) die Zustandsvektoren aller Token. Tokentyp-Eingaben sind null, die Maske wirkt als Aufmerksamkeitsmaske. Ergebnis als flache Liste von 32-Bit-Gleitkommazahlen (Zeilen mal Sequenzlänge mal Dimension), unabhängig vom Rechendatentyp. Fehler ergeben „Kodierung fehlgeschlagen“. Die Ausgabedimension ist abfragbar.
- Zwischen Hauptmodell und übriger Pipeline werden keine Typen der Tensorbibliothek ausgetauscht, nur einfache Zahlenlisten.

### Aufbau des Encoders

- Aus Beschreibung und Geräteklasse entsteht ein vollständiger Encoder aus Hauptmodell, optionaler Projektionsschicht, Pooling, Normalisierung und Speicherdatentyp. Geräteklasse und Rechendatentyp werden für die Nachverfolgung hinterlegt.
- Die Normalisierungsart wird übernommen. Ohne Angabe ist die euklidische Normalisierung Standard.
- Die Projektionsschicht wird aus der ersten Gewichtsdatei geladen. Ohne Gewichtsdatei: Ladefehler „keine Gewichtsdateien“. Ihre Eingabedimension muss der Ausgabedimension des Hauptmodells entsprechen, sonst „inkompatibel“.
- Für Mehrvektor-Modelle nach Art von ColBERT, für die die externe Bibliothek kein Modul hat, wird ein vorhandener Encoder-Rumpf (BERT, ModernBERT und weitere) mit eigenem Projektionskopf verwendet.

### Projektionsschicht

- Optionale lineare Abbildung jedes Token-Vektors von Eingabe- auf Ausgabedimension, etwa von 768 auf 128, insbesondere für späte Interaktion.
- Gewichtsmatrix Ausgabe mal Eingabe, optionaler Bias der Länge Ausgabedimension. Laden aus safetensors im Speicher oder speicherabgebildet aus einer Datei, über den Tensornamen der Beschreibung.
- Der Bias-Name entsteht durch Ersetzen der Endung „weight“ durch „bias“, sonst durch Anhängen von „bias“.
- Fehlender Tensor: „Gewichtsname passt nicht“. Falsche Form: „inkompatibel“ mit tatsächlicher und erwarteter Form. Werte in 32-Bit-, 16-Bit- oder 16-Bit-Brain-Gleitkomma werden in 32 Bit umgewandelt, andere Typen ergeben „inkompatibel“ mit Tensor und Typ. Dateifehler ergeben Ladefehler.
- Anwendung: Skalarprodukt mit jeder Gewichtszeile plus Bias, Reihenfolge der Vektoren bleibt. Ein- und Ausgabedimension sind abfragbar.

### Pooling und Normalisierung

- Pooling-Verfahren: Mittelwert über unmaskierte Token (ein Vektor je Sequenz) oder kein Zusammenfassen (ein Vektor je Token).
- Mittelwert: Summe der Zustandsvektoren mit Maske ungleich null geteilt durch deren Anzahl. Auffüllung beeinflusst das Ergebnis nicht. Vollständig maskierte Sequenzen ergeben einen Nullvektor ohne Division durch null.
- Ohne Zusammenfassen werden alle Positionen unverändert weitergegeben, einschließlich Auffüllung.
- Euklidische Normalisierung: jeder Vektor wird durch seine Norm geteilt, damit das Skalarprodukt dem Kosinus entspricht. Die Norm wird nach unten auf 10⁻¹² begrenzt. Vektoren darunter gelten als degeneriert und bleiben praktisch Nullvektoren. Die Anzahl degenerierter Vektoren wird zurückgemeldet. Die Art „keine“ lässt Vektoren unverändert.
- Die euklidische Normierung wird einmal beim Schreiben am Ende des Encoders ausgeführt, nicht im Scorer. Anfrage und Dokument werden gleich behandelt.

### Schreiben in die Ausgabe-Region

- Die Ergebnisvektoren werden bei Bedarf normalisiert, in den Ausgabedatentyp umgewandelt und lückenlos in die Ausgabe-Region geschrieben.
- Zulässig sind 32-Bit-, 16-Bit- und 16-Bit-Brain-Gleitkomma in Little-Endian-Reihenfolge, andere Typen ergeben „inkompatibel“.
- Degenerierte Vektoren erzeugen eine Warnung mit Anzahl, dass leere Embeddings als Nullvektor gespeichert wurden. Der Vorgang gilt als erfolgreich.
- Schreibfehler werden als Speicherfehler weitergereicht.
- Auf dem Grafikprozessor kann ein dünner Schreib-Kernel den Gerätezeiger des Ergebnis-Tensors lesen und normiert sowie datentypkonvertiert direkt in den vom Manager vorab vergebenen Slot schreiben. Der Tensor bleibt bis zum Ereignis am Ende des Schreibens am Leben.

### Encoder-Schnittstelle

- Abfragbar sind Ausgabedimension (der Projektionsschicht, sonst des Hauptmodells) und Speicherdatentyp.
- Eingabe ist eine Region mit Layout im Serialisierungsformat der Batch-Bildung, Ausgabe ebenfalls eine Region mit Layout. Vorab wird geprüft, sonst „inkompatibel“ mit erwarteter und tatsächlicher Form beziehungsweise Datentyp:
  - Eingabe genau zwei mal Zeilen mal Sequenzlänge als vorzeichenlose 32-Bit-Ganzzahlen,
  - Ausgabe bei Mittelwert-Pooling Zeilen mal Dimension, sonst Zeilen mal Sequenzlänge mal Dimension,
  - Ausgabedatentyp gleich dem Speicherdatentyp.
- Die Kodierung läuft asynchron auf einer übergebenen Lane. Der Aufruf kehrt nach dem Einreihen zurück und liefert ein Abschlussereignis.
- Ablauf: Eingabebytes lesen und in Token-Kennungen und Maske trennen, Zustandsvektoren berechnen, Projektion anwenden, Pooling ausführen, in die Ausgabe-Region schreiben.
- Scheitern Einreihen oder Ereigniserzeugung, wird der Fehler als Geräte- oder Speicherfehler zurückgegeben. Fehler während der Ausführung werden als Warnung „Embedding-Batch fehlgeschlagen“ protokolliert und im Abschlussereignis hinterlegt. Die Lane selbst gilt als erfolgreich abgeschlossen.
- Das Ergebnis läuft über einen einmaligen Ergebniskanal je Task plus Ereignis oder Benachrichtigung. Der Batch merkt sich die Zeilen-Offsets je Task.
- Das Abschlussereignis ist nicht blockierend abfragbar und blockierend abwartbar. Das Warten liefert einen hinterlegten Verarbeitungsfehler genau einmal. Fehler der Ausführungsumgebung beim Warten werden als Geräte- oder Speicherfehler gemeldet.

### Speicherzugriff

- Lese- und Schreibzugriff auf Regionen einer Speicherdomäne laufen über eine einheitliche, nebenläufig nutzbare Schnittstelle: Bytes einer Region in einen Puffer lesen und Bytes in eine Region schreiben. Die Zentralprozessor-Speicherverwaltung stellt sie bereit.
- Eine Tensor-Referenz verbindet Region und Layout und ist der einzige Weg, auf dem Tensoren die Encoder-Schnittstelle passieren.

## Geräte, Lanes und Ereignisse

- Gerätearten sind Zentralprozessor und Grafikprozessor, erweiterbar. Das Gerätehandle besteht aus dem Geräteordinal beziehungsweise dem Zentralprozessor.
- Beim Grafikprozessor gibt es einen Prozess, einen primären Kontext und mehrere Streams.
- Ein Ausführungskontext (Lane) besteht aus Stream, Priorität, Ereignissen, eigenem Parameterblock und Arbeitsbereich.
- Beispiel-Lanes: Anfrage (hohe Priorität, kleine Batches), Einfügen und Aktualisieren im Index (Durchsatz), Kopieren zwischen Arbeits- und Grafikspeicher in beide Richtungen.
- Übergaben zwischen Lanes laufen ausschließlich über die Warteschlange.
- Ein Ereignis ist ein undurchsichtiger Abschlussmarker für alle vor seiner Aufzeichnung auf einer Lane eingereichte Arbeit:
  - Kopien beziehen sich auf dasselbe Ereignis.
  - Der Abschluss ist nicht blockierend abfragbar und blockierend abwartbar. Bei Grafikereignissen meldet das Warten Fehler als Ereignisfehler mit Ursache.
  - Ein Zentralprozessor-Ereignis bleibt nach dem Signalisieren dauerhaft abgeschlossen und weckt alle Wartenden.
  - Die Textdarstellung zeigt den Abschlussstatus.
- Der Laufzustand wird über ein Ereignis je Batch erfasst. Im persistenten Modus dient ein Fortschrittszähler in abgebildetem Hostspeicher.

## Speicherverwaltung

### Arenen

- Der Speicher wird in sechs Arenen (Speicherklassen) nach Lebensdauer eingeteilt:
  - Gewichte: langlebig, nur beim Entladen frei.
  - Index-Segmente: gestreamt, verdrängbar. Die Kapazität umfasst alle gleichzeitig residenten Generationen.
  - Arbeitsbereich je Lane: kurzlebig.
  - Bereitstellungsspeicher (Staging): seitengesperrter Arbeitsspeicher für Kopien in beide Richtungen, beim Zentralprozessor entfallend.
  - Parameterblock je Lane: kleine feste Region für dynamische Kernel-Parameter.
  - Abgebildeter Hostspeicher: seitengesperrter Arbeitsspeicher, den das Gerät über den Systembus direkt liest.
- Jede Arena hat eine stabile, eindeutige numerische Kennung, aus der sie rückgewinnbar ist. Unbekannte Kennungen ergeben „keine Arena“. Die Liste ist vollständig in fester Reihenfolge abrufbar und erweiterbar.
- Je Arena ist eine Kapazität in Byte konfigurierbar.
- Der tatsächliche Gerätebedarf ist die Summe aus Gewichten, Index-Segmenten, Arbeitsbereich und Parameterblock, sättigend statt umlaufend. Bereitstellungsspeicher und abgebildeter Hostspeicher zählen nicht, sie liegen im Hostspeicher.
- Die Reserve für andere Generationen ist die Index-Kapazität abzüglich des Anteils der aktiven Generation bei gleichmäßiger Aufteilung. Eine Generationszahl unter eins gilt als eins.
- Die Arena wird lazy beim ersten Laden nach Budget angelegt. Alle Adressen sind 64-Bit-Offsets innerhalb der Arena.
- Reservieren und Freigeben von Grafikspeicher im laufenden Betrieb ist zu vermeiden, weil es das Gerät implizit synchronisiert.
- Stream-geordnete Wiederverwendung: Eine Region wird erst frei oder überschrieben, wenn alle Streams, die sie nutzen, ihr Ereignis erreicht haben.
- Kopien laufen über einen eigenen Kopier-Stream mit seitengesperrtem Bereitstellungsspeicher, getrennt vom Rechen-Stream.

### Budget

- Das Budget besteht aus freiem Speicher, Gesamtspeicher und Generationsreserve. Der nutzbare Anteil ist der freie Speicher abzüglich Reserve, nie negativ.
- Freier und gesamter Gerätespeicher werden zur Laufzeit über eine austauschbare Abfrage ermittelt. Scheitern ergibt „Speicherabfrage fehlgeschlagen“ mit Begründung.
- Standardspielraum: 512 Mebibyte für Treiberkontext und andere Prozesse, etwa den Bildschirmkompositor.
- Die Passprüfung stellt fest, ob die geplanten Gerätearenen in den freien Speicher abzüglich des sättigend abgezogenen Spielraums passen, sonst Fehler mit angeforderter und nutzbarer Menge.
- Das Budget plant den kurzen Doppelbedarf durch Copy-on-Write ein.

### Region, Layout und Elementtypen

- Eine Region ist ein zeigerfreies Handle aus Arena-Kennung, Offset, Länge und Generation, kopierbar, vergleichbar und als Schlüssel verwendbar. Die Generation entwertet Handles nach Verdrängen und Neuladen.
- Das Auflösen einer Region auf eine Geräteadresse ist ausschließlich Sache des Speicher-Backends. Geräteadressen verlassen das Backend nicht.
- Eigene Elementtypen unabhängig von Fremdbibliotheken: 32-Bit-Gleitkomma (4 Byte), 16-Bit-Gleitkomma (2 Byte), 16-Bit-Brain-Gleitkomma (2 Byte), vorzeichenlose 8-Bit-Ganzzahl (1 Byte), vorzeichenlose 32-Bit-Ganzzahl (4 Byte), vorzeichenbehaftete 64-Bit-Ganzzahl (8 Byte).
- Elementtypen sind aus dem safetensors-Format übernehmbar, nicht unterstützte ergeben „nicht unterstützter Datentyp“ mit Quelle. Mit der Tensorbibliothek wird in beide Richtungen umgewandelt, ebenso zwischen Elementzahl (Bibliothek) und Bytezahl (Engine).
- Für additive Aufmerksamkeitsmasken gilt der feste endliche Maskenwert minus 10 000. Negativ unendlich ergäbe bei 16-Bit-Gleitkomma ungültige Zeilen.
- Datentypen gelten je Gerät: Standard auf dem Grafikprozessor 16-Bit-, auf dem Zentralprozessor 32-Bit-Gleitkomma. Der Datentyp eines Index ist je Index fest.
- Ein Layout besteht aus Form, Schrittweiten in Byte und Elementtyp. Sichten ändern nur das Layout, nie die Daten:
  - Zusammenhängendes Layout: letzte Dimension mit Elementgröße als Schrittweite, jede vordere als Produkt der hinteren Dimensionen. Dimensionen der Größe null zählen als eins.
  - Abfragbar: Elementzahl, Bytelänge, Zusammenhängigkeit.
  - Vertauschen zweier Dimensionen tauscht Form und Schrittweiten. Verkürzen ändert nur die Form der Dimension.

### Freispeicherverwaltung

- Je Arena vergibt ein Offset-Allokator Bereiche aus fester Kapazität, ohne selbst Speicher zu halten, nach dem First-Fit-Prinzip in aufsteigender Adressreihenfolge.
- Beim Freigeben verschmelzen Lücken mit angrenzenden freien Nachbarn auf beiden Seiten.
- Ausrichtung rundet den Offset auf, null gilt als eins. Überläufe führen zu „passt nicht“, nie zum Absturz. Reste vorn und hinten bleiben freie Lücken.
- Anforderungen der Länge null werden nicht erfüllt. Kapazität und freie Menge sind abfragbar. Bei Kapazität null gibt es keine Lücke.

### Buchführung und Gültigkeit

- Je Arena verfolgt ein nebenläufig sicherer Buchführungs-Allokator Offsets und Generationen, ohne Gerätespeicher zu besitzen.
- Erstmals vergebene Kombinationen aus Offset und Länge erhalten Generation null. Erneute Vergabe derselben Kombination erhöht die Generation mit Umlauf.
- Eine Region ist lebendig, wenn Arena, Offset, Länge und Generation einer aktuellen, nicht freigegebenen Vergabe entsprechen.
- Freigabe scheitert bei fremder Arena, unbekannter, bereits freigegebener oder abweichender Region. Das allgemeine Verwaltungs-Interface dereferenziert veraltete Handles nie, sondern ignoriert sie mit Warnung.
- Platzmangel nennt angeforderte und freie Menge. Für Tests lässt sich ein Vergabefehler je Arena künstlich auslösen, gemeldet wie Erschöpfung.
- Vergaben und Freigaben werden auf Debug-Ebene mit Arena, Offset und Größe protokolliert.

### Allokator je Arena mit verzögerter Freigabe

- Jeder Allokator kennt Kapazität, Arena und Geräteart. Vor jeder Vergabe sammelt er abgeschlossene verzögerte Freigaben ein.
- Erschöpfung ergibt einen Fehler mit Arena, angeforderter und freier Menge, andere Fehler „Vergabe fehlgeschlagen“.
- Sofortige Freigabe wird bei vorgemerkten oder ungültigen Regionen mit „veraltete Region“ abgelehnt.
- Verzögerte Freigabe nimmt Region und Ereignis. Die Region kehrt erst nach Abschluss des Ereignisses zurück, sodass laufende Arbeit nie wiederverwendeten Speicher sieht. Abgelehnt bei nicht lebendigen oder bereits vorgemerkten Regionen. Nach dem Vormerken wird sofort auf Abschluss geprüft.
- Das Einsammeln gibt abgeschlossene Regionen frei und liefert die Anzahl. Ungültige Regionen erzeugen nur eine Warnung.
- Die sofort verfügbare Menge schließt wartende Regionen aus. Anzahl wartender Regionen und Lebendigkeit sind abfragbar. Alles ist nebenläufig sicher.
- Eine Menge von Allokatoren stellt je Arena genau einen bereit, liefert die Größenvorgabe und findet den zuständigen Allokator zu einer Region. Fremde Kennungen ergeben „veraltete Region“.
- Jedes Speicher-Backend erfüllt dieselbe nebenläufige Schnittstelle: Vergabe in einer Arena mit Größe und Ausrichtung, Freigabe, Freigabe nach Ereignis, Budgetabfrage.

### Zentralprozessor-Backend

- Je Arena ein mit Nullen gefüllter Bereich im Arbeitsspeicher, ohne Streams und Bereitstellung.
- Schreiben, Lesen und Kopieren auf Regionsbasis mit Prüfung auf Lebendigkeit und Länge, sonst „veraltete Region“. Beim Kopieren gilt die Quelllänge, innerhalb einer Arena überlappungssicher, zwischen Arenen über einen Zwischenpuffer.
- Budget: Summe der verfügbaren Mengen als frei, Summe der Kapazitäten als gesamt, plus Generationsreserve.

### Grafikprozessor-Backend

- Nur mit einem Nvidia-Gerät anlegbar, sonst „Gerätespeicher nicht verfügbar“.
- Beim Anlegen zuerst Speicherabfrage und Passprüfung mit Spielraum. Bei Misserfolg wird nichts reserviert.
- Freigegebener, stream-geordnet vergebener Speicher bleibt im Standard-Pool des Treibers, damit im Dauerbetrieb keine Treiber-Allokation mehr stattfindet.
- Je Gerätearena ein zusammenhängender, genullter Block in voller Kapazität. Arenen der Größe null erhalten keinen Block. Danach wird auf Abschluss aller Vorgänge gewartet.
- Für abgebildeten Hostspeicher größer null ein seitengesperrter, genullter Block mit eigener Geräteadresse. Scheitert die Abbildung, wird er freigegeben. Beim Beenden wird er freigegeben, sofern der Gerätekontext noch bindbar ist.
- Die Allokatormenge setzt die Bereitstellungskapazität auf null, da diese über den Ring verwaltet wird.
- Nach dem Anlegen werden freier und gesamter Speicher sowie die Arenagrößen in Mebibyte protokolliert.
- Kopieren und Übertragen:
  - Gerät zu Gerät zwischen gültigen Regionen verschiedener Arenen auf einem Stream. Gleiche Arena wird mit Übertragungsfehler abgelehnt, fehlende Blöcke mit „veraltete Region“. Sperren in fester Reihenfolge verhindern Verklemmungen.
  - Asynchrones Hochladen aus seitengesperrtem Puffer, der bis zum danach aufgezeichneten Ereignis unverändert bleibt. Asynchrones Herunterladen, lesbar erst nach dem Ereignis. Region lebendig, Datenmenge passend.
  - Treiberfehler: „Gerätespeicher nicht verfügbar“ oder Übertragungsfehler mit Richtung und Begründung.
- Abgebildeter Hostspeicher ist ohne Übertragung beschreib- und lesbar. Voraussetzung sind existierender Block, passende Arena und lebendige Region. Offset und Länge werden überlaufsicher gegen Region und Block geprüft. Die Geräteadresse ist für Kernel-Starts bereitstellbar.
- Ein Ring aus seitengesperrten, genullten Hostpuffern mit Anzahl und Größe ist auf Anforderung erzeugbar.
- Das Budget wird bei jedem Aufruf frisch vom Treiber ermittelt und auf Debug-Ebene protokolliert.

### Bereitstellungsring

- Feste Anzahl von Plätzen mit Nutzlast im Zustand frei, belegt oder in Übertragung mit Ereignis. Wiederverwendung erst nach Abschluss des Ereignisses.
- Anfordern reihum hinter dem zuletzt vergebenen Platz, sonst nichts. Blockierendes Anfordern wartet auf die älteste laufende Übertragung. Sind alle Plätze belegt ohne Übertragung: „Bereitstellung erschöpft“. Scheitert das Warten: Übertragungsfehler Richtung Host zum Gerät.
- Nutzlastzugriff und die Übergänge „in Übertragung“ und „freigeben“ nur aus „belegt“, sonst „ungültiger Platz“. Anzahl und Plätze in Übertragung sind abfragbar. Nebenläufig sicher.

### Einbindung von Regionen in Tensoren

- Ein Teilbereich eines Geräteblocks ist in einen Tensor der Tensorbibliothek einhüllbar, sodass sie den korrekten Bereich liest.
- Der eingehüllte Tensor besitzt den Speicher nicht und wird bewusst nicht freigegeben, damit Kontext und Block intakt bleiben.
- Kopien erhalten eigenen Speicher außerhalb des Blocks. Schreiben schlägt auf den Teilbereich durch, Nachbarelemente bleiben unverändert.
- Diese Prüfungen laufen nur auf einem echten Nvidia-Gerät.

### Übergreifende Eigenschaften

- Alle gemeinsam genutzten Strukturen sind nebenläufig nutzbar.
- Allokatoren, Ereignisabhängigkeiten und Bereitstellung sind ohne Grafikprozessor testbar, etwa mit einem künstlich steuerbaren Ereignis.

## Index

### Grundsätze

- Ein Index kann eine Struktur kapseln, eine speicherabgebildete Datei sein oder intern Bytepuffer oder Spaltenformate nutzen.
- Jeder Index implementiert sein eigenes Modell, etwa ganze Dateien, Gleitfenster, Syntaxbaum-Knoten, Chunks, kaskadierende oder fraktale Strukturen. Der Kern kennt diese Modelle nicht.
- Darstellung im Arbeitsspeicher und persistente Darstellung sind dieselbe Instanz. Der Grafikspeicher hält nur einen abgeleiteten, verwerfbaren Spiegel. Der Index ist die Quelle der Wahrheit. Die Richtungen zwischen Arbeitsspeicher, Index und Grafikspeicher sind agnostisch.
- Der Index kennt weder Subengine noch Profil noch Kernel, kennt keine Geometrie und löst keine Scopes auf. Er meldet Änderungen und liefert Struktur und Änderungsstempel. Er ist über eine gemeinsame Schnittstelle von mehreren Threads gleichzeitig nutzbar.
- Vektoren verschiedener Modelle sind nicht vergleichbar. Ein Metadatum „erzeugendes Profil“ ist höchstens optionale Diagnose.
- Ein Index wird beim ersten Zugriff lazy geladen.
- Ein Eintrag besteht aus Kennung, Änderungsstempel und Nutzlast. Eintragskennungen sind 64-Bit-Werte und vollständig geordnet. Knoten und Zellen haben eigene Kennungstypen.

### Änderungsstempel

- Jede Schreib- und Löschoperation erhält einen neuen Stempel, eindeutig innerhalb des Index, auch bei gleichzeitigen Zugriffen.
- Der Stempel ist ein undurchsichtiger Veränderungsmarker. Konsumenten vergleichen nur auf Gleichheit, nie auf Ordnung.
- Er wird im exklusiven Schreibabschnitt unmittelbar vor der Veröffentlichung vergeben.
- Auch das Entfernen verbraucht einen Stempel. Eine später neu angelegte gleiche Kennung erhält daher einen anderen Stempel.
- Ein neuer Index beginnt bei eins. Ein wiederaufgenommener setzt oberhalb des Größeren aus gespeichertem Höchststempel und höchstem tatsächlich vorkommendem Stempel fort.
- Höchststempel (null bei leerem Zähler) und Rohdarstellung für Persistenz und Benachrichtigung sind abfragbar.

### Allgemeiner Vertrag

- Stempel eines Eintrags: aktueller Stempel oder „nicht vorhanden“.
- Alle Stempel: Momentaufnahme aller Einträge samt Stempel, ohne Vektordaten zu berühren.
- Lesen eines Eintrags: Vektorbereich oder leere Daten bei Fehlen. Darf blockieren. Daten geliehen oder gemeinsam referenziert, ein gelesener Wert ändert sich durch spätere Schreibvorgänge nicht.
- Schreiben ersetzt die Daten vollständig. Mehrere Schreiber werden serialisiert, der zuletzt drankommende gewinnt. Verbindliche Reihenfolge: Daten, Stempel, Signal. Rückgabe ist der Stempel genau dieses Schreibvorgangs.
- Änderungsbenachrichtigung als Pegelsignal mit dem aktuellen Höchststempel. Beliebig viele Beobachter, Anfangswert ist der aktuelle Höchststempel. Jeder Schreib- und Löschvorgang aktualisiert das Signal. Zusammengefasste Signale sind unschädlich.
- Ein Index deklariert, welche Zugriffsmethoden er unterstützt (Zentroid, Präfix, Teilbaum, Bereich) und liefert die zugehörigen Strukturen, etwa Zusammenfassungsstruktur, Kinder eines Knotens oder Byte-Bereiche zusammenhängend gespeicherter Einträge.

### Scopes und Eintragsmengen

- Scope-Arten mit stabiler Bezeichnung für Protokolle und Kennzahlen: alle Einträge („all“), Kennungsbereich („range“), explizite Menge („set“), Teilbaum ab Knoten („subtree“), Byte-Präfix („prefix“), Zentroid-Umgebung mit Anzahl und optionalem Radius („centroid“). Neue Arten sind ergänzbar.
- Eintragsmengen sind kompakt als Bereiche oder Bitmap gespeichert, nicht als Kennungsliste, und unterstützen 64-Bit-Kennungen.
- Aufbau aus Bereichen (leere oder umgekehrte werden ignoriert) oder einzelnen Kennungen.
- Operationen: Leere, Anzahl, Enthaltensein, aufsteigende Iteration, Vereinigung, Schnitt, Gleichheit, gleichheitsverträglicher Hashwert, Ausgabe als maximale zusammenhängende aufsteigende Folgen (bei großen Mengen teuer).

### Referenzindex im Arbeitsspeicher

- Ein Referenzindex im Arbeitsspeicher ist ohne Grafikbeschleunigung und ohne Bibliotheken für maschinelles Lernen lauffähig.

### Dateigestützter Index

- **Öffnen** mit Dateipfad, erwartetem Indexkopf und Optionen:
  - Fehlt die Datei, entsteht ein leerer Index. Die Datei entsteht erst beim ersten Sichern.
  - Zugriff per Speicherabbildung oder Dateilesen. Bei Speicherabbildung wird, wo möglich, wahlfreier Zugriff angekündigt.
  - Der Kopf im Dateifuß muss vollständig und lesbar sein. Fehlende Schlüssel, nicht numerische Werte und unbekannte Aufzählungswerte ergeben Schemainkompatibilität mit Schlüssel, erwartetem und gefundenem Wert.
  - Der Kopf muss dem erwarteten entsprechen. Alle Abweichungen werden gesammelt gemeldet.
  - Die Spaltenstruktur muss in Name, Datentyp und Nullbarkeit entsprechen, Spaltenmetadaten werden ignoriert. Abweichungen nennen beide Strukturen.
  - Über die Kennungs- und Stempelspalten werden alle Einträge ohne Vektordekodierung eingelesen. Doppelte Kennungen ergeben „Index nicht verfügbar“.
  - Der Stempelzähler wird fortgesetzt wie beschrieben.
  - Optional (Standard ein) werden alle Daten dekodiert und Prüfsummen kontrolliert. Ein Fehler bricht das Öffnen ab.
  - Protokolliert werden Pfad, Schemaversion, Modellkennung, Modellrevision, Zugriffsart, Anzahl Zeilengruppen und Prüfsummenergebnis.
  - Datei- oder Lesefehler ergeben „Index nicht verfügbar“ mit Pfad und Ursache.
- **Verzögertes Laden:** Vektoren und Nutzdaten werden beim ersten Gebrauch genau einmal dekodiert und gehalten, in Batches von 4096 Zeilen, als 16-Bit-Gleitkommawerte in Little-Endian-Reihenfolge. Ein Fehlschlag wird gemerkt und bei jedem Zugriff gemeldet: beim Vektorlesen leere Daten, bei Nutzdaten „nicht vorhanden“, jeweils mit Fehlerprotokoll.
- **Optionen:** Zugriffsart „mmap“ (Standard, Speicherabbildung) oder „file“ (Dateilesen), Prüfung beim Öffnen (Standard ein), Zeilen je Zeilengruppe beim Schreiben (Standard 8192, Werte unter eins gelten als eins).
- **Einträge verwalten:**
  - Einfügen oder Ersetzen mit Kennung, Vektor und Nutzdaten. Der Vektor muss genau Dimension mal zwei Bytes lang sein, sonst Schemafehler mit erwarteter und gefundener Länge. Prüfsumme berechnen, Stempel vergeben, Index als ungesichert markieren, Signal auslösen, protokollieren. Die Änderung liegt bis zum Sichern nur im Speicher.
  - Schreiben über den allgemeinen Vertrag: genau ein Vektor, Nutzdaten eines vorhandenen Eintrags bleiben erhalten, bei neuen sind sie leer. Falsche Länge ist ein nicht behebbarer Programmfehler.
  - Entfernen: entfernen, als ungesichert markieren, Stempel vergeben, Signal auslösen. Bei Fehlen ändert sich nichts und es wird „nicht vorhanden“ gemeldet.
  - Abfragbar: Nutzdaten, Pfad, Kopf, Höchststempel, Status ungesicherter Änderungen. Eine ausdrückliche Prüfung dekodiert vollständig, vergleicht jede Prüfsumme und nennt beim ersten Fehler die Eintragskennung. Ein Index ohne Datei gilt als geprüft.
- **Dauerhaftes Sichern:**
  - Gleichzeitige Sicherungen laufen hintereinander. Ohne Änderung wird nichts geschrieben und „keine Änderung“ gemeldet, sonst „geschrieben“.
  - Geschrieben wird eine konsistente Momentaufnahme. Während des Schreibens eintreffende Änderungen bleiben als ungesichert vermerkt.
  - Die Datei enthält alle aktuellen Einträge aufsteigend nach Kennung, aus Ursprungsdatei und Speicher, mit unveränderten Stempeln, Nutzdaten und Prüfsummen. Der Kopf trägt den aktuellen Höchststempel.
  - Atomares Schreiben: Hilfsdatei neben dem Ziel, vollständig auf den Datenträger sichern, durch Umbenennen ersetzen, übergeordnetes Verzeichnis sichern. Bei Fehlern wird die Hilfsdatei gelöscht, die bisherige Datei bleibt unangetastet, gemeldet als „Index nicht verfügbar“.
  - Zeilengruppen der konfigurierten Größe, keine Kompression, Statistiken je Seite, keine Wörterbuchkodierung der Vektorspalte, Kopf als Schlüssel-Wert-Paare im Dateifuß.
  - Protokolliert werden Pfad, Schemaversion, Modell, Zeilenzahl und Zeilengruppenzahl.

### Dateiformat und Modellbindung

- Spalten in dieser Reihenfolge, keine nullbar:
  - Eintragskennung (vorzeichenlose 64-Bit-Ganzzahl),
  - Änderungsstempel (vorzeichenlose 64-Bit-Ganzzahl), eigene Spalte, damit das Einlesen der Stempel keine Vektoren lädt,
  - Vektor (Liste fester Länge gemäß Dimension aus 16-Bit-Gleitkommawerten),
  - Zieldatei (Text),
  - Zeichenanfang und Zeichenende (je vorzeichenlose 64-Bit-Ganzzahl),
  - Prüfsumme (vorzeichenlose 64-Bit-Ganzzahl).
- Der Kopf enthält Schemaversion (aktuell eins), Höchststempel, Hashalgorithmus und Hashversion, Modellkennung und Modellrevision, Dimension, Pooling, Normalisierung, Anfrage- und Passagenpräfix, Speicherdatentyp, Rechendatentyp, maximale Sequenzlänge und Fingerabdruck des Tokenizers.
- Ableitung aus einer Modellbeschreibung: Speicherdatentyp ausschließlich 16-Bit-Gleitkomma, sonst Schemainkompatibilität. Hashalgorithmus schneller 64-Bit-Hash, Hashversion eins, Höchststempel null. Rechendatentyp als Text für beide Prozessoren. Dimension und Vektorgröße (Dimension mal zwei) sind ableitbar, der Kopf ist mit anderem Höchststempel kopierbar.
- Die Bindung gilt nur bei Gleichheit von Schemaversion, Hashalgorithmus, Hashversion, Modellkennung, Modellrevision, Dimension, Pooling, Normalisierung, beiden Präfixen, Speicherdatentyp, maximaler Sequenzlänge und Tokenizer-Fingerabdruck. Höchststempel und Rechendatentyp sind informativ, weil gemischtes Indizieren auf beiden Prozessoren erlaubt ist.
- Der Kopf ist verlustfrei als Schlüssel-Wert-Paare les- und schreibbar. Aufzählungswerte: Pooling (gemaskiertes Mittel, je Token), Normalisierung (euklidisch, keine), Datentypen (wie Elementtypen), Hashalgorithmus (schnell 64 Bit, schnell 128 Bit, kryptografisch).
- Der Tokenizer-Fingerabdruck besteht aus Algorithmusbezeichnung, Doppelpunkt und dem 64-Bit-Hash der Tokenizer-Definition in 16 hexadezimalen Stellen mit führenden Nullen.

### Nutzdaten und Prüfsumme

- Jeder Eintrag verweist auf den Quelltext: Zieldatei, Zeichenanfang, Zeichenende. Standard ist leere Datei mit null und null.
- Die 64-Bit-Prüfsumme wird aus Vektorlänge, Vektorbytes, Dateinamenlänge, Dateiname, Zeichenanfang und Zeichenende berechnet. Vorangestellte Längen schließen mehrdeutige Aneinanderreihungen aus. Jede Bitänderung ist erkennbar.
- Sie wird beim Speichern berechnet und bei der Prüfung neu berechnet und verglichen.

### Erkennung von Dateiänderungen

- Änderungen an Quelldateien werden über eine Rückfallkette erkannt: Änderungsstempel, dann Größe und Zeitstempel, zuletzt Inhaltshash.
- Je Datei werden Größe, Zeit der letzten Inhaltsänderung, optional Zeit der letzten Metadatenänderung und der Referenzzeitpunkt des Durchlaufs festgehalten.
- Eine Aufzeichnung ist unsicher, wenn ihre Änderungszeit höchstens zwei Sekunden vor dem Referenzzeitpunkt oder danach liegt. Das deckt grobe Zeitstempelauflösung mancher Dateisysteme ab.
- Ergebnis „unverändert“ nur, wenn Größe und beide Zeiten übereinstimmen und die Aufzeichnung sicher ist, sonst „Inhaltsprüfung nötig“ mit Diagnoseprotokoll.
- Für die Inhaltsprüfung dient ein schneller, nicht kryptografischer 64-Bit-Hash, dessen Algorithmus und Version im Indexkopf stehen.

## Manager

### Rolle und Schichten

- **Index:** kennt Einträge, Vektordaten und seine Traversalstruktur.
- **Manager:** kennt die Zuordnung von Eintrag zu Segment und Slot, die Residenz und die Update-Warteschlange. Verantwortlich für Hochladen, Verdrängen, Aktualisieren und Rückabbildung von Adressen auf Einträge.
- **Kernel:** kennt nur Segmente, Adressen und Offsets, keine Eintragsidentität und keine Traversierung.
- Der Manager ist eine Komponente der jeweiligen Subengine, weder der Fassade noch des Index. Er wird mit einer Subengine und einem lazy aktivierten Index konstruiert.

### Gemeinsames Verhalten

- Ein Manager hängt seinen Index erst bei Bedarf an und hält bis dahin weder Ressourcen noch Abonnements.
- Das Anhängen liefert „wahr“ bei Annahme. Ist bereits ein Index angehängt, bleibt dieser bestehen, es wird „falsch“ geliefert und gewarnt.
- Die Aktivierung mit Initialisierungsfunktion initialisiert nur beim ersten Gebrauch. Gleichzeitige Aufrufer warten auf dieselbe Initialisierung. Bei Fehlschlag bleibt der Manager inaktiv.
- Zugriff ohne angehängten Index ergibt „Index nicht verfügbar“ mit Indexkennung.
- Abfragbar über dieselbe Schnittstelle: Aktivität, Zahl residenter Einträge. Zyklen laufen in einem Diagnosebereich mit Indexkennung.

### Zentralprozessor-Variante

- Lesezugriffe gehen direkt an den Index, da der Arbeitsspeicher die Primärinstanz ist. Aufgabe ist die Residenz dateigestützter Daten im Seitencache, relevant bei speicherabgebildeten Indizes größer als der Arbeitsspeicher, sowie das Entladen nach Leerlauf.
- Einzelnes Lesen: Hinweisbereich setzen („wird bald benötigt“), im Pool für blockierende Arbeit lesen, Kopie der Bytes liefern. Abbruch der Aufgabe ergibt Übertragungsfehler „Lesetask abgebrochen“.
- Mehrfaches Lesen mit Zugriffsmuster: bei dichtem Zugriff „sequentiell“ über den umspannenden Bereich, bei verstreutem „wahlfrei“ je Eintrag. Ergebnisse in Eingabereihenfolge. Leere Eingabe liefert leeres Ergebnis, der Index muss angehängt sein.
- Eine Variante erfragt die Hinweisbereiche selbst vom Index. Einträge ohne zusammenhängende Speicherung werden ohne Hinweis gelesen.
- Bereiche lassen sich nach Leerlauf als „kalt“ markieren. Ohne Hinweisquelle passiert nichts, Ablehnungen werden nur protokolliert.

### Seitencache-Hinweise und blockierende Lesezugriffe

- Empfehlungen an das Betriebssystem je Byte-Bereich: bald benötigt (Vorabladen), sequentiell, wahlfrei, kalt. „Kalt“ senkt die Rückgewinnungspriorität, ohne Inhalte zu verwerfen. Alle Empfehlungen sind unverbindlich.
- Das Hinweisziel ist über eine austauschbare, nebenläufige Schnittstelle abstrahiert.
- Ungültige Bereiche (Anfang hinter Ende, Ende hinter Abbildungslänge) werden abgewiesen. Leere Bereiche sind wirkungslos erfolgreich.
- „Kalt“ wird auf Seitengrenzen ausgerichtet (Anfang abrunden, Länge vergrößern) und ist auf Plattformen ohne Unterstützung wirkungslos erfolgreich. Betriebssystemfehler werden weitergereicht.
- Blockierende Lesezugriffe (Seitenfehler, Plattenzugriffe) laufen im Pool für blockierende Arbeit, nie im Thread der asynchronen Laufzeitumgebung. Der Vorabladehinweis sorgt dafür, dass Ein- und Ausgabe und Kernel-Lauf überlappen.
- Vor dem Lesen wird standardmäßig „wird bald benötigt“ gesetzt, abweichend etwa „sequentiell“ für dichte Durchläufe. Lesen ohne Hinweis ist möglich, Vorabladen ist eine eigene Funktion. Ohne Hinweisquelle bleibt der Hinweis wirkungslos, ein abgelehnter Hinweis wird nur protokolliert. Abbruch oder Absturz der blockierenden Aufgabe wird als Fehler gemeldet.

### Grafikprozessor-Variante

- Scopes werden über die Resolver-Registrierung aufgelöst.
- Sicherstellen einer Menge: angehängter Index, Laden über die Ladeeinrichtung, danach alle Segmente als benutzt markieren, bewegte Bytes protokollieren.
- Abgleichdurchlauf: Abgleich beim ersten Mal anlegen, residente Einträge mit dem Index vergleichen, Entfernte als Lösch-Updates einstellen, Geänderte ohne ausstehendes Update regulär nachladen, Ergebnis zurückgeben.
- Veröffentlichen eines geladenen Segments:
  - fortlaufende Segmentkennung vergeben,
  - Region in eine neue Tabellenversion und Segmentzuordnung in eine neue Zuordnungsepoche aufnehmen; weist die Zuordnung ab (etwa Überlappung), Übertragungsfehler,
  - zuerst Tabelle, dann Zuordnung veröffentlichen,
  - bei Fehlern die Region zurückgeben,
  - bei Erfolg das Segment verfolgen (Größe, Stufe Vektor, Wiederladekosten gleich Größe, Zeitpunkt jetzt) und die mitgelieferten Bindungen mit Segmentkennung und Stempel als Updates einstellen, wirksam in der nächsten Lücke.
- Abfragbar: Segmentzahl, residente Bytes aus Verdrängungssicht, ausstehende Updates.
- Freigeben eines Segments: Region zurückgeben, aus der Verdrängung entfernen, Residenz aller Einträge löschen. Noch anstehende Bindungen in dieses Segment scheitern schadlos.
- Verdrängungsschritt und Platzbeschaffung sind verfügbar, jede Verdrängung wird mit Index, Segment, Bytes und Grund protokolliert.
- Der Abschluss eines Kommunikationsslots verlangt einen angehängten Index, sperrt Updates und Residenz und läuft gegen die Zuordnung der Batch-Epoche.

### Scope-Auflösung

- Der Manager löst den Scope einer Operation in eine kompakte Eintragsmenge auf, nicht der Index.
- „Alle“ liefert alle Einträge, eine explizite Menge wird übernommen.
- Andere Arten brauchen eine Zugriffsmethode: Der Index deklariert das Verfahren und der Manager hat einen Resolver dafür. Sonst „Scope nicht unterstützt“ mit Methodenname. Ein stilles Ausweichen auf alle Einträge gibt es nicht.
- Standard-Resolver für Bereich, Teilbaum, Präfix und Zentroid. Doppelte Registrierung wird abgewiesen.
  - Bereich: genau der angegebene Bereich.
  - Teilbaum: die Kinder des angegebenen Knotens.
  - Präfix: führende Bytes der 64-Bit-Kennung in Big-Endian-Reihenfolge als zusammenhängender Bereich. Über acht Bytes trifft nichts, leeres Präfix deckt alles ab, Obergrenze begrenzt auf den größten Wert.
  - Zentroid: braucht die Zusammenfassungsstruktur des Index (sonst „Index nicht verfügbar“) und einen Zellenwähler im Kontext (sonst „nicht unterstützt“). Der Wähler wählt die nächsten Zellen mit Anzahl und optionalem Radius. Höchstens die angegebene Anzahl Zellen, Ergebnis ist die Vereinigung ihrer Mitglieder.
- Ein falscher Scope für einen Resolver ergibt „Scope nicht unterstützt“. Die Auflösung läuft in einem Diagnosebereich mit Methode und protokolliert die Anzahl.
- Ablauf am Beispiel Zentroid: Der Manager macht die Zusammenfassungsstruktur resident, Subengine und Kernel rechnen die Anfrage gegen die Zentroide, danach fragt der Manager die Mitglieder der gewählten Zellen ab und gleicht gegen den Ladezustand ab (fehlt etwas, ist etwas veraltet).
- Ist die Kandidatenmenge aus einer Vorverarbeitung bekannt (mehrstufiges Zentroid-Verfahren nach Art von PLAID, Beschneiden, Präfix, Fourier- oder Kompressionsdomäne), lädt der Manager nur diese Bereiche.

### Konsistenzmodell

- Der Index gewinnt bei Abweichung. Die Identität eines Eintrags ist das Paar aus Kennung und Stempel. Stempel werden nur auf Ungleichheit verglichen.
- Der Abgleich ist zustandsbasiert: Das Signal weckt nur, verglichen werden residente Einträge. Änderungen an nicht geladenen Einträgen brauchen keine Behandlung, weil das Laden ohnehin den aktuellen Stand liest. Der Manager-Zustand skaliert mit dem geladenen Teil.
- Segmenttabellen werden kopierend verändert und per Tausch des Wurzelzeigers veröffentlicht. Abgelöste Versionen werden erst frei, wenn alle Leser die neue Epoche erreicht haben. Zuordnungsänderungen folgen der Tabelle in der Lücke zwischen Batches.
- Der letzte Schreiber gewinnt. Änderungen am Index werden serialisiert angewendet.

### Zustandsbasierter Abgleich und Echo-Vermeidung

- Jeder Abonnent (Grafikprozessor-Manager, Persistenz-Zeitgeber, Zentralprozessor-Manager) hat seinen eigenen Abgleich mit eigener Sicht.
- Der Abgleich wartet auf ein ungesehenes Signal und meldet „falsch“, wenn der Index nicht mehr existiert. Ein anstehendes Signal ist abfragbar.
- Das Signal wird vor dem Vergleich quittiert. Ein währenddessen eintreffendes Signal löst den nächsten Abgleich aus.
- Vergleich residenter Einträge: fehlt im Index, gilt als entfernt; abweichender Stempel, zum Nachladen vorgemerkt; gleicher Stempel, aktuell, einschließlich Echos eigener Schreibvorgänge.
- Ergebnis: beide Listen aufsteigend sortiert, Zahl aktueller Einträge, Wasserstand des Signals, Aussage „sauber“. Entfernungen sind als Lösch-Updates einstellbar.
- Laden eines Eintrags in der Reihenfolge Stempel, Daten, erneuter Stempel. Bewegt sich der Stempel, wird bis zu achtmal wiederholt, danach mit dem zuerst gelesenen Stempel zurückgekehrt. Ergebnis: erster Stempel, Daten, Wiederholungen, bei Fehlen nichts. Die umgekehrte Reihenfolge würde veraltete Daten als aktuell ausweisen und ist die einzige unsichere Richtung. Ein überzähliges Nachladen ist harmlos.
- Echo-Unterdrückung: Der Manager trägt den Stempel aus dem Rückgabewert seiner eigenen Übernahme als bekannt ein, nie aus einem späteren Lesen. Ohne Rückgabewert entsteht höchstens ein redundanter Upload.
- Zwei Subengines, derselbe Eintrag: Der andere Manager sieht einen abweichenden Stempel und lädt nach. Beide konvergieren auf den Indexstand.
- Entfernen und Neuanlage werden über die Gesamtliste der Stempel beziehungsweise die Stempelabfrage je Eintrag erkannt.

### Residenzverzeichnis

- Je residentem Eintrag werden Slot (Segment und Slotnummer) und bekannter Stempel geführt.
- Einfügen mit Rückgabe des Vorgängers, Entfernen, Abfrage von Eintrag oder Stempel, Anzahl, Leere, Durchlaufen.

### Kernel-Schnittstelle und Layouts

- Der Kernel liefert nur Adresse beziehungsweise Offset und Bewertung. Die Adressdarstellung ist Implementierungsdetail. Während der Berechnung gibt es keine Kommunikation.
- Festes Layout mit Schrittweite: Eintragsgrenzen per Arithmetik. Variables Layout: Offset-Tabelle aus Präfixsummen, vom Manager aus der Indexstruktur gebaut.
- Der Kernel traversiert nie. Die Maximalähnlichkeit aggregiert je Eintrag über Schrittweite oder Tabelle, ohne Eintragsidentität.
- Eine Kandidatenmenge ist ein variables Layout, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.
- Rückabbildung durch den Manager: bei festem Layout per Arithmetik, bei variablem per Binärsuche oder Intervallabbildung auf Arena-Offsets.

### Segmente und Slots

- Slots haben feste Größe aus Token-Obergrenze, Dimension und Datentypgröße. Vordefinierte Slot-Klassen für 32, 64 und 128 Token, frei wählbare bis 65 535 Token, damit der Kernel ohne Nachschlagen auskommt. Überlauf oder ein Ergebnis über 32 Bit ergibt einen Fehler.
- Eine Segmentansicht besteht aus Region, Slot-Anzahl, Slot-Klasse, Dimension, Datentyp und Segmentgeneration. Beim Anlegen wird geprüft, ob die Region alle Slots fasst, sonst Fehler mit benötigten und verfügbaren Bytes. Gesamtgröße und Teilbereich je Slot sind abfragbar, ungültige Slotnummern ergeben Fehler mit Nummer und Anzahl. Für Kernel gibt es eine Beschreibung mit Startadresse, Slot-Anzahl und festem Abstand.
- Kodierung von Slot-Inhalten: echte Token-Zeilen in Eingabereihenfolge, danach Füllzeilen bis zur Klassengrenze mit endlichem Maskenwert statt null, damit Auffüllung nie eine Ähnlichkeit von genau null beiträgt. Datentypen 32-Bit-, 16-Bit- und 16-Bit-Brain-Gleitkomma in Little-Endian-Reihenfolge, andere werden mit Typnennung abgelehnt. Zu viele Token oder eine Wertezahl ungleich Token mal Dimension werden abgelehnt. Das Ergebnis hat volle Slot-Größe.
- Je Slot gibt es gültige Tokenzahl und Gültigkeitsbit (Löschmarkierung, noch nicht befüllt).
- Je Segment eine nebenläufig les- und schreibbare Zuordnung von Slot zu Eintragsverweis (Eintragskennung plus Abschnittsnummer), mit fester oder variabler Adressierung. Binden, Lösen, Abfragen, Slot zu Geräteadresse, absoluter Byte-Bereich je Slot und umspannender Bereich (leer ohne Ergebnis).
- Segmente wachsen nur durch Anhängen. Belegte Slots eines geladenen Segments werden nie überschrieben. Speicher gelöschter Einträge wird durch Neuaufbau des Segments zurückgewonnen.
- Schreiben:
  - Reservierung des nächsten freien Slots mit Eintragsverweis, Tokenzahl, optionaler Zentroid-Zuordnung und Zielbereich. Der Slot bleibt bis zur Bestätigung unsichtbar.
  - Abgelehnt bei Tokenzahl über Klassengrenze oder 16-Bit-Bereich sowie ohne freien Slot.
  - Bestätigung sofort, wenn die Übertragung abgeschlossen ist (sonst „noch nicht bereit“), oder zurückgestellt bis zum Ereignis. Die Abwicklung wendet zurückgestellte Bestätigungen in Reihenfolge an, hält beim ersten offenen Vorgang an und liefert die Zahl neu sichtbarer Slots.
  - Bestätigen bindet den Slot, setzt Metadaten (gültig, Tokenzahl, fortlaufende Schreibnummer, Zentroid-Zuordnung) und das Gültigkeitsbit erst nach dem Ereignis.
  - Einfügen oder Aktualisieren heißt neuer Slot, danach wird das Bit des alten Slots gelöscht. Die alte Zuordnung bleibt für ältere Treffer auflösbar.
- Ein Eintrag lässt sich ausblenden, ohne Daten zu entfernen, mit Rückmeldung über sein Vorhandensein. Abfragbar sind Gültigkeit (außerhalb des Bereichs ungültig), Anzahl gültiger Slots, Metadaten, Slot eines Eintrags und eine Momentaufnahme der Gültigkeitsbits in 64-Bit-Wörtern zum Hochladen. Gültigkeitsbits sind nebenläufig sicher.
- Treffer:
  - Kernel-Treffer aus Adresse und Bewertung werden in Segmentgeneration, Slot und Bewertung umgerechnet, sonst „Slot außerhalb des Bereichs“.
  - Auflösung über ein Manifest zu einem Eintragsverweis. Fehlt das Segment: „veraltete Generation“. Ungebundener Slot: „fehlende Zuordnung“ mit Generation und Slot.
  - Der neueste gültige Ort eines Eintrags wird ermittelt, neuere Segmente haben Vorrang.
  - Treffer einer älteren Manifestgeneration werden überführt: Auflösung im aktuellen, sonst im vorherigen Manifest. Unauflösbare werden verworfen. Ist der Slot noch gültig, bleibt der Treffer, sonst wird der aktuelle Ort gesucht und der Treffer mit unveränderter Bewertung umgesetzt oder verworfen.

### Manifeste und Veröffentlichung

- Ein Manifest ist eine Momentaufnahme der Segmente mit fortlaufender Generation, beginnend leer bei null. Ein Nachfolger liegt genau eine Generation höher. Abfragbar: Segmente, Generation, Gesamtbytes, gehaltene Sicherungen (Leases).
- Die Segmenttabelle für Kernel ist binär mit 24 Byte je Segment in Little-Endian-Reihenfolge: Segmentgeneration (8 Byte), Startadresse (8 Byte), Slot-Anzahl (4 Byte), Slot-Abstand (4 Byte, null bei variablem Layout).
- Das aktuelle Manifest ist für Leser sperrfrei verfügbar und atomar ersetzbar, mit Rückgabe des vorherigen. Eine nicht strikt größere Generation wird mit „veraltete Generation“ abgelehnt. Konkurrenz wird durch Wiederholung aufgelöst, Störungen sind für Tests auslösbar.
- Ein Leser sichert die aktuelle Generation per Lease. Solange sie besteht, bleiben Manifest und Segmente erhalten. Über die Lease werden Abschlussmarker bei allen Segmenten registriert, bereits abgeschlossene dürfen verworfen werden.
- Der Speicherbedarf eines Segmentwechsels ist aktueller Bestand plus eingehende Menge.
- Nicht mehr gebrauchte Segmente melden ihre Region samt offener Abschlussmarker (auch zurückgestellter Schreibvorgänge) an die Speicherverwaltung, sobald kein Manifest und keine Lease sie hält. Die Rückforderung liefert Regionen mit vollständig abgeschlossenen Markern und hält den Rest zurück.
- Neue Manifeste können hinter dem Ereignis ihres Kopiervorgangs vorgemerkt werden: Kopierauftrag auf der Kopier-Lane einreihen, Ereignis aufzeichnen, Manifest vormerken. Fehler ergeben einen Synchronisierungsfehler mit Ursache. Eine Abfrage veröffentlicht vorgemerkte Manifeste in Reihenfolge, solange das vorderste abgeschlossen ist, und liefert die Anzahl.
- Änderungsbenachrichtigungen an beliebig viele Abonnenten mit konfigurierbarem Puffer. Ereignisse mit Art, Segmentkennung, Segmentgeneration und Manifestgeneration: „hinzugefügt“ für neue Kennungen, „geändert“ bei anderer Segmentgeneration, keines für unveränderte. Senden ohne Abonnent scheitert nicht.

### Mutationsdisziplin

- Der Manager kennt den Laufzustand des Kernels und verändert nur in Lücken (Batch-Ende beziehungsweise Kommunikationsslot). Er ändert nichts unter dem Kernel und blockiert oder sperrt ihn nie aus.
- Atomar getauscht wird nur ein natürlich ausgerichteter Zeiger von acht Byte Breite. Größere Strukturen werden per Copy-on-Write neu angelegt, danach wird der Zeiger getauscht. Hierarchie: Wurzelzeiger, Segmenttabelle, Segment (Basis, Layout, Offset-Zeiger). Das gilt für Nachschlagetabellen und ganze Segmente, getauscht wird, sobald die Subengine dort nicht mehr liest.
- Aktualisieren an Ort und Stelle ist in einer Lücke trivial. Im persistenten Modus kann der Kernel einen halb aktualisierten Vektor mit gemischten, aber gültigen Werten lesen. Das wird bewusst akzeptiert und dokumentiert.
- **Wurzelzeiger:** nur der Host schreibt, mit Release-Semantik; das Gerät liest mit Acquire-Semantik. Vor der ersten Tabelle trägt er den größten darstellbaren Wert als Leerwert. Er kann in fest zugeordnetem, abgebildetem Hostspeicher liegen.
- **Segmenttabelle:** veröffentlichte Versionen sind unveränderlich, neue sind Kopien mit erhöhter Epoche, mit Epoche und Segmentregionen. Hinzufügen, Ersetzen (falsch bei fehlender alter Region) und Entfernen mit Rückmeldung. Kodierte Größe: 16 Byte plus 24 Byte je Segment.
- **Epochenverfolgung:** veröffentlichte Epoche und je Leser die zuletzt gemeldete, nur steigend. Ein beendeter Batch meldet seine Epoche. Ein Leser ohne laufenden Batch kann ruhen und gilt dann als angekommen. Die kleinste Leserepoche ist abfragbar, ohne Leser gleich der veröffentlichten.
- **Tabellentausch:**
  - Das Schreiben in gerätesichtbaren Speicher kehrt erst zurück, wenn der Inhalt vollständig sichtbar ist (Kopierereignis oder Freigabe abgebildeter Schreibvorgänge).
  - Eine Kopie der aktuellen Tabelle mit nächster Epoche ist verfügbar.
  - Veröffentlichung serialisiert: Epoche genau aktuell plus eins, sonst „veraltete Zuordnung“, sodass zwei Tabellen gleicher Basis nicht beide veröffentlicht werden. Inhalt sichtbar schreiben, Wurzel tauschen, Epoche voranschieben, vorherige Region mit neuer Epoche zurückstellen, protokollieren (Epoche, Segmentzahl, Offset), Epoche zurückgeben.
  - Rückgewinnung höchstens bis zur kleinsten Leserepoche und gewünschten Obergrenze, mit Anzahl.
  - Normale Starts erhalten Wurzel und Epoche als Kernel-Argument; ein Tausch wirkt ab dem nächsten Start.
  - Im persistenten Ringpuffer-Modus liest der Kernel den Zeiger einmal je Arbeitseinheit mit Acquire-Semantik. Ein Schutzobjekt hält die Version fest und meldet beim Freigeben die Epoche. Der Inhalt der neuen Tabelle muss vorher per Ereignis oder Speicherbarriere sichtbar sein.
  - Auf echter Hardware ist nachzuweisen, dass der Inhalt vor dem Wurzelwert sichtbar ist, auch über einen Kopier-Stream mit Ereignis.

### Doorbell-Synchronisation zwischen Batches

- Der Grafikprozessor veröffentlicht die Zahl abgeschlossener Batches mit Release-Semantik, der Host liest mit Acquire-Semantik.
- Der Host verändert nur zwischen Abschluss eines Batches und dessen Bestätigung. Das Gerät prüft vor jedem weiteren Batch, ob die Bestätigung den Vorgänger abdeckt. Der erste Batch darf immer starten.
- Der Host wartet nie auf den Kernel, nur auf ein Batch-Ereignis.
- Nicht blockierende Abfrage liefert eine Lücke bei Abweichung zwischen abgeschlossen und bestätigt. Zusammengefallene Abschlüsse öffnen eine Lücke mit der höchsten Batchnummer. Die wartende Abfrage wartet zuerst auf das Ereignis, Gerätefehler werden weitergereicht.
- Schließen der Lücke veröffentlicht die Bestätigung. Eine nicht geschlossene Lücke ist ein Programmierfehler. Vorher geschriebene Ergebnisse und Zuordnungen sind sichtbar.
- Beide Zellen können in fest zugeordnetem, abbildbarem Hostspeicher liegen.

### Kommunikationsslot

- Feste Reihenfolge:
  - Lesen, was der Kernel geschrieben hat: zurückgestellte und neue Schreibvorgänge Slot für Slot verarbeiten. Ist eine Adresse keinem Segment zugeordnet, wird der Rest für den nächsten Slot zurückgestellt. Gebundene Slots werden vom Gerät zurückgelesen, per Übernahme am Index angewendet und der zurückgegebene Stempel im Residenzverzeichnis vermerkt. Slots ohne Eintrag werden übersprungen, der Fortschritt beträgt mindestens ein Byte.
  - Ergebnisse auflösen: Adresse auf Eintrag abbilden. Nicht abbildbar oder im Index entfernt: verwerfen. Update ausstehend: als „veraltet“ markiert liefern (Diagnose, kein Fehler). Sonst unverändert liefern.
  - Updates einspielen. Das verändert die Abbildung und kommt daher zuletzt.
- Protokolliert werden angewandte Schreibvorgänge, aufgelöste, verworfene und veraltete Treffer, Warteschlangentiefe und Änderungen.
- Weil nur in Lücken verändert wird, sieht jedes Ergebnis den Abbildungsstand seines Batches. Eine gemeldete Adresse ist aktuell. Im Kernel-Protokoll gibt es keine Epoche und keine Sequenznummer.
- Läuft der Slot gegen eine weitergegangene Zuordnung, wird nebenwirkungsfrei „veraltete Zuordnung“ gemeldet und der Slot unter Struktursperre mit frischer Momentaufnahme wiederholt. Danach wird zurückgestellter Speicher zurückgewonnen.
- Das Zurücklesen von Gerätebereichen läuft über eine austauschbare Schnittstelle.
- **Adresskarte:** bildet Arena-Adressen auf Einträge ab, Segmentmenge kopierend verändert, Klone sind stabile Momentaufnahmen. Epoche wird mitgeführt, Nachfolger erhöhen sie um eins. Hinzufügen abgelehnt bei leerem Segment, vergebener Kennung oder Überlappung; Segmente nach Adresse sortiert. Entfernen meldet Vorhandensein. Slot-Bindungen liegen in gemeinsamen Zuordnungen und ändern sich nur in Lücken.
- **Zuordnungsspeicher:** hält die Karte versioniert, liefert Momentaufnahmen nur bei passender Epoche (sonst „veraltete Zuordnung“ mit Basis- und veröffentlichter Epoche) und veröffentlicht nur Karten mit Epoche aktuell plus eins.
- **Update-Warteschlange:** Lösen einer Bindung oder Binden an einen Slot mit Stempel. Je Eintrag nur das jüngste Update. Abfragbar: Enthaltensein, Tiefe, zurückgestellte Schreibvorgänge. Anwenden zuerst Löschungen, dann Bindungen: Lösen entfernt aus dem Residenzverzeichnis und löst die Slot-Bindung nur, wenn sie noch dazugehört; Binden löst zuerst eine alte Bindung; nicht abgebildete Ziele erzeugen eine Warnung. Rückgabe ist die Zahl angewandter Updates. Änderungen bleiben bis zur Ausführung in der Warteschlange.

### Indexaufbau durch Kernel

- Der Manager vergibt vorab einen beschreibbaren Bereich aus der Arena. Der Kernel schreibt per Cursor hinein und allokiert nie.
- Schreibzeiger-Modell: Jeder Anspruch schiebt den Zeiger voran. Passt einer nicht mehr, scheitern alle späteren, sodass die gültigen Bytes ein zusammenhängendes Präfix bilden. Ein gescheiterter Anspruch schreibt nichts, setzt einen Überlaufzustand und hält die kleinste Überlaufposition fest; der erste Überlauf wird protokolliert. Gruppenansprüche schieben mit einer einzigen Aktualisierung voran. Gültige Länge ist das Minimum aus Überlaufgrenze, Zeiger und Kapazität.
- Auf dem Grafikprozessor fasst ein Warp aus 32 Threads seine Ansprüche per Präfixsumme zusammen, mit einer atomaren Aktualisierung je Warp. Bei Überlauf wird ein Ungültig-Marker ausgegeben, Grenze und Kennzeichen gesetzt. Ergebnisse werden für den Host sichtbar gemacht.
- Am Batch-Ende meldet der Kernel die belegten Bereiche. Reicht der Bereich nicht, liefert er ein Teilergebnis und der Manager vergibt den nächsten.
- Ausführung eines Embedding-Batches: fortsetzen, bis der Kernel Treffer meldet. Vor jedem Schritt wird die Höchstzahl an Bereichen geprüft, sonst „Arena erschöpft“. Je Schritt ein Bereich reservieren und zuweisen, Eingabe hinter den verarbeiteten Bytes fortsetzen. Teilantworten zählen, kein Fehler. Der Batch wird immer beendet. Erfolg liefert Bereiche, Regionen und Teilantworten. Fehler geben alle reservierten Bereiche frei, gemeldet wird zuerst der Lauffehler.
- Adressierung gebauter Segmente: exklusive Präfixsumme auf Basisadresse (n Längen ergeben n plus eins Offsets), Offset-Tabelle mit Adress-zu-Eintrag-Abbildung. Das Layout ist fest, wenn der Schritt der größten Eintragslänge entspricht, in 32 Bit passt, größer null ist und der Füllanteil eine Promille-Schwelle nicht überschreitet, sonst variabel. Beide Layouts bilden Adressen ab und sind in eine Slot-Zuordnung umwandelbar.
- Embedding-Schreibvorgänge und Treffer laufen über getrennte Warteschlangen mit je einem Erzeuger und Verbraucher. Schreibvorgänge werden zuerst registriert, Treffer erst danach. Der Verbraucher entnimmt zuerst alle Schreibvorgänge, dann alle Treffer, sodass kein Treffer ohne Adresszuordnung beobachtet wird. Indexieren während der Suche funktioniert, weil das Lesen der Schreibvorgänge vor der Ergebnisauflösung liegt. Einstellen und Tiefe sind abfragbar und protokolliert.

### Speicherverwaltung für Index-Segmente

- Segmente sind auf mindestens 256 Byte ausgerichtet. Ein konfigurierbarer Reserveanteil wird nie vergeben.
- Vor jeder Zuteilung werden ausstehende Freigaben eingesammelt. Zugelassen wird nur, wenn Belegtes plus Anforderung plus Ersetzungsspielraum plus Reserve in die Kapazität passen, sonst „Arena erschöpft“ mit angeforderter und verfügbarer Menge.
- Fehler des darunterliegenden Speichers: Erschöpfung als „Arena erschöpft“, veraltete Region als solche, Übriges gekapselt.
- Die Arena nutzt eine geliehene Referenz oder geteilten Besitz am Speicherverwalter.
- Buchführung: lebende Bytes, größtes Segment, zurückgestellte Segmente. Ersetzungsspielraum ist Zahl gleichzeitiger Ersetzungen (Standard eins) mal größtes Segment, bei regulärer Zuteilung unter Berücksichtigung der neuen Anforderung freigehalten, beim Ersetzungsbeginn nicht erneut verlangt.
- Budgets überlaufsicher: Ersetzungsbudget (lebende Bytes, Ersetzungen, größtes Segment) und Arena-Budget (Kapazität, Reserve, lebend, zurückgestellt, Spielraum, verfügbar). Verfügbar ist Kapazität minus Belegtes, Reserve und Spielraum, nie negativ. Auslastung ist protokollierbar.
- Freigabe sofort nur ohne mögliche Leser, sonst nach Ereignis. Jede Freigabe prüft Lebendigkeit und Nicht-Zurückstellung, sonst „veraltete Region“. Ersetzte Regionen werden mit der Epoche der Ersetzung zurückgestellt. Die Rückgewinnung gibt alle Regionen frei, deren Epoche nicht über der kleinsten Leserepoche liegt; verschwundene erzeugen nur eine Warnung.
- Ersetzung durch Kopie: neues Segment gleicher Größe neben dem alten, höchstens eine Ersetzung gleichzeitig, wartend oder nicht wartend („keine Ersetzung“). Altes Segment muss leben. Abschluss veröffentlicht die Kopie mit Epoche und stellt das alte zurück; scheitert das, bleibt die Kopie beim Vorgang. Verwerfen gibt die Kopie frei. Alte und neue Region sind abfragbar.
- Schrittweise beschreibbare Segmente (Bump-Allokation) für einen Schreiber: Ansprüche mit Ausrichtung liefern absolute Adressen, passt einer nicht, „Arena erschöpft“ mit Restmenge ohne Zustandsänderung. Geschriebene und verbleibende Bytes sind abfragbar, das Segment ist zerlegbar.

### Residenz und Ladestrategie

- Geladen wird lazy und scope-getrieben: nur, was ein Task braucht.
- Eine austauschbare Geometrie liefert Segment eines Eintrags, Eintrags- und Segmentgröße. Mitgeliefert: gleich große Einträge in gleich großen Segmenten, Kapazität null gilt als eins.
- Zugriffsarten: regulär und selten (kleine, selten gelesene Mengen aus abgebildetem Hostspeicher).
- Planarten: ganze Segmente laden, Einträge sammeln und gemeinsam senden, direkte Abbildung ohne Kopie.
- Richtlinie: Dichteschwelle (Standard 0,12 der Segmentbytes, nur endlich, auf null bis eins begrenzt), Segmentgröße (Standard 64 Mebibyte), Höchstgröße für direkte Abbildung (Standard 64 Kibibyte).
- Die Segmentgröße bestimmt der Manager nach Budget: ein Achtel der verfügbaren Menge, begrenzt auf 4 bis 256 Mebibyte, abgerundet auf ganze Mebibyte. Sie ist keine Eigenschaft des Index.
- Die Dichteschwelle ist aus einem kalibrierten Profil mit der Achse „Dichte“ in Promille übernehmbar, sonst unverändert.
- Planung: leere Mengen ergeben keinen Plan. Bei seltenem Zugriff bis zur Höchstgröße genau ein Plan mit direkter Abbildung. Sonst je Segment angeforderte Bytes ermitteln; ab Dichteschwelle bei nicht leerem Segment ganzes Segment, Rest sammeln. Höchstens ein Plan je Art, Segmente zuerst. Entscheidungen werden protokolliert.
- Ausführung: Pläne nacheinander, Abbruch beim ersten Fehler, überlaufsichere Bytesumme.
- Aggregation: Tasks gegen denselben Index werden über die Vereinigung ihrer Scopes gebündelt und je Index genau einmal nebenläufig geladen. Der Sammler zählt Anfragen, ist entleerbar und auf Leere prüfbar.
- Single-Flight: Gleichzeitige Anfragen mit gleichem Schlüssel teilen einen Ladevorgang samt Ergebnis, auch im Fehlerfall. Nur der erste startet, die Startfunktion blockiert nie. Nach Abschluss wird nur der eigene Eintrag entfernt. Fehlschläge wiederholt der nächste neue Aufrufer. Laufende Vorgänge sind abfragbar, Mitläufer werden protokolliert.
- Transfers laufen im Pool für blockierende Arbeit. Ein Abbruch ergibt einen Übertragungsfehler Host zum Gerät.
- Gestaffelte Haltung: Zusammenfassungsstrukturen klein, zuerst geladen, am längsten gehalten; Vektorsegmente warm und verdrängbar; alles andere nicht geladen.
- Überlappende Einträge: Der Manager entscheidet, ob Vektoren einmal gehalten und mehrfach referenziert oder dupliziert werden. Das ist keine Vertragsfrage.
- Betriebseinheit je Indexoperation: verbindet Manager, Tabellentausch, Zuordnungsspeicher und Warteschlange geladener Segmente und veröffentlicht jedes gemeldete Segment selbständig. Strukturelle Änderungen sind serialisiert. Meldet ein Laden „Arena erschöpft“, werden ausstehende Segmente veröffentlicht, Platz geschaffen und genau einmal wiederholt. Abgleichdurchläufe veröffentlichen Nachgeladenes vor ihrer Rückkehr. Veröffentlichen versucht alle Segmente und meldet danach den ersten Fehler oder die Anzahl. Verwaltungsfehler werden zu Indexfehlern: „Arena erschöpft“ bleibt, alle anderen werden Übertragungsfehler.

### Übertragung auf das Gerät

- Bausteine: Quelle für Segmentbereiche (Index-Arena), abgebildeter Hostspeicher, geladenes Segment aus Region, Slot-Zuordnung und Bindungen mit vor den Daten gelesenem Stempel, austauschbare Senke für geladene Segmente.
- Ganze Segmente: Einträge aufsteigend zuordnen. Direktes Kopieren, wenn Einträge lückenlos in einem gestützten Bereich liegen, nicht leer und ausgerichtet sind und der Index genau die Bereichslänge liefert. Vorher „sequentiell“ setzen, Bereich anlegen, kopieren, synchron abwarten, bei Fehler freigeben. Slot-Grenzen sind absolute Adressen mit Endmarke. Nur Einträge mit bekanntem Stempel erhalten eine Bindung. Nicht direkt kopierbare Segmente gehen in den Sammelpfad.
- Sammelpfad: Stempel vor den Daten erfassen, „wahlfrei“ je Bereich setzen. Wiederholt Staging-Platz bepacken (ohne Fortschritt: „Sammeln ohne Fortschritt“), Bereich anlegen, eine Kopie absenden, abwarten, melden, Bytes summieren. Beim Bepacken wartet die Sammlung auf freie Plätze, packt ausgerichtet bis zum ersten nicht passenden Eintrag, zählt entfernte Einträge als erledigt und meldet „Arena erschöpft“, wenn schon der erste nicht passt. Plätze werden erst nach dem Ereignis wiederverwendet. Kopien laufen asynchron auf dem Stream einer eigenen Kopier-Lane. Gewöhnlicher und seitengesperrter Hostspeicher werden unterstützt.
- Direkte Abbildung: ohne abgebildete Arena oder Platz Rückfall auf den Sammelpfad mit Protokoll, sonst Stempel und Daten lesen, ausgerichtet in den abgebildeten Bereich schreiben und melden. Sehr kleine Mengen werden so kopierfrei gelesen.

### Verdrängung

- Spiegel in Grafik- und Arbeitsspeicher werden ausschließlich ausdrücklich verdrängt: Leerlaufablauf, Füllstandsgrenzen mit Hysterese, kostenbewusste Opferwahl. Gründe: Leerlauf, Druck, Platzbedarf.
- Stufen: Zusammenfassungen und festgehaltene Einträge werden nie Opfer. Festhalten wirkt, solange der Halter lebt.
- Nutzbare Kapazität ist Gesamtkapazität minus Reserve und Spielraum, Belegung aus lebenden und zurückgestellten Bytes.
- Richtlinie aus Hochmarke, Tiefmarke und Leerlaufdauer. Nicht endliche Marken werden korrigiert, Hochmarke auf null bis eins, Tiefmarke auf null bis Hochmarke begrenzt. Grafikspeicher: 0,90 und 0,75 bei 600 Sekunden. Arbeitsspeicher: 0,85 und 0,70 bei 60 Sekunden.
- Aufnehmen mit Größe, Stufe und Wiederladekosten (nicht endlich oder negativ zählt null). Start mit einem Treffer, erneute Aufnahme ersetzt korrekt.
- Priorität nach dem Prinzip GreedyDual-Size-Frequency: Boden plus Treffer mal Kosten durch Größe (Größe null als eins). Der Boden steigt bei Druck- oder Platzverdrängung auf die Opferpriorität.
- Zugriffe erhöhen Treffer, setzen den Zeitpunkt und berechnen neu. Entfernen meldet Bytes. Anzahl, Bytes und Enthaltensein sind abfragbar.
- Leerlaufablauf verdrängt deterministisch nach Schlüssel. Entlastung erst über der Hochmarke bis auf die Tiefmarke, Opferreihenfolge: niedrigste Priorität, dann am längsten unbenutzt, dann kleinerer Schlüssel. Der periodische Einstieg macht beides nacheinander.
- Platzbeschaffung: genug frei, nichts tun; sonst Opfer sammeln; reichen alle nicht, „Arena erschöpft“ ohne Verdrängung; sonst mit Grund Platzbedarf verdrängen.
- Entfernen verdrängter Segmente: aus neuer Tabelle und Zuordnung entfernen, veröffentlichen, Region hinter der neuen Epoche zurückstellen, freigeben. Unbekannte Regionen überspringen, bei Fehlschlag das Segment wieder unter Kontrolle stellen und abbrechen.
- Jede Entscheidung wird mit Opfer, Größe, Treffern und Grund protokolliert.

### Konsistente Momentaufnahmen

- Gleichzeitig veränderbare Daten werden nach dem Prinzip einer Sequenzsperre wortweise atomar gelesen, nie halb geschrieben.
- Der Leser liefert eine vollständige Kopie und die Zahl der Wiederholungen, solange ein Schreiber aktiv ist oder der Zähler sich änderte. Schreiber schließen sich aus, decken die gesamte Region ab und erhöhen den Zähler um zwei.
- Aufbau aus rohen Adressen ist erlaubt, wenn Ausrichtung, Gültigkeit und einheitliches Protokoll zugesichert sind.

## Kernel und Bewertung

### Kernel-Register und Laufzeitübersetzung

- Das Register liefert zu Name, Datentyp, Architekturstufe und Variante (etwa Dimension) die geladene Funktion. Quellen: Laufzeitkompilierung für die Zielarchitektur, vorkompilierter Maschinencode als Rückfall bei zu altem Treiber, Kernel der externen Bibliothek.
- Laden erst bei Bedarf, gleichzeitige Anfragen führen nie zu doppelter Übersetzung. Protokoll mit Name, Datentyp, Architektur, Variante und Dauer. Anzahl und Leere abfragbar, Verwerfen aller Einträge mit Anzahl. Auffindbar als Ressource unter fester Kennung.
- Übersetzung aus Quelltext mit gemeinsamer Einleitung (Typdefinition, Ladefunktion, Warp-Reduktionen), Auswahl per Name.
- Zielarchitektur aus der Rechenfähigkeit: Stufen 6.1, 7.0, 7.5, 8.0, 8.6, 8.9, unbekannte fallen auf 7.5 zurück. Lehnt der Treiber den Zwischencode ab, wird gewarnt und als Maschinencode für genau die Gerätearchitektur neu übersetzt.
- Fehler: Übersetzungsfehler mit Protokoll, Modulladefehler, unbekannter Name oder fehlende Funktion, nicht unterstützter Elementtyp als „nicht verfügbar“. Die Rechenfähigkeit wird bei Einrichtung ermittelt, sonst „nicht verfügbar“.
- Zur Übersetzungszeit festgelegt: Datentyp, Dimension, Kachelgröße als eigene Varianten. Dynamisch über den Parameterblock: Slot-Indizes, Zähler, Segmenttabelle, Anfragelänge, Kopf und Ende.

### Unterstützte Genauigkeiten

- Je Geräteart ist festgelegt, für welche Elementtypen Bewertungskernel existieren, unabhängig von aktivierter Grafikunterstützung: Zentralprozessor nur 32-Bit-Gleitkomma, Grafikprozessor 32- und 16-Bit-Gleitkomma.
- Für jeden unterstützten Typ werden die Achsen Operanden und Bewertung gemeldet, Typen ohne Kernel nie. Andere Achsen schränkt der Katalog nicht ein.

### Anfrage-Batch, Quelle und Arbeitsbereiche

- Mehrere Anfragen gleicher Dimension werden gebündelt, damit jeder Dokumentabschnitt einmal für alle gelesen wird. Jede Anfrage ist eine zeilenweise Matrix aus Token mal Dimension, dicht genau ein Token. Prüfung: Dimension ungleich null, nicht leer, Länge Vielfaches der Dimension, sonst Formfehler. Abfragbar: Dimension, Anzahl, Leere, Tokens und Rohdaten je Anfrage.
- Dokumentvektoren kommen über eine Quellenschnittstelle: Dimension, Slots je Segment, Slot-Offset für Treffer, Vektoren eines Slots (nur gültige Tokens) oder „nicht vorhanden“. Eine Arbeitsspeicher-Quelle berechnet Offsets als Summe der vorangehenden Slot-Bytes (vier Byte je Wert), unbekannte Segmente haben null Slots.
- Vor jeder Bewertung wird der Slot-Bereich geprüft, sonst Fehler mit Segment, Grenzen und Slot-Anzahl.
- Ein Arbeitsbereich ist ein halboffener Slot-Bereich und Einheit eines Kernel-Starts. Länge nie negativ, Leere abfragbar, teilbar in Vorderteil höchstens gewünschter Größe und Rest.

### Dichte Bewertung

- Kosinusähnlichkeit als Skalarprodukt, gültig für Einheitsvektoren. Exakter Modus teilt durch das Normprodukt, für abweichende Längen etwa durch 16-Bit-Speicherung; nicht positiver oder nicht endlicher Nenner ergibt null.
- Jede Anfrage genau ein Token. Leer, wenn das Normquadrat nicht endlich oder unter 10⁻¹⁸ ist: Fehler „leeres Embedding“ mit Anfrageindex, für alle Anfragen vor Beginn geprüft. Falsche Slot-Dimension oder leere Slots ergeben Formfehler.
- Ergebnis je Anfrage eine Bewertung je Slot in Slot-Reihenfolge. Scorer für beide Modi liefern die besten Treffer.

### Rechenprimitive auf dem Zentralprozessor

- Skalarprodukt mit achtfach parallelen Teilsummen für automatische Vektorisierung.
- Maximalähnlichkeit (späte Interaktion): je Anfrage-Token das größte Skalarprodukt über Dokument-Token, summiert. Ein Dokument ohne Tokens ergibt Maskenwert mal Anfrage-Tokens, wie auf dem Grafikprozessor.
- Normalisierung auf Einheitslänge, Nullvektoren bleiben.
- Slotweiser Durchlauf: Bereich und Dimension prüfen, Slots als äußere Schleife gegen alle Anfragen, laufendes Maximum je Anfrage-Token ohne Matrix über Slots. Fehlende Vektoren gelten als leer, unteilbare Längen ergeben Formfehler, Einzelfehler brechen ab.

### Maximalähnlichkeit als Operation

- Eine einheitliche Operation liefert die besten Treffer je Anfrage über einen Slot-Bereich und einen Anfrage-Batch, mit austauschbarer Implementierung.
- Kachelkette aus Bibliotheksoperationen auf Zentral- oder Grafikprozessor mit wählbarer Genauigkeit und Kachelgröße (unter eins wird eins). Nicht abbildbare Genauigkeit oder nicht öffnendes Gerät ergeben „nicht verfügbar“.
  - Dokument-Tokens in Kacheln fester Größe zerlegen und transponieren, letzte Kachel mit Nullen auffüllen und per additiver Maske ausschließen.
  - Je Kachel Matrixprodukt, Maske, Maximum je Anfrage-Token, laufendes Maximum über Kacheln, Summe in 32 Bit.
  - Zwischenmatrizen höchstens Anfrage-Tokens mal Kachelgröße. Leere Dokumente ergeben Maskenwert mal Anfrage-Tokens.
  - Anfragen werden vorher in die Genauigkeit überführt. Gleiche Prüfungen wie der Durchlauf, Bibliotheksfehler als Startfehler.
- Fusionierter Kernel hinter derselben Operation, bei dem die Zwischenmatrix nie im Grafikspeicher liegt.
- Die Dokumentseite ist maskiert, die Maskentokens der Anfrage zählen mit.

### Auswahl der besten Treffer

- Begrenzte Menge je Anfrage aus Offset und Bewertung. Höherer Wert gewinnt, bei Gleichheit niedrigerer Offset. Nicht numerische Werte werden verworfen, bei Anzahl null wird nichts aufgenommen.
- Ausgabe absteigend, bei weniger Kandidaten entsprechend kürzer. Anzahl, Sollgröße und Leere abfragbar, einzeln oder als Folge aufnehmbar.
- Lokale Reduktion je Arbeitsabschnitt, Zusammenführung der Teilergebnisse auf dem Zentralprozessor. Ein Bereichs-Scorer übernimmt Rohbewertungen mit Slot-Offsets, mehrere Bereiche werden nacheinander bewertet und zusammengeführt.

### Grafikprozessor-Kernel

- Zwei Kernel: dichtes Skalarprodukt und fusionierte Maximalähnlichkeit. Dokumente in 32- oder 16-Bit-Gleitkomma, 16 Bit beim Laden auf 32 Bit gewandelt, Anfragen und Ergebnisse in 32 Bit.
- Skalarprodukt: ein Thread-Block je Slot, Teilsummen über Threads und Warps reduziert, Ausgabe Anfragen mal Slots.
- Maximalähnlichkeit: ein Block je Dokument für genau eine Anfrage. Aufgefüllte Dokumente mit Längenliste, auf die Auffülllänge begrenzt. Token ohne gültiges Dokument-Token erhalten minus 10 000.
- Dimension wird bei der Übersetzung festgelegt. Start mit 128 Threads je Block, ein Block je Slot (mindestens einer), kein dynamischer Shared Memory. Startfehler werden mit Ursache gemeldet.

### Schnittstelle für Kernel-Batchläufe

- Segmente mit Basisadresse, Eintragsanzahl und festem oder variablem Layout (Offset-Tabelle mit Einträgen plus eins). Ein Batch aus Segmenten, Anfrageanzahl, optionalem Parameterbereich und Zeitbudget.
- Lebenszyklus: Batch beginnen mit Kennung, auf Eingaben ausführen mit Treffern oder Teilergebnis, beschreibbare Bereiche zuteilen (vor dem ersten Lauf und nach Teilergebnissen), nie abnehmenden Fortschritt abfragen, Batch beenden mit Rückgabe der Bereiche.
- Die Schnittstelle trägt keine Zeiger. Adresse zu Eintrag: fest per Abstand durch Schrittweite (kein Eintrag bei Schrittweite null, Adresse vor Basis oder Index über Anzahl), variabel per Offset-Intervall.

### Arbeitsabschnitte

- Ein Arbeitsabschnitt ist die Planungseinheit mit Zeitbudget. Die Länge zielt auf eine Startdauer von standardmäßig zwei Millisekunden, aus dem exponentiell geglätteten Durchsatz (Glättungsfaktor 0,3).
- Ohne Messung gilt die Mindestlänge. Die Länge liegt zwischen Mindest- und Höchstlänge (Standard 16 und rund eine Million Slots), Mindestlänge mindestens eins. Durchsatz in Slots je Sekunde ist abfragbar, vorher nicht vorhanden. Beobachtungen mit null Slots werden ignoriert, Dauern auf eine Mikrosekunde nach unten begrenzt.
- Unterbrechung nur an Abschnittsgrenzen, geprüft nach jedem außer dem letzten. Mindestens ein Abschnitt läuft immer vollständig. Anfrage-Tasks können Indexier-Abschnitte überholen.
- Abbruchgründe: Zeitbudget erschöpft (Vorrang) oder Verdrängung. Ergebnis: erledigte Slots, Restbereich, Grund. Fehler beenden sofort.
- Eine Kachel wird gegen alle wartenden Anfragen bewertet, um die Lesekosten zu amortisieren.

### Startmodi

- Startart und Scorer sind getrennt. Modi: direkter Start, Graphaufzeichnung für Wiederholung, persistenter Kernel mit Ringpuffer.
- Verfügbar ist der direkte Start: Arbeitsauftrag unverändert an die Lane, Lane-Fehler als Kernelfehler, Nachverfolgung auf feiner Stufe. Die anderen Modi werden mit „nicht verfügbar“ abgelehnt, mit dem Grund, dass Graphaufzeichnung beziehungsweise persistenter Modus nicht aktiviert ist.
- Startschlüssel: Operation, Formklasse, beteiligte Regionen mit Generation, Datentyp, Variante, Lane-Klasse. Geänderte Generationen ergeben neue Schlüssel, veraltete Aufzeichnungen laufen nie.
- Formklasse ist die nächsthöhere Zweierpotenz, mindestens eins, bei Überlauf 2³¹.
- Vor dem ersten produktiven Start ist eine Aufwärmphase möglich, der erste Start gilt als nicht produktiv. Ohne eigene Umsetzung geschieht nichts.

### Vergleich mit einem Referenz-Scorer

- Ergebnisse werden ohne exakte Gleichheit verglichen, absolute Toleranz 10⁻⁵ bei 32 Bit und 10⁻³ bei 16 Bit.
- Abweichungsarten: unterschiedliche Trefferanzahl (keine weiteren Ränge dieser Anfrage), Wertabweichung mit Rang und Differenz, Rangabweichung mit Offsets. Keine Rangabweichung bei Beinahe-Gleichstand benachbarter Referenzwerte.
- Beide Scorer laufen über dieselben Arbeitsbereiche, Abweichungen werden als Warnung mit Segment protokolliert und führen nie zum Abbruch. Fehler der Scorer werden weitergereicht.

## Präzision, Kalibrierung und Profile

### Prinzipien

- Zwischen Zentral- und Grafikprozessor gibt es keinen Rundungspunkt- oder Datentypvertrag.
- Der Präzisionsparameter ist ein Wunsch zwischen null (schnellste) und eins (präziseste Kombination), keine Zusicherung.
- Die Kalibrierung unterbricht oder blockiert den Nutzer nie.

### Fähigkeitsachsen und Fähigkeitsmengen

- Achsen und Werte werden über frei wählbare, erweiterbare Zeichenketten identifiziert, vergleichbar, sortierbar, als Schlüssel nutzbar, günstig kopierbar, lesbar ausgebbar. Werte sind auch aus natürlichen Zahlen erzeugbar.
- Eine Fähigkeitsmenge ordnet jeder Achse eine nie leere Wertemenge zu, mit Hinzufügen, Enthaltensein, Aufzählen und Leere. Aufzählung, Vergleich, Hash und Serialisierung sind reihenfolgeunabhängig.
- Eine Menge erfüllt eine Anforderung, wenn jeder geforderte Wert vorhanden ist. Leere Anforderungen sind immer erfüllt.
- Vordefinierte Achsen: Operanden, Akkumulator, Bewertung, Fusion, Batchgröße, Kachelgröße, Threadanzahl, Dichte. Werte: 32-Bit-Gleitkomma, 16-Bit-Gleitkomma, 16 Bit gespeichert mit 32 Bit gerechnet, 8-Bit-Ganzzahl, 8 Bit über den Vier-Wege-Skalarprodukt-Befehl des Grafikprozessors, 8 Bit über Ganzzahl-Befehle für neuronale Netze des Zentralprozessors, an, aus.
- Referenzmengen als Obergrenzen: Grafikprozessor mit Operanden und Akkumulator 32 und 16 Bit, Bewertung 32, 16 Bit und 8-Bit-Skalarprodukt, Fusion an und aus. Zentralprozessor mit Operanden 32 Bit und 16 Bit mit Umwandlung, Bewertung 32 Bit, 8 Bit und 8 Bit über Ganzzahl-Befehle. Numerische Achsen werden zur Laufzeit ermittelt.
- Es gibt keine Profilnamen und keine fest einkodierte Profilaufzählung. Achsen und Werte sind je Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle.

### Fähigkeitserkennung

- Zentralprozessor: 32 Bit immer, 16 Bit mit Umwandlung bei Halbpräzisions-Umwandlung, 8-Bit-Bewertung bei 256-Bit-Vektorerweiterung oder Ganzzahl-Befehlen für neuronale Netze, Letzteres nur mit diesen Befehlen. Threadanzahlen als Zweierpotenzen bis zur logischen Kernzahl plus die Kernzahl selbst, mindestens ein Kern. Merkmale (Vektorerweiterung, Halbpräzisions-Umwandlung, Fused-Multiply-Add, Ganzzahl-Befehle) werden vom System ermittelt, auf fremden Architekturen gelten sie als fehlend.
- Grafikprozessor: 32 Bit immer, 16 Bit ab Stufe 5.3, 8-Bit-Skalarprodukt ab 6.1, Fusion an und aus. Tensorkerne werden anhand von Stufe und Gerätename erkannt (Baureihen TU116 und TU117 haben trotz Stufe 7 keine) und nur protokolliert. Ohne Grafikunterstützung oder Gerätekontext scheitert die Erkennung mit einem Fähigkeitsfehler.
- Numerische Achsen aus der Konfiguration: Batchgrößen 1, 8, 32, Kachelgrößen Zweierpotenzen von 16 bis 512, maximale Sequenzlänge 512. Nur positive Batchgrößen und Kacheln bis zur Sequenzlänge. Die Kernel-Menge ist vorgebbar, sonst gilt der eingebaute Katalog.
- Erkannte Fähigkeiten werden auf vorhandene Kernel reduziert. Unbekannte Achsen bleiben, geleerte Katalogachsen entfallen, Entferntes wird protokolliert. Das Ergebnis wird mit Subengine, Achsen- und Wertezahl protokolliert. Die Auswahl des Verfahrens erfolgt zentral nach Geräteart.

### Messlast und Abweichungsmaße

- Reproduzierbare synthetische Messlast aus Startwert, normiert und duplikatfrei: Standard 20 000 Dokumente, 384 Dimensionen, höchstens 32 Anfragen, die besten fünf für die Rangprüfung. Referenz ist die 32-Bit-Bewertung auf dem Zentralprozessor.
- Anfragezahl aus der Batchgröße, ohne Angabe acht, begrenzt auf die Höchstzahl. Nicht numerische, null oder zu große Werte machen die Kombination unbrauchbar. Kleinere Batches nutzen die ersten Anfragen.
- Kosinusabweichung ist eins minus Kosinusähnlichkeit, mindestens null, bei Nullvektor eins. Werte unter einem Millionstel gelten als Rauschen.
- Relativer Fehler ist Abstand durch Referenzlänge, bei Nullreferenz null oder unendlich.
- Treffer-Überlappung ist der Anteil der besten Referenztreffer im Kandidaten, Gleichstände nach Position, bei null gleich eins.
- Gesamtbewertung zeilenweise mit der schlechtesten Zeile, je Zeile das Größere aus Kosinusabweichung und eins minus Überlappung.
- Gültigkeitszustände: gültig, nicht endlich, entartet, Abweichung über Maximum, Ausführung gescheitert, jeweils mit Text. Ungleiche Längen oder unteilbare Daten sind entartet. Eine Zeile ist entartet, wenn sie konstant ist (Varianz unter 10⁻⁹ des mittleren Quadrats) oder ihr Längenverhältnis außerhalb von 0,5 bis 2,0 liegt, konfigurierbar.

### Messschnittstelle und Messmittel

- Austauschbare Messschnittstelle: Abweichungsmessung gegen die Referenz (sonst „unverträglich“) und Zeitprobe mit normierter Zeit je Arbeitseinheit oder „unterbrochen“, wenn eine Produktivaufgabe das Gerät anfordert.
- Aufwärmen beim ersten Gebrauch, Wiederholungen auf das Zeitfenster skaliert (Standard zehn Millisekunden), Einzelstarts im Abschnittsbudget (Standard fünf Millisekunden).
- Eine Kombination ist ein Wert je Achse plus die Sequenzlängen des Testsatzes, die nur Verarbeitbarkeit prüfen. Textform aus Achse-Wert-Paaren.
- Messergebnis: robuste Zeit, Abweichung, Gültigkeit, Stabilität, Probenzahl, relative Unsicherheit.
- Austauschbar sind die Abfrage wartender Produktivaufgaben (Standard nie, auch über einen gemeinsamen Schalter) und die monotone Zeitquelle.
- Zentralprozessor (4 000 Dokumente): nur 32 Bit, positive Threadanzahl (Standard eins) mit eigenem Pool, parallele Bewertung, einmaliges Aufwärmen je Kombination, Zeit je Durchlauf und Anfrage, Unterbrechungsprüfung zwischen Durchläufen.
- Grafikprozessor (Standardmaße): Kosinuspfad mit 32 oder 16 Bit aus Bewertungs- oder Operandenachse, Dokumente in beiden Genauigkeiten auf dem Gerät, Zeit per Ereignissen auf dem Stream, 64 Mebibyte Cache-Leerung vor jeder Probe, mindestens 100 Millisekunden Aufwärmen. Wiederholungen eins bis 10 000, Gruppen im Abschnittsbudget, je Kombination gemerkt, Unterbrechungsprüfung zwischen Gruppen. Ohne Grafikunterstützung Fähigkeitsfehler.

### Kalibrierungslauf

- Ziel sind Subengine und Modellkennung. Der Kalibrierer zählt Kombinationen auf, misst verschränkt gegen die Referenz, verwirft ungültige und bildet die Pareto-Front über Zeit und Abweichung. Obergrenzen beenden eine Messung mit dem besten, als instabil markierten Zwischenstand.
- Konfiguration (Standard in Klammern): maximale Abweichung (0,05), Mindestwiederholungen (3, kleinere werden angehoben), Zeitfenster (zehn Millisekunden), Stabilitätsschwelle (0,05), Streuungsmaß Variationskoeffizient oder Spannweite durch Median (Variationskoeffizient), Schätzer Median oder Minimum (Median), harte Obergrenze (40 Versuche, optional 30 Sekunden), Abschnittsbudget (fünf Millisekunden), Gesamtbudget (optional 600 Sekunden), Dominanzspielraum (zehn Prozent), Rückzugspause (fünf Millisekunden), trennbare Achsen (Batchgröße, Kachelgröße, Threadanzahl), Vollmessung des kartesischen Produkts als Referenz (aus), Sequenzlängen ab 16 in Viererschritten bis zur Obergrenze einschließlich (Standard 16, 64, 256, 512).
- Ablauf: leere Fähigkeitsmenge ergibt Fähigkeitsfehler, leere Achsen werden ignoriert, Werte numerisch aufsteigend, dann alphabetisch. Achsen in Kern- und trennbare Achsen geteilt. Im Normalmodus zuerst die Achsenstufe, dann der Prüfschritt mit den Kandidaten, im Vollmodus das ganze Produkt. Die Front stammt nur aus dem Prüfschritt. Bericht mit Produktgröße, Budgetstatus und Laufzeit, Zusammenfassung und Protokollabschnitt.
- Achsenstufe (Beschneidung): Je Kernkombination wird jede trennbare Achse einzeln variiert, die anderen auf ihrem kleinsten Wert. Einwertige Achsen gewinnen automatisch. Gewinner sind die Pareto-optimalen Werte; ohne Gewinner wird die Kernkombination übersprungen. Kandidaten sind das deduplizierte Produkt. Das Budget wird vor jeder Kernkombination geprüft.
- Messen einer Gruppe: zuerst die billige Abweichungsphase mit Verwerfung (gescheitert, nicht endlich, über Maximum, sonstiger Grund), andere Fehler brechen ab. Danach rundenweise Zeitmessung reihum über alle aktiven Kombinationen, in ungeraden Runden in umgekehrter Reihenfolge, damit Lastschwankungen alle gleich treffen. Die erste Kombination ist Anker, Stabilität wird am Zeitverhältnis zum Anker derselben Runde beurteilt (nur bei positiver Ankerprobe). Jeder Versuch zählt, Unterbrechungen lösen die Rückzugspause aus.
- Beendigung: Die Streuung ignoriert das langsamste Fünftel, ist bei weniger als zwei Proben unendlich. Eine Nichtanker-Kombination endet vorrangig als „dominiert“ (bei genug Proben ist eine andere bei höchstens gleicher Abweichung um mehr als den Spielraum schneller), sonst „stabil“ bei stabiler Verhältnisreihe, sonst „instabil“ an der Obergrenze. Der Anker endet zuletzt, stabil bei stabilem Mitbewerber ohne instabilen oder eigener stabiler Reihe.
- Ergebnis je Kombination: dominiert, verworfen mit Grund, übersprungen (Budget oder Gerät nie verfügbar) oder gemessen mit Zeit, Abweichung, Stabilität, Proben und Unsicherheit (ersatzweise Dominanzspielraum). Bei Budgetende aktive Kombinationen mit Proben gelten als gemessen und instabil.
- Bericht: Ziel, Einträge mit Stufe, Kombination und Ergebnis, Front aufsteigend nach Zeit mit streng fallender Abweichung, Zähler und Protokoll jeder Einordnung.

### Pareto-Front, Positionen und Profile

- Die Front minimiert Zeit und Abweichung ohne nicht endliche Punkte, aufsteigend nach Zeit. Ein Punkt bleibt nur bei streng kleinerer Abweichung als alle schnelleren. Berechenbar für Zahlenpaare und beliebige Elemente.
- Positionen aus kumulierter Bogenlänge auf min-max-normierten Achsen, Zeit optional logarithmisch. Leere Front ohne Positionen, ein Punkt bei 0,5, Gesamtlänge null ergibt gleichmäßige Verteilung.
- Ein Profil ist unbenannt, mit Position zwischen null und eins und einem Wert je Achse. Positionen außerhalb sind „ungültige Präzision“.
- Einzige Stellgröße ist die Zusammenführungsschwelle auf der normierten Skala (Standard 0,1), modell- und hardwareunabhängig. Ableitung: nicht endliche Punkte entfernen; Punkte entfernen, die weniger präzise sind, ohne signifikant schneller zu sein (protokolliert), etwa wegen gleicher Emulationspfade oder langsamer emulierter Genauigkeit; Positionen logarithmisch berechnen; Nachbarn unter der Schwelle gruppieren; Vertreter ist der präziseste Frontpunkt in der Gruppe, sonst der schnellste; Positionen neu berechnen; Konfigurationen übernehmen.
- Die Profilzahl folgt allein aus der Streuung. Ein leeres Ergebnis bedeutet kein brauchbares Profil.

### Profilauswahl

- Der Wunsch wird auf null bis eins begrenzt, „keine Zahl“ ist „ungültige Präzision“. Gewählt wird das Profil mit kleinstem Abstand, bei Gleichstand das präzisere.
- Die Skala gilt relativ zur Subengine: 0,5 bedeutet auf beiden Prozessoren unterschiedliche Genauigkeit.
- Solange ein aktives Profil existiert, ist der Wunsch erfüllbar. Ohne aktives Profil scheitert der erste Task mit „kein aktives Profil“ und sperrt das Modell für diese Subengine. Spätere Tasks scheitern sofort. Ungültige Wünsche sperren nicht. Die Sperre ist aufhebbar mit Rückmeldung. Alles nebenläufig sicher, eine vergiftete Sperre führt nicht zum Ausfall.

### Persistenz der Kalibrierung

- Ergebnisse je Modell und Subengine mit Fingerabdruck, Profilen (Position und Konfiguration als Text), Instabilitätsmarke, verworfenen und beschnittenen Kombinationen mit Grund.
- Fingerabdruck: Schemaversion, Subengine, Grafikprozessor-Name, Rechenfähigkeit und Treiberversion, Zentralprozessor-Merkmale, Modellkennung, Engine-Version. Bewusst knapp, ein Löschen der Datei erzwingt Neukalibrierung. Ermittlung zur Laufzeit, Grafikdaten über die Treiberschnittstelle. Fehlende Felder werden beim Einlesen mit Standardwerten belegt.
- Zustände: fehlend (präziseste gültige Kombination mit Warnung oder ausdrücklicher Kalibrierbefehl), kalibriert (laden und verwenden), veraltet (altes Ergebnis nutzen, neues entsteht im Hintergrund). Das Auflösen beim Start scheitert nie, unlesbare Dateien gelten als fehlend.
- Ablage mit Laden und atomarem Speichern: lesbare strukturierte Textdatei je Modell und Subengine, fehlende Verzeichnisse werden angelegt, Schreiben über prozessspezifische temporäre Datei mit Sicherung und Umbenennen.
- Neukalibrierung läuft im Hintergrund auf dem Pool niedriger Priorität, ersetzt das Ergebnis atomar und protokolliert den Austausch.

### Hintergrundausführung und Statistik

- Pools niedriger Priorität mit Namen und konfigurierbarer Threadzahl (mindestens eins). Auf Linux wird die niedrigste Planungspriorität gesetzt, sonst protokolliert.
- Aufträge liefern einen Ergebnisbehälter, blockierend oder nicht blockierend abfragbar. Abstürze beenden den Worker nicht. Beim Beenden laufen eingereichte Aufträge zu Ende.
- Schätzer Median und Minimum (Last verlangsamt nur). Streuung als Variationskoeffizient oder Spannweite durch Median, ohne das langsamste Fünftel, unendlich bei zu wenigen Proben oder nicht positivem Mittelpunkt.

## Addons

- Ein Task kann den Scope eines Folgetasks treiben. Iteration und Rekursion entstehen allein durch ein Addon in einer Subengine, der Anwendung oder einer Ressourcenimplementierung (Index, Modell).
- Ein Addon reiht Tasks über den normalen Weg ein, mit kopierten oder geänderten Parametern. Gezogen werden sie über die vorhandenen Pull-Filter (Priorität, Affinität).
- Addon-Tasks sind von Anwendungs-Tasks nicht unterscheidbar. Der Kern braucht dafür nur das erweiterbare Parameterobjekt.
- Beispiele: ein Zentroid-Task gegen die Zusammenfassungsstruktur mit einem Folgetask über die Mitglieder der gewählten Zellen; eine Textanfrage als Embedding-Task gefolgt von einem Similarity-Task; ein Reranking als zweite Similarity auf dem verengten Scope.
