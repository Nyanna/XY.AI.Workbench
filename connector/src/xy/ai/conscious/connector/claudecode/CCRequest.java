package xy.ai.conscious.connector.claudecode;

import xy.ai.conscious.connector.harness.FrozenConfig;
import xy.ai.conscious.connector.harness.PromptArguments;
import xy.ai.conscious.model.AbstractModelRequest;

public class CCRequest extends AbstractModelRequest {

	public final String id;
	public final String title;

	public final FrozenConfig config;
	public final String promptText;
	public final PromptArguments arg;

	public CCRequest(String id, String title, FrozenConfig config, String promptText, PromptArguments arg) {
		this.id = id;
		this.title = title;
		this.config = config;
		this.promptText = promptText;
		this.arg = arg;
	}

	@Override
	public String getID() {
		return id;
	}
}
