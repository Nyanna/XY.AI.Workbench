package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum CanceledEnum {
    CANCELED("canceled");

    private final String value;

    CanceledEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static CanceledEnum fromValue(String value) {
        for (CanceledEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown CanceledEnum value: " + value);
    }
}
