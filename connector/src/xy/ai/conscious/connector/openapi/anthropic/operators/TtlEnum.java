package xy.ai.conscious.connector.openapi.anthropic.operators;

/** Closed value set over string. */
public enum TtlEnum {
    _5M("5m"),
    _1H("1h");

    private final String value;

    TtlEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static TtlEnum fromValue(String value) {
        for (TtlEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown TtlEnum value: " + value);
    }
}
