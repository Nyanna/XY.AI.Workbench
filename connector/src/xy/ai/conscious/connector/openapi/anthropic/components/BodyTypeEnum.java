package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum BodyTypeEnum {
    MESSAGE_BATCH("message_batch");

    private final String value;

    BodyTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static BodyTypeEnum fromValue(String value) {
        for (BodyTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown BodyTypeEnum value: " + value);
    }
}
