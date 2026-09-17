package xy.ai.workbench.commands;

public class ResumeCommand extends Command {

	public static final String CMD_RESUME = "/resume";

	public ResumeCommand() {
		this("");
	}

	public ResumeCommand(String sessionId) {
		super(sessionId);
	}

	@Override
	public String prefix() {
		return ResumeCommand.CMD_RESUME;
	}

	@Override
	public boolean matches(String line) {
		return line != null && line.strip().matches("(?i)" + prefix() + "\\s+\\S+");
	}

	@Override
	public Command parse(String line) {
		return new ResumeCommand(line.strip().split("\\s+", 2)[1].strip());
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.REMOVE;
	}
}
