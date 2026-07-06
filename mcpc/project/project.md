# Tasks
* agenten systemprompts setzen
*suchagenten toolnamen ändern zu den konkreten mcp tool namen von exa
* Adapt exa descriptions

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
 	
* replace ersetzen wie ersetzt eine KI am effizientesten? recherchieren, generell textbearbeitung KI.

# MCP-Tools
## Github
## Context7