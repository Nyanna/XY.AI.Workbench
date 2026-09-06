# Session Manager

Implementiere einen Session-Manager. Die Zielsprache von Code-Kommentaren und Texten ist englisch.  

## Kontex

Aktuell startet der Connector einen CLI-Prozess der im Hintergrund läuft und den er bei Bedarf beendet.
Dies soll geändert werden. Ein separater Session-Manager soll parallel eine Vielzahl von CLI-Prozessen verwalten, bei Bedarf Starten oder Beenden.
Ein eigenes Eclipse-Panel soll die aktiven Sessions und Prozesse in Echtzeit anzeigen.

## Referenzen

* Eclipse-Plugin Projekt-Root: `/home/user/xyan/xy.ai.workbench/`
* Connector Implementierung: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java`

## Connector

Der Connector verliert seine Verantwortlichkeit für die CLI.

* Das Starten der CLI wird vom Manager übernommen.
* Der Connector stellt beim Manager eine Anforderung und erhält ein Session-Objekt.
* Die Anforderung stellt er auf Basis der Promptinformation und eine im Panel eventuell ausgewählte Session-UUID.
	* Die Übergabe beider Komponenten ist wichtig damit der Manager die Kompatibilität prüfen kann.
* Der Connector ist nicht mehr für die Replikation des Ausgabestroms verantwortlich.
* Der Connector befüllt das Feld "lastParsedMessage" der Session mit dem letzten Eintrag aus `assistantEvents` während er die Ausgabe überwacht.

### Resume Kommando

Ein neues Kommando "/resume <UUID>", wird in die Vorverarbeitung eingefügt.
Bei dessen Aufruf wird eine Session vom Manager mit den aktuellen Prompt-Parametern angelegt aber nicht gestartet.
Der Promptversuch wird mit einer AIAnswer und dem Text "Session created" quittiert.
Dies dient dem Import von extern initiierten Sessions.

## Manager

Der Manager ist zentrale Verwaltung der Sessions. Er hält einen Index, erzeugt Sessions oder fährt die Herunter.

* Der Manager startet oder beendet selbst keine CLI-Session, sondern verwaltet die Sessions und aktualisiert das Panel.
* Bei einer Anforderung prüft der Manager die Gültigkeit aktiver Session und fordert abgelaufen Session auf die CLI zu terminieren.
* Ungültige terminierte Sessions werden entfernt. Beendete gültige Session können noch fortgesetzt werden.
* Der Manager lauscht auf Änderungen seiner Sessions und aktualisiert das Panel
* Eine Session kann anhand der Parameter-Hash aufgelöst werden oder anhand der Session-UUID nachdem sie einmalig gestartet wurde. 

### Session Hash

Der Manager generiert aus den angeforderten Parametern eine deterministische Hash.

* Parameter für die Hash: Modell, Effort, Thinking, Agentendefinition `cfg.getProfile().name`, Claude Profil `--profile`


### Bestehende Session-Anforderung

Wird eine bestehende Session mittel übergeben UUID angefordert so Prüft der Manager auf die bestehende Session, deren Gültigkeit und Kompatibilität. Eine Session kann abweichende Settings, Prompts oder einen bestehenden Kontext haben, der sie für die eingehende Anforderung inkompatibel macht.

* Stimmt die Hash der Parameter mit der bestehenden Session nicht überein wird ein Fehler gemeldet.

### Neue Session-Anforderung

Wird eine neue Session angefordert so erstellt der Manager ein neues Session-Objekt, trägt dieses ein und gibt es zurück.


### Session-Objekt

Die Session sichert die optimale Nutzung des CLI-Session-Caches.

* Die Session stellt ein Container-Objekt dar und enthält Referenzen auf den CLI-Prozess, STDIN, STDOUT, STDERR und den Manager.
* Die Session meldet interne Änderungen dem Manager für die Aktualisierung des Panels
* Beim erstmaligen Start der CLI, generiert die Session aus den Parametern eine UUID und übergibt diese mittels `--session-id` der CLI.
* Ein eingehender Prompt startet eine noch nicht gestartete oder gültige Session.
* Die Session speichert den Startzeitpunkt, den Zeitstempel der letzten gesendeten und empfangenen Nachricht.
* Der Zeitstempel der letzten gesendeten Nachricht bestimmt die TTL/Gültigkeit
* Eine Session verliert nach 1 Stunde ohne Nutzung ihr Gültigkeit
* Ein Prompt-Versuch in einer ungültigen Session, beendet eine noch laufende CLI und meldet dann einen Fehler.
* Die Session startet und beendet die CLI und kann `--resume <SessionID>` verwenden um eine bestehende CLI-Session wieder aufnehmen.
* Die Session speichert die ersten 100 Zeichen des initialen Prompts. Zeilenumbrüche werden aus dem Text entfernt.
* Ein Prompt, der an die CLI gesendet wird, setzt die verbleibende Lebenszeit wieder auf 1 Stunde zurück.
* Die Session ist für die Replikation des Ausgabestroms verantwortlich.
* Das Session-Objekt delegiert die Aufrufe des Connector auf STDIN und STDOUT.
* Das Session-Objekt enthält das Feld "lastParsedMessage" das von Connector befüllt werden kann und zur Statusanzeige dient.

## Panel (ViewPart)

Das Panel ist eine `ViewPart` Implementierung und wird mit dem Plugin registriert. Es zeigt eine Tabelle der aktuellen Sessions des Managers und ihren Status.

* Das Panel hat nur einen Bereich
* Das Panel hat eine Toolbar
* Tabelle wird auf Basis eines `TableViewer`implementiert.

### Toolbar

Die Toolbar enthält Aktionen, die sich auf die ausgewählten Sessions beziehen.

* Beenden: Fordert den Manager auf die Selektierten Session zu beenden
* Eine beendete Session ist noch gültig und wird vom Manager daher noch nicht entfernt.


### Tabelle

In der Tabelle ist die Selection jeweils eines Eintrages möglich. Die initiale Breite der Spalten ist an zu erwartenden Inhalt angepasst. 

Tabellenspalten:

Session-UUID
: Enthält die Parameter-Hash bis zum erstmaligen Start und danach die Session-UUID

Zustand
: Enthält den aktuellen Zustand. Dieser ist nach Priorität: Abgelaufen, Prompt, Bereit, Erzeugt

* Abgelaufen: sobald die Lebenszeit der Session erreicht ist.
* Prompt: wenn der Connector aktuell den STDIN oder STDOUT der Session verwendet.
* Bereit: Wenn der CLI-Prozess gestartet wurde.
* Erzeugt: Wenn die Session erzeugt wurde und aktuell kein CLI-Prozess gestartet ist oder die Session noch nie initialisiert wurde.

TTL
: Verbleibende Lebenszeit der Session in Minuten

Modell
: Modell-Parameter der Session

Effort
: Effort-Parameter der Session

Prompt
: Wenn aktuell nicht in Verwendung enthält die Spalte den Text des initialen Prompt der Session.
: Ist die Session in Verwendung, enthält die Spalte die `lastParsedMessage`




# Corrections

Die Parameter "String modelApiName, String reasoningName, String agentProfileName, String cliProfile" können alle strikt typisiert mittels ihres Enums behandelt werden.

Refactored myself









	