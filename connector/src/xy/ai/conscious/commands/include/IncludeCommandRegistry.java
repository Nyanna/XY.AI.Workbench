package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;

public class IncludeCommandRegistry<M> {

  private final Map<String, IIncludeCommand<M>> commands = new HashMap<>();

  public IncludeCommandRegistry(IIncludeAdapter adapter) {
    PathIncludeCommand<M> path = new PathIncludeCommand<>();
    register(path);
    register(new DirectoryIncludeCommand<>(adapter, path));
    register(new ContextPromptIncludeCommand<>(adapter));
    register(new FilesIncludeCommand<>(adapter));
    register(new FileIncludeCommand<>(adapter));
    register(new SearchIncludeCommand<>(adapter));
  }

  public final void register(IIncludeCommand<M> command) {
    commands.put(command.kind(), command);
  }

  public void execute(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    IIncludeCommand<M> command = commands.get(kind);
    if (command == null)
      throw new IllegalStateException("Unknown include kind: " + kind);
    command.execute(arg, chain, call, out, recursiv);
  }
}
