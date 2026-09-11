package xy.ai.workbench.connector.mcp;

import java.util.Collections;
import java.util.List;
import java.util.Objects;

import xy.ai.workbench.models.IModelRequest;

public class MCPRequest implements IModelRequest {
	public final String id;
	public final String systemPrompt;
	public final List<String> tools;
	public final MCPCommand cmd;

	public MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd) {
		Objects.requireNonNull(cmd, "Command can't be null");
		this.id = id;
		this.systemPrompt = systemPrompt;
		this.tools = tools != null ? tools : Collections.emptyList();
		this.cmd = cmd;
	}

	@Override
	public String getID() {
		return id;
	}
}
