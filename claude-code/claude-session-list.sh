# claude-session-list.sh - Lists available agents with their descriptions.
# Intended to be sourced by claude-session.sh; not executed directly.
#
# Requires: SCRIPT_DIR to be set by the caller.
#
# Usage: list_agents

list_agents() {
    local found=0

    echo "Available agents:"
    echo ""

    for dir in "$SCRIPT_DIR"/*/; do
        [[ -d "${dir}agents" ]] || continue

        local agent_name
        agent_name="$(basename "$dir")"

        local agent_file="${dir}agents/${agent_name}.md"
        [[ -f "$agent_file" ]] || continue

        # Extract description from frontmatter (between first pair of ---)
        local description
        description="$(awk '
            /^---/ { if (++n == 2) exit; next }
            n == 1 && /^description:[[:space:]]*/ {
                sub(/^description:[[:space:]]*/, "")
                print
            }
        ' "$agent_file")"

        printf "  %-22s %s\n" "$agent_name" "${description:-(no description)}"
        ((found++)) || true
    done

    echo ""
    if [[ $found -eq 0 ]]; then
        echo "No agents found in $SCRIPT_DIR"
    else
        echo "$found agent(s) found."
    fi
}
