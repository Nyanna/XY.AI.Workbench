package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ParamsModelEnum {
    CLAUDE_SONNET_4_5("claude-sonnet-4-5");

    private final String value;

    ParamsModelEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ParamsModelEnum fromValue(String value) {
        for (ParamsModelEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ParamsModelEnum value: " + value);
    }
}
