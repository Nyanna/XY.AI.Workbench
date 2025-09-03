package xy.ai.workbench.batch;

import java.util.Collection;
import java.util.Date;
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
			addEntry(new BatchEntry(batch.id(), batch));
	}

	public void enqueue(ResponseCreateParams req) {
		BatchEntry unsent = index.get(UNSEND_ID);
		if (unsent == null)
			addEntry(unsent = new BatchEntry(UNSEND_ID));
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
		addEntry(new BatchEntry(res.id(), res));

		unsent.request.clear();
		Display.getDefault().asyncExec(() -> viewer.refresh(unsent, true));
	}

	private void addEntry(BatchEntry entry) {
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

	public static class BatchEntry {
		public String id;
		public Batch batch;
		public Map<String, ResponseCreateParams> request = new TreeMap<>();
		private Date created = new Date();
		// string content of result file
		public String result;

		public BatchEntry(String id) {
			this(id, null);
		}

		public BatchEntry(String id, Batch batch) {
			this.id = id;
			this.batch = batch;
		}

		public String getID() {
			return batch != null ? batch.id() : id;
		}

		public String getErrorFileID() {
			return batch != null && batch.errorFileId().isPresent() ? batch.errorFileId().get() : null;
		}

		public String getOutputFileID() {
			return batch != null && batch.outputFileId().isPresent() ? batch.outputFileId().get() : null;
		}

		public String getInputFileID() {
			return batch != null ? batch.inputFileId() : "none";
		}

		public Date getExpires() {
			return batch != null && batch.expiresAt().isPresent() ? new Date(batch.expiresAt().get()) : null;
		}

		public int getTaskCount() {
			if (batch != null) {
				var count = batch.requestCounts().orElse(null);
				if (count != null)
					return (int) count.total();
			} else {
				return request.size();
			}
			return -1;
		}

		public float getCompletion() {
			if (batch != null) {
				var count = batch.requestCounts().orElse(null);
				if (count != null) {
					long done = count.failed() + count.completed();
					return done > 0 ? count.total() / done : 0f;
				}
			}
			return 0f;
		}

		public String[] getRequestIDs() {
			// TODO implement req ids list
			// TODO add metadata helpfull comment
			if (batch != null) {
				var meta = batch.metadata().orElse(null);
				if (meta != null)
					meta._additionalProperties();
			} else {
			}
			return new String[0];
		}

		public String getBatchStatusString() {
			if (batch != null)
				return batch.status().asString();
			return "n/a";
		}

		public BatchState getState() {
			if (batch == null)
				return BatchState.Prepared;
			Date cd = null;
			BatchState cs = BatchState.Unknown;
			for (BatchState s : BatchState.values()) {
				if (BatchState.Prepared.equals(s))
					continue;
				Date nd = getDate(s);
				if (nd != null && (cd == null || nd.after(cd))) {
					cd = nd;
					cs = s;
				}
			}
			if (cd == null)
				cd = getDate(BatchState.Prepared);
			return cs;
		}

		public Date getStateDate() {
			return getDate(getState());
		}

		private Date getDate(BatchState state) {
			if (batch == null) {
				switch (state) {
				case Prepared:
					return created;
				default:
				}
			} else {
				switch (state) {
				case Prepared:
					return created;
				case Created:
					return new Date(batch.createdAt());
				case Cancelling:
					return batch.cancellingAt().isPresent() ? new Date(batch.cancellingAt().get() * 1000) : null;
				case Cancelled:
					return batch.cancelledAt().isPresent() ? new Date(batch.cancelledAt().get() * 1000) : null;
				case Proccessing:
					return batch.inProgressAt().isPresent() ? new Date(batch.inProgressAt().get() * 1000) : null;
				case Completed:
					return batch.completedAt().isPresent() ? new Date(batch.completedAt().get() * 1000) : null;
				case Failed:
					return batch.failedAt().isPresent() ? new Date(batch.failedAt().get() * 1000) : null;
				case Expired:
					return batch.expiredAt().isPresent() ? new Date(batch.expiredAt().get() * 1000) : null;
				case Finalizing:
					return batch.finalizingAt().isPresent() ? new Date(batch.finalizingAt().get() * 1000) : null;
				case Unknown:
				}
			}
			return null;
		}
	}

	public static enum BatchState {
		Prepared, Created, Cancelling, Cancelled, Proccessing, Completed, Failed, Expired, Finalizing, Unknown;
	}

	public void setViewer(TableViewer viewer) {
		this.viewer = viewer;
	}
}
