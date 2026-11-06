package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ErrorTypeEnum {
    INVALID_REQUEST_ERROR("invalid_request_error");

    private final String value;

    ErrorTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ErrorTypeEnum fromValue(String value) {
        for (ErrorTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ErrorTypeEnum value: " + value);
    }
}
