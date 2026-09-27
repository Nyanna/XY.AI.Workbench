package xy.ai.workbench.layout;

import org.eclipse.e4.core.di.annotations.Execute;
import org.eclipse.e4.ui.model.application.MAddon;
import org.eclipse.e4.ui.model.application.MApplication;
import org.eclipse.e4.ui.model.application.MApplicationFactory;

public class FixedWidthPanelAddonProcessor {

	private static final String ADDON_ELEMENT_ID = "xy.ai.workbench.fixedWidthPanelAddon";
	private static final String CONTRIBUTION_URI = "bundleclass://XY.AI.Workbench/xy.ai.workbench.layout.FixedWidthPanelAddon";

	@Execute
	void execute(MApplication application) {
		for (MAddon addon : application.getAddons())
			if (ADDON_ELEMENT_ID.equals(addon.getElementId()))
				return; // allready registered
		MAddon addon = MApplicationFactory.INSTANCE.createAddon();
		addon.setElementId(ADDON_ELEMENT_ID);
		addon.setContributionURI(CONTRIBUTION_URI);
		application.getAddons().add(addon);
	}
}
