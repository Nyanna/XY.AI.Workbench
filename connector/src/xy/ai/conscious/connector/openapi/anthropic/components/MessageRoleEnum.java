package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum MessageRoleEnum {
    ASSISTANT("assistant");

    private final String value;

    MessageRoleEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static MessageRoleEnum fromValue(String value) {
        for (MessageRoleEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown MessageRoleEnum value: " + value);
    }
}
