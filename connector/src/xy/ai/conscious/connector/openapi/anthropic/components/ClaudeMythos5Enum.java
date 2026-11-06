package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeMythos5Enum {
    CLAUDE_MYTHOS_5("claude-mythos-5");

    private final String value;

    ClaudeMythos5Enum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeMythos5Enum fromValue(String value) {
        for (ClaudeMythos5Enum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeMythos5Enum value: " + value);
    }
}
