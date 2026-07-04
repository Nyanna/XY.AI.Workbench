package xy.ai.workbench.editors.spellcheck;

public class SpellingProblem {

    private final int offset;
    private final int length;
    private final String message;

    public SpellingProblem(int offset, int length, String message) {
        this.offset = offset;
        this.length = length;
        this.message = message;
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
}
