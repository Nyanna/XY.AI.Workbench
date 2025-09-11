package xy.ai.workbench.connectors.google;

import com.google.genai.types.GenerateContentResponse;

import xy.ai.workbench.models.IModelResponse;

public class GeminiResponse implements IModelResponse {
	public final String id;
	public GenerateContentResponse response;

	public GeminiResponse(String id, GenerateContentResponse resp) {
		this.id = id;
		response = resp;
	}
}
