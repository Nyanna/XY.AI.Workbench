package xy.ai.workbench.connectors.claudecode;

import java.nio.file.Path;

import xy.ai.workbench.models.IModelRequest;

public class ClaudeCodeRequest implements IModelRequest {

	public final String id;
	public final String promptJson;
	public final Path workDir;
	public final Path outputJsonFile;

	public ClaudeCodeRequest(String id, String promptJson, Path workDir, Path outputJsonFile) {
		this.id = id;
		this.promptJson = promptJson;
		this.workDir = workDir;
		this.outputJsonFile = outputJsonFile;
	}

	@Override
	public String getID() {
		return id;
	}
}
