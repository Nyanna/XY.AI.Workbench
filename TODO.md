# TODO
- Fehlermeldungen für pfade, dateien und knoten umbauen, statt not found anzahl dateien etc, irgendwas was für den nächsten call funktioniert, an root,[*], module etc andocken. Warnung kann entfernt werden, weniger kritisch. Tool aufruf soll hilfreich sein und den nächsten schritt anzeigen. STDOUT limit soll auszug geben mit hinweis auf grep und vollem output oder outline ala 2 stage und auch metriken über stdout dateien, zeilen/lines etc
- outputschema müssen sprechend sein, namen ud struktur prüfen, STDERR datei parameter prüfen, nicht besser im fleißtext erwähnen?

- Alternatives RAG mit CUDA, Omnigrep, [sweet-search](https://github.com/mrsladoje/sweet-search), SeaGOAT, Qdrant
	- alle doof, splitting mit model by domain markdown -> text modell, code -> lateon modell, augmented retrieval tool, MCPC integrated RAG
	- Multilayer RAG engine auf filesystem und parquet basis in python
		- AST engin für chunking nutzen
		- damit ein grep/ast_find liefern das nur suchausschnitt zeigt
		- geteilter index für dateiänderungen, hash und
		- reindex lazy mit erstem layer also textsuche
		- teilen sich infrastruktur, änderungserkennung, dateisystem
		- reindex priorisierung durch vorherige layer
			- layher fügen hinzu, oder reichern an oder taggen und voten für ranking
			- 2 stage system der results, liefern zeilen, zeilen, chunks/ast oder outline
			- Liefern, Datei AST ID und Zeile
			- metadaten sind separate layer wie auch zusammenfassungen und queries
			- layer muss für agent transparentes signal liefern, dies ist eine zusammenfassung, dies ist ein auszug
		- gemeinsame interfaces schaffen, query objekt, result object, query Kontext
		
		
		- fd/glob, grep, ripgrep, ugrep,
		- csearch/cindex, Zoekt, fcs, FTS5-Trigramm, ugrep-indexer
		- FTS5, bm25s, Tantivy, Recoll, qmd search, mit eigener Tokenisierung (camelCase, LaTeX)
		- bm25, trigram/n-gramm, word2vector, model2vector,
		- BGE-M3 dense/sparse, Jina, Qwen3-Embedding, SPECTER2
		- Doc2Query, ColBERT-MaxSim auf Kandidaten, BGE-M3 multi, PyLate rank
		- PyLate/FastPLAID mit LateOn-Code-edge (17M) → LateOn-Code (130M), GTE-ModernColBERT

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