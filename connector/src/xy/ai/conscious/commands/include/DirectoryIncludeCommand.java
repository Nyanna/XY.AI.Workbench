package xy.ai.conscious.commands.include;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;
import xy.ai.conscious.harness.ToolResult;

public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {
  private final PathIncludeCommand<M> pathCommand;

  public DirectoryIncludeCommand(IIncludeAdapter adapter, PathIncludeCommand<M> pathCommand) {
    super(adapter);
    this.pathCommand = pathCommand;
  }

  @Override
  public String kind() {
    return "directory";
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    String dir = IncludeArgs.path(arg);
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
    for (Path file : files) pathCommand.includePath(file.toString(), chain, call, out, recursiv);
  }
}
