package xy.ai.workbench.commands;

import xy.ai.workbench.editor.IEditorInterface;

public class ControlRequestCommand extends Command {

	public ControlRequestCommand() {
		super();
	}

	@Override
	public String prefix() {
		return IEditorInterface.CONTROL_REQUEST;
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
