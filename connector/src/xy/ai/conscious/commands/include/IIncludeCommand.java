package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.List;

import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.SessionCallbacks;

public interface IIncludeCommand<M> {

  /** Kind in {@code [include kind](arg)}; {@code null} for the kind-less path include. */
  String kind();

  void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv);
}
