package xy.ai.workbench.connectors.claudecode;

import java.util.LinkedHashMap;

import xy.ai.workbench.models.IModelResponse;

public class CCResponse implements IModelResponse {

	public final String id;
	public String resultText;
	public boolean isError;
	/**
	 * True when this response represents a pending tool_use request rather than a
	 * final result.
	 */
	public boolean isToolRequest;
	/** The tool_use id when isToolRequest is true, otherwise null. */
	public String toolUseId;

	public long inputTokens;
	public long outputTokens;
	public long reasoningTokens;
	public long cacheReadInputTokens;
	public long cacheCreationInputTokens;
	public long totalReasoningTokens;
	public final LinkedHashMap<String, String> events = new LinkedHashMap<>();

	public CCResponse(String id) {
		this.id = id;
		this.isToolRequest = false;
		this.toolUseId = null;
	}

	public CCResponse(String id, String resultText) {
		this.id = id;
		this.resultText = resultText;
	}

	public void setToolUse(String resultText, String toolUseId) {
		this.resultText = resultText;
		this.isToolRequest = true;
		this.toolUseId = toolUseId;
	}

	public boolean isReady() {
		return resultText != null;
	}
}
