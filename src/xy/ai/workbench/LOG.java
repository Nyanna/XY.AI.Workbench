package xy.ai.workbench;

import org.eclipse.core.runtime.ILog;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;

public class LOG {
	public static ILog log;

	public static void info(String message) {
		log.log(new Status(IStatus.INFO, Activator.PLUGIN_ID, message));
	}

	public static void info(String message, Throwable throwable) {
		log.log(new Status(IStatus.INFO, Activator.PLUGIN_ID, message, throwable));
	}

	public static void error(String message) {
		log.log(new Status(IStatus.ERROR, Activator.PLUGIN_ID, message));
	}

	public static void error(String message, Throwable throwable) {
		log.log(new Status(IStatus.ERROR, Activator.PLUGIN_ID, message, throwable));
	}
}
