package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ModelEnum {
    CLAUDE_SONNET_4_6("claude-sonnet-4-6");

    private final String value;

    ModelEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ModelEnum fromValue(String value) {
        for (ModelEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ModelEnum value: " + value);
    }
}
