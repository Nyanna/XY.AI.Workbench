package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum CategoryEnum {
    CYBER("cyber"),
    BIO("bio"),
    FRONTIER_LLM("frontier_llm"),
    REASONING_EXTRACTION("reasoning_extraction"),
    GENERAL_HARMS("general_harms");

    private final String value;

    CategoryEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static CategoryEnum fromValue(String value) {
        for (CategoryEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown CategoryEnum value: " + value);
    }
}
