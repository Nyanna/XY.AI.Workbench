package xy.ai.workbench.connector.claudecode;

import java.nio.file.Path;

import xy.ai.workbench.commands.Command;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.models.IModelRequest;

public class CCRequest implements IModelRequest {

	public final String id;
	public final String title;

	public final FrozenConfig config;
	public final Command command;
	public final String promptText;
	public final String yamlBlock;
	public final String absoluteFilePath;
	public final Path projectPath;

	public CCRequest(String id, String title, FrozenConfig config, Command command, String promptText,
			String yamlBlock, String absoluteFilePath, Path projectPath) {
		this.id = id;
		this.title = title;
		this.config = config;
		this.command = command;
		this.promptText = promptText;
		this.yamlBlock = yamlBlock;
		this.absoluteFilePath = absoluteFilePath;
		this.projectPath = projectPath;
	}

	@Override
	public String getID() {
		return id;
	}
}
