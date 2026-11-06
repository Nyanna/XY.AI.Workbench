package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum OverloadedErrorEnum {
    OVERLOADED_ERROR("overloaded_error");

    private final String value;

    OverloadedErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static OverloadedErrorEnum fromValue(String value) {
        for (OverloadedErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown OverloadedErrorEnum value: " + value);
    }
}
