package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum FormatTypeEnum {
    JSON_SCHEMA("json_schema");

    private final String value;

    FormatTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static FormatTypeEnum fromValue(String value) {
        for (FormatTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown FormatTypeEnum value: " + value);
    }
}
