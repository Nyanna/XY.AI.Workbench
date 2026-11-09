package xy.ai.conscious.autoapprove;

import java.util.List;
import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
import java.util.Objects;
import java.util.Optional;
import xy.ai.conscious.autoapprove.rules.RuleSettings;
import java.util.concurrent.CopyOnWriteArrayList;
import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
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
    register(new DeepPathReadRule());
    register(new DenyRootPathRule());
    register(new HoldMarkdownRule());
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

  public Optional<IApprovalRule> find(String id) {
    return rules.stream().filter(r -> r.id().equals(id)).findFirst();
  }

  /**
   * Default settings of the rule as an independent clone; empty settings for an
   * unknown id.
   */
  public RuleSettings defaultSettings(String id) {
    return find(id).map(r -> r.getDefaultSetting().clone()).orElseGet(RuleSettings::new);
  }
}
