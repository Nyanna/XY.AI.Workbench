# Claude-Code CLI Connector



Eine Session, was ist mit resume? oder Rewind?
Multiple antworten, toolabfragen und bestätigung, stdin offen lassen?
Wie agent auswählen?

* Session UUID aus aktueller datei generieren, und mit frontmatter an den anfang schreiben, wenn von hand gelöscht neu generieren. Auch datum/zeit vom sessionstart reinschreiben mit reinschreiben.
* relevante parameter aus panel in frontmatter übernehmen
* wird ein MD Frontmatter sessionformat
* Parralel alle ein und ausgaben in parralelel NDJSON datei mittels (--replay-user-messages) input spiegeln
* output modus in capabillities verlagern claude code unterstützt nur append
* Prompt, enqueue, batch in capabilities verlagen und alles bis auf Prompt deaktivieren bei Claude code
* Initial subprocess starten und erst beim Session ID wechsel prüfen, beenden und neu starten, oder bei eclipse exit
* Answers auf Liste umstellen und einzeln in datei rendern, mit thinking und tool
* log listender hängt am STDOU und spiegelt in JSON datei


* claude-work default --verbose --include-hook-events --include-partial-messages --input-format stream-json --output-format stream-json
	* mit profil im key claude-work
	* mit resume wenn session vorhanden ist bei claude und im projektverzeichnis
	* arbeitsverzeichnis ist datei projectverzeichnis
	


## Referenzen

* Java Source Root: `/home/user/xyan/xy.ai.workbench/src/`
* Connector Implementierung für die Claude-API: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claude/ClaudeConnector.java`



## Input Support

Die Implementierung muss nur den Standard-Prompt Input unterstützen.

```json
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Deine Frage"}]}}
```