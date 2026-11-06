package xy.ai.workbench.connector;

import xy.ai.workbench.api.IExecutor.IProgress;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.core.Model.KeyPattern;

public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelResponse> {

	REQ createRequest(Prompt prompt, IProgress mon);

	RESP executeRequest(REQ req, IProgress mon);

	AIAnswer convertResponse(RESP resp, IProgress mon);

	KeyPattern getSupportedKeyPattern();

}