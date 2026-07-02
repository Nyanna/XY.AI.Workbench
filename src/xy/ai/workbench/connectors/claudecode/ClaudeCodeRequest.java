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
	/** When true the subprocess is terminated after the result is received (or immediately if promptJson is null). */
	public final boolean exitAfterResult;

	public ClaudeCodeRequest(String id, List<String> preMessages, String promptJson, Path workDir, Path outputJsonFile,
			boolean exitAfterResult) {
		this.id = id;
		this.preMessages = preMessages;
		this.promptJson = promptJson;
		this.workDir = workDir;
		this.outputJsonFile = outputJsonFile;
		this.exitAfterResult = exitAfterResult;
	}

	@Override
	public String getID() {
		return id;
	}
}
