package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeOpus46Enum {
    CLAUDE_OPUS_4_6("claude-opus-4-6");

    private final String value;

    ClaudeOpus46Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeOpus46Enum fromValue(String value) {
        for (ClaudeOpus46Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeOpus46Enum value: " + value);
    }
}
