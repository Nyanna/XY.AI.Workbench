package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeOpus48Enum {
    CLAUDE_OPUS_4_8("claude-opus-4-8");

    private final String value;

    ClaudeOpus48Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeOpus48Enum fromValue(String value) {
        for (ClaudeOpus48Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeOpus48Enum value: " + value);
    }
}
