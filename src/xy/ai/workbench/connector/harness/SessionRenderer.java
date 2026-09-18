package xy.ai.workbench.connector.harness;

import java.util.List;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connector.claudecode.YamlRenderer;

/**
 * Renders {@link SessionCallbacks} turns back into the plain-text session format
 * consumed by {@link SessionProcessor}.
 */
public class SessionRenderer {
	private static final ObjectMapper MAPPER = new ObjectMapper();
	private static final YamlRenderer YAML = new YamlRenderer();
	private static final String REASONING_TEXT_PLACEHOLDER = "{{reasoning-text-";

	public static String reasoningPlaceholder(int index) {
		return REASONING_TEXT_PLACEHOLDER + index + "}}";
	}

	public static String text(String text) {
		return text == null ? "" : text.strip();
	}

	public static String reasoning(List<String> texts, String meta) {
		List<String> use = texts == null || texts.isEmpty() ? List.of("") : texts;
		StringBuilder sb = new StringBuilder();
		for (String text : use) {
			if (sb.length() > 0)
				sb.append("\n");
			sb.append(EditorInterface.THINKING).append("\n").append(text == null ? "" : text.strip());
		}
		sb.append("\n").append(EditorInterface.THINKING_META);
		if (meta != null && !meta.isBlank())
			sb.append(" ").append(meta);
		return sb.toString();
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
