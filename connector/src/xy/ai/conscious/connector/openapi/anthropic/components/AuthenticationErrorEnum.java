package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum AuthenticationErrorEnum {
    AUTHENTICATION_ERROR("authentication_error");

    private final String value;

    AuthenticationErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static AuthenticationErrorEnum fromValue(String value) {
        for (AuthenticationErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown AuthenticationErrorEnum value: " + value);
    }
}
