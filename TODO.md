# TODO
* versucht immernoch read mit root aufzurufen, erste toolbeschreibung am wichtigsten, wenn parameter schon zu spät
* was ist https://mcp2cli.dev/
* Deepseek anbinden über openai SDK, als chat tool mit caching und standalone modus
	* harnisch mit keep alive prompt für session, max limit bei 5m max eine stunde, bei 1h max 2h
	* autoprompt beim cache 5min das eine warten nachricht schickt 20 sek for timeout, wie cache bei toolverarbeitung warm halten, cody gemacht, letztes wort wiederholen? ("warte kurz" random liste gegen detection, "ich prüfe das", liste von KI generieren lassen, deutsch englisch)
	* statt controll request, direkt aufruf und result modifizieren, ein /allow liest dann aus der textdatei
* open alex zweistufig optimieren, split in separate files

##  Workbench

* ich möchte interaktive shell für refactoring per AST, die AI refactoring sessions waren zu ineffizient. Agent soll beim code helfen -> habe ich beim _engine umbau gehabt!
	* micro promt in persistent cache context, vielleicht console in tandem mit editor sessions und session graph/state
* Diff support für edit commands, um intent direkter zu erkennen
	* diff editor in Eclipse in memory aufrufen und Toolausgabe mit Action oder annotation versehen, "view as diff"
	* block selektieren und mit Parametern diff tool starten, es gibt ein compare with clipboad analog
	* sollte eine synchrone separate ansicht sein die live im chat aktualisiert
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro Zeile
	* Zeichen | mit offset an maxlength pro Spalte ändern
	* exten "---" grey the whole line?
* subagenten mit Hauptsession verknüpfen, control filter per filter Parameter nach einem sessionbaum
	* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
	* erst mit keep alive sinnvoll


## Agents

- wie erstellt man am besten einen soft promt => forschungsergebnisse?
	- LLMLingua und LLM selbstkompression
	- eclipse suchprovider für caolgrep -> oder generellen RAG tool panel um kontext auszuwählen
* Planing augmentation
	* AI Planstrukturierung self has the ability to decide abouts it's capacilities -> nein kann er nicht, reine inferenz
	* It can match effort, modell structure and coordination of a federated mind, nein kann er nicht
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis
	2. Löst dann problem und delegiert Umsetzung
	3. Umsetzngsagents

## Ideas

* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access
* scripte kommentar in englich in `/home/user/xyan/xy.ai.workbench/libs/openapi` [deepseek]