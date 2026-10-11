package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.swt.widgets.TableItem;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.connector.harness.PromptHandler;

/**
 * Checkbox table showing/enabling the available prompt input modes.
 */
public class InputsTable implements InputSelection {
  private ConfigManager cfg;

  public void create(Composite middle, ConfigManager cfg, PromptHandler prompt) {
    this.cfg = cfg;
    Table table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);
    table.setHeaderVisible(true);
    table.setLinesVisible(true);
    TableColumn column1 = new TableColumn(table, SWT.NONE);
    column1.setText("On");
    column1.setWidth(30);
    TableColumn column2 = new TableColumn(table, SWT.NONE);
    column2.setText("Input");
    column2.setWidth(120);
    TableColumn column3 = new TableColumn(table, SWT.NONE);
    column3.setText("Amount");
    column3.setWidth(45);
    table.addListener(SWT.Selection, e -> {
      if (e.detail == SWT.CHECK) {
        TableItem item = (TableItem) e.item;
        InputMode mode = InputMode.valueOf(item.getText(1).replace(" ", "_"));
        cfg.setInputMode(mode, !cfg.isInputEnabled(mode));
      }
    });
    for (int i = 0; i < InputMode.values().length; i++) {
      TableItem item = new TableItem(table, SWT.NONE);
      InputMode mode = InputMode.values()[i];
      prompt.addInputStatObs(is -> {
        var checked = item.getChecked();
        item.setText(new String[] { "", mode.name().replace("_", " "), is[mode.ordinal()] + "" });
        item.setChecked(checked);
      }, true);
      cfg.addInputObs(is -> {
        item.setChecked(is[mode.ordinal()]);
      }, true);
    }
  }

  @Override
  public String[] getSelection() {
    java.util.List<String> result = new java.util.ArrayList<>();
    for (InputMode mode : InputMode.values()) if (cfg.isInputEnabled(mode))
      result.add(mode.name());
    return result.toArray(new String[0]);
  }

  @Override
  public void setSelection(String[] items) {
    java.util.Set<String> enabled = new java.util.HashSet<>(java.util.Arrays.asList(items != null ? items : new String[0]));
    for (InputMode mode : InputMode.values()) cfg.setInputMode(mode, enabled.contains(mode.name()));
  }

  @Override
  public void clear() {
    for (InputMode mode : InputMode.values()) cfg.setInputMode(mode, false);
  }
}
