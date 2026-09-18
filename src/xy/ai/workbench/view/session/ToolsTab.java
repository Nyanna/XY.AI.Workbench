package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.view.MultiSelectListener;

/** "Tools" tab: multi-select list of tools enabled for the current model. */
public class ToolsTab implements ToolSelection {

	private List toolsList;
	private MultiSelectListener toolsMultiSelect;

	public void create(TabFolder instr, ConfigManager cfg) {
		TabItem toolSel = new TabItem(instr, SWT.NONE);
		toolSel.setText("Tools");

		Composite comp = new Composite(instr, SWT.NONE);
		comp.setLayout(new GridLayout());
		GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
		ldat3.heightHint = 100;
		comp.setLayoutData(ldat3);
		toolSel.setControl(comp);
		toolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);

		GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
		gridData.widthHint = 1;
		gridData.heightHint = 100;
		toolsList.setLayoutData(gridData);
		toolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));
		toolsList.addSelectionListener(
				SelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));
		cfg.addModelObs(m -> {
			if (m != null)
				toolsList.setItems(m.cap.getTools());
		}, true);
		toolsMultiSelect = new MultiSelectListener(toolsList);
	}

	@Override
	public String[] getSelection() {
		return toolsList.getSelection();
	}

	@Override
	public void setSelection(String[] items) {
		toolsMultiSelect.setSelection(items);
	}

	@Override
	public void clear() {
		toolsMultiSelect.clear();
	}
}
