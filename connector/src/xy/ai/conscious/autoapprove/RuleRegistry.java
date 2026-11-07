package xy.ai.conscious.autoapprove;

import java.util.List;
import java.util.Objects;
import java.util.concurrent.CopyOnWriteArrayList;
import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
import xy.ai.conscious.autoapprove.rules.IApprovalRule;
import xy.ai.conscious.autoapprove.rules.SmallResultRule;

/**
 * Ordered registry of rules.
 */
public class RuleRegistry {
  private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();

  public RuleRegistry() {
    register(new AllowedToolsRule());
    register(new SmallResultRule());
  }

  public RuleRegistry(List<IApprovalRule> rules) {
    this.rules.addAll(rules);
  }

  public void register(IApprovalRule rule) {
    rules.addIfAbsent(Objects.requireNonNull(rule));
  }

  public boolean unregister(IApprovalRule rule) {
    return rules.remove(rule);
  }

  public List<IApprovalRule> rules() {
    return rules;
  }
}
