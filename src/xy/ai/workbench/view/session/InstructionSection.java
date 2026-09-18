package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.connector.harness.PromptHandler;

/**
 * "System Prompt" area: Select/Edit/Tools/Presets tabs plus the input-mode
 * table.
 */
public class InstructionSection {

	private final FormToolkit toolkit;

	private final UpdateGuard guard = new UpdateGuard();
	private final SelectTab selectTab;
	private final EditTab editTab;
	private final ToolsTab toolsTab = new ToolsTab();
	private final PresetsTab presetsTab = new PresetsTab();

	public InstructionSection(FormToolkit toolkit, Runnable reflow) {
		this.toolkit = toolkit;
		this.selectTab = new SelectTab(toolkit, guard, reflow);
		this.editTab = new EditTab(toolkit, guard, reflow);
	}

	public void create(Composite body, ConfigManager cfg, PromptHandler prompt, Shell shell) {
		Composite middle = new Composite(body, SWT.NONE);
		middle.setLayout(new GridLayout(1, false));
		GridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);
		ldat2.heightHint = 100;
		middle.setLayoutData(ldat2);

		toolkit.createLabel(middle, "System Prompt:");

		Composite contComp = new Composite(middle, SWT.NONE);
		contComp.setLayout(new GridLayout(1, false));
		GridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);
		scl.heightHint = 100;
		scl.widthHint = 1;
		contComp.setLayoutData(scl);

		TabFolder instr = new TabFolder(contComp, SWT.NONE);
		GridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);
		ldat1.heightHint = 100;
		instr.setLayoutData(ldat1);

		selectTab.create(instr, cfg);
		editTab.create(instr, cfg);
		toolsTab.create(instr, cfg);
		presetsTab.create(instr, cfg, shell, toolsTab, selectTab);

		InputsTable.create(middle, cfg, prompt);
	}
}
