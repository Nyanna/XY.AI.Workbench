package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.Date;
import java.util.List;
import java.util.stream.Collectors;

import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class NewBatch implements IAIBatch {
	private List<IModelRequest> requests = new ArrayList<>();
	private List<AIAnswer> answers = Collections.emptyList();

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

	@Override
	public BatchState getState() {
		return BatchState.Prepared;
	}

	@Override
	public Date getStateDate() {
		return new Date();
	}

	@Override
	public String getBatchStatusString() {
		return "n/a";
	}

	public void clear() {
		requests.clear();
	}

	@Override
	public void updateBy(IAIBatch entry) {
		// ignore
	}

	@Override
	public Collection<AIAnswer> getAnswers() {
		return answers;
	}

	public void setAnswers(List<AIAnswer> answers) {
		this.answers = answers;
	}

	@Override
	public Date getExpires() {
		return null;
	}

	@Override
	public float getCompletion() {
		return 0;
	}

	@Override
	public String[] getRequestIDs() {
		return requests.stream().map(r -> r.getID()).collect(Collectors.toList()).toArray(new String[0]);
	}

	public List<Integer> getRequestIntIDs() {
		return requests.stream().map(r -> Integer.valueOf(r.getID())).collect(Collectors.toList());
	}

	@Override
	public int getDuration() {
		return 0;
	}

	@Override
	public String getResult() {
		return null;
	}

	@Override
	public String getError() {
		return null;
	}
}