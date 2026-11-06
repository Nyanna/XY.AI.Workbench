package xy.ai.workbench.api;

public interface IMonitoredRunnable {
	public void run(IProgress mon) throws Exception;
}
