package xy.ai.conscious.batch;

import java.util.ArrayList;
import java.util.List;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.connector.IAIBatch;
import xy.ai.conscious.connector.IAIBatchConnector;
import xy.ai.conscious.model.AIAnswer;

public class AIBatchResponseManager {
	private IAIBatchConnector connector;
	private List<AIAnswer> loadedAnswers = new ArrayList<>();
	private IAIBatch lastbatch;

	public AIBatchResponseManager(IAIBatchConnector connector) {
		this.connector = connector;
	}
	
	public List<AIAnswer> getLoadedAnswers() {
		return loadedAnswers;
	}

	public void load(IAIBatch obj, IProgress mon) {
		if (obj.equals(lastbatch))
			return;
		lastbatch = obj;

		IProgress sub = mon.setTask("Converting answers", 1);

		loadedAnswers.clear();
		if (obj.getResult() != null || obj.getError() != null || obj instanceof NewBatch)
			connector.convertAnswers(obj, sub.subProgress());

		loadedAnswers.addAll(obj.getAnswers());
	}

	public void clearAnswers() {
		loadedAnswers.clear();
	}
}
