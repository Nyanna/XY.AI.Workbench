package xy.ai.conscious.connector.claude;

import com.anthropic.models.messages.MessageCreateParams;

import xy.ai.conscious.model.AbstractModelRequest;

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
