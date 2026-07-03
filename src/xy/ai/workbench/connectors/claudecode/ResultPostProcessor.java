package xy.ai.workbench.connectors.claudecode;

/**
 * Post-processes the text content of a Claude Code result event.
 *
 * <p>Strips the boilerplate wrapper that is emitted when a UserPromptSubmit hook
 * blocks an operation, reducing the output to the bare hook message:
 *
 * <pre>
 * Input pattern:
 *   UserPromptSubmit operation blocked by hook:
 *   [${CLAUDE_PLUGIN_ROOT}/scripts/spell-check.sh]:
 *   &lt;hook output&gt;
 *
 *
 *   Original prompt: &lt;original prompt&gt;
 *
 * Output:
 *   &lt;hook output&gt;
 * </pre>
 */
public class ResultPostProcessor {

    private static final String BLOCKED_PREFIX = "UserPromptSubmit operation blocked by hook:";
    private static final String ORIGINAL_PROMPT_PREFIX = "Original prompt:";

    /**
     * Applies all post-processing steps to {@code text} and returns the cleaned result.
     * If {@code text} is {@code null} or blank it is returned as-is.
     */
    public String process(String text) {
        if (text == null || text.isBlank())
            return text;

        text = extractHookOutput(text);

        return text;
    }

    /**
     * Detects the "operation blocked by hook" wrapper and, when present, extracts
     * only the hook output section.
     *
     * <p>The expected structure (after trimming) is:
     * <ol>
     *   <li>Line starting with {@value #BLOCKED_PREFIX}</li>
     *   <li>A bracket line identifying the hook script: {@code [...]}: </li>
     *   <li>The actual hook output (may span multiple lines)</li>
     *   <li>One or more blank lines</li>
     *   <li>A line starting with {@value #ORIGINAL_PROMPT_PREFIX}</li>
     * </ol>
     *
     * @return the hook output, or the original {@code text} if the pattern is not matched
     */
    private String extractHookOutput(String text) {
        String[] lines = text.split("\n", -1);

        // Find the "blocked by hook" header line
        int blockedLineIdx = -1;
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].stripLeading().startsWith(BLOCKED_PREFIX)) {
                blockedLineIdx = i;
                break;
            }
        }
        if (blockedLineIdx < 0)
            return text;

        // The very next non-empty line must be the "[script]: " bracket line
        int scriptLineIdx = -1;
        for (int i = blockedLineIdx + 1; i < lines.length; i++) {
            if (!lines[i].isBlank()) {
                String stripped = lines[i].strip();
                if (stripped.startsWith("[") && stripped.endsWith("]:")) {
                    scriptLineIdx = i;
                }
                break;
            }
        }
        if (scriptLineIdx < 0)
            return text;

        // Hook output starts on the line after the bracket line
        int hookStartIdx = scriptLineIdx + 1;

        // Find the "Original prompt:" line — hook output ends before it
        int originalPromptLineIdx = -1;
        for (int i = hookStartIdx; i < lines.length; i++) {
            if (lines[i].stripLeading().startsWith(ORIGINAL_PROMPT_PREFIX)) {
                originalPromptLineIdx = i;
                break;
            }
        }
        int hookEndIdx = (originalPromptLineIdx >= 0) ? originalPromptLineIdx : lines.length;

        // Collect hook output lines, stripping leading/trailing blank lines
        StringBuilder hookOutput = new StringBuilder();
        int firstContent = -1;
        int lastContent = -1;
        for (int i = hookStartIdx; i < hookEndIdx; i++) {
            if (!lines[i].isBlank()) {
                if (firstContent < 0)
                    firstContent = i;
                lastContent = i;
            }
        }
        if (firstContent < 0)
            return text; // no hook output found — keep original

        for (int i = firstContent; i <= lastContent; i++) {
            if (i > firstContent)
                hookOutput.append("\n");
            hookOutput.append(lines[i]);
        }

        return hookOutput.toString();
    }
}
