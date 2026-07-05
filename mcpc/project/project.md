## Architektur

* configuration on connect oder per agentendefinition
* Tool hook intercept inklusive redirect, permission und stop

* recursiv agenten als tool starten per agentendefinition oder on connect 
	* Subgenten als tool wrapper, unterstützt syntax und subsession mit resume
* proxy für End MCP
	* Ersetzt toolbeschreibungen durch eigene
	* ist selbst client
* Controllagent ewndpunkt ist nachrichtenendpunkt für eclipse, holt dort nachrichten ab die ebenfalls einen loop brauchen (wenn der agent vom MCP blockiert wird liefert dieser eine nachricht, antwort geht dann zum MCP und Claude returned)

## Struktur
 * /hook endpunkt
 	* tool
 	
 
### Agent

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