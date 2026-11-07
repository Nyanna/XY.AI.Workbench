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
import xy.ai.conscious.connector.harness.Role;
import xy.ai.conscious.connector.harness.SessionCallbacks;
import xy.ai.conscious.connector.harness.SessionProcessor;
import xy.ai.conscious.connector.harness.ToolCall;
import xy.ai.conscious.connector.harness.ToolResult;

/**
 * Extracts tool calls and tool results from the rendered answer text.
 */
public class AnswerParser {
  private static final YAMLMapper MAPPER = new YAMLMapper();
  private static final SessionProcessor<Object> PROCESSOR = new SessionProcessor<>(null);

  static List<ApprovalItem> parse(String text) {
    List<ApprovalItem> items = new ArrayList<>();
    ApprovalItem item = parseCommandItem(text);
    if (item == null)
      item = parseToolResult(text);
    if (item != null)
      items.add(item);
    return items;
  }

  private static ApprovalItem parseCommandItem(String text) {
    ParsedCommand parsed = PromptInputHandler.parseTrailingCommand(text);
    if (parsed == null || parsed.yaml() == null)
      return null;
    Kind kind = kindOf(parsed.command());
    return kind == null ? null : toItem(kind, parsed.yaml());
  }

  private static ApprovalItem parseToolResult(String text) {
    if (text == null || text.isBlank())
      return null;
    ToolResult[] last = new ToolResult[1];
    try {
      PROCESSOR.process(text, true, null, new SessionCallbacks<Object>() {

        @Override
        public Object message(Role role, String text) {
          return null;
        }

        @Override
        public Object reasoning(Role role, List<String> texts, String meta) {
          return null;
        }

        @Override
        public Object toolCall(ToolCall call) {
          return null;
        }

        @Override
        public Object toolResult(ToolResult result) {
          last[0] = result;
          return null;
        }
      });
    } catch (RuntimeException e) {
      return null;
    }
    if (last[0] == null || last[0].node == null)
      return null;
    try {
      return toItem(Kind.ToolResult, MAPPER.writeValueAsString(last[0].node));
    } catch (JsonProcessingException e) {
      return null;
    }
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
