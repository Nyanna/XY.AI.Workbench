package xy.ai.workbench.connector.harness;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.ActiveEditorListener.Selection;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.InputMode;
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

		editorListener.addInputObserver(i -> updateInputStat(i));
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
			String[] tools = cfg.getEnabledTools();
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
		if (input == null || input.selection() == null)
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

	/**
	 * Selection mode: a real (multi-char) selection is a BlockSelection, an
	 * empty/caret selection falls back to the current cursor line (LineInput).
	 * 
	 * @param arg
	 */
	private void detectSelection(Selection sel, PromptArguments arg) {
		if (sel == null || sel.selection() == null || sel.selection().length == 0)
			return;

		String[] lines = sel.selection();

		if (lines.length > 1) {
			// selection solely of a YAML block
			String text = String.join(System.lineSeparator(), lines);
			Command blockCmd = CommandRegistry.detect(text);
			if (blockCmd instanceof CallEditCommand) {
				arg.command = blockCmd;
				return;
			}

			arg.inputs.add(removeCommentLines(sel));
			// An /answer command starting at the beginning of the block spans the
			// whole selection, allowing a multi-line reason/hint for
			// allow and deny alike.
			if (blockCmd instanceof AnswerCommand ac) {
				arg.command = blockCmd;
				arg.yaml = captureYamlBlock(lines, 0);
				ac.setYaml(arg.yaml);
			}
			// multi-line command takes precedence over a trailing command
			else if (!detectedCmd(lines[0], lines, 0, arg))
				detectedCmd(lines[lines.length - 1], lines, lines.length - 1, arg);
			return;
		}

		arg.inputs.add(lines[0]);
		detectedCmd(lines[0], lines, 0, arg);
	}

	/*
	 * Processor (full file) mode: the caret line is checked first, then the last
	 * line of the file, matching a command appended after the generated content.
	 */
	private void detectFullFile(Selection content, PromptArguments arg) {
		if (content == null || content.selection() == null || content.selection().length == 0)
			return;

		String[] lines = content.selection();
		arg.inputs.add(String.join("\n", lines));

		Integer cursorLine = content.cursorOffset();
		if (cursorLine != null && cursorLine >= 0 && cursorLine < lines.length
				&& detectedCmd(lines[cursorLine], lines, cursorLine, arg))
			return;

		int lastLine = lines.length - 1;
		detectedCmd(lines[lastLine], lines, lastLine, arg);
	}

	private boolean detectedCmd(String line, String[] lines, int lineIndex, PromptArguments arg) {
		Command cmd = CommandRegistry.detect(line);
		if (cmd == null)
			return false;
		arg.command = cmd;
		arg.yaml = captureYamlBlock(lines, lineIndex);
		if (cmd instanceof AnswerCommand ac)
			ac.setYaml(arg.yaml);
		return true;
	}

	/*
	 * Walks backwards from the command line and keeps the closest preceding ```yaml
	 * block.
	 */
	private String captureYamlBlock(String[] lines, int lineIndex) {
		StringBuilder prefix = new StringBuilder();
		for (int i = 0; i < lineIndex && i < lines.length; i++)
			prefix.append(lines[i]).append("\n");

		Matcher m = YAML_BLOCK.matcher(prefix);
		String last = null;
		while (m.find())
			last = m.group(1);
		return last;
	}
}
