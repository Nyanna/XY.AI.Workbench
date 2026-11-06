package xy.ai.workbench.api.ext;

import xy.ai.workbench.api.IExecutor;
import xy.ai.workbench.models.IModelRequest;

public interface IGitSupport {
	void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req);
}
