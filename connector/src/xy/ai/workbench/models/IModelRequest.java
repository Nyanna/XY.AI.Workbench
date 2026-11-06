package xy.ai.workbench.models;

import xy.ai.workbench.connector.harness.Prompt;

public interface IModelRequest {
	public String getID();

	public Prompt getPrompt();

	public void setPrompt(Prompt prompt);
}
