package xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json;

/** Closed value set over string. */
public enum TypeEnum {
    MESSAGE_BATCH_DELETED("message_batch_deleted");

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
