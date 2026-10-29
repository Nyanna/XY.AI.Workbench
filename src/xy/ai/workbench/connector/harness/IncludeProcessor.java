package xy.ai.workbench.connector.harness;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;
import xy.ai.workbench.connector.claudecode.YamlRenderer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {
  public static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\)\\s*)+$");
  public static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
  private final YamlRenderer yaml = new YamlRenderer();
  private final IIncludeAdapter adapter;

  public IncludeProcessor(IIncludeAdapter adapter) {
    this.adapter = adapter;
  }

  public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    if (kind == null) {
      if (arg.isBlank())
        throw new IllegalStateException("Include path must not be empty");
      includePath(arg, chain, call, out, recursiv);
      return;
    }
    switch(kind) {
      case "contextprompt":
        includeContextPrompt(arg, call, out);
        return;
      case "directory":
        includeDirectory(arg, chain, call, out, recursiv);
        return;
      case "files":
        includeFiles(arg, call, out);
        return;
      case "file":
        includeFile(arg, call, out);
        return;
      case "search":
        includeSearch(arg, call, out);
        return;
      default:
        throw new IllegalStateException("Unknown include kind: " + kind);
    }
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

  private void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    Path path = Paths.get(pathText).toAbsolutePath().normalize();
    if (chain.contains(path))
      throw new IncludeCycleException(path.toString());
    String content;
    try {
      content = Files.readString(path);
    } catch (IOException e) {
      throw new IllegalStateException("Could not read include target: " + path, e);
    }
    List<Path> nested = new ArrayList<>(chain);
    nested.add(path);
    recursiv.processText(content, nested, call, out, recursiv);
  }

  private void includeDirectory(String dir, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    if (dir.isBlank()) {
      for (IIncludeAdapter.Entry entry : requireAdapter().selectedDirectories()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
      return;
    }
    Path directory = Paths.get(dir).toAbsolutePath().normalize();
    List<Path> files;
    try (Stream<Path> stream = Files.list(directory)) {
      files = stream.filter(Files::isRegularFile).sorted().collect(Collectors.toList());
    } catch (IOException e) {
      throw new IllegalStateException("Could not read include directory: " + directory, e);
    }
    for (Path file : files) includePath(file.toString(), chain, call, out, recursiv);
  }

  private void includeContextPrompt(String dir, SessionCallbacks<M> call, List<M> out) {
    String text = requireAdapter().contextPrompt(dir);
    if (text != null && !text.isBlank())
      out.add(call.message(Role.User, text.strip()));
  }

  private void includeFiles(String arg, SessionCallbacks<M> call, List<M> out) {
    for (IIncludeAdapter.Entry entry : requireAdapter().files(arg)) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
  }

  private void includeFile(String path, SessionCallbacks<M> call, List<M> out) {
    String content = requireAdapter().file(path);
    if (content != null)
      out.add(call.toolResult(new ToolResult(path, content)));
  }

  private void includeSearch(String arg, SessionCallbacks<M> call, List<M> out) {
    IIncludeAdapter a = requireAdapter();
    if ("files".equals(arg))
      for (IIncludeAdapter.Entry entry : a.searchFiles()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
    else if ("matches".equals(arg)) {
      List<IIncludeAdapter.Match> matches = a.searchMatches();
      out.add(call.toolResult(new ToolResult("search-matches", renderMatches(matches))));
    } else
      throw new IllegalStateException("Unknown search include argument: " + arg);
  }

  private String renderMatches(List<IIncludeAdapter.Match> matches) {
    ArrayNode arr = JsonNodeFactory.instance.arrayNode();
    for (IIncludeAdapter.Match match : matches) {
      ObjectNode node = arr.addObject();
      node.put("file", match.file);
      node.put("line", match.line);
      node.put("text", match.text);
    }
    return yaml.toYaml(arr);
  }

  private IIncludeAdapter requireAdapter() {
    if (adapter == null)
      throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
    return adapter;
  }
}
