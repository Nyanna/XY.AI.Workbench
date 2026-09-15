package xy.ai.workbench.connector.harness;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connector.claudecode.YamlRenderer;

/**
 * Deterministic, symmetric text encoding for session messages: the markers
 * and fenced YAML blocks produced here are exactly what {@link SessionProcessor}
 * recognizes on the way back in, so a model answer inserted into the document
 * and reprocessed yields byte-identical messages (stable prefix caching).
 */
public final class SessionRenderer {

	private static final ObjectMapper MAPPER = new ObjectMapper();
	private static final YamlRenderer YAML = new YamlRenderer();

	private SessionRenderer() {
	}

	public static String text(String text) {
		return text == null ? "" : text.strip();
	}

	public static String reasoning(String text) {
		return EditorInterface.THINKING + "\n" + (text == null ? "" : text.strip());
	}

	public static String toolCall(ToolCall call) {
		ObjectNode node = MAPPER.createObjectNode();
		node.put("id", call.id == null ? "" : call.id);
		node.put("tool", call.name == null ? "" : call.name);
		node.set("arguments",
				call.arguments != null && call.arguments.isObject() ? call.arguments : MAPPER.createObjectNode());
		return EditorInterface.TOOLUSE + "\n" + YAML.toYamlBlock(node);
	}

	public static String toolResult(ToolResult result) {
		ObjectNode node = MAPPER.createObjectNode();
		node.put("id", result.id == null ? "" : result.id);
		node.put("result", result.content == null ? "" : result.content);
		return EditorInterface.TOOLRESULT + "\n" + YAML.toYamlBlock(node);
	}
}
