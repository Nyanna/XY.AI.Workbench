package xy.ai.workbench.api.ext;

public interface IMCPCLauncher {
	boolean startMcpcLaunch(ProbeCallback ping);

	public static interface ProbeCallback {
		boolean ping();
	}
}
