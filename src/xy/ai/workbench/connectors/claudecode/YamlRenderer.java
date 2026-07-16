package xy.ai.workbench.connectors.claudecode;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Pattern;

public class YamlRenderer {

	private static final Pattern TRAILING_WS_PER_LINE = Pattern.compile("[ \\t]+(?=\\n|$)");

	/**
	 * A second, YAML-flavoured mapper used exclusively to render/parse the
	 * human-facing side of the control loop (never for the wire protocol, which
	 * stays plain JSON via {@link #mapper} / {@link JsonUtil}).
	 *
	 * <p>
	 * Multi-line String values are written as literal block scalars ({@code |...})
	 * instead of {@code \n}-escaped one-liners &mdash; that is the whole point: a
	 * human can read and edit them as real, multi-line text. Everything else keeps
	 * the default double-quoting ({@code MINIMIZE_QUOTES} stays disabled) so YAML's
	 * implicit scalar typing never applies to untouched values: an unmodified
	 * String such as {@code country_code: "NO"} can never silently turn into the
	 * boolean {@code false} on the way back (the "Norway problem"), because it is
	 * never written as a bare, unquoted scalar in the first place. That risk only
	 * exists for values a user edits and (mistakenly) unquotes by hand &mdash; an
	 * accepted trade-off for readability.
	 */
	private final YAMLMapper yaml = YAMLMapper.builder().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER)
			.disable(YAMLGenerator.Feature.SPLIT_LINES).enable(YAMLGenerator.Feature.LITERAL_BLOCK_STYLE)
			.enable(YAMLGenerator.Feature.MINIMIZE_QUOTES).build();

	public JsonNode readTree(String text) throws JsonProcessingException {
		return yaml.readTree(text);
	}

	public String toYaml(JsonNode node) {
		if (node == null || node.isMissingNode() || node.isNull())
			return "";
		try {
			JsonNode sanitized = sanitizeWhitespace(node.deepCopy());
			return yaml.writeValueAsString(sanitized).stripTrailing();
		} catch (JsonProcessingException e) {
			throw new IllegalStateException("Failed to render control item as YAML", e);
		}
	}

	private JsonNode sanitizeWhitespace(JsonNode node) {
		if (node.isTextual())
			return TextNode.valueOf(stripTrailingPerLine(node.textValue()));
		if (node.isObject()) {
			ObjectNode obj = (ObjectNode) node;
			List<String> fieldNames = new ArrayList<>();
			obj.fieldNames().forEachRemaining(fieldNames::add);
			for (String name : fieldNames)
				obj.set(name, sanitizeWhitespace(obj.get(name)));
			return obj;
		}
		if (node.isArray()) {
			ArrayNode arr = (ArrayNode) node;
			for (int i = 0; i < arr.size(); i++)
				arr.set(i, sanitizeWhitespace(arr.get(i)));
			return arr;
		}
		return node;
	}

	private String stripTrailingPerLine(String input) {
		return TRAILING_WS_PER_LINE.matcher(input).replaceAll("");
	}
}