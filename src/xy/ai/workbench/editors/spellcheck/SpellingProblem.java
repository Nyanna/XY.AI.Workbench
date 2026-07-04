package xy.ai.workbench.editors.spellcheck;

import java.util.List;

public class SpellingProblem {

    private final int offset;
    private final int length;
    private final String message;
    private final List<String> suggestions;

    public SpellingProblem(int offset, int length, String message, List<String> suggestions) {
        this.offset = offset;
        this.length = length;
        this.message = message;
        this.suggestions = suggestions;
    }

    public int getOffset() {
        return offset;
    }

    public int getLength() {
        return length;
    }

    public String getMessage() {
        return message;
    }

    public List<String> getSuggestions() {
        return suggestions;
    }
}
