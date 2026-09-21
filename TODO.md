# TODO

## Backlog

### Tools
- Alternatives RAG mit CUDA, Omnigrep, [sweet-search](https://github.com/mrsladoje/sweet-search), SeaGOAT, Qdrant
- OpenAlex zweistufig optimieren – aufteilen in separate Dateien
- AST, JavaParser für Java-AST
- Virtual Environment – alles außerhalb Project-Path oder Filterliste unsichtbar (grep/ast), funktioniert nicht mit bash/python
  - Tool außerhalb automatisch umleiten
- Include-Handling
  - Include persist (`tools/search file`, all...) für lange Sessions – auf Basis Prompt-Objekt-Hash in Verzeichnis
  - Erneutes Einlesen bestimmt sich aus Timestamp der gecachten Datei

### UX
- F3 drücken, um Includes in Eclipse zu öffnen – bei jedem Dateiverweis (relativ oder absolut in `/datei`)
- Drag & Drop für Includes – Includes-Autocompletion (nur wenn Processor aktiviert, mit Projekt-Datei-Autocompletion)
- Bessere Shortcuts für Datei-Erstellung
  - Shortcut für Selection in extra Task auslagern (immer Project-Verzeichnis, notfalls erstellen)
- Autocompletion nach `/`, `<` und nach YAML-Block: `/call`, `/prompt` mit Autocompletion für Includes

### SDK
- Google/OpenAI/Anthropic SDK entfernen und gegen eigene SDK tauschen
- Bei OpenAI Cache-Breakpoint-Marker im Editor/Processor schreiben

### Auto-Runner-Panel
- Datei-basierte SessionConfig mit Fixierung (muss aber als Editor einmal gestartet worden sein)
- Auto-Prompt-Panel mit Tabelle
  - Letztes Kommando pro Datei basierend auf Bedingungen mit Config ausführen – Result wird appended, kein Tag-Replace
  - Abbruch bei Exception
  - Statt Control-Request: direkter Aufruf und Result-Modifizierung
- Deepseek kann gleichzeitig mehrere Tool-Calls senden – als ein Block ausgeben mit nur einem Call-Kommando und zwei YAML-Blöcken
- Warnung bei mehrturn-Kontext ohne Cache-Hit – gelbe Zeile, vorhergehende Result-Zeile mit Cache-Metriken
- Claude Code: Keep-alive-Session, Max-Limit: bei 5min max 1 Stunde, bei 1h max 2h, "warte kurz" random list

### Auto-Runner Approval
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

## Ideas

- **Diff-Support** für Edit-Commands zur direkteren Intent-Erkennung
  - Diff-Editor in Eclipse in-memory aufrufen – Tool-Ausgabe mit Action/Annotation versehen: "view as diff"
  - Block selektieren und Diff-Tool mit Parametern starten – Compare with Clipboard analog
  - Synchrone separate Ansicht, live im Chat aktualisiert → immer letzter Edit
- **Markdown-Table** Autoformat-Support
- **Sub-Agenten** mit Hauptsession verknüpfen – Control-Filter per Filter-Parameter nach Sessionbaum
  - Sub-Agent Interleaving (gibt es nicht mit MCP Controller → sollte kein Problem sein)
- **Soft-Prompts** komprimiert erstellen mit LLMLingua und LLM-Selbstkompression
- **Phases**: Research (human augmented) → Retrieval (Preprocess-Agent: Dateien, Specs, Schemas, Studien, APIs) → Planning (High-Tier) → Execution (Dumb-Agent)
- Was ist [mcp2cli.dev](https://mcp2cli.dev/)?
- **Selbstlernende Agenten** – Lazy Common-Knowledge-Base mit dynamischem Memory, erkennt Patterns und Tools
  - KB zur Vermeidung häufiger Fehler?
- Weitere Tools für Research: Semantic Scholar, arXiv API Access
