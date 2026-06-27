* All agents Answer short, precise and direct without explanation unless explicitly requested


## Rules

* Isolated modular interfaces with skills and scripts
* Agents talk and prompt to each other
* Best prompt pattern ist structure as soft-prompt
* Avoid MCP for overhead reason, preffer CLI. (MCP is bloated and not intuition optimized)
* Use modell intuition with lazy tool injection on intercept
* Modell should report failing or not intercepted intuition like not enabled tools in the session
* Spell corrections increases quality
* Bind to model intuition for not internalized tools, lazy inject directions
* User level hooks should non block subagents, check for subagent context
* Agents should have a meaningfull size. Not to small not to big to be token and salience efficient


## Findings
* Use local LanguageTool Server with Docker
* Exa is available as CLI but requieres extensive coding to implement a guided wizard for the agent to inject usage advice on demand. Without MCP overhead.
* use and integrate commands
* context7 is available asl CLI with command, agent and skill
* Set Effort and Thinking based on exspected input/output and total estimated token cost
* Thinking cost is quality indicator for prompt. Better specified => less thinking.
* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Cancel claude pro and go soley for API keys? At least test a month.
* For exports use /copy, press W, enter filename
* Current Bare-Mode don't supports Subscription OAuth

Command/Skill
: When simple, small or unchanged input/output

Agent
: When post- or preprocessing makes sense to condense or clarify

## Todo
Subagent tool verbieten mit redirect zum bash wrapper script
* eigenes SUbagent tool, Agent call abfangen, mit eigenen settings für modell und effort, wie funktioniert das agent tool replizieren, auch interface, auch andere agents und KI möglich
* A:Remark, für remark subagent scripte bereitstellen, gehen agentenressourcen wie scripte? markdown ast parser und tool für mcp
* A: python3 direkt ohne shell, python-mcp, persistent MCP python server
* Skill: Inline latex, chapter formating, remove subchapter headings, as skill, formatingskill mit sonderzeichen und Pandoc kompatibelität
* Lanugagetool in workbench einbauen
* workbench, session branching und prefix cache support, bessere tool loops
* add logging to all script to track an analyse execution and token costs. Whole analyzer tool für usage aggregation.
* selbst lerne agenten die ihren prompt selbst modifizieren
* Rag server bauen
* github access 
* command line MCP client that bypasses context pollution
* context7 und exa CLI migration erübrigt sich durch zentralen MCP controller. Dieser filter und restrukturiert die beiden nach intuition
* Alle commands, python markdown, cli können vom MCP controller abgebildet werden. Damit gibt es nur noch ein session/modell + kontext das sämtlicher tools beraubt nur noch den MCP controller als fenster zur welt hat. Alls hooks werden dahin umgeleitet.
* Kein extended exa und proxy MCP mit permissions, extended exa deaktivieren das belastet den promt zu sehr und es gibt ohnehin keine advanced intuition für exa
* Stream responses, Use --output-format stream-json with --verbose and --include-partial-messages

### Ideas unformulated

* der web-researcher hat entscheidungsempfehlungen gegeben (schlecht)
	* Im prompt Anweisung keine Handlungsempfehlungen zu geben. Das ist Aufgabe des Coordinator
	* Diese gegenanweisung frisst salienz
	* Ein zwischenAgent könnte die Anweisungen filtern (Was ist besser oder günstiger?)
* Zwischenagents könntenm generell Prompt und Result Pre/Postproccessing machen. In-place prompt optimize.
	* Das Kostet Token oder verschlechtert vielleicht alles
* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* Wenn ich einen MCP controller habe. Brauche ich dann noch eine aufrufende container session oder ist koordination nicht ein subagent mit gecachtem prompt?
* ein MCP controller erlaubt Infinite subagents, infinite recursion und separate permissions steuerung
* ermissions hook zur umlritung kann zur umlenkung und steuerung in den MCP verwendet werden wenn er ohne timout läuft
* Splitt terminal verwenden für permission controll und session visualisierung über MCP controller
* maybe use in shell execution with !! in any subagents, for python
* statusline links hook, footerLinksRegexes, maybe for command approvals

## Notes
When running with --agent or inside a subagent, two additional fields are included:
: agent_id	Unique identifier for the subagent. Present only when the hook fires inside a subagent call. Use this to distinguish subagent hook calls from main-thread calls.
: agent_type	Agent name (for example, "Explore" or "security-reviewer"). Present when the session uses --agent or the hook fires inside a subagent. For subagents, the subagent’s type takes precedence over the session’s --agent value. For custom subagents, this is the name field from the agent’s frontmatter, not the filename.