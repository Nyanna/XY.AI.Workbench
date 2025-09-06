package xy.ai.workbench.connectors.openai;

import com.openai.models.responses.Response;

import xy.ai.workbench.models.IModelResponse;

public class OpenAIResponse implements IModelResponse {
	public Response response;

	public OpenAIResponse(Response resp) {
		response = resp;
	}
}
