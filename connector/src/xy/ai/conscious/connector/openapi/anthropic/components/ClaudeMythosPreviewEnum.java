package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum ClaudeMythosPreviewEnum {
    CLAUDE_MYTHOS_PREVIEW("claude-mythos-preview");

    private final String value;

    ClaudeMythosPreviewEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static ClaudeMythosPreviewEnum fromValue(String value) {
        for (ClaudeMythosPreviewEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown ClaudeMythosPreviewEnum value: " + value);
    }
}
