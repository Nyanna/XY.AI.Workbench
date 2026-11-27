package xy.ai.conscious.commands.include;

import xy.ai.conscious.harness.IIncludeAdapter;

abstract class AdapterIncludeCommand<M> implements IIncludeCommand<M> {

  private final IIncludeAdapter adapter;

  protected AdapterIncludeCommand(IIncludeAdapter adapter) {
    this.adapter = adapter;
  }

  protected IIncludeAdapter requireAdapter() {
    if (adapter == null)
      throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
    return adapter;
  }
}
