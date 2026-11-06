package xy.ai.conscious.connector;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.IModelRequest;
import xy.ai.conscious.model.IModelResponse;
import xy.ai.conscious.model.Model.KeyPattern;

public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelResponse> {

	REQ createRequest(Prompt prompt, IProgress mon);

	RESP executeRequest(REQ req, IProgress mon);

	AIAnswer convertResponse(RESP resp, IProgress mon);

	KeyPattern getSupportedKeyPattern();

}