package xy.ai.workbench.connector.mcp;

import java.util.HashSet;
import java.util.Iterator;
import java.util.Map;
import java.util.Set;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.connector.claudecode.JsonUtil;
import xy.ai.workbench.connector.claudecode.YamlRenderer;

/**
 * Renders tool schemas as fillable YAML templates and turns a submitted
 * template back into a tool call. Results are returned as pretty JSON, giving
 * the user a maximal prefill using the same tools the agent later sees.
 */
public class MCPControlClient {
	private final YamlRenderer yaml = new YamlRenderer();

	/**
	 * Renders a tool's input schema as a YAML template (fenced block). Everything
	 * required for the call is bare YAML; descriptions and constraints are
	 * comments; defaults are pre-filled.
	 */
	@SuppressWarnings("deprecation")
	public String renderSchema(JsonNode tool) {
		StringBuilder sb = new StringBuilder();
		sb.append("```yaml\n");
		sb.append("tool: ").append(tool.path("name").asText()).append("\n");

		JsonNode schema = tool.path("inputSchema");
		JsonNode props = schema.path("properties");
		Set<String> required = requiredSet(schema);
		StringBuilder argsBody = new StringBuilder();
		if (props.isObject()) {
			Iterator<Map.Entry<String, JsonNode>> it = props.fields();
			while (it.hasNext()) {
				Map.Entry<String, JsonNode> e = it.next();
				if ("reason".equals(e.getKey()))
					continue;
				appendProperty(argsBody, e.getKey(), e.getValue(), required.contains(e.getKey()));
			}
		}
		if (argsBody.length() > 0) {
			sb.append("arguments:\n").append(argsBody);
		} else {
			sb.append("arguments: {}\n");
		}
		sb.append("```");
		return sb.toString();
	}

	/** Extracts the content of a leading/embedded ```yaml block, or null. */
	public String extractYamlBlock(String text) {
		if (text == null)
			return null;
		String t = text.strip();
		int start = t.indexOf("```yaml");
		if (start < 0)
			return null;
		int contentStart = start + "```yaml".length();
		int end = t.indexOf("```", contentStart);
		if (end == -1)
			return null;
		return t.substring(contentStart, end).strip();
	}

	public JsonNode parseYaml(String block) {
		try {
			JsonNode node = yaml.readTree(block);
			if (node == null || !node.isObject())
				throw new IllegalArgumentException("Tool call YAML must be a mapping");
			return node;
		} catch (JsonProcessingException e) {
			throw new IllegalArgumentException("Invalid YAML tool call: " + e.getMessage(), e);
		}
	}

	public String prettyResult(JsonNode result) {
		JsonNode structured = result.path("structuredContent");
		if (!structured.isMissingNode() && !structured.isNull())
			return "```json\n" + JsonUtil.pretty(structured) + "\n```";

		JsonNode content = result.path("content");
		if (content.isArray())
			return extractText(content);

		return "```json\n" + JsonUtil.pretty(result) + "\n```";
	}

	/** Concatenates the "text" entries of an MCP "content" array (plain, unfenced). */
	public String extractText(JsonNode content) {
		StringBuilder sb = new StringBuilder();
		for (JsonNode c : content) {
			if (sb.length() > 0)
				sb.append("\n");
			sb.append(c.path("text").asText(""));
		}
		return sb.toString();
	}

	/**
	 * Ensures a "reason" argument is present when the tool's schema declares one,
	 * since it is filtered out of the rendered template and never filled in by
	 * the user.
	 */
	public JsonNode fillReason(JsonNode tool, JsonNode arguments) {
		ObjectNode args = arguments != null && arguments.isObject() ? (ObjectNode) arguments
				: JsonUtil.mapper().createObjectNode();
		JsonNode props = tool == null ? null : tool.path("inputSchema").path("properties");
		if (props != null && props.has("reason") && !args.has("reason"))
			args.put("reason", "");
		return args;
	}

	private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolean required) {
		String indent = "  ";
		appendComment(sb, indent, prop.path("description").asText(""));
		String type = prop.path("type").asText(null);
		String value = renderValue(prop, type, indent);
		sb.append(indent).append(key).append(":");
		if (value.startsWith("\n"))
			sb.append(value).append("\n");
		else
			sb.append(" ").append(value).append("\n");
	}

	private String renderValue(JsonNode prop, String type, String indent) {
		JsonNode def = prop.get("default");
		if (def != null && !def.isNull())
			return JsonUtil.compact(def);
		if (type == null)
			return "null";
		switch (type) {
		case "string":
			return "\"\"";
		case "integer":
		case "number":
			return "0";
		case "boolean":
			return "false";
		case "array":
			return renderArrayValue(prop, indent);
		case "object":
			return "{}";
		default:
			return "null";
		}
	}

	/**
	 * Renders an array value. Generically pre-fills a single example entry when
	 * the array's item schema is an object with a "path" property, since that is
	 * by far the most common list-of-paths shape (e.g. "- path: /path").
	 */
	/**
	 * Renders an array value. Generically pre-fills a single example entry from
	 * the array's item schema when it is an object with properties, instead of
	 * just an empty "[]" — so the user sees the expected item shape directly.
	 */
	@SuppressWarnings("deprecation")
	private String renderArrayValue(JsonNode prop, String indent) {
		JsonNode itemProps = prop.path("items").path("properties");
		if (!itemProps.isObject() || itemProps.size() == 0)
			return "[]";

		String itemIndent = indent + "  ";
		StringBuilder sb = new StringBuilder();
		Iterator<Map.Entry<String, JsonNode>> it = itemProps.fields();
		boolean first = true;
		while (it.hasNext()) {
			Map.Entry<String, JsonNode> e = it.next();
			String childType = e.getValue().path("type").asText(null);
			String value = renderValue(e.getValue(), childType, itemIndent + "  ");
			sb.append("\n").append(itemIndent).append(first ? "- " : "  ").append(e.getKey()).append(": ")
					.append(value);
			first = false;
		}
		return sb.toString();
	}

	private Set<String> requiredSet(JsonNode schema) {
		Set<String> req = new HashSet<>();
		JsonNode r = schema.path("required");
		if (r.isArray())
			for (JsonNode n : r)
				req.add(n.asText());
		return req;
	}

	private void appendComment(StringBuilder sb, String indent, String text) {
		if (text == null || text.isBlank())
			return;
		for (String line : text.strip().split("\n"))
			sb.append(indent).append("# ").append(line.stripTrailing()).append("\n");
	}
}
