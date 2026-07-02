package xy.ai.workbench.connectors.claudecode;

import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeResponse implements IModelResponse {

	public final String id;
	public final String resultText;
	public final boolean isError;
	/** True when this response represents a pending tool_use request rather than a final result. */
	public final boolean isToolRequest;
	/** The tool_use id when isToolRequest is true, otherwise null. */
	public final String toolUseId;

	public long inputTokens;
	public long outputTokens;
	public long cacheReadInputTokens;
	public long cacheCreationInputTokens;
	/** True when the subprocess was terminated as part of handling a /exit command. */
	public boolean isExited;

	public ClaudeCodeResponse(String id, String resultText, boolean isError) {
		this.id = id;
		this.resultText = resultText;
		this.isError = isError;
		this.isToolRequest = false;
		this.toolUseId = null;
	}

	public ClaudeCodeResponse(String id, String resultText, String toolUseId) {
		this.id = id;
		this.resultText = resultText;
		this.isError = false;
		this.isToolRequest = true;
		this.toolUseId = toolUseId;
	}
}
