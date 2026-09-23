package xy.ai.workbench.connector.harness;

import java.lang.ref.WeakReference;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.ActiveEditorListener.Selection;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.LOG;
import xy.ai.workbench.commands.AnswerCommand;
import xy.ai.workbench.commands.CallEditCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.CommandRegistry;
import xy.ai.workbench.editor.md.AbstractRule;

/**
 * Input handling extracted from {@link PromptHandler}: aggregates editor/config
 * inputs, tracks input-size stats and runs command preprocessing (command
 * substitution / detection) needed to build a {@link Prompt}.
 */
public class PromptInputHandler {
	private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```$",
			Pattern.MULTILINE | Pattern.DOTALL);

	private final ConfigManager cfg;
	private final ActiveEditorListener editorListener;

	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();

	public PromptInputHandler(ConfigManager cfg, ActiveEditorListener editorListener) {
		this.cfg = cfg;
		this.editorListener = editorListener;

		editorListener.setInputObserver(i -> updateInputStat(i));
		cfg.addInputModeObs(i -> updateInputStat(i));
		cfg.addEnabledToolsObs(t -> updateInputStat(InputMode.Tools), false);
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		inputStatObs.add(obs);
		if (initialize)
			obs.accept(inputStats);
	}

	private void updateInputStat(InputMode mode) {
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
	}

	private String getInput(InputMode mode) {
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
			return removeCommentLines(editorListener.getSelection());
		case Converter:
			return removeCommentLines(editorListener.getFileContent());
		case Tools:
			throw new UnsupportedOperationException();
		}
		return null;
	}

	private String removeCommentLines(Selection input) {
		if (input.selection() == null)
			return null;

		StringBuffer result = new StringBuffer();

		for (String line : input.selection())
			if (!line.trim().startsWith(AbstractRule.LINE_COMMENT))
				result.append(line).append(System.lineSeparator());

		return result.toString();
	}

	/**
	 * Freezes the current editor/config state into a {@link Prompt}; runs command
	 * detection (see class doc).
	 */
	public Prompt buildPrompt(Display display, boolean batch) {
		PromptArguments arg = new PromptArguments();
		display.syncExec(() -> {
			arg.setEditor(editorListener.getLastTextEditor());
			arg.absoluteFilePath = editorListener.resolveAbsoluteFilePath();
			arg.project = editorListener.resolveProjectPath();

			if (cfg.isInputEnabled(InputMode.Selection))
				detectSelection(editorListener.getSelection(), arg);
			else if (cfg.isInputEnabled(InputMode.Converter))
				detectFullFile(editorListener.getFileContent(), arg);
		});
		FrozenConfig frozen = FrozenConfig.from(cfg);
		arg.processorEnabled = cfg.isInputEnabled(InputMode.Converter);

		if (arg.inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank())
				&& arg.command == null)
			throw new IllegalArgumentException("Input and System Prompt and Command Empty");

		if (arg.getEditor() == null && !batch)
			throw new IllegalArgumentException("Result editor unset");

		return new Prompt(arg.inputs, batch, frozen, arg);
	}

	public static class PromptArguments {
		public List<String> inputs = new ArrayList<>();
		public String absoluteFilePath;
		public Path project;
		public Command command;
		public String yaml;
		// Editor active when the prompt was built; used as a hint for tag replacement.
		private WeakReference<ITextEditor> editor;
		public boolean processorEnabled;

		public void setEditor(ITextEditor editor) {
			this.editor = editor != null ? new WeakReference<>(editor) : null;
		}

		public ITextEditor getEditor() {
			return editor != null ? editor.get() : null;
		}
	}

	/**
	 * Selection mode: a real (multi-char) selection is a BlockSelection, an
	 * empty/caret selection falls back to the current cursor line (LineInput).
	 * 
	 * @param arg
	 */
	private void detectSelection(Selection selection, PromptArguments arg) {
		ITextEditor editor = arg.getEditor();
		if (editor == null)
			return;

		IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
		if (doc == null)
			return;

		ITextSelection tsel = getSeletion(editor);
		if (tsel != null) {
			if (!tsel.isEmpty() && tsel.getLength() > 1) {
				// selection solely of a YAML block
				Command blockCmd = CommandRegistry.detect(tsel.getText());
				if (blockCmd instanceof CallEditCommand) {
					arg.command = blockCmd;
					return;
				}

				arg.inputs.add(removeCommentLines(new Selection(tsel.getText().split("\n"), null)));
				// An /answer command starting at the beginning of the block spans the
				// whole selection, allowing a multi-line reason/hint for
				// allow and deny alike.
				if (blockCmd instanceof AnswerCommand) {
					arg.command = blockCmd;
					arg.yaml = captureYamlBlock(doc, tsel.getStartLine());
				}
				// multi-line command takes precedence over a trailing command
				else if (!detectedCmd(getLine(doc, tsel.getStartLine()), doc, tsel.getStartLine(), arg))
					detectedCmd(getLine(doc, tsel.getEndLine()), doc, tsel.getEndLine(), arg);
				return;
			}

			try {
				IRegion lineInfo = doc.getLineInformation(tsel.getEndLine());
				arg.inputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));
			} catch (BadLocationException e) {
				LOG.error("Can't get selection", e);
			}
			detectedCmd(getLine(doc, tsel.getEndLine()), doc, tsel.getEndLine(), arg);
		}
	}

	/*
	 * Processor (full file) mode: the caret line is checked first, then the last
	 * line of the file, matching a command appended after the generated content.
	 */
	private void detectFullFile(Selection content, PromptArguments arg) {
		ITextEditor editor = arg.getEditor();
		if (editor == null)
			return;

		IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
		if (doc == null)
			return;
		arg.inputs.add(doc.get());

		ITextSelection tsel = getSeletion(editor);
		if (tsel != null && detectedCmd(getLine(doc, tsel.getEndLine()), doc, tsel.getEndLine(), arg))
			return;

		int lastLine = doc.getNumberOfLines() - 1;
		detectedCmd(getLine(doc, lastLine), doc, lastLine, arg);
		return;
	}

	private ITextSelection getSeletion(ITextEditor editor) {
		ISelectionProvider prv = editor.getSelectionProvider();
		ISelection sel = prv != null ? prv.getSelection() : null;
		return sel instanceof ITextSelection ? (ITextSelection) sel : null;
	}

	private boolean detectedCmd(String line, IDocument doc, int lineIndex, PromptArguments arg) {
		Command cmd = CommandRegistry.detect(line);
		if (cmd == null)
			return false;
		arg.command = cmd;
		arg.yaml = captureYamlBlock(doc, lineIndex);
		return true;
	}

	private String getLine(IDocument doc, int lineIndex) {
		try {
			IRegion info = doc.getLineInformation(lineIndex);
			return doc.get(info.getOffset(), info.getLength());
		} catch (BadLocationException e) {
			LOG.error("Exception", e);
			return "";
		}
	}

	/*
	 * Walks backwards from the command line and keeps the closest preceding ```yaml
	 * block.
	 */
	private String captureYamlBlock(IDocument doc, int lineIndex) {
		try {
			String prefix = doc.get(0, doc.getLineOffset(lineIndex));
			Matcher m = YAML_BLOCK.matcher(prefix);
			String last = null;
			while (m.find())
				last = m.group(1);
			return last;
		} catch (BadLocationException e) {
			LOG.error("Can't capture YAML block", e);
			return null;
		}
	}
}
