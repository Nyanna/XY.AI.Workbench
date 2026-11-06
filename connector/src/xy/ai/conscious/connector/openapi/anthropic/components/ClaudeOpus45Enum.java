package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeOpus45Enum {
    CLAUDE_OPUS_4_5("claude-opus-4-5");

    private final String value;

    ClaudeOpus45Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeOpus45Enum fromValue(String value) {
        for (ClaudeOpus45Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeOpus45Enum value: " + value);
    }
}
