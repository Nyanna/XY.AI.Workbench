package xy.ai.workbench.models;

import xy.ai.workbench.connector.harness.Prompt;

/** Base implementation of {@link IModelRequest} holding the originating {@link Prompt}. */
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
