package xy.ai.workbench.connector.openai;

import com.openai.models.responses.ResponseCreateParams;

import xy.ai.workbench.models.AbstractModelRequest;

public class OpenAIRequest extends AbstractModelRequest {
	public ResponseCreateParams reqquest;

	public OpenAIRequest(ResponseCreateParams params) {
		this.reqquest = params;
	}

	@Override
	public String getID() {
		return reqquest.safetyIdentifier().get();
	}
}
