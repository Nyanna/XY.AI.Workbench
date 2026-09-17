package xy.ai.workbench.commands;

import xy.ai.workbench.EditorInterface;

public class ControlRequestCommand extends Command {

	public ControlRequestCommand() {
		super();
	}

	@Override
	public String prefix() {
		return EditorInterface.CONTROL_REQUEST;
	}

	@Override
	public Command parse(String line) {
		return new ControlRequestCommand();
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.IGNORE_BLOCK;
	}
}
