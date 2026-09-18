package xy.ai.workbench.connector.harness;

import java.util.List;

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

	/**
	 * A list of reasoning text entries (typically one) together with opaque,
	 * connector-specific metadata required to replay them byte-exact; each entry
	 * corresponds to a {@link SessionRenderer#reasoningPlaceholder(int)} occurring
	 * exactly once in {@code meta}. A connector without such a schema may ignore
	 * {@code meta} and fall back to {@link #reasoning(Role, String)}.
	 */
	default M reasoning(Role role, List<String> texts, String meta) {
		return message(role, SessionRenderer.reasoning(texts, "{}"));
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
