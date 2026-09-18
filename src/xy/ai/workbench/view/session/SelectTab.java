package xy.ai.workbench.view.session;

import java.util.ArrayList;
import java.util.Arrays;

import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.events.FocusListener;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;

/**
 * "Select" tab: toggles system-prompt lines on/off and holds the free-text
 * addendum below it.
 */
public class SelectTab implements Clearable {

	private final FormToolkit toolkit;
	private final UpdateGuard guard;
	private final Runnable reflow;

	private List instructionList;
	private Text instructionFree;
	private java.util.List<String> instructionSelection = new ArrayList<>();

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
		instructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
		cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);

		GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
		gridData.widthHint = 1;
		gridData.heightHint = 100;
		instructionList.setLayoutData(gridData);
		instructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));

		instructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
			if (guard.isUpdating())
				return;
			String[] cur = instructionList.getItems();
			instructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));
			cfg.setSystemPrompt(updatePromptLines(cur));
		}));
		instructionList.addListener(SWT.MouseDown, event -> {
			if (guard.isUpdating())
				return;
			String[] clickedIndex = instructionList.getSelection();
			LOG.info(Arrays.toString(clickedIndex));
			if (clickedIndex.length != 1)
				return;

			if (!instructionSelection.remove(clickedIndex[0]))
				instructionSelection.add(clickedIndex[0]);

			String[] cur = instructionList.getItems();
			cfg.setSystemPrompt(updatePromptLines(cur));
		});

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
		instructionFree
				.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));
		GridData gridData = new GridData(GridData.FILL_HORIZONTAL);
		gridData.widthHint = 1;
		instructionFree.setLayoutData(gridData);
		instructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));
		sash.setWeights(3, 1);
	}

	private String[] updatePromptLines(String[] cur) {
		String[] upd = new String[cur.length];
		for (int i = 0; i < cur.length; i++) {
			String line = cur[i];
			boolean isSelected = instructionSelection.contains(line);
			if (!isSelected && !line.startsWith("#")) {
				line = "#" + line;
			} else if (isSelected && line.startsWith("#")) {
				line = line.substring(1);
			}
			upd[i] = line;
		}
		return upd;
	}

	private void updateInstructionList(String[] systemPrompt) {
		guard.run(() -> {
			instructionList.setItems(systemPrompt);
			instructionList.deselectAll();
			for (int i = 0; i < systemPrompt.length; i++) {
				if (!systemPrompt[i].startsWith("#"))
					instructionList.select(i);
			}
			reflow.run();
		});
	}

	@Override
	public void clear() {
		instructionSelection.clear();
	}
}
