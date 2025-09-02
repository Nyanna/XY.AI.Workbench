package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import com.openai.models.batches.Batch;
import com.openai.models.responses.ResponseCreateParams;

public class AIBatchManager {
	private OpenAIBatchConnector connector = new OpenAIBatchConnector();

	private Map<String, BatchEntry> index = new TreeMap<>();
	private List<BatchEntry> stored = new ArrayList<>();

	public void updateBatches() {
		for (Batch batch : connector.updateBatches())
			index.put(batch.id(), new BatchEntry(batch.id(), batch));
	}

	public void enqueue(ResponseCreateParams req) {
		stored.add(new BatchEntry(req));
	}


	public static class BatchEntry {
		public String id;
		public Batch batch;
		public ResponseCreateParams request;

		public BatchEntry(String id, Batch batch) {
			this.id = id;
			this.batch = batch;
		}

		public BatchEntry(ResponseCreateParams request) {
			this.request = request;
		}

	}
}
