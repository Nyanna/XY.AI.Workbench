package xy.ai.conscious.autoapprove.rules;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.connector.claudecode.ControlApproval;

public interface IApprovalRule {

  default String id() {
    return getClass().getSimpleName();
  }

  Vote vote(ApprovalContext ctx);

  RuleSettings getDefaultSetting();

  /**
   * Called for every answer the operator gave to a control request.
   */
  default void onApproval(ControlApproval approval) {
  }
}
