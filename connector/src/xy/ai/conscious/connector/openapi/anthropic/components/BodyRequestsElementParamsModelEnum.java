package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum BodyRequestsElementParamsModelEnum {
    CLAUDE_HAIKU_4_5_20251001("claude-haiku-4-5-20251001");

    private final String value;

    BodyRequestsElementParamsModelEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static BodyRequestsElementParamsModelEnum fromValue(String value) {
        for (BodyRequestsElementParamsModelEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown BodyRequestsElementParamsModelEnum value: " + value);
    }
}
