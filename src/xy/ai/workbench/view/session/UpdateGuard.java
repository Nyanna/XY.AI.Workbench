package xy.ai.workbench.view.session;

/**
 * Suppresses re-entrant selection/modify events while a control's content is
 * being set programmatically.
 */
public class UpdateGuard {

	private boolean updating = false;

	public boolean isUpdating() {
		return updating;
	}

	public void run(Runnable action) {
		try {
			updating = true;
			action.run();
		} finally {
			updating = false;
		}
	}
}
