package xy.ai.workbench.connector.harness;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

import xy.ai.workbench.api.IProgress;

import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.IncludeAdapter;
import xy.ai.workbench.api.IExecutor;
import xy.ai.workbench.api.IMonitoredRunnable;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.commands.AnswerCommand;
import xy.ai.workbench.commands.CallCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.ToolCommand;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.core.ConfigManager;
import xy.ai.workbench.core.LOG;
import xy.ai.workbench.core.Model;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;
import xy.ai.workbench.view.diff.DiffPanel;
import xy.ai.workbench.view.diff.OpSnapshotter;
import xy.ai.workbench.view.diff.SnapshotResult;

/**
 * Prompt-specific orchestration extracted from {@code AISessionManager}: drives
 * the prepare/insertTag/execute/replaceTag job pipeline, delegating input
 * aggregation/preprocessing to {@link PromptInputHandler}.
 */
public class PromptHandler {

	private final AdaptingConnector connector;
	private final EditorInterface editIfc;
	private final IncludeAdapter includeAdapter;
	private final AIBatchManager batch;
	private final PromptInputHandler input;

	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();

	public PromptHandler(ConfigManager cfg, ActiveEditorListener editorListener, AdaptingConnector connector,
			AIBatchManager batch, EditorInterface editIfc, IncludeAdapter includeAdapter) {
		this.connector = connector;
		this.editIfc = editIfc;
		this.includeAdapter = includeAdapter;
		this.batch = batch;
		this.input = new PromptInputHandler(cfg, editorListener);
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		input.addInputStatObs(obs, initialize);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public void initializeInputs() {
		input.initializeInputs();
		includeAdapter.initializeBindings();
	}

	public void execute(IExecutor exec) {
		exec.schedule("Starting Prompt", new PromptRun(exec));
	}

	private class PromptRun implements IMonitoredRunnable {
		private final IExecutor exec;

		private PromptRun(IExecutor exec) {
			this.exec = exec;
		}

		@Override
		public void run(IProgress mon) {
			IProgress sub = mon.setTask("Executing prompt", 4);
			String reqId = null;
			try {
				sub.setTaskName("Prepare inputs");
				var req = prepareRequest(exec, false, sub);
				sub.worked();
				sub.setTaskName("Insert Tag");
				editIfc.insertTag(exec, req, sub);
				reqId = req.getID();
				mon.worked();

				sub.setTaskName("Execute prompt");
				AIAnswer ans = executeInner(exec, req, sub);
				mon.worked();
				sub.setTaskName("Process Answer");
				editIfc.replaceTag(ans, sub);
				mon.worked();
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				// uncatched error case
				if (reqId != null) {
					AIAnswer error = new AIAnswer(reqId);
					error.answer = e.getMessage();
					editIfc.replaceTag(error, sub);
				}
			} finally {
				mon.done();
			}
		}
	}

	public void queueAsync(IExecutor exec) {
		exec.schedule("Enqueue Prompt", mon -> {
			queueSync(exec, mon);
		});
	}

	private void queueSync(IExecutor exec, IProgress mon) {
		IProgress sub = mon.setTask("Enqueue batch prompt", 3);
		sub.setTaskName("Prepare inputs");
		var req = prepareRequest(exec, true, sub.subProgress());
		sub.setTaskName("Insert Tag");
		editIfc.insertTag(exec, req, sub.subProgress());
		sub.setTaskName("Enqueue prompt");
		batch.enqueue(req, sub.subProgress());
	}

	public void queueAndSubmit(IExecutor exec) {
		exec.schedule("Enqueue Prompt", (mon) -> {
			queueSync(exec, mon);
			batch.submitBatches(mon);
		});
	}

	private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgress mon) {
		IProgress sub = mon.setTask("Preparing Call", 1);
		sub.setTaskName("Preparing Call");
		Prompt prompt = input.buildPrompt(exec, batchFix);
		preprocessPrompt(prompt);
		IModelRequest req = connector.createRequest(prompt, sub);
		req.setPrompt(prompt);
		sub.worked();
		return req;
	}

	private void preprocessPrompt(Prompt prompt) {
		for (String key : prompt.config.keys.split(","))
			if (prompt.config.model.cap.acceptsKey(key))
				prompt.config.keys = key;
		if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
			prompt.config.model = Model.MCP_TOOLS;
	}

	private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon) {
		exec.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon);
		AIAnswer res = connector.convertResponse(resp, mon);
		res.prompt = req.getPrompt();
		exec.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		triggerSnapshotIfNeeded(exec,req);
		return res;
	}

	private void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req) {
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
