# Project Checklist

Comprehensive knowledge for agent creation

## (MCP-)Agent Rules

* All agents Answer short, precise and direct without explanation unless explicitly requested
	* Optimize every agent definition with AI
	* Spell corrections increases quality (LanguageTool Server with Docker)
	* Agents should not inteferre with calling agents decission making, no suggestions or follow up questions
* Isolated modular interfaces with skills and scripts
	* Agents talk and prompt to each other
	* Use Agent(web-research:web-research) tools definition to link and restrict subagents
	* Agents should have a meaningfull size. Not to small not to big to be token and salience efficient
	* Use in-place Pre/Postproccessor agents for extensive interfaces that justifies additional optimization costs
	* Use Advisors for out-of-proccess decision making and permission approvals
* Best prompt pattern is a structure soft-prompt
* Use model intuition with lazy tool injection on intercept
	* Model should report failing or not intercepted intuition like not enabled tools in the session to be able to recognize and correct them
	* User level hooks should not block subagents like spellchecking
	* Keep MCP minimal, e.g. don't use Exa advanced search for all agents - no context bloat
* Always check /Context and system prompt , injected agents and tools when changing/creating a agent
* Use code blocks with shell name for prompts: ```bash
* Use absolute file path with `/path/x.md`
* Group Context in skill or MD file trees, Models loads them lazy on demand, if required.

## Choose

Skill
: When simple, small or unchanged input/output or injectable context or context from commands

Agent
: When context isolation, post- or preprocessing makes sense to condense or clarify

Agent colors: red, blue, green, yellow, purple, orange, pink, cyan

## Known Bugs

* Issue #49713 (17. April 2026): „Plugin subagent namespace prefix stripped when launching subagent", workaround -> deny redirect
* Context still lists default and self plugin, empty, which is acceptable
* Current Bare-Mode don't supports Subscription OAuth
* MCP is preferred interface and will route other MCP by optimizing the interface specs. Thats better than CLI optimizing.