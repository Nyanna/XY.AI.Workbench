package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ResultTypeEnum {
    EXPIRED("expired");

    private final String value;

    ResultTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ResultTypeEnum fromValue(String value) {
        for (ResultTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ResultTypeEnum value: " + value);
    }
}
