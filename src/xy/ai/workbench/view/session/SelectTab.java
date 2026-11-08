package xy.ai.workbench.view.session;

import java.util.stream.IntStream;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.events.FocusListener;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.forms.widgets.FormToolkit;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.workbench.view.MultiSelectTable;

/**
 * "Select" tab: toggles system-prompt lines on/off and holds the free-text
 * addendum below it.
 */
public class SelectTab implements Clearable {
  private final FormToolkit toolkit;
  private final UpdateGuard guard;
  private final Runnable reflow;
  private MultiSelectTable instructionList;
  private Text instructionFree;

  public SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {
    this.toolkit = toolkit;
    this.guard = guard;
    this.reflow = reflow;
  }

  public void create(TabFolder instr, ConfigManager cfg) {
    TabItem instrSel = new TabItem(instr, SWT.NONE);
    instrSel.setText("Select");
    SashForm sash = new SashForm(instr, SWT.VERTICAL);
    sash.setLayout(new GridLayout(1, false));
    GridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);
    scl2.heightHint = 100;
    scl2.widthHint = 1;
    sash.setLayoutData(scl2);
    Composite comp = new Composite(sash, SWT.NONE);
    comp.setLayout(new GridLayout());
    GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
    ldat3.heightHint = 100;
    comp.setLayoutData(ldat3);
    instrSel.setControl(sash);
    instructionList = new MultiSelectTable(comp, () -> cfg.setSystemPrompt(togglePromptLines()));
    cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);
    GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
    gridData.widthHint = 1;
    gridData.heightHint = 100;
    instructionList.setLayoutData(gridData);
    createFreeTextArea(sash, cfg);
  }

  private void createFreeTextArea(SashForm sash, ConfigManager cfg) {
    instructionFree = toolkit.createText(sash, "", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);
    cfg.addSystemFreeObs(p -> {
      if (!instructionFree.isFocusControl())
        guard.run(() -> {
          instructionFree.setText(p != null ? p : "");
          reflow.run();
        });
    }, true);
    instructionFree.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));
    GridData gridData = new GridData(GridData.FILL_HORIZONTAL);
    gridData.widthHint = 1;
    instructionFree.setLayoutData(gridData);
    instructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));
    sash.setWeights(3, 1);
  }

  /**
   * Rewrites the "#" prefix of every line according to the active rows.
   */
  private String[] togglePromptLines() {
    String[] lines = instructionList.getItems();
    boolean[] active = new boolean[lines.length];
    for (int i : instructionList.getSelectionIndices()) active[i] = true;
    String[] upd = new String[lines.length];
    for (int i = 0; i < lines.length; i++) {
      String line = lines[i];
      if (!active[i] && !line.startsWith("#"))
        line = "#" + line;
      else if (active[i] && line.startsWith("#"))
        line = line.substring(1);
      upd[i] = line;
    }
    return upd;
  }

  private void updateInstructionList(String[] systemPrompt) {
    guard.run(() -> {
      if (!instructionList.updateTexts(systemPrompt))
        instructionList.setItems(systemPrompt);
      instructionList.setSelectionIndices(IntStream.range(0, systemPrompt.length).filter(i -> !systemPrompt[i].startsWith("#")).toArray());
      reflow.run();
    });
  }

  @Override
  public void clear() {
    instructionList.resetAnchor();
  }
}
