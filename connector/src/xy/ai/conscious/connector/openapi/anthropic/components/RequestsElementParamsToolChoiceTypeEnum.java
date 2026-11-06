package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum RequestsElementParamsToolChoiceTypeEnum {
    TOOL("tool");

    private final String value;

    RequestsElementParamsToolChoiceTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static RequestsElementParamsToolChoiceTypeEnum fromValue(String value) {
        for (RequestsElementParamsToolChoiceTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown RequestsElementParamsToolChoiceTypeEnum value: " + value);
    }
}
