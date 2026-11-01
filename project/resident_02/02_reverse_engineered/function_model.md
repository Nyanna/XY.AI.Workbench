# Funktionale Anforderungen Model

## Stapelbildung für Eingabesequenzen

**Eingabe und Ergebnis**
- Das System muss eine Liste von Token-Sequenzen entgegennehmen und daraus eine Liste von Stapeln bilden.
- Jede Eingabesequenz muss in genau einem Stapel enthalten sein.
- Jeder Stapel muss die ursprünglichen Positionen seiner Zeilen in der Eingabereihenfolge des Aufrufers mitführen, damit der Aufrufer die Ergebnisse seiner Eingabereihenfolge zuordnen kann.
- Bei leerer Eingabe muss eine leere Stapelliste entstehen.

**Konfiguration**
- Die Stapelbildung muss über drei Parameter steuerbar sein: eine Obergrenze für die Token-Anzahl je Stapel (Zeilenanzahl multipliziert mit Sequenzlänge), eine maximale Sequenzlänge und die Kennung des Auffüll-Tokens.

**Kürzung und Längenklassen**
- Sequenzen, die länger als die maximale Sequenzlänge sind, müssen auf diese Länge gekürzt werden.
- Die aufgefüllte Länge einer Sequenz muss der kleinsten Längenklasse entsprechen, die mindestens so groß ist wie die (gekürzte) Sequenzlänge.
- Die festen Längenklassen lauten 32, 64, 128, 256 und 512.
- Ist die Sequenz länger als die größte Längenklasse, muss die maximale Sequenzlänge verwendet werden.
- Die aufgefüllte Länge darf die maximale Sequenzlänge nie überschreiten und muss mindestens 1 betragen.

**Gruppierung**
- Die Sequenzen müssen aufsteigend nach ihrer ursprünglichen Länge sortiert werden. Bei gleicher Länge bleibt die Eingabereihenfolge erhalten.
- Die Sequenzen werden in dieser Reihenfolge nacheinander einem laufenden Stapel hinzugefügt.
- Vor jedem Hinzufügen muss geprüft werden, ob die Token-Anzahl des Stapels mit der neuen Zeile die Obergrenze überschreiten würde. Maßgeblich ist dabei die aufgefüllte Länge der neuen Zeile, die wegen der Sortierung die größte im Stapel ist. In diesem Fall muss der laufende Stapel abgeschlossen und ein neuer begonnen werden.
- Eine einzelne Sequenz, deren aufgefüllte Länge die Obergrenze allein überschreitet, muss einen eigenen Stapel bilden.
- Die Sequenzlänge eines Stapels entspricht der aufgefüllten Länge seiner längsten Zeile. Alle Zeilen eines Stapels haben dieselbe Länge.

**Stapelinhalt**
- Jeder Stapel muss die Token-Kennungen zeilenweise hintereinander als zweidimensionales Feld (Zeilen mal Sequenzlänge) enthalten.
- Jede Zeile muss am Ende mit dem konfigurierten Auffüll-Token auf die Stapel-Sequenzlänge aufgefüllt werden.
- Zusätzlich muss jeder Stapel eine gleich geformte Maske enthalten, die für echte Token den Wert 1 und für Auffüllpositionen den Wert 0 trägt.
- Der Stapel muss seine Zeilenanzahl angeben können.

**Serialisierung**
- Ein Stapel muss sich in einen Byte-Block überführen lassen, der zuerst alle Token-Kennungen und danach die gesamte Maske enthält.
- Jeder Wert wird als vorzeichenlose 32-Bit-Ganzzahl in Little-Endian-Reihenfolge abgelegt.
- Der Block entspricht damit einem dreidimensionalen Feld der Form zwei mal Zeilen mal Sequenzlänge.

## Modellbeschreibung und Validierung

**Inhalt**
- Das System muss ein Modell durch eine reine Beschreibung darstellen, die keine Ressourcen besitzt. Sie enthält:
  - die Modellkennung und die Architekturkennung,
  - die Einbettungsdimension,
  - das Pooling-Verfahren und die Normalisierungsart,
  - Präfixe für Suchanfragen und für Passagen,
  - den Speicherdatentyp der erzeugten Einbettungen,
  - die Rechendatentypen des Hauptmodells, getrennt für Hauptprozessor und Grafikprozessor,
  - die maximale Sequenzlänge,
  - eine optionale Projektionsschicht,
  - die Pfade zur Konfigurationsdatei und zu den Gewichtsdateien.
- Der Speicherdatentyp muss unabhängig von den Rechendatentypen wählbar sein.
- Der Rechendatentyp muss anhand der Geräteklasse (Hauptprozessor oder Grafikprozessor) ermittelt werden können.

**Vordefiniertes Modell**
- Das System muss eine Vorgabe für das Modell „intfloat/multilingual-e5-small“ bereitstellen, die auf ein Verzeichnis und einen wählbaren Speicherdatentyp aufsetzt. Die Werte lauten:
  - Architektur „bert“,
  - Dimension 384,
  - Mittelwert-Pooling über unmaskierte Token,
  - euklidische Normalisierung,
  - Präfix „query: “ für Anfragen und „passage: “ für Passagen,
  - maximale Sequenzlänge 512,
  - Rechendatentyp 32-Bit-Gleitkomma auf beiden Geräteklassen,
  - keine Projektionsschicht,
  - Konfigurationsdatei „config.json“ und Gewichtsdatei „model.safetensors“ im angegebenen Verzeichnis.

**Validierung**
- Dimension und maximale Sequenzlänge müssen größer als null sein.
- Ein nicht leeres Präfix muss mit einem Leerzeichen enden.
- Bei einem Verstoß muss ein Fehler der Kategorie „inkompatible Modellkonfiguration“ mit einer Begründung gemeldet werden. Die Validierung dient dazu, Konfigurationen abzulehnen, die die Einbettungsqualität unbemerkt verschlechtern würden.

## Modellregister und Laden

- Das System muss ein Register führen, das Architekturkennungen auf Konstruktionsfunktionen für Encoder abbildet.
- Es muss möglich sein, eine Konstruktionsfunktion für eine Architekturkennung zu registrieren. Eine erneute Registrierung derselben Kennung ersetzt die bisherige.
- Es muss ein Register mit allen in den jeweiligen Build kompilierten Architekturen erzeugt werden können. Die Architektur „bert“ ist nur enthalten, wenn die entsprechende Backend-Unterstützung aktiviert wurde.
- Beim Laden eines Modells für eine Geräteklasse muss das System in dieser Reihenfolge vorgehen:
  - die Modellbeschreibung validieren,
  - die Konstruktionsfunktion zur Architekturkennung suchen,
  - den Encoder konstruieren.
- Ist die Architektur nicht registriert, muss ein Fehler „nicht unterstützte Architektur“ mit der Kennung gemeldet werden.
- Das Ergebnis muss ein geladener Encoder sein, der Beschreibung, Geräteklasse und Encoder bündelt und lesenden Zugriff darauf bietet.
- Der geladene Encoder muss als verwaltbare Ressource auftreten:
  - Sein Schlüssel setzt sich aus Architekturkennung und Geräteklasse zusammen.
  - Seine Initialisierungskosten sind mit dem Wert 100 angegeben.
  - Das Freigeben des geladenen Encoders muss die Gewichte entladen.

## Laden des Transformer-Hauptmodells (Architektur „bert“)

**Prüfungen**
- Das System muss die Konfigurationsdatei lesen und als strukturierten Text auswerten.
- Die Modelltyp-Angabe in der Konfigurationsdatei muss „bert“ lauten. Fehlt sie oder weicht sie ab, muss ein Fehler „inkompatibel“ gemeldet werden, der den gefundenen Typ nennt.
- Die versteckte Dimension aus der Konfigurationsdatei muss mit der Dimension der Modellbeschreibung übereinstimmen.
- Die maximale Sequenzlänge der Beschreibung darf die maximale Anzahl der Positionseinbettungen aus der Konfigurationsdatei nicht überschreiten.
- Bei einem Verstoß muss ein Fehler „inkompatibel“ mit beiden Werten gemeldet werden.
- Kann die Konfigurationsdatei nicht gelesen oder ausgewertet werden, muss ein Ladefehler mit Begründung gemeldet werden.

**Gerät und Datentyp**
- Für den Hauptprozessor muss das Modell stets ladbar sein.
- Für den Grafikprozessor darf das Modell nur geladen werden, wenn die Grafikprozessor-Unterstützung in den Build einkompiliert ist. Es wird dann das erste Grafikgerät verwendet.
- Andernfalls muss ein Fehler „inkompatibel“ gemeldet werden, der auf die fehlende Grafikprozessor-Unterstützung hinweist.
- Der Rechendatentyp muss aus der Beschreibung für die gewählte Geräteklasse entnommen und in den Datentyp des Backends übersetzt werden. Ist er dort nicht darstellbar, muss ein Fehler „inkompatibel“ gemeldet werden.

**Gewichte**
- Die Gewichte müssen aus den angegebenen Safetensors-Dateien direkt aus dem Speicher abgebildet (ohne vollständiges Kopieren) und im gewählten Datentyp auf das gewählte Gerät geladen werden.
- Fehlt ein erwarteter Gewichtstensor im Checkpoint, muss ein Fehler „Gewichtsname passt nicht“ gemeldet werden, der den Namen des fehlenden Tensors enthält.
- Alle übrigen Ladefehler müssen als Ladefehler mit Begründung gemeldet werden.

**Vorwärtsberechnung**
- Das Hauptmodell muss zu einem Stapel aus Token-Kennungen und Maske, jeweils der Form Zeilen mal Sequenzlänge, die Zustandsvektoren aller Token berechnen.
- Alle Tokentyp-Eingaben müssen dabei den Wert null haben. Die Maske muss als Aufmerksamkeitsmaske berücksichtigt werden.
- Das Ergebnis muss unabhängig vom Rechendatentyp als flache Liste von 32-Bit-Gleitkommazahlen in der Form Zeilen mal Sequenzlänge mal Dimension zurückgegeben werden.
- Schlägt die Berechnung fehl, muss ein Fehler „Kodierung fehlgeschlagen“ mit Begründung gemeldet werden.
- Das Hauptmodell muss seine Ausgabedimension bekanntgeben.
- Zwischen dem Hauptmodell und dem restlichen Verarbeitungsablauf dürfen keine Typen des Backend-Frameworks ausgetauscht werden. Die Schnittstelle arbeitet ausschließlich mit einfachen Zahlenlisten.

## Aufbau des Encoders aus der Modellbeschreibung

- Das System muss aus der Modellbeschreibung und einer Geräteklasse einen vollständigen Encoder bauen. Dieser umfasst das Hauptmodell, die optionale Projektionsschicht, Pooling, Normalisierung und Speicherdatentyp.
- Ist eine Projektionsschicht beschrieben, muss sie aus der ersten angegebenen Gewichtsdatei geladen werden. Gibt es keine Gewichtsdatei, muss ein Ladefehler „keine Gewichtsdateien“ gemeldet werden.
- Die Normalisierungsart der Beschreibung muss übernommen werden.
- Die Geräteklasse und der Rechendatentyp müssen für die Nachverfolgung hinterlegt werden.
- Ohne ausdrückliche Angabe muss die euklidische Normalisierung die Standardeinstellung des Encoders sein.
- Beim Aufbau muss geprüft werden, dass die Eingabedimension der Projektionsschicht der Ausgabedimension des Hauptmodells entspricht. Andernfalls muss ein Fehler „inkompatibel“ mit beiden Werten gemeldet werden.

## Projektionsschicht

- Das System muss eine optionale lineare Projektion unterstützen, die jeden Token-Vektor von der Eingabedimension auf die Ausgabedimension abbildet. Sie dient insbesondere Modellen mit Late-Interaction-Verfahren.
- Die Gewichtsmatrix hat die Form Ausgabedimension mal Eingabedimension. Ein Bias-Vektor der Länge Ausgabedimension ist optional.
- Die Schicht muss aus einer Safetensors-Datei geladen werden können, wahlweise aus einem Byte-Block im Speicher oder aus einer Datei, die speicherabgebildet gelesen wird.
- Der Gewichtstensor wird über den in der Beschreibung genannten Namen gefunden.
- Der Name des Bias-Tensors ergibt sich, indem die Endung „weight“ des Gewichtsnamens durch „bias“ ersetzt wird. Endet der Name nicht auf „weight“, wird „bias“ an den vollständigen Namen angehängt.
- Fehlt ein Tensor, muss ein Fehler „Gewichtsname passt nicht“ mit dem erwarteten Namen gemeldet werden.
- Weicht die Form eines Tensors von der erwarteten ab, muss ein Fehler „inkompatibel“ mit tatsächlicher und erwarteter Form gemeldet werden.
- Gelesene Werte dürfen als 32-Bit-Gleitkomma, 16-Bit-Gleitkomma oder 16-Bit-Brain-Gleitkomma vorliegen und müssen in 32-Bit-Gleitkomma umgewandelt werden. Jeder andere Datentyp führt zu einem Fehler „inkompatibel“, der Tensor und Datentyp nennt.
- Kann die Datei nicht geöffnet, abgebildet oder interpretiert werden, muss ein Ladefehler gemeldet werden.
- Die Anwendung muss jeden Eingabevektor auf das Skalarprodukt mit der jeweiligen Gewichtszeile abbilden und, falls vorhanden, den passenden Bias-Wert addieren.
- Eingaben bestehen aus aufeinanderfolgenden Vektoren der Eingabedimension. Die Ausgabe enthält die entsprechenden Vektoren der Ausgabedimension in derselben Reihenfolge.
- Eingabe- und Ausgabedimension müssen abfragbar sein.

## Pooling und Normalisierung

**Pooling-Verfahren**
- Das System muss zwei Pooling-Verfahren unterstützen:
  - Mittelwert über unmaskierte Token, also ein Vektor je Sequenz,
  - kein Zusammenfassen, also ein Vektor je Token.
- Beim Mittelwert-Pooling muss je Sequenz die Summe der Zustandsvektoren aller Positionen mit Maskenwert ungleich null gebildet und durch deren Anzahl geteilt werden.
- Auffüllpositionen dürfen das Ergebnis nicht beeinflussen.
- Ist eine Sequenz vollständig maskiert, muss ein Nullvektor entstehen. Es darf keine Division durch null auftreten.
- Beim Verfahren ohne Zusammenfassen werden die Zustandsvektoren aller Positionen unverändert weitergegeben, einschließlich der Auffüllpositionen.

**Euklidische Normalisierung**
- Jeder Vektor wird zeilenweise auf die Länge eins gebracht, indem er durch seine euklidische Norm geteilt wird. Das Skalarprodukt zweier Vektoren entspricht dann dem Kosinus ihres Winkels.
- Zur Vermeidung einer Division durch null wird die Norm nach unten auf den sehr kleinen Wert 10 hoch minus 12 begrenzt.
- Vektoren, deren Norm unter diesem Wert liegt, gelten als degeneriert und bleiben praktisch ein Nullvektor.
- Die Normalisierung muss die Anzahl der degenerierten Vektoren zurückmelden.
- Die Normalisierungsart „keine“ lässt alle Vektoren unverändert.

## Schreiben der Ergebnisse in den Ausgabespeicher

- Das System muss die Ergebnisvektoren falls gewünscht euklidisch normalisieren und anschließend in den Datentyp des Ausgabespeichers umwandeln.
- Danach muss es sie lückenlos hintereinander in die Ausgabe-Speicherregion schreiben.
- Zulässige Ausgabedatentypen sind 32-Bit-Gleitkomma, 16-Bit-Gleitkomma und 16-Bit-Brain-Gleitkomma. Die Werte werden in Little-Endian-Reihenfolge abgelegt.
- Bei jedem anderen Datentyp muss ein Fehler „inkompatibel“ gemeldet werden, der den Datentyp nennt.
- Wurden bei der Normalisierung degenerierte Vektoren festgestellt, muss eine Warnung mit deren Anzahl protokolliert werden, dass leere Einbettungen als Nullvektor gespeichert wurden. Der Vorgang selbst gilt dennoch als erfolgreich.
- Fehler beim Schreiben in den Speicher müssen als Speicherfehler weitergereicht werden.

## Kodierung eines Stapels (Encoder-Schnittstelle)

**Abfragen**
- Der Encoder muss seine Ausgabedimension nennen. Sie entspricht der Ausgabedimension der Projektionsschicht, sofern vorhanden, sonst der Dimension des Hauptmodells.
- Der Encoder muss seinen Speicherdatentyp nennen.

**Vorab-Prüfungen**
- Der Encoder muss einen Stapel kodieren, dessen Eingabe als Speicherregion mit Layout übergeben wird. Die Region enthält zuerst die Token-Kennungen und danach die Maske, als vorzeichenlose 32-Bit-Ganzzahlen der Form zwei mal Zeilen mal Sequenzlänge. Dieses Layout entspricht dem Serialisierungsformat der Stapelbildung.
- Die Ausgabe wird ebenfalls als Speicherregion mit Layout übergeben.
- Der Encoder muss vor Beginn der Arbeit folgende Punkte prüfen und andernfalls einen Fehler „inkompatibel“ melden, der erwartete und tatsächliche Form beziehungsweise Datentyp nennt:
  - Die Eingabe hat genau die Form zwei mal Zeilen mal Sequenzlänge und den Datentyp vorzeichenlose 32-Bit-Ganzzahl.
  - Die Ausgabe hat bei Mittelwert-Pooling die Form Zeilen mal Dimension, bei Verarbeitung je Token die Form Zeilen mal Sequenzlänge mal Dimension.
  - Der Datentyp der Ausgabe entspricht dem Speicherdatentyp des Encoders.

**Verarbeitung**
- Die Kodierung muss asynchron auf einer übergebenen Ausführungsspur laufen. Der Aufruf kehrt nach dem Einreihen der Arbeit zurück und liefert ein Abschlussereignis.
- Die Arbeit selbst besteht aus diesen Schritten in dieser Reihenfolge:
  - Eingabe-Bytes aus dem Speicher lesen und in Token-Kennungen und Maske trennen,
  - Zustandsvektoren über das Hauptmodell berechnen,
  - die Projektionsschicht anwenden, falls vorhanden,
  - das konfigurierte Pooling ausführen,
  - das Ergebnis in die Ausgaberegion schreiben, einschließlich Normalisierung und Datentypumwandlung.
- Kann die Arbeit nicht eingereiht oder das Ereignis nicht erzeugt werden, muss der Fehler der Ausführungsumgebung als Geräte- oder Speicherfehler an den Aufrufer zurückgegeben werden.
- Tritt während der Ausführung ein Fehler auf, muss das System:
  - eine Warnung „Einbettungsstapel fehlgeschlagen“ mit der Fehlerursache protokollieren,
  - den Fehler im Abschlussereignis hinterlegen,
  - die Ausführungsspur selbst als erfolgreich abgeschlossen behandeln.

## Abschlussereignis

- Das Ereignis muss jederzeit abfragen lassen, ob die Arbeit abgeschlossen ist, ohne zu blockieren.
- Das Ereignis muss ein blockierendes Warten bis zum Abschluss unterstützen.
- Nach dem Abschluss muss das Warten einen gegebenenfalls aufgetretenen Verarbeitungsfehler zurückmelden.
- Der hinterlegte Fehler wird dabei entnommen. Bei einem weiteren Warten liegt er nicht mehr vor und es wird Erfolg gemeldet.
- Fehler der Ausführungsumgebung beim Warten müssen als Geräte- oder Speicherfehler gemeldet werden.

## Speicherzugriff

- Das System muss den Lese- und Schreibzugriff auf Speicherregionen einer Speicherdomäne über eine einheitliche Schnittstelle kapseln. Diese Schnittstelle liest die Bytes einer Region in einen Puffer und schreibt Bytes in eine Region.
- Die Schnittstelle muss von mehreren Ausführungsspuren gleichzeitig nutzbar sein.
- Der Speicherverwalter für den Hauptprozessor muss diese Schnittstelle bereitstellen.
- Eine Tensor-Referenz muss Speicherregion und Layout (Form und Datentyp) verbinden. Sie ist der einzige Weg, auf dem Tensoren die Encoder-Schnittstelle passieren.