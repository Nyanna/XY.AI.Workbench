package xy.ai.workbench.commands;

public class ToolCommand extends Command {
	public static final String CMD_TOOL = "/tool";

	public ToolCommand() {
		this("");
	}

	public ToolCommand(String toolId) {
		super(toolId);
	}

	@Override
	public String prefix() {
		return ToolCommand.CMD_TOOL;
	}

	@Override
	public boolean matches(String line) {
		return line != null && line.strip().matches("(?i)" + prefix() + "\\s+\\S+");
	}

	@Override
	public Command parse(String line) {
		return new ToolCommand(line.strip().split("\\s+", 2)[1].strip());
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.REMOVE;
	}
}
