package xy.ai.workbench.eclipse;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import xy.ai.workbench.api.IExecutor.IProgress;

public class EclipseProgress implements IProgress {

	private SubMonitor mon;

	public EclipseProgress(IProgressMonitor mon) {
		// got from job just converted
		this(SubMonitor.convert(mon));
	}

	private EclipseProgress(SubMonitor mon) {
		this.mon = mon;
	}

	@Override
	public void done() {
		mon.done();
	}

	@Override
	public void setTaskName(String name) {
		mon.subTask(name);
	}

	@Override
	public void worked() {
		mon.worked(1);
	}

	@Override
	public IProgress subProgress() {
		// called when giving a submonitor deeper
		return new EclipseProgress(mon.split(1));
	}

	@Override
	public IProgress setTask(String taskName, int expectedWork) {
		// always called first in a method
		mon.setWorkRemaining(expectedWork);
		mon.subTask(taskName);
		return this;
	}

}
