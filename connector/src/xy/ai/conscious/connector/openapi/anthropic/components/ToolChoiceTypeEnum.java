package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ToolChoiceTypeEnum {
    ANY("any");

    private final String value;

    ToolChoiceTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ToolChoiceTypeEnum fromValue(String value) {
        for (ToolChoiceTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ToolChoiceTypeEnum value: " + value);
    }
}
