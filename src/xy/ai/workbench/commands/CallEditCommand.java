package xy.ai.workbench.commands;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

/** Detected when the whole input consists solely of a single fenced ```yaml ...``` block (a pending edit/approval). */
public class CallEditCommand extends Command {

	private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)\\R?```$", Pattern.DOTALL);

	public CallEditCommand() {
		super();
	}

	private CallEditCommand(String yaml) {
		super(yaml);
	}

	@Override
	public String prefix() {
		return "```yaml";
	}

	@Override
	public boolean matches(String line) {
		return line != null && YAML_BLOCK.matcher(line.strip()).matches();
	}

	@Override
	public Command parse(String line) {
		Matcher m = YAML_BLOCK.matcher(line.strip());
		m.matches();
		return new CallEditCommand(m.group(1).strip());
	}

	@Override
	public ProcessorAction processorAction() {
		return ProcessorAction.REMOVE;
	}

	public String yaml() {
		return parameter(0);
	}
}
