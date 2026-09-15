package xy.ai.workbench.connector.harness;

/** The (textual) result of a previously issued {@link ToolCall}. */
public final class ToolResult {
	public final String id;
	public final String content;

	public ToolResult(String id, String content) {
		this.id = id;
		this.content = content;
	}
}
