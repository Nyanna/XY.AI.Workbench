package xy.ai.conscious.autoapprove;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;

import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
import xy.ai.conscious.core.IEditorInterface;

/** Extracts tool calls and tool results from the rendered answer text. */
public class AnswerParser {

	private static final Pattern BLOCK = Pattern.compile(
			"^(" + Pattern.quote(IEditorInterface.CONTROL_REQUEST) + "|" + Pattern.quote(IEditorInterface.TOOLUSE) + "|"
					+ Pattern.quote(IEditorInterface.TOOLRESULT) + ")[ \\t]*\\R```yaml\\R(.*?)^```[ \\t]*$",
			Pattern.MULTILINE | Pattern.DOTALL);
	private static final YAMLMapper MAPPER = new YAMLMapper();

	static List<ApprovalItem> parse(String text) {
		List<ApprovalItem> items = new ArrayList<>();
		if (text == null || text.isEmpty())
			return items;
		Matcher m = BLOCK.matcher(text);
		while (m.find()) {
			ApprovalItem item = toItem(m.group(1), m.group(2));
			if (item != null)
				items.add(item);
		}
		return items;
	}

	private static ApprovalItem toItem(String marker, String yaml) {
		JsonNode node;
		try {
			node = MAPPER.readTree(yaml);
		} catch (JsonProcessingException e) {
			return null;
		}
		if (node == null || !node.isObject())
			return null;
		Kind kind;
		if (IEditorInterface.CONTROL_REQUEST.equals(marker))
			kind = node.has("result") ? Kind.ControlResult : Kind.ControlCall;
		else
			kind = IEditorInterface.TOOLUSE.equals(marker) ? Kind.ToolCall : Kind.ToolResult;
		String tool = node.path("tool").asText(node.path("name").asText(""));
		return new ApprovalItem(kind, node.path("id").asText(""), tool, node, yaml);
	}
}
