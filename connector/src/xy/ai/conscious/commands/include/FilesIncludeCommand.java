package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.List;
import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;
import xy.ai.conscious.harness.ToolResult;

public class FilesIncludeCommand<M> extends AdapterIncludeCommand<M> {

  public FilesIncludeCommand(IIncludeAdapter adapter) {
    super(adapter);
  }

  @Override
  public String kind() {
    return "files";
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    for (IIncludeAdapter.Entry entry : requireAdapter().files(IncludeArgs.path(arg))) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
  }
}
