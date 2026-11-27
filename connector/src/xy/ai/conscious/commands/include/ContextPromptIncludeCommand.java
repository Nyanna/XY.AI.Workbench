package xy.ai.conscious.commands.include;

import java.nio.file.Path;
import java.util.List;

import xy.ai.conscious.harness.IIncludeAdapter;
import xy.ai.conscious.harness.ISessionIncludeProcessor;
import xy.ai.conscious.harness.Role;
import xy.ai.conscious.harness.SessionCallbacks;

public class ContextPromptIncludeCommand<M> extends AdapterIncludeCommand<M> {

  public ContextPromptIncludeCommand(IIncludeAdapter adapter) {
    super(adapter);
  }

  @Override
  public String kind() {
    return "contextprompt";
  }

  @Override
  public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
    String text = requireAdapter().contextPrompt(arg);
    if (text != null && !text.isBlank())
      out.add(call.message(Role.User, text.strip()));
  }
}
