package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ResultErrorErrorTypeEnum {
    API_ERROR("api_error");

    private final String value;

    ResultErrorErrorTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ResultErrorErrorTypeEnum fromValue(String value) {
        for (ResultErrorErrorTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ResultErrorErrorTypeEnum value: " + value);
    }
}
