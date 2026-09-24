package xy.ai.workbench.view.diff;

import org.eclipse.jgit.lib.ObjectId;

public record SnapshotResult(boolean created, String chain, String ref, ObjectId commit, ObjectId parent,
		String triggerLabel) {
}
