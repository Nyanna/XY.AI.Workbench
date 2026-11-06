package xy.ai.conscious.connector.harness;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

import xy.ai.conscious.api.IActiveEditor;
import xy.ai.conscious.api.IActiveEditor.Selection;
import xy.ai.conscious.api.IExecutor;
import xy.ai.conscious.commands.AnswerCommand;
import xy.ai.conscious.commands.CallEditCommand;
import xy.ai.conscious.commands.Command;
import xy.ai.conscious.commands.CommandRegistry;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.IEditorInterface;
import xy.ai.conscious.core.InputMode;

/**
 * Input handling extracted from {@link PromptHandler}: aggregates editor/config
 * inputs, tracks input-size stats and runs command preprocessing (command
 * substitution / detection) needed to build a {@link Prompt}.
 */
public class PromptInputHandler {
  private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```$", Pattern.MULTILINE | Pattern.DOTALL);
  private final ConfigManager cfg;
  private final IActiveEditor editorListener;

  public PromptInputHandler(ConfigManager cfg, IActiveEditor editorListener) {
    this.cfg = cfg;
    this.editorListener = editorListener;
  }

  public Prompt buildPrompt(IExecutor exec, boolean batch) {
    PromptArguments arg = new PromptArguments();
    exec.syncExec(() -> {
      arg.setEditor(editorListener.getLastTextEditor());
      arg.absoluteFilePath = editorListener.resolveAbsoluteFilePath();
      arg.project = editorListener.resolveProjectPath();
      if (cfg.isInputEnabled(InputMode.Selection))
        detectSelection(editorListener.getSelection(), arg);
      else if (cfg.isInputEnabled(InputMode.Converter))
        detectFullFile(editorListener.getFileContent(), arg);
    });
    for (String input : arg.inputs) if (IEditorInterface.containsPromptTag(input))
      throw new IllegalStateException("Prompt accidentally triggered on a line containing a prompt tag");
    FrozenConfig frozen = FrozenConfig.from(cfg);
    arg.processorEnabled = cfg.isInputEnabled(InputMode.Converter);
    if (arg.inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()) && arg.command == null)
      throw new IllegalArgumentException("Input and System Prompt and Command Empty");
    if (arg.getEditor() == null && !batch)
      throw new IllegalArgumentException("Result editor unset");
    return new Prompt(arg.inputs, batch, frozen, arg);
  }

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
      arg.inputs.add(IActiveEditor.removeCommentLines(sel));
      // An /answer command starting at the beginning of the block spans the
      // whole selection, allowing a multi-line reason/hint for
      // allow and deny alike.
      if (blockCmd instanceof AnswerCommand ac) {
        arg.command = blockCmd;
        arg.yaml = captureYamlBlock(lines, 0);
        ac.setYaml(arg.yaml);
      } else // multi-line command takes precedence over a trailing command
      if (!detectedCmd(lines[0], lines, 0, arg))
        detectedCmd(lines[lines.length - 1], lines, lines.length - 1, arg);
      return;
    }
    arg.inputs.add(lines[0]);
    // Cursor-only (no real selection): "lines" is just the current line, so a
    // preceding ```yaml block cannot be found within it. Fall back to the full
    // document to locate the block before the cursor line.
    if (detectedCmd(lines[0], lines, 0, arg) && arg.yaml == null) {
      Selection full = editorListener.getFileContent();
      if (full != null && full.selection() != null && full.cursorOffset() != null) {
        arg.yaml = captureYamlBlock(full.selection(), full.cursorOffset());
        if (arg.command instanceof AnswerCommand ac)
          ac.setYaml(arg.yaml);
      }
    }
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
    if (cursorLine != null && cursorLine >= 0 && cursorLine < lines.length && detectedCmd(lines[cursorLine], lines, cursorLine, arg))
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
    for (int i = 0; i < lineIndex && i < lines.length; i++) prefix.append(lines[i]).append("\n");
    Matcher m = YAML_BLOCK.matcher(prefix);
    String last = null;
    while (m.find()) last = m.group(1);
    return last;
  }
}
