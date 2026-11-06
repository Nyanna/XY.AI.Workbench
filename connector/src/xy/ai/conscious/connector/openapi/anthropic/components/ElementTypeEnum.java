package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ElementTypeEnum {
    CUSTOM("custom");

    private final String value;

    ElementTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ElementTypeEnum fromValue(String value) {
        for (ElementTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ElementTypeEnum value: " + value);
    }
}
