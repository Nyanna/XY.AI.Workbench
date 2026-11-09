package xy.ai.conscious.autoapprove.rules;

import xy.ai.conscious.autoapprove.ApprovalContext;

public interface IApprovalRule {

  default String id() {
    return getClass().getSimpleName();
  }

  Vote vote(ApprovalContext ctx);

  RuleSettings getDefaultSetting();
}
