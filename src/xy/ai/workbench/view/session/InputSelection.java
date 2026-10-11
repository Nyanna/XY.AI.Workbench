package xy.ai.workbench.view.session;

/**
 * Read/write access to the set of enabled {@link xy.ai.workbench.InputMode}s,
 * needed for cross-tab preset save/load and reset.
 */
public interface InputSelection extends Clearable {
	public String[] getSelection();

	public void setSelection(String[] items);
}
