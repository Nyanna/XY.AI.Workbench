package xy.ai.conscious.connector.harness;

public final class ToolResult {
  public final String id;
  public final String content;
  /**
   * The complete parsed result block with all its fields; {@code null} if not parsed from a block.
   */
  public final com.fasterxml.jackson.databind.JsonNode node;
  /**
   * The block was written as JSON and is rendered as JSON again; YAML otherwise.
   */
  public final boolean json;

  public ToolResult(String id, String content) {
    this(id, content, null);
  }

  public ToolResult(String id, String content, com.fasterxml.jackson.databind.JsonNode node) {
    this(id, content, node, false);
  }

  public ToolResult(String id, String content, com.fasterxml.jackson.databind.JsonNode node, boolean json) {
    this.id = id;
    this.content = content;
    this.node = node;
    this.json = json;
  }
}
