package xy.ai.workbench.view;

import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IMemento;
import org.eclipse.ui.IViewSite;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.forms.widgets.FormToolkit;
import org.eclipse.ui.forms.widgets.ScrolledForm;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.connector.harness.PromptHandler;
import xy.ai.workbench.view.session.ActionButtonsSection;
import xy.ai.workbench.view.session.InstructionSection;
import xy.ai.workbench.view.session.OutputModeSection;
import xy.ai.workbench.view.session.TopParametersSection;
import xy.ai.workbench.view.session.UsageLogSection;

/**
 * Coordinates the session view's sections; owns only cross-cutting handles
 * (toolkit/form/display) that every section needs.
 */
public class AISessionView extends ViewPart {

	/**
	 * The ID of the view as specified by the extension.
	 */
	public static final String ID = "xy.ai.workbench.views.AISessionView";

	public static AISessionView currentInstance;

	@Inject
	IWorkbench workbench;

	private FormToolkit toolkit;
	private ScrolledForm form;

	public Display display;

	@Override
	public void saveState(IMemento memento) {
		super.saveState(memento);
		Activator.getDefault().cfg.saveConfig(memento);
	}

	@Override
	public void init(IViewSite site, IMemento memento) throws PartInitException {
		super.init(site, memento);
		Activator.getDefault().cfg.loadConfig(memento);
	}

	@Override
	public void createPartControl(Composite parent) {
		currentInstance = this;
		display = parent.getDisplay();
		toolkit = new FormToolkit(parent.getDisplay());
		form = toolkit.createScrolledForm(parent);
		Activator act = Activator.getDefault();
		ConfigManager cfg = act.cfg;
		PromptHandler prompt = act.session.getPromptHandler();

		Composite body = form.getBody();
		body.setLayout(new GridLayout());

		new TopParametersSection(toolkit).create(body, cfg);
		new InstructionSection(toolkit, () -> form.reflow(true)).create(body, cfg, prompt, getSite().getShell());
		OutputModeSection.create(toolkit, body, cfg);
		ActionButtonsSection.create(body, cfg, prompt);
		UsageLogSection.create(toolkit, body, prompt, display);

		prompt.initializeInputs();

		form.reflow(true);
	}

	@Override
	public void setFocus() {
		// form.setFocus();
	}
}
