package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ContentElementTypeEnum {
    SERVER_TOOL_USE("server_tool_use");

    private final String value;

    ContentElementTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ContentElementTypeEnum fromValue(String value) {
        for (ContentElementTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ContentElementTypeEnum value: " + value);
    }
}
