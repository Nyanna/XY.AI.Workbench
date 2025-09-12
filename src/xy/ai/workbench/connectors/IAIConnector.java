package xy.ai.workbench.connectors;

import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public interface IAIConnector {

	IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon);

	IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon);

	AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon);
	
	KeyPattern getSupportedKeyPattern();

}