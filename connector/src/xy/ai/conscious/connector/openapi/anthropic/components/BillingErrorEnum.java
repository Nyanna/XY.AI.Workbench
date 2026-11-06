package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum BillingErrorEnum {
    BILLING_ERROR("billing_error");

    private final String value;

    BillingErrorEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static BillingErrorEnum fromValue(String value) {
        for (BillingErrorEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown BillingErrorEnum value: " + value);
    }
}
