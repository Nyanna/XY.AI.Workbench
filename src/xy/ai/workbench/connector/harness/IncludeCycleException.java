package xy.ai.workbench.connector.harness;

/** Thrown by {@link SessionProcessor} when an {@code [include]} directive would recurse into itself. */
public class IncludeCycleException extends RuntimeException {
	private static final long serialVersionUID = 1L;

	public IncludeCycleException(String path) {
		super("Cyclic [include] detected: " + path);
	}
}
