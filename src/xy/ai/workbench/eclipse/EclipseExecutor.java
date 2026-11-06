package xy.ai.workbench.eclipse;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.swt.widgets.Display;

import xy.ai.conscious.api.IExecutor;

public class EclipseExecutor implements IExecutor {

	@Override
	public void asyncExec(Runnable run) {
		Display.getDefault().asyncExec(run);
	}

	@Override
	public void syncExec(Runnable run) {
		Display.getDefault().syncExec(run);
	}

	@Override
	public void schedule(String name, IMonitoredRunnable run, long delay) {
		Job job = new Job(name) {
			@Override
			protected IStatus run(IProgressMonitor mon) {
				try {
					run.run(new EclipseProgress(mon));
				} catch (Exception e) {
					return Status.CANCEL_STATUS;
				} finally {
					mon.done();
				}
				return Status.OK_STATUS;
			}
		};
		job.schedule();
	}
}
