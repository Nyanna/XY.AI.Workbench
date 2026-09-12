package xy.ai.workbench.connector.deepseek;

import xy.ai.workbench.connector.openapi.deepseek.request.responses.post.json.AllOfBody;
import xy.ai.workbench.models.IModelRequest;

public class DeepSeekRequest implements IModelRequest {
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
