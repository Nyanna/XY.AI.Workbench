package xy.ai.workbench.connector.harness;

import java.io.File;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.core.resources.IProject;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IURIEditorInput;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.Activator;
import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.IncludeAdapter;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.LOG;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.commands.AnswerCommand;
import xy.ai.workbench.commands.CallEditCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.CommandRegistry;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.editor.md.AbstractRule;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

/**
 * Prompt-specific orchestration extracted from {@code AISessionManager}: builds
 * a frozen {@link Prompt} from the active editor/selection, runs command
 * detection and drives the prepare/insertTag/execute/replaceTag job pipeline.
 */
public final class PromptHandler {

	// Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
	// before a command line.
	private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```$",
			Pattern.MULTILINE | Pattern.DOTALL);

	private final ConfigManager cfg;
	private final AdaptingConnector connector;
	private final ActiveEditorListener editorListener;
	private final EditorInterface editIfc;
	private final IncludeAdapter includeAdapter;
	private final AIBatchManager batch;

	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();


	public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEditorListener editorListener,
			EditorInterface editIfc, IncludeAdapter includeAdapter) {
		this.cfg = cfg;
		this.connector = connector;
		this.editorListener = editorListener;
		this.editIfc = editIfc;
		this.includeAdapter = includeAdapter;
		this.batch = Activator.getDefault().batch;
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

	private String removeCommentLines(String input) {
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
		Prompt prompt = buildPrompt(display, batchFix);
		IModelRequest req = connector.createRequest(prompt, sub);
		sub.worked(1);
		return req;
	}

	/**
	 * Freezes the current editor/config state into a {@link Prompt}; runs command
	 * detection (see class doc).
	 */
	private Prompt buildPrompt(Display display, boolean batch) {
		List<String> inputs = new ArrayList<>();
		String[] absoluteFilePath = new String[1];
		Path[] projectPath = new Path[1];
		Command[] detected = new Command[1];
		String[] yamlBlock = new String[1];

		display.syncExec(() -> {
			ITextEditor textEditor = editorListener.getLastTextEditor();
			absoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);
			projectPath[0] = resolveProjectPath(textEditor);

			if (cfg.isInputEnabled(InputMode.Selection))
				detectSelection(textEditor, inputs, detected, yamlBlock);
			else if (cfg.isInputEnabled(InputMode.Converter))
				detectFullFile(textEditor, inputs, detected, yamlBlock);
		});

		FrozenConfig frozen = FrozenConfig.from(cfg);

		if (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))
			throw new IllegalArgumentException("Input and System Prompt Empty");

		if (editorListener.getLastTextEditor() == null && !batch)
			throw new IllegalArgumentException("Result editor unset");

		return new Prompt(inputs, batch, frozen, cfg.isInputEnabled(InputMode.Converter), absoluteFilePath[0],
				projectPath[0], detected[0], yamlBlock[0]);
	}

	// Selection mode: a real (multi-char) selection is a BlockSelection, an
	// empty/caret selection
	// falls back to the current cursor line (LineInput).
	private void detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,
			String[] yamlBlock) {
		if (textEditor == null)
			return;

		ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
		ISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;
		ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
		IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
		if (doc == null)
			return;

		if (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {
			// A selection consisting solely of a ```yaml block is itself a command
			// (CallEditCommand).
			Command block = CommandRegistry.detect(tsel.getText());
			if (block instanceof CallEditCommand) {
				detected[0] = block;
				return;
			}

			inputs.add(removeCommentLines(tsel.getText()));
			// An /answer command starting at the very beginning of the block spans the
			// whole
			// selection, allowing a multi-line (better formatted) reason/hint for allow and
			// deny alike.
			if (block instanceof AnswerCommand) {
				detected[0] = block;
				yamlBlock[0] = captureYamlBlock(doc, tsel.getStartLine());
			}
			// Block start (multi-line command) takes precedence over a trailing command
			// line.
			else if (!applyDetected(CommandRegistry.detect(lineText(doc, tsel.getStartLine())), doc,
					tsel.getStartLine(), detected, yamlBlock))
				applyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),
						detected, yamlBlock);
			return;
		}

		if (tsel != null) {
			try {
				IRegion lineInfo = doc.getLineInformation(tsel.getEndLine());
				inputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));
			} catch (BadLocationException e) {
				LOG.error("Exception", e);
			}
			applyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,
					yamlBlock);
		}
	}

	// Processor (full file) mode: the caret line is checked first, then the last
	// line of the file,
	// matching a command appended after the generated content.
	private void detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,
			String[] yamlBlock) {
		if (textEditor == null)
			return;

		IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
		if (doc == null)
			return;
		inputs.add(doc.get());

		ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
		ISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;
		ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
		if (tsel != null && applyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc,
				tsel.getEndLine(), detected, yamlBlock))
			return;

		int lastLine = doc.getNumberOfLines() - 1;
		applyDetected(CommandRegistry.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);
		return;
	}

	private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Command[] detected, String[] yamlBlock) {
		if (cmd == null)
			return false;
		detected[0] = cmd;
		yamlBlock[0] = captureYamlBlock(doc, lineIndex);
		return true;
	}

	private String lineText(IDocument doc, int lineIndex) {
		try {
			IRegion info = doc.getLineInformation(lineIndex);
			return doc.get(info.getOffset(), info.getLength());
		} catch (BadLocationException e) {
			LOG.error("Exception", e);
			return "";
		}
	}

	// Walks backwards from the command line and keeps the closest preceding ```yaml
	// block.
	private String captureYamlBlock(IDocument doc, int lineIndex) {
		try {
			String prefix = doc.get(0, doc.getLineOffset(lineIndex));
			Matcher m = YAML_BLOCK.matcher(prefix);
			String last = null;
			while (m.find())
				last = m.group(1);
			return last;
		} catch (BadLocationException e) {
			LOG.error("Exception", e);
			return null;
		}
	}

	private String resolveAbsoluteFilePath(ITextEditor textEditor) {
		if (textEditor == null)
			return null;
		IEditorInput input = textEditor.getEditorInput();
		if (input instanceof IFileEditorInput)
			return ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();
		if (input instanceof IURIEditorInput)
			return new File(((IURIEditorInput) input).getURI()).getAbsolutePath();
		return null;
	}

	private Path resolveProjectPath(ITextEditor textEditor) {
		if (textEditor == null)
			return null;
		IEditorInput input = textEditor.getEditorInput();
		if (!(input instanceof IFileEditorInput))
			return null;
		IProject project = ((IFileEditorInput) input).getFile().getProject();
		return Paths.get(project.getLocation().toOSString());
	}

	private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon, job);
		AIAnswer res = connector.convertResponse(resp, mon);
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}
}
