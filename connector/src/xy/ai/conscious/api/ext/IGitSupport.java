package xy.ai.conscious.api.ext;

import xy.ai.conscious.api.IExecutor;
import xy.ai.conscious.model.IModelRequest;

public interface IGitSupport {
	void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req);
}
