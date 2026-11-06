package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ElementParamsModelEnum {
    CLAUDE_OPUS_4_7("claude-opus-4-7");

    private final String value;

    ElementParamsModelEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ElementParamsModelEnum fromValue(String value) {
        for (ElementParamsModelEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ElementParamsModelEnum value: " + value);
    }
}
