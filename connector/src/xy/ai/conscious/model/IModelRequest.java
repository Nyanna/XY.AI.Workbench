package xy.ai.conscious.model;

import xy.ai.conscious.harness.Prompt;

public interface IModelRequest {
	public String getID();

	public Prompt getPrompt();

	public void setPrompt(Prompt prompt);
}
