package xy.ai.workbench.batch;

import java.util.Collection;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Collectors;

import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.swt.widgets.Display;

import com.openai.models.batches.Batch;
import com.openai.models.responses.ResponseCreateParams;

public class AIBatchManager {
	private static final String UNSEND_ID = "New";
	public static String KEY_REQIDS = "reqIds";
	private OpenAIBatchConnector connector = new OpenAIBatchConnector();

	private Map<String, BatchEntry> index = new TreeMap<>();
	private TableViewer viewer;

	public void updateBatches() {
		for (Batch batch : connector.updateBatches())
			addOrUpdateEntry(new BatchEntry(batch.id(), batch));
	}

	public void enqueue(ResponseCreateParams req) {
		BatchEntry unsent = index.get(UNSEND_ID);
		if (unsent == null)
			addOrUpdateEntry(unsent = new BatchEntry(UNSEND_ID));
		unsent.request.put(unsent.request.size() + "", req);
		final BatchEntry toUpdate = unsent;
		Display.getDefault().asyncExec(() -> viewer.refresh(toUpdate, true));
	}

	public void submitBatches() {
		final BatchEntry unsent = index.get(UNSEND_ID);
		if (unsent == null || unsent.request.isEmpty())
			return;

		String json = requestsToString(unsent.request.values());
		Batch res = connector.submitBatch(json, unsent.request.keySet());
		addOrUpdateEntry(new BatchEntry(res.id(), res));

		unsent.request.clear();
		Display.getDefault().asyncExec(() -> viewer.refresh(unsent, true));
	}

	public void cancelbatch(BatchEntry batch) {
		if (batch.getID().equals(UNSEND_ID))
			return;

		Batch res = connector.cancelBatch(batch);
		if (res != null)
			addOrUpdateEntry(new BatchEntry(res.id(), res));
	}

	private void addOrUpdateEntry(BatchEntry entry) {
		BatchEntry indexed = index.get(entry.id);
		if (indexed == null) {
			index.put(entry.id, entry);
			Display.getDefault().asyncExec(() -> viewer.add(entry));
		} else {
			indexed.batch = entry.batch;
			Display.getDefault().asyncExec(() -> viewer.refresh(indexed, true));
		}
	}

	public String requestsToString(Collection<ResponseCreateParams> request) {
		return request.stream().map((r) -> connector.requestToBatch(r)).collect(Collectors.joining("\n"));
	}

	public void loadBatch(BatchEntry entry) {
		if(entry.result != null)
			return;
		
		connector.loadBatch(entry);
		Display.getDefault().asyncExec(() -> viewer.refresh(entry, true));
	}

	public void setViewer(TableViewer viewer) {
		this.viewer = viewer;
	}
}
