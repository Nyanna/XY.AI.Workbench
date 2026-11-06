package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum EffortEnum {
    LOW("low"),
    MEDIUM("medium"),
    HIGH("high"),
    XHIGH("xhigh"),
    MAX("max");

    private final String value;

    EffortEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static EffortEnum fromValue(String value) {
        for (EffortEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown EffortEnum value: " + value);
    }
}
