# TODO
! mcpc startet zu spät nach tool laden. in plugin launch config prüfen und dann selber scho MCPC vor memento starten -> vielleicht?
	-> besser, memento nach MCPC connectio tools neu einlesen, oder besser ein tools cache vor MCPC, und wenn MCPC danach ein update vpm cache
		-> wenn dann MCPC nicht verfübar immernoch cahce, cache in Memento mit speichern und dann danach aktualisieren
* include handling
	- include file tool muss absoluten dateinamen mit in das toolresult geben
	- inlcude persist(tools/search file,all...) um lange sessions durchführen zu können, auf Basis Prompt Objekt Hash in Verzeichnis
		- reread bestimmt sich aus timestamp der gecachten datei -> oder auch nicht
	- UX
		- F3, drücken um includes dann in eclipse zu öffnen, immer wenn dateireferenzen, relativ oder absolut in `/datei`
		- dragNdrop includes, includes autocompletion (nur wenn processor aktiviert, mit projet datei autocompletion)
		* besserer shortcuts für datei erstellen
			- shortcut für selection in extra task auslagern (immer project verzeichnis, nottfals erstellen)
		- / autocomplete after "/" und wenn nach YAML block dann /call /prompt, mit autocomplete für includes
- tools
	* anderes RAG mit CUDA, omengrep, https://github.com/mrsladoje/sweet-search, SeaGOAT, Qdrant
	* open alex zweistufig optimieren, split in separate files
	- AST, JavaParser für java AST
	- virtual environment, alles außerhalb project path oder filterliste unsichtbar (grep/ast), funktioniert nicht mit bash oder python
	
- Schema doku notieren
	- "Label:" -> Segmente mit Metadaten
	- "``yaml" -> komplexe aufrufe unt metadaten
	- "/command" -> Slash kommadnos
	- (Neu) [include file newlines=True](/pfad/datei) und [include prefix](/pfad/datei)

# Backlog

* Cleanups, Google/OpenAI/Anthropic SDK entfernen und gegen eigene SDK tauschen
	* bei openai cache breakpoint marker mit in editor/processor schreiben
* autorunner panel
	* Dann file based SessionConfig mit fixierung, muss aber als editor mal gestartet worden sein
	* dann auto prompter panel mit tabelle, immer letztes kommando auf basis von konditions pro datei mit config ausführen, result wird appended, kein tag replace
		* abbruch bei exception
		* nach tool result eine neues /prompt kommando
		* statt controll request, direkt aufruf und result modifizieren
	* deepseek kann gleichzeit mehrere toolcalls schicken, das muss als ein block ausgegeben werden mit nur einem call kommando und zwei(x) YAML blöcken
	- warning wenn mehrturn kontext ohne cache hit zurückkommt > wo, wie? -> inline gelbe zeile aber woher wissen wann cache an sein soltle? -> vorhergehende result zeile mit cache metriken
	- claude code: keep alive session, max limit bei 5m max eine stunde, bei 1h max 2h, "warte kurz" random list -> zusatzfeature vom autorunner
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

# Ideas

* ich möchte interaktive shell für refactoring per AST, die AI refactoring sessions waren zu ineffizient. Agent soll beim code helfen -> habe ich beim _engine umbau gehabt!
	* micro promt in persistent cache context, vielleicht console in tandem mit editor sessions und session graph/state
	* Oder eine console/promt mit AST java zugriff auf eclipse?
* Diff support für edit commands, um intent direkter zu erkennen
	* diff editor in Eclipse in memory aufrufen und Toolausgabe mit Action oder annotation versehen, "view as diff"
	* block selektieren und mit Parametern diff tool starten, es gibt ein compare with clipboad analog
	* sollte eine synchrone separate ansicht sein die live im chat aktualisiert
* Table renderer support
	* vielleicht nicht rendern sondern tabelle mit autoformat neu ausrichten?
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro Zeile
	* Zeichen | mit offset an maxlength pro Spalte ändern
	* exten "---" grey the whole line?
* subagenten mit Hauptsession verknüpfen, control filter per filter Parameter nach einem sessionbaum
	* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
	* erst mit keep alive sinnvoll
* modell von dopplung durch kindknoten verwirrt - `/home/user/xyan/xy.ai.workbench/project/done/tools_provider.md`
* tabs sind was den yaml block scalar bricht, mit java formatter könnte man das ändern -> richtiger java validator formatter
* always use python scripts cuz of implicite knowledge in coding instead of interface description
- wie erstellt man am besten einen soft promt => forschungsergebnisse?
	- LLMLingua und LLM selbstkompression
	- eclipse suchprovider für caolgrep -> oder generellen RAG tool panel um kontext auszuwählen
* Planing augmentation
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis
	2. Löst dann problem und delegiert Umsetzung
	3. Umsetzngsagents
* was ist https://mcp2cli.dev/
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
	* brauche dynamischen memory das patterns un tools erkennt, ist das eine KB um häufige fehler zu vermeiden
* weitere tools für research Semantic Scholar, arXiv API Access