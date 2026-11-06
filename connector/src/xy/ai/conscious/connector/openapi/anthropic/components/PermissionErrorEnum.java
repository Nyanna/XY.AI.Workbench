package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum PermissionErrorEnum {
    PERMISSION_ERROR("permission_error");

    private final String value;

    PermissionErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static PermissionErrorEnum fromValue(String value) {
        for (PermissionErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown PermissionErrorEnum value: " + value);
    }
}
