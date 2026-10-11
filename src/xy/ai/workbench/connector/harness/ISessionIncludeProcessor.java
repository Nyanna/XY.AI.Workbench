package xy.ai.workbench.connector.harness;

import java.nio.file.Path;
import java.util.List;

public interface ISessionIncludeProcessor<M> {
	/**
	 * Processes {@code text} as a flat document: every {@code [include...]} line is
	 * resolved, everything else is appended verbatim and emitted as a single
	 * {@code User} message per contiguous non-include run.
	 */
	public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv);
}
