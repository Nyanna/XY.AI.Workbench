package xy.ai.workbench.connectors.claudecode;

import java.nio.file.Path;
import java.util.List;

import xy.ai.workbench.models.IModelRequest;

public class ClaudeCodeRequest implements IModelRequest {

	public final String id;
	/** Approve/deny JSON messages to send before the prompt. Never null. */
	public final List<String> preMessages;
	/** Prompt JSON to send after preMessages, or null if no remaining text. */
	public final String promptJson;
	public final Path workDir;
	public final Path outputJsonFile;

	public ClaudeCodeRequest(String id, List<String> preMessages, String promptJson, Path workDir, Path outputJsonFile) {
		this.id = id;
		this.preMessages = preMessages;
		this.promptJson = promptJson;
		this.workDir = workDir;
		this.outputJsonFile = outputJsonFile;
	}

	@Override
	public String getID() {
		return id;
	}
}
