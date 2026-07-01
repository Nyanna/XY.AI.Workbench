# Project Checklist

Comprehensive knowledge database

## (MCP-)Agent Rules

* All agents Answer short, precise and direct without explanation unless explicitly requested
* Isolated modular interfaces with skills and scripts
	* Agents talk and prompt to each other
	* Use Agent(web-research:web-research) tools definition to link and restrict subagents
	* Agents should have a meaningfull size. Not to small not to big to be token and salience efficient
	* Use in-place Pre/Postproccessor agents for extensive interfaces that justifies additional optimization costs
	* Use Advisors for out-of-proccess decision making and permission approvals
	* Agents should not inteferre with calling agents decission making, no suggestions or follow up questions
* Best prompt pattern is a structure soft-prompt
* Use modell intuition with lazy tool injection on intercept
	* Modell should report failing or not intercepted intuition like not enabled tools in the session to be able to recognize and correct them
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
* Optimize every agent definition with AI


## Findings

! caching 5m/1h just up to 20 blocks back
* Use Jira style Markdown threads, continously thread for internal project organization
* Set Effort and Thinking based on exspected input/output and total estimated token cost
* Thinking cost is quality indicator for prompt. Better specified => less thinking.
	* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* For exports use /copy and copy outputfile
* Current Bare-Mode don't supports Subscription OAuth
* MCP is preferred interface and will route other MCP by optimizing the interface specs. Thats better than CLI optimizing.
` Agent colors: red, blue, green, yellow, purple, orange, pink, cyan
* Model is worth 50/150 USD API-Tokens, but API allows for Cache and Thinking optimization
	* Sonnet API   - Input: $3,   Output: $15,   5m Cache Writes: $3.75, 1h Cache Writes: $6, Hits/Refreshes: $0.3
	* Sonnet Batch - Input: $1.5, Output: $7.5
	
Skill
: When simple, small or unchanged input/output or injectable context or context from commands

Agent
: When context isolation, post- or preprocessing makes sense to condense or clarify


## Todo

*ask the user back tool, agent should have the possibillity to ask for user inforamtion when more efficient, enable ask back by plugin and do research before
* python agent ein python/ typescript(remark) sprachmodell geben/LSP/syntax parser/lint/prettier, als preproccessor, lanugageserver für python3
	* hm nur über hooks vor und nach tool usage nutzbar oder via MCP
	* Maybe produce something like skill as ouput so main kontext kann use scripts without knowing them in detail

* Rag server tool bauen/installieren und einbinden für projektknowledge retriefal statt grep/cat/ls
* Coordinator vs Advisor, use advisors for in-proccess decisions, like agent use and permissions
	* is this query related to context?
	* is this agent use correct?
	* violates this some target or increases costs or is meaningless?
	* is this request good or this tool usage?
	* Tool hook can be used to intercet and control


### Ideas unformulated

* in thought fragen abfangen und beantworten, mehr zwischenschritte zum einhaken
* Mehrstufige Aufgaben oder Prompts initial in Datenstruktur überführen mit klarer trennen der Steps, Aufgaben, Zwischenergebnisse und Zusammenführung. /session/multistep.md
* selbst lerne agenten die ihren prompt selbst modifizieren und persistieren. Quasi wie memorry
* AI self has the ability to decide abouts it's capacilities. I can match effort, modell structure and coordination of a federated mind
* stop oder intervention architektur. Je nach sweet spot 

## Known Bugs

* Issue #49713 (17. April 2026): „Plugin subagent namespace prefix stripped when launching subagent", workaround -> deny redirect
* Context still lists default and self plugin, empty, which is acceptable