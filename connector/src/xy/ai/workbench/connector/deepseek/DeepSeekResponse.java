package xy.ai.workbench.connector.deepseek;

import xy.ai.workbench.connector.openapi.deepseek.responses.Responses;
import xy.ai.workbench.models.IModelResponse;

public class DeepSeekResponse implements IModelResponse {
	public Responses response;
	public final String id;

	public DeepSeekResponse(Responses response, String id) {
		this.response = response;
		this.id = id;
	}
}
