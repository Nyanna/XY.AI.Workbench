package xy.ai.workbench.connector.claude;

import com.anthropic.models.messages.MessageCreateParams;

import xy.ai.workbench.models.AbstractModelRequest;

public class ClaudeRequest extends AbstractModelRequest {
	public static final String CUSTOM_ID = "customId";
	MessageCreateParams params;

	public ClaudeRequest(MessageCreateParams params) {
		this.params = params;
	}

	@Override
	public String getID() {
		return params.metadata().get().userId().get();
	}
}
