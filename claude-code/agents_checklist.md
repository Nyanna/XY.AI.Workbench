# Project Checklist

Comprehensive knowledge database

## (MCP-)Agent Rules

* All agents Answer short, precise and direct without explanation unless explicitly requested
* Isolated modular interfaces with skills and scripts
	* Agents talk and prompt to each other
	* Use Agent(web-research:web-research) tools definition to link and restrict subagents
	* Agents should have a meaningfull size. Not to small not to big to be token and salience efficient
	* Use in-place Pre/Postproccessor agents for extensive interfaces that justifies additional optimization costs
	* Use Advisors for out-of-proccess decision making an permission approvals
	* Agents shoudl not inteferre with calling agengs decission making, no suggestions or follow up questions
* Best prompt pattern is a structure soft-prompt
* Use modell intuition with lazy tool injection on intercept
	* Modell should report failing or not intercepted intuition like not enabled tools in the session
	* User level hooks should not block subagents like spellchecking
	* Keep MCP minimal, e.g. don't use Exa advanced search for all agents
* Spell corrections increases quality (LanguageTool Server with Docker)
* Always check /Context and system prompt , injected agents and tools when changing/creating a agent
* Use code blocks with shell name for prompts: ```bash
* Use absolute file path with `/path/x.md`
* Use external prompt injection single @/path/to/prompt
* Group Context in skill or MD file trees, Models loads them lazy on demand, if required.
* Always use a plan stage for big changes
* Complete a task withing 1h/5m cuz Prompt prefix caching (pro/api) 


## Findings

* Use Jira style Markdown threads, continously thread for internal project organization
* Set Effort and Thinking based on exspected input/output and total estimated token cost
* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* For exports use /copy and copy outputfile
* Current Bare-Mode don't supports Subscription OAuth
* MCP is preferred interface and will route other MCP by optimizing the interface specs. Thats better than CLI optimizing.

Skill
: When simple, small or unchanged input/output or injectable context or context from commands

Agent
: When context isolation, post- or preprocessing makes sense to condense or clarify

* Model is worth 50/150 USD API-Tokens, but API allows for Cache and Thinking optimization
	* Sonnet API   - Input: $3,   Output: $15,   5m Cache Writes: $3.75, 1h Cache Writes: $6, Hits/Refreshes: $0.3
	* Sonnet Batch - Input: $1.5, Output: $7.5

### Skills
* follow standard https://agentskills.io/home, Frontmatter: https://code.claude.com/docs/en/skills
* who can use the skill disable-model-invocation, user-invocable
* Has ARGUMENTS[1] placeholder and inline command execution !`gh pr diff` (dynamic contex)
	* ${CLAUDE_SKILL_DIR} for Bash directory hint


## Todo
! skill mit command execution schreiben der /copy output in eine zieldatei per argument kopiert
* python -c direkt benutzen

* Suche möglichkeit von interaktivität mittel MCP controller -> retry session/mcp_interactive.md
	* eigenes SUbagent tool, Agent call abfangen, mit eigenen settings für modell und effort, wie funktioniert das agent tool replizieren, auch interface, auch andere agents und KI möglich
	* Alle commands, python markdown, cli können vom MCP controller abgebildet werden. Damit gibt es nur noch ein session/modell + kontext das sämtlicher tools beraubt nur noch den MCP controller als fenster zur welt hat. Alls hooks werden dahin umgeleitet.
	* Stream responses, Use --output-format stream-json with --verbose and --include-partial-messages
	* Tools wie Bash, Read, Write, Edit, Agent haben 5k Token, das meiste in den Beschreibungen -> session/tools_systemprompt.md -> mit MCP Controller tools ersetzen
	* MCP timeout via millisecond setting in MCP config
	* MCP controller supports resume and optimiizes prompt cache
	
* Statuszeile kann token loggen, subagents token loggen wie? Monthly usage tracken auf die art. Mit session tracken in logfile
	* add logging to all script to track an analyse execution and token costs. Whole analyzer tool für usage aggregation.
* A: python3 direkt ohne shell, python-mcp, persistent MCP python server, via HEREDOC wie markdown
* Rag server bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?
	* is this agent use correct?
	* violates this some target or increases costs or is meaningless?
	* is this request good or this tool usage?
	* Tool hook can be used to intercet and control
* generische bash tool kompplet durch MCP controller ersetzen, intuition fällt ständig darauf zurück

### Ideas unformulated

* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* Wenn ich einen MCP controller habe. Brauche ich dann noch eine aufrufende container session oder ist koordination nicht ein subagent mit gecachtem prompt?
	* ein MCP controller erlaubt Infinite subagents, infinite recursion und separate permissions steuerung
* Splitt terminal verwenden für permission controll und session visualisierung über MCP controller
* statusline links hook, footerLinksRegexes, maybe for command approvals
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry

## Notes

When running with --agent or inside a subagent, two additional fields are included:
: agent_id	Unique identifier for the subagent. Present only when the hook fires inside a subagent call. Use this to distinguish subagent hook calls from main-thread calls.
: agent_type	Agent name (for example, "Explore" or "security-reviewer"). Present when the session uses --agent or the hook fires inside a subagent. For subagents, the subagent’s type takes precedence over the session’s --agent value. For custom subagents, this is the name field from the agent’s frontmatter, not the filename.

### MCP Agent Wrapper Leftover idea

Das Skript unterstützt das Fortführen einer Session mittels Resume (--resume)
Das Ausgabeformat ist immer Stream-JSON (--output-format stream-json)
Der Ausgabestrom wird parallel im Unterverzeichnis "sessions/" mitgeschrieben. Der Dateiname beginnt mit dem Timecode im Format "YYMMDD.HHMMSS", gefolgt von dem Namen des Agents und einer zufälligen Hash aus 4 Hexadezimalzeichen.

### Known Bugs

* Issue #49713 (17. April 2026): „Plugin subagent namespace prefix stripped when launching subagent"
* Context stil lists default and self plugin, which is acceptable