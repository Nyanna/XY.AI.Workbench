package xy.ai.workbench.connectors.claudecode;

import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeResponse implements IModelResponse {

	public final String id;
	public final String resultText;
	public final boolean isError;

	public long inputTokens;
	public long outputTokens;
	public long cacheReadInputTokens;
	public long cacheCreationInputTokens;

	public ClaudeCodeResponse(String id, String resultText, boolean isError) {
		this.id = id;
		this.resultText = resultText;
		this.isError = isError;
	}
}
