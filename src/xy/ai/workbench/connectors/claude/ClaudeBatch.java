package xy.ai.workbench.connectors.claude;

import java.util.Collection;
import java.util.Collections;
import java.util.Date;
import java.util.List;

import com.anthropic.models.messages.batches.MessageBatch;

import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;

public class ClaudeBatch implements IAIBatch {
	private String id;
	private MessageBatch batch;

	private Date created = new Date();
	// string content of result file
	private String result;
	// string content of error file
	private String error;
	private List<AIAnswer> answers = Collections.emptyList();

	public ClaudeBatch(String id) {
		this(id, null);
	}

	public ClaudeBatch(String id, MessageBatch batch) {
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
		if (!(entry instanceof ClaudeBatch))
			throw new IllegalArgumentException("Tried to update incompatible batch entries");
		ClaudeBatch oi = (ClaudeBatch) entry;

		if (oi.batch != null)
			batch = oi.batch;
		if (oi.result != null)
			result = oi.result;
	}

	public void setBatch(MessageBatch batch) {
		this.batch = batch;
	}

	public MessageBatch getBatch() {
		return batch;
	}

	@Override
	public String getID() {
		return batch != null ? batch.id() : id;
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
		if (batch != null)
			return Date.from(batch.expiresAt().toInstant());
		return null;
	}

	@Override
	public int getTaskCount() {
		if (batch != null)
			return (int) batch.requestCounts().processing();
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
		return new String[0];
	}

	@Override
	public String getBatchStatusString() {
		if (batch != null)
			return batch.processingStatus().known().name();
		return "n/a";
	}

	@Override
	public BatchState getState() {
		if (batch == null)
			return BatchState.Prepared;

		switch (batch.processingStatus().known()) {
		case CANCELING:
			return BatchState.Cancelling;
		case ENDED:
			if (batch.requestCounts().errored() > 0)
				return BatchState.Failed;
			return BatchState.Completed;
		case IN_PROGRESS:
			return BatchState.Proccessing;
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
				return Date.from(batch.createdAt().toInstant());
			case Cancelling:
			case Cancelled:
				if (batch.cancelInitiatedAt().isPresent())
					return Date.from(batch.cancelInitiatedAt().get().toInstant());
				else
					break;
			case Completed:
			case Failed:
			case Expired:
				if (batch.endedAt().isPresent())
					return Date.from(batch.endedAt().get().toInstant());
				else
					break;
			case Proccessing:
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