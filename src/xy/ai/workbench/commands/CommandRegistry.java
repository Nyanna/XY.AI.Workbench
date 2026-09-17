package xy.ai.workbench.commands;

import java.util.List;

public class CommandRegistry {
	private static final List<Command> REGISTRY = List.of( //
			new AnswerCommand(), //
			new ControlRequestCommand(), //
			new ResumeCommand(), //
			new ExitCommand(), //
			new ToolCommand(), //
			new CallCommand(), //
			new CallEditCommand());

	public static Command detect(String line) {
		for (Command prototype : REGISTRY)
			if (prototype.matches(line))
				return prototype.parse(line);
		return null;
	}
}
