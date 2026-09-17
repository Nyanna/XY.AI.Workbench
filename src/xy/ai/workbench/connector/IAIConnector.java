package xy.ai.workbench.connector;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.jobs.Job;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;
import xy.ai.workbench.connector.harness.Prompt;

public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelResponse> {

	REQ createRequest(Prompt prompt, IProgressMonitor mon);

	RESP executeRequest(REQ req, IProgressMonitor mon, Job job);

	AIAnswer convertResponse(RESP resp, IProgressMonitor mon);

	KeyPattern getSupportedKeyPattern();

}