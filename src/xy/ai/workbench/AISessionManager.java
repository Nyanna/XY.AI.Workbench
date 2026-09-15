package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.function.Consumer;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.texteditor.ITextEditor;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.editor.md.AbstractRule;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class AISessionManager {
	public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";

	private ActiveEditorListener editorListener = new ActiveEditorListener(this);
	private IncludeAdapter includeAdapter;

	private final ConfigManager cfg;
	private final AdaptingConnector connector;
	private final MCPClient mcpClient;
	private final SessionProcessor sessionProcessor;
	public final EditorInterface editIfc;
	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector, MCPClient mcpClient,
			SessionProcessor sessionProcessor) {
		this.cfg = cfg;
		this.connector = connector;
		this.mcpClient = mcpClient;
		this.sessionProcessor = sessionProcessor;
		editIfc = new EditorInterface(editorListener, connector, cfg);
		cfg.addInputModeObs(i -> updateInputStat(i));
		cfg.addEnabledToolsObs(t -> updateInputStat(InputMode.Tools), false);
		cfg.addModelObs(this::discoverTools, false);
		includeAdapter = new IncludeAdapter(editorListener);
		sessionProcessor.setAdapter(includeAdapter);
	}

	private void discoverTools(Model model) {
		if (model == null)
			return;
		List<String> discovered = new ArrayList<>();
		for (JsonNode tool : mcpClient.listTools())
			discovered.add(tool.path("name").asText());
		model.cap.tools(sortByToolOrder(discovered));
	}

	private String[] sortByToolOrder(Collection<String> names) {
		LinkedHashSet<String> remaining = new LinkedHashSet<>(names);
		List<String> ordered = new ArrayList<>();
		for (String known : Tools.ALL)
			if (remaining.remove(known))
				ordered.add(known);
		ordered.addAll(remaining);
		return ordered.toArray(new String[0]);
	}

	public void clearObserver() {
		answerObs.clear();
		inputStatObs.clear();
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		inputStatObs.add(obs);
		if (initialize)
			obs.accept(inputStats);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public void updateInputStat(InputMode mode) {
		if (mode == InputMode.Tools) {
			String[] tools = cfg.getTools();
			inputStats[mode.ordinal()] = tools != null ? tools.length : 0;
		} else {
			String input = getInput(mode);
			inputStats[mode.ordinal()] = input != null ? input.length() : -1;
		}
		inputStatObs.forEach(c -> c.accept(inputStats));
	}

	public void initializeInputs() {
		for (var mode : InputMode.values())
			updateInputStat(mode);
		includeAdapter.initializeInputs();
	}


	private String getInput(InputMode mode) {
		ITextEditor textEditor = editorListener.getLastTextEditor();

		switch (mode) {
		case SystemPrompt:
			StringBuffer systemPrompt = new StringBuffer();
			Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
					.forEach(e -> systemPrompt.append("* ").append(e).append(".\n"));
			String freeText = cfg.getFreeText();
			if (freeText != null && !freeText.isBlank())
				systemPrompt.append(".\n").append(cfg.getFreeText()).append(".\n");
			String prompttext = systemPrompt.toString();
			return prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;
		case Selection:
			if (textEditor != null) {
				ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
				if (selectionProvider != null) {
					ISelection selection = selectionProvider.getSelection();
					ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
					if (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)
						return removeCommentLines(tsel.getText());

					if (tsel != null) {
						int line = tsel.getEndLine();
						IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
						try {
							IRegion lineInfo = doc.getLineInformation(line);
							return doc.get(lineInfo.getOffset(), lineInfo.getLength());
						} catch (BadLocationException e1) {
							LOG.error("Exception", e1);
						}
					}
				}
			}
			break;
		case Converter:
			if (textEditor != null) {
				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				if (doc != null)
					return doc.get();
			}
			break;
		case Tools:
			throw new UnsupportedOperationException();
		}
		return null;
	}

	public String removeCommentLines(String input) {
		if (input == null || input.isEmpty())
			return input;

		StringBuffer result = new StringBuffer();
		String[] lines = input.split("\\R");

		for (String line : lines)
			if (!line.trim().startsWith(AbstractRule.LINE_COMMENT))
				result.append(line).append(System.lineSeparator());

		return result.toString();
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
				var req = prepareInner(display, false, sub);
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

	public void queueAsync(Display display, AIBatchManager batch) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, batch, mon);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Enqueue batch prompt", 3);
		sub.subTask("Prepare inputs");
		var req = prepareInner(display, true, sub.split(1));
		sub.subTask("Insert Tag");
		editIfc.insertTag(display, req, sub.split(1));
		sub.subTask("Enqueue prompt");
		batch.enqueue(req, sub.split(1));
	}

	public void queueAndSubmit(Display display, AIBatchManager batch) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, batch, mon);
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

	private IModelRequest prepareInner(Display display, boolean batchFix, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Preparing Call", 1);
		sub.subTask("Preparing Call");

		List<String> inputs = new ArrayList<String>();
		display.syncExec(() -> {
			String input = null;
			if (cfg.isInputEnabled(InputMode.Selection))
				input = getInput(InputMode.Selection);
			else if (cfg.isInputEnabled(InputMode.Converter))
				input = getInput(InputMode.Converter);
			if (input != null)
				inputs.add(input);
		});

		StringBuffer systemPrompt = new StringBuffer();
		if (cfg.isInputEnabled(InputMode.SystemPrompt)) {
			String input = getInput(InputMode.SystemPrompt);
			if (input == null)
				throw new IllegalArgumentException("Systemprompt is selected but null");
			systemPrompt.append(input);
		}

		if ((inputs == null || inputs.isEmpty()) && systemPrompt.length() == 0)
			throw new IllegalArgumentException("Input and System Prompt Empty");

		if (editorListener.getLastTextEditor() == null && !batchFix)
			throw new IllegalArgumentException("Result editor unset");

		List<String> tools = cfg.isInputEnabled(InputMode.Tools) ? List.of(cfg.getTools()) : List.of();

		sub.subTask("Input prepared");

		// InputMode.Converter also toggles the shared SessionProcessor itself: while off, all
		// connectors turn the whole input into a single plain message via their message callback.
		sessionProcessor.setEnabled(cfg.isInputEnabled(InputMode.Converter));

		IModelRequest req = connector.createRequest(//
				inputs, //
				systemPrompt.toString(), //
				tools, //
				batchFix, //
				sub//
		);
		sub.worked(1);
		return req;
	}

	private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon, job);
		AIAnswer res = connector.convertResponse(resp, mon);
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}
}
