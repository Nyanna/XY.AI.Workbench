package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.List;
import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;
import xy.ai.conscious.harness.ToolResult;

public class FileIncludeCommand<M> extends AdapterIncludeCommand<M> {

  public FileIncludeCommand(IIncludeAdapter adapter) {
    super(adapter);
  }

  @Override
  public String kind() {
    return "file";
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    String path = IncludeArgs.path(arg);
    String content = requireAdapter().file(path);
    if (content != null)
      out.add(call.toolResult(new ToolResult(path, content)));
  }
}
