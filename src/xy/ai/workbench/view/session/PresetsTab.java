package xy.ai.workbench.view.session;

import java.util.ArrayList;
import java.util.Arrays;

import org.eclipse.core.resources.IFile;
import org.eclipse.swt.SWT;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionAdapter;
import org.eclipse.swt.events.SelectionEvent;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.view.PresetHandler;

/** "Presets" tab: save/load/reset the whole system-prompt configuration. */
public class PresetsTab {

	private List presetList;
	private final java.util.List<IFile> presetFiles = new ArrayList<>();

	public void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools,
			Clearable instructionSelection) {
		TabItem presEdit = new TabItem(instr, SWT.NONE);
		presEdit.setText("Presets");

		Composite comp = new Composite(instr, SWT.NONE);
		GridLayout compLay = new GridLayout(1, false);
		comp.setLayout(compLay);
		GridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);
		comp.setLayoutData(compDat);
		presEdit.setControl(comp);

		Composite buttons = new Composite(comp, SWT.NONE);
		buttons.setLayout(new GridLayout(2, false));

		Button writeButton = new Button(buttons, SWT.PUSH);
		writeButton.setText("Save");
		writeButton.addSelectionListener(new SelectionAdapter() {
			@Override
			public void widgetSelected(SelectionEvent e) {
				PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), shell);
				refreshPresetList();
			}
		});

		Button resetButton = new Button(buttons, SWT.PUSH);
		resetButton.setText("Reset");
		resetButton.addSelectionListener(new SelectionAdapter() {
			@Override
			public void widgetSelected(SelectionEvent e) {
				String[] cur = cfg.getSystemPrompt();
				String[] upd = new String[cur.length];
				for (int i = 0; i < cur.length; i++) {
					String line = cur[i];
					upd[i] = line.startsWith("#") ? line : "#" + line;
				}
				instructionSelection.clear();
				cfg.setSystemPrompt(upd);

				tools.clear();
				cfg.setEnabledTools(new String[0]);
			}
		});

		presetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);
		GridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);
		presetListDat.heightHint = 60;
		presetList.setLayoutData(presetListDat);
		presetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));
		presetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
			int idx = presetList.getSelectionIndex();
			if (idx < 0 || idx >= presetFiles.size())
				return;
			PresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));
			cfg.setSystemPrompt(preset.body);
			if (preset.tools != null) {
				tools.setSelection(preset.tools);
				cfg.setEnabledTools(preset.tools);
			}
			if (preset.outputMode != null)
				cfg.setOuputMode(preset.outputMode);
		}));
		refreshPresetList();
	}

	private void refreshPresetList() {
		presetFiles.clear();
		presetFiles.addAll(Arrays.asList(PresetHandler.listPresetFiles()));
		presetList.setItems(presetFiles.stream().map(f -> f.getFullPath().toString()).toArray(String[]::new));
	}
}
