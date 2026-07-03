# Resolved at source time; HOME is always available
_CLAUDE_SESSION="${HOME}/xyan/xy.ai.workbench/claude-code/claude-session.sh"
_LT_COMPOSE_FILE="${HOME}/xyan/xy.ai.workbench/language-tool/docker-compose.yml"

alias claude-personal="${_CLAUDE_SESSION} --profile personal"
alias claude-work="${_CLAUDE_SESSION} --profile work"
alias languagetool="docker compose -f ${_LT_COMPOSE_FILE}"
