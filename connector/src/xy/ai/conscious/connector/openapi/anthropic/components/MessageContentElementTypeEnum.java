package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum MessageContentElementTypeEnum {
    TOOL_USE("tool_use");

    private final String value;

    MessageContentElementTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static MessageContentElementTypeEnum fromValue(String value) {
        for (MessageContentElementTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown MessageContentElementTypeEnum value: " + value);
    }
}
