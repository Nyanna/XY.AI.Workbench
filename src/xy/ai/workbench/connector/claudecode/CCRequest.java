package xy.ai.workbench.connector.claudecode;

import java.util.Objects;

import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.models.IModelRequest;

public class CCRequest implements IModelRequest {

	public final String id;
	public final String title;

	public final FrozenConfig config;
	public final Command cmd;

	public CCRequest(String id, String title, FrozenConfig config, Command cmd) {
		Objects.requireNonNull(cmd, "Command can't be null");
		this.id = id;
		this.title = title;
		this.config = config;
		this.cmd = cmd;
	}

	@Override
	public String getID() {
		return id;
	}

	public static class Command {
		public final CommandType type;
		public final String parameter;
		public final String[] parameters;

		public Command(CommandType type, String... parameters) {
			Objects.requireNonNull(type, "Type can't be null");
			this.type = type;
			this.parameter = parameters[0];
			this.parameters = parameters;
		}
	}
}
