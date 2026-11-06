package xy.ai.workbench.eclipse;

import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.NullProgressMonitor;
import org.eclipse.debug.core.DebugPlugin;
import org.eclipse.debug.core.ILaunch;
import org.eclipse.debug.core.ILaunchConfiguration;
import org.eclipse.debug.core.ILaunchConfigurationType;
import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
import org.eclipse.debug.core.ILaunchManager;
import xy.ai.workbench.api.IMCPCLauncher;
import xy.ai.workbench.core.LOG;

public class EclipseMCPCLaucher implements IMCPCLauncher {
	private static final String MCPC_LAUNCH_NAME = "MCPC";

	@Override
	public boolean startMcpcLaunch(ProbeCallback ping) {
		try {
			ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
			for (ILaunch l : lm.getLaunches()) {
				ILaunchConfiguration c = l.getLaunchConfiguration();
				if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
					return false;
			}
			ILaunchConfiguration cfg = null;
			for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
				if (MCPC_LAUNCH_NAME.equals(c.getName())) {
					cfg = c;
					break;
				}
			}
			if (cfg == null) {
				ILaunchConfigurationType type = lm
						.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
				ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
				String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
				wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
				wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
				wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
				wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
				wc.setAttribute("org.eclipse.debug.core.environmentVariables",
						java.util.Map.of("TERM", "xterm-256color"));
				cfg = wc.doSave();
			}
			cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
			long end = System.currentTimeMillis() + 3000;
			while (System.currentTimeMillis() < end) {
				if (ping.ping())
					return true;
				Thread.sleep(250);
			}
		} catch (InterruptedException e) {
			Thread.currentThread().interrupt();
		} catch (CoreException e) {
			LOG.error("Failed to start MCPC run configuration", e);
		}
		return false;
	}
}
