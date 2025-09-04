package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import xy.ai.workbench.connectors.openai.IBatchEntry;
import xy.ai.workbench.models.IModelRequest;

class NewBatch implements IBatchEntry {
	private List<IModelRequest> requests = new ArrayList<>();

	@Override
	public String getID() {
		return AIBatchManager.UNSEND_ID;
	}

	@Override
	public boolean hasRequests() {
		return requests.isEmpty();
	}

	@Override
	public int getTaskCount() {
		return requests.size();
	}

	public void addRequest(IModelRequest req) {
		requests.add(req);
	}
	
	public List<IModelRequest> getRequests() {
		return requests;
	}
	
	public void clear() {
		requests.clear();
	}
}