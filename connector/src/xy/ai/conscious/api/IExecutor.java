package xy.ai.conscious.api;

import xy.ai.conscious.core.LOG;

public interface IExecutor {

	void asyncExec(Runnable run);

	void syncExec(Runnable run);

	public default void schedule(String name, IMonitoredRunnable run) {
		schedule(name, run, 0);
	}

	public void schedule(String name, IMonitoredRunnable run, long delay);

	public interface IMonitoredRunnable {
		void run(IProgress mon) throws Exception;
	}

	public interface IProgress {

		default IProgress subProgress() {
			return this;
		}

		default IProgress setTask(String taskName, int expectedWork) {
			LOG.info(String.format("Task: %s (%d)", taskName, expectedWork));
			return this;
		}

		default void setTaskName(String name) {
		}

		default void worked() {
		}

		default void done() {
		}
	}
}
