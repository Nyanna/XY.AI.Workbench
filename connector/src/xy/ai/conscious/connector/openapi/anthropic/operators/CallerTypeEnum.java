package xy.ai.conscious.connector.openapi.anthropic.operators;

/** Closed value set over string. */
public enum CallerTypeEnum {
    DIRECT("direct");

    private final String value;

    CallerTypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static CallerTypeEnum fromValue(String value) {
        for (CallerTypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown CallerTypeEnum value: " + value);
    }
}
