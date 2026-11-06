package xy.ai.workbench.api;

public interface ILogging {

	public void info(String message);

	public void info(String message, Throwable throwable);

	public void error(String message);

	public void error(String message, Throwable throwable);
}
