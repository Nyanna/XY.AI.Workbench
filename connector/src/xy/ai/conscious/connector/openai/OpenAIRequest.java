package xy.ai.conscious.connector.openai;

import com.openai.models.responses.ResponseCreateParams;

import xy.ai.conscious.model.AbstractModelRequest;

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
