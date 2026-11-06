package xy.ai.conscious.connector.deepseek;

import xy.ai.conscious.model.AbstractModelRequest;
import xy.ai.workbench.connector.openapi.deepseek.request.responses.post.json.AllOfBody;

public class DeepSeekRequest extends AbstractModelRequest {
	public AllOfBody request;
	private final String id;

	public DeepSeekRequest(AllOfBody request, String id) {
		this.request = request;
		this.id = id;
	}

	@Override
	public String getID() {
		return id;
	}
}
