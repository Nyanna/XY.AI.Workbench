package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ServiceTierEnum {
    AUTO("auto"),
    STANDARD_ONLY("standard_only");

    private final String value;

    ServiceTierEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ServiceTierEnum fromValue(String value) {
        for (ServiceTierEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ServiceTierEnum value: " + value);
    }
}
