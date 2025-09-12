package xy.ai.workbench.connectors.google;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.Date;
import java.util.List;

import com.google.genai.types.BatchJob;

import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;

public class GeminiBatch implements IAIBatch {
	private String id;
	private BatchJob batch;

	private Date created = new Date();
	// string content of result file
	private String result;
	// string content of error file
	private String error;
	private List<AIAnswer> answers = Collections.emptyList();

	public GeminiBatch(String id) {
		this(id, null);
	}

	public GeminiBatch(String id, BatchJob batch) {
		this.id = id;
		this.batch = batch;
	}

	@Override
	public void setAnswers(List<AIAnswer> answ) {
		answers = answ;
	}

	@Override
	public Collection<AIAnswer> getAnswers() {
		return answers;
	}

	@Override
	public void updateBy(IAIBatch entry) {
		if (!(entry instanceof GeminiBatch))
			throw new IllegalArgumentException("Tried to update incompatible batch entries");
		GeminiBatch oi = (GeminiBatch) entry;

		if (oi.batch != null)
			batch = oi.batch;
		if (oi.result != null)
			result = oi.result;
	}

	public void setBatch(BatchJob batch) {
		this.batch = batch;
	}

	public BatchJob getBatch() {
		return batch;
	}

	@Override
	public String getID() {
		return batch != null ? batch.name().orElseThrow() : id;
	}

	@Override
	public String getResult() {
		return result;
	}

	public void setResult(String result) {
		this.result = result;
	}

	@Override
	public String getError() {
		return error;
	}

	public void setError(String error) {
		this.error = error;
	}

	@Override
	public Date getExpires() {
		return null;
	}

	@Override
	public int getTaskCount() {
		String[] reqIds = getRequestIDs();
		if (reqIds != null && reqIds.length > 0)
			return reqIds.length;
		return -1;
	}

	@Override
	public float getCompletion() {
		if (BatchState.Completed.equals(getState()))
			return 1f;
		return 0f;
	}

	@Override
	public String[] getRequestIDs() {
		if (batch != null && batch.displayName().isPresent()) {
			List<String> result = new ArrayList<>();
			String concatenated = batch.displayName().get();

			if (concatenated.length() % 8 == 0) {
				for (int i = 0; i < concatenated.length(); i += 8)
					result.add(Integer.parseUnsignedInt(concatenated.substring(i, i + 8), 16) + "");
				return result.toArray(new String[0]);
			}
		}
		return new String[0];
	}

	@Override
	public String getBatchStatusString() {
		if (batch != null && batch.state().isPresent())
			return batch.state().get().toString();
		return "n/a";
	}

	@Override
	public BatchState getState() {
		if (batch == null)
			return BatchState.Prepared;

		switch (batch.state().get().knownEnum()) {
		case JOB_STATE_CANCELLED:
			return BatchState.Cancelled;
		case JOB_STATE_CANCELLING:
			return BatchState.Cancelling;
		case JOB_STATE_EXPIRED:
			return BatchState.Expired;
		case JOB_STATE_FAILED:
			return BatchState.Failed;
		case JOB_STATE_PARTIALLY_SUCCEEDED:
			return BatchState.Failed;
		case JOB_STATE_PAUSED:
		case JOB_STATE_PENDING:
		case JOB_STATE_QUEUED:
		case JOB_STATE_RUNNING:
		case JOB_STATE_UPDATING:
			return BatchState.Proccessing;
		case JOB_STATE_SUCCEEDED:
			return BatchState.Completed;
		case JOB_STATE_UNSPECIFIED:
		default:
		}
		return BatchState.Unknown;
	}

	@Override
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
				if (batch.createTime().isPresent())
					return Date.from(batch.createTime().get());
				else
					break;
			case Cancelling:
			case Cancelled:
			case Proccessing:
				if (batch.updateTime().isPresent())
					return Date.from(batch.updateTime().get());
				else
					break;
			case Completed:
			case Failed:
			case Expired:
				if (batch.endTime().isPresent())
					return Date.from(batch.endTime().get());
				else
					break;
			case Finalizing:
			case Unknown:
			}
		}
		return created;
	}

	@Override
	public int getDuration() {
		Date proc = getDate(BatchState.Created);
		Date com = getDate(BatchState.Completed);
		if (proc != null && com != null)
			return (int) ((com.getTime() - proc.getTime()) / 1000);
		return 0;
	}

	@Override
	public boolean hasRequests() {
		return false;
	}
}