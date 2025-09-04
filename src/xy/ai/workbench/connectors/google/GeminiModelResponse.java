package xy.ai.workbench.connectors.google;

import com.google.genai.types.GenerateContentResponse;

import xy.ai.workbench.models.IModelResponse;

public class GeminiModelResponse implements IModelResponse {
	public GenerateContentResponse response;

	public GeminiModelResponse(GenerateContentResponse resp) {
		response = resp;
	}
}
