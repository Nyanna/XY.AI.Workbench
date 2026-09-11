package xy.ai.workbench.connector.mcp;

import java.util.Objects;

public class MCPCommand {
	public final MCPCommandType type;
	public final String parameter;
	public final String[] parameters;

	public MCPCommand(MCPCommandType type, String... parameters) {
		Objects.requireNonNull(type, "Type can't be null");
		this.type = type;
		this.parameter = parameters[0];
		this.parameters = parameters;
	}
}
