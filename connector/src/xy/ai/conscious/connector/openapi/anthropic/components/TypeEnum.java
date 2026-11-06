package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum TypeEnum {
    ERRORED("errored");

    private final String value;

    TypeEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static TypeEnum fromValue(String value) {
        for (TypeEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown TypeEnum value: " + value);
    }
}
