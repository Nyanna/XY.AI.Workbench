package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum StopReasonEnum {
    END_TURN("end_turn"),
    MAX_TOKENS("max_tokens"),
    STOP_SEQUENCE("stop_sequence"),
    TOOL_USE("tool_use"),
    PAUSE_TURN("pause_turn"),
    REFUSAL("refusal"),
    MODEL_CONTEXT_WINDOW_EXCEEDED("model_context_window_exceeded");

    private final String value;

    StopReasonEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static StopReasonEnum fromValue(String value) {
        for (StopReasonEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown StopReasonEnum value: " + value);
    }
}
