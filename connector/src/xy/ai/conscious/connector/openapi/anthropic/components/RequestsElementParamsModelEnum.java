package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum RequestsElementParamsModelEnum {
    CLAUDE_SONNET_4_5_20250929("claude-sonnet-4-5-20250929");

    private final String value;

    RequestsElementParamsModelEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static RequestsElementParamsModelEnum fromValue(String value) {
        for (RequestsElementParamsModelEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown RequestsElementParamsModelEnum value: " + value);
    }
}
