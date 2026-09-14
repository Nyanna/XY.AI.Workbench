# TODO

- Include im Harness,  wie wurde man einen Include realisieren vielleicht ein script block ```script und dann metacode -> [include/tool_read](datei.md)
	- muss am anfang beginnen dann eine oder mehrere pro zeile, ersetzt zeile komplett und trennt an der stelle
	- rekursionsschutz, liste führen wenn zielinclude zu datei höher im baum auflöst
	- [Label] erlaubt metadaten
* einen processor, unabhängiges parsing mit callbacks
	- wird ein Callback nicht angegeben wird auf ein generisches text Only callback zurückgefallen
* umgekehrt für den output wird ein Modellanwort in bestandteile zerlegt die der processor, symetrisch in entsprechenden ouput umwandelt
* Der session processor garantiert symetrisches encoding, text rein -> message struktur raus
* der connector extrahiert aus der modellanwort die structur -> gibt sie dem session processor -> dieser wandelt wieder um in Text, Thinking und Tool Calls

## Harness
* eine AI SessionConfig pro datei, bei neuem editor aktuelle kopieren
	* Overlay mechanismus, initial eine Default config, bei neuem editor wird default verwendet,
	* wenn diese verändert -> clonen
	* wenn nciht offen wird default verändert, persistier wird default
	* über frontmatter in editor aktuelle config ändern aber auch zurückspiegeln
	* session processor soll frontmatter ignorieren
- Prompt Input
	-Die bisherigen Toolinputs für Search/Files werden abgegrenzt
	- inputmodes entfernen context,files,search
		- statdessen haken für Tagersetzung <search>
	* Controlltoken und zeilen wieder als hint benutzen und zurück umwandeln
		* Markdown wäre dann effektive session
	* Inputmode müsste dann wieder ganzer editor sein wegen prefix cache
		* oder backed by memorry modell oder beides?, mit session ID analog zu Claude Code?
	- durch meta syntax im Body ersetzen
		- [include search mode](all|selected|asFiles)
		- [include files](selected|list) - auch mit Zeilenbereichen
		- analog den MCP tool schema aber von eclipse als input deklariert, harness tool schicht
	* bei openai cache breakpoint marker mit in editor schreiben
	
- MCP CLient 
	* /call <ID> (statt /answer), tool call mit ID Versenen, direkt nach call schreiben, direkten block davor parsen
	* zurückgehen zum letzten ^```yaml$ und result JSON block parsen
		* in SDK nachrichtentypen packen, alles andere wird developer/system
		
* Harness
	* autolauf modus
	* keep alive session, max limit bei 5m max eine stunde, bei 1h max 2h, "warte kurz" random list
	* statt controll request, direkt aufruf und result modifizieren, ein /allow liest dann aus der textdatei

* harness always use python scripts cuz of implicite knowledge in coding instead of interface description
	
- harness loop
	1. Chatverlauf
	2. Separate Datei und view
	3. Separate Dateien für branching
	4. Über MCP Client, Subagent call in MCPC mit Deepseek backend
!! SUbagenten nachden

## Other
- anzeige für geschätzen cache read/write(neuen input)
* approval tool control anders, tool use nur für langwierige teure operationen,
	- sonst immer den output abwarten und zusammen approven
	- Dann nur ein approval call + ergebnis notwendig
	- dann muss reaspon auch in die response gespiegelt werden, immer als letztetes
		- zusammen mit stats wie Zeichen und ob modifizierend
	- request approval categorie mit auto approval pro cat und argument
		- IO-Web/Andere API, Schreiboperation (Pfad), Große Operation (Batch Limit),
		- Viel Kontext(Zeichen Threshold), viele turns/cache read hoch -> per flag togglebar
* anderes RAG mit CUDA, omengrep, https://github.com/mrsladoje/sweet-search, SeaGOAT, Qdrant
* deepseek, temperaturee deaktivieren wenn thinking, top_p only thinking und minval 0.95 - 1
* open alex zweistufig optimieren, split in separate files
* modell von dopplung durch kindknoten verwirrt - `/home/user/xyan/xy.ai.workbench/project/done/tools_provider.md`
* tabs sind was den yaml block scalar bricht, mit java formatter könnte man das ändern
* auto approve im harness wenn kontext in kleiner X zeichen und kein fehler -> warum nicht auch über MCPC controler, mit user timeout zum reagieren?

- F3, drücken um includes dann in eclipse zu öffnen, immer wenn dateireferenzen, relativ oder absolut in `/datei`
- Schema doku notieren
	- "Label:" -> Segmente mit Metadaten
	- "``yaml" -> komplexe aufrufe unt metadaten
	- "/command" -> Slash kommadnos
	- (Neu) [include file newlines=True](/pfad/datei) und [include prefix](/pfad/datei)

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

* was ist https://mcp2cli.dev/
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access