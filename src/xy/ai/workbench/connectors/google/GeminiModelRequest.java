package xy.ai.workbench.connectors.google;

import java.util.List;

import com.google.genai.types.Content;
import com.google.genai.types.GenerateContentConfig;

import xy.ai.workbench.SessionConfig.Model;
import xy.ai.workbench.models.IModelRequest;

public class GeminiModelRequest implements IModelRequest {
	public static final String CUSTOM_ID = "customId";
	Model model;
	List<Content> prompt;
	GenerateContentConfig config;

	public GeminiModelRequest(Model model, List<Content> prompt, GenerateContentConfig config) {
		this.model = model;
		this.prompt = prompt;
		this.config = config;
	}

	@Override
	public String getID() {
		return "none"; //config.labels().get().get(CUSTOM_ID); // not supported
	}
}
