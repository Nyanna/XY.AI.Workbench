package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum UsageServiceTierEnum {
    STANDARD("standard"),
    PRIORITY("priority"),
    BATCH("batch");

    private final String value;

    UsageServiceTierEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static UsageServiceTierEnum fromValue(String value) {
        for (UsageServiceTierEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown UsageServiceTierEnum value: " + value);
    }
}
