package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeSonnet5Enum {
    CLAUDE_SONNET_5("claude-sonnet-5");

    private final String value;

    ClaudeSonnet5Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeSonnet5Enum fromValue(String value) {
        for (ClaudeSonnet5Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeSonnet5Enum value: " + value);
    }
}
