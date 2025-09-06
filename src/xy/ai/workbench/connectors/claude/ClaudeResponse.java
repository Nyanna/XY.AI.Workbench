package xy.ai.workbench.connectors.claude;

import com.anthropic.models.messages.Message;

import xy.ai.workbench.models.IModelResponse;

public class ClaudeResponse implements IModelResponse {
	public Message response;

	public ClaudeResponse(Message resp) {
		response = resp;
	}
}
