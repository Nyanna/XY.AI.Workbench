package xy.ai.conscious.batch;

import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.connector.IAIBatch;
import xy.ai.conscious.connector.IAIBatchConnector;
import xy.ai.conscious.model.IModelRequest;

public class AIBatchManager {
	static final String UNSEND_ID = "New";
	public static String KEY_REQIDS = "reqIds";
	private IAIBatchConnector connector;
	private IBatchListener listener;

	private Map<String, IAIBatch> index = new TreeMap<>();

	public AIBatchManager(IAIBatchConnector connector) {
		this.connector = connector;
	}
	
	public void setListener(IBatchListener listener) {
		this.listener = listener;
	}
	
	public Map<String, IAIBatch> getIndex() {
		return index;
	}

	public void updateBatches(IProgress mon, boolean updateAll) {
		List<IAIBatch> updateBatches = connector.updateBatches(mon);
		IProgress sub = mon.setTask("Update view entry", updateBatches.size());
		sub.setTaskName("Update view entries");
		for (IAIBatch batch : updateBatches) {
			if (updateAll)
				addOrUpdateEntry(batch);
			else
				updateEntry(batch);
			sub.worked();
		}
		listener.update();
	}

	public void enqueue(IModelRequest req, IProgress mon) {
		IProgress sub = mon.setTask("Enqueue request", 1);
		sub.setTaskName("Enqueue request");
		NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null)
			addOrUpdateEntry(unsent = new NewBatch());
		unsent.addRequest(req);
		final IAIBatch toUpdate = unsent;
		listener.update(toUpdate);
	}

	public void submitBatches(IProgress mon) {
		final NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null || unsent.hasRequests())
			return;

		IProgress sub = mon.setTask("Submit Batch", 1);
		IAIBatch res = connector.submitBatch(unsent, sub.subProgress());
		addOrUpdateEntry(res);

		unsent.clear();
		listener.update(unsent);
	}

	public void cancelBatch(IAIBatch batch, IProgress mon) {
		IProgress sub = mon.setTask("Cancel Batch", 1);
		IAIBatch res = connector.cancelBatch(batch, sub.subProgress());
		if (res != null) {
			sub.setTaskName("Update Batch");
			batch.updateBy(res);
			addOrUpdateEntry(batch);
		}
	}

	public void removeBatch(IAIBatch batch, IProgress mon) {
		IProgress sub = mon.setTask("Remove Batch", 1);
		index.remove(batch.getID());
		listener.remove(batch);
		sub.worked();
	}

	private void addOrUpdateEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed == null) {
			index.put(entry.getID(), entry);
			listener.add(entry);
		} else {
			indexed.updateBy(entry);
			listener.update(indexed);
		}
	}

	private void updateEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed != null) {
			indexed.updateBy(entry);
			listener.update(indexed);
		}
	}

	@SuppressWarnings("unused")
	private void addEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed == null) {
			index.put(entry.getID(), entry);
			listener.add(entry);
		}
	}

	public String requestsToString(IAIBatch entry) {
		return connector.requestsToJson(((NewBatch) entry).getRequests());
	}

	public void loadBatch(IAIBatch entry, IProgress mon) {
		if (entry.getResult() != null)
			return;

		IProgress sub = mon.setTask("Load Batch", 1);
		connector.loadBatch(entry, sub.subProgress());
		listener.update(entry);
	}

	public void clearBatches() {
		index.clear();
	}
	
	public static interface IBatchListener {
		void update();
		void update(IAIBatch entry);
		void add(IAIBatch entry);
		void remove(IAIBatch entry);
	}
}
