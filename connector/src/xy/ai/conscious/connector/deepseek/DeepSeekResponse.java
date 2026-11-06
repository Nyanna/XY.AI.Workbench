package xy.ai.conscious.connector.deepseek;

import xy.ai.conscious.model.IModelResponse;
import xy.ai.workbench.connector.openapi.deepseek.responses.Responses;

public class DeepSeekResponse implements IModelResponse {
	public Responses response;
	public final String id;

	public DeepSeekResponse(Responses response, String id) {
		this.response = response;
		this.id = id;
	}
}
