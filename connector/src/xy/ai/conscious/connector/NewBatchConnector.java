package xy.ai.conscious.connector;

import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.batch.NewBatch;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.IModelRequest;

public class NewBatchConnector implements IAIBatchConnector {

	@Override
	public List<IAIBatch> updateBatches(IProgress mon) {
		throw new UnsupportedOperationException();
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry, IProgress mon) {
		throw new UnsupportedOperationException();
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry, IProgress mon) {
		((NewBatch) entry).clear();
		return entry;
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		throw new UnsupportedOperationException();
	}

	@Override
	public void loadBatch(IAIBatch entry, IProgress mon) {
	}

	@Override
	public void convertAnswers(IAIBatch obj, IProgress mon) {
		NewBatch entry = (NewBatch) obj;
		entry.setAnswers(entry.getRequests().stream().map(r -> {
			AIAnswer ans = new AIAnswer(r.getID());
			ans.answer = "Request not sent";
			return ans;
		}).collect(Collectors.toList()));
	}

}
