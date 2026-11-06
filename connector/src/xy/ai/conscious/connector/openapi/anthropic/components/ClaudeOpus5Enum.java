package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeOpus5Enum {
    CLAUDE_OPUS_5("claude-opus-5");

    private final String value;

    ClaudeOpus5Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeOpus5Enum fromValue(String value) {
        for (ClaudeOpus5Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeOpus5Enum value: " + value);
    }
}
