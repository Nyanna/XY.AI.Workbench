package xy.ai.workbench.eclipse;

import org.eclipse.core.runtime.ILog;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;

import xy.ai.workbench.Activator;
import xy.ai.workbench.api.ILogging;

public class EclipseLogging implements ILogging {
	private final ILog log;

	public EclipseLogging(ILog log) {
		this.log = log;
	}

	public void info(String message) {
		log.log(new Status(IStatus.INFO, Activator.PLUGIN_ID, message));
	}

	public void info(String message, Throwable throwable) {
		log.log(new Status(IStatus.INFO, Activator.PLUGIN_ID, message, throwable));
	}

	public void error(String message) {
		log.log(new Status(IStatus.ERROR, Activator.PLUGIN_ID, message));
	}

	public void error(String message, Throwable throwable) {
		log.log(new Status(IStatus.ERROR, Activator.PLUGIN_ID, message, throwable));
	}
}
