package xy.ai.conscious.autoapprove.rules;

import xy.ai.conscious.autoapprove.ApprovalContext;

public class ChainLimitRule implements IApprovalRule {
  public static final String ID = ChainLimitRule.class.getSimpleName();
  public static final String MAX_CHAIN_SETTING = "maxChain";
  public static final int MAX_CHAIN = 30;
  private final RuleSettings defaults;

  public ChainLimitRule() {
    this(MAX_CHAIN);
  }

  public ChainLimitRule(long maxChain) {
    this.defaults = new RuleSettings().addNumber(MAX_CHAIN_SETTING, maxChain);
  }

  @Override
  public String id() {
    return ID;
  }

  @Override
  public RuleSettings getDefaultSetting() {
    return defaults.clone();
  }

  @Override
  public Vote vote(ApprovalContext ctx) {
    long max = ctx.settings(ID).getNumber(MAX_CHAIN_SETTING);
    return ctx.depth() >= max ? Vote.hold("auto-approve chain limit reached (" + max + ")") : Vote.abstain();
  }
}
