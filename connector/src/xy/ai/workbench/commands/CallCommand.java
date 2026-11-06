package xy.ai.workbench.commands;

public class CallCommand extends Command {
	public static final String CMD_CALL = "/call";

	public CallCommand() {
		super();
	}

	@Override
	public String prefix() {
		return CallCommand.CMD_CALL;
	}

	@Override
	public boolean matches(String line) {
		return line != null && line.strip().startsWith(prefix());
	}

	@Override
	public Command parse(String line) {
		return new CallCommand();
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.REMOVE;
	}
}
