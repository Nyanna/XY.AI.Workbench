package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ParamsToolChoiceTypeEnum {
    AUTO("auto");

    private final String value;

    ParamsToolChoiceTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ParamsToolChoiceTypeEnum fromValue(String value) {
        for (ParamsToolChoiceTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ParamsToolChoiceTypeEnum value: " + value);
    }
}
