package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ParamsThinkingTypeEnum {
    DISABLED("disabled");

    private final String value;

    ParamsThinkingTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ParamsThinkingTypeEnum fromValue(String value) {
        for (ParamsThinkingTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ParamsThinkingTypeEnum value: " + value);
    }
}
