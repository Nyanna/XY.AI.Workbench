# TODO
! neues AST tool das alle editoperationen verein via batch
	- aber nicht liste sondern openapi save, liste pro edit operation
Einheitliche warnings und fehler durch helper auf item oder batch ebene
      warning: "Limit of 30 matches reached (total 681); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
      error: 2 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis.
      
! batchsupport vereinfachen und vereinheitlichen
	- gemeinsamer Wrapper und Batch Objekt
	- Variables Itemschema und Objekte
	- Feste methodic für per Batch(global) felder die für den Stack wirken
! mit agent ignore verezichniss ignorieren (.mcpcignore)
	- oder umgekehrt, nur die allow liste
	- datei caching und nur bei bedarf aktualisieren, zusätzlich config über agent
	- recursiv vom Zielpfad additiv (invers, höchste zuerst, gitignore syntax)
		- Dann Zusätzlich per Header für session
		- Dann Zusätzlich in session per tool call, ja tool call zur veränderung
			- vielleicht exec_command und extra tool



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
	- bei include autocompletion, zuerst variationen lisen (search, file) dann project Root einfügen /home/user/xyan/xy.ai.workbench
- Wenn F3 auf /answer <ID> dann code navigation auf den yaml block header mit der ID, wenn result dann nochmal F3 und auf tool call springen
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
Cannot process command: no active Claude Code session exists