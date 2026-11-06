package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ErrorErrorTypeEnum {
    NOT_FOUND_ERROR("not_found_error");

    private final String value;

    ErrorErrorTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ErrorErrorTypeEnum fromValue(String value) {
        for (ErrorErrorTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ErrorErrorTypeEnum value: " + value);
    }
}
