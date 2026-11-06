package xy.ai.conscious.connector;

import java.util.Collection;
import java.util.Date;
import java.util.List;

import xy.ai.conscious.batch.BatchState;
import xy.ai.conscious.model.AIAnswer;

public interface IAIBatch {

	String getID();

	Date getExpires();

	int getTaskCount();

	float getCompletion();

	String[] getRequestIDs();

	String getBatchStatusString();

	BatchState getState();

	Date getStateDate();

	int getDuration();

	String getResult();

	String getError();

	boolean hasRequests();

	void updateBy(IAIBatch entry);

	Collection<AIAnswer> getAnswers();

	void setAnswers(List<AIAnswer> answ);
}