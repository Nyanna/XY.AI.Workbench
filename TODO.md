# TODO

## TODO - MCP Controller

[mcpp/project/TODO.md]()

## TODO - Workbench

* Bessere Tool Loops anzeigen, problem mit nicht gespeicherter datei?
	* datei bei tag replace forced speichern? Tag aus dem laufenden Editor filtern als fallback? Ohne index
	
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro zeile
	* Zeichen | mit offset an maxlength pro spalte ändern
	* exten "---" grey the whole line?
* Workbench support for Glossar : syntax mit Formatierung, maybe linespacing oder farbe in grau
* update alte api key  model and model parameters -> fetch from models API and only report missing feature support
* Rewind support, in session, panel mit rebulld/reextraktion der session aus dem JSON, context rebuild
* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all


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
* Die sessionmanager wäre besser gelaufen wenn ich instream eingegriffen hätte.
	* Dateiset und dateien vorher benennen oder durch top Level Plan ergründen, 2 Stufig wäre besser gewesen

## Ideas

* elicitation is supported by claude
* in thought fragen abfangen und beantworten, mehr zwischenschritte zum einhaken
	* idee: ich habe bei recherche nur den initialprompt, der ezeugt suchanfragen an exa im günstigen agent, die anwot der tools kann ich brauchen die zusammenfassung aber nicht. Mit MCP COntroller kann ich nach der Toolantwort abbrechen und token sparen. Alternativ starte ich einen neuen prompt für die zusammenfassung mit allem input
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* AI self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind