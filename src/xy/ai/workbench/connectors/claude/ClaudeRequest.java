package xy.ai.workbench.connectors.claude;

import com.anthropic.models.messages.MessageCreateParams;

import xy.ai.workbench.models.IModelRequest;

public class ClaudeRequest implements IModelRequest {
	public static final String CUSTOM_ID = "customId";
	MessageCreateParams params;

	public ClaudeRequest(MessageCreateParams params) {
		this.params = params;
	}

	@Override
	public String getID() {
		if (params != null && params.metadata().isPresent())
			return params.metadata().get().userId().get();
		return "none";
	}
}
