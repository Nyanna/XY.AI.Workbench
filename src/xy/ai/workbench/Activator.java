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

	private MCPClient mcpClient;
	public ActiveEditorListener editorListener;
	public CCSessionManager ccSessionManager;
	public ConfigManagerAdapter cma;
	public IncludeAdapter includeAdapter;
	private AdaptingConnector connector;
	public AIBatchManager batch;
	public AIBatchResponseManager batchRequests;
	public EditorInterface editorInterface;
	public PromptHandler promptHandler;

	@Override
	public void start(BundleContext context) throws Exception {
		super.start(context);
		plugin = this;
		LOG.log = new EclipseLogging(Platform.getLog(context.getBundle()));

		mcpClient = new MCPClient();
		editorListener = new ActiveEditorListener();
		ccSessionManager = new CCSessionManager();
		cma = new ConfigManagerAdapter(editorListener, mcpClient, new EclipseStateCache());
		includeAdapter = new IncludeAdapter(editorListener);
		connector = new AdaptingConnector(cma.getManager(), ccSessionManager, mcpClient, includeAdapter,
				new EclipseMCPCLaucher());
		batch = new AIBatchManager(connector);
		batchRequests = new AIBatchResponseManager(connector);
		editorInterface = new EditorInterface(cma.getManager(), editorListener, connector);
		promptHandler = new PromptHandler(cma.getManager(), editorListener, connector, batch, editorInterface,
				new GitSupport());

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
