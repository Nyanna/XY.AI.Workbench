# TODO

## Backlog

- tools
	* anderes RAG mit CUDA, omengrep, https://github.com/mrsladoje/sweet-search, SeaGOAT, Qdrant
	* open alex zweistufig optimieren, split in separate files
	- AST, JavaParser für java AST
	- virtual environment, alles außerhalb project path oder filterliste unsichtbar (grep/ast), funktioniert nicht mit bash oder python
		- tool außerhalb automatisch umleiten
* include handling
	- include file tool muss absoluten dateinamen mit in das toolresult geben
	- inlcude persist(tools/search file,all...) um lange sessions durchführen zu können, auf Basis Prompt Objekt Hash in Verzeichnis
		- reread bestimmt sich aus timestamp der gecachten datei -> oder auch nicht
	- UX
		- F3, drücken um includes dann in eclipse zu öffnen, immer wenn dateireferenzen, relativ oder absolut in `/datei`
		- dragNdrop includes, includes autocompletion (nur wenn processor aktiviert, mit projet datei autocompletion)
		* besserer shortcuts für datei erstellen
			- shortcut für selection in extra task auslagern (immer project verzeichnis, nottfals erstellen)
		- / autocomplete after "/" und "<" und wenn nach YAML block dann /call /prompt, mit autocomplete für includes
* Cleanups, Google/OpenAI/Anthropic SDK entfernen und gegen eigene SDK tauschen
	* bei openai cache breakpoint marker mit in editor/processor schreiben
* autorunner panel
	* Dann file based SessionConfig mit fixierung, muss aber als editor mal gestartet worden sein
	* dann auto prompter panel mit tabelle
		- immer letztes kommando auf basis von konditions pro datei mit config ausführen, result wird appended, kein tag replace
		* abbruch bei exception
		* nach tool result eine neues /prompt kommando
		* statt controll request, direkt aufruf und result modifizieren
	* deepseek kann gleichzeit mehrere toolcalls schicken, das muss als ein block ausgegeben werden mit nur einem call kommando und zwei(x) YAML blöcken
	- warning wenn mehrturn kontext ohne cache hit zurückkommt, gelbe zeile, vorhergehende result zeile mit cache metriken
	- claude code: keep alive session, max limit bei 5m max eine stunde, bei 1h max 2h, "warte kurz" random list
	- autorunner approval
		* approval tool control anders, tool use nur für langwierige teure operationen,
			- sonst immer den output abwarten und zusammen approven
			- Dann nur ein approval call + ergebnis notwendig
			- dann muss reaspon auch in die response gespiegelt werden vom autorunner (last reason), immer als letztetes
				- zusammen mit stats wie Zeichen und ob modifizierend
		- request approval categorie mit auto approval pro cat und argument
			- IO-Web/Andere API, Schreiboperation (Pfad), Große Operation (Batch Limit),
			- Viel Kontext(Zeichen Threshold), viele turns/cache read hoch -> per flag togglebar
		* auto approve im harness wenn kontext in kleiner X zeichen und kein fehler -> warum nicht auch über MCPC controler, mit user timeout zum reagieren?

## Ideas

* Diff support für edit commands, um intent direkter zu erkennen
	* diff editor in Eclipse in memory aufrufen und Toolausgabe mit Action oder annotation versehen, "view as diff"
	* block selektieren und mit Parametern diff tool starten, es gibt ein compare with clipboad analog
	* sollte eine synchrone separate ansicht sein die live im chat aktualisiert -> immer letzter edit
* Markdown Table autoformat support
* subagenten mit Hauptsession verknüpfen, control filter per filter Parameter nach einem sessionbaum
	* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
- Softpromt komprimiert erstellen mit LLMLingua und LLM selbstkompression
- Phases, Research(human augmented), Retrieval(Preprocess agent,dateien, specs, schemas, studien, apis), Planing(high tier), Execution(Dumb agent)
- was ist https://mcp2cli.dev/
- selbst lerne agenten, lazy common knowledge base dynamischen memory das patterns un tools erkennt, ist das eine KB um häufige fehler zu vermeiden
- andere tools für research Semantic Scholar, arXiv API Access