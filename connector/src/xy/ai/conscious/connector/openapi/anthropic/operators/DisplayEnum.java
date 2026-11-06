package xy.ai.conscious.connector.openapi.anthropic.operators;

/** Closed value set over string. */
public enum DisplayEnum {
    SUMMARIZED("summarized"),
    OMITTED("omitted");

    private final String value;

    DisplayEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static DisplayEnum fromValue(String value) {
        for (DisplayEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown DisplayEnum value: " + value);
    }
}
