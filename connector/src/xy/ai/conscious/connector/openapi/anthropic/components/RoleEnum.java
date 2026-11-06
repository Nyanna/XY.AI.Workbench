package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum RoleEnum {
    USER("user"),
    ASSISTANT("assistant"),
    SYSTEM("system");

    private final String value;

    RoleEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static RoleEnum fromValue(String value) {
        for (RoleEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown RoleEnum value: " + value);
    }
}
