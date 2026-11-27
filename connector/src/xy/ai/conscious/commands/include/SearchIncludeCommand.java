package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.List;

import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.conscious.connector.claudecode.YamlRenderer;
import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;
import xy.ai.conscious.harness.ToolResult;

public class SearchIncludeCommand<M> extends AdapterIncludeCommand<M> {

  private final YamlRenderer yaml = new YamlRenderer();

  public SearchIncludeCommand(IIncludeAdapter adapter) {
    super(adapter);
  }

  @Override
  public String kind() {
    return "search";
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
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
}
