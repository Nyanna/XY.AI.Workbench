# TODO
* LT englisch installieren
* elicitation is supported by claude

## TODO - MCP Controller
* man kann permission permissions direkt über den tool hook abfangen und dort den input prüfen
	* CLI hooks sind fast besser als MCP, nur bash stört und das die scripte kein python sind
	* Agent definition ließe sich auch nativ über CLI argumente und umgebungsvariablen aus eclipse heraus abbilden lassen
	* allein ein MCP für die tool use und definitionen wäre dann notwendig, mcp per cli übergeben statt tools
		* "tool inject" und  "tool use (permission)" sind dann nur 2 schnittstellen von derselben konfiguration ausgehend
			* nur eine Tooldefinition, redirect ist ein spezialfall
		 	* nur die schwierigkeit der agent schnittstelle gut zu emulieren
		 	* agentenprofile sind dann obsolete und nur veränderbare presets
	* permission check deaktiviert tool use ist für permission verantwortlich
* Tree viewer für kontext cache verwaltung und laufzeit von cache
* Eigenes read tool mit redirect wenn Datei schon gelesen, unverändert und im kontext
* Die user tool input JSON benutzen um dateinhalt proaktiv einzufügen
* Change set virtuell im MCP cache, dateiänderung im MCP cahce und erst beim commit anwenden, oder checkpoints und versioning pro datei, vielleit in memory git

## TODO - Workbench

! Tool usage loogen
	#: Tool:
	* thinking mit extra Zeile
	#: Thinking
	* beide anders einfärben
! reasoning disabled flag benutzen um es abzuschalten mit umgebungsvariable
! LT-Spellcheck support line listener in current line mark red only
	* dann für claude hook mit umgebungsvariable deaktivieren
	* startscript auch kein LT start oder shutdown dann
* Bessere Tool Loops anzeigen, problem mit nicht gespeicherter datei?
	*datei bei tag replace forced speichern? Tag aus dem laufenden Editor filtern als fallback? Ohne index
	* vielleicht über tool use zyklen gehen
* workbench, session branching und prefix cache support
	* anzeige wie alt cache und zustand
	* Workbench muss resume unterstützen für chats und cache /resume <session> command
	
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro zeile
	* Zeichen | mit offset an maxlength pro spalte ändern
	* exten "---" grey the whole line?
* Workbench support for Glossar : syntax mit Formatierung, maybe linespacing oder farbe in grau
* update alte api key  model and model parameters -> fetch from models API and only report missing feature support
* docs aufarbeiten


## TODO - Agents

* Stop and correct mode
	* PostToolBatch kann nach einer tool ausführung anhalten, so wie PermissionRequest auch
	*ask the user back tool, agent should have the possibillity to ask for user inforamtion when more efficient, enable ask back by plugin and do research before
	* alternativ stop pattern und einhacken wenn falsch gerichtet
* python agent ein python/ typescript(remark) sprachmodell geben/LSP/syntax parser/lint/prettier, als preproccessor, lanugageserver für python3
	* hm nur über hooks vor und nach tool usage nutzbar oder via MCP
	* Maybe produce something like skill as ouput so main kontext kann use scripts without knowing them in detail
* Rag server tool bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?
	* is this agent use correct?
	* violates this some target or increases costs or is meaningless?
	* is this request good or this tool usage?
	* Tool hook can be used to intercet and control

## Findings

* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Empty feault agent allready produces 6843 cache write token with /context reporting 31/200k

## Ideas

* in thought fragen abfangen und beantworten, mehr zwischenschritte zum einhaken
	* idee: ich habe bei recherche nur den initialprompt, der ezeugt suchanfragen an exa im günstigen agent, die anwot der tools kann ich brauchen die zusammenfassung aber nicht. Mit MCP COntroller kann ich nach der Toolantwort abbrechen und token sparen. Alternativ starte ich einen neuen prompt für die zusammenfassung mit allem input
* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* AI self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind