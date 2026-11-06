package xy.ai.conscious.connector.openai;

import com.openai.models.responses.Response;

import xy.ai.conscious.model.IModelResponse;

public class OpenAIResponse implements IModelResponse {
	public Response response;

	public OpenAIResponse(Response resp) {
		response = resp;
	}
}
