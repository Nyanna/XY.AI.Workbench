package xy.ai.conscious.autoapprove;

import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.model.AIAnswer;

/**
 * Input of a rule.
 *
 * @param item {@code null} if the answer contains no (or more than one) tool call/result
 */
public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, SessionSnapshot session) {

	public boolean hasItem() {
		return item != null;
	}
}
