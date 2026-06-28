# Project Checklist

Comprehensive knowledge database

## (MCP-)Agent Rules

* All agents Answer short, precise and direct without explanation unless explicitly requested
* Isolated modular interfaces with skills and scripts
	* Agents talk and prompt to each other
	* Use Agent(web-research) tools definition to link and restrict subagents
	* Agents should have a meaningfull size. Not to small not to big to be token and salience efficient
	* Use in-place Pre/Postproccessor agents for extensive interfaces that justifies additional optimization costs
	* Use Advisors for in-proccess decision making an permission approvals
* Best prompt pattern ist structure as soft-prompt
* Use modell intuition with lazy tool injection on intercept
	* Modell should report failing or not intercepted intuition like not enabled tools in the session
* Spell corrections increases quality
* Bind to model intuition for not internalized tools, lazy inject directions
* User level hooks should not block subagents like spellchecking
* Keep MCP minimal, e.g. don't use Exa advanced search for all agents
* Labor type Agents shoudl not inteferre  with calling agengs decission making, no suggestions or follow up questions


## Findings

* Use Jira style Markdown threads, continously thread for internal project organization
* Use local LanguageTool Server with Docker
* Set Effort and Thinking based on exspected input/output and total estimated token cost
* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Cancel claude pro and go soley for API keys? At least test a month.
* For exports use /copy and copy outputfile
* Current Bare-Mode don't supports Subscription OAuth
* MCP timeout via millisecond setting in MCP config
* MCP is preferred interface and will route other MCP by optimizing the interface specs. Thats better than CLI optimizing.

Command/Skill
: When simple, small or unchanged input/output

Agent
: When post- or preprocessing makes sense to condense or clarify
: Use pseudoagents via MCP


## Todo
* Suche möglichkeit von interaktivität mittel MCP controller -> retry session/mcp_interactive.md
	* eigenes SUbagent tool, Agent call abfangen, mit eigenen settings für modell und effort, wie funktioniert das agent tool replizieren, auch interface, auch andere agents und KI möglich
	* Alle commands, python markdown, cli können vom MCP controller abgebildet werden. Damit gibt es nur noch ein session/modell + kontext das sämtlicher tools beraubt nur noch den MCP controller als fenster zur welt hat. Alls hooks werden dahin umgeleitet.
	* Stream responses, Use --output-format stream-json with --verbose and --include-partial-messages
	* Tools wie Bash, Read, Write, Edit, Agent haben 5k Token, das meiste in den Beschreibungen -> session/tools_systemprompt.md -> mit MCP Controller tools ersetzen
	* MCP COntroller muss Agent tool mit einer custom liste pro aufrufendem subagent liefern, damit nur die erlaubten in den kontext geladen werden
	
* A:Remark, für remark subagent scripte bereitstellen, gehen agentenressourcen wie scripte? markdown ast parser und tool für mcp
* A: python3 direkt ohne shell, python-mcp, persistent MCP python server
* Skill: Inline latex, chapter formating, remove subchapter headings, as skill, formatingskill mit sonderzeichen und Pandoc kompatibelität
* add logging to all script to track an analyse execution and token costs. Whole analyzer tool für usage aggregation.
* Rag server bauen/installieren und einbinden für projektknowledge
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?
	* is this agent use correct?
	* violates this some target or increases costs or is meaningless?
	* is this request good or this tool usage?
	* Tool hook can be used to intercet and control
* Github PAT MCP for work profile {"type":"http","url":"https://api.githubcopilot.com/mcp","headers":{"Authorization":"Bearer YOUR_GITHUB_PAT"}}

### Ideas unformulated

* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* Wenn ich einen MCP controller habe. Brauche ich dann noch eine aufrufende container session oder ist koordination nicht ein subagent mit gecachtem prompt?
	* ein MCP controller erlaubt Infinite subagents, infinite recursion und separate permissions steuerung
* Splitt terminal verwenden für permission controll und session visualisierung über MCP controller
* maybe use in shell execution with !! in any subagents, for python
* statusline links hook, footerLinksRegexes, maybe for command approvals
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry

## Notes

When running with --agent or inside a subagent, two additional fields are included:
: agent_id	Unique identifier for the subagent. Present only when the hook fires inside a subagent call. Use this to distinguish subagent hook calls from main-thread calls.
: agent_type	Agent name (for example, "Explore" or "security-reviewer"). Present when the session uses --agent or the hook fires inside a subagent. For subagents, the subagent’s type takes precedence over the session’s --agent value. For custom subagents, this is the name field from the agent’s frontmatter, not the filename.

### MCP Agent Wrapper

Erstelle ein Bash-Skript, das eine Schnittstelle zu einem Subagenten darstellt. Das Skript nimmt Parameter von einer aufrufenden Instanz entgegen, wo es als Tool integriert ist, startet den Subagenten im non-interactive Modus (–print) und leitet die Ausgaben entsprechend zurück.

Das Skript unterstützt das Fortführen einer Session mittels Resume (--resume)
Das Ausgabeformat ist immer Stream-JSON (--output-format stream-json)
Der Ausgabestrom wird parallel im Unterverzeichnis "sessions/" mitgeschrieben. Der Dateiname beginnt mit dem Timecode im Format "YYMMDD.HHMMSS", gefolgt von dem Namen des Agents und einer zufälligen Hash aus 4 Hexadezimalzeichen.
