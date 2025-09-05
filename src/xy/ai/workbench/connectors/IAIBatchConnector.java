package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;

import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.models.IModelRequest;

public interface IAIBatchConnector {

	List<IAIBatch> updateBatches();

	IAIBatch submitBatch(NewBatch entry);

	IAIBatch cancelBatch(IAIBatch entry);

	void loadBatch(IAIBatch entry);

	String requestsToJson(Collection<IModelRequest> reqs);

	void convertAnswers(IAIBatch obj);
}