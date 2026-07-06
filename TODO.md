# TODO

## TODO - MCP Controller

[mcpp/project/TODO.md]()

## TODO - Workbench

* tool konfiguration und "X-MCPC-TOOLS" implementieren
* in preset tab presets aus unterverzeichnis anzeigen und on click laden
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

* tool für science research wie google scholar
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
* Parralele Agents bringen garnichts wenn man das ergebnis ohnehin stundenlang verifiziert
	* Unterscheidung gleich unterbechen für wichtige sachen im folgekontext
	* spätere unwichtigere korrekturen auf bassis des finalen kontextes
* Empty feault agent allready produces 6843 cache write token with /context reporting 31/200k
* Die sessionmanager wäre besser gelaufen wenn ich instream eingegriffen hätte.
	* Dateiset und dateien vorher benennen oder durch top Level Plan ergründen, 2 Stufig wäre besser gewesen

## Ideas

* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* AI self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind