* All agents Answer short, precise and direct without explanation unless explicitly requested


## Rules

* Isolated modular interfaces with skills and scripts
* Agents talk and prompt to each other
* Best prompt pattern ist structure as soft-prompt
* Avoid MCP for overhead reason, preffer CLI
* Use modell intuition with lazy tool injection on intercept
* Modell should report failing or not intercepted intuition like not enabled tools in the session
* Spell corrections increases quality
* Bind to model intuition for not internalized tools, lazy inject directions


## Findings
* Use local LanguageTool Server with Docker + Watchtower
* Exa is available as CLI but requieres extensive coding to implement a guided wizard for the agent to inject usage advice on demand. Without MCP overhead.
* use and integrate commands
* context7 is available asl CLI with command, agent and skill
* Set Effort and Thinking based on exspected input/output and total estimated token cost
* Thinking cost is quality indicator for prompt. Better specified => less thinking.
* Thinking costs measures resistance against prompt. Two high thinking costs and the prompt should optiimized an tuned to the model.
* Cancel claude pro and go soley for API keys? At least test a month.

## Todo
* eigenes SUbagent tool, Agent call abfangen, mit eigenen settings für modell und effort, wie funktioniert das agent tool replizieren, auch interface, auch andere agents und KI möglich
* A:Remark, für remark subagent scripte bereitstellen, gehen agentenressourcen wie scripte? markdown ast parser und tool für mcp
* A: python3 direkt ohne shell, python-mcp, persistent MCP python server
* Skill: Inline latex, chapter formating, remove subchapter headings, as skill, formatingskill mit sonderzeichen und Pandoc kompatibelität
* Lanugagetool in workbench einbauen
* workbench, session branching und prefix cache support, bessere tool loops
* add logging to all script to track an analyse execution and token costs. Whole analyzer tool für usage aggregation.