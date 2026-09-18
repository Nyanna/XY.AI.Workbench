package xy.ai.workbench.connector.harness;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.Activator;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.IncludeAdapter;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.LOG;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

/**
 * Prompt-specific orchestration extracted from {@code AISessionManager}: drives
 * the prepare/insertTag/execute/replaceTag job pipeline, delegating input
 * aggregation/preprocessing to {@link PromptInputHandler}.
 */
public final class PromptHandler {

	private final AdaptingConnector connector;
	private final EditorInterface editIfc;
	private final AIBatchManager batch;
	private final PromptInputHandler input;

	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();

	public PromptHandler(ConfigManager cfg, AdaptingConnector connector, EditorInterface editIfc,
			IncludeAdapter includeAdapter) {
		this.connector = connector;
		this.editIfc = editIfc;
		this.batch = Activator.getDefault().batch;
		this.input = new PromptInputHandler(cfg, includeAdapter);
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		input.addInputStatObs(obs, initialize);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public void updateInputStat(InputMode mode) {
		input.updateInputStat(mode);
	}

	public void initializeInputs() {
		input.initializeInputs();
	}

	public void execute(Display display) {
		new PromptJob("Starting Prompt", display).schedule();
	}

	private class PromptJob extends Job {
		private final Display display;

		private PromptJob(String name, Display display) {
			super(name);
			this.display = display;
		}

		@Override
		protected IStatus run(IProgressMonitor mon) {
			SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 4);
			String reqId = null;
			try {
				sub.subTask("Prepare inputs");
				var req = prepareRequest(display, false, sub);
				sub.worked(1);
				sub.subTask("Insert Tag");
				editIfc.insertTag(display, req, sub);
				reqId = req.getID();
				mon.worked(1);

				sub.subTask("Execute prompt");
				var ans = executeInner(display, req, sub, this);
				mon.worked(1);
				sub.subTask("Process Answer");
				editIfc.replaceTag(display, ans, sub);
				mon.worked(1);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				// uncatched error case
				if (reqId != null) {
					AIAnswer error = new AIAnswer(reqId);
					error.answer = e.getMessage();
					editIfc.replaceTag(display, error, sub);
				}
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}
	}

	public void queueAsync(Display display) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, mon);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private void queueSync(Display display, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Enqueue batch prompt", 3);
		sub.subTask("Prepare inputs");
		var req = prepareRequest(display, true, sub.split(1));
		sub.subTask("Insert Tag");
		editIfc.insertTag(display, req, sub.split(1));
		sub.subTask("Enqueue prompt");
		batch.enqueue(req, sub.split(1));
	}

	public void queueAndSubmit(Display display) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, mon);
				batch.submitBatches(mon);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Preparing Call", 1);
		sub.subTask("Preparing Call");
		Prompt prompt = input.buildPrompt(display, batchFix);
		IModelRequest req = connector.createRequest(prompt, sub);
		req.setPrompt(prompt);
		sub.worked(1);
		return req;
	}

	private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon, job);
		AIAnswer res = connector.convertResponse(resp, mon);
		res.prompt = req.getPrompt();
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}
}
