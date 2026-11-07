package xy.ai.conscious.autoapprove;

import java.util.List;

/**
 * Outcome of one evaluation. Also the event published to listeners.
 *
 * @param item        {@code null} if no single tool call/result was detected
 * @param instruction follow-up instruction requested by rules, empty if none
 * @param followUp    prompt the approver triggers, {@code null} if none
 */
public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,
		List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp) {

	public Decision {
		result = result == null ? ApprovalResult.Pass : result;
		reason = reason == null ? "" : reason;
		hint = hint == null ? "" : hint;
		warnings = warnings == null ? List.of() : List.copyOf(warnings);
		instruction = instruction == null ? "" : instruction;
		votes = votes == null ? List.of() : List.copyOf(votes);
	}

	public static Decision pass(String sessionId) {
		return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null);
	}
}
