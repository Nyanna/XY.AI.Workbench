package xy.ai.workbench.view.session;

import java.util.Arrays;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.workbench.view.MultiSelectTable;

/**
 * "Tools" tab: multi-select list of tools enabled for the current model.
 */
public class ToolsTab implements ToolSelection {
  private MultiSelectTable toolsTable;

  public void create(TabFolder instr, ConfigManager cfg) {
    TabItem toolSel = new TabItem(instr, SWT.NONE);
    toolSel.setText("Tools");
    Composite comp = new Composite(instr, SWT.NONE);
    comp.setLayout(new GridLayout());
    GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
    ldat3.heightHint = 100;
    comp.setLayoutData(ldat3);
    toolSel.setControl(comp);
    toolsTable = new MultiSelectTable(comp, () -> cfg.setEnabledTools(toolsTable.getSelection()));
    GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
    gridData.widthHint = 1;
    gridData.heightHint = 100;
    toolsTable.setLayoutData(gridData);
    cfg.addModelObs(m -> {
      if (m != null)
        toolsTable.setItems(m.cap.getTools());
    }, true);
    cfg.addEnabledToolsObs(t -> {
      if (!Arrays.equals(toolsTable.getSelection(), t))
        toolsTable.setSelection(t);
    }, true);
  }

  @Override
  public String[] getSelection() {
    return toolsTable.getSelection();
  }

  @Override
  public void setSelection(String[] items) {
    toolsTable.setSelection(items);
  }

  @Override
  public void clear() {
    toolsTable.clear();
  }
}
