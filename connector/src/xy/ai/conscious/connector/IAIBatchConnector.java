package xy.ai.conscious.connector;

import java.util.Collection;
import java.util.List;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.batch.NewBatch;
import xy.ai.conscious.model.IModelRequest;

public interface IAIBatchConnector {

	List<IAIBatch> updateBatches(IProgress mon);

	IAIBatch submitBatch(NewBatch entry, IProgress mon);

	IAIBatch cancelBatch(IAIBatch entry, IProgress mon);

	void loadBatch(IAIBatch entry, IProgress mon);

	String requestsToJson(Collection<IModelRequest> reqs);

	void convertAnswers(IAIBatch obj, IProgress mon);
}