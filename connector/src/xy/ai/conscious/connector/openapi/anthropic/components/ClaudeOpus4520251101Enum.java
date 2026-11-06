package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeOpus4520251101Enum {
    CLAUDE_OPUS_4_5_20251101("claude-opus-4-5-20251101");

    private final String value;

    ClaudeOpus4520251101Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeOpus4520251101Enum fromValue(String value) {
        for (ClaudeOpus4520251101Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeOpus4520251101Enum value: " + value);
    }
}
