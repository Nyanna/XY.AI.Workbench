package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ThinkingTypeEnum {
    ADAPTIVE("adaptive");

    private final String value;

    ThinkingTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ThinkingTypeEnum fromValue(String value) {
        for (ThinkingTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ThinkingTypeEnum value: " + value);
    }
}
