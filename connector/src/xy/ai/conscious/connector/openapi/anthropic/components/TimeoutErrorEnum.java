package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum TimeoutErrorEnum {
    TIMEOUT_ERROR("timeout_error");

    private final String value;

    TimeoutErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static TimeoutErrorEnum fromValue(String value) {
        for (TimeoutErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown TimeoutErrorEnum value: " + value);
    }
}
