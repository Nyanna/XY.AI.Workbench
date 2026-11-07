package xy.ai.conscious.autoapprove.rules;

import xy.ai.conscious.autoapprove.ApprovalContext;

/**
 * A voter. Every registered rule is evaluated for every answer and decides on
 * its own whether it is applicable; if not it returns {@link Vote#abstain()}.
 * Rules must not have side effects.
 */
public interface IApprovalRule {

	default String id() {
		return getClass().getSimpleName();
	}

	Vote vote(ApprovalContext ctx);
}
