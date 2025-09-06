package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;

import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.models.IModelRequest;

public interface IAIBatchConnector {

	List<IAIBatch> updateBatches(IProgressMonitor mon);

	IAIBatch submitBatch(NewBatch entry, IProgressMonitor mon);

	IAIBatch cancelBatch(IAIBatch entry, IProgressMonitor mon);

	void loadBatch(IAIBatch entry, IProgressMonitor mon);

	String requestsToJson(Collection<IModelRequest> reqs);

	void convertAnswers(IAIBatch obj, IProgressMonitor mon);
}