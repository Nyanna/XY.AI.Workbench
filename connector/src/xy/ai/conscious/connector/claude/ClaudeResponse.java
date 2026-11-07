package xy.ai.conscious.connector.claude;

import xy.ai.conscious.connector.openapi.anthropic.components.Message;
import xy.ai.conscious.model.IModelResponse;

public class ClaudeResponse implements IModelResponse {
  public Message response;
  public final String id;

  public ClaudeResponse(Message resp, String id) {
    response = resp;
    this.id = id;
  }
}
