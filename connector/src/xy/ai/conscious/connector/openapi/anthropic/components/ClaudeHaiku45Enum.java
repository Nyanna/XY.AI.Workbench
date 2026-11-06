package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeHaiku45Enum {
    CLAUDE_HAIKU_4_5("claude-haiku-4-5");

    private final String value;

    ClaudeHaiku45Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeHaiku45Enum fromValue(String value) {
        for (ClaudeHaiku45Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeHaiku45Enum value: " + value);
    }
}
