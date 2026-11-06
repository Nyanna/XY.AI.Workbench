package xy.ai.conscious.connector.mcp;

import xy.ai.conscious.model.IModelResponse;
import xy.ai.conscious.model.TokenStats;

public class MCPResponse implements IModelResponse {
	public final String id;
	public String resultText;
	public final TokenStats stats = new TokenStats();

	public MCPResponse(String id) {
		this.id = id;
	}

	public MCPResponse(String id, String resultText) {
		this.id = id;
		this.resultText = resultText;
	}
}
