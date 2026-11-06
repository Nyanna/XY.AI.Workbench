package xy.ai.workbench.view;

import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.IMemento;
import org.eclipse.ui.IViewSite;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.forms.widgets.FormToolkit;
import org.eclipse.ui.forms.widgets.ScrolledForm;
import org.eclipse.ui.part.ViewPart;

import xy.ai.conscious.connector.harness.PromptHandler;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.workbench.Activator;
import xy.ai.workbench.eclipse.EclipseExecutor;
import xy.ai.workbench.view.session.ActionButtonsSection;
import xy.ai.workbench.view.session.InstructionSection;
import xy.ai.workbench.view.session.ModelParametersSection;
import xy.ai.workbench.view.session.OutputModeSection;
import xy.ai.workbench.view.session.UsageLogSection;

/**
 * Coordinates the session view's sections; owns only cross-cutting handles
 * (toolkit/form/display) that every section needs.
 */
public class AISessionView extends ViewPart {

	public static final String ID = "xy.ai.workbench.view.AISessionView";
	private InputStats inputStats;

	@Override
	public void saveState(IMemento memento) {
		super.saveState(memento);
		Activator.getDefault().cma.saveConfig(memento);
	}

	@Override
	public void init(IViewSite site, IMemento memento) throws PartInitException {
		super.init(site, memento);
		Activator.getDefault().cma.loadConfig(memento);
	}

	@Override
	public void createPartControl(Composite parent) {
		FormToolkit toolkit = new FormToolkit(parent.getDisplay());
		ScrolledForm form = toolkit.createScrolledForm(parent);
		Activator act = Activator.getDefault();
		ConfigManager cfg = act.cma.getManager();
		PromptHandler prompt = act.promptHandler;

		Composite body = form.getBody();
		body.setLayout(new GridLayout());

		new ModelParametersSection(toolkit).create(body, cfg);
		inputStats = new InputStats(cfg, act.editorListener);
		new InstructionSection(toolkit, () -> form.reflow(true)).create(body, cfg, inputStats, getSite().getShell());
		OutputModeSection.create(toolkit, body, cfg);
		ActionButtonsSection.create(body, cfg, prompt);
		UsageLogSection.create(toolkit, body, prompt, new EclipseExecutor());

		inputStats.initializeInputs();
		act.includeAdapter.initializeBindings();

		form.reflow(true);
	}

	@Override
	public void setFocus() {
	}
}
