package xy.ai.conscious.autoapprove;

public enum ApprovalResult {
	/** No decision; the answer is not a tool call/result or no rule decided. */
	Pass,
	/** Tool call or result of a control request was rejected. */
	Deny,
	/** Tool call or result of a control request was approved. */
	Allow,
	/** Tool call without control request was approved and is executed. */
	Call
}
