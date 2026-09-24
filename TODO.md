# TODO
* AISession view initialsiert editorlistender erst
* better restore tooles also
* config restore resettet immer das model auf nicht MCPC sonnet
* AI MD editor menü reduzieren
* refactor IPartListener2 to use ActiveEditorListener

## Backlog
- **Diff-Support** für Edit-Commands zur direkteren Intent-Erkennung
  - Diff-Editor in Eclipse in-memory aufrufen – Tool-Ausgabe mit Action/Annotation versehen: "view as diff"
  - Block selektieren und Diff-Tool mit Parametern starten – Compare with Clipboard analog
  - Synchrone separate Ansicht, live im Chat aktualisiert → immer letzter Edit, oder cursor in Tool Call Block oder /answer /call
  - Muss über MCPC laufen als dry run aller edit tools, originale calls verwenden, mit flag wird nicht result returned sondern AST knoten alt/neu
  	- URL parameter setzt interceptor in session, AST schickt Node read und write mit ID an Interceptor und speichert nicht, interceptor fängt dann tool result ab und ersetzt es durch eigene compare ausgabe
* commit viewer in panel
  * update logik wenn model return IAwnser
  	* bash script dann über JGit intern
  * ref immer dann aktualisieren mit neueste, commit editor ebenfalls aktualisieren
  * mcp control client braucht nach allow noch execute was autoapprove deaktiviert
  	* Damit der agent nach einem success nicht weiter läuft und zeit für ein revert ist
  	* ein deny im result macht dann auto revert vom aktuellen ref
  
### Tools
- AST, JavaParser für Java-AST
- Include-Handling
  - Include persist (`tools/search file`, all...) für lange Sessions – auf Basis Prompt-Objekt-Hash in Verzeichnis
  - Erneutes Einlesen bestimmt sich aus Timestamp der gecachten Datei
- Claude kombiniert filter und toolcalls im retrieval häufig -> Multi Tool Batch
  - Bash(echo "=== oidc/server dir ===" && ls -R smc-swissdamed/.../server/ 2>/dev/null && echo "=== oidc dir ===" && ls smc-swissdamed/src/main/java/ch/swissmedic/swissdamed/config/security/oidc/ && echo "=== security dir ===" && ls
  - find ~/.gradle /root/.gradle -iname "*authorization-server*.jar" 2>/dev/null | head; echo "=== spring security version in BOM ==="; find ~/.gradle -path "*spring-security-oauth2-authorization-server*" -name "*.jar" 2>/dev/null | head; echo "=== catalog ==="; grep -i "spring-boot\|spring-security\|jackson" /home/.../libs.versions.toml

### UX
- F3 drücken, um Includes in Eclipse zu öffnen – bei jedem Dateiverweis (relativ oder absolut in `/datei`)
- Drag & Drop für Includes – Includes-Autocompletion (nur wenn Processor aktiviert, mit Projekt-Datei-Autocompletion)
- Bessere Shortcuts für Datei-Erstellung
  - Shortcut für Selection in extra Task auslagern (immer Project-Verzeichnis, notfalls erstellen)
- Autocompletion nach `/`, `<` und nach YAML-Block: `/call`, `/prompt` mit Autocompletion für Includes

### Auto-Runner-Panel
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
- **Sub-Agenten** mit Hauptsession verknüpfen – Control-Filter per Filter-Parameter nach Sessionbaum
  - Sub-Agent Interleaving (gibt es nicht mit MCP Controller → sollte kein Problem sein)
- **Soft-Prompts** komprimiert erstellen mit LLMLingua und LLM-Selbstkompression
- **Phases**: Research (human augmented) → Retrieval (Preprocess-Agent: Dateien, Specs, Schemas, Studien, APIs) → Planning (High-Tier) → Execution (Dumb-Agent)
- Weitere Tools für Research: Semantic Scholar, arXiv API Access
- Alternatives RAG mit CUDA, Omnigrep, [sweet-search](https://github.com/mrsladoje/sweet-search), SeaGOAT, Qdrant
