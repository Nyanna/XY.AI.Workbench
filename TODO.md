# TODO

## Workbench
* Diff support für edit commands
	* diff editor in eclipse in memory aufrufen und toolausgabe mit action oder annotation versehen, "view as diff"

### Ideas
* in preset tab presets aus unterverzeichnis anzeigen und on click laden, inklusive tools
* subagenten mit hauptsession verknüpfen, control filter per filter parameter nach einem sessionbaum
	* load, save, select
	* selectionliste folgt aktivem editor projekt -> .presets mit auflistung absoluter oder relativer pfade?
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro zeile
	* Zeichen | mit offset an maxlength pro spalte ändern
	* exten "---" grey the whole line?
* Workbench support for Glossar : syntax mit Formatierung, maybe linespacing oder farbe in grau
* update alte api key model and model parameters -> fetch from models API and only report missing feature support
* Rewind support, in session, panel mit rebulld/reextraktion der session aus dem JSON, context rebuild
* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all


## Agents

### Ideas
* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove
* AST/LanguageServer Suppport python/ typescript(remark) sprachmodell geben/LSP/syntax parser/lint/prettier/block diff, als preproccessor, lanugageserver
	* hm nur über hooks vor und nach tool usage nutzbar oder via MCP
* RAG tool zur indizierung von projekten nach aspekten, projekte -> module -> dateien -> methoden -> parameter/rückgaben
	* Rag server tool bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
	* research der kompletten baumstruktur mit allen aspekten eines projektes
	* Callback tools zum Problem, Projektverzeichnis, Projektinfo, Kontexte
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?, is this agent use correct?, violates this some target or increases costs or is meaningless? is this request good or this tool usage?
	* Tool hook can be used to intercet and control
* Eingabeoptimierung, der agent ließt ganze codebäume obwohl nur bestimmte pfade für eine problemlösung relevant ist
	* das projekt muss entsprechend aspektisoliert modelliert sein (vielleicht hilft RAG hier oder AST)
	* Der agent muss geignete tools haben um einen fokkusierten input zu ermitteln
* AI Planstrukturierung self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis 2. Löst dann problem und delegiert umsetzung 3. Umsetzngsagents

## Findings

* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Parralele Agents bringen garnichts wenn man das ergebnis ohnehin stundenlang verifiziert
	* Unterscheidung gleich unterbechen für wichtige sachen im folgekontext, spätere unwichtigere korrekturen auf bassis des finalen kontextes
* Agenten früh kontrollieren und umleiten
* Input hat großen Kontextanteil, fokussieren so gut wie möglich
* Agenten denken ressourcen also dateisystembasiert
* Wenn erweiterung oder bezug auf tickets ist das patch/diff delta ein kleiner und fokussierter eingabekontext, besser als den agenten das feature neu verstehen zu lassen

## Ideas

* lokale claude code alternativre anbinden wie olama
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access