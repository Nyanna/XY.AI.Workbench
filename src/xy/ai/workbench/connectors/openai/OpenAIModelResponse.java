package xy.ai.workbench.connectors.openai;

import com.openai.models.responses.Response;

import xy.ai.workbench.models.IModelResponse;

public class OpenAIModelResponse implements IModelResponse {
	public Response response;

	public OpenAIModelResponse(Response resp) {
		response = resp;
	}
}
