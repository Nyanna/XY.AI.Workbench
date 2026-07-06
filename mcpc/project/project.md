## Architektur


* configuration on connect oder per agentendefinition
* Tool hook intercept inklusive redirect, permission und stop

* recursiv agenten als tool starten per agentendefinition oder on connect 
	* Subgenten als tool wrapper, unterstützt syntax und subsession mit resume
* proxy für End MCP
	* Ersetzt toolbeschreibungen durch eigene
	* ist selbst client
* Controllagent ewndpunkt ist nachrichtenendpunkt für eclipse, holt dort nachrichten ab die ebenfalls einen loop brauchen (wenn der agent vom MCP blockiert wird liefert dieser eine nachricht, antwort geht dann zum MCP und Claude returned)

* session init tool header, enthält erlaube tools für sich selbst

## Struktur

* tool use and stop pattern
	* Wrapper um den Tooolaufruf blockiert schreibt in session state
	* REST control endpunkt prüft auf tool use und liefert liste von tool use requests
	* alter deny wird umgebraut, selber befehl andere ziel und quelle /allow 2342423 und /deny 234234 reason
		/ control abfrage returned immer zuerst und braucht neuen prompt
		* es könnte mehr als ein tool use da liegen, dann alle im selben loop beantworten
		*  tool use ist #: mit vorgewählten /allow 234234 
	 * endpunkt /hook/tools und /control/tools
 	
* replace ersetzen durch replace-chars und replace-lines
	* wie ersetzt eine KI am effizientesten?

* kein agenten tool, agenten nsind normale agents, und muss nur ausgaben aggregiren, second level agent 
## Agententools
agt-python, agt-markdown, agt-web-research, agt-github-research

* input sind prompt, modell und effort
* bei CLI aufruf wird session tool header in MCP config übergeben
* eclipse agent bekommt config via header
* MCPC subagent liest agentendefinition erzeugt session und setzt tools direkt

## Direct Tools
### Python
### Markdown

### Github
### Exa
### Context7
 	
## Idea

* Die user tool input JSON benutzen um dateinhalt proaktiv einzufügen
* tool aufrufe verändern können um den kontext zu reduzieren: Bash(find /home/user/xyan/xy.ai.workbench/src -type f -name "*.java" | sort) => zu viel im Kontext
* Change set virtuell im MCP cache, dateiänderung im MCP cahce und erst beim commit anwenden, oder checkpoints und versioning pro datei, vielleit in memory git
* MCPC supports resume and optimizes prompt cache with hundreds of subsessions within an hour