package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum StopDetailsTypeEnum {
    REFUSAL("refusal");

    private final String value;

    StopDetailsTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static StopDetailsTypeEnum fromValue(String value) {
        for (StopDetailsTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown StopDetailsTypeEnum value: " + value);
    }
}
