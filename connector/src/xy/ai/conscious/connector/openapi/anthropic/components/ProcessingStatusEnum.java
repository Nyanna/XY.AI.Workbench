package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ProcessingStatusEnum {
    IN_PROGRESS("in_progress"),
    CANCELING("canceling"),
    ENDED("ended");

    private final String value;

    ProcessingStatusEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ProcessingStatusEnum fromValue(String value) {
        for (ProcessingStatusEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ProcessingStatusEnum value: " + value);
    }
}
