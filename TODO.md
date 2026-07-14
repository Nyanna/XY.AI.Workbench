# TODO
!python benutzen für codearbeit? Oder besser script ast?
	* bash wird gern zur erkundung eingesetzt und python für umsetzung und edit

!ausgabelimits für andere tools, read soll eine confirmation holenm bei großen dateien ohne line numbers
	* wenn bash output zu großt in datei umleiten und dateinamen zurück, für weitere operationen
* control requests entfernen bei client disconencts
	*felder im control request "phase", "tool" wird in der respone nicht gebraucht warum gebraucht?

* git trailing-space problem: Prüfen, bash stdout kann optimiert werden und muss nicht dateigenau sein?
	* multiple leerzeilen stauchen, leerzeilen mit whitespace bereinigen
```yaml
id: "af61fc55-237a-4816-903c-5717b5908757"
phase: "result"
toolName: "bash"
result:
  content: []
  structuredContent:
    exit_code: 0
    stdout: "commit d855e3dd156685daf832b42f58751f602f6a064d\nAuthor: Xyan <Xyan@xyan.icu>\nDate:   Sun Jul 12 23:17:23 2026 +0200\n\n    -fixed double content\n\ndiff --git a/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py b/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py\nindex 693b879..9ee79ce 100644\n--- a/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py\n+++ b/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py\n@@ -24,7 +24,6 @@ from __future__ import annotations\n \n from typing import Any\n \n-from ...codec import JsonCodec\n from ...config import ServerConfig\n from ...openalex import (\n     DEFAULT_SEARCH_PRESET,\n@@ -37,7 +36,7 @@ from ...openalex import (\n )\n from ...openalex.client import ENTITIES\n from ...openalex.presets import WORK_PRESET_NAMES\n-from ...registry import ToolContext, ToolRegistry, ToolResult, text_content\n+from ...registry import ToolContext, ToolRegistry, ToolResult\n \n #: Hard caps that mirror the OpenAlex API limits.\n _MAX_PER_PAGE = 200\n@@ -72,16 +71,11 @@ def _error_result(exc: Exception) -> ToolResult:\n     structured: dict[str, Any] = {\"error\": message}\n     if isinstance(exc, OpenAlexAPIError) and exc.status is not None:\n         structured[\"status\"] = exc.status\n-    return ToolResult(\n-        content=[text_content(message)],\n-        structured_content=structured,\n-        is_error=True,\n-    )\n+    return ToolResult(structured_content=structured, is_error=True)\n \n \n def _ok_result(structured: dict[str, Any]) -> ToolResult:\n-    text = JsonCodec.encode(structured, indent=2)\n-    return ToolResult(content=[text_content(text)], structured_content=structured)\n+    return ToolResult(structured_content=structured)\n \n \n def _summarise_list(data: dict[str, Any]) -> dict[str, Any]:\n"
```

* wo ist der unterschied zwischen normaler Inferenz und thinking? Oft kein Unterschied macht das modell keinen unterschied und es ist dasselbe.
* session logs für die implementierung überprüfen und auf optimierbarkeit checken, alle tools da, das richtige verständnis, wo falsch abgeboden usw.
	* eigentlich zwei phasen, input auswählen, output lösung validieren oder eingreifen
	* für input muss der context aufs minimum und affected sources beschränkt werden
	* für output muss der ansatz validiert werden
* project > dir > file > imports/class > global > nodes, code ist immer ein baum 

zu viel escaping, hin und zurück
	#: Control Request:
	#:id: "701d7547-df51-40e6-80ad-af4fc3e8b9cc"
	#:phase: "request"
	#:toolName: "change"
	#:arguments:
	#:  path: "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java"
	#:  start: "\t\t\t\tif ((line = session.readLine()) != null) {\n\t\t\t\t\t\tjsonParser.parseLine(resp, session, sub, line);\n\n\t\t\t\t\t}"
	#:  end: "\t\t\t\t}"
	#:  content: "\t\t\t\tif ((line = session.readLine()) != null) {\n\t\t\t\t\t\tsession.setLastRawLine(line);\n\t\t\t\t\t\tjsonParser.parseLine(resp, session, sub, line);\n\n\t\t\t\t\t}"
	/allow 701d7547-df51-40e6-80ad-af4fc3e8b9cc
	#: Control Request:
	#:id: "183f1a46-d6ed-49be-a06f-0f387abbb0c8"
	#:phase: "result"
	#:toolName: "change"
	#:result:
	#:  content: []
	#:  structuredContent:
	#:    error: "End marker is ambiguous – found 6 occurrences in file: '\\t\\t\\t\\t}'"
	#:  isError: true
	/allow 183f1a46-d6ed-49be-a06f-0f387abbb0c8

## Workbench
* Diff support für edit commands
	* diff editor in eclipse in memory aufrufen und toolausgabe mit action oder annotation versehen, "view as diff", ist mit YAML jetzt einfacher
	* block selektieren und mit parametern diff tool starten, es gibt ein compare with clipboad analo

### Ideas
* in preset tab presets aus unterverzeichnis anzeigen und on click laden, inklusive tools
* subagenten mit hauptsession verknüpfen, control filter per filter parameter nach einem sessionbaum
	* load, save, select
	* selectionliste folgt aktivem editor projekt -> .presets mit auflistung absoluter oder relativer pfade?
* Table renderer support
	* Zeile beginnt mit |, gleiche Anzahl | pro block pro zeile
	* Zeichen | mit offset an maxlength pro spalte ändern
	* exten "---" grey the whole line?
* Workbench support for Glossar : syntax mit Formatierung, maybe linespacing oder farbe in grau
* update alte api key model and model parameters -> fetch from models API and only report missing feature support
* Rewind support, in session, panel mit rebulld/reextraktion der session aus dem JSON, context rebuild
* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all


## Agents

### Ideas
* AST tool augmentieren, spezifische tools, ersetze Abschnitt, ersetze Überschrift, ersetze Funktion etc.
	# headings list/change/remove, paragraph ast-path, replace, edit, add, remove
* AST/LanguageServer Suppport python/ typescript(remark) sprachmodell geben/LSP/syntax parser/lint/prettier/block diff, als preproccessor, lanugageserver
	* hm nur über hooks vor und nach tool usage nutzbar oder via MCP
* RAG tool zur indizierung von projekten nach aspekten, projekte -> module -> dateien -> methoden -> parameter/rückgaben
	* Rag server tool bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
	* research der kompletten baumstruktur mit allen aspekten eines projektes
	* Callback tools zum Problem, Projektverzeichnis, Projektinfo, Kontexte
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?, is this agent use correct?, violates this some target or increases costs or is meaningless? is this request good or this tool usage?
	* Tool hook can be used to intercet and control
* Eingabeoptimierung, der agent ließt ganze codebäume obwohl nur bestimmte pfade für eine problemlösung relevant ist
	* das projekt muss entsprechend aspektisoliert modelliert sein (vielleicht hilft RAG hier oder AST)
	* Der agent muss geignete tools haben um einen fokkusierten input zu ermitteln
* AI Planstrukturierung self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind
	1. Ein agent erstellt die notwendigen inputs für einen prompt, dateien, specs, schemas, studien, apis 2. Löst dann problem und delegiert umsetzung 3. Umsetzngsagents

## Findings

* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Parralele Agents bringen garnichts wenn man das ergebnis ohnehin stundenlang verifiziert
	* Unterscheidung gleich unterbechen für wichtige sachen im folgekontext, spätere unwichtigere korrekturen auf bassis des finalen kontextes
* Agenten früh kontrollieren und umleiten
* Input hat großen Kontextanteil, fokussieren so gut wie möglich
* Agenten denken ressourcen also dateisystembasiert
* Wenn erweiterung oder bezug auf tickets ist das patch/diff delta ein kleiner und fokussierter eingabekontext, besser als den agenten das feature neu verstehen zu lassen
* besser etwas extra thinking durch iteration und abbrüche von Tools, als zu viel in den Kontext laden, die Rechnung ist salienz gegen token

## Ideas

* lokale claude code alternativre anbinden wie olama
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* weitere tools für research Semantic Scholar, arXiv API Access