package xy.ai.workbench.connectors.openai;

import com.openai.models.responses.ResponseCreateParams;

import xy.ai.workbench.models.IModelRequest;

public class OpenAIRequest implements IModelRequest {
	public ResponseCreateParams reqquest;

	public OpenAIRequest(ResponseCreateParams params) {
		this.reqquest = params;
	}
	
	@Override
	public String getID() {
		return reqquest.safetyIdentifier().get();
	}
}
