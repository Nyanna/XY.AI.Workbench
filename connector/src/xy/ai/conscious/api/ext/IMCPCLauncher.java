package xy.ai.conscious.api.ext;

public interface IMCPCLauncher {
	boolean startMcpcLaunch(ProbeCallback ping);

	public static interface ProbeCallback {
		boolean ping();
	}
}
