package xy.ai.workbench.commands;

public class PromptCommand extends Command {

	public PromptCommand() {
		this("");
	}

	public PromptCommand(String text) {
		super(text);
	}

	@Override
	public String prefix() {
		return "";
	}

	@Override
	public boolean matches(String line) {
		return true;
	}

	@Override
	public Command parse(String line) {
		return new PromptCommand(line.strip());
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.NONE;
	}

	/** First 100 chars of the prompt text, single-lined; used as session/title label. */
	public String title() {
		String text = parameter(0);
		return text.substring(0, Math.min(100, text.length())).replace('\n', ' ');
	}
}
