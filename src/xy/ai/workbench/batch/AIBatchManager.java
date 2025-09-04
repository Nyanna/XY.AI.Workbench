package xy.ai.workbench.batch;

import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Collectors;

import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.connectors.openai.IBatchEntry;
import xy.ai.workbench.connectors.openai.OpenAIBatchConnector;
import xy.ai.workbench.models.IModelRequest;

public class AIBatchManager {
	static final String UNSEND_ID = "New";
	public static String KEY_REQIDS = "reqIds";
	private OpenAIBatchConnector connector = new OpenAIBatchConnector();

	private Map<String, IBatchEntry> index = new TreeMap<>();
	private TableViewer viewer;

	public void updateBatches() {
		for (IBatchEntry batch : connector.updateBatches())
			addOrUpdateEntry(batch);
	}

	public void enqueue(IModelRequest req) {
		NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null)
			addOrUpdateEntry(unsent = new NewBatch());
		unsent.addRequest(req);
		final IBatchEntry toUpdate = unsent;
		Display.getDefault().asyncExec(() -> viewer.refresh(toUpdate, true));
	}

	public void submitBatches() {
		final NewBatch unsent = (NewBatch) index.get(UNSEND_ID);
		if (unsent == null || unsent.hasRequests())
			return;

		String json = requestsToString(unsent);
		IBatchEntry res = connector.submitBatch(json,
				unsent.getRequests().stream().map(r -> r.getID()).collect(Collectors.toList()));
		addOrUpdateEntry(res);

		unsent.clear();
		Display.getDefault().asyncExec(() -> viewer.refresh(unsent, true));
	}

	public void cancelbatch(IBatchEntry batch) {
		if (batch.getID().equals(UNSEND_ID))
			return;

		IBatchEntry res = connector.cancelBatch(batch);
		if (res != null) {
			batch.updateBy(res);
			addOrUpdateEntry(batch);
		}
	}

	private void addOrUpdateEntry(IBatchEntry entry) {
		IBatchEntry indexed = index.get(entry.getID());
		if (indexed == null) {
			index.put(entry.getID(), entry);
			Display.getDefault().asyncExec(() -> viewer.add(entry));
		} else {
			indexed.updateBy(entry);
			Display.getDefault().asyncExec(() -> viewer.refresh(indexed, true));
		}
	}

	public String requestsToString(IBatchEntry entry) {
		return connector.requestsToJson(((NewBatch) entry).getRequests());
	}

	public void loadBatch(IBatchEntry entry) {
		if (entry.getResult() != null)
			return;

		connector.loadBatch(entry);
		Display.getDefault().asyncExec(() -> viewer.refresh(entry, true));
	}

	public void setViewer(TableViewer viewer) {
		this.viewer = viewer;
	}
}
