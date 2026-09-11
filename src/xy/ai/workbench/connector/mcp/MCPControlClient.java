package xy.ai.workbench.connector.mcp;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Set;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

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
	public String renderSchema(JsonNode tool) {
		StringBuilder sb = new StringBuilder();
		sb.append("```yaml\n");
		appendComment(sb, "", tool.path("description").asText(""));
		sb.append("tool: ").append(tool.path("name").asText()).append("\n");

		JsonNode schema = tool.path("inputSchema");
		JsonNode props = schema.path("properties");
		if (props.isObject() && props.size() > 0) {
			sb.append("arguments:\n");
			Set<String> required = requiredSet(schema);
			Iterator<Map.Entry<String, JsonNode>> it = props.fields();
			while (it.hasNext()) {
				Map.Entry<String, JsonNode> e = it.next();
				appendProperty(sb, e.getKey(), e.getValue(), required.contains(e.getKey()));
			}
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
			throw new IllegalArgumentException("Invalid YAML tool call", e);
		}
	}

	public String prettyResult(JsonNode result) {
		return "```json\n" + JsonUtil.pretty(result) + "\n```";
	}

	private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolean required) {
		String indent = "  ";
		appendComment(sb, indent, prop.path("description").asText(""));

		List<String> notes = new ArrayList<>();
		notes.add(required ? "required" : "optional");
		String type = prop.path("type").asText(null);
		if (type != null)
			notes.add("type: " + type);
		if (prop.has("enum"))
			notes.add("enum: " + JsonUtil.compact(prop.get("enum")));
		if (prop.has("default"))
			notes.add("default: " + JsonUtil.compact(prop.get("default")));
		if (prop.has("minimum"))
			notes.add("min: " + prop.get("minimum").asText());
		if (prop.has("maximum"))
			notes.add("max: " + prop.get("maximum").asText());
		if (prop.has("minLength"))
			notes.add("minLength: " + prop.get("minLength").asText());
		if (prop.has("maxLength"))
			notes.add("maxLength: " + prop.get("maxLength").asText());
		if (prop.has("format"))
			notes.add("format: " + prop.get("format").asText());
		sb.append(indent).append("# (").append(String.join(", ", notes)).append(")\n");

		sb.append(indent).append(key).append(": ").append(renderValue(prop, type)).append("\n");
	}

	private String renderValue(JsonNode prop, String type) {
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
			return "[]";
		case "object":
			return "{}";
		default:
			return "null";
		}
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
