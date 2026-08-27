# TODO
* mcpc autostart, von eclipse gestartet wenn nicht da, pro session starten, mit custom port für session
* vielleicht toolsearch implementieren und tool update machen? Oder tool nach anfrage aktivieren können. ask-tool und user aktiviert tool
	* was ist mit spill to file in bash?
	* aktivierbar mit flag, `{"capabilities": {"tools": {"listChanged": true}}}`, `Server: {"jsonrpc": "2.0", "method": "notifications/tools/list_changed"}
		* keine Toolaktiiverung, tools umstellen auf virtuellen Dateisystem mit code schema auf python basis
		* sampling inferenz syntax nur mit "strict": true
		* tool registry auf PYI basis umbauen
		* problem kleiner inkrments
		* PY umgebung stellt console.out breit und das modell kriegt die rückgabe daraus, kann das laden gezielt steuern
			* User kriegt ausgabe und kann sie kürzen oder deny machen
			* sicherheitsgates für große ausgaben erzeugen warnungen in out, muss dann überschrieben werden (console.out(kontextPollutionProtection: false))
			* begrenzte umgebung nur erlaube imports
			* python kontext ist zu weit -> das wird echo /cat loops erzeugen, modell muss beschränkt werden
			* tool registrierung muss abstrakte datei angeben die sie implementiert, schema kommt in abstrakte datei, nich mehr per decoration registrieren
			* gesicherte umgebung extra dateisystem mit symlinks /tool/python/tools, quasi ein beschränktes system mit chrott simulieren, ressourcen werden nur freigegeben und eingehängt
			* oder MCP tool geben nur infos zu den abstrakten python files un rufen zur verwendung von skript auf, also nur info tool- enweter tool-list -> tool-usage
* (java ast bauen)
* rag auf basis von knoten retriever sind list, find, rag,
	* verschiedene feld filter, vielleicht autoselect auf basis von filtern, dann ein field resolver, durch retriever jagen und
	* verschiedene felder resolven, wie fqnd id, methoden imports usw.
	* knotenbaum editor in eclipse zur sichtkontrolle und entfewrrnen vom result und bäumen
	* heiku davorschalten für üromptkompression und kontext retreival, qualis exploration phase im phase konzept
	* resolver muss eine liste sein, ast-python resolver wird pro knoten vom parent resolver aufgerufen
		* detected python datei so wird python ast drunter gehänt,
	* tool liefert crud operationen auf allen ebenen auch mit block replace

- wie erstellt man am besten einen soft promt => forschungsergebnisse?
	- LLMLingua und LLM selbstkompression
	- Kompression für RAG?
- wie wird RAG am besten von einem agenten oder menschen verwendet, gibt es einen nicht KI kontext, was ist mit google?
	- RAG ist von KI unabhängiger suchmechanismus, benutzt nur NN vektoren
	- semantische kompression für nur wissen ohne ändern, kein code
- Rag weiter
	- eclipse suchprovider für caolgrep -> oder generellen RAG tool panel um kontext auszuwählen
* autoprompt beim cache 5min das eine warten nachricht schickt 20 sek for timeout, wie cache bei toolverarbeitung warm halten?

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
