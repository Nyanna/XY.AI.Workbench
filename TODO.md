# TODO
! Ast tool descriptions pruefen und interfaces und beschreibungen prüfen
	* ast_read wird mit module aufgerufen -> description verbessern
	
! Mehr grep xu Zeilennummern statt list, toolbeschreibungen verbessern, die kombi war optimal, also mit ast_find
	* finding, grep war abensp 2stufig, grep durch viele dateien mit preview, zweiter call dann gezielt mit zeilennummer in datei

! erwartete contenthash für read tools um fehler abfangen zu können, oder last updates seid read (also last read)
	* verweis auf toolresult mit ID ?, result ID timecode plus kurze content hash

* exa verbessern, outline vorstufe mit read war sehr effektiv, das gleiche mit exa machen, nur übersicht mit title und result hash id, dann bulk request nach search details (2 stufig), alle results im cache für später
	
* ast script muss generisch werden
* Tests reparieren
* mcpc autostart, von eclipse gestartet wenn nicht da, pro session starten, mit custom port für session
	* mit log datei, control tool connect
* autoprompt beim cache 5min das eine warten nachricht schickt 20 sek for timeout, wie cache bei toolverarbeitung warm halten, cody gemacht, letztes wort wiederholen?

##  Workbench

* ich möchte interaktive shell für refactoring per AST, die AI refactoring sessions waren zu ineffizient. Agent soll beim code helfen
	* micro promt in persistent cache context, vielleicht console in tandem mit editor sessions und session graph/state
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

* rag retrieval auf basis von knoten retriever sind list, find, rag,
	* verschiedene feld filter, vielleicht autoselect auf basis von filtern, dann ein field resolver, durch retriever jagen und
	* verschiedene felder resolven, wie fqnd id, methoden imports usw.
	* knotenbaum editor in eclipse zur sichtkontrolle und entfewrrnen vom result und bäumen
	* heiku davorschalten für üromptkompression und kontext retreival, qualis exploration phase im phase konzept
	* resolver muss eine liste sein, ast-python resolver wird pro knoten vom parent resolver aufgerufen
		* detected python datei so wird python ast drunter gehänt,
	* tool liefert crud operationen auf allen ebenen auch mit block replace
- wie erstellt man am besten einen soft promt => forschungsergebnisse?
	- LLMLingua und LLM selbstkompression
	- eclipse suchprovider für caolgrep -> oder generellen RAG tool panel um kontext auszuwählen
* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove
	* tree-sitter für AST?
	* java ast bauen
	* Project AST: project > dir > file > imports/class > global > node s, code ist immer ein baum (für planing phase)
	* AST/LanguageServer geben/LSP/syntax parser/lint/prettier/block diff
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
