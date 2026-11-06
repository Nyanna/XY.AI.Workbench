package xy.ai.conscious.connector.openapi.anthropic.components;

/** Closed value set over string. */
public enum NameEnum {
    WEB_SEARCH("web_search"),
    WEB_FETCH("web_fetch"),
    CODE_EXECUTION("code_execution"),
    BASH_CODE_EXECUTION("bash_code_execution"),
    TEXT_EDITOR_CODE_EXECUTION("text_editor_code_execution"),
    TOOL_SEARCH_TOOL_REGEX("tool_search_tool_regex"),
    TOOL_SEARCH_TOOL_BM25("tool_search_tool_bm25");

    private final String value;

    NameEnum(String value) {
        this.value = value;
    }

    public String rawValue() {
        return value;
    }

    public static NameEnum fromValue(String value) {
        for (NameEnum candidate : values()) {
            if (candidate.value.equals(value)) {
                return candidate;
            }
        }
        throw new IllegalArgumentException("unknown NameEnum value: " + value);
    }
}
