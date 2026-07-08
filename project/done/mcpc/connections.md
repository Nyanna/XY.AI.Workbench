# MCPC-Extensions

Erweitere das Projekt um folgenden Anforderungen.

## Referenzen

* Projektverzeichnis: `/home/user/xyan/xy.ai.workbench/mcpc/src`
* Python Quellen: `/home/user/xyan/xy.ai.workbench/mcpc/src`
* Tool Pakete: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools`

## Session Konfiguration

Während die Registry alle Tools enthält, ist davon nur ein Subset Session-spezifisch aktiviert.
Das aktivierte Toolset wird auf Basis des HTTP-Header "X-MCPC-TOOLS" mit jedem Request neu in der Session gesetzt.

* Der Header enthält eine Kommaseparierte Liste der aktivierten Toolnamen.
* Innerhalb einer Tool-Implementierung muss es einfach möglich sein, die Liste der Aktiven Tools für eine bestimmte Session-ID zu konfigurieren.
* Es muss einfach möglich sein ein Session-Objekt für eine bestimmte ID im Vorfeld anzulegen bevor sich der Client erstmalig verbunden hat.

## Agenten-Tool

Implementiere ein Tool, das dafür gedacht ist Subagenten aufzurufen.
Subagenten sind dazu gedacht komplexe Arbeiten oder Arbeiten mit umfangreichem Kontext auszulagern, um den Hauptkontext zu entlasten oder schnellere oder spezialisierte Subagenten zu verwenden.
Das hier ein Subagent arbeitet ist für den aufrufenden Agenten nicht wichtig und auch nicht von einem normalen Toolaufruf zu unterscheiden.

* Die Inputparameter sind Prompt, Modell, Effort, Systemprompt und Resume
	* Prompt ist der angeforderte Prompt
	* Modell ist ein Enum bestehend aus:  haiku, sonnet, opus
	* Effort ist ein Enum bestehend aus: disabled, minimal, low, medium, high, xhigh
	* Systemprompt ist der zur Initialisierung verwendete Systemprompt
	* Resume ist eine optionale Session UUID
* Zurückgegeben wird die Antwort des Agenten oder mögliche Fehler.
* Wird das Agententool aufgerufen so wird eine UUID erzeugt und eine neue Session angelegt in der aktivierte Tools bereits gesetzt werden. Der nachfolgend startende Agent wird keinen "X-MCPC-TOOLS" Header mehr senden.
* Das Erzeugen einer Session speichert deren Erstellungszeitpunkt
* Wird Resume angegeben so wird versucht eine bestehende Session fortzusetzen. Die anderen Felder werden dann nicht mehr benötigt.
	
### CLI-Session-ID

Wird das Agententool verwendet so wird in der Session die CLI-Session-ID des Subagenten zusammen mit dem Zeitpunkt der letzten Verwendung hinterlegt. Eine Session wird ungültig, wenn sie seit mehr als einer Stunde nicht verwendet wurde (TTL).

* Wird Resume verwendet aber die Session kann nicht gefunden werden oder ist nicht mehr gültig wird ein entsprechender Fehler ausgegeben. 

### Prozessintanzierung

Für die eigentlichen CLI-Prozesse ist der CLI-Manager Verantwortlich: `/home/user/xyan/xy.ai.workbench/mcpc/project/sessionmanager.md`

## Wrapper

Lege Wrapper-Tools für die Profile agt-python, agt-markdown, agt-web-research, agt-github-research.
Der Wrapper dient der Verknüpfung eines Profils mit seiner Beschreibung und Systemprompt als Preset und ruft intern das Agent-Tool auf.

* Der Wrapper delegiert alle Aufrufe an das Agenten-Tool bis auf den Profile-Parameter und dem Systemprompt, die vorbelegt werden.
* Das Agenten-Tool wird selten direkt verwendet, sondern Hauptsächlich über die konkreten Wrapper aufgerufen.
* Die Toolbeschreibung enthält die Aufgabenbeschreibung des Agentenprofils aus der Konfiguration
* Eine Profilkonfiguration enthält auch eine Beschreibung, die im Agenten-Tool aber nicht ausgegeben wird.
* Eine Profilkonfiguration enthält auch einen Systemprompt

### Profile

Profile sind ein Alias für ein vorkonfiguriertes Toolset. Nicht alle referenzierten Tools sind bereits implementiert.

agt-python
:Tools: python
:Beschreibung: Python code tool that translates instructions, plans, and tasks into Python code, executes them inline, and handles errors autonomously.
:Systemprompt: TBD

agt-markdown
:Tools: markdown
:Beschreibung: Read, write, edit, and transform Markdown files.
:Systemprompt: TBD

agt-web-research
:Tools: exa, context7
:Beschreibung: Conducts structured web research, internet-based lookups, and external queries; aggregates comprehensive, prioritized results using Context7 and Exa MCP tools.
:Systemprompt: TBD

agt-github-research
:Tools: github
:Beschreibung: Conducts structured research on behalf of the caller and aggregates comprehensive, prioritized results using GitHub MCP tools for GitHub repositories.
:Systemprompt: TBD











