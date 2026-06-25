* All agents Answer short, precise and direct without explanation unless explicitly requested


## Rules

* Isolated modular interfaces with skills and scripts
* Agents talk and prompt to each other
* Best prompt pattern ist structure as soft-prompt
* Avoid MCP for overhead reason, preffer CLI
* Use modell intuition with lazy tool injection on intercept
* Modell should report failing or not intercepted intuition like not enabled tools in the session
* Spell corrections increases quality


## Findings
* Use local LanguageTool Server with Docker + Watchtower
* Exa is available as CLI but requieres extensive coding to implement a guided wizard for the agent to inject usage advice on demand. Without MCP overhead.

## Todo
* A:Remark, für remark subagent scripte bereitstellen, gehen agentenressourcen wie scripte? markdown ast parser und tool für mcp
* A: python3 direkt ohne shell, python-mcp, persistent MCP python server
* Skill: Inline latex, chapter formating, remove subchapter headings, as skill, formatingskill mit sonderzeichen und Pandoc kompatibelität
* Lanugagetool in workbench einbauen