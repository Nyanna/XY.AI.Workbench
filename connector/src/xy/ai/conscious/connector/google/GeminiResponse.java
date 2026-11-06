package xy.ai.conscious.connector.google;

import com.google.genai.types.GenerateContentResponse;

import xy.ai.conscious.model.IModelResponse;

public class GeminiResponse implements IModelResponse {
	public final String id;
	public GenerateContentResponse response;

	public GeminiResponse(String id, GenerateContentResponse resp) {
		this.id = id;
		response = resp;
	}
}
