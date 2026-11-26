package xy.ai.conscious.connector.harness;

import java.util.ArrayList;
import java.util.List;
import xy.ai.conscious.model.ModelMessage;
import com.fasterxml.jackson.databind.JsonNode;

public final class SessionAnswerBuilder {
  private final StringBuilder out = new StringBuilder();
  private final List<ModelMessage> messages = new ArrayList<>();

  public List<ModelMessage> messages() {
    return messages;
  }

  private SessionAnswerBuilder separate() {
    if (out.length() > 0)
      out.append("\n");
    return this;
  }

  public SessionAnswerBuilder text(String text) {
    if (text == null || text.isBlank())
      return this;
    messages.add(new ModelMessage(ModelMessage.Kind.Text, text));
    return separate().appendRaw(SessionRenderer.text(text));
  }

  public SessionAnswerBuilder reasoning(List<String> texts, String meta) {
    boolean noText = texts == null || texts.stream().allMatch(t -> t == null || t.isBlank());
    if (noText && (meta == null || meta.isBlank()))
      return this;
    if (!noText)
      messages.add(new ModelMessage(ModelMessage.Kind.Reasoning, String.join("\n\n", texts.stream().filter(t -> t != null && !t.isBlank()).toList())));
    return separate().appendRaw(SessionRenderer.reasoning(texts, meta));
  }

  public SessionAnswerBuilder toolCall(String id, String name, JsonNode arguments) {
    return separate().appendRaw(SessionRenderer.toolCall(new ToolCall(id, name, arguments)));
  }

  public SessionAnswerBuilder toolResult(String id, String content) {
    return toolResult(new ToolResult(id, content));
  }

  /**
   * Renders the result in the format it was read in.
   */
  public SessionAnswerBuilder toolResult(ToolResult result) {
    return separate().appendRaw(SessionRenderer.toolResult(result));
  }

  private SessionAnswerBuilder appendRaw(String text) {
    out.append(text);
    return this;
  }

  public boolean isEmpty() {
    return out.length() == 0;
  }

  @Override
  public String toString() {
    return out.toString();
  }
}
