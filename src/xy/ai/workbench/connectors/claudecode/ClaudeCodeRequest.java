package xy.ai.workbench.connectors.claudecode;

import xy.ai.workbench.models.IModelRequest;

public class ClaudeCodeRequest implements IModelRequest {

	public final String id;

	public final String thissystemPrompt;
	/** Prompt JSON to send after preMessages, or null if no remaining text. */
	public final String promptJson;

	public final boolean exitAfterResult;

	// UUID extracted from a {@code /resume <uuid>} command
	public final String resumeUuid;

	public final String title;

	public ClaudeCodeRequest(String id, String title, String systemPrompt, String promptJson, boolean exitAfterResult, String resumeUuid) {
		this.id = id;
		this.title = title;
		thissystemPrompt = systemPrompt;
		this.promptJson = promptJson;
		this.exitAfterResult = exitAfterResult;
		this.resumeUuid = resumeUuid;
	}

	@Override
	public String getID() {
		return id;
	}
}
