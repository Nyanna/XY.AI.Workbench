package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.ConfigManager;

/** "Edit" tab: raw multi-line editor for the system prompt. */
public class EditTab {

	private final FormToolkit toolkit;
	private final UpdateGuard guard;
	private final Runnable reflow;

	private Text instructionEdit;

	public EditTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {
		this.toolkit = toolkit;
		this.guard = guard;
		this.reflow = reflow;
	}

	public void create(TabFolder instr, ConfigManager cfg) {
		TabItem instrEdit = new TabItem(instr, SWT.NONE);
		instrEdit.setText("Edit");

		Composite comp = new Composite(instr, SWT.NONE);
		comp.setLayout(new GridLayout());
		instrEdit.setControl(comp);
		instructionEdit = toolkit.createText(comp, "", SWT.WRAP | SWT.V_SCROLL);
		cfg.addSystemPromptObs(p -> {
			if (!instructionEdit.isFocusControl())
				updateEditList(p.systemPrompt);
		}, true);
		instructionEdit.addModifyListener(e -> {
			if (guard.isUpdating())
				return;
			cfg.setSystemPrompt(instructionEdit.getText().split("\n"));
		});
		GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
		gridData.widthHint = 1;
		gridData.heightHint = 100;
		instructionEdit.setLayoutData(gridData);
		instructionEdit.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionEdit.setFocus()));
	}

	private void updateEditList(String[] systemPrompt) {
		guard.run(() -> {
			instructionEdit.setText(String.join("\n", systemPrompt));
			reflow.run();
		});
	}
}
