package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum SucceededEnum {
    SUCCEEDED("succeeded");

    private final String value;

    SucceededEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static SucceededEnum fromValue(String value) {
        for (SucceededEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown SucceededEnum value: " + value);
    }
}
