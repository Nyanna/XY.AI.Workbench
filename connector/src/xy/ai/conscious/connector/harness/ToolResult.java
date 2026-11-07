package xy.ai.conscious.connector.harness;

/**
 * The (textual) result of a previously issued {@link ToolCall}.
 */
public final class ToolResult {
  public final String id;
  public final String content;
  /**
   * The complete parsed result block with all its fields; {@code null} if not parsed from a block.
   */
  public final com.fasterxml.jackson.databind.JsonNode node;

  public ToolResult(String id, String content) {
    this(id, content, null);
  }

  public ToolResult(String id, String content, com.fasterxml.jackson.databind.JsonNode node) {
    this.id = id;
    this.content = content;
    this.node = node;
  }
}
