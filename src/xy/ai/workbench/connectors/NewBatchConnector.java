package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

import org.eclipse.core.runtime.IProgressMonitor;

import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class NewBatchConnector implements IAIBatchConnector {

	@Override
	public List<IAIBatch> updateBatches(IProgressMonitor mon) {
		throw new UnsupportedOperationException();
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry, IProgressMonitor mon) {
		throw new UnsupportedOperationException();
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry, IProgressMonitor mon) {
		((NewBatch) entry).clear();
		return entry;
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		throw new UnsupportedOperationException();
	}

	@Override
	public void loadBatch(IAIBatch entry, IProgressMonitor mon) {
	}

	@Override
	public void convertAnswers(IAIBatch obj, IProgressMonitor mon) {
		NewBatch entry = (NewBatch) obj;
		entry.setAnswers(entry.getRequests().stream().map(r -> {
			AIAnswer ans = new AIAnswer();
			ans.id = r.getID();
			ans.answer = "Request not sent";
			return ans;
		}).collect(Collectors.toList()));
	}

}
