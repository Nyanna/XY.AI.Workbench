package xy.ai.conscious.connector.google;

import java.util.List;

import com.google.genai.types.Content;
import com.google.genai.types.GenerateContentConfig;

import xy.ai.conscious.model.AbstractModelRequest;
import xy.ai.conscious.model.Model;

public class GeminiRequest extends AbstractModelRequest {
	public static final String CUSTOM_ID = "customId";
	Model model;
	List<Content> prompt;
	GenerateContentConfig config;
	private String id;

	public GeminiRequest(Model model, List<Content> prompt, GenerateContentConfig config, String id) {
		this.model = model;
		this.prompt = prompt;
		this.config = config;
		this.id = id;
	}

	@Override
	public String getID() {
		return id;
	}
}
