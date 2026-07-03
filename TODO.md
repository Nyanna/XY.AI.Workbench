# TODO

## Tasks - MCP Controller

* LT englisch installieren

* Workbench benutzen können mit Claude CLI aufruf, muss resume unterstützen für chats und cache, speelcheck dann übergehen
* Workbench lokaler cli Agent Modus ohne key mit resume für cache, benutzt lokalen claude code, braucht extra panel, braucht kein Batch und diverse inputparameter entfallen, option panel muss dann wiederverwendet werden der mittlere teil
* Thinking in editor flag für wirkbench, zeigt extended thinking irgendwo für prompt qualität check
* Workbench support for Glossar : syntax mit Formatierung, maybe linespacing oder farbe in grau
* Lanugagetool in workbench einbauen
	* eclipse spell check -> language tool, in seitenpannel korrekturen nur selektierter text
* workbench, session branching und prefix cache support, bessere tool loops
* Tree viewer für kontext cache verwaltung und laufzeit von cache

* update model and model parameters -> fetch from models API and only report missing feature support
* Eigenes read tool mit redirect wenn Datei schon gelesen, unverändert und im kontext
* Die user tool input JSON benutzen um dateinhalt proaktiv einzufügen
* Change set virtuell im MCP cache, dateiänderung im MCP cahce und erst beim commit anwenden, oder checkpoints und versioning pro datei, vielleit in memory git

## Todo - Agents
! problem, tool zugriff ist zwar erlaubt permission aber nicht (decision.behavior) hook: PermissionRequest
* PostToolBatch kann nach einer tool ausführung anhalten
* elicitation is supported by claude
* der postproccessing task hat kein thinking verbraucht?!?
* thinking token anzeigen
	* es gibt bereits ein feld in der tabelle
	* sind wichtig um einzuschätzen wieviel thinking ein block braucht
	* am ende von einem thinking block einfügen #: Token: <thinking_token>
	* {"type":"stream_event","event":{"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null,"stop_details":null},"usage":{"input_tokens":8,"cache_creation_input_tokens":1196,"cache_read_input_tokens":10612,"output_tokens":758,"output_tokens_details":{"thinking_tokens":241}},"context_management":{"applied_edits":[]}},"session_id":"20e5b528-540b-40ea-8fcc-1cd038008a1a","parent_tool_use_id":null,"uuid":"2832ae72-7b2b-438c-b8fe-279c82b4fb22"}

! convert spell check block, shorter, remove original prompt:

UserPromptSubmit operation blocked by hook:
[${CLAUDE_PLUGIN_ROOT}/scripts/spell-check.sh]: 
.*


Original prompt: .*


*ask the user back tool, agent should have the possibillity to ask for user inforamtion when more efficient, enable ask back by plugin and do research before
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
	* stop oder intervention architektur. Je nach sweet spot 
* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* AI self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind
