package xy.ai.workbench.api;

public interface IMCPCLauncher {
	boolean startMcpcLaunch(ProbeCallback ping);

	public static interface ProbeCallback {
		boolean ping();
	}
}
