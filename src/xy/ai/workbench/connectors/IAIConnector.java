package xy.ai.workbench.connectors;

import java.util.List;

import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public interface IAIConnector {

	IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix);

	IModelResponse executeRequest(IModelRequest request);

	AIAnswer convertResponse(IModelResponse response);

}