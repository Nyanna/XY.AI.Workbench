package xy.ai.workbench;

import org.eclipse.core.runtime.Platform;
import org.eclipse.ui.plugin.AbstractUIPlugin;
import org.osgi.framework.BundleContext;

import xy.ai.conscious.batch.AIBatchManager;
import xy.ai.conscious.batch.AIBatchResponseManager;
import xy.ai.conscious.connector.AdaptingConnector;
import xy.ai.conscious.connector.claudecode.CCSessionManager;
import xy.ai.conscious.connector.harness.PromptHandler;
import xy.ai.conscious.connector.mcp.MCPClient;
import xy.ai.conscious.core.LOG;
import xy.ai.workbench.eclipse.ConfigManagerAdapter;
import xy.ai.workbench.eclipse.EclipseLogging;
import xy.ai.workbench.eclipse.EclipseMCPCLaucher;
import xy.ai.workbench.eclipse.EclipseStateCache;
import xy.ai.workbench.eclipse.GitSupport;

/**
 * The activator class controls the plug-in life cycle
 */
public class Activator extends AbstractUIPlugin {

	public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$

	private static Activator plugin;

	private final MCPClient mcpClient = new MCPClient();
	public final ActiveEditorListener editorListener = new ActiveEditorListener();
	public final CCSessionManager ccSessionManager = new CCSessionManager();
	public final ConfigManagerAdapter cma = new ConfigManagerAdapter(editorListener, mcpClient, new EclipseStateCache());
	public final IncludeAdapter includeAdapter = new IncludeAdapter(editorListener);
	private final AdaptingConnector connector = new AdaptingConnector(cma.getManager(), ccSessionManager, mcpClient, includeAdapter, new EclipseMCPCLaucher());
	public final AIBatchManager batch = new AIBatchManager(connector);
	public final AIBatchResponseManager batchRequests = new AIBatchResponseManager(connector);
	public final EditorInterface editorInterface = new EditorInterface(cma.getManager(), editorListener, connector);
	public final PromptHandler promptHandler = new PromptHandler(cma.getManager(), editorListener, connector, batch, editorInterface,
			new GitSupport());

	@Override
	public void start(BundleContext context) throws Exception {
		super.start(context);
		plugin = this;
		LOG.log = new EclipseLogging(Platform.getLog(context.getBundle()));
		editorInterface.register(context);
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
