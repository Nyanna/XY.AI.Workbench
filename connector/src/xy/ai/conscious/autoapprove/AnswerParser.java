package xy.ai.conscious.autoapprove;

import java.util.ArrayList;
import java.util.List;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
import xy.ai.conscious.commands.AnswerCommand;
import xy.ai.conscious.commands.AnswerCommand.BlockType;
import xy.ai.conscious.commands.CallCommand;
import xy.ai.conscious.commands.Command;
import xy.ai.conscious.connector.harness.PromptInputHandler;
import xy.ai.conscious.connector.harness.PromptInputHandler.ParsedCommand;

/**
 * Extracts tool calls and tool results from the rendered answer text.
 */
public class AnswerParser {
  private static final YAMLMapper MAPPER = new YAMLMapper();

  static List<ApprovalItem> parse(String text) {
    List<ApprovalItem> items = new ArrayList<>();
    ParsedCommand parsed = PromptInputHandler.parseTrailingCommand(text);
    if (parsed == null || parsed.yaml() == null)
      return items;
    Kind kind = kindOf(parsed.command());
    ApprovalItem item = kind == null ? null : toItem(kind, parsed.yaml());
    if (item != null)
      items.add(item);
    return items;
  }

  private static Kind kindOf(Command command) {
    if (command instanceof AnswerCommand ac)
      return ac.blockType() == BlockType.Result ? Kind.ControlResult : Kind.ControlCall;
    if (command instanceof CallCommand)
      return Kind.ToolCall;
    return null;
  }

  private static ApprovalItem toItem(Kind kind, String yaml) {
    JsonNode node;
    try {
      node = MAPPER.readTree(yaml);
    } catch (JsonProcessingException e) {
      return null;
    }
    if (node == null || !node.isObject())
      return null;
    String tool = node.path("tool").asText(node.path("name").asText(""));
    return new ApprovalItem(kind, node.path("id").asText(""), tool, node, yaml);
  }
}
