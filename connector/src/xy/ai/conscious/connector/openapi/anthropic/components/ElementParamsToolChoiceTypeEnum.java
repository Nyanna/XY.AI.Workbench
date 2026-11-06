package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ElementParamsToolChoiceTypeEnum {
    NONE("none");

    private final String value;

    ElementParamsToolChoiceTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ElementParamsToolChoiceTypeEnum fromValue(String value) {
        for (ElementParamsToolChoiceTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ElementParamsToolChoiceTypeEnum value: " + value);
    }
}
