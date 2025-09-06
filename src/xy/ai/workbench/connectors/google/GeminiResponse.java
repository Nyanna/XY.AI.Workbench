package xy.ai.workbench.connectors.google;

import com.google.genai.types.GenerateContentResponse;

import xy.ai.workbench.models.IModelResponse;

public class GeminiResponse implements IModelResponse {
	public GenerateContentResponse response;

	public GeminiResponse(GenerateContentResponse resp) {
		response = resp;
	}
}
