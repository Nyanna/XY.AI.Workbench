package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeFable5Enum {
    CLAUDE_FABLE_5("claude-fable-5");

    private final String value;

    ClaudeFable5Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeFable5Enum fromValue(String value) {
        for (ClaudeFable5Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeFable5Enum value: " + value);
    }
}
