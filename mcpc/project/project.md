# Tasks
* test von subagent und mcp

* tool use and stop pattern
	* Wrapper um den Tooolaufruf blockiert schreibt in session state
	* Tool hook intercept inklusive redirect, permission und stop
	* REST control endpunkt prüft auf tool use und liefert liste von tool use requests
	* alter deny wird umgebraut, selber befehl andere ziel und quelle /allow 2342423 und /deny 234234 reason
		/ control abfrage returned immer zuerst und braucht neuen prompt
		* es könnte mehr als ein tool use da liegen, dann alle im selben loop beantworten
		*  tool use ist #: mit vorgewählten /allow 234234 
	* endpunkt /hook/tools und /control/tools
	* Controllagent ewndpunkt ist nachrichtenendpunkt für eclipse, holt dort nachrichten ab die ebenfalls einen loop brauchen (wenn der agent vom MCP blockiert wird liefert dieser eine nachricht, antwort geht dann zum MCP und Claude returned)
	* tool aufrufe verändern können um den kontext zu reduzieren: Bash(find /home/user/xyan/xy.ai.workbench/src -type f -name "*.java" | sort) => zu viel im Kontext
	* auch tool result return reporten und abbrechbar machen, wenn sich das ergebnis bereits reicht, token für zusammenfassung sparen, manchmal sind die search prompts ausreichend. Dann search und zusammanefassung extra
		* In eclipse result anzeigen an claude aber nur erfolgreiche durchführung melden
 	
* Block diff tool implementieren
* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove

# MCP-Tools
Implementiere Analog zu Exa entsprechende Brücken zu Github und Context7.
In der Konfiguration sind die Felder für URL und API-Keys bereits angelegt.

## Github

Die Tool-Dokumentation liegt hier`/home/user/xyan/xy.ai.workbench/mcpc/project/github-api.md`.
Es werden nur Tools benötigt, die mit reinem Lesezugriff auskommen und Recherchezwecken dienen. Hauptsächlich der Abruf von Code und Dateien, Issues, Discussions, Projektinformationen.

## Context7

Implementiere für Context7 die beiden hier`/home/user/xyan/xy.ai.workbench/mcpc/project/context7.md` Dokumentierten Tools.

## Tools Checkliste

* Inputschema vorhanden
* Outputscheme vorhanden
* Result: structuredContent verwendet, nicht content