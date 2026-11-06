package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum RateLimitErrorEnum {
    RATE_LIMIT_ERROR("rate_limit_error");

    private final String value;

    RateLimitErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static RateLimitErrorEnum fromValue(String value) {
        for (RateLimitErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown RateLimitErrorEnum value: " + value);
    }
}
