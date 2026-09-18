package xy.ai.workbench.view.session;

/**
 * Read/write access to a widget's multi-selection, needed for cross-tab preset
 * handling.
 */
public interface ToolSelection extends Clearable {
	public String[] getSelection();

	public void setSelection(String[] items);
}
