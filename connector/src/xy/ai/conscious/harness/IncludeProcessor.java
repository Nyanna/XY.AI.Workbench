package xy.ai.conscious.harness;

import java.nio.file.Path;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import xy.ai.conscious.commands.include.IncludeCommandRegistry;

public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {
  public static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\)\\s*)+$");
  public static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
  private final IncludeCommandRegistry<M> commands;

  public IncludeProcessor(IIncludeAdapter adapter) {
    this.commands = new IncludeCommandRegistry<>(adapter);
  }

  public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    commands.execute(kind, arg, chain, call, out, recursiv);
  }

  @Override
  public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    String[] lines = text == null ? new String[0] : text.split("\n", -1);
    StringBuilder buffer = new StringBuilder();
    for (String line : lines) {
      if (INCLUDE_LINE.matcher(line).matches()) {
        flush(buffer, call, out);
        Matcher m = INCLUDE_TAG.matcher(line);
        while (m.find()) include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);
        continue;
      }
      if (buffer.length() > 0)
        buffer.append("\n");
      buffer.append(line);
    }
    flush(buffer, call, out);
  }

  private void flush(StringBuilder buffer, SessionCallbacks<M> call, List<M> out) {
    String text = buffer.toString().strip();
    buffer.setLength(0);
    if (!text.isEmpty())
      out.add(call.message(Role.User, text));
  }
}
