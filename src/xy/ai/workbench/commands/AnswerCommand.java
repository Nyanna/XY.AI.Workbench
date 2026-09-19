package xy.ai.workbench.commands;

public class AnswerCommand extends Command {
	public static final String CMD_ANSWER = "/answer";

	public AnswerCommand() {
		this("", "", "");
	}

	public AnswerCommand(String id, String action, String reason) {
		super(id, action, reason);
	}

	@Override
	public String prefix() {
		return AnswerCommand.CMD_ANSWER;
	}

	@Override
	public boolean matches(String line) {
		// "s" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.
		return line != null && line.strip().matches("(?is)" + prefix() + "\\s+\\S+\\s+(allow|deny)(\\s+.*)?");
	}

	@Override
	public Command parse(String line) {
		String[] parts = line.strip().split("\\s+", 4);
		String id = parts[1];
		String action = parts[2].toLowerCase();
		String reason = parts.length > 3 ? parts[3].strip() : "";
		return new AnswerCommand(id, action, reason);
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.TRANSFORM;
	}

	public enum Action {
		Allow, Deny
	}

	public Action action() {
		return "allow".equalsIgnoreCase(parameter(1)) ? Action.Allow : Action.Deny;
	}
}
