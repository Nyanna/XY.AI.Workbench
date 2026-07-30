# TODO

* token stat tabelle durch log ersetzen
* lt kann auf zeilenebene arbeiten, im Satz, und braucht nur ein invalidate der zeile ohne hightlighting neu berechnen
	* wenn innerhalb kindknoten, nur zeile für spellcheck, extra Region objekt zur eingrenzung
* build setup spart checks

##  Workbench  

* Diff support für edit commands
	* diff editor in eclipse in memory aufrufen und toolausgabe mit action oder annotation versehen, "view as diff"
	* block selektieren und mit parametern diff tool starten, es gibt ein compare with clipboad analog
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro zeile
	* Zeichen | mit offset an maxlength pro spalte ändern
	* exten "---" grey the whole line?
* in preset tab presets aus unterverzeichnis anzeigen und on click laden, inklusive tools
	* ein preset unterstützt prompts, settings und tools, load, save, select
	* selectionliste folgt aktivem editor projekt -> .presets mit auflistung absoluter oder relativer pfade?

* subagenten mit hauptsession verknüpfen, control filter per filter parameter nach einem sessionbaum
	* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
* update alte api key model and model parameters -> fetch from models API and only report missing feature support


## Agents

* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove
	* Project AST: project > dir > file > imports/class > global > node s, code ist immer ein baum 
	* AST/LanguageServer python/typescript(remark) geben/LSP/syntax parser/lint/prettier/block diff
	* MCPC proxy zu eclipse proxy für java project?
	* python benutzen für codearbeit/syntaktisches edit? Oder besser script ast?
	* bash/grep wird gern zur erkundung eingesetzt und python für umsetzung und edit
	* sed sogar zur editierung von python in batch edit
	* bash ist kürzer und effizienter daher kein python
* RAG tool zur indizierung von projekten
	* suchergebniss semantisch komprimieren in mcpc
	* eignet sich RAG zur kondensierung?
	* nach aspekten, projekte -> module -> dateien -> methoden -> parameter/rückgaben
	* wie findet der agent leichter was er sucht in einer datei oder projekt, mit code oder referenzen?
	* Suchtool für dateiinhalt, analog grep mit kontext
	* Rag server tool bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
	* research der kompletten baumstruktur mit allen aspekten eines projektes
	* Callback tools zum Problem, Projektverzeichnis, Projektinfo, Kontexte
	* RAG muss in MD auf absätzen basieren.
* Planing augmentation
	* AI Planstrukturierung self has the ability to decide abouts it's capacilities.
	* It can match effort, modell structure and coordination of a federated mind
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis
	2. Löst dann problem und delegiert umsetzung
	3. Umsetzngsagents

## Ideas

* lokale claude code alternativre anbinden wie olama
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access
