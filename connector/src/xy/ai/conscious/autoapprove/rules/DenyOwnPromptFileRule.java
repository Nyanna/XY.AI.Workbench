package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.List;
import java.util.Set;
import com.fasterxml.jackson.databind.JsonNode;
import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
import xy.ai.conscious.connector.harness.Prompt;

/**
 * Denies every tool call and tool result that contains the own prompt file.
 * Reading the own prompt file only confuses the agent, so the agent is told to
 * adjust the call and iterate.
 * <p>
 * The values of the configured fields are searched recursively at any depth of
 * the call arguments or the result. A value matches if it contains the absolute
 * path of the prompt file or its last {@link #LEVELS_SETTING} path levels
 * (including the file name), or if it resolves to the prompt file.
 */
public class DenyOwnPromptFileRule implements IApprovalRule {
  public static final String ID = DenyOwnPromptFileRule.class.getSimpleName();
  public static final String PATH_FIELDS_SETTING = "pathFields";
  public static final String LEVELS_SETTING = "levels";
  public static final String[] PATH_FIELDS = { "path", "paths", "directory", "file", "file_path", "filePath", "command", "pattern", "text", "content" };
  public static final int LEVELS = 3;
  private final RuleSettings defaults;
  private final Path cwd;

  public DenyOwnPromptFileRule() {
    this(Set.of(PATH_FIELDS), LEVELS, Path.of("").toAbsolutePath().normalize());
  }

  public DenyOwnPromptFileRule(Set<String> pathFields, int levels, Path cwd) {
    this.defaults = //
    new RuleSettings().addStrings(PATH_FIELDS_SETTING, //
    pathFields.stream().sorted().toList()).addNumber(LEVELS_SETTING, levels);
    this.cwd = cwd.toAbsolutePath().normalize();
  }

  @Override
  public String id() {
    return ID;
  }

  @Override
  public RuleSettings getDefaultSetting() {
    return defaults.clone();
  }

  @Override
  public Vote vote(ApprovalContext ctx) {
    ApprovalItem item = ctx.item();
    Prompt prompt = ctx.prompt();
    if (item == null || prompt == null || prompt.arg.absoluteFilePath == null || prompt.arg.absoluteFilePath.isBlank())
      return Vote.abstain();
    Path file;
    try {
      file = Path.of(prompt.arg.absoluteFilePath).toAbsolutePath().normalize();
    } catch (RuntimeException e) {
      return Vote.abstain();
    }
    if (file.getNameCount() == 0)
      return Vote.abstain();
    RuleSettings settings = ctx.settings(ID);
    Set<String> fields = settings.getStringSet(PATH_FIELDS_SETTING);
    String absolute = normalize(file.toString());
    String tail = tail(file, (int) settings.getNumber(LEVELS_SETTING));
    boolean call = item.kind() == Kind.ControlCall || item.kind() == Kind.ToolCall;
    boolean found = call ? matches(PathArguments.collect(item.arguments(), fields), file, absolute, tail) : matches(resultValues(item, fields), file, absolute, tail);
    if (!found)
      return Vote.abstain();
    return Vote.deny(reason(call, absolute));
  }

  private static String reason(boolean call, String absolute) {
    return "The " + (call ? "call" : "result") + " contains the own prompt file " + absolute + ". Reading the own prompt file leads to confusion. Adjust the call so that it does not include this file, and iterate again.";
  }

  private List<String> resultValues(ApprovalItem item, Set<String> fields) {
    JsonNode result = item.result();
    if (result.isMissingNode())
      return List.of(item.yaml());
    if (result.isTextual())
      return List.of(result.asText());
    return PathArguments.collect(result, fields);
  }

  private boolean matches(List<String> values, Path file, String absolute, String tail) {
    for (String value : values) {
      String text = normalize(value);
      if (text.contains(absolute) || text.contains(tail))
        return true;
      Path path = PathArguments.resolve(cwd, value);
      if (path != null && path.equals(file))
        return true;
    }
    return false;
  }

  /**
   * Last {@code levels} path levels including the file name, separated by '/'.
   */
  private static String tail(Path file, int levels) {
    int count = file.getNameCount();
    int from = Math.max(0, count - Math.max(1, levels));
    return normalize(file.subpath(from, count).toString());
  }

  private static String normalize(String value) {
    return value.replace('\\', '/');
  }
}
