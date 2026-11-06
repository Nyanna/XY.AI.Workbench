package xy.ai.workbench.eclipse;

import xy.ai.workbench.api.IExecutor;
import xy.ai.workbench.api.ext.IGitSupport;
import xy.ai.workbench.commands.AnswerCommand;
import xy.ai.workbench.commands.CallCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.view.diff.DiffPanel;
import xy.ai.workbench.view.diff.OpSnapshotter;
import xy.ai.workbench.view.diff.SnapshotResult;

public class GitSupport implements IGitSupport{

	public void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req) {
		Prompt prompt = req.getPrompt();
		if (prompt == null || prompt.arg == null)
			return;

		Command cmd = prompt.arg.command;
		if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
			return;

		String opId = extractYamlId(prompt.arg.yaml);
		boolean revertToolResultDeny = cmd instanceof AnswerCommand ac && ac.action() == AnswerCommand.Action.Deny
				&& ac.blockType() == AnswerCommand.BlockType.Result;

		exec.schedule("Workbench-Snapshot", mon -> {
			OpSnapshotter snap = new OpSnapshotter(prompt.arg.project.toFile());
			SnapshotResult result = revertToolResultDeny ? snap.revertLast() : snap.snapshot(opId);
			if (result != null && DiffPanel.INSTANCE != null)
				exec.asyncExec(() -> DiffPanel.INSTANCE.onSnapshot(result, snap.getRepository()));
		});
	}

	/** Extracts the top-level "id:" field from a captured tool-call/result YAML block. */
	private String extractYamlId(String yaml) {
		if (yaml == null)
			return null;
		var m = java.util.regex.Pattern.compile("(?m)^id:\\s*[\"']?([^\"'\\r\\n]+?)[\"']?\\s*$").matcher(yaml);
		return m.find() ? m.group(1).strip() : null;
	}
}
