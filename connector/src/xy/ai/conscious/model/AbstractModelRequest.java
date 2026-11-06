package xy.ai.conscious.model;

import xy.ai.conscious.connector.harness.Prompt;

public abstract class AbstractModelRequest implements IModelRequest {

	private Prompt prompt;

	@Override
	public Prompt getPrompt() {
		return prompt;
	}

	@Override
	public void setPrompt(Prompt prompt) {
		this.prompt = prompt;
	}
}
