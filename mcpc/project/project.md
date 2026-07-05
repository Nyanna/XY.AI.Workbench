# MCP Controller

## Architektur

* 1 MCP entpunkt
* Tool-Registry
* Per Agent-Configuration
* logging aller kommunikation
* logging von tokens
* permission controller -> redirect über permission hook (MCP hook https://code.claude.com/docs/en/hooks#mcp-tool-hook-fields)
* spell check controller -> redirect über prompt hook (MCP hook)
* Tool hook ist obsolete -> MCP unterstützt agentendefinition dennoch Tool hook für redirect (MCP hook)
* Agenten als tool wrapper, unterstützt syntax und subsession mit resume
	* Tool beschreibung muss statefull liefern für den hauptagenten
	* Subagenten wieder durch MCP getunnelt
* proxy für End MCP
	* Ersetzt toolbeschreibungen durch eigene
	* ist selbst client
* detailliertes sessionptotocoll und logging
* statusline endpunkt
	* logt weitere details im sessionprotokoll, JSON logformat
* selber session ID generiern und session start script generiert session und gibt in umgebungsvariable


* Control Endpunkt
* Control Clients und observatio
* Split console setup
* Multiple control clients, können nach agent/session filtern

## Test

claude-work default -p "Sag mal Hallo" --verbose --include-hook-events --output-format stream-json --include-partial-messages


## Struktur
 * streamable HTTP endpunkt "mcp"
 * JSON-RPC 2.0 abstraktions
 * hook endpunkt
 	* spell-check
 	* tool-use
 	* permission
 	* statusline -> relay script
 * endpunkt control
 	* streamable HTTP mit JSON-RPC 2.0
 	
## Idea

CLI rewind with session file snapshot and restore before resuming session
 	
## Copy/Notes

* MCP controller
	* eigenes SUbagent tool, wie funktioniert das agent tool replizieren, auch interface, auch andere agents und KI möglich
	* Alle commands, bash, python markdown, cli können vom MCP controller abgebildet werden.
		* Tools wie Bash, Read, Write, Edit, Agent haben 5k Token, das meiste in den Beschreibungen -> session/tools_systemprompt.md -> mit MCP Controller tools ersetzen
		* generische bash tool kompplet durch MCP controller ersetzen, intuition fällt ständig darauf zurück* generische bash tool kompplet durch MCP controller ersetzen, intuition fällt ständig darauf zurück
	* Stream responses, Use --output-format stream-json with --verbose and --include-partial-messages
	* MCP timeout via millisecond setting in MCP config
	* MCP controller supports resume and optimizes prompt cache with hundreds of subsessions within an hour (--resume)
	* Der Ausgabestrom wird parallel im Unterverzeichnis "sessions/" mitgeschrieben. Der Dateiname beginnt mit dem Timecode im Format "YYMMDD.HHMMSS", gefolgt von dem Namen des Agents und einer zufälligen Hash aus 4 Hexadezimalzeichen.
	
* MCP Controller tool selector <- session start --plugin parameter überschreibt agent tools, tool pro prompt aktivieren

* Wenn ich einen MCP controller habe. Brauche ich dann noch eine aufrufende container session oder ist koordination nicht ein subagent mit gecachtem prompt?
	* ein MCP controller erlaubt Infinite subagents, infinite recursion und separate permissions steuerung
