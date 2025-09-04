package xy.ai.workbench.connectors.openai;

import java.util.Date;

import xy.ai.workbench.batch.BatchState;

public interface IBatchEntry {

	String getID();

	default String getErrorFileID() {
		return null;
	}

	default String getOutputFileID() {
		return null;
	}

	default String getInputFileID() {
		return null;
	}

	default Date getExpires() {
		return null;
	}

	default int getTaskCount() {
		return 0;
	}

	default float getCompletion() {
		return 0f;
	}

	default String[] getRequestIDs() {
		return null;
	}

	default String getBatchStatusString() {
		return null;
	}

	default BatchState getState() {
		return null;
	}

	default Date getStateDate() {
		return null;
	}

	default int getDuration() {
		return -1;
	}

	default String getResult() {
		return null;
	}

	default void setResult(String result) {
		throw new IllegalStateException("Immutable");
	}

	default boolean hasRequests() {
		return false;
	}
	
	default void updateBy(IBatchEntry entry) {
		throw new IllegalStateException("Immutable");
	}
}