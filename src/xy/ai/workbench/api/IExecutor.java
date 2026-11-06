package xy.ai.workbench.api;

public interface IExecutor {

	void asyncExec(Runnable run);

	void syncExec(Runnable run);

	public default void schedule(String name, IMonitoredRunnable run) {
		schedule(name, run, 0);
	}

	public void schedule(String name, IMonitoredRunnable run, long delay);
}
