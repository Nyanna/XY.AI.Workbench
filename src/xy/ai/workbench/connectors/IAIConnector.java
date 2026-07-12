package xy.ai.workbench.connectors;

import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelResponse> {

	REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon);

	RESP executeRequest(REQ req, IProgressMonitor mon);

	AIAnswer convertResponse(RESP resp, IProgressMonitor mon);

	KeyPattern getSupportedKeyPattern();

}