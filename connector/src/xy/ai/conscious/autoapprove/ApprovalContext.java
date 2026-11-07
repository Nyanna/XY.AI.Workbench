package xy.ai.conscious.autoapprove;

import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.model.AIAnswer;

/**
 * Input of a rule.
 *
 * @param item {@code null} if the answer contains no (or more than one) tool call/result
 * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or
 *        the call is unknown (e.g. older than the remembered calls)
 */
public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,
		SessionSnapshot session) {

	public boolean hasItem() {
		return item != null;
	}
}
