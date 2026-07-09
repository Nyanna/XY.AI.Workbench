package xy.ai.workbench.connectors.claudecode;

import java.util.Collections;
import java.util.List;

import xy.ai.workbench.models.IModelRequest;

public class ClaudeCodeRequest implements IModelRequest {

	public final String id;

	public final String systemPrompt;
	public final List<String> tools;
	/** Prompt JSON to send after preMessages, or null if no remaining text. */
	public final String promptJson;

	public final boolean exitAfterResult;

	// UUID extracted from a {@code /resume <uuid>} command
	public final String resumeUuid;

	public final String title;

	public ClaudeCodeRequest(String id, String title, String systemPrompt, List<String> tools, String promptJson,
			boolean exitAfterResult, String resumeUuid) {
		this.id = id;
		this.title = title;
		this.systemPrompt = systemPrompt;
		this.tools = tools != null ? tools : Collections.emptyList();
		this.promptJson = promptJson;
		this.exitAfterResult = exitAfterResult;
		this.resumeUuid = resumeUuid;
	}

	@Override
	public String getID() {
		return id;
	}
}
