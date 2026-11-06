package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ElementParamsThinkingTypeEnum {
    ENABLED("enabled");

    private final String value;

    ElementParamsThinkingTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ElementParamsThinkingTypeEnum fromValue(String value) {
        for (ElementParamsThinkingTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ElementParamsThinkingTypeEnum value: " + value);
    }
}
