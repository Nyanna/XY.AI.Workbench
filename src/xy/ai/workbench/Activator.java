package xy.ai.workbench;

import org.eclipse.core.runtime.Platform;
import org.eclipse.ui.plugin.AbstractUIPlugin;
import org.osgi.framework.BundleContext;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.AIBatchResponseManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.connector.claudecode.CCSessionManager;
import xy.ai.workbench.connector.harness.PromptHandler;
import xy.ai.workbench.connector.mcp.MCPClient;

/**
 * The activator class controls the plug-in life cycle
 */
public class Activator extends AbstractUIPlugin {

	public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$

	private static Activator plugin;

	private final MCPClient mcpClient = new MCPClient();
	public final ActiveEditorListener editorListener = new ActiveEditorListener();
	public final CCSessionManager ccSessionManager = new CCSessionManager();
	public final ConfigManager cfg = new ConfigManager(mcpClient, editorListener);
	private final IncludeAdapter includeAdapter = new IncludeAdapter(editorListener);
	private final AdaptingConnector connector = new AdaptingConnector(cfg, ccSessionManager, mcpClient, includeAdapter);
	public final AIBatchManager batch = new AIBatchManager(connector);
	public final AIBatchResponseManager batchRequests = new AIBatchResponseManager(connector);
	public final EditorInterface editorInterface = new EditorInterface(cfg, editorListener, connector);
	public final PromptHandler promptHandler = new PromptHandler(cfg, editorListener, connector, batch, editorInterface,
			includeAdapter);

	@Override
	public void start(BundleContext context) throws Exception {
		super.start(context);
		plugin = this;
		editorInterface.register(context);
		LOG.log = Platform.getLog(context.getBundle());
	}

	@Override
	public void stop(BundleContext context) throws Exception {
		plugin = null;
		editorInterface.dispose(context);
		super.stop(context);
	}

	public static Activator getDefault() {
		return plugin;
	}
}
