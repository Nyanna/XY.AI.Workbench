package xy.ai.conscious.commands.include;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import xy.ai.conscious.harness.IncludeCycleException;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;

public class PathIncludeCommand<M> implements IIncludeCommand<M> {

  @Override
  public String kind() {
    return null;
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    String path = IncludeArgs.path(arg);
    if (path.isBlank())
      throw new IllegalStateException("Include path must not be empty");
    includePath(path, chain, call, out, recursiv);
  }

  void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
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
}
