package xy.ai.workbench.api;

public interface IProgress {

	public IProgress subProgress();

	public IProgress setTask(String taskName, int expectedWork);

	public void setTaskName(String name);

	public void worked();

	public void done();
}