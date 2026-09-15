package xy.ai.workbench.connector.harness;

/**
 * Connector callback set invoked by {@link SessionProcessor}, in document
 * order, while turning session text into connector-native message objects
 * {@code M}. Only {@link #message(Role, String)} is mandatory; every other
 * callback falls back to it (rendered as plain text via
 * {@link SessionRenderer}) unless a connector overrides it - which is
 * exactly what happens for a message type a connector does not implement,
 * or when {@link SessionProcessor} itself is disabled.
 */
public interface SessionCallbacks<M> {

	/** Plain text turn: normal user input or ordinary agent text. */
	M message(Role role, String text);

	/** Reasoning/thinking text emitted by a previous agent turn. */
	default M reasoning(Role role, String text) {
		return message(role, SessionRenderer.reasoning(text));
	}

	/** A tool invocation, freshly requested or replayed from a previous turn. */
	default M toolCall(ToolCall call) {
		return message(Role.Agent, SessionRenderer.toolCall(call));
	}

	/** The result of executing a {@link ToolCall}. */
	default M toolResult(ToolResult result) {
		return message(Role.User, SessionRenderer.toolResult(result));
	}
}
