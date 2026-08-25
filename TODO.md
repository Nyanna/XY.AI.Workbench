# TODO
- wie erstellt man am besten einen soft promt => forschungsergebnisse?
- wie wird RAG am besten von einem agenten oder menschen verwendet, gibt es einen nicht KI kontext, was ist mit google?
	- RAG ist von KI unabhängiger suchmechanismus, benutzt nur NN vektoren
	- semantische kompression für nur wissen ohne ändern, kein code
- Rag weiter
	- eclipse suchprovider für caolgrep
* autoprompt beim cache 5min das eine warten nachricht schickt 20 sek for timeout

##  Workbench  

* Diff support für edit commands
	* diff editor in Eclipse in memory aufrufen und Toolausgabe mit Action oder annotation versehen, "view as diff"
	* block selektieren und mit Parametern diff tool starten, es gibt ein compare with clipboad analog
	* sollte eine synchrone separate ansicht sein die live im chat aktualisiert
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro Zeile
	* Zeichen | mit offset an maxlength pro Spalte ändern
	* exten "---" grey the whole line?

* subagenten mit Hauptsession verknüpfen, control filter per filter Parameter nach einem sessionbaum
	* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
* update alte api key model and model parameters -> fetch from models API and only report missing feature support


## Agents

* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove
	* Project AST: project > dir > file > imports/class > global > node s, code ist immer ein baum (für planing phase)
	* AST/LanguageServer typescript(remark) geben/LSP/syntax parser/lint/prettier/block diff
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
	* RAG muss in MD auf absätzen basieren (AST).
* Planing augmentation
	* AI Planstrukturierung self has the ability to decide abouts it's capacilities.
	* It can match effort, modell structure and coordination of a federated mind
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis
	2. Löst dann problem und delegiert Umsetzung
	3. Umsetzngsagents

## Ideas

* lokale claude code alternativre anbinden wie olama
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access
