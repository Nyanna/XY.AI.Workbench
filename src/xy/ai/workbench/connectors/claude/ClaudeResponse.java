package xy.ai.workbench.connectors.claude;

import com.anthropic.models.messages.Message;

import xy.ai.workbench.models.IModelResponse;

public class ClaudeResponse implements IModelResponse {
	public Message response;
	public final String id;

	public ClaudeResponse(Message resp, String id) {
		response = resp;
		this.id = id;
	}
}
