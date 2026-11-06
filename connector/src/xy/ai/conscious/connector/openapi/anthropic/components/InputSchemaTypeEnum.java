package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum InputSchemaTypeEnum {
    OBJECT("object");

    private final String value;

    InputSchemaTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static InputSchemaTypeEnum fromValue(String value) {
        for (InputSchemaTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown InputSchemaTypeEnum value: " + value);
    }
}
