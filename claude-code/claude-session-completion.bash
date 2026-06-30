# claude-session-completion.bash - Bash completion for claude-session.sh aliases.
# Source this file from ~/.bashrc.
# source "${HOME}/xyan/xy.ai.workbench/claude-code/claude-session-completion.bash"

_claude_session_agents() {
    local script_dir cur
    script_dir="${HOME}/xyan/xy.ai.workbench/claude-code"
    cur="${COMP_WORDS[COMP_CWORD]}"

    local agents=()
    for dir in "$script_dir"/*/; do
        [[ -d "${dir}agents" ]] && agents+=("$(basename "$dir")")
    done

    COMPREPLY=($(compgen -W "${agents[*]}" -- "$cur"))
}

complete -F _claude_session_agents claude-personal
complete -F _claude_session_agents claude-work
