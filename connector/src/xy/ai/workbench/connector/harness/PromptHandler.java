package xy.ai.workbench.connector.harness;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

import xy.ai.workbench.api.IExecutor.IProgress;
import xy.ai.workbench.api.ext.IGitSupport;
import xy.ai.workbench.api.IActiveEditor;
import xy.ai.workbench.api.IExecutor;
import xy.ai.workbench.api.IExecutor.IMonitoredRunnable;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.commands.CallCommand;
import xy.ai.workbench.commands.ToolCommand;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.core.ConfigManager;
import xy.ai.workbench.core.LOG;
import xy.ai.workbench.core.Model;
import xy.ai.workbench.editor.IEditorInterface;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

/**
 * Prompt-specific orchestration extracted from {@code AISessionManager}: drives
 * the prepare/insertTag/execute/replaceTag job pipeline, delegating input
 * aggregation/preprocessing to {@link PromptInputHandler}.
 */
public class PromptHandler {

	private final AdaptingConnector connector;
	private final IEditorInterface editIfc;
	private final AIBatchManager batch;
	private final PromptInputHandler input;
	private final IGitSupport gitSupport;

	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();

	public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingConnector connector,
			AIBatchManager batch, IEditorInterface editIfc, IGitSupport gitSupport) {
		this.connector = connector;
		this.editIfc = editIfc;
		this.batch = batch;
		this.input = new PromptInputHandler(cfg, editorListener);
		this.gitSupport = gitSupport;
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
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
		if (gitSupport != null)
			gitSupport.triggerSnapshotIfNeeded(exec, req);
		return res;
	}
}
