package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum MessageTypeEnum {
    MESSAGE("message");

    private final String value;

    MessageTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static MessageTypeEnum fromValue(String value) {
        for (MessageTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown MessageTypeEnum value: " + value);
    }
}
