# TODO
- mit compaction experimentieren wenn kontexte minimiert (also anteile) gelöscht werden
- neuer modus, platzhalter semantik für code, KI bekommt kontext und ein give_answer tool
	- der platzhalten wird mit dem give_answer ersetzt
- splitting von bash mcp anhand von ; und batching für bach machen, (parrallel execution)
- regel für autoapprover: keine CLAUDE.md oder AGENTS.md oder andere uppercase markdown dateien laden oder in argumenteninput (reads)
	- Kontext Input größer als 3k zeichen
	- autoapprover abgestufte ausführung 1. nur warnung raussuchen und anzeigen, 2. timouts, 3. fully automated

- neues konzept (code as tool), claude nutzt die bash mit sed um selbst output/kontext input zu filtern
	- ohne bash kann er das nicht
	- batch support für bash um kommandos voneinander zu trennen
		- mit name pipe mechanismus?
		- oder übergreifender mechanismus?
			- gemeinsame basis ist nicht struktur und strukturverständnis, sondern eine text pipe
		- output filter optionen für tools die filter können/wollen?
- MCP automatisch starten wenn nicht running und MCPC profil, oder profil programatisch anlegen
- "resident" in vorhandenes RAG integrieren aber separat startbar
- Per AST Markdown Absätze direkt finden (mehr Kontext, nur ein Tool Call)
	- AST ID mit in ergebnis
- vectorsuche auf normalisierten dateien, normalisierungsspiegel auslagern
- RAG als tool in subprozess server anbinden
	- Multilayer RAG engine auf filesystem und parquet basis in python
		- AST engin für chunking nutzen
		- 2 stage system der results, liefern zeilen, zeilen, chunks/ast oder outline
		- Liefern, Datei AST ID und Zeile			

### Tools
- Auch AST für JS/TS
- Include-Handling
  - Include persist (`tools/search file`, all...) für lange Sessions – auf Basis Prompt-Objekt-Hash in Verzeichnis
  	- ist ein speicher hash prompt, zieldatei im cahe für session, eher backup als cache
  - Erneutes Einlesen bestimmt sich aus Timestamp der gecachten Datei
  - F3 drücken, um Includes in Eclipse zu öffnen – bei jedem Dateiverweis (relativ oder absolut in `/datei`)
  - Drag & Drop für Includes – Includes-Autocompletion (nur wenn Processor aktiviert, mit Projekt-Datei-Autocompletion)
- Bessere Shortcuts für Datei-Erstellung
  - Shortcut für Selection in extra Task auslagern (immer Project-Verzeichnis, notfalls erstellen)
- Autocompletion nach `/`, `<` und nach YAML-Block: `/call`, `/prompt` mit Autocompletion für Includes

### Auto-Runner-Panel
! autorunner soll überwachung erleichtern, vielleicht einfaches penl und nicht vollen editor anzeigen (performance)
- Datei-basierte SessionConfig mit Fixierung (muss aber als Editor einmal gestartet worden sein)
- Auto-Prompt-Panel mit Tabelle
  - Letztes Kommando pro Datei basierend auf Bedingungen mit Config ausführen – Result wird appended, kein Tag-Replace
  - Abbruch bei Exception
  - Statt Control-Request: direkter Aufruf und Result-Modifizierung
- Deepseek kann gleichzeitig mehrere Tool-Calls senden – als ein Block ausgeben mit nur einem Call-Kommando und zwei YAML-Blöcken
- Approval-Tool-Control anders – Tool-Use nur für langwierige teure Operationen
  - Sonst immer den Output abwarten und zusammen approven
  - Nur ein Approval-Call + Ergebnis notwendig
  - Response muss im Autorunner gespiegelt werden (Last Reason) – immer als Letztes
    - Zusammen mit Stats (Zeichen, ob modifizierend)
- Request-Approval-Kategorie mit Auto-Approval pro Kategorie und Argument
  - IO/Web/andere API
  - Schreiboperation (Pfad)
  - Große Operation (Batch-Limit)
  - Viel Kontext (Zeichen-Threshold)
  - Viele Turns/Cache-Read hoch – per Flag togglebar
- Auto-Approve im Harness bei kleinerem X-Zeichen-Kontext und kein Fehler → warum nicht auch über MCPC-Controller mit User-Timeout?
- Virtual Environment – alles außerhalb Project-Path oder Filterliste unsichtbar (grep/ast), funktioniert nicht mit bash/python
  - Tool außerhalb automatisch umleiten -> Auto-Runner korrigiert Tool Calls oder lehnt ab
- **Selbstlernende Agenten** – Lazy Common-Knowledge-Base mit dynamischem Memory, erkennt Patterns und Tools
  - KB zur Vermeidung häufiger Fehler -> Autorunner hat heuristik und regelset
- Claude Code: Keep-alive-Session, Max-Limit: bei 5min max 1 Stunde, bei 1h max 2h, "warte kurz" random list
- Warnung bei mehrturn-Kontext ohne Cache-Hit – gelbe Zeile, vorhergehende Result-Zeile mit Cache-Metriken

### SDK
- Google/OpenAI/Anthropic SDK entfernen und gegen eigene SDK tauschen
- Bei OpenAI Cache-Breakpoint-Marker im Editor/Processor schreiben

## Ideas

- **Markdown-Table** Autoformat-Support
- **Soft-Prompts** komprimiert erstellen mit LLMLingua und LLM-Selbstkompression
- **Phases**: Research (human augmented) → Retrieval (Preprocess-Agent: Dateien, Specs, Schemas, Studien, APIs) → Planning (High-Tier) → Execution (Dumb-Agent)
- Weitere Tools für Research: Semantic Scholar, arXiv API Access