package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ResultErrorTypeEnum {
    ERROR("error");

    private final String value;

    ResultErrorTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ResultErrorTypeEnum fromValue(String value) {
        for (ResultErrorTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ResultErrorTypeEnum value: " + value);
    }
}
