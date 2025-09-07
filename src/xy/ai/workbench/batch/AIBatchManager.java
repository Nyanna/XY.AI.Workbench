package xy.ai.workbench.batch;

import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
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

	public void updateBatches(IProgressMonitor mon, boolean updateAll) {
		List<IAIBatch> updateBatches = connector.updateBatches(mon);
		SubMonitor sub = SubMonitor.convert(mon, "Update view entry", updateBatches.size());
		for (IAIBatch batch : updateBatches) {
			if (updateAll)
				addOrUpdateEntry(batch);
			else
				updateEntry(batch);
			sub.worked(1);
		}
		sub.done();
	}

	public void enqueue(IModelRequest req, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Enqueue request", 1);
		NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null)
			addOrUpdateEntry(unsent = new NewBatch());
		unsent.addRequest(req);
		final IAIBatch toUpdate = unsent;
		Display.getDefault().asyncExec(() -> viewer.refresh(toUpdate, true));
		sub.done();
	}

	public void submitBatches(IProgressMonitor mon) {
		final NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null || unsent.hasRequests())
			return;

		SubMonitor sub = SubMonitor.convert(mon, "Submit Batch", 1);
		IAIBatch res = connector.submitBatch(unsent, sub);
		addOrUpdateEntry(res);

		unsent.clear();
		Display.getDefault().asyncExec(() -> viewer.refresh(unsent, true));
		sub.done();
	}

	public void cancelBatch(IAIBatch batch, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Cancel Batch", 1);
		IAIBatch res = connector.cancelBatch(batch, sub);
		if (res != null) {
			sub.subTask("Update Batch");
			batch.updateBy(res);
			addOrUpdateEntry(batch);
		}
		sub.done();
	}

	public void removeBatch(IAIBatch batch, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Remove Batch", 1);
		index.remove(batch.getID());
		Display.getDefault().asyncExec(() -> viewer.remove(batch));
		sub.done();
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

	private void updateEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed != null) {
			indexed.updateBy(entry);
			Display.getDefault().asyncExec(() -> viewer.refresh(indexed, true));
		}
	}

	@SuppressWarnings("unused")
	private void addEntry(IAIBatch entry) {
		IAIBatch indexed = index.get(entry.getID());
		if (indexed == null) {
			index.put(entry.getID(), entry);
			Display.getDefault().asyncExec(() -> viewer.add(entry));
		}
	}

	public String requestsToString(IAIBatch entry) {
		return connector.requestsToJson(((NewBatch) entry).getRequests());
	}

	public void loadBatch(IAIBatch entry, IProgressMonitor mon) {
		if (entry.getResult() != null)
			return;

		SubMonitor sub = SubMonitor.convert(mon, "Load Batch", 1);
		connector.loadBatch(entry, sub);
		Display.getDefault().asyncExec(() -> viewer.refresh(entry, true));
		sub.done();
	}

	public void setViewer(TableViewer viewer) {
		this.viewer = viewer;
	}

	public void clearBatches() {
		index.clear();
	}
}
