package xy.ai.workbench.connector.mcp;

import java.util.Collections;
import java.util.List;
import java.util.Objects;

import xy.ai.workbench.commands.Command;
import xy.ai.workbench.models.IModelRequest;

public class MCPRequest implements IModelRequest {
	public final String id;
	public final String systemPrompt;
	public final List<String> tools;
	public final Command command;
	public final String yamlBlock;

	public MCPRequest(String id, String systemPrompt, List<String> tools, Command command, String yamlBlock) {
		Objects.requireNonNull(command, "Command can't be null");
		this.id = id;
		this.systemPrompt = systemPrompt;
		this.tools = tools != null ? tools : Collections.emptyList();
		this.command = command;
		this.yamlBlock = yamlBlock;
	}

	@Override
	public String getID() {
		return id;
	}
}
