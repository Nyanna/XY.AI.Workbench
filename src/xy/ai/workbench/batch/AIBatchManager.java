package xy.ai.workbench.batch;

import java.util.Map;
import java.util.TreeMap;

import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.models.IModelRequest;

public class AIBatchManager {
	static final String UNSEND_ID = "New";
	public static String KEY_REQIDS = "reqIds";
	private IAIBatchConnector connector;

	private Map<String, IAIBatch> index = new TreeMap<>();
	private TableViewer viewer;

	public AIBatchManager(IAIBatchConnector connector) {
		this.connector = connector;
	}

	public void updateBatches() {
		for (IAIBatch batch : connector.updateBatches())
			addOrUpdateEntry(batch);
	}

	public void enqueue(IModelRequest req) {
		NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null)
			addOrUpdateEntry(unsent = new NewBatch());
		unsent.addRequest(req);
		final IAIBatch toUpdate = unsent;
		Display.getDefault().asyncExec(() -> viewer.refresh(toUpdate, true));
	}

	public void submitBatches() {
		final NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null || unsent.hasRequests())
			return;

		IAIBatch res = connector.submitBatch(unsent);
		addOrUpdateEntry(res);

		unsent.clear();
		Display.getDefault().asyncExec(() -> viewer.refresh(unsent, true));
	}

	public void cancelbatch(IAIBatch batch) {
		IAIBatch res = connector.cancelBatch(batch);
		if (res != null) {
			batch.updateBy(res);
			addOrUpdateEntry(batch);
		}
	}

	private void addOrUpdateEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed == null) {
			index.put(entry.getID(), entry);
			Display.getDefault().asyncExec(() -> viewer.add(entry));
		} else {
			indexed.updateBy(entry);
			Display.getDefault().asyncExec(() -> viewer.refresh(indexed, true));
		}
	}

	public String requestsToString(IAIBatch entry) {
		return connector.requestsToJson(((NewBatch) entry).getRequests());
	}

	public void loadBatch(IAIBatch entry) {
		if (entry.getResult() != null)
			return;

		connector.loadBatch(entry);
		Display.getDefault().asyncExec(() -> viewer.refresh(entry, true));
	}

	public void setViewer(TableViewer viewer) {
		this.viewer = viewer;
	}
}
