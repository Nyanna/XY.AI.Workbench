package xy.ai.workbench.commands;

public abstract class Command {

	public enum ProcessorAction {
		REMOVE, TRANSFORM, IGNORE_BLOCK, NONE
	}

	protected final String[] parameters;

	protected Command(String... parameters) {
		this.parameters = parameters;
	}

	public abstract String prefix();

	public boolean matches(String line) {
		return line != null && line.strip().startsWith(prefix());
	}

	public abstract Command parse(String line);

	public abstract ProcessorAction processorAction();

	public String parameter(int i) {
		return parameters[i];
	}
}
