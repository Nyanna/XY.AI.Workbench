package xy.ai.conscious.connector.claude;

import xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams;
import xy.ai.conscious.model.AbstractModelRequest;

public class ClaudeRequest extends AbstractModelRequest {
  public static final String CUSTOM_ID = "customId";
  CreateMessageParams params;

  public ClaudeRequest(CreateMessageParams params) {
    this.params = params;
  }

  @Override
  public String getID() {
    return params.getMetadata().getUserId().getString();
  }
}
