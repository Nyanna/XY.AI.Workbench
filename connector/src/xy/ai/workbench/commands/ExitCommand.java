package xy.ai.workbench.commands;

public class ExitCommand extends Command {
	public static final String CMD_EXIT = "/exit";

	public ExitCommand() {
		super();
	}

	@Override
	public String prefix() {
		return ExitCommand.CMD_EXIT;
	}

	@Override
	public boolean matches(String line) {
		return line != null && prefix().equalsIgnoreCase(line.strip());
	}

	@Override
	public Command parse(String line) {
		return new ExitCommand();
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.REMOVE;
	}
}
