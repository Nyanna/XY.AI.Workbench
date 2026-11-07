# TODO
- claude code hat bei den permissions auch schon einen deny/allow reason, wie wird der oim tool result abgebildet?
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
- Autoapprover: Deepseek kann gleichzeitig mehrere Tool-Calls senden – als ein Block ausgeben mit nur einem Call-Kommando und zwei YAML-Blöcken

### SDK
- Google/OpenAI SDK entfernen und gegen eigene SDK tauschen
- Bei OpenAI Cache-Breakpoint-Marker im Editor/Processor schreiben

## Ideas

- **Markdown-Table** Autoformat-Support
- **Soft-Prompts** komprimiert erstellen mit LLMLingua und LLM-Selbstkompression
- **Phases**: Research (human augmented) → Retrieval (Preprocess-Agent: Dateien, Specs, Schemas, Studien, APIs) → Planning (High-Tier) → Execution (Dumb-Agent)
	- Idee alle retrieval calls in einer session zu mergen, so lassen sich kontext dumps erstellen udn wiederverwenden, spart retreval und lopps
- Weitere Tools für Research: Semantic Scholar, arXiv API Access
- neuer modus, platzhalter semantik für code, KI bekommt kontext und ein give_answer tool
	- der platzhalten wird mit dem give_answer ersetzt