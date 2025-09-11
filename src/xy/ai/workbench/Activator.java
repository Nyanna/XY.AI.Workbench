package xy.ai.workbench;

import org.eclipse.ui.plugin.AbstractUIPlugin;
import org.osgi.framework.BundleContext;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.AIBatchResponseManager;
import xy.ai.workbench.connectors.AdaptingConnector;
import xy.ai.workbench.marker.MarkerRessourceScanner;

/**
 * The activator class controls the plug-in life cycle
 */
public class Activator extends AbstractUIPlugin {

	// The plug-in ID
	public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$

	// The shared instance
	private static Activator plugin;

	public ConfigManager cfg = new ConfigManager();
	private AdaptingConnector connector = new AdaptingConnector(cfg);

	public AISessionManager session = new AISessionManager(cfg, connector);

	public AIBatchManager batch = new AIBatchManager(connector);
	public AIBatchResponseManager batchRequests = new AIBatchResponseManager(connector);

	public MarkerRessourceScanner markerScanner;

	@Override
	public void start(BundleContext context) throws Exception {
		super.start(context);
		plugin = this;
		markerScanner = new MarkerRessourceScanner(context);
	}

	@Override
	public void stop(BundleContext context) throws Exception {
		plugin = null;
		if (markerScanner != null)
			markerScanner.dispose(context);
		markerScanner = null;
		super.stop(context);
	}

	/**
	 * Returns the shared instance
	 *
	 * @return the shared instance
	 */
	public static Activator getDefault() {
		return plugin;
	}

}
